#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MIPI_CSI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MipiCsi {
    ptr: *mut u8,
}
unsafe impl Send for MipiCsi {}
unsafe impl Sync for MipiCsi {}
impl MipiCsi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "version code."]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "the number of active lanes."]
    #[inline(always)]
    pub const fn n_lanes(self) -> crate::common::Reg<regs::NLanes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "the internal logic of the controller goes into the reset state when active."]
    #[inline(always)]
    pub const fn csi2_resetn(self) -> crate::common::Reg<regs::Csi2Resetn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "contains the stateus of individual interrupt sources."]
    #[inline(always)]
    pub const fn int_st_main(self) -> crate::common::Reg<regs::IntStMain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[inline(always)]
    pub const fn data_ids_1(self) -> crate::common::Reg<regs::DataIds1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[inline(always)]
    pub const fn data_ids_2(self) -> crate::common::Reg<regs::DataIds2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "contains the status of individual interrupt sources."]
    #[inline(always)]
    pub const fn int_st_ap_main(self) -> crate::common::Reg<regs::IntStApMain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "controls the phy shutdown mode."]
    #[inline(always)]
    pub const fn phy_shutdownz(self) -> crate::common::Reg<regs::PhyShutdownz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "controls the phy reset mode."]
    #[inline(always)]
    pub const fn dphy_rstz(self) -> crate::common::Reg<regs::DphyRstz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "contains the status of rx-related signals from phy."]
    #[inline(always)]
    pub const fn phy_rx(self) -> crate::common::Reg<regs::PhyRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "contains the stopstate signal status from phy."]
    #[inline(always)]
    pub const fn phy_stopstate(self) -> crate::common::Reg<regs::PhyStopstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "selects how the ipi interface generates the video frame."]
    #[inline(always)]
    pub const fn ipi_mode(self) -> crate::common::Reg<regs::IpiMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "selects the vritual channel processed by ipi."]
    #[inline(always)]
    pub const fn ipi_vcid(self) -> crate::common::Reg<regs::IpiVcid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "selects the data type processed by ipi."]
    #[inline(always)]
    pub const fn ipi_data_type(self) -> crate::common::Reg<regs::IpiDataType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "control the flush of ipi memory."]
    #[inline(always)]
    pub const fn ipi_mem_flash(self) -> crate::common::Reg<regs::IpiMemFlash, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "configures the video horizontal synchronism active time."]
    #[inline(always)]
    pub const fn ipi_hsa_time(self) -> crate::common::Reg<regs::IpiHsaTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "configures the video horizontal synchronism back porch time."]
    #[inline(always)]
    pub const fn ipi_hbp_time(self) -> crate::common::Reg<regs::IpiHbpTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "configures the vedeo Horizontal Sync Delay time."]
    #[inline(always)]
    pub const fn ipi_hsd_time(self) -> crate::common::Reg<regs::IpiHsdTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "configures the overall tiem for each video line."]
    #[inline(always)]
    pub const fn ipi_hline_time(self) -> crate::common::Reg<regs::IpiHlineTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "congtrols the ipi logic reset state."]
    #[inline(always)]
    pub const fn ipi_softrstn(self) -> crate::common::Reg<regs::IpiSoftrstn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "configures advanced features for ipi mode."]
    #[inline(always)]
    pub const fn ipi_adv_features(
        self,
    ) -> crate::common::Reg<regs::IpiAdvFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "configures the vertical synchronism active period."]
    #[inline(always)]
    pub const fn ipi_vsa_lines(self) -> crate::common::Reg<regs::IpiVsaLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "configures the verticall back porch period."]
    #[inline(always)]
    pub const fn ipi_vbp_lines(self) -> crate::common::Reg<regs::IpiVbpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "configures the vertical front porch period."]
    #[inline(always)]
    pub const fn ipi_vfp_lines(self) -> crate::common::Reg<regs::IpiVfpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "configures the vertical resolution of video."]
    #[inline(always)]
    pub const fn ipi_vactive_lines(
        self,
    ) -> crate::common::Reg<regs::IpiVactiveLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "active extra bits for virtual channel."]
    #[inline(always)]
    pub const fn vc_extension(self) -> crate::common::Reg<regs::VcExtension, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "contains the calibration signal status from synopsys d-phy."]
    #[inline(always)]
    pub const fn phy_cal(self) -> crate::common::Reg<regs::PhyCal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "groups the phy interruptions caused by phy packets discarded."]
    #[inline(always)]
    pub const fn int_st_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_phy_fatal."]
    #[inline(always)]
    pub const fn int_msk_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "interrupt force register for test purposes."]
    #[inline(always)]
    pub const fn int_force_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "groups the fatal interruption related with packet construction."]
    #[inline(always)]
    pub const fn int_st_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_pkt_fatal."]
    #[inline(always)]
    pub const fn int_msk_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purpos."]
    #[inline(always)]
    pub const fn int_force_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "interruption caused by phy."]
    #[inline(always)]
    pub const fn int_st_phy(self) -> crate::common::Reg<regs::IntStPhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "interrupt mask for int_st_phy."]
    #[inline(always)]
    pub const fn int_msk_phy(self) -> crate::common::Reg<regs::IntMskPhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_phy(self) -> crate::common::Reg<regs::IntForcePhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "fatal interruption caused by ipi interface."]
    #[inline(always)]
    pub const fn int_st_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ipi_fatal."]
    #[inline(always)]
    pub const fn int_msk_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "groups and notifies which interruption bits caused the interruption."]
    #[inline(always)]
    pub const fn int_st_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntStApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ap_generic."]
    #[inline(always)]
    pub const fn int_msk_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntMskApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "interrupt force register used for test purposes."]
    #[inline(always)]
    pub const fn int_force_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntForceApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "groups and notifies which interruption bits."]
    #[inline(always)]
    pub const fn int_st_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ap_ipi_fatal controls."]
    #[inline(always)]
    pub const fn int_msk_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "interrupt mask for int_st_bndry_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "interrupt mask for int_st_seq_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
}
pub mod regs {
    #[doc = "the internal logic of the controller goes into the reset state when active."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csi2Resetn(pub u32);
    impl Csi2Resetn {
        #[doc = "DWC_mipi_csi2_host reset output, active low."]
        #[inline(always)]
        pub const fn csi2_resetn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DWC_mipi_csi2_host reset output, active low."]
        #[inline(always)]
        pub fn set_csi2_resetn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Csi2Resetn {
        #[inline(always)]
        fn default() -> Csi2Resetn {
            Csi2Resetn(0)
        }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataIds1(pub u32);
    impl DataIds1 {
        #[doc = "data type for programmed data ID 0."]
        #[inline(always)]
        pub const fn di0_dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 0."]
        #[inline(always)]
        pub fn set_di0_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "data type for programmed data ID 1."]
        #[inline(always)]
        pub const fn di1_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 1."]
        #[inline(always)]
        pub fn set_di1_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "data type for programmed data ID 2."]
        #[inline(always)]
        pub const fn di2_dt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 2."]
        #[inline(always)]
        pub fn set_di2_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "data type for programmed data ID 3."]
        #[inline(always)]
        pub const fn di3_dt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 3."]
        #[inline(always)]
        pub fn set_di3_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for DataIds1 {
        #[inline(always)]
        fn default() -> DataIds1 {
            DataIds1(0)
        }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataIds2(pub u32);
    impl DataIds2 {
        #[doc = "data type for programmed data ID 4."]
        #[inline(always)]
        pub const fn di4_dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 4."]
        #[inline(always)]
        pub fn set_di4_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "data type for programmed data ID 5."]
        #[inline(always)]
        pub const fn di5_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 5."]
        #[inline(always)]
        pub fn set_di5_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "data type for programmed data ID 6."]
        #[inline(always)]
        pub const fn di6_dt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 6."]
        #[inline(always)]
        pub fn set_di6_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "data type for programmed data ID 7."]
        #[inline(always)]
        pub const fn di7_dt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 7."]
        #[inline(always)]
        pub fn set_di7_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for DataIds2 {
        #[inline(always)]
        fn default() -> DataIds2 {
            DataIds2(0)
        }
    }
    #[doc = "controls the phy reset mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DphyRstz(pub u32);
    impl DphyRstz {
        #[doc = "phy reset output, active low."]
        #[inline(always)]
        pub const fn dphy_rstz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "phy reset output, active low."]
        #[inline(always)]
        pub fn set_dphy_rstz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DphyRstz {
        #[inline(always)]
        fn default() -> DphyRstz {
            DphyRstz(0)
        }
    }
    #[doc = "interrupt force register used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceApGeneric(pub u32);
    impl IntForceApGeneric {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntForceApGeneric {
        #[inline(always)]
        fn default() -> IntForceApGeneric {
            IntForceApGeneric(0)
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceApIpiFatal(pub u32);
    impl IntForceApIpiFatal {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn force_redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_force_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntForceApIpiFatal {
        #[inline(always)]
        fn default() -> IntForceApIpiFatal {
            IntForceApIpiFatal(0)
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceBndryFrameFatal(pub u32);
    impl IntForceBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_force_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntForceBndryFrameFatal {
            IntForceBndryFrameFatal(0)
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceCrcFrameFatal(pub u32);
    impl IntForceCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_force_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntForceCrcFrameFatal {
            IntForceCrcFrameFatal(0)
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceIpiFatal(pub u32);
    impl IntForceIpiFatal {
        #[doc = "force for pixel_if_fifo_underflow."]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force for pixel_if_fifo_underflow."]
        #[inline(always)]
        pub fn set_force_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub fn set_force_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force for frame_sync_err."]
        #[inline(always)]
        pub const fn force_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force for frame_sync_err."]
        #[inline(always)]
        pub fn set_force_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "force pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "force pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub fn set_force_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "force pixel_if_hline_err."]
        #[inline(always)]
        pub const fn force_pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force pixel_if_hline_err."]
        #[inline(always)]
        pub fn set_force_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force int_event_fifo_overflow."]
        #[inline(always)]
        pub const fn force_int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force int_event_fifo_overflow."]
        #[inline(always)]
        pub fn set_force_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntForceIpiFatal {
        #[inline(always)]
        fn default() -> IntForceIpiFatal {
            IntForceIpiFatal(0)
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePhy(pub u32);
    impl IntForcePhy {
        #[doc = "force phy_errsoths_0."]
        #[inline(always)]
        pub const fn force_phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsoths_0."]
        #[inline(always)]
        pub fn set_force_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force phy_errsoths_1."]
        #[inline(always)]
        pub const fn force_phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsoths_1."]
        #[inline(always)]
        pub fn set_force_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force phy_erresc_0."]
        #[inline(always)]
        pub const fn force_phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_erresc_0."]
        #[inline(always)]
        pub fn set_force_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "force phy_erresc_1."]
        #[inline(always)]
        pub const fn force_phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_erresc_1."]
        #[inline(always)]
        pub fn set_force_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntForcePhy {
        #[inline(always)]
        fn default() -> IntForcePhy {
            IntForcePhy(0)
        }
    }
    #[doc = "interrupt force register for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePhyFatal(pub u32);
    impl IntForcePhyFatal {
        #[doc = "force phy_errsotsynchs_0."]
        #[inline(always)]
        pub const fn force_phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsotsynchs_0."]
        #[inline(always)]
        pub fn set_force_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force phy_errsotsynchs_1."]
        #[inline(always)]
        pub const fn force_phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsotsynchs_1."]
        #[inline(always)]
        pub fn set_force_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force err_deskew."]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "force err_deskew."]
        #[inline(always)]
        pub fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntForcePhyFatal {
        #[inline(always)]
        fn default() -> IntForcePhyFatal {
            IntForcePhyFatal(0)
        }
    }
    #[doc = "interrupt force register is used for test purpos."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePktFatal(pub u32);
    impl IntForcePktFatal {
        #[doc = "force err_ecc_double."]
        #[inline(always)]
        pub const fn force_err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force err_ecc_double."]
        #[inline(always)]
        pub fn set_force_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntForcePktFatal {
        #[inline(always)]
        fn default() -> IntForcePktFatal {
            IntForcePktFatal(0)
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePldCrcFrameFatal(pub u32);
    impl IntForcePldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn force_err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_force_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForcePldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntForcePldCrcFrameFatal {
            IntForcePldCrcFrameFatal(0)
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceSeqFrameFatal(pub u32);
    impl IntForceSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_force_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntForceSeqFrameFatal {
            IntForceSeqFrameFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_ap_generic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskApGeneric(pub u32);
    impl IntMskApGeneric {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn msk_synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_msk_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntMskApGeneric {
        #[inline(always)]
        fn default() -> IntMskApGeneric {
            IntMskApGeneric(0)
        }
    }
    #[doc = "interrupt mask for int_st_ap_ipi_fatal controls."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskApIpiFatal(pub u32);
    impl IntMskApIpiFatal {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mask_redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mask_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntMskApIpiFatal {
        #[inline(always)]
        fn default() -> IntMskApIpiFatal {
            IntMskApIpiFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_bndry_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskBndryFrameFatal(pub u32);
    impl IntMskBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_msk_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntMskBndryFrameFatal {
            IntMskBndryFrameFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskCrcFrameFatal(pub u32);
    impl IntMskCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_msk_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntMskCrcFrameFatal {
            IntMskCrcFrameFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_ipi_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskIpiFatal(pub u32);
    impl IntMskIpiFatal {
        #[doc = "mask for pixel_if_fifo_unterflow."]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_fifo_unterflow."]
        #[inline(always)]
        pub fn set_msk_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub fn set_msk_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for pixel_if_frame_sync_err."]
        #[inline(always)]
        pub const fn msk_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_frame_sync_err."]
        #[inline(always)]
        pub fn set_msk_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "mask pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "mask pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub fn set_msk_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "mask pixel_if_hline_err."]
        #[inline(always)]
        pub const fn msk_pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "mask pixel_if_hline_err."]
        #[inline(always)]
        pub fn set_msk_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "mask int_event_fifo_overflow."]
        #[inline(always)]
        pub const fn msk_int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "mask int_event_fifo_overflow."]
        #[inline(always)]
        pub fn set_msk_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntMskIpiFatal {
        #[inline(always)]
        fn default() -> IntMskIpiFatal {
            IntMskIpiFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPhy(pub u32);
    impl IntMskPhy {
        #[doc = "mask for phy_errsoths_0."]
        #[inline(always)]
        pub const fn mask_phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsoths_0."]
        #[inline(always)]
        pub fn set_mask_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for phy_errsoths_1."]
        #[inline(always)]
        pub const fn mask_phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsoths_1."]
        #[inline(always)]
        pub fn set_mask_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for phy_erresc_0."]
        #[inline(always)]
        pub const fn mask_phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_erresc_0."]
        #[inline(always)]
        pub fn set_mask_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "mask for phy_erresc_1."]
        #[inline(always)]
        pub const fn mask_phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_erresc_1."]
        #[inline(always)]
        pub fn set_mask_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntMskPhy {
        #[inline(always)]
        fn default() -> IntMskPhy {
            IntMskPhy(0)
        }
    }
    #[doc = "interrupt mask for int_st_phy_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPhyFatal(pub u32);
    impl IntMskPhyFatal {
        #[doc = "mask for phy_errsotsynchs_0."]
        #[inline(always)]
        pub const fn mask_phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsotsynchs_0."]
        #[inline(always)]
        pub fn set_mask_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for phy_errsotsynchs_1."]
        #[inline(always)]
        pub const fn mask_phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsotsynchs_1."]
        #[inline(always)]
        pub fn set_mask_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for err_deskew."]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "mask for err_deskew."]
        #[inline(always)]
        pub fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntMskPhyFatal {
        #[inline(always)]
        fn default() -> IntMskPhyFatal {
            IntMskPhyFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_pkt_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPktFatal(pub u32);
    impl IntMskPktFatal {
        #[doc = "mask for err_ecc_double."]
        #[inline(always)]
        pub const fn mask_err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for err_ecc_double."]
        #[inline(always)]
        pub fn set_mask_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntMskPktFatal {
        #[inline(always)]
        fn default() -> IntMskPktFatal {
            IntMskPktFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPldCrcFrameFatal(pub u32);
    impl IntMskPldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_msk_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskPldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntMskPldCrcFrameFatal {
            IntMskPldCrcFrameFatal(0)
        }
    }
    #[doc = "interrupt mask for int_st_seq_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskSeqFrameFatal(pub u32);
    impl IntMskSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_msk_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntMskSeqFrameFatal {
            IntMskSeqFrameFatal(0)
        }
    }
    #[doc = "groups and notifies which interruption bits caused the interruption."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApGeneric(pub u32);
    impl IntStApGeneric {
        #[doc = "ap error in apb block."]
        #[inline(always)]
        pub const fn apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in apb block."]
        #[inline(always)]
        pub fn set_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ap error in register bank block."]
        #[inline(always)]
        pub const fn reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in register bank block."]
        #[inline(always)]
        pub fn set_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "ap error in de-skew block."]
        #[inline(always)]
        pub const fn de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in de-skew block."]
        #[inline(always)]
        pub fn set_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ap error in pipeline delay block."]
        #[inline(always)]
        pub const fn pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in pipeline delay block."]
        #[inline(always)]
        pub fn set_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ap error in descrambler block."]
        #[inline(always)]
        pub const fn descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in descrambler block."]
        #[inline(always)]
        pub fn set_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ap error in phy adapter block."]
        #[inline(always)]
        pub const fn phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in phy adapter block."]
        #[inline(always)]
        pub fn set_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ap error in packet analyzer block."]
        #[inline(always)]
        pub const fn packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in packet analyzer block."]
        #[inline(always)]
        pub fn set_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "ap error in prepare outs block."]
        #[inline(always)]
        pub const fn prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in prepare outs block."]
        #[inline(always)]
        pub fn set_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "ap error in err msgr block."]
        #[inline(always)]
        pub const fn err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in err msgr block."]
        #[inline(always)]
        pub fn set_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ap error in error handler block."]
        #[inline(always)]
        pub const fn err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in error handler block."]
        #[inline(always)]
        pub fn set_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ap error in synchronizer block for fpclk domain."]
        #[inline(always)]
        pub const fn synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for fpclk domain."]
        #[inline(always)]
        pub fn set_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ap error in synchronizer block for rxbyteclkhs domain."]
        #[inline(always)]
        pub const fn synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for rxbyteclkhs domain."]
        #[inline(always)]
        pub fn set_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ap error in synchronizer block for pixclk domain."]
        #[inline(always)]
        pub const fn synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for pixclk domain."]
        #[inline(always)]
        pub fn set_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntStApGeneric {
        #[inline(always)]
        fn default() -> IntStApGeneric {
            IntStApGeneric(0)
        }
    }
    #[doc = "groups and notifies which interruption bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApIpiFatal(pub u32);
    impl IntStApIpiFatal {
        #[doc = "ap parity tx error in ipi1."]
        #[inline(always)]
        pub const fn parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ap parity tx error in ipi1."]
        #[inline(always)]
        pub fn set_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ap parity rx error in ipi1."]
        #[inline(always)]
        pub const fn parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ap parity rx error in ipi1."]
        #[inline(always)]
        pub fn set_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ap ecc sigle error in ipi1."]
        #[inline(always)]
        pub const fn ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ap ecc sigle error in ipi1."]
        #[inline(always)]
        pub fn set_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ap ecc multiple error in ipi1."]
        #[inline(always)]
        pub const fn ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ap ecc multiple error in ipi1."]
        #[inline(always)]
        pub fn set_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ap crc error in ipi1."]
        #[inline(always)]
        pub const fn crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ap crc error in ipi1."]
        #[inline(always)]
        pub fn set_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ap redundancy error in ipi1."]
        #[inline(always)]
        pub const fn redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ap redundancy error in ipi1."]
        #[inline(always)]
        pub fn set_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntStApIpiFatal {
        #[inline(always)]
        fn default() -> IntStApIpiFatal {
            IntStApIpiFatal(0)
        }
    }
    #[doc = "contains the status of individual interrupt sources."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApMain(pub u32);
    impl IntStApMain {
        #[doc = "status of int_st_ap_generic."]
        #[inline(always)]
        pub const fn status_int_st_ap_generic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ap_generic."]
        #[inline(always)]
        pub fn set_status_int_st_ap_generic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub const fn status_int_phy_fatal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub fn set_status_int_phy_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub const fn status_int_pkt_fatal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub fn set_status_int_pkt_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_bndry_frame_fatal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_bndry_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_seq_frame_fatal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_seq_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_crc_frame_fatal(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_crc_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub const fn status_int_phy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub fn set_status_int_phy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub const fn status_int_pld_crc_fatal(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub fn set_status_int_pld_crc_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub const fn status_int_data_id(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub fn set_status_int_data_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub const fn status_int_ecc_corrected(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub fn set_status_int_ecc_corrected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub const fn status_int_line(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub fn set_status_int_line(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "status of int_st_ap_ipi_fatal."]
        #[inline(always)]
        pub const fn status_int_st_ap_ipi_fatal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ap_ipi_fatal."]
        #[inline(always)]
        pub fn set_status_int_st_ap_ipi_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub const fn status_int_ipi_fatal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub fn set_status_int_ipi_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for IntStApMain {
        #[inline(always)]
        fn default() -> IntStApMain {
            IntStApMain(0)
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStBndryFrameFatal(pub u32);
    impl IntStBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntStBndryFrameFatal {
            IntStBndryFrameFatal(0)
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStCrcFrameFatal(pub u32);
    impl IntStCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntStCrcFrameFatal {
            IntStCrcFrameFatal(0)
        }
    }
    #[doc = "fatal interruption caused by ipi interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStIpiFatal(pub u32);
    impl IntStIpiFatal {
        #[doc = "the fifo has become empty before the expected bumber of pixels could be extracted to the pixel intefcese."]
        #[inline(always)]
        pub const fn pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo has become empty before the expected bumber of pixels could be extracted to the pixel intefcese."]
        #[inline(always)]
        pub fn set_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the fifo of pixel interface has lost information because some data arrived and fifo is already full."]
        #[inline(always)]
        pub const fn pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo of pixel interface has lost information because some data arrived and fifo is already full."]
        #[inline(always)]
        pub fn set_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "whenever in controller mode, notifies if a new frame is received but previous has not been completed."]
        #[inline(always)]
        pub const fn pixel_if_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "whenever in controller mode, notifies if a new frame is received but previous has not been completed."]
        #[inline(always)]
        pub fn set_pixel_if_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the fifo of pixel interface is not empty at the starat of a new frame."]
        #[inline(always)]
        pub const fn pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo of pixel interface is not empty at the starat of a new frame."]
        #[inline(always)]
        pub fn set_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "horizontal line time error."]
        #[inline(always)]
        pub const fn pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "horizontal line time error."]
        #[inline(always)]
        pub fn set_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "reporting internal fifo overflow."]
        #[inline(always)]
        pub const fn int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "reporting internal fifo overflow."]
        #[inline(always)]
        pub fn set_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntStIpiFatal {
        #[inline(always)]
        fn default() -> IntStIpiFatal {
            IntStIpiFatal(0)
        }
    }
    #[doc = "contains the stateus of individual interrupt sources."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStMain(pub u32);
    impl IntStMain {
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub const fn status_int_phy_fatal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub fn set_status_int_phy_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub const fn status_int_pkt_fatal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub fn set_status_int_pkt_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_bndry_frame_fatal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_bndry_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_seq_frame_fatal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_seq_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub const fn status_int_crc_frame_fatal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub fn set_status_int_crc_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub const fn status_int_pld_crc_fatal(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub fn set_status_int_pld_crc_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub const fn status_int_data_id(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub fn set_status_int_data_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub const fn status_int_ecc_corrected(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub fn set_status_int_ecc_corrected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub const fn status_int_phy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub fn set_status_int_phy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub const fn status_int_line(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub fn set_status_int_line(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub const fn status_int_ipi4_fatal(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub fn set_status_int_ipi4_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntStMain {
        #[inline(always)]
        fn default() -> IntStMain {
            IntStMain(0)
        }
    }
    #[doc = "interruption caused by phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPhy(pub u32);
    impl IntStPhy {
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub const fn phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub fn set_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub const fn phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub fn set_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub const fn phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub fn set_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub const fn phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub fn set_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntStPhy {
        #[inline(always)]
        fn default() -> IntStPhy {
            IntStPhy(0)
        }
    }
    #[doc = "groups the phy interruptions caused by phy packets discarded."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPhyFatal(pub u32);
    impl IntStPhyFatal {
        #[doc = "start of transmission error on data lane0."]
        #[inline(always)]
        pub const fn phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane0."]
        #[inline(always)]
        pub fn set_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start of transmission error on data lane1."]
        #[inline(always)]
        pub const fn phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane1."]
        #[inline(always)]
        pub fn set_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "reports whenever data is lost due to an existent skew between lanes greater than 2 rxwordclkhs."]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "reports whenever data is lost due to an existent skew between lanes greater than 2 rxwordclkhs."]
        #[inline(always)]
        pub fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntStPhyFatal {
        #[inline(always)]
        fn default() -> IntStPhyFatal {
            IntStPhyFatal(0)
        }
    }
    #[doc = "groups the fatal interruption related with packet construction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPktFatal(pub u32);
    impl IntStPktFatal {
        #[doc = "header ecc contains at least 2 errors."]
        #[inline(always)]
        pub const fn err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "header ecc contains at least 2 errors."]
        #[inline(always)]
        pub fn set_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntStPktFatal {
        #[inline(always)]
        fn default() -> IntStPktFatal {
            IntStPktFatal(0)
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPldCrcFrameFatal(pub u32);
    impl IntStPldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStPldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntStPldCrcFrameFatal {
            IntStPldCrcFrameFatal(0)
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStSeqFrameFatal(pub u32);
    impl IntStSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub fn set_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntStSeqFrameFatal {
            IntStSeqFrameFatal(0)
        }
    }
    #[doc = "configures advanced features for ipi mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiAdvFeatures(pub u32);
    impl IpiAdvFeatures {
        #[doc = "ignore datatype of the header using the programmed datatype for decoding."]
        #[inline(always)]
        pub const fn ipi_dt_overwrite(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ignore datatype of the header using the programmed datatype for decoding."]
        #[inline(always)]
        pub fn set_ipi_dt_overwrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "datatype to overwrite."]
        #[inline(always)]
        pub const fn ipi_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "datatype to overwrite."]
        #[inline(always)]
        pub fn set_ipi_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "for camero mode, allows manual selection of the packet fo line delimiter as follows: 0x0-controller seletc it automaticlly 0x1-select packets from list programmed in 17:21."]
        #[inline(always)]
        pub const fn line_event_selection(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "for camero mode, allows manual selection of the packet fo line delimiter as follows: 0x0-controller seletc it automaticlly 0x1-select packets from list programmed in 17:21."]
        #[inline(always)]
        pub fn set_line_event_selection(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "allows the use of video packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn en_video(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of video packets for ipi synchronization events."]
        #[inline(always)]
        pub fn set_en_video(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "allows the use of line start packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn en_line_start(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of line start packets for ipi synchronization events."]
        #[inline(always)]
        pub fn set_en_line_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "allows the use of null packets for IPI synchronization events."]
        #[inline(always)]
        pub const fn en_null(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of null packets for IPI synchronization events."]
        #[inline(always)]
        pub fn set_en_null(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "allows the use of blankong packets for IPI synchronization events."]
        #[inline(always)]
        pub const fn en_blanking(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of blankong packets for IPI synchronization events."]
        #[inline(always)]
        pub fn set_en_blanking(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "allows the use of embendded packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn en_embedded(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of embendded packets for ipi synchronization events."]
        #[inline(always)]
        pub fn set_en_embedded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "for camera mode: 0x0- frame start do not trigger any sync event."]
        #[inline(always)]
        pub const fn ipi_sync_event_mode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "for camera mode: 0x0- frame start do not trigger any sync event."]
        #[inline(always)]
        pub fn set_ipi_sync_event_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IpiAdvFeatures {
        #[inline(always)]
        fn default() -> IpiAdvFeatures {
            IpiAdvFeatures(0)
        }
    }
    #[doc = "selects the data type processed by ipi."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiDataType(pub u32);
    impl IpiDataType {
        #[doc = "data type of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn ipi_data_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type of data to be processed by pixel interface."]
        #[inline(always)]
        pub fn set_ipi_data_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "enable embedded data processing on ipi interface."]
        #[inline(always)]
        pub const fn embended_data(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable embedded data processing on ipi interface."]
        #[inline(always)]
        pub fn set_embended_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IpiDataType {
        #[inline(always)]
        fn default() -> IpiDataType {
            IpiDataType(0)
        }
    }
    #[doc = "configures the video horizontal synchronism back porch time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHbpTime(pub u32);
    impl IpiHbpTime {
        #[doc = "configures the Horizontal Synchronism back porch period in pixclk cycles."]
        #[inline(always)]
        pub const fn ipi_hbp_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Synchronism back porch period in pixclk cycles."]
        #[inline(always)]
        pub fn set_ipi_hbp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHbpTime {
        #[inline(always)]
        fn default() -> IpiHbpTime {
            IpiHbpTime(0)
        }
    }
    #[doc = "configures the overall tiem for each video line."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHlineTime(pub u32);
    impl IpiHlineTime {
        #[doc = "configures the size of the line time counted in pixclk cycles."]
        #[inline(always)]
        pub const fn ipi_hlin_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configures the size of the line time counted in pixclk cycles."]
        #[inline(always)]
        pub fn set_ipi_hlin_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for IpiHlineTime {
        #[inline(always)]
        fn default() -> IpiHlineTime {
            IpiHlineTime(0)
        }
    }
    #[doc = "configures the video horizontal synchronism active time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHsaTime(pub u32);
    impl IpiHsaTime {
        #[doc = "configures the Horizontal Synchronism Active period in pixclk cycles."]
        #[inline(always)]
        pub const fn ipi_hsa_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Synchronism Active period in pixclk cycles."]
        #[inline(always)]
        pub fn set_ipi_hsa_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHsaTime {
        #[inline(always)]
        fn default() -> IpiHsaTime {
            IpiHsaTime(0)
        }
    }
    #[doc = "configures the vedeo Horizontal Sync Delay time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHsdTime(pub u32);
    impl IpiHsdTime {
        #[doc = "configures the Horizontal Sync Porch delay period in pixclk cycles."]
        #[inline(always)]
        pub const fn ipi_hsd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Sync Porch delay period in pixclk cycles."]
        #[inline(always)]
        pub fn set_ipi_hsd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHsdTime {
        #[inline(always)]
        fn default() -> IpiHsdTime {
            IpiHsdTime(0)
        }
    }
    #[doc = "control the flush of ipi memory."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiMemFlash(pub u32);
    impl IpiMemFlash {
        #[doc = "flush ipi memory, this bit is auto clear."]
        #[inline(always)]
        pub const fn ipi_flush(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "flush ipi memory, this bit is auto clear."]
        #[inline(always)]
        pub fn set_ipi_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "memory is automatically flashed at each vsync."]
        #[inline(always)]
        pub const fn ipi_auto_flush(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "memory is automatically flashed at each vsync."]
        #[inline(always)]
        pub fn set_ipi_auto_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IpiMemFlash {
        #[inline(always)]
        fn default() -> IpiMemFlash {
            IpiMemFlash(0)
        }
    }
    #[doc = "selects how the ipi interface generates the video frame."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiMode(pub u32);
    impl IpiMode {
        #[doc = "indicates the video mode transmission type 0x0: camera timing 0x1:controller timing."]
        #[inline(always)]
        pub const fn ipi_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the video mode transmission type 0x0: camera timing 0x1:controller timing."]
        #[inline(always)]
        pub fn set_ipi_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "if color mode components are deliverd as follows: 0x0 48bit intercase 0x1: 16bit interface."]
        #[inline(always)]
        pub const fn ipi_color_com(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "if color mode components are deliverd as follows: 0x0 48bit intercase 0x1: 16bit interface."]
        #[inline(always)]
        pub fn set_ipi_color_com(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "cut-through mode state active when high."]
        #[inline(always)]
        pub const fn ipi_cut_through(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "cut-through mode state active when high."]
        #[inline(always)]
        pub fn set_ipi_cut_through(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "enables the interface."]
        #[inline(always)]
        pub const fn ipi_enable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enables the interface."]
        #[inline(always)]
        pub fn set_ipi_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IpiMode {
        #[inline(always)]
        fn default() -> IpiMode {
            IpiMode(0)
        }
    }
    #[doc = "congtrols the ipi logic reset state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiSoftrstn(pub u32);
    impl IpiSoftrstn {
        #[doc = "resets ipi one, active low."]
        #[inline(always)]
        pub const fn ipi_softrstn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "resets ipi one, active low."]
        #[inline(always)]
        pub fn set_ipi_softrstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IpiSoftrstn {
        #[inline(always)]
        fn default() -> IpiSoftrstn {
            IpiSoftrstn(0)
        }
    }
    #[doc = "configures the vertical resolution of video."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVactiveLines(pub u32);
    impl IpiVactiveLines {
        #[doc = "configures the vertical active period measured in bumber of horizontal lines."]
        #[inline(always)]
        pub const fn ipi_vactive_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "configures the vertical active period measured in bumber of horizontal lines."]
        #[inline(always)]
        pub fn set_ipi_vactive_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for IpiVactiveLines {
        #[inline(always)]
        fn default() -> IpiVactiveLines {
            IpiVactiveLines(0)
        }
    }
    #[doc = "configures the verticall back porch period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVbpLines(pub u32);
    impl IpiVbpLines {
        #[doc = "configuress the vertical back porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn ipi_vbp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configuress the vertical back porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_ipi_vbp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVbpLines {
        #[inline(always)]
        fn default() -> IpiVbpLines {
            IpiVbpLines(0)
        }
    }
    #[doc = "selects the vritual channel processed by ipi."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVcid(pub u32);
    impl IpiVcid {
        #[doc = "virtual channel of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn ip_vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "virtual channel of data to be processed by pixel interface."]
        #[inline(always)]
        pub fn set_ip_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "virtual channel extension of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn ipi_vcx_0_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "virtual channel extension of data to be processed by pixel interface."]
        #[inline(always)]
        pub fn set_ipi_vcx_0_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for IpiVcid {
        #[inline(always)]
        fn default() -> IpiVcid {
            IpiVcid(0)
        }
    }
    #[doc = "configures the vertical front porch period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVfpLines(pub u32);
    impl IpiVfpLines {
        #[doc = "configures the vertical front porch period measured in number of horizontall lines."]
        #[inline(always)]
        pub const fn ipi_vfp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical front porch period measured in number of horizontall lines."]
        #[inline(always)]
        pub fn set_ipi_vfp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVfpLines {
        #[inline(always)]
        fn default() -> IpiVfpLines {
            IpiVfpLines(0)
        }
    }
    #[doc = "configures the vertical synchronism active period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVsaLines(pub u32);
    impl IpiVsaLines {
        #[doc = "configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn ipi_vsa_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_ipi_vsa_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVsaLines {
        #[inline(always)]
        fn default() -> IpiVsaLines {
            IpiVsaLines(0)
        }
    }
    #[doc = "the number of active lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NLanes(pub u32);
    impl NLanes {
        #[doc = "number of active data lanes."]
        #[inline(always)]
        pub const fn n_lanes(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "number of active data lanes."]
        #[inline(always)]
        pub fn set_n_lanes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for NLanes {
        #[inline(always)]
        fn default() -> NLanes {
            NLanes(0)
        }
    }
    #[doc = "contains the calibration signal status from synopsys d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCal(pub u32);
    impl PhyCal {
        #[doc = "a low-to-high transition on rxskewcalhs signal means the the phy has initiated the de-skew calibration."]
        #[inline(always)]
        pub const fn rxskewcalhs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "a low-to-high transition on rxskewcalhs signal means the the phy has initiated the de-skew calibration."]
        #[inline(always)]
        pub fn set_rxskewcalhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyCal {
        #[inline(always)]
        fn default() -> PhyCal {
            PhyCal(0)
        }
    }
    #[doc = "contains the status of rx-related signals from phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyRx(pub u32);
    impl PhyRx {
        #[doc = "lane module 0 has entered the ultra low power mode."]
        #[inline(always)]
        pub const fn phy_rxulpsesc_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "lane module 0 has entered the ultra low power mode."]
        #[inline(always)]
        pub fn set_phy_rxulpsesc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "lane module 1 has entered the ultra low power mode."]
        #[inline(always)]
        pub const fn phy_rxullpsesc_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "lane module 1 has entered the ultra low power mode."]
        #[inline(always)]
        pub fn set_phy_rxullpsesc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "active low. Indicates the d-phy clock lane module has entered the Ultra low power state."]
        #[inline(always)]
        pub const fn phy_rxulpsclknot(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "active low. Indicates the d-phy clock lane module has entered the Ultra low power state."]
        #[inline(always)]
        pub fn set_phy_rxulpsclknot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "indicates the d-phy clock lane is actively receiving a ddr clock."]
        #[inline(always)]
        pub const fn phy_rxclkactivehs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the d-phy clock lane is actively receiving a ddr clock."]
        #[inline(always)]
        pub fn set_phy_rxclkactivehs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for PhyRx {
        #[inline(always)]
        fn default() -> PhyRx {
            PhyRx(0)
        }
    }
    #[doc = "controls the phy shutdown mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyShutdownz(pub u32);
    impl PhyShutdownz {
        #[doc = "shutdown input,active low."]
        #[inline(always)]
        pub const fn phy_shutdownz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "shutdown input,active low."]
        #[inline(always)]
        pub fn set_phy_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyShutdownz {
        #[inline(always)]
        fn default() -> PhyShutdownz {
            PhyShutdownz(0)
        }
    }
    #[doc = "contains the stopstate signal status from phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStopstate(pub u32);
    impl PhyStopstate {
        #[doc = "data lane 0 in stop state."]
        #[inline(always)]
        pub const fn phy_stopstatedata_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "data lane 0 in stop state."]
        #[inline(always)]
        pub fn set_phy_stopstatedata_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "data lane 1 in stop state."]
        #[inline(always)]
        pub const fn phy_stopstatedata_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "data lane 1 in stop state."]
        #[inline(always)]
        pub fn set_phy_stopstatedata_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "d-phy clock lane in stop state."]
        #[inline(always)]
        pub const fn phy_stopstateclk(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "d-phy clock lane in stop state."]
        #[inline(always)]
        pub fn set_phy_stopstateclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for PhyStopstate {
        #[inline(always)]
        fn default() -> PhyStopstate {
            PhyStopstate(0)
        }
    }
    #[doc = "active extra bits for virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VcExtension(pub u32);
    impl VcExtension {
        #[doc = "indicates status of virtual channel extension: 0-virtual channel extension is enable 1-legacy mode."]
        #[inline(always)]
        pub const fn vcx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates status of virtual channel extension: 0-virtual channel extension is enable 1-legacy mode."]
        #[inline(always)]
        pub fn set_vcx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for VcExtension {
        #[inline(always)]
        fn default() -> VcExtension {
            VcExtension(0)
        }
    }
    #[doc = "version code."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version(pub u32);
    impl Version {
        #[doc = "version code."]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "version code."]
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
}
