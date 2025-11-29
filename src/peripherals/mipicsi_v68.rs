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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "the number of active lanes."]
    #[inline(always)]
    pub const fn n_lanes(self) -> crate::common::Reg<regs::NLanes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "the internal logic of the controller goes into the reset state when active."]
    #[inline(always)]
    pub const fn csi2_resetn(self) -> crate::common::Reg<regs::Csi2Resetn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "contains the stateus of individual interrupt sources."]
    #[inline(always)]
    pub const fn int_st_main(self) -> crate::common::Reg<regs::IntStMain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[inline(always)]
    pub const fn data_ids_1(self) -> crate::common::Reg<regs::DataIds1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[inline(always)]
    pub const fn data_ids_2(self) -> crate::common::Reg<regs::DataIds2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "contains the status of individual interrupt sources."]
    #[inline(always)]
    pub const fn int_st_ap_main(self) -> crate::common::Reg<regs::IntStApMain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "controls the phy shutdown mode."]
    #[inline(always)]
    pub const fn phy_shutdownz(self) -> crate::common::Reg<regs::PhyShutdownz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "controls the phy reset mode."]
    #[inline(always)]
    pub const fn dphy_rstz(self) -> crate::common::Reg<regs::DphyRstz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "contains the status of rx-related signals from phy."]
    #[inline(always)]
    pub const fn phy_rx(self) -> crate::common::Reg<regs::PhyRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "contains the stopstate signal status from phy."]
    #[inline(always)]
    pub const fn phy_stopstate(self) -> crate::common::Reg<regs::PhyStopstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "selects how the ipi interface generates the video frame."]
    #[inline(always)]
    pub const fn ipi_mode(self) -> crate::common::Reg<regs::IpiMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "selects the vritual channel processed by ipi."]
    #[inline(always)]
    pub const fn ipi_vcid(self) -> crate::common::Reg<regs::IpiVcid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "selects the data type processed by ipi."]
    #[inline(always)]
    pub const fn ipi_data_type(self) -> crate::common::Reg<regs::IpiDataType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "control the flush of ipi memory."]
    #[inline(always)]
    pub const fn ipi_mem_flash(self) -> crate::common::Reg<regs::IpiMemFlash, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "configures the video horizontal synchronism active time."]
    #[inline(always)]
    pub const fn ipi_hsa_time(self) -> crate::common::Reg<regs::IpiHsaTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "configures the video horizontal synchronism back porch time."]
    #[inline(always)]
    pub const fn ipi_hbp_time(self) -> crate::common::Reg<regs::IpiHbpTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "configures the vedeo Horizontal Sync Delay time."]
    #[inline(always)]
    pub const fn ipi_hsd_time(self) -> crate::common::Reg<regs::IpiHsdTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "configures the overall tiem for each video line."]
    #[inline(always)]
    pub const fn ipi_hline_time(self) -> crate::common::Reg<regs::IpiHlineTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "congtrols the ipi logic reset state."]
    #[inline(always)]
    pub const fn ipi_softrstn(self) -> crate::common::Reg<regs::IpiSoftrstn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "configures advanced features for ipi mode."]
    #[inline(always)]
    pub const fn ipi_adv_features(
        self,
    ) -> crate::common::Reg<regs::IpiAdvFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "configures the vertical synchronism active period."]
    #[inline(always)]
    pub const fn ipi_vsa_lines(self) -> crate::common::Reg<regs::IpiVsaLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "configures the verticall back porch period."]
    #[inline(always)]
    pub const fn ipi_vbp_lines(self) -> crate::common::Reg<regs::IpiVbpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "configures the vertical front porch period."]
    #[inline(always)]
    pub const fn ipi_vfp_lines(self) -> crate::common::Reg<regs::IpiVfpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "configures the vertical resolution of video."]
    #[inline(always)]
    pub const fn ipi_vactive_lines(
        self,
    ) -> crate::common::Reg<regs::IpiVactiveLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "active extra bits for virtual channel."]
    #[inline(always)]
    pub const fn vc_extension(self) -> crate::common::Reg<regs::VcExtension, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "contains the calibration signal status from synopsys d-phy."]
    #[inline(always)]
    pub const fn phy_cal(self) -> crate::common::Reg<regs::PhyCal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "groups the phy interruptions caused by phy packets discarded."]
    #[inline(always)]
    pub const fn int_st_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_phy_fatal."]
    #[inline(always)]
    pub const fn int_msk_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "interrupt force register for test purposes."]
    #[inline(always)]
    pub const fn int_force_phy_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePhyFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "groups the fatal interruption related with packet construction."]
    #[inline(always)]
    pub const fn int_st_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_pkt_fatal."]
    #[inline(always)]
    pub const fn int_msk_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purpos."]
    #[inline(always)]
    pub const fn int_force_pkt_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePktFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "interruption caused by phy."]
    #[inline(always)]
    pub const fn int_st_phy(self) -> crate::common::Reg<regs::IntStPhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "interrupt mask for int_st_phy."]
    #[inline(always)]
    pub const fn int_msk_phy(self) -> crate::common::Reg<regs::IntMskPhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_phy(self) -> crate::common::Reg<regs::IntForcePhy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "fatal interruption caused by ipi interface."]
    #[inline(always)]
    pub const fn int_st_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ipi_fatal."]
    #[inline(always)]
    pub const fn int_msk_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "groups and notifies which interruption bits caused the interruption."]
    #[inline(always)]
    pub const fn int_st_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntStApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ap_generic."]
    #[inline(always)]
    pub const fn int_msk_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntMskApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "interrupt force register used for test purposes."]
    #[inline(always)]
    pub const fn int_force_ap_generic(
        self,
    ) -> crate::common::Reg<regs::IntForceApGeneric, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "groups and notifies which interruption bits."]
    #[inline(always)]
    pub const fn int_st_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "interrupt mask for int_st_ap_ipi_fatal controls."]
    #[inline(always)]
    pub const fn int_msk_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "interrupt force register."]
    #[inline(always)]
    pub const fn int_force_ap_ipi_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceApIpiFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "interrupt mask for int_st_bndry_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_bndry_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceBndryFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "interrupt mask for int_st_seq_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_seq_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceSeqFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForceCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[inline(always)]
    pub const fn int_st_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntStPldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[inline(always)]
    pub const fn int_msk_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntMskPldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[inline(always)]
    pub const fn int_force_pld_crc_frame_fatal(
        self,
    ) -> crate::common::Reg<regs::IntForcePldCrcFrameFatal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
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
    #[doc = "the internal logic of the controller goes into the reset state when active."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csi2Resetn(pub u32);
    impl Csi2Resetn {
        #[doc = "DWC_mipi_csi2_host reset output, active low."]
        #[must_use]
        #[inline(always)]
        pub const fn csi2_resetn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DWC_mipi_csi2_host reset output, active low."]
        #[inline(always)]
        pub const fn set_csi2_resetn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Csi2Resetn {
        #[inline(always)]
        fn default() -> Csi2Resetn {
            Csi2Resetn(0)
        }
    }
    impl core::fmt::Debug for Csi2Resetn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csi2Resetn")
                .field("csi2_resetn", &self.csi2_resetn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csi2Resetn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csi2Resetn {{ csi2_resetn: {=bool:?} }}",
                self.csi2_resetn()
            )
        }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataIds1(pub u32);
    impl DataIds1 {
        #[doc = "data type for programmed data ID 0."]
        #[must_use]
        #[inline(always)]
        pub const fn di0_dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 0."]
        #[inline(always)]
        pub const fn set_di0_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "data type for programmed data ID 1."]
        #[must_use]
        #[inline(always)]
        pub const fn di1_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 1."]
        #[inline(always)]
        pub const fn set_di1_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "data type for programmed data ID 2."]
        #[must_use]
        #[inline(always)]
        pub const fn di2_dt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 2."]
        #[inline(always)]
        pub const fn set_di2_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "data type for programmed data ID 3."]
        #[must_use]
        #[inline(always)]
        pub const fn di3_dt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 3."]
        #[inline(always)]
        pub const fn set_di3_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for DataIds1 {
        #[inline(always)]
        fn default() -> DataIds1 {
            DataIds1(0)
        }
    }
    impl core::fmt::Debug for DataIds1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DataIds1")
                .field("di0_dt", &self.di0_dt())
                .field("di1_dt", &self.di1_dt())
                .field("di2_dt", &self.di2_dt())
                .field("di3_dt", &self.di3_dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DataIds1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DataIds1 {{ di0_dt: {=u8:?}, di1_dt: {=u8:?}, di2_dt: {=u8:?}, di3_dt: {=u8:?} }}",
                self.di0_dt(),
                self.di1_dt(),
                self.di2_dt(),
                self.di3_dt()
            )
        }
    }
    #[doc = "programs data type fields for data ID monitors."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataIds2(pub u32);
    impl DataIds2 {
        #[doc = "data type for programmed data ID 4."]
        #[must_use]
        #[inline(always)]
        pub const fn di4_dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 4."]
        #[inline(always)]
        pub const fn set_di4_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "data type for programmed data ID 5."]
        #[must_use]
        #[inline(always)]
        pub const fn di5_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 5."]
        #[inline(always)]
        pub const fn set_di5_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "data type for programmed data ID 6."]
        #[must_use]
        #[inline(always)]
        pub const fn di6_dt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 6."]
        #[inline(always)]
        pub const fn set_di6_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "data type for programmed data ID 7."]
        #[must_use]
        #[inline(always)]
        pub const fn di7_dt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "data type for programmed data ID 7."]
        #[inline(always)]
        pub const fn set_di7_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for DataIds2 {
        #[inline(always)]
        fn default() -> DataIds2 {
            DataIds2(0)
        }
    }
    impl core::fmt::Debug for DataIds2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DataIds2")
                .field("di4_dt", &self.di4_dt())
                .field("di5_dt", &self.di5_dt())
                .field("di6_dt", &self.di6_dt())
                .field("di7_dt", &self.di7_dt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DataIds2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DataIds2 {{ di4_dt: {=u8:?}, di5_dt: {=u8:?}, di6_dt: {=u8:?}, di7_dt: {=u8:?} }}",
                self.di4_dt(),
                self.di5_dt(),
                self.di6_dt(),
                self.di7_dt()
            )
        }
    }
    #[doc = "controls the phy reset mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DphyRstz(pub u32);
    impl DphyRstz {
        #[doc = "phy reset output, active low."]
        #[must_use]
        #[inline(always)]
        pub const fn dphy_rstz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "phy reset output, active low."]
        #[inline(always)]
        pub const fn set_dphy_rstz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DphyRstz {
        #[inline(always)]
        fn default() -> DphyRstz {
            DphyRstz(0)
        }
    }
    impl core::fmt::Debug for DphyRstz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DphyRstz")
                .field("dphy_rstz", &self.dphy_rstz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DphyRstz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DphyRstz {{ dphy_rstz: {=bool:?} }}", self.dphy_rstz())
        }
    }
    #[doc = "interrupt force register used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceApGeneric(pub u32);
    impl IntForceApGeneric {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntForceApGeneric {
        #[inline(always)]
        fn default() -> IntForceApGeneric {
            IntForceApGeneric(0)
        }
    }
    impl core::fmt::Debug for IntForceApGeneric {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceApGeneric")
                .field("force_apb_ap_err", &self.force_apb_ap_err())
                .field("force_reg_bank_ap_err", &self.force_reg_bank_ap_err())
                .field("force_de_skew_ap_err", &self.force_de_skew_ap_err())
                .field(
                    "force_pipeline_delay_ap_err",
                    &self.force_pipeline_delay_ap_err(),
                )
                .field("force_descrambler_ap_err", &self.force_descrambler_ap_err())
                .field("force_phy_adapter_ap_err", &self.force_phy_adapter_ap_err())
                .field(
                    "force_packet_analyzer_ap_err",
                    &self.force_packet_analyzer_ap_err(),
                )
                .field("force_prep_outs_ap_err", &self.force_prep_outs_ap_err())
                .field("force_err_msgr_ap_err", &self.force_err_msgr_ap_err())
                .field("force_err_handle_ap_err", &self.force_err_handle_ap_err())
                .field(
                    "force_synchronizer_fpclk_ap_err",
                    &self.force_synchronizer_fpclk_ap_err(),
                )
                .field(
                    "force_synchronizer_rxbyteclkhs_ap_err",
                    &self.force_synchronizer_rxbyteclkhs_ap_err(),
                )
                .field(
                    "force_synchronizer_pixclk_ap_err",
                    &self.force_synchronizer_pixclk_ap_err(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceApGeneric {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceApGeneric {{ force_apb_ap_err: {=u8:?}, force_reg_bank_ap_err: {=u8:?}, force_de_skew_ap_err: {=bool:?}, force_pipeline_delay_ap_err: {=bool:?}, force_descrambler_ap_err: {=bool:?}, force_phy_adapter_ap_err: {=bool:?}, force_packet_analyzer_ap_err: {=u8:?}, force_prep_outs_ap_err: {=u8:?}, force_err_msgr_ap_err: {=bool:?}, force_err_handle_ap_err: {=bool:?}, force_synchronizer_fpclk_ap_err: {=bool:?}, force_synchronizer_rxbyteclkhs_ap_err: {=bool:?}, force_synchronizer_pixclk_ap_err: {=bool:?} }}" , self . force_apb_ap_err () , self . force_reg_bank_ap_err () , self . force_de_skew_ap_err () , self . force_pipeline_delay_ap_err () , self . force_descrambler_ap_err () , self . force_phy_adapter_ap_err () , self . force_packet_analyzer_ap_err () , self . force_prep_outs_ap_err () , self . force_err_msgr_ap_err () , self . force_err_handle_ap_err () , self . force_synchronizer_fpclk_ap_err () , self . force_synchronizer_rxbyteclkhs_ap_err () , self . force_synchronizer_pixclk_ap_err ())
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceApIpiFatal(pub u32);
    impl IntForceApIpiFatal {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn force_redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_force_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntForceApIpiFatal {
        #[inline(always)]
        fn default() -> IntForceApIpiFatal {
            IntForceApIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntForceApIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceApIpiFatal")
                .field("force_parity_tx_err", &self.force_parity_tx_err())
                .field("force_parity_rx_err", &self.force_parity_rx_err())
                .field("force_ecc_single_err", &self.force_ecc_single_err())
                .field("force_ecc_multiple_err", &self.force_ecc_multiple_err())
                .field("force_crc_err", &self.force_crc_err())
                .field("force_redundancy_err", &self.force_redundancy_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceApIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceApIpiFatal {{ force_parity_tx_err: {=bool:?}, force_parity_rx_err: {=bool:?}, force_ecc_single_err: {=bool:?}, force_ecc_multiple_err: {=bool:?}, force_crc_err: {=bool:?}, force_redundancy_err: {=bool:?} }}" , self . force_parity_tx_err () , self . force_parity_rx_err () , self . force_ecc_single_err () , self . force_ecc_multiple_err () , self . force_crc_err () , self . force_redundancy_err ())
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceBndryFrameFatal(pub u32);
    impl IntForceBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_force_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntForceBndryFrameFatal {
            IntForceBndryFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntForceBndryFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceBndryFrameFatal")
                .field(
                    "force_err_f_bndry_match_vc0",
                    &self.force_err_f_bndry_match_vc0(),
                )
                .field(
                    "force_err_f_bndry_match_vc1",
                    &self.force_err_f_bndry_match_vc1(),
                )
                .field(
                    "force_err_f_bndry_match_vc2",
                    &self.force_err_f_bndry_match_vc2(),
                )
                .field(
                    "force_err_f_bndry_match_vc3",
                    &self.force_err_f_bndry_match_vc3(),
                )
                .field(
                    "force_err_f_bndry_match_vc4",
                    &self.force_err_f_bndry_match_vc4(),
                )
                .field(
                    "force_err_f_bndry_match_vc5",
                    &self.force_err_f_bndry_match_vc5(),
                )
                .field(
                    "force_err_f_bndry_match_vc6",
                    &self.force_err_f_bndry_match_vc6(),
                )
                .field(
                    "force_err_f_bndry_match_vc7",
                    &self.force_err_f_bndry_match_vc7(),
                )
                .field(
                    "force_err_f_bndry_match_vc8",
                    &self.force_err_f_bndry_match_vc8(),
                )
                .field(
                    "force_err_f_bndry_match_vc9",
                    &self.force_err_f_bndry_match_vc9(),
                )
                .field(
                    "force_err_f_bndry_match_vc10",
                    &self.force_err_f_bndry_match_vc10(),
                )
                .field(
                    "force_err_f_bndry_match_vc11",
                    &self.force_err_f_bndry_match_vc11(),
                )
                .field(
                    "force_err_f_bndry_match_vc12",
                    &self.force_err_f_bndry_match_vc12(),
                )
                .field(
                    "force_err_f_bndry_match_vc13",
                    &self.force_err_f_bndry_match_vc13(),
                )
                .field(
                    "force_err_f_bndry_match_vc14",
                    &self.force_err_f_bndry_match_vc14(),
                )
                .field(
                    "force_err_f_bndry_match_vc15",
                    &self.force_err_f_bndry_match_vc15(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceBndryFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceBndryFrameFatal {{ force_err_f_bndry_match_vc0: {=bool:?}, force_err_f_bndry_match_vc1: {=bool:?}, force_err_f_bndry_match_vc2: {=bool:?}, force_err_f_bndry_match_vc3: {=bool:?}, force_err_f_bndry_match_vc4: {=bool:?}, force_err_f_bndry_match_vc5: {=bool:?}, force_err_f_bndry_match_vc6: {=bool:?}, force_err_f_bndry_match_vc7: {=bool:?}, force_err_f_bndry_match_vc8: {=bool:?}, force_err_f_bndry_match_vc9: {=bool:?}, force_err_f_bndry_match_vc10: {=bool:?}, force_err_f_bndry_match_vc11: {=bool:?}, force_err_f_bndry_match_vc12: {=bool:?}, force_err_f_bndry_match_vc13: {=bool:?}, force_err_f_bndry_match_vc14: {=bool:?}, force_err_f_bndry_match_vc15: {=bool:?} }}" , self . force_err_f_bndry_match_vc0 () , self . force_err_f_bndry_match_vc1 () , self . force_err_f_bndry_match_vc2 () , self . force_err_f_bndry_match_vc3 () , self . force_err_f_bndry_match_vc4 () , self . force_err_f_bndry_match_vc5 () , self . force_err_f_bndry_match_vc6 () , self . force_err_f_bndry_match_vc7 () , self . force_err_f_bndry_match_vc8 () , self . force_err_f_bndry_match_vc9 () , self . force_err_f_bndry_match_vc10 () , self . force_err_f_bndry_match_vc11 () , self . force_err_f_bndry_match_vc12 () , self . force_err_f_bndry_match_vc13 () , self . force_err_f_bndry_match_vc14 () , self . force_err_f_bndry_match_vc15 ())
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceCrcFrameFatal(pub u32);
    impl IntForceCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_force_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntForceCrcFrameFatal {
            IntForceCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntForceCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceCrcFrameFatal")
                .field(
                    "force_err_f_crc_match_vc0",
                    &self.force_err_f_crc_match_vc0(),
                )
                .field(
                    "force_err_f_crc_match_vc1",
                    &self.force_err_f_crc_match_vc1(),
                )
                .field(
                    "force_err_f_crc_match_vc2",
                    &self.force_err_f_crc_match_vc2(),
                )
                .field(
                    "force_err_f_crc_match_vc3",
                    &self.force_err_f_crc_match_vc3(),
                )
                .field(
                    "force_err_f_crc_match_vc4",
                    &self.force_err_f_crc_match_vc4(),
                )
                .field(
                    "force_err_f_crc_match_vc5",
                    &self.force_err_f_crc_match_vc5(),
                )
                .field(
                    "force_err_f_crc_match_vc6",
                    &self.force_err_f_crc_match_vc6(),
                )
                .field(
                    "force_err_f_crc_match_vc7",
                    &self.force_err_f_crc_match_vc7(),
                )
                .field(
                    "force_err_f_crc_match_vc8",
                    &self.force_err_f_crc_match_vc8(),
                )
                .field(
                    "force_err_f_crc_match_vc9",
                    &self.force_err_f_crc_match_vc9(),
                )
                .field(
                    "force_err_f_crc_match_vc10",
                    &self.force_err_f_crc_match_vc10(),
                )
                .field(
                    "force_err_f_crc_match_vc11",
                    &self.force_err_f_crc_match_vc11(),
                )
                .field(
                    "force_err_f_crc_match_vc12",
                    &self.force_err_f_crc_match_vc12(),
                )
                .field(
                    "force_err_f_crc_match_vc13",
                    &self.force_err_f_crc_match_vc13(),
                )
                .field(
                    "force_err_f_crc_match_vc14",
                    &self.force_err_f_crc_match_vc14(),
                )
                .field(
                    "force_err_f_crc_match_vc15",
                    &self.force_err_f_crc_match_vc15(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceCrcFrameFatal {{ force_err_f_crc_match_vc0: {=bool:?}, force_err_f_crc_match_vc1: {=bool:?}, force_err_f_crc_match_vc2: {=bool:?}, force_err_f_crc_match_vc3: {=bool:?}, force_err_f_crc_match_vc4: {=bool:?}, force_err_f_crc_match_vc5: {=bool:?}, force_err_f_crc_match_vc6: {=bool:?}, force_err_f_crc_match_vc7: {=bool:?}, force_err_f_crc_match_vc8: {=bool:?}, force_err_f_crc_match_vc9: {=bool:?}, force_err_f_crc_match_vc10: {=bool:?}, force_err_f_crc_match_vc11: {=bool:?}, force_err_f_crc_match_vc12: {=bool:?}, force_err_f_crc_match_vc13: {=bool:?}, force_err_f_crc_match_vc14: {=bool:?}, force_err_f_crc_match_vc15: {=bool:?} }}" , self . force_err_f_crc_match_vc0 () , self . force_err_f_crc_match_vc1 () , self . force_err_f_crc_match_vc2 () , self . force_err_f_crc_match_vc3 () , self . force_err_f_crc_match_vc4 () , self . force_err_f_crc_match_vc5 () , self . force_err_f_crc_match_vc6 () , self . force_err_f_crc_match_vc7 () , self . force_err_f_crc_match_vc8 () , self . force_err_f_crc_match_vc9 () , self . force_err_f_crc_match_vc10 () , self . force_err_f_crc_match_vc11 () , self . force_err_f_crc_match_vc12 () , self . force_err_f_crc_match_vc13 () , self . force_err_f_crc_match_vc14 () , self . force_err_f_crc_match_vc15 ())
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceIpiFatal(pub u32);
    impl IntForceIpiFatal {
        #[doc = "force for pixel_if_fifo_underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force for pixel_if_fifo_underflow."]
        #[inline(always)]
        pub const fn set_force_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force for pixel_if_fifo_overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub const fn set_force_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force for frame_sync_err."]
        #[must_use]
        #[inline(always)]
        pub const fn force_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force for frame_sync_err."]
        #[inline(always)]
        pub const fn set_force_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "force pixel_if_fifo_nempty_fs."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "force pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub const fn set_force_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "force pixel_if_hline_err."]
        #[must_use]
        #[inline(always)]
        pub const fn force_pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force pixel_if_hline_err."]
        #[inline(always)]
        pub const fn set_force_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force int_event_fifo_overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn force_int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force int_event_fifo_overflow."]
        #[inline(always)]
        pub const fn set_force_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntForceIpiFatal {
        #[inline(always)]
        fn default() -> IntForceIpiFatal {
            IntForceIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntForceIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceIpiFatal")
                .field(
                    "force_pixel_if_fifo_underflow",
                    &self.force_pixel_if_fifo_underflow(),
                )
                .field(
                    "force_pixel_if_fifo_overflow",
                    &self.force_pixel_if_fifo_overflow(),
                )
                .field("force_frame_sync_err", &self.force_frame_sync_err())
                .field(
                    "force_pixel_if_fifo_nempty_fs",
                    &self.force_pixel_if_fifo_nempty_fs(),
                )
                .field("force_pixel_if_hline_err", &self.force_pixel_if_hline_err())
                .field(
                    "force_int_event_fifo_overflow",
                    &self.force_int_event_fifo_overflow(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceIpiFatal {{ force_pixel_if_fifo_underflow: {=bool:?}, force_pixel_if_fifo_overflow: {=bool:?}, force_frame_sync_err: {=bool:?}, force_pixel_if_fifo_nempty_fs: {=bool:?}, force_pixel_if_hline_err: {=bool:?}, force_int_event_fifo_overflow: {=bool:?} }}" , self . force_pixel_if_fifo_underflow () , self . force_pixel_if_fifo_overflow () , self . force_frame_sync_err () , self . force_pixel_if_fifo_nempty_fs () , self . force_pixel_if_hline_err () , self . force_int_event_fifo_overflow ())
        }
    }
    #[doc = "interrupt force register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePhy(pub u32);
    impl IntForcePhy {
        #[doc = "force phy_errsoths_0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsoths_0."]
        #[inline(always)]
        pub const fn set_force_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force phy_errsoths_1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsoths_1."]
        #[inline(always)]
        pub const fn set_force_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force phy_erresc_0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_erresc_0."]
        #[inline(always)]
        pub const fn set_force_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "force phy_erresc_1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_erresc_1."]
        #[inline(always)]
        pub const fn set_force_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntForcePhy {
        #[inline(always)]
        fn default() -> IntForcePhy {
            IntForcePhy(0)
        }
    }
    impl core::fmt::Debug for IntForcePhy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForcePhy")
                .field("force_phy_errsoths_0", &self.force_phy_errsoths_0())
                .field("force_phy_errsoths_1", &self.force_phy_errsoths_1())
                .field("force_phy_erresc_0", &self.force_phy_erresc_0())
                .field("force_phy_erresc_1", &self.force_phy_erresc_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForcePhy {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForcePhy {{ force_phy_errsoths_0: {=bool:?}, force_phy_errsoths_1: {=bool:?}, force_phy_erresc_0: {=bool:?}, force_phy_erresc_1: {=bool:?} }}" , self . force_phy_errsoths_0 () , self . force_phy_errsoths_1 () , self . force_phy_erresc_0 () , self . force_phy_erresc_1 ())
        }
    }
    #[doc = "interrupt force register for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePhyFatal(pub u32);
    impl IntForcePhyFatal {
        #[doc = "force phy_errsotsynchs_0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsotsynchs_0."]
        #[inline(always)]
        pub const fn set_force_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force phy_errsotsynchs_1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force phy_errsotsynchs_1."]
        #[inline(always)]
        pub const fn set_force_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force err_deskew."]
        #[must_use]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "force err_deskew."]
        #[inline(always)]
        pub const fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntForcePhyFatal {
        #[inline(always)]
        fn default() -> IntForcePhyFatal {
            IntForcePhyFatal(0)
        }
    }
    impl core::fmt::Debug for IntForcePhyFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForcePhyFatal")
                .field("force_phy_errsotsynchs_0", &self.force_phy_errsotsynchs_0())
                .field("force_phy_errsotsynchs_1", &self.force_phy_errsotsynchs_1())
                .field("err_deskew", &self.err_deskew())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForcePhyFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForcePhyFatal {{ force_phy_errsotsynchs_0: {=bool:?}, force_phy_errsotsynchs_1: {=bool:?}, err_deskew: {=bool:?} }}" , self . force_phy_errsotsynchs_0 () , self . force_phy_errsotsynchs_1 () , self . err_deskew ())
        }
    }
    #[doc = "interrupt force register is used for test purpos."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePktFatal(pub u32);
    impl IntForcePktFatal {
        #[doc = "force err_ecc_double."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force err_ecc_double."]
        #[inline(always)]
        pub const fn set_force_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntForcePktFatal {
        #[inline(always)]
        fn default() -> IntForcePktFatal {
            IntForcePktFatal(0)
        }
    }
    impl core::fmt::Debug for IntForcePktFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForcePktFatal")
                .field("force_err_ecc_double", &self.force_err_ecc_double())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForcePktFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntForcePktFatal {{ force_err_ecc_double: {=bool:?} }}",
                self.force_err_ecc_double()
            )
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForcePldCrcFrameFatal(pub u32);
    impl IntForcePldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_force_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForcePldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntForcePldCrcFrameFatal {
            IntForcePldCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntForcePldCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForcePldCrcFrameFatal")
                .field("force_err_crc_match_vc0", &self.force_err_crc_match_vc0())
                .field("force_err_crc_match_vc1", &self.force_err_crc_match_vc1())
                .field("force_err_crc_match_vc2", &self.force_err_crc_match_vc2())
                .field("force_err_crc_match_vc3", &self.force_err_crc_match_vc3())
                .field("force_err_crc_match_vc4", &self.force_err_crc_match_vc4())
                .field("force_err_crc_match_vc5", &self.force_err_crc_match_vc5())
                .field("force_err_crc_match_vc6", &self.force_err_crc_match_vc6())
                .field("force_err_crc_match_vc7", &self.force_err_crc_match_vc7())
                .field("force_err_crc_match_vc8", &self.force_err_crc_match_vc8())
                .field("force_err_crc_match_vc9", &self.force_err_crc_match_vc9())
                .field("force_err_crc_match_vc10", &self.force_err_crc_match_vc10())
                .field("force_err_crc_match_vc11", &self.force_err_crc_match_vc11())
                .field("force_err_crc_match_vc12", &self.force_err_crc_match_vc12())
                .field("force_err_crc_match_vc13", &self.force_err_crc_match_vc13())
                .field("force_err_crc_match_vc14", &self.force_err_crc_match_vc14())
                .field("force_err_crc_match_vc15", &self.force_err_crc_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForcePldCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForcePldCrcFrameFatal {{ force_err_crc_match_vc0: {=bool:?}, force_err_crc_match_vc1: {=bool:?}, force_err_crc_match_vc2: {=bool:?}, force_err_crc_match_vc3: {=bool:?}, force_err_crc_match_vc4: {=bool:?}, force_err_crc_match_vc5: {=bool:?}, force_err_crc_match_vc6: {=bool:?}, force_err_crc_match_vc7: {=bool:?}, force_err_crc_match_vc8: {=bool:?}, force_err_crc_match_vc9: {=bool:?}, force_err_crc_match_vc10: {=bool:?}, force_err_crc_match_vc11: {=bool:?}, force_err_crc_match_vc12: {=bool:?}, force_err_crc_match_vc13: {=bool:?}, force_err_crc_match_vc14: {=bool:?}, force_err_crc_match_vc15: {=bool:?} }}" , self . force_err_crc_match_vc0 () , self . force_err_crc_match_vc1 () , self . force_err_crc_match_vc2 () , self . force_err_crc_match_vc3 () , self . force_err_crc_match_vc4 () , self . force_err_crc_match_vc5 () , self . force_err_crc_match_vc6 () , self . force_err_crc_match_vc7 () , self . force_err_crc_match_vc8 () , self . force_err_crc_match_vc9 () , self . force_err_crc_match_vc10 () , self . force_err_crc_match_vc11 () , self . force_err_crc_match_vc12 () , self . force_err_crc_match_vc13 () , self . force_err_crc_match_vc14 () , self . force_err_crc_match_vc15 ())
        }
    }
    #[doc = "interrupt force register is used for test purposes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForceSeqFrameFatal(pub u32);
    impl IntForceSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn force_err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_force_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntForceSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntForceSeqFrameFatal {
            IntForceSeqFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntForceSeqFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntForceSeqFrameFatal")
                .field(
                    "force_err_f_seq_match_vc0",
                    &self.force_err_f_seq_match_vc0(),
                )
                .field(
                    "force_err_f_seq_match_vc1",
                    &self.force_err_f_seq_match_vc1(),
                )
                .field(
                    "force_err_f_seq_match_vc2",
                    &self.force_err_f_seq_match_vc2(),
                )
                .field(
                    "force_err_f_seq_match_vc3",
                    &self.force_err_f_seq_match_vc3(),
                )
                .field(
                    "force_err_f_seq_match_vc4",
                    &self.force_err_f_seq_match_vc4(),
                )
                .field(
                    "force_err_f_seq_match_vc5",
                    &self.force_err_f_seq_match_vc5(),
                )
                .field(
                    "force_err_f_seq_match_vc6",
                    &self.force_err_f_seq_match_vc6(),
                )
                .field(
                    "force_err_f_seq_match_vc7",
                    &self.force_err_f_seq_match_vc7(),
                )
                .field(
                    "force_err_f_seq_match_vc8",
                    &self.force_err_f_seq_match_vc8(),
                )
                .field(
                    "force_err_f_seq_match_vc9",
                    &self.force_err_f_seq_match_vc9(),
                )
                .field(
                    "force_err_f_seq_match_vc10",
                    &self.force_err_f_seq_match_vc10(),
                )
                .field(
                    "force_err_f_seq_match_vc11",
                    &self.force_err_f_seq_match_vc11(),
                )
                .field(
                    "force_err_f_seq_match_vc12",
                    &self.force_err_f_seq_match_vc12(),
                )
                .field(
                    "force_err_f_seq_match_vc13",
                    &self.force_err_f_seq_match_vc13(),
                )
                .field(
                    "force_err_f_seq_match_vc14",
                    &self.force_err_f_seq_match_vc14(),
                )
                .field(
                    "force_err_f_seq_match_vc15",
                    &self.force_err_f_seq_match_vc15(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntForceSeqFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntForceSeqFrameFatal {{ force_err_f_seq_match_vc0: {=bool:?}, force_err_f_seq_match_vc1: {=bool:?}, force_err_f_seq_match_vc2: {=bool:?}, force_err_f_seq_match_vc3: {=bool:?}, force_err_f_seq_match_vc4: {=bool:?}, force_err_f_seq_match_vc5: {=bool:?}, force_err_f_seq_match_vc6: {=bool:?}, force_err_f_seq_match_vc7: {=bool:?}, force_err_f_seq_match_vc8: {=bool:?}, force_err_f_seq_match_vc9: {=bool:?}, force_err_f_seq_match_vc10: {=bool:?}, force_err_f_seq_match_vc11: {=bool:?}, force_err_f_seq_match_vc12: {=bool:?}, force_err_f_seq_match_vc13: {=bool:?}, force_err_f_seq_match_vc14: {=bool:?}, force_err_f_seq_match_vc15: {=bool:?} }}" , self . force_err_f_seq_match_vc0 () , self . force_err_f_seq_match_vc1 () , self . force_err_f_seq_match_vc2 () , self . force_err_f_seq_match_vc3 () , self . force_err_f_seq_match_vc4 () , self . force_err_f_seq_match_vc5 () , self . force_err_f_seq_match_vc6 () , self . force_err_f_seq_match_vc7 () , self . force_err_f_seq_match_vc8 () , self . force_err_f_seq_match_vc9 () , self . force_err_f_seq_match_vc10 () , self . force_err_f_seq_match_vc11 () , self . force_err_f_seq_match_vc12 () , self . force_err_f_seq_match_vc13 () , self . force_err_f_seq_match_vc14 () , self . force_err_f_seq_match_vc15 ())
        }
    }
    #[doc = "interrupt mask for int_st_ap_generic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskApGeneric(pub u32);
    impl IntMskApGeneric {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_msk_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntMskApGeneric {
        #[inline(always)]
        fn default() -> IntMskApGeneric {
            IntMskApGeneric(0)
        }
    }
    impl core::fmt::Debug for IntMskApGeneric {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskApGeneric")
                .field("msk_apb_ap_err", &self.msk_apb_ap_err())
                .field("msk_reg_bank_ap_err", &self.msk_reg_bank_ap_err())
                .field("msk_de_skew_ap_err", &self.msk_de_skew_ap_err())
                .field(
                    "msk_pipeline_delay_ap_err",
                    &self.msk_pipeline_delay_ap_err(),
                )
                .field("msk_descrambler_ap_err", &self.msk_descrambler_ap_err())
                .field("msk_phy_adapter_ap_err", &self.msk_phy_adapter_ap_err())
                .field(
                    "msk_packet_analyzer_ap_err",
                    &self.msk_packet_analyzer_ap_err(),
                )
                .field("msk_prep_outs_ap_err", &self.msk_prep_outs_ap_err())
                .field("msk_err_msgr_ap_err", &self.msk_err_msgr_ap_err())
                .field("msk_err_handle_ap_err", &self.msk_err_handle_ap_err())
                .field(
                    "msk_synchronizer_fpclk_ap_err",
                    &self.msk_synchronizer_fpclk_ap_err(),
                )
                .field(
                    "msk_synchronizer_rxbyteclkhs_ap_err",
                    &self.msk_synchronizer_rxbyteclkhs_ap_err(),
                )
                .field(
                    "msk_synchronizer_pixclk_ap_err",
                    &self.msk_synchronizer_pixclk_ap_err(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskApGeneric {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskApGeneric {{ msk_apb_ap_err: {=u8:?}, msk_reg_bank_ap_err: {=u8:?}, msk_de_skew_ap_err: {=bool:?}, msk_pipeline_delay_ap_err: {=bool:?}, msk_descrambler_ap_err: {=bool:?}, msk_phy_adapter_ap_err: {=bool:?}, msk_packet_analyzer_ap_err: {=u8:?}, msk_prep_outs_ap_err: {=u8:?}, msk_err_msgr_ap_err: {=bool:?}, msk_err_handle_ap_err: {=bool:?}, msk_synchronizer_fpclk_ap_err: {=bool:?}, msk_synchronizer_rxbyteclkhs_ap_err: {=bool:?}, msk_synchronizer_pixclk_ap_err: {=bool:?} }}" , self . msk_apb_ap_err () , self . msk_reg_bank_ap_err () , self . msk_de_skew_ap_err () , self . msk_pipeline_delay_ap_err () , self . msk_descrambler_ap_err () , self . msk_phy_adapter_ap_err () , self . msk_packet_analyzer_ap_err () , self . msk_prep_outs_ap_err () , self . msk_err_msgr_ap_err () , self . msk_err_handle_ap_err () , self . msk_synchronizer_fpclk_ap_err () , self . msk_synchronizer_rxbyteclkhs_ap_err () , self . msk_synchronizer_pixclk_ap_err ())
        }
    }
    #[doc = "interrupt mask for int_st_ap_ipi_fatal controls."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskApIpiFatal(pub u32);
    impl IntMskApIpiFatal {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mask_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntMskApIpiFatal {
        #[inline(always)]
        fn default() -> IntMskApIpiFatal {
            IntMskApIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskApIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskApIpiFatal")
                .field("mask_parity_tx_err", &self.mask_parity_tx_err())
                .field("mask_parity_rx_err", &self.mask_parity_rx_err())
                .field("mask_ecc_single_err", &self.mask_ecc_single_err())
                .field("mask_ecc_multiple_err", &self.mask_ecc_multiple_err())
                .field("mask_crc_err", &self.mask_crc_err())
                .field("mask_redundancy_err", &self.mask_redundancy_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskApIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskApIpiFatal {{ mask_parity_tx_err: {=bool:?}, mask_parity_rx_err: {=bool:?}, mask_ecc_single_err: {=bool:?}, mask_ecc_multiple_err: {=bool:?}, mask_crc_err: {=bool:?}, mask_redundancy_err: {=bool:?} }}" , self . mask_parity_tx_err () , self . mask_parity_rx_err () , self . mask_ecc_single_err () , self . mask_ecc_multiple_err () , self . mask_crc_err () , self . mask_redundancy_err ())
        }
    }
    #[doc = "interrupt mask for int_st_bndry_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskBndryFrameFatal(pub u32);
    impl IntMskBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_msk_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntMskBndryFrameFatal {
            IntMskBndryFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskBndryFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskBndryFrameFatal")
                .field(
                    "msk_err_f_bndry_match_vc0",
                    &self.msk_err_f_bndry_match_vc0(),
                )
                .field(
                    "msk_err_f_bndry_match_vc1",
                    &self.msk_err_f_bndry_match_vc1(),
                )
                .field(
                    "msk_err_f_bndry_match_vc2",
                    &self.msk_err_f_bndry_match_vc2(),
                )
                .field(
                    "msk_err_f_bndry_match_vc3",
                    &self.msk_err_f_bndry_match_vc3(),
                )
                .field(
                    "msk_err_f_bndry_match_vc4",
                    &self.msk_err_f_bndry_match_vc4(),
                )
                .field(
                    "msk_err_f_bndry_match_vc5",
                    &self.msk_err_f_bndry_match_vc5(),
                )
                .field(
                    "msk_err_f_bndry_match_vc6",
                    &self.msk_err_f_bndry_match_vc6(),
                )
                .field(
                    "msk_err_f_bndry_match_vc7",
                    &self.msk_err_f_bndry_match_vc7(),
                )
                .field(
                    "msk_err_f_bndry_match_vc8",
                    &self.msk_err_f_bndry_match_vc8(),
                )
                .field(
                    "msk_err_f_bndry_match_vc9",
                    &self.msk_err_f_bndry_match_vc9(),
                )
                .field(
                    "msk_err_f_bndry_match_vc10",
                    &self.msk_err_f_bndry_match_vc10(),
                )
                .field(
                    "msk_err_f_bndry_match_vc11",
                    &self.msk_err_f_bndry_match_vc11(),
                )
                .field(
                    "msk_err_f_bndry_match_vc12",
                    &self.msk_err_f_bndry_match_vc12(),
                )
                .field(
                    "msk_err_f_bndry_match_vc13",
                    &self.msk_err_f_bndry_match_vc13(),
                )
                .field(
                    "msk_err_f_bndry_match_vc14",
                    &self.msk_err_f_bndry_match_vc14(),
                )
                .field(
                    "msk_err_f_bndry_match_vc15",
                    &self.msk_err_f_bndry_match_vc15(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskBndryFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskBndryFrameFatal {{ msk_err_f_bndry_match_vc0: {=bool:?}, msk_err_f_bndry_match_vc1: {=bool:?}, msk_err_f_bndry_match_vc2: {=bool:?}, msk_err_f_bndry_match_vc3: {=bool:?}, msk_err_f_bndry_match_vc4: {=bool:?}, msk_err_f_bndry_match_vc5: {=bool:?}, msk_err_f_bndry_match_vc6: {=bool:?}, msk_err_f_bndry_match_vc7: {=bool:?}, msk_err_f_bndry_match_vc8: {=bool:?}, msk_err_f_bndry_match_vc9: {=bool:?}, msk_err_f_bndry_match_vc10: {=bool:?}, msk_err_f_bndry_match_vc11: {=bool:?}, msk_err_f_bndry_match_vc12: {=bool:?}, msk_err_f_bndry_match_vc13: {=bool:?}, msk_err_f_bndry_match_vc14: {=bool:?}, msk_err_f_bndry_match_vc15: {=bool:?} }}" , self . msk_err_f_bndry_match_vc0 () , self . msk_err_f_bndry_match_vc1 () , self . msk_err_f_bndry_match_vc2 () , self . msk_err_f_bndry_match_vc3 () , self . msk_err_f_bndry_match_vc4 () , self . msk_err_f_bndry_match_vc5 () , self . msk_err_f_bndry_match_vc6 () , self . msk_err_f_bndry_match_vc7 () , self . msk_err_f_bndry_match_vc8 () , self . msk_err_f_bndry_match_vc9 () , self . msk_err_f_bndry_match_vc10 () , self . msk_err_f_bndry_match_vc11 () , self . msk_err_f_bndry_match_vc12 () , self . msk_err_f_bndry_match_vc13 () , self . msk_err_f_bndry_match_vc14 () , self . msk_err_f_bndry_match_vc15 ())
        }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskCrcFrameFatal(pub u32);
    impl IntMskCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_msk_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntMskCrcFrameFatal {
            IntMskCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskCrcFrameFatal")
                .field("msk_err_f_crc_match_vc0", &self.msk_err_f_crc_match_vc0())
                .field("msk_err_f_crc_match_vc1", &self.msk_err_f_crc_match_vc1())
                .field("msk_err_f_crc_match_vc2", &self.msk_err_f_crc_match_vc2())
                .field("msk_err_f_crc_match_vc3", &self.msk_err_f_crc_match_vc3())
                .field("msk_err_f_crc_match_vc4", &self.msk_err_f_crc_match_vc4())
                .field("msk_err_f_crc_match_vc5", &self.msk_err_f_crc_match_vc5())
                .field("msk_err_f_crc_match_vc6", &self.msk_err_f_crc_match_vc6())
                .field("msk_err_f_crc_match_vc7", &self.msk_err_f_crc_match_vc7())
                .field("msk_err_f_crc_match_vc8", &self.msk_err_f_crc_match_vc8())
                .field("msk_err_f_crc_match_vc9", &self.msk_err_f_crc_match_vc9())
                .field("msk_err_f_crc_match_vc10", &self.msk_err_f_crc_match_vc10())
                .field("msk_err_f_crc_match_vc11", &self.msk_err_f_crc_match_vc11())
                .field("msk_err_f_crc_match_vc12", &self.msk_err_f_crc_match_vc12())
                .field("msk_err_f_crc_match_vc13", &self.msk_err_f_crc_match_vc13())
                .field("msk_err_f_crc_match_vc14", &self.msk_err_f_crc_match_vc14())
                .field("msk_err_f_crc_match_vc15", &self.msk_err_f_crc_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskCrcFrameFatal {{ msk_err_f_crc_match_vc0: {=bool:?}, msk_err_f_crc_match_vc1: {=bool:?}, msk_err_f_crc_match_vc2: {=bool:?}, msk_err_f_crc_match_vc3: {=bool:?}, msk_err_f_crc_match_vc4: {=bool:?}, msk_err_f_crc_match_vc5: {=bool:?}, msk_err_f_crc_match_vc6: {=bool:?}, msk_err_f_crc_match_vc7: {=bool:?}, msk_err_f_crc_match_vc8: {=bool:?}, msk_err_f_crc_match_vc9: {=bool:?}, msk_err_f_crc_match_vc10: {=bool:?}, msk_err_f_crc_match_vc11: {=bool:?}, msk_err_f_crc_match_vc12: {=bool:?}, msk_err_f_crc_match_vc13: {=bool:?}, msk_err_f_crc_match_vc14: {=bool:?}, msk_err_f_crc_match_vc15: {=bool:?} }}" , self . msk_err_f_crc_match_vc0 () , self . msk_err_f_crc_match_vc1 () , self . msk_err_f_crc_match_vc2 () , self . msk_err_f_crc_match_vc3 () , self . msk_err_f_crc_match_vc4 () , self . msk_err_f_crc_match_vc5 () , self . msk_err_f_crc_match_vc6 () , self . msk_err_f_crc_match_vc7 () , self . msk_err_f_crc_match_vc8 () , self . msk_err_f_crc_match_vc9 () , self . msk_err_f_crc_match_vc10 () , self . msk_err_f_crc_match_vc11 () , self . msk_err_f_crc_match_vc12 () , self . msk_err_f_crc_match_vc13 () , self . msk_err_f_crc_match_vc14 () , self . msk_err_f_crc_match_vc15 ())
        }
    }
    #[doc = "interrupt mask for int_st_ipi_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskIpiFatal(pub u32);
    impl IntMskIpiFatal {
        #[doc = "mask for pixel_if_fifo_unterflow."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_fifo_unterflow."]
        #[inline(always)]
        pub const fn set_msk_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for pixel_if_fifo_overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_fifo_overflow."]
        #[inline(always)]
        pub const fn set_msk_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for pixel_if_frame_sync_err."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "mask for pixel_if_frame_sync_err."]
        #[inline(always)]
        pub const fn set_msk_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "mask pixel_if_fifo_nempty_fs."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "mask pixel_if_fifo_nempty_fs."]
        #[inline(always)]
        pub const fn set_msk_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "mask pixel_if_hline_err."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "mask pixel_if_hline_err."]
        #[inline(always)]
        pub const fn set_msk_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "mask int_event_fifo_overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "mask int_event_fifo_overflow."]
        #[inline(always)]
        pub const fn set_msk_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntMskIpiFatal {
        #[inline(always)]
        fn default() -> IntMskIpiFatal {
            IntMskIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskIpiFatal")
                .field(
                    "msk_pixel_if_fifo_underflow",
                    &self.msk_pixel_if_fifo_underflow(),
                )
                .field(
                    "msk_pixel_if_fifo_overflow",
                    &self.msk_pixel_if_fifo_overflow(),
                )
                .field("msk_frame_sync_err", &self.msk_frame_sync_err())
                .field(
                    "msk_pixel_if_fifo_nempty_fs",
                    &self.msk_pixel_if_fifo_nempty_fs(),
                )
                .field("msk_pixel_if_hline_err", &self.msk_pixel_if_hline_err())
                .field(
                    "msk_int_event_fifo_overflow",
                    &self.msk_int_event_fifo_overflow(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskIpiFatal {{ msk_pixel_if_fifo_underflow: {=bool:?}, msk_pixel_if_fifo_overflow: {=bool:?}, msk_frame_sync_err: {=bool:?}, msk_pixel_if_fifo_nempty_fs: {=bool:?}, msk_pixel_if_hline_err: {=bool:?}, msk_int_event_fifo_overflow: {=bool:?} }}" , self . msk_pixel_if_fifo_underflow () , self . msk_pixel_if_fifo_overflow () , self . msk_frame_sync_err () , self . msk_pixel_if_fifo_nempty_fs () , self . msk_pixel_if_hline_err () , self . msk_int_event_fifo_overflow ())
        }
    }
    #[doc = "interrupt mask for int_st_phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPhy(pub u32);
    impl IntMskPhy {
        #[doc = "mask for phy_errsoths_0."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsoths_0."]
        #[inline(always)]
        pub const fn set_mask_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for phy_errsoths_1."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsoths_1."]
        #[inline(always)]
        pub const fn set_mask_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for phy_erresc_0."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_erresc_0."]
        #[inline(always)]
        pub const fn set_mask_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "mask for phy_erresc_1."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_erresc_1."]
        #[inline(always)]
        pub const fn set_mask_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntMskPhy {
        #[inline(always)]
        fn default() -> IntMskPhy {
            IntMskPhy(0)
        }
    }
    impl core::fmt::Debug for IntMskPhy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskPhy")
                .field("mask_phy_errsoths_0", &self.mask_phy_errsoths_0())
                .field("mask_phy_errsoths_1", &self.mask_phy_errsoths_1())
                .field("mask_phy_erresc_0", &self.mask_phy_erresc_0())
                .field("mask_phy_erresc_1", &self.mask_phy_erresc_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskPhy {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskPhy {{ mask_phy_errsoths_0: {=bool:?}, mask_phy_errsoths_1: {=bool:?}, mask_phy_erresc_0: {=bool:?}, mask_phy_erresc_1: {=bool:?} }}" , self . mask_phy_errsoths_0 () , self . mask_phy_errsoths_1 () , self . mask_phy_erresc_0 () , self . mask_phy_erresc_1 ())
        }
    }
    #[doc = "interrupt mask for int_st_phy_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPhyFatal(pub u32);
    impl IntMskPhyFatal {
        #[doc = "mask for phy_errsotsynchs_0."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsotsynchs_0."]
        #[inline(always)]
        pub const fn set_mask_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "mask for phy_errsotsynchs_1."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "mask for phy_errsotsynchs_1."]
        #[inline(always)]
        pub const fn set_mask_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "mask for err_deskew."]
        #[must_use]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "mask for err_deskew."]
        #[inline(always)]
        pub const fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntMskPhyFatal {
        #[inline(always)]
        fn default() -> IntMskPhyFatal {
            IntMskPhyFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskPhyFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskPhyFatal")
                .field("mask_phy_errsotsynchs_0", &self.mask_phy_errsotsynchs_0())
                .field("mask_phy_errsotsynchs_1", &self.mask_phy_errsotsynchs_1())
                .field("err_deskew", &self.err_deskew())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskPhyFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskPhyFatal {{ mask_phy_errsotsynchs_0: {=bool:?}, mask_phy_errsotsynchs_1: {=bool:?}, err_deskew: {=bool:?} }}" , self . mask_phy_errsotsynchs_0 () , self . mask_phy_errsotsynchs_1 () , self . err_deskew ())
        }
    }
    #[doc = "interrupt mask for int_st_pkt_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPktFatal(pub u32);
    impl IntMskPktFatal {
        #[doc = "mask for err_ecc_double."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mask for err_ecc_double."]
        #[inline(always)]
        pub const fn set_mask_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntMskPktFatal {
        #[inline(always)]
        fn default() -> IntMskPktFatal {
            IntMskPktFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskPktFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskPktFatal")
                .field("mask_err_ecc_double", &self.mask_err_ecc_double())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskPktFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntMskPktFatal {{ mask_err_ecc_double: {=bool:?} }}",
                self.mask_err_ecc_double()
            )
        }
    }
    #[doc = "interrupt mask for int_st_crc_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskPldCrcFrameFatal(pub u32);
    impl IntMskPldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_msk_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskPldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntMskPldCrcFrameFatal {
            IntMskPldCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskPldCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskPldCrcFrameFatal")
                .field("msk_err_crc_match_vc0", &self.msk_err_crc_match_vc0())
                .field("msk_err_crc_match_vc1", &self.msk_err_crc_match_vc1())
                .field("msk_err_crc_match_vc2", &self.msk_err_crc_match_vc2())
                .field("msk_err_crc_match_vc3", &self.msk_err_crc_match_vc3())
                .field("msk_err_crc_match_vc4", &self.msk_err_crc_match_vc4())
                .field("msk_err_crc_match_vc5", &self.msk_err_crc_match_vc5())
                .field("msk_err_crc_match_vc6", &self.msk_err_crc_match_vc6())
                .field("msk_err_crc_match_vc7", &self.msk_err_crc_match_vc7())
                .field("msk_err_crc_match_vc8", &self.msk_err_crc_match_vc8())
                .field("msk_err_crc_match_vc9", &self.msk_err_crc_match_vc9())
                .field("msk_err_crc_match_vc10", &self.msk_err_crc_match_vc10())
                .field("msk_err_crc_match_vc11", &self.msk_err_crc_match_vc11())
                .field("msk_err_crc_match_vc12", &self.msk_err_crc_match_vc12())
                .field("msk_err_crc_match_vc13", &self.msk_err_crc_match_vc13())
                .field("msk_err_crc_match_vc14", &self.msk_err_crc_match_vc14())
                .field("msk_err_crc_match_vc15", &self.msk_err_crc_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskPldCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskPldCrcFrameFatal {{ msk_err_crc_match_vc0: {=bool:?}, msk_err_crc_match_vc1: {=bool:?}, msk_err_crc_match_vc2: {=bool:?}, msk_err_crc_match_vc3: {=bool:?}, msk_err_crc_match_vc4: {=bool:?}, msk_err_crc_match_vc5: {=bool:?}, msk_err_crc_match_vc6: {=bool:?}, msk_err_crc_match_vc7: {=bool:?}, msk_err_crc_match_vc8: {=bool:?}, msk_err_crc_match_vc9: {=bool:?}, msk_err_crc_match_vc10: {=bool:?}, msk_err_crc_match_vc11: {=bool:?}, msk_err_crc_match_vc12: {=bool:?}, msk_err_crc_match_vc13: {=bool:?}, msk_err_crc_match_vc14: {=bool:?}, msk_err_crc_match_vc15: {=bool:?} }}" , self . msk_err_crc_match_vc0 () , self . msk_err_crc_match_vc1 () , self . msk_err_crc_match_vc2 () , self . msk_err_crc_match_vc3 () , self . msk_err_crc_match_vc4 () , self . msk_err_crc_match_vc5 () , self . msk_err_crc_match_vc6 () , self . msk_err_crc_match_vc7 () , self . msk_err_crc_match_vc8 () , self . msk_err_crc_match_vc9 () , self . msk_err_crc_match_vc10 () , self . msk_err_crc_match_vc11 () , self . msk_err_crc_match_vc12 () , self . msk_err_crc_match_vc13 () , self . msk_err_crc_match_vc14 () , self . msk_err_crc_match_vc15 ())
        }
    }
    #[doc = "interrupt mask for int_st_seq_frame_fatal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMskSeqFrameFatal(pub u32);
    impl IntMskSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_msk_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntMskSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntMskSeqFrameFatal {
            IntMskSeqFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntMskSeqFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMskSeqFrameFatal")
                .field("msk_err_f_seq_match_vc0", &self.msk_err_f_seq_match_vc0())
                .field("msk_err_f_seq_match_vc1", &self.msk_err_f_seq_match_vc1())
                .field("msk_err_f_seq_match_vc2", &self.msk_err_f_seq_match_vc2())
                .field("msk_err_f_seq_match_vc3", &self.msk_err_f_seq_match_vc3())
                .field("msk_err_f_seq_match_vc4", &self.msk_err_f_seq_match_vc4())
                .field("msk_err_f_seq_match_vc5", &self.msk_err_f_seq_match_vc5())
                .field("msk_err_f_seq_match_vc6", &self.msk_err_f_seq_match_vc6())
                .field("msk_err_f_seq_match_vc7", &self.msk_err_f_seq_match_vc7())
                .field("msk_err_f_seq_match_vc8", &self.msk_err_f_seq_match_vc8())
                .field("msk_err_f_seq_match_vc9", &self.msk_err_f_seq_match_vc9())
                .field("msk_err_f_seq_match_vc10", &self.msk_err_f_seq_match_vc10())
                .field("msk_err_f_seq_match_vc11", &self.msk_err_f_seq_match_vc11())
                .field("msk_err_f_seq_match_vc12", &self.msk_err_f_seq_match_vc12())
                .field("msk_err_f_seq_match_vc13", &self.msk_err_f_seq_match_vc13())
                .field("msk_err_f_seq_match_vc14", &self.msk_err_f_seq_match_vc14())
                .field("msk_err_f_seq_match_vc15", &self.msk_err_f_seq_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntMskSeqFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntMskSeqFrameFatal {{ msk_err_f_seq_match_vc0: {=bool:?}, msk_err_f_seq_match_vc1: {=bool:?}, msk_err_f_seq_match_vc2: {=bool:?}, msk_err_f_seq_match_vc3: {=bool:?}, msk_err_f_seq_match_vc4: {=bool:?}, msk_err_f_seq_match_vc5: {=bool:?}, msk_err_f_seq_match_vc6: {=bool:?}, msk_err_f_seq_match_vc7: {=bool:?}, msk_err_f_seq_match_vc8: {=bool:?}, msk_err_f_seq_match_vc9: {=bool:?}, msk_err_f_seq_match_vc10: {=bool:?}, msk_err_f_seq_match_vc11: {=bool:?}, msk_err_f_seq_match_vc12: {=bool:?}, msk_err_f_seq_match_vc13: {=bool:?}, msk_err_f_seq_match_vc14: {=bool:?}, msk_err_f_seq_match_vc15: {=bool:?} }}" , self . msk_err_f_seq_match_vc0 () , self . msk_err_f_seq_match_vc1 () , self . msk_err_f_seq_match_vc2 () , self . msk_err_f_seq_match_vc3 () , self . msk_err_f_seq_match_vc4 () , self . msk_err_f_seq_match_vc5 () , self . msk_err_f_seq_match_vc6 () , self . msk_err_f_seq_match_vc7 () , self . msk_err_f_seq_match_vc8 () , self . msk_err_f_seq_match_vc9 () , self . msk_err_f_seq_match_vc10 () , self . msk_err_f_seq_match_vc11 () , self . msk_err_f_seq_match_vc12 () , self . msk_err_f_seq_match_vc13 () , self . msk_err_f_seq_match_vc14 () , self . msk_err_f_seq_match_vc15 ())
        }
    }
    #[doc = "groups and notifies which interruption bits caused the interruption."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApGeneric(pub u32);
    impl IntStApGeneric {
        #[doc = "ap error in apb block."]
        #[must_use]
        #[inline(always)]
        pub const fn apb_ap_err(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in apb block."]
        #[inline(always)]
        pub const fn set_apb_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ap error in register bank block."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_bank_ap_err(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in register bank block."]
        #[inline(always)]
        pub const fn set_reg_bank_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "ap error in de-skew block."]
        #[must_use]
        #[inline(always)]
        pub const fn de_skew_ap_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in de-skew block."]
        #[inline(always)]
        pub const fn set_de_skew_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ap error in pipeline delay block."]
        #[must_use]
        #[inline(always)]
        pub const fn pipeline_delay_ap_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in pipeline delay block."]
        #[inline(always)]
        pub const fn set_pipeline_delay_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ap error in descrambler block."]
        #[must_use]
        #[inline(always)]
        pub const fn descrambler_ap_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in descrambler block."]
        #[inline(always)]
        pub const fn set_descrambler_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ap error in phy adapter block."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_adapter_ap_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in phy adapter block."]
        #[inline(always)]
        pub const fn set_phy_adapter_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ap error in packet analyzer block."]
        #[must_use]
        #[inline(always)]
        pub const fn packet_analyzer_ap_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in packet analyzer block."]
        #[inline(always)]
        pub const fn set_packet_analyzer_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "ap error in prepare outs block."]
        #[must_use]
        #[inline(always)]
        pub const fn prep_outs_ap_err(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "ap error in prepare outs block."]
        #[inline(always)]
        pub const fn set_prep_outs_ap_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "ap error in err msgr block."]
        #[must_use]
        #[inline(always)]
        pub const fn err_msgr_ap_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in err msgr block."]
        #[inline(always)]
        pub const fn set_err_msgr_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ap error in error handler block."]
        #[must_use]
        #[inline(always)]
        pub const fn err_handle_ap_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in error handler block."]
        #[inline(always)]
        pub const fn set_err_handle_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ap error in synchronizer block for fpclk domain."]
        #[must_use]
        #[inline(always)]
        pub const fn synchronizer_fpclk_ap_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for fpclk domain."]
        #[inline(always)]
        pub const fn set_synchronizer_fpclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ap error in synchronizer block for rxbyteclkhs domain."]
        #[must_use]
        #[inline(always)]
        pub const fn synchronizer_rxbyteclkhs_ap_err(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for rxbyteclkhs domain."]
        #[inline(always)]
        pub const fn set_synchronizer_rxbyteclkhs_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ap error in synchronizer block for pixclk domain."]
        #[must_use]
        #[inline(always)]
        pub const fn synchronizer_pixclk_ap_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ap error in synchronizer block for pixclk domain."]
        #[inline(always)]
        pub const fn set_synchronizer_pixclk_ap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IntStApGeneric {
        #[inline(always)]
        fn default() -> IntStApGeneric {
            IntStApGeneric(0)
        }
    }
    impl core::fmt::Debug for IntStApGeneric {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStApGeneric")
                .field("apb_ap_err", &self.apb_ap_err())
                .field("reg_bank_ap_err", &self.reg_bank_ap_err())
                .field("de_skew_ap_err", &self.de_skew_ap_err())
                .field("pipeline_delay_ap_err", &self.pipeline_delay_ap_err())
                .field("descrambler_ap_err", &self.descrambler_ap_err())
                .field("phy_adapter_ap_err", &self.phy_adapter_ap_err())
                .field("packet_analyzer_ap_err", &self.packet_analyzer_ap_err())
                .field("prep_outs_ap_err", &self.prep_outs_ap_err())
                .field("err_msgr_ap_err", &self.err_msgr_ap_err())
                .field("err_handle_ap_err", &self.err_handle_ap_err())
                .field(
                    "synchronizer_fpclk_ap_err",
                    &self.synchronizer_fpclk_ap_err(),
                )
                .field(
                    "synchronizer_rxbyteclkhs_ap_err",
                    &self.synchronizer_rxbyteclkhs_ap_err(),
                )
                .field(
                    "synchronizer_pixclk_ap_err",
                    &self.synchronizer_pixclk_ap_err(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStApGeneric {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStApGeneric {{ apb_ap_err: {=u8:?}, reg_bank_ap_err: {=u8:?}, de_skew_ap_err: {=bool:?}, pipeline_delay_ap_err: {=bool:?}, descrambler_ap_err: {=bool:?}, phy_adapter_ap_err: {=bool:?}, packet_analyzer_ap_err: {=u8:?}, prep_outs_ap_err: {=u8:?}, err_msgr_ap_err: {=bool:?}, err_handle_ap_err: {=bool:?}, synchronizer_fpclk_ap_err: {=bool:?}, synchronizer_rxbyteclkhs_ap_err: {=bool:?}, synchronizer_pixclk_ap_err: {=bool:?} }}" , self . apb_ap_err () , self . reg_bank_ap_err () , self . de_skew_ap_err () , self . pipeline_delay_ap_err () , self . descrambler_ap_err () , self . phy_adapter_ap_err () , self . packet_analyzer_ap_err () , self . prep_outs_ap_err () , self . err_msgr_ap_err () , self . err_handle_ap_err () , self . synchronizer_fpclk_ap_err () , self . synchronizer_rxbyteclkhs_ap_err () , self . synchronizer_pixclk_ap_err ())
        }
    }
    #[doc = "groups and notifies which interruption bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApIpiFatal(pub u32);
    impl IntStApIpiFatal {
        #[doc = "ap parity tx error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_tx_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ap parity tx error in ipi1."]
        #[inline(always)]
        pub const fn set_parity_tx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ap parity rx error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_rx_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ap parity rx error in ipi1."]
        #[inline(always)]
        pub const fn set_parity_rx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ap ecc sigle error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn ecc_single_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ap ecc sigle error in ipi1."]
        #[inline(always)]
        pub const fn set_ecc_single_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ap ecc multiple error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn ecc_multiple_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ap ecc multiple error in ipi1."]
        #[inline(always)]
        pub const fn set_ecc_multiple_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ap crc error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ap crc error in ipi1."]
        #[inline(always)]
        pub const fn set_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ap redundancy error in ipi1."]
        #[must_use]
        #[inline(always)]
        pub const fn redundancy_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ap redundancy error in ipi1."]
        #[inline(always)]
        pub const fn set_redundancy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntStApIpiFatal {
        #[inline(always)]
        fn default() -> IntStApIpiFatal {
            IntStApIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntStApIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStApIpiFatal")
                .field("parity_tx_err", &self.parity_tx_err())
                .field("parity_rx_err", &self.parity_rx_err())
                .field("ecc_single_err", &self.ecc_single_err())
                .field("ecc_multiple_err", &self.ecc_multiple_err())
                .field("crc_err", &self.crc_err())
                .field("redundancy_err", &self.redundancy_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStApIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStApIpiFatal {{ parity_tx_err: {=bool:?}, parity_rx_err: {=bool:?}, ecc_single_err: {=bool:?}, ecc_multiple_err: {=bool:?}, crc_err: {=bool:?}, redundancy_err: {=bool:?} }}" , self . parity_tx_err () , self . parity_rx_err () , self . ecc_single_err () , self . ecc_multiple_err () , self . crc_err () , self . redundancy_err ())
        }
    }
    #[doc = "contains the status of individual interrupt sources."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStApMain(pub u32);
    impl IntStApMain {
        #[doc = "status of int_st_ap_generic."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_st_ap_generic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ap_generic."]
        #[inline(always)]
        pub const fn set_status_int_st_ap_generic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "status of int_st_phy_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_phy_fatal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub const fn set_status_int_phy_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_pkt_fatal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub const fn set_status_int_pkt_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_bndry_frame_fatal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_bndry_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_seq_frame_fatal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_seq_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_crc_frame_fatal(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_crc_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "status of int_st_phy."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_phy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub const fn set_status_int_phy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_pld_crc_fatal(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub const fn set_status_int_pld_crc_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "status of status_int_data_id."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_data_id(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub const fn set_status_int_data_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_ecc_corrected(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub const fn set_status_int_ecc_corrected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "status of int_st_line."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_line(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub const fn set_status_int_line(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "status of int_st_ap_ipi_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_st_ap_ipi_fatal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ap_ipi_fatal."]
        #[inline(always)]
        pub const fn set_status_int_st_ap_ipi_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_ipi_fatal(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub const fn set_status_int_ipi_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for IntStApMain {
        #[inline(always)]
        fn default() -> IntStApMain {
            IntStApMain(0)
        }
    }
    impl core::fmt::Debug for IntStApMain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStApMain")
                .field("status_int_st_ap_generic", &self.status_int_st_ap_generic())
                .field("status_int_phy_fatal", &self.status_int_phy_fatal())
                .field("status_int_pkt_fatal", &self.status_int_pkt_fatal())
                .field(
                    "status_int_bndry_frame_fatal",
                    &self.status_int_bndry_frame_fatal(),
                )
                .field(
                    "status_int_seq_frame_fatal",
                    &self.status_int_seq_frame_fatal(),
                )
                .field(
                    "status_int_crc_frame_fatal",
                    &self.status_int_crc_frame_fatal(),
                )
                .field("status_int_phy", &self.status_int_phy())
                .field("status_int_pld_crc_fatal", &self.status_int_pld_crc_fatal())
                .field("status_int_data_id", &self.status_int_data_id())
                .field("status_int_ecc_corrected", &self.status_int_ecc_corrected())
                .field("status_int_line", &self.status_int_line())
                .field(
                    "status_int_st_ap_ipi_fatal",
                    &self.status_int_st_ap_ipi_fatal(),
                )
                .field("status_int_ipi_fatal", &self.status_int_ipi_fatal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStApMain {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStApMain {{ status_int_st_ap_generic: {=bool:?}, status_int_phy_fatal: {=bool:?}, status_int_pkt_fatal: {=bool:?}, status_int_bndry_frame_fatal: {=bool:?}, status_int_seq_frame_fatal: {=bool:?}, status_int_crc_frame_fatal: {=bool:?}, status_int_phy: {=bool:?}, status_int_pld_crc_fatal: {=bool:?}, status_int_data_id: {=bool:?}, status_int_ecc_corrected: {=bool:?}, status_int_line: {=bool:?}, status_int_st_ap_ipi_fatal: {=bool:?}, status_int_ipi_fatal: {=bool:?} }}" , self . status_int_st_ap_generic () , self . status_int_phy_fatal () , self . status_int_pkt_fatal () , self . status_int_bndry_frame_fatal () , self . status_int_seq_frame_fatal () , self . status_int_crc_frame_fatal () , self . status_int_phy () , self . status_int_pld_crc_fatal () , self . status_int_data_id () , self . status_int_ecc_corrected () , self . status_int_line () , self . status_int_st_ap_ipi_fatal () , self . status_int_ipi_fatal ())
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStBndryFrameFatal(pub u32);
    impl IntStBndryFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_bndry_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_err_f_bndry_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStBndryFrameFatal {
        #[inline(always)]
        fn default() -> IntStBndryFrameFatal {
            IntStBndryFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntStBndryFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStBndryFrameFatal")
                .field("err_f_bndry_match_vc0", &self.err_f_bndry_match_vc0())
                .field("err_f_bndry_match_vc1", &self.err_f_bndry_match_vc1())
                .field("err_f_bndry_match_vc2", &self.err_f_bndry_match_vc2())
                .field("err_f_bndry_match_vc3", &self.err_f_bndry_match_vc3())
                .field("err_f_bndry_match_vc4", &self.err_f_bndry_match_vc4())
                .field("err_f_bndry_match_vc5", &self.err_f_bndry_match_vc5())
                .field("err_f_bndry_match_vc6", &self.err_f_bndry_match_vc6())
                .field("err_f_bndry_match_vc7", &self.err_f_bndry_match_vc7())
                .field("err_f_bndry_match_vc8", &self.err_f_bndry_match_vc8())
                .field("err_f_bndry_match_vc9", &self.err_f_bndry_match_vc9())
                .field("err_f_bndry_match_vc10", &self.err_f_bndry_match_vc10())
                .field("err_f_bndry_match_vc11", &self.err_f_bndry_match_vc11())
                .field("err_f_bndry_match_vc12", &self.err_f_bndry_match_vc12())
                .field("err_f_bndry_match_vc13", &self.err_f_bndry_match_vc13())
                .field("err_f_bndry_match_vc14", &self.err_f_bndry_match_vc14())
                .field("err_f_bndry_match_vc15", &self.err_f_bndry_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStBndryFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStBndryFrameFatal {{ err_f_bndry_match_vc0: {=bool:?}, err_f_bndry_match_vc1: {=bool:?}, err_f_bndry_match_vc2: {=bool:?}, err_f_bndry_match_vc3: {=bool:?}, err_f_bndry_match_vc4: {=bool:?}, err_f_bndry_match_vc5: {=bool:?}, err_f_bndry_match_vc6: {=bool:?}, err_f_bndry_match_vc7: {=bool:?}, err_f_bndry_match_vc8: {=bool:?}, err_f_bndry_match_vc9: {=bool:?}, err_f_bndry_match_vc10: {=bool:?}, err_f_bndry_match_vc11: {=bool:?}, err_f_bndry_match_vc12: {=bool:?}, err_f_bndry_match_vc13: {=bool:?}, err_f_bndry_match_vc14: {=bool:?}, err_f_bndry_match_vc15: {=bool:?} }}" , self . err_f_bndry_match_vc0 () , self . err_f_bndry_match_vc1 () , self . err_f_bndry_match_vc2 () , self . err_f_bndry_match_vc3 () , self . err_f_bndry_match_vc4 () , self . err_f_bndry_match_vc5 () , self . err_f_bndry_match_vc6 () , self . err_f_bndry_match_vc7 () , self . err_f_bndry_match_vc8 () , self . err_f_bndry_match_vc9 () , self . err_f_bndry_match_vc10 () , self . err_f_bndry_match_vc11 () , self . err_f_bndry_match_vc12 () , self . err_f_bndry_match_vc13 () , self . err_f_bndry_match_vc14 () , self . err_f_bndry_match_vc15 ())
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStCrcFrameFatal(pub u32);
    impl IntStCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_err_f_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntStCrcFrameFatal {
            IntStCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntStCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStCrcFrameFatal")
                .field("err_f_crc_match_vc0", &self.err_f_crc_match_vc0())
                .field("err_f_crc_match_vc1", &self.err_f_crc_match_vc1())
                .field("err_f_crc_match_vc2", &self.err_f_crc_match_vc2())
                .field("err_f_crc_match_vc3", &self.err_f_crc_match_vc3())
                .field("err_f_crc_match_vc4", &self.err_f_crc_match_vc4())
                .field("err_f_crc_match_vc5", &self.err_f_crc_match_vc5())
                .field("err_f_crc_match_vc6", &self.err_f_crc_match_vc6())
                .field("err_f_crc_match_vc7", &self.err_f_crc_match_vc7())
                .field("err_f_crc_match_vc8", &self.err_f_crc_match_vc8())
                .field("err_f_crc_match_vc9", &self.err_f_crc_match_vc9())
                .field("err_f_crc_match_vc10", &self.err_f_crc_match_vc10())
                .field("err_f_crc_match_vc11", &self.err_f_crc_match_vc11())
                .field("err_f_crc_match_vc12", &self.err_f_crc_match_vc12())
                .field("err_f_crc_match_vc13", &self.err_f_crc_match_vc13())
                .field("err_f_crc_match_vc14", &self.err_f_crc_match_vc14())
                .field("err_f_crc_match_vc15", &self.err_f_crc_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStCrcFrameFatal {{ err_f_crc_match_vc0: {=bool:?}, err_f_crc_match_vc1: {=bool:?}, err_f_crc_match_vc2: {=bool:?}, err_f_crc_match_vc3: {=bool:?}, err_f_crc_match_vc4: {=bool:?}, err_f_crc_match_vc5: {=bool:?}, err_f_crc_match_vc6: {=bool:?}, err_f_crc_match_vc7: {=bool:?}, err_f_crc_match_vc8: {=bool:?}, err_f_crc_match_vc9: {=bool:?}, err_f_crc_match_vc10: {=bool:?}, err_f_crc_match_vc11: {=bool:?}, err_f_crc_match_vc12: {=bool:?}, err_f_crc_match_vc13: {=bool:?}, err_f_crc_match_vc14: {=bool:?}, err_f_crc_match_vc15: {=bool:?} }}" , self . err_f_crc_match_vc0 () , self . err_f_crc_match_vc1 () , self . err_f_crc_match_vc2 () , self . err_f_crc_match_vc3 () , self . err_f_crc_match_vc4 () , self . err_f_crc_match_vc5 () , self . err_f_crc_match_vc6 () , self . err_f_crc_match_vc7 () , self . err_f_crc_match_vc8 () , self . err_f_crc_match_vc9 () , self . err_f_crc_match_vc10 () , self . err_f_crc_match_vc11 () , self . err_f_crc_match_vc12 () , self . err_f_crc_match_vc13 () , self . err_f_crc_match_vc14 () , self . err_f_crc_match_vc15 ())
        }
    }
    #[doc = "fatal interruption caused by ipi interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStIpiFatal(pub u32);
    impl IntStIpiFatal {
        #[doc = "the fifo has become empty before the expected bumber of pixels could be extracted to the pixel intefcese."]
        #[must_use]
        #[inline(always)]
        pub const fn pixel_if_fifo_underflow(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo has become empty before the expected bumber of pixels could be extracted to the pixel intefcese."]
        #[inline(always)]
        pub const fn set_pixel_if_fifo_underflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the fifo of pixel interface has lost information because some data arrived and fifo is already full."]
        #[must_use]
        #[inline(always)]
        pub const fn pixel_if_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo of pixel interface has lost information because some data arrived and fifo is already full."]
        #[inline(always)]
        pub const fn set_pixel_if_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "whenever in controller mode, notifies if a new frame is received but previous has not been completed."]
        #[must_use]
        #[inline(always)]
        pub const fn pixel_if_frame_sync_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "whenever in controller mode, notifies if a new frame is received but previous has not been completed."]
        #[inline(always)]
        pub const fn set_pixel_if_frame_sync_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the fifo of pixel interface is not empty at the starat of a new frame."]
        #[must_use]
        #[inline(always)]
        pub const fn pixel_if_fifo_nempty_fs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the fifo of pixel interface is not empty at the starat of a new frame."]
        #[inline(always)]
        pub const fn set_pixel_if_fifo_nempty_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "horizontal line time error."]
        #[must_use]
        #[inline(always)]
        pub const fn pixel_if_hline_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "horizontal line time error."]
        #[inline(always)]
        pub const fn set_pixel_if_hline_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "reporting internal fifo overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn int_event_fifo_overflow(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "reporting internal fifo overflow."]
        #[inline(always)]
        pub const fn set_int_event_fifo_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntStIpiFatal {
        #[inline(always)]
        fn default() -> IntStIpiFatal {
            IntStIpiFatal(0)
        }
    }
    impl core::fmt::Debug for IntStIpiFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStIpiFatal")
                .field("pixel_if_fifo_underflow", &self.pixel_if_fifo_underflow())
                .field("pixel_if_fifo_overflow", &self.pixel_if_fifo_overflow())
                .field("pixel_if_frame_sync_err", &self.pixel_if_frame_sync_err())
                .field("pixel_if_fifo_nempty_fs", &self.pixel_if_fifo_nempty_fs())
                .field("pixel_if_hline_err", &self.pixel_if_hline_err())
                .field("int_event_fifo_overflow", &self.int_event_fifo_overflow())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStIpiFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStIpiFatal {{ pixel_if_fifo_underflow: {=bool:?}, pixel_if_fifo_overflow: {=bool:?}, pixel_if_frame_sync_err: {=bool:?}, pixel_if_fifo_nempty_fs: {=bool:?}, pixel_if_hline_err: {=bool:?}, int_event_fifo_overflow: {=bool:?} }}" , self . pixel_if_fifo_underflow () , self . pixel_if_fifo_overflow () , self . pixel_if_frame_sync_err () , self . pixel_if_fifo_nempty_fs () , self . pixel_if_hline_err () , self . int_event_fifo_overflow ())
        }
    }
    #[doc = "contains the stateus of individual interrupt sources."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStMain(pub u32);
    impl IntStMain {
        #[doc = "status of int_st_phy_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_phy_fatal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy_fatal."]
        #[inline(always)]
        pub const fn set_status_int_phy_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_pkt_fatal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_pkt_fatal."]
        #[inline(always)]
        pub const fn set_status_int_pkt_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_bndry_frame_fatal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_bndry_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_bndry_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_seq_frame_fatal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_seq_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_seq_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_crc_frame_fatal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_crc_frame_fatal."]
        #[inline(always)]
        pub const fn set_status_int_crc_frame_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_pld_crc_fatal(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_pld_crc_fatal."]
        #[inline(always)]
        pub const fn set_status_int_pld_crc_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "status of status_int_data_id."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_data_id(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_data_id."]
        #[inline(always)]
        pub const fn set_status_int_data_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_ecc_corrected(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "status of status_int_ecc_corrected."]
        #[inline(always)]
        pub const fn set_status_int_ecc_corrected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "status of int_st_phy."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_phy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_phy."]
        #[inline(always)]
        pub const fn set_status_int_phy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "status of int_st_line."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_line(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_line."]
        #[inline(always)]
        pub const fn set_status_int_line(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[must_use]
        #[inline(always)]
        pub const fn status_int_ipi4_fatal(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "status of int_st_ipi_fatal."]
        #[inline(always)]
        pub const fn set_status_int_ipi4_fatal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntStMain {
        #[inline(always)]
        fn default() -> IntStMain {
            IntStMain(0)
        }
    }
    impl core::fmt::Debug for IntStMain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStMain")
                .field("status_int_phy_fatal", &self.status_int_phy_fatal())
                .field("status_int_pkt_fatal", &self.status_int_pkt_fatal())
                .field(
                    "status_int_bndry_frame_fatal",
                    &self.status_int_bndry_frame_fatal(),
                )
                .field(
                    "status_int_seq_frame_fatal",
                    &self.status_int_seq_frame_fatal(),
                )
                .field(
                    "status_int_crc_frame_fatal",
                    &self.status_int_crc_frame_fatal(),
                )
                .field("status_int_pld_crc_fatal", &self.status_int_pld_crc_fatal())
                .field("status_int_data_id", &self.status_int_data_id())
                .field("status_int_ecc_corrected", &self.status_int_ecc_corrected())
                .field("status_int_phy", &self.status_int_phy())
                .field("status_int_line", &self.status_int_line())
                .field("status_int_ipi4_fatal", &self.status_int_ipi4_fatal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStMain {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStMain {{ status_int_phy_fatal: {=bool:?}, status_int_pkt_fatal: {=bool:?}, status_int_bndry_frame_fatal: {=bool:?}, status_int_seq_frame_fatal: {=bool:?}, status_int_crc_frame_fatal: {=bool:?}, status_int_pld_crc_fatal: {=bool:?}, status_int_data_id: {=bool:?}, status_int_ecc_corrected: {=bool:?}, status_int_phy: {=bool:?}, status_int_line: {=bool:?}, status_int_ipi4_fatal: {=bool:?} }}" , self . status_int_phy_fatal () , self . status_int_pkt_fatal () , self . status_int_bndry_frame_fatal () , self . status_int_seq_frame_fatal () , self . status_int_crc_frame_fatal () , self . status_int_pld_crc_fatal () , self . status_int_data_id () , self . status_int_ecc_corrected () , self . status_int_phy () , self . status_int_line () , self . status_int_ipi4_fatal ())
        }
    }
    #[doc = "interruption caused by phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPhy(pub u32);
    impl IntStPhy {
        #[doc = "start of transmission error on data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_errsoths_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub const fn set_phy_errsoths_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start of transmission error on data lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_errsoths_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub const fn set_phy_errsoths_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "start of transmission error on data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_erresc_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 0."]
        #[inline(always)]
        pub const fn set_phy_erresc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "start of transmission error on data lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_erresc_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane 1."]
        #[inline(always)]
        pub const fn set_phy_erresc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IntStPhy {
        #[inline(always)]
        fn default() -> IntStPhy {
            IntStPhy(0)
        }
    }
    impl core::fmt::Debug for IntStPhy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStPhy")
                .field("phy_errsoths_0", &self.phy_errsoths_0())
                .field("phy_errsoths_1", &self.phy_errsoths_1())
                .field("phy_erresc_0", &self.phy_erresc_0())
                .field("phy_erresc_1", &self.phy_erresc_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStPhy {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStPhy {{ phy_errsoths_0: {=bool:?}, phy_errsoths_1: {=bool:?}, phy_erresc_0: {=bool:?}, phy_erresc_1: {=bool:?} }}" , self . phy_errsoths_0 () , self . phy_errsoths_1 () , self . phy_erresc_0 () , self . phy_erresc_1 ())
        }
    }
    #[doc = "groups the phy interruptions caused by phy packets discarded."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPhyFatal(pub u32);
    impl IntStPhyFatal {
        #[doc = "start of transmission error on data lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_errsotsynchs_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane0."]
        #[inline(always)]
        pub const fn set_phy_errsotsynchs_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start of transmission error on data lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_errsotsynchs_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start of transmission error on data lane1."]
        #[inline(always)]
        pub const fn set_phy_errsotsynchs_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "reports whenever data is lost due to an existent skew between lanes greater than 2 rxwordclkhs."]
        #[must_use]
        #[inline(always)]
        pub const fn err_deskew(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "reports whenever data is lost due to an existent skew between lanes greater than 2 rxwordclkhs."]
        #[inline(always)]
        pub const fn set_err_deskew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IntStPhyFatal {
        #[inline(always)]
        fn default() -> IntStPhyFatal {
            IntStPhyFatal(0)
        }
    }
    impl core::fmt::Debug for IntStPhyFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStPhyFatal")
                .field("phy_errsotsynchs_0", &self.phy_errsotsynchs_0())
                .field("phy_errsotsynchs_1", &self.phy_errsotsynchs_1())
                .field("err_deskew", &self.err_deskew())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStPhyFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStPhyFatal {{ phy_errsotsynchs_0: {=bool:?}, phy_errsotsynchs_1: {=bool:?}, err_deskew: {=bool:?} }}" , self . phy_errsotsynchs_0 () , self . phy_errsotsynchs_1 () , self . err_deskew ())
        }
    }
    #[doc = "groups the fatal interruption related with packet construction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPktFatal(pub u32);
    impl IntStPktFatal {
        #[doc = "header ecc contains at least 2 errors."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ecc_double(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "header ecc contains at least 2 errors."]
        #[inline(always)]
        pub const fn set_err_ecc_double(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IntStPktFatal {
        #[inline(always)]
        fn default() -> IntStPktFatal {
            IntStPktFatal(0)
        }
    }
    impl core::fmt::Debug for IntStPktFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStPktFatal")
                .field("err_ecc_double", &self.err_ecc_double())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStPktFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntStPktFatal {{ err_ecc_double: {=bool:?} }}",
                self.err_ecc_double()
            )
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStPldCrcFrameFatal(pub u32);
    impl IntStPldCrcFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn err_crc_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_err_crc_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStPldCrcFrameFatal {
        #[inline(always)]
        fn default() -> IntStPldCrcFrameFatal {
            IntStPldCrcFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntStPldCrcFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStPldCrcFrameFatal")
                .field("err_crc_match_vc0", &self.err_crc_match_vc0())
                .field("err_crc_match_vc1", &self.err_crc_match_vc1())
                .field("err_crc_match_vc2", &self.err_crc_match_vc2())
                .field("err_crc_match_vc3", &self.err_crc_match_vc3())
                .field("err_crc_match_vc4", &self.err_crc_match_vc4())
                .field("err_crc_match_vc5", &self.err_crc_match_vc5())
                .field("err_crc_match_vc6", &self.err_crc_match_vc6())
                .field("err_crc_match_vc7", &self.err_crc_match_vc7())
                .field("err_crc_match_vc8", &self.err_crc_match_vc8())
                .field("err_crc_match_vc9", &self.err_crc_match_vc9())
                .field("err_crc_match_vc10", &self.err_crc_match_vc10())
                .field("err_crc_match_vc11", &self.err_crc_match_vc11())
                .field("err_crc_match_vc12", &self.err_crc_match_vc12())
                .field("err_crc_match_vc13", &self.err_crc_match_vc13())
                .field("err_crc_match_vc14", &self.err_crc_match_vc14())
                .field("err_crc_match_vc15", &self.err_crc_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStPldCrcFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStPldCrcFrameFatal {{ err_crc_match_vc0: {=bool:?}, err_crc_match_vc1: {=bool:?}, err_crc_match_vc2: {=bool:?}, err_crc_match_vc3: {=bool:?}, err_crc_match_vc4: {=bool:?}, err_crc_match_vc5: {=bool:?}, err_crc_match_vc6: {=bool:?}, err_crc_match_vc7: {=bool:?}, err_crc_match_vc8: {=bool:?}, err_crc_match_vc9: {=bool:?}, err_crc_match_vc10: {=bool:?}, err_crc_match_vc11: {=bool:?}, err_crc_match_vc12: {=bool:?}, err_crc_match_vc13: {=bool:?}, err_crc_match_vc14: {=bool:?}, err_crc_match_vc15: {=bool:?} }}" , self . err_crc_match_vc0 () , self . err_crc_match_vc1 () , self . err_crc_match_vc2 () , self . err_crc_match_vc3 () , self . err_crc_match_vc4 () , self . err_crc_match_vc5 () , self . err_crc_match_vc6 () , self . err_crc_match_vc7 () , self . err_crc_match_vc8 () , self . err_crc_match_vc9 () , self . err_crc_match_vc10 () , self . err_crc_match_vc11 () , self . err_crc_match_vc12 () , self . err_crc_match_vc13 () , self . err_crc_match_vc14 () , self . err_crc_match_vc15 ())
        }
    }
    #[doc = "fatal interruption related with matching frame start with frame end for a specific virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStSeqFrameFatal(pub u32);
    impl IntStSeqFrameFatal {
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 0."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 1."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 2."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 3."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 4."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 5."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 6."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 7."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 8."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 9."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 10."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 11."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 12."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 13."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 14."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[must_use]
        #[inline(always)]
        pub const fn err_f_seq_match_vc15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "error matching frame start with frame end for virtual channel 15."]
        #[inline(always)]
        pub const fn set_err_f_seq_match_vc15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for IntStSeqFrameFatal {
        #[inline(always)]
        fn default() -> IntStSeqFrameFatal {
            IntStSeqFrameFatal(0)
        }
    }
    impl core::fmt::Debug for IntStSeqFrameFatal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStSeqFrameFatal")
                .field("err_f_seq_match_vc0", &self.err_f_seq_match_vc0())
                .field("err_f_seq_match_vc1", &self.err_f_seq_match_vc1())
                .field("err_f_seq_match_vc2", &self.err_f_seq_match_vc2())
                .field("err_f_seq_match_vc3", &self.err_f_seq_match_vc3())
                .field("err_f_seq_match_vc4", &self.err_f_seq_match_vc4())
                .field("err_f_seq_match_vc5", &self.err_f_seq_match_vc5())
                .field("err_f_seq_match_vc6", &self.err_f_seq_match_vc6())
                .field("err_f_seq_match_vc7", &self.err_f_seq_match_vc7())
                .field("err_f_seq_match_vc8", &self.err_f_seq_match_vc8())
                .field("err_f_seq_match_vc9", &self.err_f_seq_match_vc9())
                .field("err_f_seq_match_vc10", &self.err_f_seq_match_vc10())
                .field("err_f_seq_match_vc11", &self.err_f_seq_match_vc11())
                .field("err_f_seq_match_vc12", &self.err_f_seq_match_vc12())
                .field("err_f_seq_match_vc13", &self.err_f_seq_match_vc13())
                .field("err_f_seq_match_vc14", &self.err_f_seq_match_vc14())
                .field("err_f_seq_match_vc15", &self.err_f_seq_match_vc15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStSeqFrameFatal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStSeqFrameFatal {{ err_f_seq_match_vc0: {=bool:?}, err_f_seq_match_vc1: {=bool:?}, err_f_seq_match_vc2: {=bool:?}, err_f_seq_match_vc3: {=bool:?}, err_f_seq_match_vc4: {=bool:?}, err_f_seq_match_vc5: {=bool:?}, err_f_seq_match_vc6: {=bool:?}, err_f_seq_match_vc7: {=bool:?}, err_f_seq_match_vc8: {=bool:?}, err_f_seq_match_vc9: {=bool:?}, err_f_seq_match_vc10: {=bool:?}, err_f_seq_match_vc11: {=bool:?}, err_f_seq_match_vc12: {=bool:?}, err_f_seq_match_vc13: {=bool:?}, err_f_seq_match_vc14: {=bool:?}, err_f_seq_match_vc15: {=bool:?} }}" , self . err_f_seq_match_vc0 () , self . err_f_seq_match_vc1 () , self . err_f_seq_match_vc2 () , self . err_f_seq_match_vc3 () , self . err_f_seq_match_vc4 () , self . err_f_seq_match_vc5 () , self . err_f_seq_match_vc6 () , self . err_f_seq_match_vc7 () , self . err_f_seq_match_vc8 () , self . err_f_seq_match_vc9 () , self . err_f_seq_match_vc10 () , self . err_f_seq_match_vc11 () , self . err_f_seq_match_vc12 () , self . err_f_seq_match_vc13 () , self . err_f_seq_match_vc14 () , self . err_f_seq_match_vc15 ())
        }
    }
    #[doc = "configures advanced features for ipi mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiAdvFeatures(pub u32);
    impl IpiAdvFeatures {
        #[doc = "ignore datatype of the header using the programmed datatype for decoding."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_dt_overwrite(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ignore datatype of the header using the programmed datatype for decoding."]
        #[inline(always)]
        pub const fn set_ipi_dt_overwrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "datatype to overwrite."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_dt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "datatype to overwrite."]
        #[inline(always)]
        pub const fn set_ipi_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "for camero mode, allows manual selection of the packet fo line delimiter as follows: 0x0-controller seletc it automaticlly 0x1-select packets from list programmed in 17:21."]
        #[must_use]
        #[inline(always)]
        pub const fn line_event_selection(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "for camero mode, allows manual selection of the packet fo line delimiter as follows: 0x0-controller seletc it automaticlly 0x1-select packets from list programmed in 17:21."]
        #[inline(always)]
        pub const fn set_line_event_selection(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "allows the use of video packets for ipi synchronization events."]
        #[must_use]
        #[inline(always)]
        pub const fn en_video(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of video packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn set_en_video(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "allows the use of line start packets for ipi synchronization events."]
        #[must_use]
        #[inline(always)]
        pub const fn en_line_start(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of line start packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn set_en_line_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "allows the use of null packets for IPI synchronization events."]
        #[must_use]
        #[inline(always)]
        pub const fn en_null(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of null packets for IPI synchronization events."]
        #[inline(always)]
        pub const fn set_en_null(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "allows the use of blankong packets for IPI synchronization events."]
        #[must_use]
        #[inline(always)]
        pub const fn en_blanking(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of blankong packets for IPI synchronization events."]
        #[inline(always)]
        pub const fn set_en_blanking(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "allows the use of embendded packets for ipi synchronization events."]
        #[must_use]
        #[inline(always)]
        pub const fn en_embedded(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "allows the use of embendded packets for ipi synchronization events."]
        #[inline(always)]
        pub const fn set_en_embedded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "for camera mode: 0x0- frame start do not trigger any sync event."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_sync_event_mode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "for camera mode: 0x0- frame start do not trigger any sync event."]
        #[inline(always)]
        pub const fn set_ipi_sync_event_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IpiAdvFeatures {
        #[inline(always)]
        fn default() -> IpiAdvFeatures {
            IpiAdvFeatures(0)
        }
    }
    impl core::fmt::Debug for IpiAdvFeatures {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiAdvFeatures")
                .field("ipi_dt_overwrite", &self.ipi_dt_overwrite())
                .field("ipi_dt", &self.ipi_dt())
                .field("line_event_selection", &self.line_event_selection())
                .field("en_video", &self.en_video())
                .field("en_line_start", &self.en_line_start())
                .field("en_null", &self.en_null())
                .field("en_blanking", &self.en_blanking())
                .field("en_embedded", &self.en_embedded())
                .field("ipi_sync_event_mode", &self.ipi_sync_event_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiAdvFeatures {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IpiAdvFeatures {{ ipi_dt_overwrite: {=bool:?}, ipi_dt: {=u8:?}, line_event_selection: {=bool:?}, en_video: {=bool:?}, en_line_start: {=bool:?}, en_null: {=bool:?}, en_blanking: {=bool:?}, en_embedded: {=bool:?}, ipi_sync_event_mode: {=bool:?} }}" , self . ipi_dt_overwrite () , self . ipi_dt () , self . line_event_selection () , self . en_video () , self . en_line_start () , self . en_null () , self . en_blanking () , self . en_embedded () , self . ipi_sync_event_mode ())
        }
    }
    #[doc = "selects the data type processed by ipi."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiDataType(pub u32);
    impl IpiDataType {
        #[doc = "data type of data to be processed by pixel interface."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_data_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "data type of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn set_ipi_data_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "enable embedded data processing on ipi interface."]
        #[must_use]
        #[inline(always)]
        pub const fn embended_data(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable embedded data processing on ipi interface."]
        #[inline(always)]
        pub const fn set_embended_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IpiDataType {
        #[inline(always)]
        fn default() -> IpiDataType {
            IpiDataType(0)
        }
    }
    impl core::fmt::Debug for IpiDataType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiDataType")
                .field("ipi_data_type", &self.ipi_data_type())
                .field("embended_data", &self.embended_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiDataType {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiDataType {{ ipi_data_type: {=u8:?}, embended_data: {=bool:?} }}",
                self.ipi_data_type(),
                self.embended_data()
            )
        }
    }
    #[doc = "configures the video horizontal synchronism back porch time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHbpTime(pub u32);
    impl IpiHbpTime {
        #[doc = "configures the Horizontal Synchronism back porch period in pixclk cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_hbp_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Synchronism back porch period in pixclk cycles."]
        #[inline(always)]
        pub const fn set_ipi_hbp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHbpTime {
        #[inline(always)]
        fn default() -> IpiHbpTime {
            IpiHbpTime(0)
        }
    }
    impl core::fmt::Debug for IpiHbpTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiHbpTime")
                .field("ipi_hbp_time", &self.ipi_hbp_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiHbpTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiHbpTime {{ ipi_hbp_time: {=u16:?} }}",
                self.ipi_hbp_time()
            )
        }
    }
    #[doc = "configures the overall tiem for each video line."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHlineTime(pub u32);
    impl IpiHlineTime {
        #[doc = "configures the size of the line time counted in pixclk cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_hlin_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configures the size of the line time counted in pixclk cycles."]
        #[inline(always)]
        pub const fn set_ipi_hlin_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for IpiHlineTime {
        #[inline(always)]
        fn default() -> IpiHlineTime {
            IpiHlineTime(0)
        }
    }
    impl core::fmt::Debug for IpiHlineTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiHlineTime")
                .field("ipi_hlin_time", &self.ipi_hlin_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiHlineTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiHlineTime {{ ipi_hlin_time: {=u16:?} }}",
                self.ipi_hlin_time()
            )
        }
    }
    #[doc = "configures the video horizontal synchronism active time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHsaTime(pub u32);
    impl IpiHsaTime {
        #[doc = "configures the Horizontal Synchronism Active period in pixclk cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_hsa_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Synchronism Active period in pixclk cycles."]
        #[inline(always)]
        pub const fn set_ipi_hsa_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHsaTime {
        #[inline(always)]
        fn default() -> IpiHsaTime {
            IpiHsaTime(0)
        }
    }
    impl core::fmt::Debug for IpiHsaTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiHsaTime")
                .field("ipi_hsa_time", &self.ipi_hsa_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiHsaTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiHsaTime {{ ipi_hsa_time: {=u16:?} }}",
                self.ipi_hsa_time()
            )
        }
    }
    #[doc = "configures the vedeo Horizontal Sync Delay time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiHsdTime(pub u32);
    impl IpiHsdTime {
        #[doc = "configures the Horizontal Sync Porch delay period in pixclk cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_hsd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal Sync Porch delay period in pixclk cycles."]
        #[inline(always)]
        pub const fn set_ipi_hsd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for IpiHsdTime {
        #[inline(always)]
        fn default() -> IpiHsdTime {
            IpiHsdTime(0)
        }
    }
    impl core::fmt::Debug for IpiHsdTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiHsdTime")
                .field("ipi_hsd_time", &self.ipi_hsd_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiHsdTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiHsdTime {{ ipi_hsd_time: {=u16:?} }}",
                self.ipi_hsd_time()
            )
        }
    }
    #[doc = "control the flush of ipi memory."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiMemFlash(pub u32);
    impl IpiMemFlash {
        #[doc = "flush ipi memory, this bit is auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_flush(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "flush ipi memory, this bit is auto clear."]
        #[inline(always)]
        pub const fn set_ipi_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "memory is automatically flashed at each vsync."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_auto_flush(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "memory is automatically flashed at each vsync."]
        #[inline(always)]
        pub const fn set_ipi_auto_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for IpiMemFlash {
        #[inline(always)]
        fn default() -> IpiMemFlash {
            IpiMemFlash(0)
        }
    }
    impl core::fmt::Debug for IpiMemFlash {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiMemFlash")
                .field("ipi_flush", &self.ipi_flush())
                .field("ipi_auto_flush", &self.ipi_auto_flush())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiMemFlash {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiMemFlash {{ ipi_flush: {=bool:?}, ipi_auto_flush: {=bool:?} }}",
                self.ipi_flush(),
                self.ipi_auto_flush()
            )
        }
    }
    #[doc = "selects how the ipi interface generates the video frame."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiMode(pub u32);
    impl IpiMode {
        #[doc = "indicates the video mode transmission type 0x0: camera timing 0x1:controller timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the video mode transmission type 0x0: camera timing 0x1:controller timing."]
        #[inline(always)]
        pub const fn set_ipi_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "if color mode components are deliverd as follows: 0x0 48bit intercase 0x1: 16bit interface."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_color_com(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "if color mode components are deliverd as follows: 0x0 48bit intercase 0x1: 16bit interface."]
        #[inline(always)]
        pub const fn set_ipi_color_com(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "cut-through mode state active when high."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_cut_through(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "cut-through mode state active when high."]
        #[inline(always)]
        pub const fn set_ipi_cut_through(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "enables the interface."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_enable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enables the interface."]
        #[inline(always)]
        pub const fn set_ipi_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IpiMode {
        #[inline(always)]
        fn default() -> IpiMode {
            IpiMode(0)
        }
    }
    impl core::fmt::Debug for IpiMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiMode")
                .field("ipi_mode", &self.ipi_mode())
                .field("ipi_color_com", &self.ipi_color_com())
                .field("ipi_cut_through", &self.ipi_cut_through())
                .field("ipi_enable", &self.ipi_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IpiMode {{ ipi_mode: {=bool:?}, ipi_color_com: {=bool:?}, ipi_cut_through: {=bool:?}, ipi_enable: {=bool:?} }}" , self . ipi_mode () , self . ipi_color_com () , self . ipi_cut_through () , self . ipi_enable ())
        }
    }
    #[doc = "congtrols the ipi logic reset state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiSoftrstn(pub u32);
    impl IpiSoftrstn {
        #[doc = "resets ipi one, active low."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_softrstn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "resets ipi one, active low."]
        #[inline(always)]
        pub const fn set_ipi_softrstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IpiSoftrstn {
        #[inline(always)]
        fn default() -> IpiSoftrstn {
            IpiSoftrstn(0)
        }
    }
    impl core::fmt::Debug for IpiSoftrstn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiSoftrstn")
                .field("ipi_softrstn", &self.ipi_softrstn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiSoftrstn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiSoftrstn {{ ipi_softrstn: {=bool:?} }}",
                self.ipi_softrstn()
            )
        }
    }
    #[doc = "configures the vertical resolution of video."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVactiveLines(pub u32);
    impl IpiVactiveLines {
        #[doc = "configures the vertical active period measured in bumber of horizontal lines."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_vactive_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "configures the vertical active period measured in bumber of horizontal lines."]
        #[inline(always)]
        pub const fn set_ipi_vactive_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for IpiVactiveLines {
        #[inline(always)]
        fn default() -> IpiVactiveLines {
            IpiVactiveLines(0)
        }
    }
    impl core::fmt::Debug for IpiVactiveLines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiVactiveLines")
                .field("ipi_vactive_lines", &self.ipi_vactive_lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiVactiveLines {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiVactiveLines {{ ipi_vactive_lines: {=u16:?} }}",
                self.ipi_vactive_lines()
            )
        }
    }
    #[doc = "configures the verticall back porch period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVbpLines(pub u32);
    impl IpiVbpLines {
        #[doc = "configuress the vertical back porch period measured in number of horizontal lines."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_vbp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configuress the vertical back porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn set_ipi_vbp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVbpLines {
        #[inline(always)]
        fn default() -> IpiVbpLines {
            IpiVbpLines(0)
        }
    }
    impl core::fmt::Debug for IpiVbpLines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiVbpLines")
                .field("ipi_vbp_lines", &self.ipi_vbp_lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiVbpLines {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiVbpLines {{ ipi_vbp_lines: {=u16:?} }}",
                self.ipi_vbp_lines()
            )
        }
    }
    #[doc = "selects the vritual channel processed by ipi."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVcid(pub u32);
    impl IpiVcid {
        #[doc = "virtual channel of data to be processed by pixel interface."]
        #[must_use]
        #[inline(always)]
        pub const fn ip_vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "virtual channel of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn set_ip_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "virtual channel extension of data to be processed by pixel interface."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_vcx_0_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "virtual channel extension of data to be processed by pixel interface."]
        #[inline(always)]
        pub const fn set_ipi_vcx_0_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
    }
    impl Default for IpiVcid {
        #[inline(always)]
        fn default() -> IpiVcid {
            IpiVcid(0)
        }
    }
    impl core::fmt::Debug for IpiVcid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiVcid")
                .field("ip_vcid", &self.ip_vcid())
                .field("ipi_vcx_0_1", &self.ipi_vcx_0_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiVcid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiVcid {{ ip_vcid: {=u8:?}, ipi_vcx_0_1: {=u8:?} }}",
                self.ip_vcid(),
                self.ipi_vcx_0_1()
            )
        }
    }
    #[doc = "configures the vertical front porch period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVfpLines(pub u32);
    impl IpiVfpLines {
        #[doc = "configures the vertical front porch period measured in number of horizontall lines."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_vfp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical front porch period measured in number of horizontall lines."]
        #[inline(always)]
        pub const fn set_ipi_vfp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVfpLines {
        #[inline(always)]
        fn default() -> IpiVfpLines {
            IpiVfpLines(0)
        }
    }
    impl core::fmt::Debug for IpiVfpLines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiVfpLines")
                .field("ipi_vfp_lines", &self.ipi_vfp_lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiVfpLines {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiVfpLines {{ ipi_vfp_lines: {=u16:?} }}",
                self.ipi_vfp_lines()
            )
        }
    }
    #[doc = "configures the vertical synchronism active period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpiVsaLines(pub u32);
    impl IpiVsaLines {
        #[doc = "configures the vertical synchronism active period measured in number of horizontal lines."]
        #[must_use]
        #[inline(always)]
        pub const fn ipi_vsa_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn set_ipi_vsa_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for IpiVsaLines {
        #[inline(always)]
        fn default() -> IpiVsaLines {
            IpiVsaLines(0)
        }
    }
    impl core::fmt::Debug for IpiVsaLines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpiVsaLines")
                .field("ipi_vsa_lines", &self.ipi_vsa_lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IpiVsaLines {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IpiVsaLines {{ ipi_vsa_lines: {=u16:?} }}",
                self.ipi_vsa_lines()
            )
        }
    }
    #[doc = "the number of active lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NLanes(pub u32);
    impl NLanes {
        #[doc = "number of active data lanes."]
        #[must_use]
        #[inline(always)]
        pub const fn n_lanes(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "number of active data lanes."]
        #[inline(always)]
        pub const fn set_n_lanes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for NLanes {
        #[inline(always)]
        fn default() -> NLanes {
            NLanes(0)
        }
    }
    impl core::fmt::Debug for NLanes {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NLanes")
                .field("n_lanes", &self.n_lanes())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NLanes {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NLanes {{ n_lanes: {=u8:?} }}", self.n_lanes())
        }
    }
    #[doc = "contains the calibration signal status from synopsys d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCal(pub u32);
    impl PhyCal {
        #[doc = "a low-to-high transition on rxskewcalhs signal means the the phy has initiated the de-skew calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn rxskewcalhs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "a low-to-high transition on rxskewcalhs signal means the the phy has initiated the de-skew calibration."]
        #[inline(always)]
        pub const fn set_rxskewcalhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyCal {
        #[inline(always)]
        fn default() -> PhyCal {
            PhyCal(0)
        }
    }
    impl core::fmt::Debug for PhyCal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCal")
                .field("rxskewcalhs", &self.rxskewcalhs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyCal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhyCal {{ rxskewcalhs: {=bool:?} }}", self.rxskewcalhs())
        }
    }
    #[doc = "contains the status of rx-related signals from phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyRx(pub u32);
    impl PhyRx {
        #[doc = "lane module 0 has entered the ultra low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rxulpsesc_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "lane module 0 has entered the ultra low power mode."]
        #[inline(always)]
        pub const fn set_phy_rxulpsesc_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "lane module 1 has entered the ultra low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rxullpsesc_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "lane module 1 has entered the ultra low power mode."]
        #[inline(always)]
        pub const fn set_phy_rxullpsesc_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "active low. Indicates the d-phy clock lane module has entered the Ultra low power state."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rxulpsclknot(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "active low. Indicates the d-phy clock lane module has entered the Ultra low power state."]
        #[inline(always)]
        pub const fn set_phy_rxulpsclknot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "indicates the d-phy clock lane is actively receiving a ddr clock."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rxclkactivehs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the d-phy clock lane is actively receiving a ddr clock."]
        #[inline(always)]
        pub const fn set_phy_rxclkactivehs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for PhyRx {
        #[inline(always)]
        fn default() -> PhyRx {
            PhyRx(0)
        }
    }
    impl core::fmt::Debug for PhyRx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyRx")
                .field("phy_rxulpsesc_0", &self.phy_rxulpsesc_0())
                .field("phy_rxullpsesc_1", &self.phy_rxullpsesc_1())
                .field("phy_rxulpsclknot", &self.phy_rxulpsclknot())
                .field("phy_rxclkactivehs", &self.phy_rxclkactivehs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyRx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyRx {{ phy_rxulpsesc_0: {=bool:?}, phy_rxullpsesc_1: {=bool:?}, phy_rxulpsclknot: {=bool:?}, phy_rxclkactivehs: {=bool:?} }}" , self . phy_rxulpsesc_0 () , self . phy_rxullpsesc_1 () , self . phy_rxulpsclknot () , self . phy_rxclkactivehs ())
        }
    }
    #[doc = "controls the phy shutdown mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyShutdownz(pub u32);
    impl PhyShutdownz {
        #[doc = "shutdown input,active low."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_shutdownz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "shutdown input,active low."]
        #[inline(always)]
        pub const fn set_phy_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyShutdownz {
        #[inline(always)]
        fn default() -> PhyShutdownz {
            PhyShutdownz(0)
        }
    }
    impl core::fmt::Debug for PhyShutdownz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyShutdownz")
                .field("phy_shutdownz", &self.phy_shutdownz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyShutdownz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PhyShutdownz {{ phy_shutdownz: {=bool:?} }}",
                self.phy_shutdownz()
            )
        }
    }
    #[doc = "contains the stopstate signal status from phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStopstate(pub u32);
    impl PhyStopstate {
        #[doc = "data lane 0 in stop state."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_stopstatedata_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "data lane 0 in stop state."]
        #[inline(always)]
        pub const fn set_phy_stopstatedata_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "data lane 1 in stop state."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_stopstatedata_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "data lane 1 in stop state."]
        #[inline(always)]
        pub const fn set_phy_stopstatedata_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "d-phy clock lane in stop state."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_stopstateclk(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "d-phy clock lane in stop state."]
        #[inline(always)]
        pub const fn set_phy_stopstateclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for PhyStopstate {
        #[inline(always)]
        fn default() -> PhyStopstate {
            PhyStopstate(0)
        }
    }
    impl core::fmt::Debug for PhyStopstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyStopstate")
                .field("phy_stopstatedata_0", &self.phy_stopstatedata_0())
                .field("phy_stopstatedata_1", &self.phy_stopstatedata_1())
                .field("phy_stopstateclk", &self.phy_stopstateclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyStopstate {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyStopstate {{ phy_stopstatedata_0: {=bool:?}, phy_stopstatedata_1: {=bool:?}, phy_stopstateclk: {=bool:?} }}" , self . phy_stopstatedata_0 () , self . phy_stopstatedata_1 () , self . phy_stopstateclk ())
        }
    }
    #[doc = "active extra bits for virtual channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VcExtension(pub u32);
    impl VcExtension {
        #[doc = "indicates status of virtual channel extension: 0-virtual channel extension is enable 1-legacy mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vcx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates status of virtual channel extension: 0-virtual channel extension is enable 1-legacy mode."]
        #[inline(always)]
        pub const fn set_vcx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for VcExtension {
        #[inline(always)]
        fn default() -> VcExtension {
            VcExtension(0)
        }
    }
    impl core::fmt::Debug for VcExtension {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VcExtension")
                .field("vcx", &self.vcx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VcExtension {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VcExtension {{ vcx: {=bool:?} }}", self.vcx())
        }
    }
    #[doc = "version code."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version(pub u32);
    impl Version {
        #[doc = "version code."]
        #[must_use]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "version code."]
        #[inline(always)]
        pub const fn set_version(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Version {
        #[inline(always)]
        fn default() -> Version {
            Version(0)
        }
    }
    impl core::fmt::Debug for Version {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Version")
                .field("version", &self.version())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Version {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Version {{ version: {=u32:?} }}", self.version())
        }
    }
}
