#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ENET0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet {
    ptr: *mut u8,
}
unsafe impl Send for Enet {}
unsafe impl Sync for Enet {}
impl Enet {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MAC Configuration Register."]
    #[inline(always)]
    pub const fn maccfg(self) -> crate::common::Reg<regs::Maccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC Frame Filter."]
    #[inline(always)]
    pub const fn macff(self) -> crate::common::Reg<regs::Macff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Hash Table High Register."]
    #[inline(always)]
    pub const fn hash_h(self) -> crate::common::Reg<regs::HashH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Hash Table Low Register."]
    #[inline(always)]
    pub const fn hash_l(self) -> crate::common::Reg<regs::HashL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GMII Address Register."]
    #[inline(always)]
    pub const fn gmii_addr(self) -> crate::common::Reg<regs::GmiiAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GMII Data Register."]
    #[inline(always)]
    pub const fn gmii_data(self) -> crate::common::Reg<regs::GmiiData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Flow Control Register."]
    #[inline(always)]
    pub const fn flowctrl(self) -> crate::common::Reg<regs::Flowctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "VLAN Tag Register."]
    #[inline(always)]
    pub const fn vlan_tag(self) -> crate::common::Reg<regs::VlanTag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Remote Wake-Up Frame Filter Register."]
    #[inline(always)]
    pub const fn rwkfrmfilt(self) -> crate::common::Reg<regs::Rwkfrmfilt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PMT Control and Status Register."]
    #[inline(always)]
    pub const fn pmt_csr(self) -> crate::common::Reg<regs::PmtCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "LPI Control and Status Register."]
    #[inline(always)]
    pub const fn lpi_csr(self) -> crate::common::Reg<regs::LpiCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "LPI Timers Control Register."]
    #[inline(always)]
    pub const fn lpi_tcr(self) -> crate::common::Reg<regs::LpiTcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn intr_status(self) -> crate::common::Reg<regs::IntrStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Mask Register."]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MAC Address 0 High Register."]
    #[inline(always)]
    pub const fn mac_addr_0_high(
        self,
    ) -> crate::common::Reg<regs::MacAddr0High, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MAC Address 0 Low Register."]
    #[inline(always)]
    pub const fn mac_addr_0_low(self) -> crate::common::Reg<regs::MacAddr0Low, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn mac_addr(self, n: usize) -> MacAddr {
        assert!(n < 4usize);
        unsafe { MacAddr::from_ptr(self.ptr.wrapping_add(0x48usize + n * 8usize) as _) }
    }
    #[doc = "SGMII/RGMII/SMII Control and Status Register."]
    #[inline(always)]
    pub const fn xmii_csr(self) -> crate::common::Reg<regs::XmiiCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Watchdog Timeout Register."]
    #[inline(always)]
    pub const fn wdog_wto(self) -> crate::common::Reg<regs::WdogWto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "MMC Control establishes the operating mode of MMC."]
    #[inline(always)]
    pub const fn mmc_cntrl(self) -> crate::common::Reg<regs::MmcCntrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "MMC Receive Interrupt maintains the interrupt generated from all of the receive statistic counters."]
    #[inline(always)]
    pub const fn mmc_intr_rx(self) -> crate::common::Reg<regs::MmcIntrRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "MMC Transmit Interrupt maintains the interrupt generated from all of the transmit statistic counters."]
    #[inline(always)]
    pub const fn mmc_intr_tx(self) -> crate::common::Reg<regs::MmcIntrTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "MMC Receive Interrupt mask maintains the mask for the interrupt generated from all of the receive statistic counters."]
    #[inline(always)]
    pub const fn mmc_intr_mask_rx(
        self,
    ) -> crate::common::Reg<regs::MmcIntrMaskRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "MMC Transmit Interrupt Mask."]
    #[inline(always)]
    pub const fn mmc_intr_mask_tx(
        self,
    ) -> crate::common::Reg<regs::MmcIntrMaskTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
    #[inline(always)]
    pub const fn txoctetcount_gb(
        self,
    ) -> crate::common::Reg<regs::TxoctetcountGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted, exclusive of retried frames."]
    #[inline(always)]
    pub const fn txframecount_gb(
        self,
    ) -> crate::common::Reg<regs::TxframecountGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Number of good broadcast frames transmitted."]
    #[inline(always)]
    pub const fn txbroadcastframes_g(
        self,
    ) -> crate::common::Reg<regs::TxbroadcastframesG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Number of good multicast frames transmitted."]
    #[inline(always)]
    pub const fn txmlticastframes_g(
        self,
    ) -> crate::common::Reg<regs::TxmlticastframesG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx64octets_gb(self) -> crate::common::Reg<regs::Tx64octetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx65to127octets_gb(
        self,
    ) -> crate::common::Reg<regs::Tx65to127octetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx128to255octets_gb(
        self,
    ) -> crate::common::Reg<regs::Tx128to255octetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx256to511octets_gb(
        self,
    ) -> crate::common::Reg<regs::Tx256to511octetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx512to1023octets_gb(
        self,
    ) -> crate::common::Reg<regs::Tx512to1023octetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[inline(always)]
    pub const fn tx1024tomaxoctets_gb(
        self,
    ) -> crate::common::Reg<regs::Tx1024tomaxoctetsGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Number of good and bad frames received."]
    #[inline(always)]
    pub const fn rxframecount_gb(
        self,
    ) -> crate::common::Reg<regs::RxframecountGb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "MMC IPC Receive Checksum Offload Interrupt Mask maintains the mask for the interrupt generated from the receive IPC statistic counters."]
    #[inline(always)]
    pub const fn mmc_ipc_intr_mask_rx(
        self,
    ) -> crate::common::Reg<regs::MmcIpcIntrMaskRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MMC Receive Checksum Offload Interrupt maintains the interrupt that the receive IPC statistic counters generate. See Table 4-25 for further detail."]
    #[inline(always)]
    pub const fn mmc_ipc_intr_rx(
        self,
    ) -> crate::common::Reg<regs::MmcIpcIntrRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    #[inline(always)]
    pub const fn rxipv4_gd_fms(self) -> crate::common::Reg<regs::Rxipv4GdFms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn l3_l4_cfg(self, n: usize) -> L3L4Cfg {
        assert!(n < 1usize);
        unsafe { L3L4Cfg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 32usize) as _) }
    }
    #[doc = "VLAN Tag Inclusion or Replacement Register."]
    #[inline(always)]
    pub const fn vlan_tag_inc_rpl(
        self,
    ) -> crate::common::Reg<regs::VlanTagIncRpl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "VLAN Hash Table Register."]
    #[inline(always)]
    pub const fn vlan_hash(self) -> crate::common::Reg<regs::VlanHash, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "Timestamp Control Register."]
    #[inline(always)]
    pub const fn ts_ctrl(self) -> crate::common::Reg<regs::TsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "Sub-Second Increment Register."]
    #[inline(always)]
    pub const fn sub_sec_incr(self) -> crate::common::Reg<regs::SubSecIncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "System Time - Seconds Register."]
    #[inline(always)]
    pub const fn syst_sec(self) -> crate::common::Reg<regs::SystSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "System Time - Nanoseconds Register."]
    #[inline(always)]
    pub const fn syst_nsec(self) -> crate::common::Reg<regs::SystNsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "System Time - Seconds Update Register."]
    #[inline(always)]
    pub const fn syst_sec_upd(self) -> crate::common::Reg<regs::SystSecUpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "System Time - Nanoseconds Update Register."]
    #[inline(always)]
    pub const fn syst_nsec_upd(self) -> crate::common::Reg<regs::SystNsecUpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "Timestamp Addend Register."]
    #[inline(always)]
    pub const fn ts_addend(self) -> crate::common::Reg<regs::TsAddend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "Target Time Seconds Register."]
    #[inline(always)]
    pub const fn tgttm_sec(self) -> crate::common::Reg<regs::Enet0TgttmSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "Target Time Nanoseconds Register."]
    #[inline(always)]
    pub const fn tgttm_nsec(self) -> crate::common::Reg<regs::Enet0TgttmNsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "System Time - Higher Word Seconds Register."]
    #[inline(always)]
    pub const fn systm_h_sec(self) -> crate::common::Reg<regs::SystmHSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "Timestamp Status Register."]
    #[inline(always)]
    pub const fn ts_status(self) -> crate::common::Reg<regs::TsStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "PPS Control Register."]
    #[inline(always)]
    pub const fn pps_ctrl(self) -> crate::common::Reg<regs::PpsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "Auxiliary Timestamp - Nanoseconds Register."]
    #[inline(always)]
    pub const fn aux_ts_nsec(self) -> crate::common::Reg<regs::AuxTsNsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "Auxiliary Timestamp - Seconds Register."]
    #[inline(always)]
    pub const fn aux_ts_sec(self) -> crate::common::Reg<regs::AuxTsSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "PPS0 Interval Register."]
    #[inline(always)]
    pub const fn pps0_interval(self) -> crate::common::Reg<regs::Pps0Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "PPS0 Width Register."]
    #[inline(always)]
    pub const fn pps0_width(self) -> crate::common::Reg<regs::Pps0Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pps(self, n: usize) -> Pps {
        assert!(n < 3usize);
        unsafe { Pps::from_ptr(self.ptr.wrapping_add(0x0780usize + n * 32usize) as _) }
    }
    #[doc = "Bus Mode Register."]
    #[inline(always)]
    pub const fn dma_bus_mode(self) -> crate::common::Reg<regs::DmaBusMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Transmit Poll Demand Register."]
    #[inline(always)]
    pub const fn dma_tx_poll_demand(
        self,
    ) -> crate::common::Reg<regs::DmaTxPollDemand, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Receive Poll Demand Register."]
    #[inline(always)]
    pub const fn dma_rx_poll_demand(
        self,
    ) -> crate::common::Reg<regs::DmaRxPollDemand, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Receive Descriptor List Address Register."]
    #[inline(always)]
    pub const fn dma_rx_desc_list_addr(
        self,
    ) -> crate::common::Reg<regs::DmaRxDescListAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Transmit Descriptor List Address Register."]
    #[inline(always)]
    pub const fn dma_tx_desc_list_addr(
        self,
    ) -> crate::common::Reg<regs::DmaTxDescListAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn dma_status(self) -> crate::common::Reg<regs::DmaStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Operation Mode Register."]
    #[inline(always)]
    pub const fn dma_op_mode(self) -> crate::common::Reg<regs::DmaOpMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn dma_intr_en(self) -> crate::common::Reg<regs::DmaIntrEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Missed Frame And Buffer Overflow Counter Register."]
    #[inline(always)]
    pub const fn dma_miss_ovf_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaMissOvfCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Receive Interrupt Watchdog Timer Register."]
    #[inline(always)]
    pub const fn dma_rx_intr_wdog(
        self,
    ) -> crate::common::Reg<regs::DmaRxIntrWdog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "AXI Bus Mode Register."]
    #[inline(always)]
    pub const fn dma_axi_mode(self) -> crate::common::Reg<regs::DmaAxiMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "AHB or AXI Status Register."]
    #[inline(always)]
    pub const fn dma_bus_status(self) -> crate::common::Reg<regs::DmaBusStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Current Host Transmit Descriptor Register."]
    #[inline(always)]
    pub const fn dma_curr_host_tx_desc(
        self,
    ) -> crate::common::Reg<regs::DmaCurrHostTxDesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Current Host Receive Descriptor Register."]
    #[inline(always)]
    pub const fn dma_curr_host_rx_desc(
        self,
    ) -> crate::common::Reg<regs::DmaCurrHostRxDesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "Current Host Transmit Buffer Address Register."]
    #[inline(always)]
    pub const fn dma_curr_host_tx_buf(
        self,
    ) -> crate::common::Reg<regs::DmaCurrHostTxBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Current Host Receive Buffer Address Register."]
    #[inline(always)]
    pub const fn dma_curr_host_rx_buf(
        self,
    ) -> crate::common::Reg<regs::DmaCurrHostRxBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Control Register 0."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "Control Register 1."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3L4Cfg {
    ptr: *mut u8,
}
unsafe impl Send for L3L4Cfg {}
unsafe impl Sync for L3L4Cfg {}
impl L3L4Cfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Layer 3 and Layer 4 Control Register."]
    #[inline(always)]
    pub const fn l3_l4_ctrl(self) -> crate::common::Reg<regs::L3L4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Layer 4 Address Register."]
    #[inline(always)]
    pub const fn l4_addr(self) -> crate::common::Reg<regs::L4Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Layer 3 Address 0 Register."]
    #[inline(always)]
    pub const fn l3_addr_0(self) -> crate::common::Reg<regs::L3Addr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Layer 3 Address 1 Register."]
    #[inline(always)]
    pub const fn l3_addr_1(self) -> crate::common::Reg<regs::L3Addr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Layer 3 Address 2 Register."]
    #[inline(always)]
    pub const fn l3_addr_2(self) -> crate::common::Reg<regs::L3Addr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Layer 3 Address 3 Register."]
    #[inline(always)]
    pub const fn l3_addr_3(self) -> crate::common::Reg<regs::L3Addr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAddr {
    ptr: *mut u8,
}
unsafe impl Send for MacAddr {}
unsafe impl Sync for MacAddr {}
impl MacAddr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MAC Address High Register."]
    #[inline(always)]
    pub const fn high(self) -> crate::common::Reg<regs::High, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC Address Low Register."]
    #[inline(always)]
    pub const fn low(self) -> crate::common::Reg<regs::Low, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pps {
    ptr: *mut u8,
}
unsafe impl Send for Pps {}
unsafe impl Sync for Pps {}
impl Pps {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PPS Target Time Seconds Register."]
    #[inline(always)]
    pub const fn tgttm_sec(self) -> crate::common::Reg<regs::PpsTgttmSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PPS Target Time Nanoseconds Register."]
    #[inline(always)]
    pub const fn tgttm_nsec(self) -> crate::common::Reg<regs::PpsTgttmNsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PPS Interval Register."]
    #[inline(always)]
    pub const fn interval(self) -> crate::common::Reg<regs::Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PPS Width Register."]
    #[inline(always)]
    pub const fn width(self) -> crate::common::Reg<regs::Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
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
    #[doc = "Auxiliary Timestamp - Nanoseconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuxTsNsec(pub u32);
    impl AuxTsNsec {
        #[doc = "Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
        #[inline(always)]
        pub const fn set_auxtslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for AuxTsNsec {
        #[inline(always)]
        fn default() -> AuxTsNsec {
            AuxTsNsec(0)
        }
    }
    impl core::fmt::Debug for AuxTsNsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuxTsNsec")
                .field("auxtslo", &self.auxtslo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AuxTsNsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AuxTsNsec {{ auxtslo: {=u32:?} }}", self.auxtslo())
        }
    }
    #[doc = "Auxiliary Timestamp - Seconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuxTsSec(pub u32);
    impl AuxTsSec {
        #[doc = "Contains the lower 32 bits of the Seconds field of the auxiliary timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contains the lower 32 bits of the Seconds field of the auxiliary timestamp."]
        #[inline(always)]
        pub const fn set_auxtshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AuxTsSec {
        #[inline(always)]
        fn default() -> AuxTsSec {
            AuxTsSec(0)
        }
    }
    impl core::fmt::Debug for AuxTsSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuxTsSec")
                .field("auxtshi", &self.auxtshi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AuxTsSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AuxTsSec {{ auxtshi: {=u32:?} }}", self.auxtshi())
        }
    }
    #[doc = "Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    impl core::fmt::Debug for Ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl0")
                .field("enet0_txclk_dly_sel", &self.enet0_txclk_dly_sel())
                .field("enet0_rxclk_dly_sel", &self.enet0_rxclk_dly_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl0 {{ enet0_txclk_dly_sel: {=u8:?}, enet0_rxclk_dly_sel: {=u8:?} }}",
                self.enet0_txclk_dly_sel(),
                self.enet0_rxclk_dly_sel()
            )
        }
    }
    #[doc = "Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl2(pub u32);
    impl Ctrl2 {
        #[doc = "RMII mode output clock pad select set to use txck as RMII refclk; clr to use rxck as RMII refclk; default 0(rxck) refclk is always from pad, can use external clock from pad, or use internal clock output to pad then loopback."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RMII mode output clock pad select set to use txck as RMII refclk; clr to use rxck as RMII refclk; default 0(rxck) refclk is always from pad, can use external clock from pad, or use internal clock output to pad then loopback."]
        #[inline(always)]
        pub const fn set_enet0_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "flow control request."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_flowctrl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "flow control request."]
        #[inline(always)]
        pub const fn set_enet0_flowctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PHY mode select 000MII; 001RGMII; 100RMII; should be set before config IOMUX, otherwise may cause glitch for RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_phy_inf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "PHY mode select 000MII; 001RGMII; 100RMII; should be set before config IOMUX, otherwise may cause glitch for RGMII."]
        #[inline(always)]
        pub const fn set_enet0_phy_inf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "set to enable output 50MHz clock to rmii phy. User should set it if use soc internal clock as refclk."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_refclk_oe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable output 50MHz clock to rmii phy. User should set it if use soc internal clock as refclk."]
        #[inline(always)]
        pub const fn set_enet0_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "lowpower interrupt enable, for internal use only, user should use core registers for enable/disable interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_lpi_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "lowpower interrupt enable, for internal use only, user should use core registers for enable/disable interrupt."]
        #[inline(always)]
        pub const fn set_enet0_lpi_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl2 {
        #[inline(always)]
        fn default() -> Ctrl2 {
            Ctrl2(0)
        }
    }
    impl core::fmt::Debug for Ctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl2")
                .field("enet0_rmii_txclk_sel", &self.enet0_rmii_txclk_sel())
                .field("enet0_flowctrl", &self.enet0_flowctrl())
                .field("enet0_phy_inf_sel", &self.enet0_phy_inf_sel())
                .field("enet0_refclk_oe", &self.enet0_refclk_oe())
                .field("enet0_lpi_irq_en", &self.enet0_lpi_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl2 {{ enet0_rmii_txclk_sel: {=bool:?}, enet0_flowctrl: {=bool:?}, enet0_phy_inf_sel: {=u8:?}, enet0_refclk_oe: {=bool:?}, enet0_lpi_irq_en: {=bool:?} }}" , self . enet0_rmii_txclk_sel () , self . enet0_flowctrl () , self . enet0_phy_inf_sel () , self . enet0_refclk_oe () , self . enet0_lpi_irq_en ())
        }
    }
    #[doc = "AXI Bus Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaAxiMode(pub u32);
    impl DmaAxiMode {
        #[doc = "AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
        #[must_use]
        #[inline(always)]
        pub const fn undef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
        #[inline(always)]
        pub const fn set_undef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn blen4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
        #[inline(always)]
        pub const fn set_blen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn blen8(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
        #[inline(always)]
        pub const fn set_blen8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
        #[must_use]
        #[inline(always)]
        pub const fn blen16(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
        #[inline(always)]
        pub const fn set_blen16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn blen32(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[inline(always)]
        pub const fn set_blen32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn blen64(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[inline(always)]
        pub const fn set_blen64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn blen128(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
        #[inline(always)]
        pub const fn set_blen128(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn blen256(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
        #[inline(always)]
        pub const fn set_blen256(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_aal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
        #[inline(always)]
        pub const fn set_axi_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn onekbbe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
        #[inline(always)]
        pub const fn set_onekbbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_osr_lmt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
        #[inline(always)]
        pub const fn set_rd_osr_lmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_osr_lmt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
        #[inline(always)]
        pub const fn set_wr_osr_lmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
        #[must_use]
        #[inline(always)]
        pub const fn lpi_xit_frm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
        #[inline(always)]
        pub const fn set_lpi_xit_frm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
        #[must_use]
        #[inline(always)]
        pub const fn en_lpi(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
        #[inline(always)]
        pub const fn set_en_lpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DmaAxiMode {
        #[inline(always)]
        fn default() -> DmaAxiMode {
            DmaAxiMode(0)
        }
    }
    impl core::fmt::Debug for DmaAxiMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaAxiMode")
                .field("undef", &self.undef())
                .field("blen4", &self.blen4())
                .field("blen8", &self.blen8())
                .field("blen16", &self.blen16())
                .field("blen32", &self.blen32())
                .field("blen64", &self.blen64())
                .field("blen128", &self.blen128())
                .field("blen256", &self.blen256())
                .field("axi_aal", &self.axi_aal())
                .field("onekbbe", &self.onekbbe())
                .field("rd_osr_lmt", &self.rd_osr_lmt())
                .field("wr_osr_lmt", &self.wr_osr_lmt())
                .field("lpi_xit_frm", &self.lpi_xit_frm())
                .field("en_lpi", &self.en_lpi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaAxiMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaAxiMode {{ undef: {=bool:?}, blen4: {=bool:?}, blen8: {=bool:?}, blen16: {=bool:?}, blen32: {=bool:?}, blen64: {=bool:?}, blen128: {=bool:?}, blen256: {=bool:?}, axi_aal: {=bool:?}, onekbbe: {=bool:?}, rd_osr_lmt: {=u8:?}, wr_osr_lmt: {=u8:?}, lpi_xit_frm: {=bool:?}, en_lpi: {=bool:?} }}" , self . undef () , self . blen4 () , self . blen8 () , self . blen16 () , self . blen32 () , self . blen64 () , self . blen128 () , self . blen256 () , self . axi_aal () , self . onekbbe () , self . rd_osr_lmt () , self . wr_osr_lmt () , self . lpi_xit_frm () , self . en_lpi ())
        }
    }
    #[doc = "Bus Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBusMode(pub u32);
    impl DmaBusMode {
        #[doc = "Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
        #[must_use]
        #[inline(always)]
        pub const fn swr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
        #[inline(always)]
        pub const fn set_swr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
        #[inline(always)]
        pub const fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
        #[must_use]
        #[inline(always)]
        pub const fn atds(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
        #[inline(always)]
        pub const fn set_atds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
        #[must_use]
        #[inline(always)]
        pub const fn pbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
        #[inline(always)]
        pub const fn set_pbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
        #[must_use]
        #[inline(always)]
        pub const fn fb(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
        #[inline(always)]
        pub const fn set_fb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn rpbl(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
        #[inline(always)]
        pub const fn set_rpbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
        #[doc = "Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
        #[must_use]
        #[inline(always)]
        pub const fn usp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
        #[inline(always)]
        pub const fn set_usp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
        #[must_use]
        #[inline(always)]
        pub const fn pblx8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
        #[inline(always)]
        pub const fn set_pblx8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
        #[must_use]
        #[inline(always)]
        pub const fn aal(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
        #[inline(always)]
        pub const fn set_aal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
        #[must_use]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
        #[inline(always)]
        pub const fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn txpr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
        #[inline(always)]
        pub const fn set_txpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn prwg(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
        #[inline(always)]
        pub const fn set_prwg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
        #[must_use]
        #[inline(always)]
        pub const fn rib(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
        #[inline(always)]
        pub const fn set_rib(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DmaBusMode {
        #[inline(always)]
        fn default() -> DmaBusMode {
            DmaBusMode(0)
        }
    }
    impl core::fmt::Debug for DmaBusMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBusMode")
                .field("swr", &self.swr())
                .field("da", &self.da())
                .field("dsl", &self.dsl())
                .field("atds", &self.atds())
                .field("pbl", &self.pbl())
                .field("pr", &self.pr())
                .field("fb", &self.fb())
                .field("rpbl", &self.rpbl())
                .field("usp", &self.usp())
                .field("pblx8", &self.pblx8())
                .field("aal", &self.aal())
                .field("mb", &self.mb())
                .field("txpr", &self.txpr())
                .field("prwg", &self.prwg())
                .field("rib", &self.rib())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaBusMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaBusMode {{ swr: {=bool:?}, da: {=bool:?}, dsl: {=u8:?}, atds: {=bool:?}, pbl: {=u8:?}, pr: {=u8:?}, fb: {=bool:?}, rpbl: {=u8:?}, usp: {=bool:?}, pblx8: {=bool:?}, aal: {=bool:?}, mb: {=bool:?}, txpr: {=bool:?}, prwg: {=u8:?}, rib: {=bool:?} }}" , self . swr () , self . da () , self . dsl () , self . atds () , self . pbl () , self . pr () , self . fb () , self . rpbl () , self . usp () , self . pblx8 () , self . aal () , self . mb () , self . txpr () , self . prwg () , self . rib ())
        }
    }
    #[doc = "AHB or AXI Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBusStatus(pub u32);
    impl DmaBusStatus {
        #[doc = "AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
        #[must_use]
        #[inline(always)]
        pub const fn axwhsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
        #[inline(always)]
        pub const fn set_axwhsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
        #[must_use]
        #[inline(always)]
        pub const fn axirdsts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
        #[inline(always)]
        pub const fn set_axirdsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DmaBusStatus {
        #[inline(always)]
        fn default() -> DmaBusStatus {
            DmaBusStatus(0)
        }
    }
    impl core::fmt::Debug for DmaBusStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBusStatus")
                .field("axwhsts", &self.axwhsts())
                .field("axirdsts", &self.axirdsts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaBusStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmaBusStatus {{ axwhsts: {=bool:?}, axirdsts: {=bool:?} }}",
                self.axwhsts(),
                self.axirdsts()
            )
        }
    }
    #[doc = "Current Host Receive Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCurrHostRxBuf(pub u32);
    impl DmaCurrHostRxBuf {
        #[doc = "Host Receive Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[must_use]
        #[inline(always)]
        pub const fn currbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host Receive Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[inline(always)]
        pub const fn set_currbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaCurrHostRxBuf {
        #[inline(always)]
        fn default() -> DmaCurrHostRxBuf {
            DmaCurrHostRxBuf(0)
        }
    }
    impl core::fmt::Debug for DmaCurrHostRxBuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCurrHostRxBuf")
                .field("currbufaptr", &self.currbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaCurrHostRxBuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmaCurrHostRxBuf {{ currbufaptr: {=u32:?} }}",
                self.currbufaptr()
            )
        }
    }
    #[doc = "Current Host Receive Descriptor Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCurrHostRxDesc(pub u32);
    impl DmaCurrHostRxDesc {
        #[doc = "Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[must_use]
        #[inline(always)]
        pub const fn currdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[inline(always)]
        pub const fn set_currdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaCurrHostRxDesc {
        #[inline(always)]
        fn default() -> DmaCurrHostRxDesc {
            DmaCurrHostRxDesc(0)
        }
    }
    impl core::fmt::Debug for DmaCurrHostRxDesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCurrHostRxDesc")
                .field("currdesaptr", &self.currdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaCurrHostRxDesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmaCurrHostRxDesc {{ currdesaptr: {=u32:?} }}",
                self.currdesaptr()
            )
        }
    }
    #[doc = "Current Host Transmit Buffer Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCurrHostTxBuf(pub u32);
    impl DmaCurrHostTxBuf {
        #[doc = "Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[must_use]
        #[inline(always)]
        pub const fn curtbufaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[inline(always)]
        pub const fn set_curtbufaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaCurrHostTxBuf {
        #[inline(always)]
        fn default() -> DmaCurrHostTxBuf {
            DmaCurrHostTxBuf(0)
        }
    }
    impl core::fmt::Debug for DmaCurrHostTxBuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCurrHostTxBuf")
                .field("curtbufaptr", &self.curtbufaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaCurrHostTxBuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmaCurrHostTxBuf {{ curtbufaptr: {=u32:?} }}",
                self.curtbufaptr()
            )
        }
    }
    #[doc = "Current Host Transmit Descriptor Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCurrHostTxDesc(pub u32);
    impl DmaCurrHostTxDesc {
        #[doc = "Host Transmit Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[must_use]
        #[inline(always)]
        pub const fn curtdesaptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host Transmit Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
        #[inline(always)]
        pub const fn set_curtdesaptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaCurrHostTxDesc {
        #[inline(always)]
        fn default() -> DmaCurrHostTxDesc {
            DmaCurrHostTxDesc(0)
        }
    }
    impl core::fmt::Debug for DmaCurrHostTxDesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCurrHostTxDesc")
                .field("curtdesaptr", &self.curtdesaptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaCurrHostTxDesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DmaCurrHostTxDesc {{ curtdesaptr: {=u32:?} }}",
                self.curtdesaptr()
            )
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaIntrEn(pub u32);
    impl DmaIntrEn {
        #[doc = "Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmission Stopped Interrupt is enabled. When this bit is reset, the Transmission Stopped Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmission Stopped Interrupt is enabled. When this bit is reset, the Transmission Stopped Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tue(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_tue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tje(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_tje(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Overflow Interrupt is enabled. When this bit is reset, the Overflow Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ove(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Overflow Interrupt is enabled. When this bit is reset, the Overflow Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_ove(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmit Underflow Interrupt is enabled. When this bit is reset, the Underflow Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn une(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Transmit Underflow Interrupt is enabled. When this bit is reset, the Underflow Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_une(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_rue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_rse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rwe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_rwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (Bit 15), the Early Transmit Interrupt is enabled. When this bit is reset, the Early Transmit Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ete(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable (Bit 15), the Early Transmit Interrupt is enabled. When this bit is reset, the Early Transmit Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_ete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn fbe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable (Bit 15), the Fatal Bus Error Interrupt is enabled. When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_fbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Early Receive Interrupt is enabled. When this bit is reset, the Early Receive Interrupt is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ere(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable (Bit 16), the Early Receive Interrupt is enabled. When this bit is reset, the Early Receive Interrupt is disabled."]
        #[inline(always)]
        pub const fn set_ere(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled. When this bit is reset, the abnormal interrupt summary is disabled. This bit enables the following interrupts in Register 5 (Status Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error."]
        #[must_use]
        #[inline(always)]
        pub const fn aie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled. When this bit is reset, the abnormal interrupt summary is disabled. This bit enables the following interrupts in Register 5 (Status Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error."]
        #[inline(always)]
        pub const fn set_aie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled. When this bit is reset, normal interrupt summary is disabled. This bit enables the following interrupts in Register 5 (Status Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn nie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled. When this bit is reset, normal interrupt summary is disabled. This bit enables the following interrupts in Register 5 (Status Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt."]
        #[inline(always)]
        pub const fn set_nie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for DmaIntrEn {
        #[inline(always)]
        fn default() -> DmaIntrEn {
            DmaIntrEn(0)
        }
    }
    impl core::fmt::Debug for DmaIntrEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaIntrEn")
                .field("tie", &self.tie())
                .field("tse", &self.tse())
                .field("tue", &self.tue())
                .field("tje", &self.tje())
                .field("ove", &self.ove())
                .field("une", &self.une())
                .field("rie", &self.rie())
                .field("rue", &self.rue())
                .field("rse", &self.rse())
                .field("rwe", &self.rwe())
                .field("ete", &self.ete())
                .field("fbe", &self.fbe())
                .field("ere", &self.ere())
                .field("aie", &self.aie())
                .field("nie", &self.nie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaIntrEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaIntrEn {{ tie: {=bool:?}, tse: {=bool:?}, tue: {=bool:?}, tje: {=bool:?}, ove: {=bool:?}, une: {=bool:?}, rie: {=bool:?}, rue: {=bool:?}, rse: {=bool:?}, rwe: {=bool:?}, ete: {=bool:?}, fbe: {=bool:?}, ere: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}" , self . tie () , self . tse () , self . tue () , self . tje () , self . ove () , self . une () , self . rie () , self . rue () , self . rse () , self . rwe () , self . ete () , self . fbe () , self . ere () , self . aie () , self . nie ())
        }
    }
    #[doc = "Missed Frame And Buffer Overflow Counter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaMissOvfCnt(pub u32);
    impl DmaMissOvfCnt {
        #[doc = "Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
        #[must_use]
        #[inline(always)]
        pub const fn misfrmcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
        #[inline(always)]
        pub const fn set_misfrmcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
        #[must_use]
        #[inline(always)]
        pub const fn miscntovf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
        #[inline(always)]
        pub const fn set_miscntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
        #[must_use]
        #[inline(always)]
        pub const fn ovffrmcnt(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
        #[inline(always)]
        pub const fn set_ovffrmcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
        #[must_use]
        #[inline(always)]
        pub const fn onfcntovf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
        #[inline(always)]
        pub const fn set_onfcntovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DmaMissOvfCnt {
        #[inline(always)]
        fn default() -> DmaMissOvfCnt {
            DmaMissOvfCnt(0)
        }
    }
    impl core::fmt::Debug for DmaMissOvfCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaMissOvfCnt")
                .field("misfrmcnt", &self.misfrmcnt())
                .field("miscntovf", &self.miscntovf())
                .field("ovffrmcnt", &self.ovffrmcnt())
                .field("onfcntovf", &self.onfcntovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaMissOvfCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaMissOvfCnt {{ misfrmcnt: {=u16:?}, miscntovf: {=bool:?}, ovffrmcnt: {=u16:?}, onfcntovf: {=bool:?} }}" , self . misfrmcnt () , self . miscntovf () , self . ovffrmcnt () , self . onfcntovf ())
        }
    }
    #[doc = "Operation Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaOpMode(pub u32);
    impl DmaOpMode {
        #[doc = "Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
        #[must_use]
        #[inline(always)]
        pub const fn osf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
        #[inline(always)]
        pub const fn set_osf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128."]
        #[inline(always)]
        pub const fn set_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
        #[must_use]
        #[inline(always)]
        pub const fn dgf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
        #[inline(always)]
        pub const fn set_dgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
        #[must_use]
        #[inline(always)]
        pub const fn fuf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
        #[inline(always)]
        pub const fn set_fuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
        #[must_use]
        #[inline(always)]
        pub const fn fef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
        #[inline(always)]
        pub const fn set_fef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
        #[must_use]
        #[inline(always)]
        pub const fn efc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
        #[inline(always)]
        pub const fn set_efc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition."]
        #[must_use]
        #[inline(always)]
        pub const fn rfa(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition."]
        #[inline(always)]
        pub const fn set_rfa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
        #[must_use]
        #[inline(always)]
        pub const fn rfd(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
        #[inline(always)]
        pub const fn set_rfd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16."]
        #[must_use]
        #[inline(always)]
        pub const fn ttc(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16."]
        #[inline(always)]
        pub const fn set_ttc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
        #[inline(always)]
        pub const fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
        #[must_use]
        #[inline(always)]
        pub const fn rfd_2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
        #[inline(always)]
        pub const fn set_rfd_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
        #[must_use]
        #[inline(always)]
        pub const fn rfa_2(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
        #[inline(always)]
        pub const fn set_rfa_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)."]
        #[must_use]
        #[inline(always)]
        pub const fn dff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)."]
        #[inline(always)]
        pub const fn set_dff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
        #[must_use]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
        #[inline(always)]
        pub const fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
        #[must_use]
        #[inline(always)]
        pub const fn dt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
        #[inline(always)]
        pub const fn set_dt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DmaOpMode {
        #[inline(always)]
        fn default() -> DmaOpMode {
            DmaOpMode(0)
        }
    }
    impl core::fmt::Debug for DmaOpMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaOpMode")
                .field("sr", &self.sr())
                .field("osf", &self.osf())
                .field("rtc", &self.rtc())
                .field("dgf", &self.dgf())
                .field("fuf", &self.fuf())
                .field("fef", &self.fef())
                .field("efc", &self.efc())
                .field("rfa", &self.rfa())
                .field("rfd", &self.rfd())
                .field("st", &self.st())
                .field("ttc", &self.ttc())
                .field("ftf", &self.ftf())
                .field("tsf", &self.tsf())
                .field("rfd_2", &self.rfd_2())
                .field("rfa_2", &self.rfa_2())
                .field("dff", &self.dff())
                .field("rsf", &self.rsf())
                .field("dt", &self.dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaOpMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaOpMode {{ sr: {=bool:?}, osf: {=bool:?}, rtc: {=u8:?}, dgf: {=bool:?}, fuf: {=bool:?}, fef: {=bool:?}, efc: {=bool:?}, rfa: {=u8:?}, rfd: {=u8:?}, st: {=bool:?}, ttc: {=u8:?}, ftf: {=bool:?}, tsf: {=bool:?}, rfd_2: {=bool:?}, rfa_2: {=bool:?}, dff: {=bool:?}, rsf: {=bool:?}, dt: {=bool:?} }}" , self . sr () , self . osf () , self . rtc () , self . dgf () , self . fuf () , self . fef () , self . efc () , self . rfa () , self . rfd () , self . st () , self . ttc () , self . ftf () , self . tsf () , self . rfd_2 () , self . rfa_2 () , self . dff () , self . rsf () , self . dt ())
        }
    }
    #[doc = "Receive Descriptor List Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRxDescListAddr(pub u32);
    impl DmaRxDescListAddr {
        #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn rdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Receive List This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB bits (1:0, 2:0, or 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
        #[inline(always)]
        pub const fn set_rdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaRxDescListAddr {
        #[inline(always)]
        fn default() -> DmaRxDescListAddr {
            DmaRxDescListAddr(0)
        }
    }
    impl core::fmt::Debug for DmaRxDescListAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRxDescListAddr")
                .field("rdesla", &self.rdesla())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaRxDescListAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaRxDescListAddr {{ rdesla: {=u32:?} }}", self.rdesla())
        }
    }
    #[doc = "Receive Interrupt Watchdog Timer Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRxIntrWdog(pub u32);
    impl DmaRxIntrWdog {
        #[doc = "RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
        #[must_use]
        #[inline(always)]
        pub const fn riwt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RI Watchdog Timer Count This bit indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the Rx DMA completes the transfer of a frame for which the RI status bit is not set because of the setting in the corresponding descriptor RDES1\\[31\\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
        #[inline(always)]
        pub const fn set_riwt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for DmaRxIntrWdog {
        #[inline(always)]
        fn default() -> DmaRxIntrWdog {
            DmaRxIntrWdog(0)
        }
    }
    impl core::fmt::Debug for DmaRxIntrWdog {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRxIntrWdog")
                .field("riwt", &self.riwt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaRxIntrWdog {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaRxIntrWdog {{ riwt: {=u8:?} }}", self.riwt())
        }
    }
    #[doc = "Receive Poll Demand Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaRxPollDemand(pub u32);
    impl DmaRxPollDemand {
        #[doc = "Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
        #[must_use]
        #[inline(always)]
        pub const fn rpd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
        #[inline(always)]
        pub const fn set_rpd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaRxPollDemand {
        #[inline(always)]
        fn default() -> DmaRxPollDemand {
            DmaRxPollDemand(0)
        }
    }
    impl core::fmt::Debug for DmaRxPollDemand {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaRxPollDemand")
                .field("rpd", &self.rpd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaRxPollDemand {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaRxPollDemand {{ rpd: {=u32:?} }}", self.rpd())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaStatus(pub u32);
    impl DmaStatus {
        #[doc = "Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
        #[must_use]
        #[inline(always)]
        pub const fn ti(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
        #[inline(always)]
        pub const fn set_ti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Process Stopped This bit is set when the transmission is stopped."]
        #[inline(always)]
        pub const fn set_tps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
        #[must_use]
        #[inline(always)]
        pub const fn tu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
        #[inline(always)]
        pub const fn set_tu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
        #[must_use]
        #[inline(always)]
        pub const fn tjt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
        #[inline(always)]
        pub const fn set_tjt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
        #[must_use]
        #[inline(always)]
        pub const fn unf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
        #[inline(always)]
        pub const fn set_unf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
        #[must_use]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
        #[inline(always)]
        pub const fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn ru(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
        #[inline(always)]
        pub const fn set_ru(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
        #[inline(always)]
        pub const fn set_rps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn rwt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
        #[inline(always)]
        pub const fn set_rwt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn eti(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
        #[inline(always)]
        pub const fn set_eti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn fbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
        #[inline(always)]
        pub const fn set_fbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
        #[must_use]
        #[inline(always)]
        pub const fn eri(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
        #[inline(always)]
        pub const fn set_eri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn eb(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
        #[inline(always)]
        pub const fn set_eb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
        #[must_use]
        #[inline(always)]
        pub const fn gli(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
        #[inline(always)]
        pub const fn set_gli(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn gmi(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
        #[inline(always)]
        pub const fn set_gmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
        #[must_use]
        #[inline(always)]
        pub const fn gpi(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
        #[inline(always)]
        pub const fn set_gpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn tti(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
        #[inline(always)]
        pub const fn set_tti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
        #[must_use]
        #[inline(always)]
        pub const fn glpii(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
        #[inline(always)]
        pub const fn set_glpii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for DmaStatus {
        #[inline(always)]
        fn default() -> DmaStatus {
            DmaStatus(0)
        }
    }
    impl core::fmt::Debug for DmaStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaStatus")
                .field("ti", &self.ti())
                .field("tps", &self.tps())
                .field("tu", &self.tu())
                .field("tjt", &self.tjt())
                .field("ovf", &self.ovf())
                .field("unf", &self.unf())
                .field("ri", &self.ri())
                .field("ru", &self.ru())
                .field("rps", &self.rps())
                .field("rwt", &self.rwt())
                .field("eti", &self.eti())
                .field("fbi", &self.fbi())
                .field("eri", &self.eri())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("rs", &self.rs())
                .field("ts", &self.ts())
                .field("eb", &self.eb())
                .field("gli", &self.gli())
                .field("gmi", &self.gmi())
                .field("gpi", &self.gpi())
                .field("tti", &self.tti())
                .field("glpii", &self.glpii())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DmaStatus {{ ti: {=bool:?}, tps: {=bool:?}, tu: {=bool:?}, tjt: {=bool:?}, ovf: {=bool:?}, unf: {=bool:?}, ri: {=bool:?}, ru: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, fbi: {=bool:?}, eri: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, rs: {=u8:?}, ts: {=u8:?}, eb: {=u8:?}, gli: {=bool:?}, gmi: {=bool:?}, gpi: {=bool:?}, tti: {=bool:?}, glpii: {=bool:?} }}" , self . ti () , self . tps () , self . tu () , self . tjt () , self . ovf () , self . unf () , self . ri () , self . ru () , self . rps () , self . rwt () , self . eti () , self . fbi () , self . eri () , self . ais () , self . nis () , self . rs () , self . ts () , self . eb () , self . gli () , self . gmi () , self . gpi () , self . tti () , self . glpii ())
        }
    }
    #[doc = "Transmit Descriptor List Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTxDescListAddr(pub u32);
    impl DmaTxDescListAddr {
        #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB bits (1:0, 2:0, 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and are internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
        #[must_use]
        #[inline(always)]
        pub const fn tdesla(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of Transmit List This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB bits (1:0, 2:0, 3:0) for 32-bit, 64-bit, or 128-bit bus width are ignored and are internally taken as all-zero by the DMA. Therefore, these LSB bits are read-only (RO)."]
        #[inline(always)]
        pub const fn set_tdesla(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaTxDescListAddr {
        #[inline(always)]
        fn default() -> DmaTxDescListAddr {
            DmaTxDescListAddr(0)
        }
    }
    impl core::fmt::Debug for DmaTxDescListAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTxDescListAddr")
                .field("tdesla", &self.tdesla())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaTxDescListAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaTxDescListAddr {{ tdesla: {=u32:?} }}", self.tdesla())
        }
    }
    #[doc = "Transmit Poll Demand Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaTxPollDemand(pub u32);
    impl DmaTxPollDemand {
        #[doc = "Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
        #[must_use]
        #[inline(always)]
        pub const fn tpd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
        #[inline(always)]
        pub const fn set_tpd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DmaTxPollDemand {
        #[inline(always)]
        fn default() -> DmaTxPollDemand {
            DmaTxPollDemand(0)
        }
    }
    impl core::fmt::Debug for DmaTxPollDemand {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaTxPollDemand")
                .field("tpd", &self.tpd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmaTxPollDemand {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmaTxPollDemand {{ tpd: {=u32:?} }}", self.tpd())
        }
    }
    #[doc = "Target Time Nanoseconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Enet0TgttmNsec(pub u32);
    impl Enet0TgttmNsec {
        #[doc = "Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
        #[must_use]
        #[inline(always)]
        pub const fn ttslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Target Timestamp Low Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL0 field (Bits \\[6:5\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
        #[inline(always)]
        pub const fn set_ttslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtbusy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Target Time Register Busy The MAC sets this bit when the PPSCMD field (Bit \\[3:0\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD field to 010 or 011, instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted. This bit is reserved when the Enable Flexible Pulse-Per-Second Output feature is not selected."]
        #[inline(always)]
        pub const fn set_trgtbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Enet0TgttmNsec {
        #[inline(always)]
        fn default() -> Enet0TgttmNsec {
            Enet0TgttmNsec(0)
        }
    }
    impl core::fmt::Debug for Enet0TgttmNsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Enet0TgttmNsec")
                .field("ttslo", &self.ttslo())
                .field("trgtbusy", &self.trgtbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Enet0TgttmNsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Enet0TgttmNsec {{ ttslo: {=u32:?}, trgtbusy: {=bool:?} }}",
                self.ttslo(),
                self.trgtbusy()
            )
        }
    }
    #[doc = "Target Time Seconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Enet0TgttmSec(pub u32);
    impl Enet0TgttmSec {
        #[doc = "Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
        #[must_use]
        #[inline(always)]
        pub const fn tstr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[6:5\\]
of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
        #[inline(always)]
        pub const fn set_tstr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Enet0TgttmSec {
        #[inline(always)]
        fn default() -> Enet0TgttmSec {
            Enet0TgttmSec(0)
        }
    }
    impl core::fmt::Debug for Enet0TgttmSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Enet0TgttmSec")
                .field("tstr", &self.tstr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Enet0TgttmSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Enet0TgttmSec {{ tstr: {=u32:?} }}", self.tstr())
        }
    }
    #[doc = "Flow Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flowctrl(pub u32);
    impl Flowctrl {
        #[doc = "Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn fcb_bpa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
        #[inline(always)]
        pub const fn set_fcb_bpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
        #[inline(always)]
        pub const fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
        #[must_use]
        #[inline(always)]
        pub const fn plt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
        #[inline(always)]
        pub const fn set_plt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dzpq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
        #[inline(always)]
        pub const fn set_dzpq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Flowctrl {
        #[inline(always)]
        fn default() -> Flowctrl {
            Flowctrl(0)
        }
    }
    impl core::fmt::Debug for Flowctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flowctrl")
                .field("fcb_bpa", &self.fcb_bpa())
                .field("tfe", &self.tfe())
                .field("rfe", &self.rfe())
                .field("up", &self.up())
                .field("plt", &self.plt())
                .field("dzpq", &self.dzpq())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flowctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Flowctrl {{ fcb_bpa: {=bool:?}, tfe: {=bool:?}, rfe: {=bool:?}, up: {=bool:?}, plt: {=u8:?}, dzpq: {=bool:?}, pt: {=u16:?} }}" , self . fcb_bpa () , self . tfe () , self . rfe () , self . up () , self . plt () , self . dzpq () , self . pt ())
        }
    }
    #[doc = "GMII Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GmiiAddr(pub u32);
    impl GmiiAddr {
        #[doc = "GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
        #[must_use]
        #[inline(always)]
        pub const fn gb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
        #[inline(always)]
        pub const fn set_gb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
        #[must_use]
        #[inline(always)]
        pub const fn gw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
        #[inline(always)]
        pub const fn set_gw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
        #[must_use]
        #[inline(always)]
        pub const fn gr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
        #[inline(always)]
        pub const fn set_gr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for GmiiAddr {
        #[inline(always)]
        fn default() -> GmiiAddr {
            GmiiAddr(0)
        }
    }
    impl core::fmt::Debug for GmiiAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GmiiAddr")
                .field("gb", &self.gb())
                .field("gw", &self.gw())
                .field("cr", &self.cr())
                .field("gr", &self.gr())
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GmiiAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GmiiAddr {{ gb: {=bool:?}, gw: {=bool:?}, cr: {=u8:?}, gr: {=u8:?}, pa: {=u8:?} }}" , self . gb () , self . gw () , self . cr () , self . gr () , self . pa ())
        }
    }
    #[doc = "GMII Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GmiiData(pub u32);
    impl GmiiData {
        #[doc = "GMII Data This field contains the 16-bit data value read from the PHY or RevMII after a Management Read operation or the 16-bit data value to be written to the PHY or RevMII before a Management Write operation."]
        #[must_use]
        #[inline(always)]
        pub const fn gd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GMII Data This field contains the 16-bit data value read from the PHY or RevMII after a Management Read operation or the 16-bit data value to be written to the PHY or RevMII before a Management Write operation."]
        #[inline(always)]
        pub const fn set_gd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for GmiiData {
        #[inline(always)]
        fn default() -> GmiiData {
            GmiiData(0)
        }
    }
    impl core::fmt::Debug for GmiiData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GmiiData").field("gd", &self.gd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GmiiData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GmiiData {{ gd: {=u16:?} }}", self.gd())
        }
    }
    #[doc = "Hash Table High Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashH(pub u32);
    impl HashH {
        #[doc = "Hash Table High This field contains the upper 32 bits of the Hash table."]
        #[must_use]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash Table High This field contains the upper 32 bits of the Hash table."]
        #[inline(always)]
        pub const fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HashH {
        #[inline(always)]
        fn default() -> HashH {
            HashH(0)
        }
    }
    impl core::fmt::Debug for HashH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashH").field("hth", &self.hth()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HashH {{ hth: {=u32:?} }}", self.hth())
        }
    }
    #[doc = "Hash Table Low Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HashL(pub u32);
    impl HashL {
        #[doc = "Hash Table Low This field contains the lower 32 bits of the Hash table."]
        #[must_use]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash Table Low This field contains the lower 32 bits of the Hash table."]
        #[inline(always)]
        pub const fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HashL {
        #[inline(always)]
        fn default() -> HashL {
            HashL(0)
        }
    }
    impl core::fmt::Debug for HashL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HashL").field("htl", &self.htl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HashL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HashL {{ htl: {=u32:?} }}", self.htl())
        }
    }
    #[doc = "MAC Address High Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct High(pub u32);
    impl High {
        #[doc = "MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC Address1 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the second 6-byte MAC address."]
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes. When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers. Each bit controls the masking of the bytes as follows: - Bit 29: Register 18\\[15:8\\]
- Bit 28: Register 18\\[7:0\\]
- Bit 27: Register 19\\[31:24\\]
- ... - Bit 24: Register 19\\[7:0\\]
You can filter a group of addresses (known as group address filtering) by masking one or more bytes of the address."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address When this bit is set, the MAC Address1\\[47:0\\]
is used to compare with the SA fields of the received frame. When this bit is reset, the MAC Address1\\[47:0\\]
is used to compare with the DA fields of the received frame."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable When this bit is set, the address filter module uses the second MAC address for perfect filtering. When this bit is reset, the address filter module ignores the address for filtering."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for High {
        #[inline(always)]
        fn default() -> High {
            High(0)
        }
    }
    impl core::fmt::Debug for High {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("High")
                .field("addrhi", &self.addrhi())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for High {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "High {{ addrhi: {=u16:?}, mbc: {=u8:?}, sa: {=bool:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.mbc(),
                self.sa(),
                self.ae()
            )
        }
    }
    #[doc = "PPS Interval Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Interval(pub u32);
    impl Interval {
        #[doc = "PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Interval {
        #[inline(always)]
        fn default() -> Interval {
            Interval(0)
        }
    }
    impl core::fmt::Debug for Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Interval {{ ppsint: {=u32:?} }}", self.ppsint())
        }
    }
    #[doc = "Interrupt Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntrMask(pub u32);
    impl IntrMask {
        #[doc = "RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn rgsmiiim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_rgsmiiim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn pcslchgim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_pcslchgim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn pcsancim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_pcsancim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_pmtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn tsim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_tsim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
        #[inline(always)]
        pub const fn set_lpiim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for IntrMask {
        #[inline(always)]
        fn default() -> IntrMask {
            IntrMask(0)
        }
    }
    impl core::fmt::Debug for IntrMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntrMask")
                .field("rgsmiiim", &self.rgsmiiim())
                .field("pcslchgim", &self.pcslchgim())
                .field("pcsancim", &self.pcsancim())
                .field("pmtim", &self.pmtim())
                .field("tsim", &self.tsim())
                .field("lpiim", &self.lpiim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntrMask {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntrMask {{ rgsmiiim: {=bool:?}, pcslchgim: {=bool:?}, pcsancim: {=bool:?}, pmtim: {=bool:?}, tsim: {=bool:?}, lpiim: {=bool:?} }}" , self . rgsmiiim () , self . pcslchgim () , self . pcsancim () , self . pmtim () , self . tsim () , self . lpiim ())
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntrStatus(pub u32);
    impl IntrStatus {
        #[doc = "RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface (Bit 3 in Register 54 (SGMII/RGMII/SMII Control and Status Register)). This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register."]
        #[must_use]
        #[inline(always)]
        pub const fn rgsmiiis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface (Bit 3 in Register 54 (SGMII/RGMII/SMII Control and Status Register)). This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register."]
        #[inline(always)]
        pub const fn set_rgsmiiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface (Bit 2 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation on the AN Status register."]
        #[must_use]
        #[inline(always)]
        pub const fn pcslchgis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface (Bit 2 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation on the AN Status register."]
        #[inline(always)]
        pub const fn set_pcslchgis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PCS Auto-Negotiation Complete This bit is set when the Auto-negotiation is completed in the TBI, RTBI, or SGMII PHY interface (Bit 5 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation to the AN Status register."]
        #[must_use]
        #[inline(always)]
        pub const fn pcsancis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PCS Auto-Negotiation Complete This bit is set when the Auto-negotiation is completed in the TBI, RTBI, or SGMII PHY interface (Bit 5 in Register 49 (AN Status Register)). This bit is cleared when you perform a read operation to the AN Status register."]
        #[inline(always)]
        pub const fn set_pcsancis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PMT Interrupt Status This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bits 5 and 6 in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\]
are cleared because of a read operation to the PMT Control and Status register."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT Interrupt Status This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bits 5 and 6 in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\]
are cleared because of a read operation to the PMT Control and Status register."]
        #[inline(always)]
        pub const fn set_pmtis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\]
is set high and cleared only when all of these bits are low."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\]
is set high and cleared only when all of these bits are low."]
        #[inline(always)]
        pub const fn set_mmcis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrxis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[inline(always)]
        pub const fn set_mmcrxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn mmctxis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[inline(always)]
        pub const fn set_mmctxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrxipis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register. This bit is cleared when all the bits in this interrupt register are cleared."]
        #[inline(always)]
        pub const fn set_mmcrxipis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when any of the following conditions is true: - The system time value equals or exceeds the value specified in the Target Time High and Low registers. - There is an overflow in the seconds register. - The Auxiliary snapshot trigger is asserted. This bit is cleared on reading Bit 0 of Register 458 (Timestamp Status Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn tsis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when any of the following conditions is true: - The system time value equals or exceeds the value specified in the Target Time High and Low registers. - There is an overflow in the seconds register. - The Auxiliary snapshot trigger is asserted. This bit is cleared on reading Bit 0 of Register 458 (Timestamp Status Register)."]
        #[inline(always)]
        pub const fn set_tsis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit 0 of Register 12 (LPI Control and Status Register). In all other modes, this bit is reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn lpiis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit 0 of Register 12 (LPI Control and Status Register). In all other modes, this bit is reserved."]
        #[inline(always)]
        pub const fn set_lpiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event (LL or LH) occurs on the GPIS field (Bits \\[3:0\\]) of Register 56 (General Purpose IO Register) and the corresponding GPIE bit is enabled. This bit is cleared on reading lane 0 (GPIS) of Register 56 (General Purpose IO Register). When the GPIO feature is not enabled, this bit is reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event (LL or LH) occurs on the GPIS field (Bits \\[3:0\\]) of Register 56 (General Purpose IO Register) and the corresponding GPIE bit is enabled. This bit is cleared on reading lane 0 (GPIS) of Register 56 (General Purpose IO Register). When the GPIO feature is not enabled, this bit is reserved."]
        #[inline(always)]
        pub const fn set_gpiis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for IntrStatus {
        #[inline(always)]
        fn default() -> IntrStatus {
            IntrStatus(0)
        }
    }
    impl core::fmt::Debug for IntrStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntrStatus")
                .field("rgsmiiis", &self.rgsmiiis())
                .field("pcslchgis", &self.pcslchgis())
                .field("pcsancis", &self.pcsancis())
                .field("pmtis", &self.pmtis())
                .field("mmcis", &self.mmcis())
                .field("mmcrxis", &self.mmcrxis())
                .field("mmctxis", &self.mmctxis())
                .field("mmcrxipis", &self.mmcrxipis())
                .field("tsis", &self.tsis())
                .field("lpiis", &self.lpiis())
                .field("gpiis", &self.gpiis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntrStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntrStatus {{ rgsmiiis: {=bool:?}, pcslchgis: {=bool:?}, pcsancis: {=bool:?}, pmtis: {=bool:?}, mmcis: {=bool:?}, mmcrxis: {=bool:?}, mmctxis: {=bool:?}, mmcrxipis: {=bool:?}, tsis: {=bool:?}, lpiis: {=bool:?}, gpiis: {=bool:?} }}" , self . rgsmiiis () , self . pcslchgis () , self . pcsancis () , self . pmtis () , self . mmcis () , self . mmcrxis () , self . mmctxis () , self . mmcrxipis () , self . tsis () , self . lpiis () , self . gpiis ())
        }
    }
    #[doc = "Layer 3 Address 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L3Addr0(pub u32);
    impl L3Addr0 {
        #[doc = "Layer 3 Address 0 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[31:0\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[31:0\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 2 (L3SAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Source Address field in the IPv4 frames."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a00(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 0 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[31:0\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[31:0\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 2 (L3SAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Source Address field in the IPv4 frames."]
        #[inline(always)]
        pub const fn set_l3a00(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for L3Addr0 {
        #[inline(always)]
        fn default() -> L3Addr0 {
            L3Addr0(0)
        }
    }
    impl core::fmt::Debug for L3Addr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L3Addr0")
                .field("l3a00", &self.l3a00())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L3Addr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "L3Addr0 {{ l3a00: {=u32:?} }}", self.l3a00())
        }
    }
    #[doc = "Layer 3 Address 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L3Addr1(pub u32);
    impl L3Addr1 {
        #[doc = "Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a10(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 1 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[63:32\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset and Bit 4 (L3DAM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the IP Destination Address field in the IPv4 frames."]
        #[inline(always)]
        pub const fn set_l3a10(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for L3Addr1 {
        #[inline(always)]
        fn default() -> L3Addr1 {
            L3Addr1(0)
        }
    }
    impl core::fmt::Debug for L3Addr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L3Addr1")
                .field("l3a10", &self.l3a10())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L3Addr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "L3Addr1 {{ l3a10: {=u32:?} }}", self.l3a10())
        }
    }
    #[doc = "Layer 3 Address 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L3Addr2(pub u32);
    impl L3Addr2 {
        #[doc = "Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a20(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
        #[inline(always)]
        pub const fn set_l3a20(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for L3Addr2 {
        #[inline(always)]
        fn default() -> L3Addr2 {
            L3Addr2(0)
        }
    }
    impl core::fmt::Debug for L3Addr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L3Addr2")
                .field("l3a20", &self.l3a20())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L3Addr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "L3Addr2 {{ l3a20: {=u32:?} }}", self.l3a20())
        }
    }
    #[doc = "Layer 3 Address 3 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L3Addr3(pub u32);
    impl L3Addr3 {
        #[doc = "Layer 3 Address 3 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[127:96\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[127:96\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
        #[must_use]
        #[inline(always)]
        pub const fn l3a30(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Layer 3 Address 3 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[127:96\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[127:96\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
        #[inline(always)]
        pub const fn set_l3a30(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for L3Addr3 {
        #[inline(always)]
        fn default() -> L3Addr3 {
            L3Addr3(0)
        }
    }
    impl core::fmt::Debug for L3Addr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L3Addr3")
                .field("l3a30", &self.l3a30())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L3Addr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "L3Addr3 {{ l3a30: {=u32:?} }}", self.l3a30())
        }
    }
    #[doc = "Layer 3 and Layer 4 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L3L4Ctrl(pub u32);
    impl L3L4Ctrl {
        #[doc = "Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l3pen0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
        #[inline(always)]
        pub const fn set_l3pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
        #[must_use]
        #[inline(always)]
        pub const fn l3sam0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
        #[inline(always)]
        pub const fn set_l3sam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l3saim0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
        #[inline(always)]
        pub const fn set_l3saim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn l3dam0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
        #[inline(always)]
        pub const fn set_l3dam0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l3daim0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
        #[inline(always)]
        pub const fn set_l3daim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hsbm0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
        #[inline(always)]
        pub const fn set_l3hsbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l3hdbm0(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
        #[inline(always)]
        pub const fn set_l3hdbm0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l4pen0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
        #[inline(always)]
        pub const fn set_l4pen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spm0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
        #[inline(always)]
        pub const fn set_l4spm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l4spim0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
        #[inline(always)]
        pub const fn set_l4spim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpm0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
        #[inline(always)]
        pub const fn set_l4dpm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dpim0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
        #[inline(always)]
        pub const fn set_l4dpim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for L3L4Ctrl {
        #[inline(always)]
        fn default() -> L3L4Ctrl {
            L3L4Ctrl(0)
        }
    }
    impl core::fmt::Debug for L3L4Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L3L4Ctrl")
                .field("l3pen0", &self.l3pen0())
                .field("l3sam0", &self.l3sam0())
                .field("l3saim0", &self.l3saim0())
                .field("l3dam0", &self.l3dam0())
                .field("l3daim0", &self.l3daim0())
                .field("l3hsbm0", &self.l3hsbm0())
                .field("l3hdbm0", &self.l3hdbm0())
                .field("l4pen0", &self.l4pen0())
                .field("l4spm0", &self.l4spm0())
                .field("l4spim0", &self.l4spim0())
                .field("l4dpm0", &self.l4dpm0())
                .field("l4dpim0", &self.l4dpim0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L3L4Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "L3L4Ctrl {{ l3pen0: {=bool:?}, l3sam0: {=bool:?}, l3saim0: {=bool:?}, l3dam0: {=bool:?}, l3daim0: {=bool:?}, l3hsbm0: {=u8:?}, l3hdbm0: {=u8:?}, l4pen0: {=bool:?}, l4spm0: {=bool:?}, l4spim0: {=bool:?}, l4dpm0: {=bool:?}, l4dpim0: {=bool:?} }}" , self . l3pen0 () , self . l3sam0 () , self . l3saim0 () , self . l3dam0 () , self . l3daim0 () , self . l3hsbm0 () , self . l3hdbm0 () , self . l4pen0 () , self . l4spm0 () , self . l4spim0 () , self . l4dpm0 () , self . l4dpim0 ())
        }
    }
    #[doc = "Layer 4 Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct L4Addr(pub u32);
    impl L4Addr {
        #[doc = "Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
        #[must_use]
        #[inline(always)]
        pub const fn l4sp0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
        #[inline(always)]
        pub const fn set_l4sp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
        #[must_use]
        #[inline(always)]
        pub const fn l4dp0(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
        #[inline(always)]
        pub const fn set_l4dp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for L4Addr {
        #[inline(always)]
        fn default() -> L4Addr {
            L4Addr(0)
        }
    }
    impl core::fmt::Debug for L4Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("L4Addr")
                .field("l4sp0", &self.l4sp0())
                .field("l4dp0", &self.l4dp0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for L4Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "L4Addr {{ l4sp0: {=u16:?}, l4dp0: {=u16:?} }}",
                self.l4sp0(),
                self.l4dp0()
            )
        }
    }
    #[doc = "MAC Address Low Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Low(pub u32);
    impl Low {
        #[doc = "MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Low {
        #[inline(always)]
        fn default() -> Low {
            Low(0)
        }
    }
    impl core::fmt::Debug for Low {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Low")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Low {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Low {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "LPI Control and Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpiCsr(pub u32);
    impl LpiCsr {
        #[doc = "Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
        #[inline(always)]
        pub const fn set_tlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpiex(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
        #[inline(always)]
        pub const fn set_tlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
        #[inline(always)]
        pub const fn set_rlpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpiex(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
        #[inline(always)]
        pub const fn set_rlpiex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpist(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
        #[inline(always)]
        pub const fn set_tlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
        #[must_use]
        #[inline(always)]
        pub const fn rlpist(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
        #[inline(always)]
        pub const fn set_rlpist(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn lpien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
        #[inline(always)]
        pub const fn set_lpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
        #[must_use]
        #[inline(always)]
        pub const fn plsen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
        #[inline(always)]
        pub const fn set_plsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lpitxa(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
        #[inline(always)]
        pub const fn set_lpitxa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for LpiCsr {
        #[inline(always)]
        fn default() -> LpiCsr {
            LpiCsr(0)
        }
    }
    impl core::fmt::Debug for LpiCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LpiCsr")
                .field("tlpien", &self.tlpien())
                .field("tlpiex", &self.tlpiex())
                .field("rlpien", &self.rlpien())
                .field("rlpiex", &self.rlpiex())
                .field("tlpist", &self.tlpist())
                .field("rlpist", &self.rlpist())
                .field("lpien", &self.lpien())
                .field("pls", &self.pls())
                .field("plsen", &self.plsen())
                .field("lpitxa", &self.lpitxa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LpiCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LpiCsr {{ tlpien: {=bool:?}, tlpiex: {=bool:?}, rlpien: {=bool:?}, rlpiex: {=bool:?}, tlpist: {=bool:?}, rlpist: {=bool:?}, lpien: {=bool:?}, pls: {=bool:?}, plsen: {=bool:?}, lpitxa: {=bool:?} }}" , self . tlpien () , self . tlpiex () , self . rlpien () , self . rlpiex () , self . tlpist () , self . rlpist () , self . lpien () , self . pls () , self . plsen () , self . lpitxa ())
        }
    }
    #[doc = "LPI Timers Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpiTcr(pub u32);
    impl LpiTcr {
        #[doc = "LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
        #[must_use]
        #[inline(always)]
        pub const fn twt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
        #[inline(always)]
        pub const fn set_twt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
        #[must_use]
        #[inline(always)]
        pub const fn lst(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
        #[inline(always)]
        pub const fn set_lst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for LpiTcr {
        #[inline(always)]
        fn default() -> LpiTcr {
            LpiTcr(0)
        }
    }
    impl core::fmt::Debug for LpiTcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LpiTcr")
                .field("twt", &self.twt())
                .field("lst", &self.lst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LpiTcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LpiTcr {{ twt: {=u16:?}, lst: {=u16:?} }}",
                self.twt(),
                self.lst()
            )
        }
    }
    #[doc = "MAC Address 0 High Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacAddr0High(pub u32);
    impl MacAddr0High {
        #[doc = "MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
        #[must_use]
        #[inline(always)]
        pub const fn addrhi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC Address0 \\[47:32\\]
This field contains the upper 16 bits (47:32) of the first 6-byte MAC address. The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
        #[inline(always)]
        pub const fn set_addrhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Address Enable This bit is RO. The bit value is fixed at 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address Enable This bit is RO. The bit value is fixed at 1."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MacAddr0High {
        #[inline(always)]
        fn default() -> MacAddr0High {
            MacAddr0High(0)
        }
    }
    impl core::fmt::Debug for MacAddr0High {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacAddr0High")
                .field("addrhi", &self.addrhi())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacAddr0High {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacAddr0High {{ addrhi: {=u16:?}, ae: {=bool:?} }}",
                self.addrhi(),
                self.ae()
            )
        }
    }
    #[doc = "MAC Address 0 Low Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacAddr0Low(pub u32);
    impl MacAddr0Low {
        #[doc = "MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC Address0 \\[31:0\\]
This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacAddr0Low {
        #[inline(always)]
        fn default() -> MacAddr0Low {
            MacAddr0Low(0)
        }
    }
    impl core::fmt::Debug for MacAddr0Low {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacAddr0Low")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacAddr0Low {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacAddr0Low {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "MAC Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccfg(pub u32);
    impl Maccfg {
        #[doc = "Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn prelen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved."]
        #[inline(always)]
        pub const fn set_prelen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
        #[must_use]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
        #[inline(always)]
        pub const fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k."]
        #[must_use]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k."]
        #[inline(always)]
        pub const fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
        #[must_use]
        #[inline(always)]
        pub const fn acs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
        #[inline(always)]
        pub const fn set_acs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up."]
        #[must_use]
        #[inline(always)]
        pub const fn lud(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up."]
        #[inline(always)]
        pub const fn set_lud(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
        #[must_use]
        #[inline(always)]
        pub const fn ipc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
        #[inline(always)]
        pub const fn set_ipc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously."]
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously."]
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
        #[must_use]
        #[inline(always)]
        pub const fn lm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
        #[inline(always)]
        pub const fn set_lm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
        #[must_use]
        #[inline(always)]
        pub const fn do_(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
        #[inline(always)]
        pub const fn set_do_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
        #[inline(always)]
        pub const fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrs(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
        #[inline(always)]
        pub const fn set_dcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
        #[must_use]
        #[inline(always)]
        pub const fn ifg(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
        #[inline(always)]
        pub const fn set_ifg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
        #[must_use]
        #[inline(always)]
        pub const fn je(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
        #[inline(always)]
        pub const fn set_je(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
        #[must_use]
        #[inline(always)]
        pub const fn be(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
        #[inline(always)]
        pub const fn set_be(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
        #[inline(always)]
        pub const fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
        #[inline(always)]
        pub const fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
        #[inline(always)]
        pub const fn set_cst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sfterr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
        #[inline(always)]
        pub const fn set_sfterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
        #[must_use]
        #[inline(always)]
        pub const fn twokpe(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
        #[inline(always)]
        pub const fn set_twokpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sarc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
        #[inline(always)]
        pub const fn set_sarc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Maccfg {
        #[inline(always)]
        fn default() -> Maccfg {
            Maccfg(0)
        }
    }
    impl core::fmt::Debug for Maccfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccfg")
                .field("prelen", &self.prelen())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("dc", &self.dc())
                .field("bl", &self.bl())
                .field("acs", &self.acs())
                .field("lud", &self.lud())
                .field("dr", &self.dr())
                .field("ipc", &self.ipc())
                .field("dm", &self.dm())
                .field("lm", &self.lm())
                .field("do_", &self.do_())
                .field("fes", &self.fes())
                .field("ps", &self.ps())
                .field("dcrs", &self.dcrs())
                .field("ifg", &self.ifg())
                .field("je", &self.je())
                .field("be", &self.be())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .field("tc", &self.tc())
                .field("cst", &self.cst())
                .field("sfterr", &self.sfterr())
                .field("twokpe", &self.twokpe())
                .field("sarc", &self.sarc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maccfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Maccfg {{ prelen: {=u8:?}, re: {=bool:?}, te: {=bool:?}, dc: {=bool:?}, bl: {=u8:?}, acs: {=bool:?}, lud: {=bool:?}, dr: {=bool:?}, ipc: {=bool:?}, dm: {=bool:?}, lm: {=bool:?}, do_: {=bool:?}, fes: {=bool:?}, ps: {=bool:?}, dcrs: {=bool:?}, ifg: {=u8:?}, je: {=bool:?}, be: {=bool:?}, jd: {=bool:?}, wd: {=bool:?}, tc: {=bool:?}, cst: {=bool:?}, sfterr: {=bool:?}, twokpe: {=bool:?}, sarc: {=u8:?} }}" , self . prelen () , self . re () , self . te () , self . dc () , self . bl () , self . acs () , self . lud () , self . dr () , self . ipc () , self . dm () , self . lm () , self . do_ () , self . fes () , self . ps () , self . dcrs () , self . ifg () , self . je () , self . be () , self . jd () , self . wd () , self . tc () , self . cst () , self . sfterr () , self . twokpe () , self . sarc ())
        }
    }
    #[doc = "MAC Frame Filter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macff(pub u32);
    impl Macff {
        #[doc = "Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
        #[must_use]
        #[inline(always)]
        pub const fn huc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
        #[inline(always)]
        pub const fn set_huc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
        #[must_use]
        #[inline(always)]
        pub const fn hmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
        #[inline(always)]
        pub const fn set_hmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
        #[inline(always)]
        pub const fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
        #[must_use]
        #[inline(always)]
        pub const fn vtfe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
        #[inline(always)]
        pub const fn set_vtfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
        #[must_use]
        #[inline(always)]
        pub const fn ipfe(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
        #[inline(always)]
        pub const fn set_ipfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
        #[must_use]
        #[inline(always)]
        pub const fn dntu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
        #[inline(always)]
        pub const fn set_dntu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macff {
        #[inline(always)]
        fn default() -> Macff {
            Macff(0)
        }
    }
    impl core::fmt::Debug for Macff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macff")
                .field("pr", &self.pr())
                .field("huc", &self.huc())
                .field("hmc", &self.hmc())
                .field("daif", &self.daif())
                .field("pm", &self.pm())
                .field("dbf", &self.dbf())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("vtfe", &self.vtfe())
                .field("ipfe", &self.ipfe())
                .field("dntu", &self.dntu())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Macff {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Macff {{ pr: {=bool:?}, huc: {=bool:?}, hmc: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {=bool:?}, pcf: {=u8:?}, saif: {=bool:?}, saf: {=bool:?}, hpf: {=bool:?}, vtfe: {=bool:?}, ipfe: {=bool:?}, dntu: {=bool:?}, ra: {=bool:?} }}" , self . pr () , self . huc () , self . hmc () , self . daif () , self . pm () , self . dbf () , self . pcf () , self . saif () , self . saf () , self . hpf () , self . vtfe () , self . ipfe () , self . dntu () , self . ra ())
        }
    }
    #[doc = "MMC Control establishes the operating mode of MMC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcCntrl(pub u32);
    impl MmcCntrl {
        #[doc = "Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Reset When this bit is set, all counters are reset. This bit is cleared automatically after 1 clock cycle."]
        #[inline(always)]
        pub const fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn cntstopro(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter Stop Rollover When this bit is set, the counter does not roll over to zero after reaching the maximum value."]
        #[inline(always)]
        pub const fn set_cntstopro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
        #[must_use]
        #[inline(always)]
        pub const fn rstonrd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on Read When this bit is set, the MMC counters are reset to zero after Read (self-clearing after reset). The counters are cleared when the least significant byte lane (Bits\\[7:0\\]) is read."]
        #[inline(always)]
        pub const fn set_rstonrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received frame. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cntfreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Counter Freeze When this bit is set, it freezes all MMC counters to their current value. Until this bit is reset to 0, no MMC counter is updated because of any transmitted or received frame. If any MMC counter is read with the Reset on Read bit set, then that counter is also cleared in this mode."]
        #[inline(always)]
        pub const fn set_cntfreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to Bit 5. This bit is cleared automatically after 1 clock cycle. This bit, along with Bit 5, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
        #[must_use]
        #[inline(always)]
        pub const fn cntprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Counters Preset When this bit is set, all counters are initialized or preset to almost full or almost half according to Bit 5. This bit is cleared automatically after 1 clock cycle. This bit, along with Bit 5, is useful for debugging and testing the assertion of interrupts because of MMC counter becoming half-full or full."]
        #[inline(always)]
        pub const fn set_cntprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Full-Half Preset When this bit is low and Bit 4 is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (half - 2KBytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16). When this bit is high and Bit 4 is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (full - 2KBytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and frame counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
        #[must_use]
        #[inline(always)]
        pub const fn cntprstlvl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Full-Half Preset When this bit is low and Bit 4 is set, all MMC counters get preset to almost-half value. All octet counters get preset to 0x7FFF_F800 (half - 2KBytes) and all frame-counters gets preset to 0x7FFF_FFF0 (half - 16). When this bit is high and Bit 4 is set, all MMC counters get preset to almost-full value. All octet counters get preset to 0xFFFF_F800 (full - 2KBytes) and all frame-counters gets preset to 0xFFFF_FFF0 (full - 16). For 16-bit counters, the almost-half preset values are 0x7800 and 0x7FF0 for the respective octet and frame counters. Similarly, the almost-full preset values for the 16-bit counters are 0xF800 and 0xFFF0."]
        #[inline(always)]
        pub const fn set_cntprstlvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Frames When set, the MAC updates all related MMC Counters for Broadcast frames that are dropped because of the setting of Bit 5 (DBF) of Register 1 (MAC Frame Filter). When reset, the MMC Counters are not updated for dropped Broadcast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn ucdbc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update MMC Counters for Dropped Broadcast Frames When set, the MAC updates all related MMC Counters for Broadcast frames that are dropped because of the setting of Bit 5 (DBF) of Register 1 (MAC Frame Filter). When reset, the MMC Counters are not updated for dropped Broadcast frames."]
        #[inline(always)]
        pub const fn set_ucdbc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MmcCntrl {
        #[inline(always)]
        fn default() -> MmcCntrl {
            MmcCntrl(0)
        }
    }
    impl core::fmt::Debug for MmcCntrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcCntrl")
                .field("cntrst", &self.cntrst())
                .field("cntstopro", &self.cntstopro())
                .field("rstonrd", &self.rstonrd())
                .field("cntfreez", &self.cntfreez())
                .field("cntprst", &self.cntprst())
                .field("cntprstlvl", &self.cntprstlvl())
                .field("ucdbc", &self.ucdbc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcCntrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcCntrl {{ cntrst: {=bool:?}, cntstopro: {=bool:?}, rstonrd: {=bool:?}, cntfreez: {=bool:?}, cntprst: {=bool:?}, cntprstlvl: {=bool:?}, ucdbc: {=bool:?} }}" , self . cntrst () , self . cntstopro () , self . rstonrd () , self . cntfreez () , self . cntprst () , self . cntprstlvl () , self . ucdbc ())
        }
    }
    #[doc = "MMC Receive Interrupt mask maintains the mask for the interrupt generated from all of the receive statistic counters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIntrMaskRx(pub u32);
    impl MmcIntrMaskRx {
        #[doc = "MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxgboctim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxgboctim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxgoctim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxgoctim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbcgfim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxbcgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmcgfim(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxmcgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerfim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxcrcerfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerfim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxalgnerfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxruntfim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxruntfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxjaberfim(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxjaberfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxusizegfim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxusizegfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxosizegfim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxosizegfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx64octgbfim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx64octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx65t127octgbfim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx65t127octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx128t255octgbfim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx128t255octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx256t511octgbfim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx256t511octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx512t1023octgbfim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx512t1023octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx1024tmaxoctgbfim(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx1024tmaxoctgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxucgfim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxucgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlenerfim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxlenerfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxorangefim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxorangefim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpausfim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxpausfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfovfim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxfovfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxvlangbfim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxvlangbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxwdogfim(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxwdogfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxrcverrfim(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxrcverrfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxctrlfim(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxctrlfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MmcIntrMaskRx {
        #[inline(always)]
        fn default() -> MmcIntrMaskRx {
            MmcIntrMaskRx(0)
        }
    }
    impl core::fmt::Debug for MmcIntrMaskRx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIntrMaskRx")
                .field("rxgboctim", &self.rxgboctim())
                .field("rxgoctim", &self.rxgoctim())
                .field("rxbcgfim", &self.rxbcgfim())
                .field("rxmcgfim", &self.rxmcgfim())
                .field("rxcrcerfim", &self.rxcrcerfim())
                .field("rxalgnerfim", &self.rxalgnerfim())
                .field("rxruntfim", &self.rxruntfim())
                .field("rxjaberfim", &self.rxjaberfim())
                .field("rxusizegfim", &self.rxusizegfim())
                .field("rxosizegfim", &self.rxosizegfim())
                .field("rx64octgbfim", &self.rx64octgbfim())
                .field("rx65t127octgbfim", &self.rx65t127octgbfim())
                .field("rx128t255octgbfim", &self.rx128t255octgbfim())
                .field("rx256t511octgbfim", &self.rx256t511octgbfim())
                .field("rx512t1023octgbfim", &self.rx512t1023octgbfim())
                .field("rx1024tmaxoctgbfim", &self.rx1024tmaxoctgbfim())
                .field("rxucgfim", &self.rxucgfim())
                .field("rxlenerfim", &self.rxlenerfim())
                .field("rxorangefim", &self.rxorangefim())
                .field("rxpausfim", &self.rxpausfim())
                .field("rxfovfim", &self.rxfovfim())
                .field("rxvlangbfim", &self.rxvlangbfim())
                .field("rxwdogfim", &self.rxwdogfim())
                .field("rxrcverrfim", &self.rxrcverrfim())
                .field("rxctrlfim", &self.rxctrlfim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIntrMaskRx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIntrMaskRx {{ rxgboctim: {=bool:?}, rxgoctim: {=bool:?}, rxbcgfim: {=bool:?}, rxmcgfim: {=bool:?}, rxcrcerfim: {=bool:?}, rxalgnerfim: {=bool:?}, rxruntfim: {=bool:?}, rxjaberfim: {=bool:?}, rxusizegfim: {=bool:?}, rxosizegfim: {=bool:?}, rx64octgbfim: {=bool:?}, rx65t127octgbfim: {=bool:?}, rx128t255octgbfim: {=bool:?}, rx256t511octgbfim: {=bool:?}, rx512t1023octgbfim: {=bool:?}, rx1024tmaxoctgbfim: {=bool:?}, rxucgfim: {=bool:?}, rxlenerfim: {=bool:?}, rxorangefim: {=bool:?}, rxpausfim: {=bool:?}, rxfovfim: {=bool:?}, rxvlangbfim: {=bool:?}, rxwdogfim: {=bool:?}, rxrcverrfim: {=bool:?}, rxctrlfim: {=bool:?} }}" , self . rxgboctim () , self . rxgoctim () , self . rxbcgfim () , self . rxmcgfim () , self . rxcrcerfim () , self . rxalgnerfim () , self . rxruntfim () , self . rxjaberfim () , self . rxusizegfim () , self . rxosizegfim () , self . rx64octgbfim () , self . rx65t127octgbfim () , self . rx128t255octgbfim () , self . rx256t511octgbfim () , self . rx512t1023octgbfim () , self . rx1024tmaxoctgbfim () , self . rxucgfim () , self . rxlenerfim () , self . rxorangefim () , self . rxpausfim () , self . rxfovfim () , self . rxvlangbfim () , self . rxwdogfim () , self . rxrcverrfim () , self . rxctrlfim ())
        }
    }
    #[doc = "MMC Transmit Interrupt Mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIntrMaskTx(pub u32);
    impl MmcIntrMaskTx {
        #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgboctim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgboctim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgbfrmim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgbfrmim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txbcgfim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txbcgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcgfim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx64octgbfim(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx64octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx65t127octgbfim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx65t127octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx128t255octgbfim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx128t255octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx256t511octgbfim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx256t511octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx512t1023octgbfim(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx512t1023octgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx1024tmaxoctgbfim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx1024tmaxoctgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txucgbfim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txucgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcgbfim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txbcgbfim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txbcgbfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txuflowerfim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txuflowerfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txscolgfim(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txscolgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgfim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcolgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txdeffim(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txdeffim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txlatcolfim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txlatcolfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txexcolfim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txexcolfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txcarerfim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txcarerfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgoctim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgoctim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgfrmim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgfrmim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txexdeffim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txexdeffim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txpausfim(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txpausfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txvlangfim(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txvlangfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txosizegfim(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txosizegfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MmcIntrMaskTx {
        #[inline(always)]
        fn default() -> MmcIntrMaskTx {
            MmcIntrMaskTx(0)
        }
    }
    impl core::fmt::Debug for MmcIntrMaskTx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIntrMaskTx")
                .field("txgboctim", &self.txgboctim())
                .field("txgbfrmim", &self.txgbfrmim())
                .field("txbcgfim", &self.txbcgfim())
                .field("txmcgfim", &self.txmcgfim())
                .field("tx64octgbfim", &self.tx64octgbfim())
                .field("tx65t127octgbfim", &self.tx65t127octgbfim())
                .field("tx128t255octgbfim", &self.tx128t255octgbfim())
                .field("tx256t511octgbfim", &self.tx256t511octgbfim())
                .field("tx512t1023octgbfim", &self.tx512t1023octgbfim())
                .field("tx1024tmaxoctgbfim", &self.tx1024tmaxoctgbfim())
                .field("txucgbfim", &self.txucgbfim())
                .field("txmcgbfim", &self.txmcgbfim())
                .field("txbcgbfim", &self.txbcgbfim())
                .field("txuflowerfim", &self.txuflowerfim())
                .field("txscolgfim", &self.txscolgfim())
                .field("txmcolgfim", &self.txmcolgfim())
                .field("txdeffim", &self.txdeffim())
                .field("txlatcolfim", &self.txlatcolfim())
                .field("txexcolfim", &self.txexcolfim())
                .field("txcarerfim", &self.txcarerfim())
                .field("txgoctim", &self.txgoctim())
                .field("txgfrmim", &self.txgfrmim())
                .field("txexdeffim", &self.txexdeffim())
                .field("txpausfim", &self.txpausfim())
                .field("txvlangfim", &self.txvlangfim())
                .field("txosizegfim", &self.txosizegfim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIntrMaskTx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIntrMaskTx {{ txgboctim: {=bool:?}, txgbfrmim: {=bool:?}, txbcgfim: {=bool:?}, txmcgfim: {=bool:?}, tx64octgbfim: {=bool:?}, tx65t127octgbfim: {=bool:?}, tx128t255octgbfim: {=bool:?}, tx256t511octgbfim: {=bool:?}, tx512t1023octgbfim: {=bool:?}, tx1024tmaxoctgbfim: {=bool:?}, txucgbfim: {=bool:?}, txmcgbfim: {=bool:?}, txbcgbfim: {=bool:?}, txuflowerfim: {=bool:?}, txscolgfim: {=bool:?}, txmcolgfim: {=bool:?}, txdeffim: {=bool:?}, txlatcolfim: {=bool:?}, txexcolfim: {=bool:?}, txcarerfim: {=bool:?}, txgoctim: {=bool:?}, txgfrmim: {=bool:?}, txexdeffim: {=bool:?}, txpausfim: {=bool:?}, txvlangfim: {=bool:?}, txosizegfim: {=bool:?} }}" , self . txgboctim () , self . txgbfrmim () , self . txbcgfim () , self . txmcgfim () , self . tx64octgbfim () , self . tx65t127octgbfim () , self . tx128t255octgbfim () , self . tx256t511octgbfim () , self . tx512t1023octgbfim () , self . tx1024tmaxoctgbfim () , self . txucgbfim () , self . txmcgbfim () , self . txbcgbfim () , self . txuflowerfim () , self . txscolgfim () , self . txmcolgfim () , self . txdeffim () , self . txlatcolfim () , self . txexcolfim () , self . txcarerfim () , self . txgoctim () , self . txgfrmim () , self . txexdeffim () , self . txpausfim () , self . txvlangfim () , self . txosizegfim ())
        }
    }
    #[doc = "MMC Receive Interrupt maintains the interrupt generated from all of the receive statistic counters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIntrRx(pub u32);
    impl MmcIntrRx {
        #[doc = "MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxgbfrmis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxgbfrmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxgboctis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxgboctis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxgoctis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxgoctis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbcgfis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxbcgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxmcgfis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxmcgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxcrcerfis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxcrcerfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxalgnerfis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxalgnerfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxruntfis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxruntfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxjaberfis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxjaberfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxusizegfis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxusizegfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxosizegfis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxosizegfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx64octgbfis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx64octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx65t127octgbfis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx65t127octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx128t255octgbfis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx128t255octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx256t511octgbfis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx256t511octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx512t1023octgbfis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx512t1023octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rx1024tmaxoctgbfis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rx1024tmaxoctgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxucgfis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxucgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlenerfis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxlenerfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxorangefis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxorangefis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpausfis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxpausfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfovfis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxfovfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxvlangbfis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxvlangbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxwdogfis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxwdogfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxrcverrfis(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxrcverrfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxctrlfis(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxctrlfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MmcIntrRx {
        #[inline(always)]
        fn default() -> MmcIntrRx {
            MmcIntrRx(0)
        }
    }
    impl core::fmt::Debug for MmcIntrRx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIntrRx")
                .field("rxgbfrmis", &self.rxgbfrmis())
                .field("rxgboctis", &self.rxgboctis())
                .field("rxgoctis", &self.rxgoctis())
                .field("rxbcgfis", &self.rxbcgfis())
                .field("rxmcgfis", &self.rxmcgfis())
                .field("rxcrcerfis", &self.rxcrcerfis())
                .field("rxalgnerfis", &self.rxalgnerfis())
                .field("rxruntfis", &self.rxruntfis())
                .field("rxjaberfis", &self.rxjaberfis())
                .field("rxusizegfis", &self.rxusizegfis())
                .field("rxosizegfis", &self.rxosizegfis())
                .field("rx64octgbfis", &self.rx64octgbfis())
                .field("rx65t127octgbfis", &self.rx65t127octgbfis())
                .field("rx128t255octgbfis", &self.rx128t255octgbfis())
                .field("rx256t511octgbfis", &self.rx256t511octgbfis())
                .field("rx512t1023octgbfis", &self.rx512t1023octgbfis())
                .field("rx1024tmaxoctgbfis", &self.rx1024tmaxoctgbfis())
                .field("rxucgfis", &self.rxucgfis())
                .field("rxlenerfis", &self.rxlenerfis())
                .field("rxorangefis", &self.rxorangefis())
                .field("rxpausfis", &self.rxpausfis())
                .field("rxfovfis", &self.rxfovfis())
                .field("rxvlangbfis", &self.rxvlangbfis())
                .field("rxwdogfis", &self.rxwdogfis())
                .field("rxrcverrfis", &self.rxrcverrfis())
                .field("rxctrlfis", &self.rxctrlfis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIntrRx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIntrRx {{ rxgbfrmis: {=bool:?}, rxgboctis: {=bool:?}, rxgoctis: {=bool:?}, rxbcgfis: {=bool:?}, rxmcgfis: {=bool:?}, rxcrcerfis: {=bool:?}, rxalgnerfis: {=bool:?}, rxruntfis: {=bool:?}, rxjaberfis: {=bool:?}, rxusizegfis: {=bool:?}, rxosizegfis: {=bool:?}, rx64octgbfis: {=bool:?}, rx65t127octgbfis: {=bool:?}, rx128t255octgbfis: {=bool:?}, rx256t511octgbfis: {=bool:?}, rx512t1023octgbfis: {=bool:?}, rx1024tmaxoctgbfis: {=bool:?}, rxucgfis: {=bool:?}, rxlenerfis: {=bool:?}, rxorangefis: {=bool:?}, rxpausfis: {=bool:?}, rxfovfis: {=bool:?}, rxvlangbfis: {=bool:?}, rxwdogfis: {=bool:?}, rxrcverrfis: {=bool:?}, rxctrlfis: {=bool:?} }}" , self . rxgbfrmis () , self . rxgboctis () , self . rxgoctis () , self . rxbcgfis () , self . rxmcgfis () , self . rxcrcerfis () , self . rxalgnerfis () , self . rxruntfis () , self . rxjaberfis () , self . rxusizegfis () , self . rxosizegfis () , self . rx64octgbfis () , self . rx65t127octgbfis () , self . rx128t255octgbfis () , self . rx256t511octgbfis () , self . rx512t1023octgbfis () , self . rx1024tmaxoctgbfis () , self . rxucgfis () , self . rxlenerfis () , self . rxorangefis () , self . rxpausfis () , self . rxfovfis () , self . rxvlangbfis () , self . rxwdogfis () , self . rxrcverrfis () , self . rxctrlfis ())
        }
    }
    #[doc = "MMC Transmit Interrupt maintains the interrupt generated from all of the transmit statistic counters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIntrTx(pub u32);
    impl MmcIntrTx {
        #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgboctis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgboctis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgbfrmis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgbfrmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txbcgfis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txbcgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcgfis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx64octgbfis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx64octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx65t127octgbfis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
        #[inline(always)]
        pub const fn set_tx65t127octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx128t255octgbfis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx128t255octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx256t511octgbfis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx256t511octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx512t1023octgbfis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx512t1023octgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn tx1024tmaxoctgbfis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_tx1024tmaxoctgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txucgbfis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txucgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcgbfis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txbcgbfis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txbcgbfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txuflowerfis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txuflowerfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txscolgfis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txscolgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txmcolgfis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txmcolgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txdeffis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txdeffis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txlatcolfis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txlatcolfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txexcolfis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txexcolfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txcarerfis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txcarerfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgoctis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgoctis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txgfrmis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txgfrmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txexdeffis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txexdeffis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txpausfis(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txpausfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txvlangfis(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txvlangfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn txosizegfis(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_txosizegfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for MmcIntrTx {
        #[inline(always)]
        fn default() -> MmcIntrTx {
            MmcIntrTx(0)
        }
    }
    impl core::fmt::Debug for MmcIntrTx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIntrTx")
                .field("txgboctis", &self.txgboctis())
                .field("txgbfrmis", &self.txgbfrmis())
                .field("txbcgfis", &self.txbcgfis())
                .field("txmcgfis", &self.txmcgfis())
                .field("tx64octgbfis", &self.tx64octgbfis())
                .field("tx65t127octgbfis", &self.tx65t127octgbfis())
                .field("tx128t255octgbfis", &self.tx128t255octgbfis())
                .field("tx256t511octgbfis", &self.tx256t511octgbfis())
                .field("tx512t1023octgbfis", &self.tx512t1023octgbfis())
                .field("tx1024tmaxoctgbfis", &self.tx1024tmaxoctgbfis())
                .field("txucgbfis", &self.txucgbfis())
                .field("txmcgbfis", &self.txmcgbfis())
                .field("txbcgbfis", &self.txbcgbfis())
                .field("txuflowerfis", &self.txuflowerfis())
                .field("txscolgfis", &self.txscolgfis())
                .field("txmcolgfis", &self.txmcolgfis())
                .field("txdeffis", &self.txdeffis())
                .field("txlatcolfis", &self.txlatcolfis())
                .field("txexcolfis", &self.txexcolfis())
                .field("txcarerfis", &self.txcarerfis())
                .field("txgoctis", &self.txgoctis())
                .field("txgfrmis", &self.txgfrmis())
                .field("txexdeffis", &self.txexdeffis())
                .field("txpausfis", &self.txpausfis())
                .field("txvlangfis", &self.txvlangfis())
                .field("txosizegfis", &self.txosizegfis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIntrTx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIntrTx {{ txgboctis: {=bool:?}, txgbfrmis: {=bool:?}, txbcgfis: {=bool:?}, txmcgfis: {=bool:?}, tx64octgbfis: {=bool:?}, tx65t127octgbfis: {=bool:?}, tx128t255octgbfis: {=bool:?}, tx256t511octgbfis: {=bool:?}, tx512t1023octgbfis: {=bool:?}, tx1024tmaxoctgbfis: {=bool:?}, txucgbfis: {=bool:?}, txmcgbfis: {=bool:?}, txbcgbfis: {=bool:?}, txuflowerfis: {=bool:?}, txscolgfis: {=bool:?}, txmcolgfis: {=bool:?}, txdeffis: {=bool:?}, txlatcolfis: {=bool:?}, txexcolfis: {=bool:?}, txcarerfis: {=bool:?}, txgoctis: {=bool:?}, txgfrmis: {=bool:?}, txexdeffis: {=bool:?}, txpausfis: {=bool:?}, txvlangfis: {=bool:?}, txosizegfis: {=bool:?} }}" , self . txgboctis () , self . txgbfrmis () , self . txbcgfis () , self . txmcgfis () , self . tx64octgbfis () , self . tx65t127octgbfis () , self . tx128t255octgbfis () , self . tx256t511octgbfis () , self . tx512t1023octgbfis () , self . tx1024tmaxoctgbfis () , self . txucgbfis () , self . txmcgbfis () , self . txbcgbfis () , self . txuflowerfis () , self . txscolgfis () , self . txmcolgfis () , self . txdeffis () , self . txlatcolfis () , self . txexcolfis () , self . txcarerfis () , self . txgoctis () , self . txgfrmis () , self . txexdeffis () , self . txpausfis () , self . txvlangfis () , self . txosizegfis ())
        }
    }
    #[doc = "MMC IPC Receive Checksum Offload Interrupt Mask maintains the mask for the interrupt generated from the receive IPC statistic counters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIpcIntrMaskRx(pub u32);
    impl MmcIpcIntrMaskRx {
        #[doc = "MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4gfim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4gfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4herfim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4herfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4nopayfim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4nopayfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4fragfim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4fragfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4udsblfim(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4udsblfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6gfim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6gfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6herfim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6herfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6nopayfim(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6nopayfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudpgfim(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudpgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudperfim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudperfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcpgfim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcpgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcperfim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcperfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmpgfim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmpgfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmperfim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmperfim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4goim(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4goim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4heroim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4heroim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4nopayoim(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4nopayoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4fragoim(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4fragoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4udsbloim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4udsbloim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6goim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6goim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6heroim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6heroim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6nopayoim(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6nopayoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudpgoim(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudpgoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudperoim(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudperoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcpgoim(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcpgoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcperoim(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcperoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmpgoim(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmpgoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmperoim(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmperoim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MmcIpcIntrMaskRx {
        #[inline(always)]
        fn default() -> MmcIpcIntrMaskRx {
            MmcIpcIntrMaskRx(0)
        }
    }
    impl core::fmt::Debug for MmcIpcIntrMaskRx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIpcIntrMaskRx")
                .field("rxipv4gfim", &self.rxipv4gfim())
                .field("rxipv4herfim", &self.rxipv4herfim())
                .field("rxipv4nopayfim", &self.rxipv4nopayfim())
                .field("rxipv4fragfim", &self.rxipv4fragfim())
                .field("rxipv4udsblfim", &self.rxipv4udsblfim())
                .field("rxipv6gfim", &self.rxipv6gfim())
                .field("rxipv6herfim", &self.rxipv6herfim())
                .field("rxipv6nopayfim", &self.rxipv6nopayfim())
                .field("rxudpgfim", &self.rxudpgfim())
                .field("rxudperfim", &self.rxudperfim())
                .field("rxtcpgfim", &self.rxtcpgfim())
                .field("rxtcperfim", &self.rxtcperfim())
                .field("rxicmpgfim", &self.rxicmpgfim())
                .field("rxicmperfim", &self.rxicmperfim())
                .field("rxipv4goim", &self.rxipv4goim())
                .field("rxipv4heroim", &self.rxipv4heroim())
                .field("rxipv4nopayoim", &self.rxipv4nopayoim())
                .field("rxipv4fragoim", &self.rxipv4fragoim())
                .field("rxipv4udsbloim", &self.rxipv4udsbloim())
                .field("rxipv6goim", &self.rxipv6goim())
                .field("rxipv6heroim", &self.rxipv6heroim())
                .field("rxipv6nopayoim", &self.rxipv6nopayoim())
                .field("rxudpgoim", &self.rxudpgoim())
                .field("rxudperoim", &self.rxudperoim())
                .field("rxtcpgoim", &self.rxtcpgoim())
                .field("rxtcperoim", &self.rxtcperoim())
                .field("rxicmpgoim", &self.rxicmpgoim())
                .field("rxicmperoim", &self.rxicmperoim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIpcIntrMaskRx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIpcIntrMaskRx {{ rxipv4gfim: {=bool:?}, rxipv4herfim: {=bool:?}, rxipv4nopayfim: {=bool:?}, rxipv4fragfim: {=bool:?}, rxipv4udsblfim: {=bool:?}, rxipv6gfim: {=bool:?}, rxipv6herfim: {=bool:?}, rxipv6nopayfim: {=bool:?}, rxudpgfim: {=bool:?}, rxudperfim: {=bool:?}, rxtcpgfim: {=bool:?}, rxtcperfim: {=bool:?}, rxicmpgfim: {=bool:?}, rxicmperfim: {=bool:?}, rxipv4goim: {=bool:?}, rxipv4heroim: {=bool:?}, rxipv4nopayoim: {=bool:?}, rxipv4fragoim: {=bool:?}, rxipv4udsbloim: {=bool:?}, rxipv6goim: {=bool:?}, rxipv6heroim: {=bool:?}, rxipv6nopayoim: {=bool:?}, rxudpgoim: {=bool:?}, rxudperoim: {=bool:?}, rxtcpgoim: {=bool:?}, rxtcperoim: {=bool:?}, rxicmpgoim: {=bool:?}, rxicmperoim: {=bool:?} }}" , self . rxipv4gfim () , self . rxipv4herfim () , self . rxipv4nopayfim () , self . rxipv4fragfim () , self . rxipv4udsblfim () , self . rxipv6gfim () , self . rxipv6herfim () , self . rxipv6nopayfim () , self . rxudpgfim () , self . rxudperfim () , self . rxtcpgfim () , self . rxtcperfim () , self . rxicmpgfim () , self . rxicmperfim () , self . rxipv4goim () , self . rxipv4heroim () , self . rxipv4nopayoim () , self . rxipv4fragoim () , self . rxipv4udsbloim () , self . rxipv6goim () , self . rxipv6heroim () , self . rxipv6nopayoim () , self . rxudpgoim () , self . rxudperoim () , self . rxtcpgoim () , self . rxtcperoim () , self . rxicmpgoim () , self . rxicmperoim ())
        }
    }
    #[doc = "MMC Receive Checksum Offload Interrupt maintains the interrupt that the receive IPC statistic counters generate. See Table 4-25 for further detail."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MmcIpcIntrRx(pub u32);
    impl MmcIpcIntrRx {
        #[doc = "MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4gfis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4gfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4herfis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4herfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4nopayfis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4nopayfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4fragfis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4fragfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4udsblfis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4udsblfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6gfis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6gfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6herfis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6herfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6nopayfis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6nopayfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudpgfis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudpgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudperfis(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudperfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcpgfis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcpgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcperfis(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcperfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmpgfis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmpgfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmperfis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmperfis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4gois(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4gois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4herois(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4herois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4nopayois(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4nopayois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4fragois(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4fragois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv4udsblois(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv4udsblois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6gois(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6gois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6herois(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6herois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxipv6nopayois(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxipv6nopayois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudpgois(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudpgois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxudperois(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxudperois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcpgois(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcpgois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtcperois(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxtcperois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmpgois(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmpgois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[must_use]
        #[inline(always)]
        pub const fn rxicmperois(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
        #[inline(always)]
        pub const fn set_rxicmperois(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MmcIpcIntrRx {
        #[inline(always)]
        fn default() -> MmcIpcIntrRx {
            MmcIpcIntrRx(0)
        }
    }
    impl core::fmt::Debug for MmcIpcIntrRx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MmcIpcIntrRx")
                .field("rxipv4gfis", &self.rxipv4gfis())
                .field("rxipv4herfis", &self.rxipv4herfis())
                .field("rxipv4nopayfis", &self.rxipv4nopayfis())
                .field("rxipv4fragfis", &self.rxipv4fragfis())
                .field("rxipv4udsblfis", &self.rxipv4udsblfis())
                .field("rxipv6gfis", &self.rxipv6gfis())
                .field("rxipv6herfis", &self.rxipv6herfis())
                .field("rxipv6nopayfis", &self.rxipv6nopayfis())
                .field("rxudpgfis", &self.rxudpgfis())
                .field("rxudperfis", &self.rxudperfis())
                .field("rxtcpgfis", &self.rxtcpgfis())
                .field("rxtcperfis", &self.rxtcperfis())
                .field("rxicmpgfis", &self.rxicmpgfis())
                .field("rxicmperfis", &self.rxicmperfis())
                .field("rxipv4gois", &self.rxipv4gois())
                .field("rxipv4herois", &self.rxipv4herois())
                .field("rxipv4nopayois", &self.rxipv4nopayois())
                .field("rxipv4fragois", &self.rxipv4fragois())
                .field("rxipv4udsblois", &self.rxipv4udsblois())
                .field("rxipv6gois", &self.rxipv6gois())
                .field("rxipv6herois", &self.rxipv6herois())
                .field("rxipv6nopayois", &self.rxipv6nopayois())
                .field("rxudpgois", &self.rxudpgois())
                .field("rxudperois", &self.rxudperois())
                .field("rxtcpgois", &self.rxtcpgois())
                .field("rxtcperois", &self.rxtcperois())
                .field("rxicmpgois", &self.rxicmpgois())
                .field("rxicmperois", &self.rxicmperois())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MmcIpcIntrRx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MmcIpcIntrRx {{ rxipv4gfis: {=bool:?}, rxipv4herfis: {=bool:?}, rxipv4nopayfis: {=bool:?}, rxipv4fragfis: {=bool:?}, rxipv4udsblfis: {=bool:?}, rxipv6gfis: {=bool:?}, rxipv6herfis: {=bool:?}, rxipv6nopayfis: {=bool:?}, rxudpgfis: {=bool:?}, rxudperfis: {=bool:?}, rxtcpgfis: {=bool:?}, rxtcperfis: {=bool:?}, rxicmpgfis: {=bool:?}, rxicmperfis: {=bool:?}, rxipv4gois: {=bool:?}, rxipv4herois: {=bool:?}, rxipv4nopayois: {=bool:?}, rxipv4fragois: {=bool:?}, rxipv4udsblois: {=bool:?}, rxipv6gois: {=bool:?}, rxipv6herois: {=bool:?}, rxipv6nopayois: {=bool:?}, rxudpgois: {=bool:?}, rxudperois: {=bool:?}, rxtcpgois: {=bool:?}, rxtcperois: {=bool:?}, rxicmpgois: {=bool:?}, rxicmperois: {=bool:?} }}" , self . rxipv4gfis () , self . rxipv4herfis () , self . rxipv4nopayfis () , self . rxipv4fragfis () , self . rxipv4udsblfis () , self . rxipv6gfis () , self . rxipv6herfis () , self . rxipv6nopayfis () , self . rxudpgfis () , self . rxudperfis () , self . rxtcpgfis () , self . rxtcperfis () , self . rxicmpgfis () , self . rxicmperfis () , self . rxipv4gois () , self . rxipv4herois () , self . rxipv4nopayois () , self . rxipv4fragois () , self . rxipv4udsblois () , self . rxipv6gois () , self . rxipv6herois () , self . rxipv6nopayois () , self . rxudpgois () , self . rxudperois () , self . rxtcpgois () , self . rxtcperois () , self . rxicmpgois () , self . rxicmperois ())
        }
    }
    #[doc = "PMT Control and Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PmtCsr(pub u32);
    impl PmtCsr {
        #[doc = "Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by the MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high. Note: You can gate-off the CSR clock during the power-down mode. However, when the CSR clock is gated-off, you cannot perform any read or write operations on this register. Therefore, the Software cannot clear this bit."]
        #[inline(always)]
        pub const fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
        #[must_use]
        #[inline(always)]
        pub const fn mgkpkten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
        #[inline(always)]
        pub const fn set_mgkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkpkten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
        #[inline(always)]
        pub const fn set_rwkpkten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
        #[must_use]
        #[inline(always)]
        pub const fn mgkprcvd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet Received When set, this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
        #[inline(always)]
        pub const fn set_mgkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkprcvd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
        #[inline(always)]
        pub const fn set_rwkprcvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
        #[must_use]
        #[inline(always)]
        pub const fn glblucast(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global Unicast When set, enables any unicast packet filtered by the MAC (DAF) address recognition to be a remote wake-up frame."]
        #[inline(always)]
        pub const fn set_glblucast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkptr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Remote Wake-up FIFO Pointer This field gives the current value (0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is equal to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
        #[inline(always)]
        pub const fn set_rwkptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn rwkfiltrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Remote Wake-Up Frame Filter Register Pointer Reset When this bit is set, it resets the remote wake-up frame filter register pointer to 3’b000. It is automatically cleared after 1 clock cycle."]
        #[inline(always)]
        pub const fn set_rwkfiltrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PmtCsr {
        #[inline(always)]
        fn default() -> PmtCsr {
            PmtCsr(0)
        }
    }
    impl core::fmt::Debug for PmtCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PmtCsr")
                .field("pwrdwn", &self.pwrdwn())
                .field("mgkpkten", &self.mgkpkten())
                .field("rwkpkten", &self.rwkpkten())
                .field("mgkprcvd", &self.mgkprcvd())
                .field("rwkprcvd", &self.rwkprcvd())
                .field("glblucast", &self.glblucast())
                .field("rwkptr", &self.rwkptr())
                .field("rwkfiltrst", &self.rwkfiltrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PmtCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PmtCsr {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, rwkpkten: {=bool:?}, mgkprcvd: {=bool:?}, rwkprcvd: {=bool:?}, glblucast: {=bool:?}, rwkptr: {=u8:?}, rwkfiltrst: {=bool:?} }}" , self . pwrdwn () , self . mgkpkten () , self . rwkpkten () , self . mgkprcvd () , self . rwkprcvd () , self . glblucast () , self . rwkptr () , self . rwkfiltrst ())
        }
    }
    #[doc = "PPS0 Interval Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pps0Interval(pub u32);
    impl Pps0Interval {
        #[doc = "PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS0 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS0 Output Signal Interval These bits store the interval between the rising edges of PPS0 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS0 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pps0Interval {
        #[inline(always)]
        fn default() -> Pps0Interval {
            Pps0Interval(0)
        }
    }
    impl core::fmt::Debug for Pps0Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pps0Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pps0Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pps0Interval {{ ppsint: {=u32:?} }}", self.ppsint())
        }
    }
    #[doc = "PPS0 Width Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pps0Width(pub u32);
    impl Pps0Width {
        #[doc = "PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn ppswidth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS0 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS0 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS0 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
        #[inline(always)]
        pub const fn set_ppswidth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pps0Width {
        #[inline(always)]
        fn default() -> Pps0Width {
            Pps0Width(0)
        }
    }
    impl core::fmt::Debug for Pps0Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pps0Width")
                .field("ppswidth", &self.ppswidth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pps0Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pps0Width {{ ppswidth: {=u32:?} }}", self.ppswidth())
        }
    }
    #[doc = "PPS Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsCtrl(pub u32);
    impl PpsCtrl {
        #[doc = "PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high) PPSCMD0: Flexible PPS0 Output Control 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers and of a duration defined in the PPS0 Width Register. 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by ‘STOP Pulse train at time’ or ‘STOP Pulse Train immediately’ commands. 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD = 0010) after the time programmed in the Target Time registers elapses. 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD = 0010). 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111-1111: Reserved Note: These bits get cleared automatically."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsctrlcmd0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high) PPSCMD0: Flexible PPS0 Output Control 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers and of a duration defined in the PPS0 Width Register. 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by ‘STOP Pulse train at time’ or ‘STOP Pulse Train immediately’ commands. 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD = 0010) after the time programmed in the Target Time registers elapses. 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD = 0010). 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111-1111: Reserved Note: These bits get cleared automatically."]
        #[inline(always)]
        pub const fn set_ppsctrlcmd0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsen0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
        #[inline(always)]
        pub const fn set_ppsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel0(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
        #[inline(always)]
        pub const fn set_trgtmodsel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[must_use]
        #[inline(always)]
        pub const fn ppscmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[inline(always)]
        pub const fn set_ppscmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Flexible PPS1 Output Mode Enable When set high, Bits\\[10:8\\]
function as PPSCMD."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsen1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible PPS1 Output Mode Enable When set high, Bits\\[10:8\\]
function as PPSCMD."]
        #[inline(always)]
        pub const fn set_ppsen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel1(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[inline(always)]
        pub const fn set_trgtmodsel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[must_use]
        #[inline(always)]
        pub const fn ppscmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[inline(always)]
        pub const fn set_ppscmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel2(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[inline(always)]
        pub const fn set_trgtmodsel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[must_use]
        #[inline(always)]
        pub const fn ppscmd3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
        #[inline(always)]
        pub const fn set_ppscmd3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtmodsel3(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
        #[inline(always)]
        pub const fn set_trgtmodsel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for PpsCtrl {
        #[inline(always)]
        fn default() -> PpsCtrl {
            PpsCtrl(0)
        }
    }
    impl core::fmt::Debug for PpsCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PpsCtrl")
                .field("ppsctrlcmd0", &self.ppsctrlcmd0())
                .field("ppsen0", &self.ppsen0())
                .field("trgtmodsel0", &self.trgtmodsel0())
                .field("ppscmd1", &self.ppscmd1())
                .field("ppsen1", &self.ppsen1())
                .field("trgtmodsel1", &self.trgtmodsel1())
                .field("ppscmd2", &self.ppscmd2())
                .field("trgtmodsel2", &self.trgtmodsel2())
                .field("ppscmd3", &self.ppscmd3())
                .field("trgtmodsel3", &self.trgtmodsel3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PpsCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PpsCtrl {{ ppsctrlcmd0: {=u8:?}, ppsen0: {=bool:?}, trgtmodsel0: {=u8:?}, ppscmd1: {=u8:?}, ppsen1: {=bool:?}, trgtmodsel1: {=u8:?}, ppscmd2: {=u8:?}, trgtmodsel2: {=u8:?}, ppscmd3: {=u8:?}, trgtmodsel3: {=u8:?} }}" , self . ppsctrlcmd0 () , self . ppsen0 () , self . trgtmodsel0 () , self . ppscmd1 () , self . ppsen1 () , self . trgtmodsel1 () , self . ppscmd2 () , self . trgtmodsel2 () , self . ppscmd3 () , self . trgtmodsel3 ())
        }
    }
    #[doc = "PPS Target Time Nanoseconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsTgttmNsec(pub u32);
    impl PpsTgttmNsec {
        #[doc = "Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
        #[must_use]
        #[inline(always)]
        pub const fn ttsl1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Target Time Low for PPS1 Register This register stores the time in (signed) nanoseconds. When the value of the timestamp matches the both Target Timestamp registers, then based on the TRGTMODSEL1 field (Bits \\[14:13\\]) in Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled). This value should not exceed 0x3B9A_C9FF when Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register). The actual start or stop time of the PPS signal output may have an error margin up to one unit of sub-second increment value."]
        #[inline(always)]
        pub const fn set_ttsl1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn trgtbusy1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PPS1 Target Time Register Busy The MAC sets this bit when the PPSCMD1 field (Bits \\[10:8\\]) in Register 459 (PPS Control Register) is programmed to 010 or 011. Programming the PPSCMD1 field to 010 or 011 instructs the MAC to synchronize the Target Time Registers to the PTP clock domain. The MAC clears this bit after synchronizing the Target Time Registers to the PTP clock domain The application must not update the Target Time Registers when this bit is read as 1. Otherwise, the synchronization of the previous programmed time gets corrupted."]
        #[inline(always)]
        pub const fn set_trgtbusy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PpsTgttmNsec {
        #[inline(always)]
        fn default() -> PpsTgttmNsec {
            PpsTgttmNsec(0)
        }
    }
    impl core::fmt::Debug for PpsTgttmNsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PpsTgttmNsec")
                .field("ttsl1", &self.ttsl1())
                .field("trgtbusy1", &self.trgtbusy1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PpsTgttmNsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PpsTgttmNsec {{ ttsl1: {=u32:?}, trgtbusy1: {=bool:?} }}",
                self.ttsl1(),
                self.trgtbusy1()
            )
        }
    }
    #[doc = "PPS Target Time Seconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsTgttmSec(pub u32);
    impl PpsTgttmSec {
        #[doc = "PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrh1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS1 Target Time Seconds Register This register stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, then based on Bits \\[14:13\\], TRGTMODSEL1, of Register 459 (PPS Control Register), the MAC starts or stops the PPS signal output and generates an interrupt (if enabled)."]
        #[inline(always)]
        pub const fn set_tstrh1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PpsTgttmSec {
        #[inline(always)]
        fn default() -> PpsTgttmSec {
            PpsTgttmSec(0)
        }
    }
    impl core::fmt::Debug for PpsTgttmSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PpsTgttmSec")
                .field("tstrh1", &self.tstrh1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PpsTgttmSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PpsTgttmSec {{ tstrh1: {=u32:?} }}", self.tstrh1())
        }
    }
    #[doc = "Remote Wake-Up Frame Filter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwkfrmfilt(pub u32);
    impl Rwkfrmfilt {
        #[doc = "This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupfrmfilt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers."]
        #[inline(always)]
        pub const fn set_wkupfrmfilt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rwkfrmfilt {
        #[inline(always)]
        fn default() -> Rwkfrmfilt {
            Rwkfrmfilt(0)
        }
    }
    impl core::fmt::Debug for Rwkfrmfilt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rwkfrmfilt")
                .field("wkupfrmfilt", &self.wkupfrmfilt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rwkfrmfilt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rwkfrmfilt {{ wkupfrmfilt: {=u32:?} }}",
                self.wkupfrmfilt()
            )
        }
    }
    #[doc = "Number of good and bad frames received."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxframecountGb(pub u32);
    impl RxframecountGb {
        #[doc = "Number of good and bad frames received."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames received."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxframecountGb {
        #[inline(always)]
        fn default() -> RxframecountGb {
            RxframecountGb(0)
        }
    }
    impl core::fmt::Debug for RxframecountGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxframecountGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxframecountGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxframecountGb {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxipv4GdFms(pub u32);
    impl Rxipv4GdFms {
        #[doc = "Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxipv4GdFms {
        #[inline(always)]
        fn default() -> Rxipv4GdFms {
            Rxipv4GdFms(0)
        }
    }
    impl core::fmt::Debug for Rxipv4GdFms {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxipv4GdFms")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxipv4GdFms {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxipv4GdFms {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Sub-Second Increment Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SubSecIncr(pub u32);
    impl SubSecIncr {
        #[doc = "Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
        #[must_use]
        #[inline(always)]
        pub const fn ssinc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
        #[inline(always)]
        pub const fn set_ssinc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SubSecIncr {
        #[inline(always)]
        fn default() -> SubSecIncr {
            SubSecIncr(0)
        }
    }
    impl core::fmt::Debug for SubSecIncr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SubSecIncr")
                .field("ssinc", &self.ssinc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SubSecIncr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SubSecIncr {{ ssinc: {=u8:?} }}", self.ssinc())
        }
    }
    #[doc = "System Time - Nanoseconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystNsec(pub u32);
    impl SystNsec {
        #[doc = "Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the maximum value is 0x3B9A_C9FF, after which it rolls-over to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the maximum value is 0x3B9A_C9FF, after which it rolls-over to zero."]
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for SystNsec {
        #[inline(always)]
        fn default() -> SystNsec {
            SystNsec(0)
        }
    }
    impl core::fmt::Debug for SystNsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SystNsec")
                .field("tsss", &self.tsss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SystNsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SystNsec {{ tsss: {=u32:?} }}", self.tsss())
        }
    }
    #[doc = "System Time - Nanoseconds Update Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystNsecUpd(pub u32);
    impl SystNsecUpd {
        #[doc = "Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 0.46 ns. When Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register), each bit represents 1 ns and the programmed value should not exceed 0x3B9A_C9FF."]
        #[inline(always)]
        pub const fn set_tsss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
        #[must_use]
        #[inline(always)]
        pub const fn addsub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add or Subtract Time When this bit is set, the time value is subtracted with the contents of the update register. When this bit is reset, the time value is added with the contents of the update register."]
        #[inline(always)]
        pub const fn set_addsub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SystNsecUpd {
        #[inline(always)]
        fn default() -> SystNsecUpd {
            SystNsecUpd(0)
        }
    }
    impl core::fmt::Debug for SystNsecUpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SystNsecUpd")
                .field("tsss", &self.tsss())
                .field("addsub", &self.addsub())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SystNsecUpd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SystNsecUpd {{ tsss: {=u32:?}, addsub: {=bool:?} }}",
                self.tsss(),
                self.addsub()
            )
        }
    }
    #[doc = "System Time - Seconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystSec(pub u32);
    impl SystSec {
        #[doc = "Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SystSec {
        #[inline(always)]
        fn default() -> SystSec {
            SystSec(0)
        }
    }
    impl core::fmt::Debug for SystSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SystSec").field("tss", &self.tss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SystSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SystSec {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "System Time - Seconds Update Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystSecUpd(pub u32);
    impl SystSecUpd {
        #[doc = "Timestamp Second The value in this field indicates the time in seconds to be initialized or added to the system time."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Second The value in this field indicates the time in seconds to be initialized or added to the system time."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SystSecUpd {
        #[inline(always)]
        fn default() -> SystSecUpd {
            SystSecUpd(0)
        }
    }
    impl core::fmt::Debug for SystSecUpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SystSecUpd")
                .field("tss", &self.tss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SystSecUpd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SystSecUpd {{ tss: {=u32:?} }}", self.tss())
        }
    }
    #[doc = "System Time - Higher Word Seconds Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SystmHSec(pub u32);
    impl SystmHSec {
        #[doc = "Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
        #[must_use]
        #[inline(always)]
        pub const fn tshwr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp Higher Word Register This field contains the most significant 16-bits of the timestamp seconds value. This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration. The register is directly written to initialize the value. This register is incremented when there is an overflow from the 32-bits of the System Time - Seconds register."]
        #[inline(always)]
        pub const fn set_tshwr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SystmHSec {
        #[inline(always)]
        fn default() -> SystmHSec {
            SystmHSec(0)
        }
    }
    impl core::fmt::Debug for SystmHSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SystmHSec")
                .field("tshwr", &self.tshwr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SystmHSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SystmHSec {{ tshwr: {=u16:?} }}", self.tshwr())
        }
    }
    #[doc = "Timestamp Addend Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsAddend(pub u32);
    impl TsAddend {
        #[doc = "Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn tsar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
        #[inline(always)]
        pub const fn set_tsar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsAddend {
        #[inline(always)]
        fn default() -> TsAddend {
            TsAddend(0)
        }
    }
    impl core::fmt::Debug for TsAddend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsAddend")
                .field("tsar", &self.tsar())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsAddend {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsAddend {{ tsar: {=u32:?} }}", self.tsar())
        }
    }
    #[doc = "Timestamp Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsCtrl(pub u32);
    impl TsCtrl {
        #[doc = "Timestamp Enable When set, the timestamp is added for the transmit and receive frames. When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the receive side, the MAC processes the 1588 frames only if this bit is set."]
        #[must_use]
        #[inline(always)]
        pub const fn tsena(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Enable When set, the timestamp is added for the transmit and receive frames. When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended. You need to initialize the Timestamp (system time) after enabling this mode. On the receive side, the MAC processes the 1588 frames only if this bit is set."]
        #[inline(always)]
        pub const fn set_tsena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method. When reset, it indicates the system timestamp update should be done using the Coarse method."]
        #[must_use]
        #[inline(always)]
        pub const fn tscfupdt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method. When reset, it indicates the system timestamp update should be done using the Coarse method."]
        #[inline(always)]
        pub const fn set_tscfupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timestamp Initialize When set, the system time is initialized (overwritten) with the value specified in the Register 452 (System Time – Seconds Update Register) and Register 453 (System Time – Nanoseconds Update Register). This bit should be read zero before updating it. This bit is reset when the initialization is complete. The “Timestamp Higher Word” register (if enabled during core configuration) can only be initialized."]
        #[must_use]
        #[inline(always)]
        pub const fn tsinit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Initialize When set, the system time is initialized (overwritten) with the value specified in the Register 452 (System Time – Seconds Update Register) and Register 453 (System Time – Nanoseconds Update Register). This bit should be read zero before updating it. This bit is reset when the initialization is complete. The “Timestamp Higher Word” register (if enabled during core configuration) can only be initialized."]
        #[inline(always)]
        pub const fn set_tsinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp Update When set, the system time is updated (added or subtracted) with the value specified in Register 452 (System Time – Seconds Update Register) and Register 453 (System Time – Nanoseconds Update Register). This bit should be read zero before updating it. This bit is reset when the update is completed in hardware. The “Timestamp Higher Word” register (if enabled during core configuration) is not updated."]
        #[must_use]
        #[inline(always)]
        pub const fn tsupdt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Update When set, the system time is updated (added or subtracted) with the value specified in Register 452 (System Time – Seconds Update Register) and Register 453 (System Time – Nanoseconds Update Register). This bit should be read zero before updating it. This bit is reset when the update is completed in hardware. The “Timestamp Higher Word” register (if enabled during core configuration) is not updated."]
        #[inline(always)]
        pub const fn set_tsupdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register. This bit is reset after the generation of the Timestamp Trigger Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register. This bit is reset after the generation of the Timestamp Trigger Interrupt."]
        #[inline(always)]
        pub const fn set_tstrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This is cleared when the update is completed. This register bit should be zero before setting it."]
        #[must_use]
        #[inline(always)]
        pub const fn tsaddreg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction. This is cleared when the update is completed. This register bit should be zero before setting it."]
        #[inline(always)]
        pub const fn set_tsaddreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC."]
        #[must_use]
        #[inline(always)]
        pub const fn tsenall(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC."]
        #[inline(always)]
        pub const fn set_tsenall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When reset, the rollover value of sub-second register is 0x7FFF_FFFF. The sub-second increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn tsctrlssr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds. When reset, the rollover value of sub-second register is 0x7FFF_FFFF. The sub-second increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit."]
        #[inline(always)]
        pub const fn set_tsctrlssr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format. Otherwise, the PTP packets are processed using the version 1 format."]
        #[must_use]
        #[inline(always)]
        pub const fn tsver2ena(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format. Otherwise, the PTP packets are processed using the version 1 format."]
        #[inline(always)]
        pub const fn set_tsver2ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames. When this bit is clear, the MAC ignores the PTP over Ethernet packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipena(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames. When this bit is clear, the MAC ignores the PTP over Ethernet packets."]
        #[inline(always)]
        pub const fn set_tsipena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Processing of PTP Frames Sent over IPv6-UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets. When this bit is clear, the MAC ignores the PTP transported over UDP-IPv6 packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipv6ena(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Frames Sent over IPv6-UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets. When this bit is clear, the MAC ignores the PTP transported over UDP-IPv6 packets."]
        #[inline(always)]
        pub const fn set_tsipv6ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Processing of PTP Frames Sent over IPv4-UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets. When this bit is clear, the MAC ignores the PTP transported over UDP-IPv4 packets. This bit is set by default."]
        #[must_use]
        #[inline(always)]
        pub const fn tsipv4ena(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Processing of PTP Frames Sent over IPv4-UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets. When this bit is clear, the MAC ignores the PTP transported over UDP-IPv4 packets. This bit is set by default."]
        #[inline(always)]
        pub const fn set_tsipv4ena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When reset, the snapshot is taken for all messages except Announce, Management, and Signaling."]
        #[must_use]
        #[inline(always)]
        pub const fn tsevntena(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp). When reset, the snapshot is taken for all messages except Announce, Management, and Signaling."]
        #[inline(always)]
        pub const fn set_tsevntena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
        #[must_use]
        #[inline(always)]
        pub const fn tsmstrena(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node. Otherwise, the snapshot is taken for the messages relevant to the slave node."]
        #[inline(always)]
        pub const fn set_tsmstrena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken."]
        #[must_use]
        #[inline(always)]
        pub const fn snaptypsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken."]
        #[inline(always)]
        pub const fn set_snaptypsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Enable MAC address for PTP Frame Filtering When set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP frames when PTP is directly sent over Ethernet."]
        #[must_use]
        #[inline(always)]
        pub const fn tsenmacaddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable MAC address for PTP Frame Filtering When set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP frames when PTP is directly sent over Ethernet."]
        #[inline(always)]
        pub const fn set_tsenmacaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, auxiliary snapshots get stored in the FIFO. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn atsfc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO. This bit is cleared when the pointers are reset and the FIFO is empty. When this bit is high, auxiliary snapshots get stored in the FIFO. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration."]
        #[inline(always)]
        pub const fn set_atsfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\]
input is enabled. When this bit is reset, the events on this input are ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\]
input is enabled. When this bit is reset, the events on this input are ignored."]
        #[inline(always)]
        pub const fn set_atsen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two."]
        #[inline(always)]
        pub const fn set_atsen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen2(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three."]
        #[inline(always)]
        pub const fn set_atsen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen3(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3. When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\]
input is enabled. When this bit is reset, the events on this input are ignored. This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four."]
        #[inline(always)]
        pub const fn set_atsen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for TsCtrl {
        #[inline(always)]
        fn default() -> TsCtrl {
            TsCtrl(0)
        }
    }
    impl core::fmt::Debug for TsCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsCtrl")
                .field("tsena", &self.tsena())
                .field("tscfupdt", &self.tscfupdt())
                .field("tsinit", &self.tsinit())
                .field("tsupdt", &self.tsupdt())
                .field("tstrig", &self.tstrig())
                .field("tsaddreg", &self.tsaddreg())
                .field("tsenall", &self.tsenall())
                .field("tsctrlssr", &self.tsctrlssr())
                .field("tsver2ena", &self.tsver2ena())
                .field("tsipena", &self.tsipena())
                .field("tsipv6ena", &self.tsipv6ena())
                .field("tsipv4ena", &self.tsipv4ena())
                .field("tsevntena", &self.tsevntena())
                .field("tsmstrena", &self.tsmstrena())
                .field("snaptypsel", &self.snaptypsel())
                .field("tsenmacaddr", &self.tsenmacaddr())
                .field("atsfc", &self.atsfc())
                .field("atsen0", &self.atsen0())
                .field("atsen1", &self.atsen1())
                .field("atsen2", &self.atsen2())
                .field("atsen3", &self.atsen3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsCtrl {{ tsena: {=bool:?}, tscfupdt: {=bool:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tstrig: {=bool:?}, tsaddreg: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {=bool:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, atsfc: {=bool:?}, atsen0: {=bool:?}, atsen1: {=bool:?}, atsen2: {=bool:?}, atsen3: {=bool:?} }}" , self . tsena () , self . tscfupdt () , self . tsinit () , self . tsupdt () , self . tstrig () , self . tsaddreg () , self . tsenall () , self . tsctrlssr () , self . tsver2ena () , self . tsipena () , self . tsipv6ena () , self . tsipv4ena () , self . tsevntena () , self . tsmstrena () , self . snaptypsel () , self . tsenmacaddr () , self . atsfc () , self . atsen0 () , self . atsen1 () , self . atsen2 () , self . atsen3 ())
        }
    }
    #[doc = "Timestamp Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsStatus(pub u32);
    impl TsStatus {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tssovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tssovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstargt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstargt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn auxtstrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_auxtstrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstrgterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstargt1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstargt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstrgterr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstargt2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstargt2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tstrgterr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 (PPS3 Target Time High Register) and Register 497 (PPS3 Target Time Low Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn tstargt3(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 (PPS3 Target Time High Register) and Register 497 (PPS3 Target Time Low Register)."]
        #[inline(always)]
        pub const fn set_tstargt3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed. This bit is cleared when read by the application."]
        #[must_use]
        #[inline(always)]
        pub const fn tstrgterr3(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed. This bit is cleared when read by the application."]
        #[inline(always)]
        pub const fn set_tstrgterr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: - Bit 16: Auxiliary trigger 0 - Bit 17: Auxiliary trigger 1 - Bit 18: Auxiliary trigger 2 - Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
        #[must_use]
        #[inline(always)]
        pub const fn atsstn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable. When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock. These bits are applicable only if the number of Auxiliary snapshots is more than one. One bit is assigned for each trigger as shown in the following list: - Bit 16: Auxiliary trigger 0 - Bit 17: Auxiliary trigger 1 - Bit 18: Auxiliary trigger 2 - Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken."]
        #[inline(always)]
        pub const fn set_atsstn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn atsstm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Timestamp Snapshot Trigger Missed This bit is set when the Auxiliary timestamp snapshot FIFO is full and external trigger was set. This indicates that the latest snapshot is not stored in the FIFO. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
        #[inline(always)]
        pub const fn set_atsstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the selected depth of FIFO (4, 8, or 16) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn atsns(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO. A value equal to the selected depth of FIFO (4, 8, or 16) indicates that the Auxiliary Snapshot FIFO is full. These bits are cleared (to 00000) when the Auxiliary snapshot FIFO clear bit is set. This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration."]
        #[inline(always)]
        pub const fn set_atsns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for TsStatus {
        #[inline(always)]
        fn default() -> TsStatus {
            TsStatus(0)
        }
    }
    impl core::fmt::Debug for TsStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsStatus")
                .field("tssovf", &self.tssovf())
                .field("tstargt", &self.tstargt())
                .field("auxtstrig", &self.auxtstrig())
                .field("tstrgterr", &self.tstrgterr())
                .field("tstargt1", &self.tstargt1())
                .field("tstrgterr1", &self.tstrgterr1())
                .field("tstargt2", &self.tstargt2())
                .field("tstrgterr2", &self.tstrgterr2())
                .field("tstargt3", &self.tstargt3())
                .field("tstrgterr3", &self.tstrgterr3())
                .field("atsstn", &self.atsstn())
                .field("atsstm", &self.atsstm())
                .field("atsns", &self.atsns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsStatus {{ tssovf: {=bool:?}, tstargt: {=bool:?}, auxtstrig: {=bool:?}, tstrgterr: {=bool:?}, tstargt1: {=bool:?}, tstrgterr1: {=bool:?}, tstargt2: {=bool:?}, tstrgterr2: {=bool:?}, tstargt3: {=bool:?}, tstrgterr3: {=bool:?}, atsstn: {=u8:?}, atsstm: {=bool:?}, atsns: {=u8:?} }}" , self . tssovf () , self . tstargt () , self . auxtstrig () , self . tstrgterr () , self . tstargt1 () , self . tstrgterr1 () , self . tstargt2 () , self . tstrgterr2 () , self . tstargt3 () , self . tstrgterr3 () , self . atsstn () , self . atsstm () , self . atsns ())
        }
    }
    #[doc = "Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx1024tomaxoctetsGb(pub u32);
    impl Tx1024tomaxoctetsGb {
        #[doc = "Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx1024tomaxoctetsGb {
        #[inline(always)]
        fn default() -> Tx1024tomaxoctetsGb {
            Tx1024tomaxoctetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx1024tomaxoctetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx1024tomaxoctetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx1024tomaxoctetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tx1024tomaxoctetsGb {{ frmcnt: {=u32:?} }}",
                self.frmcnt()
            )
        }
    }
    #[doc = "Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx128to255octetsGb(pub u32);
    impl Tx128to255octetsGb {
        #[doc = "Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx128to255octetsGb {
        #[inline(always)]
        fn default() -> Tx128to255octetsGb {
            Tx128to255octetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx128to255octetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx128to255octetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx128to255octetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tx128to255octetsGb {{ frmcnt: {=u32:?} }}",
                self.frmcnt()
            )
        }
    }
    #[doc = "Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx256to511octetsGb(pub u32);
    impl Tx256to511octetsGb {
        #[doc = "Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx256to511octetsGb {
        #[inline(always)]
        fn default() -> Tx256to511octetsGb {
            Tx256to511octetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx256to511octetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx256to511octetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx256to511octetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tx256to511octetsGb {{ frmcnt: {=u32:?} }}",
                self.frmcnt()
            )
        }
    }
    #[doc = "Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx512to1023octetsGb(pub u32);
    impl Tx512to1023octetsGb {
        #[doc = "Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx512to1023octetsGb {
        #[inline(always)]
        fn default() -> Tx512to1023octetsGb {
            Tx512to1023octetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx512to1023octetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx512to1023octetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx512to1023octetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tx512to1023octetsGb {{ frmcnt: {=u32:?} }}",
                self.frmcnt()
            )
        }
    }
    #[doc = "Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx64octetsGb(pub u32);
    impl Tx64octetsGb {
        #[doc = "Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx64octetsGb {
        #[inline(always)]
        fn default() -> Tx64octetsGb {
            Tx64octetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx64octetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx64octetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx64octetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tx64octetsGb {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx65to127octetsGb(pub u32);
    impl Tx65to127octetsGb {
        #[doc = "Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tx65to127octetsGb {
        #[inline(always)]
        fn default() -> Tx65to127octetsGb {
            Tx65to127octetsGb(0)
        }
    }
    impl core::fmt::Debug for Tx65to127octetsGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tx65to127octetsGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tx65to127octetsGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tx65to127octetsGb {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Number of good broadcast frames transmitted."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxbroadcastframesG(pub u32);
    impl TxbroadcastframesG {
        #[doc = "Number of good broadcast frames transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good broadcast frames transmitted."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxbroadcastframesG {
        #[inline(always)]
        fn default() -> TxbroadcastframesG {
            TxbroadcastframesG(0)
        }
    }
    impl core::fmt::Debug for TxbroadcastframesG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxbroadcastframesG")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxbroadcastframesG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxbroadcastframesG {{ frmcnt: {=u32:?} }}",
                self.frmcnt()
            )
        }
    }
    #[doc = "Number of good and bad frames transmitted, exclusive of retried frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxframecountGb(pub u32);
    impl TxframecountGb {
        #[doc = "Number of good and bad frames transmitted, exclusive of retried frames."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good and bad frames transmitted, exclusive of retried frames."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxframecountGb {
        #[inline(always)]
        fn default() -> TxframecountGb {
            TxframecountGb(0)
        }
    }
    impl core::fmt::Debug for TxframecountGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxframecountGb")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxframecountGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxframecountGb {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Number of good multicast frames transmitted."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxmlticastframesG(pub u32);
    impl TxmlticastframesG {
        #[doc = "Number of good multicast frames transmitted."]
        #[must_use]
        #[inline(always)]
        pub const fn frmcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of good multicast frames transmitted."]
        #[inline(always)]
        pub const fn set_frmcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxmlticastframesG {
        #[inline(always)]
        fn default() -> TxmlticastframesG {
            TxmlticastframesG(0)
        }
    }
    impl core::fmt::Debug for TxmlticastframesG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxmlticastframesG")
                .field("frmcnt", &self.frmcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxmlticastframesG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxmlticastframesG {{ frmcnt: {=u32:?} }}", self.frmcnt())
        }
    }
    #[doc = "Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxoctetcountGb(pub u32);
    impl TxoctetcountGb {
        #[doc = "Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
        #[must_use]
        #[inline(always)]
        pub const fn bytecnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
        #[inline(always)]
        pub const fn set_bytecnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxoctetcountGb {
        #[inline(always)]
        fn default() -> TxoctetcountGb {
            TxoctetcountGb(0)
        }
    }
    impl core::fmt::Debug for TxoctetcountGb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxoctetcountGb")
                .field("bytecnt", &self.bytecnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxoctetcountGb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxoctetcountGb {{ bytecnt: {=u32:?} }}", self.bytecnt())
        }
    }
    #[doc = "VLAN Hash Table Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VlanHash(pub u32);
    impl VlanHash {
        #[doc = "VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
        #[must_use]
        #[inline(always)]
        pub const fn vlht(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
        #[inline(always)]
        pub const fn set_vlht(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for VlanHash {
        #[inline(always)]
        fn default() -> VlanHash {
            VlanHash(0)
        }
    }
    impl core::fmt::Debug for VlanHash {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VlanHash")
                .field("vlht", &self.vlht())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VlanHash {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VlanHash {{ vlht: {=u16:?} }}", self.vlht())
        }
    }
    #[doc = "VLAN Tag Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VlanTag(pub u32);
    impl VlanTag {
        #[doc = "VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
        #[must_use]
        #[inline(always)]
        pub const fn vl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag Identifier for Receive Frames This field contains the 802.1Q VLAN tag to identify the VLAN frames and is compared to the 15th and 16th bytes of the frames being received for VLAN frames. The following list describes the bits of this field: - Bits \\[15:13\\]: User Priority - Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) - Bits\\[11:0\\]: VLAN tag’s VLAN Identifier (VID) field When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the MAC does not check the fifteenth and 16th bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 or 0x88a8 as VLAN frames."]
        #[inline(always)]
        pub const fn set_vl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn etv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier is used for comparing and filtering instead of the complete 16-bit VLAN tag. Bits \\[11:0\\]
of VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. Similarly, when enabled, only 12 bits of the VLAN tag in the received frame are used for hash-based VLAN filtering. When this bit is reset, all 16 bits of the 15th and 16th bytes of the received VLAN frame are used for comparison and VLAN hash filtering."]
        #[inline(always)]
        pub const fn set_etv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
        #[must_use]
        #[inline(always)]
        pub const fn vtim(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Inverse Match Enable When set, this bit enables the VLAN Tag inverse matching. The frames that do not have matching VLAN Tag are marked as matched. When reset, this bit enables the VLAN Tag perfect matching. The frames with matched VLAN Tag are marked as matched."]
        #[inline(always)]
        pub const fn set_vtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
        #[must_use]
        #[inline(always)]
        pub const fn esvl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable S-VLAN When this bit is set, the MAC transmitter and receiver also consider the S-VLAN (Type = 0x88A8) frames as valid VLAN tagged frames."]
        #[inline(always)]
        pub const fn set_esvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
        #[must_use]
        #[inline(always)]
        pub const fn vthm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Tag Hash Table Match Enable When set, the most significant four bits of the VLAN tag’s CRC are used to index the content of Register 354 (VLAN Hash Table Register). A value of 1 in the VLAN Hash Table register, corresponding to the index, indicates that the frame matched the VLAN hash table. When Bit 16 (ETV) is set, the CRC of the 12-bit VLAN Identifier (VID) is used for comparison whereas when ETV is reset, the CRC of the 16-bit VLAN tag is used for comparison. When reset, the VLAN Hash Match operation is not performed."]
        #[inline(always)]
        pub const fn set_vthm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for VlanTag {
        #[inline(always)]
        fn default() -> VlanTag {
            VlanTag(0)
        }
    }
    impl core::fmt::Debug for VlanTag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VlanTag")
                .field("vl", &self.vl())
                .field("etv", &self.etv())
                .field("vtim", &self.vtim())
                .field("esvl", &self.esvl())
                .field("vthm", &self.vthm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VlanTag {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VlanTag {{ vl: {=u16:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, vthm: {=bool:?} }}" , self . vl () , self . etv () , self . vtim () , self . esvl () , self . vthm ())
        }
    }
    #[doc = "VLAN Tag Inclusion or Replacement Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VlanTagIncRpl(pub u32);
    impl VlanTagIncRpl {
        #[doc = "VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
        #[must_use]
        #[inline(always)]
        pub const fn vlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN Tag for Transmit Frames This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. Bits\\[15:13\\]
are the User Priority, Bit 12 is the CFI/DEI, and Bits\\[11:0\\]
are the VLAN tag’s VID field."]
        #[inline(always)]
        pub const fn set_vlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
        #[must_use]
        #[inline(always)]
        pub const fn vlc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "VLAN Tag Control in Transmit Frames - 2’b00: No VLAN tag deletion, insertion, or replacement - 2’b01: VLAN tag deletion The MAC removes the VLAN type (bytes 13 and 14) and VLAN tag (bytes 15 and 16) of all transmitted frames with VLAN tags. - 2’b10: VLAN tag insertion The MAC inserts VLT in bytes 15 and 16 of the frame after inserting the Type value (0x8100/0x88a8) in bytes 13 and 14. This operation is performed on all transmitted frames, irrespective of whether they already have a VLAN tag. - 2’b11: VLAN tag replacement The MAC replaces VLT in bytes 15 and 16 of all VLAN-type transmitted frames (Bytes 13 and 14 are 0x8100/0x88a8). Note: Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value."]
        #[inline(always)]
        pub const fn set_vlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn vlp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN Priority Control When this bit is set, the control Bits \\[17:16\\]
are used for VLAN deletion, insertion, or replacement. When this bit is reset, the mti_vlan_ctrl_i control input is used, and Bits \\[17:16\\]
are ignored."]
        #[inline(always)]
        pub const fn set_vlp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
        #[must_use]
        #[inline(always)]
        pub const fn csvl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted frames. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the transmitted frames."]
        #[inline(always)]
        pub const fn set_csvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for VlanTagIncRpl {
        #[inline(always)]
        fn default() -> VlanTagIncRpl {
            VlanTagIncRpl(0)
        }
    }
    impl core::fmt::Debug for VlanTagIncRpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VlanTagIncRpl")
                .field("vlt", &self.vlt())
                .field("vlc", &self.vlc())
                .field("vlp", &self.vlp())
                .field("csvl", &self.csvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VlanTagIncRpl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VlanTagIncRpl {{ vlt: {=u16:?}, vlc: {=u8:?}, vlp: {=bool:?}, csvl: {=bool:?} }}",
                self.vlt(),
                self.vlc(),
                self.vlp(),
                self.csvl()
            )
        }
    }
    #[doc = "Watchdog Timeout Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdogWto(pub u32);
    impl WdogWto {
        #[doc = "Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn wto(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Watchdog Timeout When Bit 16 (PWE) is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, this field is used as watchdog timeout for a received frame. If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame. Note: When Bit 16 (PWE) is set, the value in this field should be more than 1,522 (0x05F2). Otherwise, the IEEE Std 802.3-specified valid tagged frames are declared as error frames and are dropped."]
        #[inline(always)]
        pub const fn set_wto(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable Watchdog Enable When this bit is set and Bit 23 (WD) of Register 0 (MAC Configuration Register) is reset, the WTO field (Bits\\[13:0\\]) is used as watchdog timeout for a received frame. When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 (WD) and Bit 20 (JE) in Register 0 (MAC Configuration Register)."]
        #[inline(always)]
        pub const fn set_pwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for WdogWto {
        #[inline(always)]
        fn default() -> WdogWto {
            WdogWto(0)
        }
    }
    impl core::fmt::Debug for WdogWto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdogWto")
                .field("wto", &self.wto())
                .field("pwe", &self.pwe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdogWto {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WdogWto {{ wto: {=u16:?}, pwe: {=bool:?} }}",
                self.wto(),
                self.pwe()
            )
        }
    }
    #[doc = "PPS Width Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Width(pub u32);
    impl Width {
        #[doc = "PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
        #[must_use]
        #[inline(always)]
        pub const fn ppswidth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
        #[inline(always)]
        pub const fn set_ppswidth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Width {
        #[inline(always)]
        fn default() -> Width {
            Width(0)
        }
    }
    impl core::fmt::Debug for Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Width")
                .field("ppswidth", &self.ppswidth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Width {{ ppswidth: {=u32:?} }}", self.ppswidth())
        }
    }
    #[doc = "SGMII/RGMII/SMII Control and Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XmiiCsr(pub u32);
    impl XmiiCsr {
        #[doc = "Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lnkmod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode."]
        #[inline(always)]
        pub const fn set_lnkmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
        #[must_use]
        #[inline(always)]
        pub const fn lnkspeed(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
        #[inline(always)]
        pub const fn set_lnkspeed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
        #[must_use]
        #[inline(always)]
        pub const fn lnksts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
        #[inline(always)]
        pub const fn set_lnksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
        #[must_use]
        #[inline(always)]
        pub const fn jabto(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
        #[inline(always)]
        pub const fn set_jabto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
        #[must_use]
        #[inline(always)]
        pub const fn falscardet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
        #[inline(always)]
        pub const fn set_falscardet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for XmiiCsr {
        #[inline(always)]
        fn default() -> XmiiCsr {
            XmiiCsr(0)
        }
    }
    impl core::fmt::Debug for XmiiCsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XmiiCsr")
                .field("lnkmod", &self.lnkmod())
                .field("lnkspeed", &self.lnkspeed())
                .field("lnksts", &self.lnksts())
                .field("jabto", &self.jabto())
                .field("falscardet", &self.falscardet())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XmiiCsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "XmiiCsr {{ lnkmod: {=bool:?}, lnkspeed: {=u8:?}, lnksts: {=bool:?}, jabto: {=bool:?}, falscardet: {=bool:?} }}" , self . lnkmod () , self . lnkspeed () , self . lnksts () , self . jabto () , self . falscardet ())
        }
    }
}
