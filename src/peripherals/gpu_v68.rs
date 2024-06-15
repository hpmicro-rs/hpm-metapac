#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GPU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpu {
    ptr: *mut u8,
}
unsafe impl Send for Gpu {}
unsafe impl Sync for Gpu {}
impl Gpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "clock control register."]
    #[inline(always)]
    pub const fn aqhi_clock_control(
        self,
    ) -> crate::common::Reg<regs::AqhiClockControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "idle status register."]
    #[inline(always)]
    pub const fn aqhildle(self) -> crate::common::Reg<regs::Aqhildle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt acknoledge register."]
    #[inline(always)]
    pub const fn aqintr_acknowledge(
        self,
    ) -> crate::common::Reg<regs::AqintrAcknowledge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "interrupt enable register."]
    #[inline(always)]
    pub const fn aqintr_enbl(self) -> crate::common::Reg<regs::AqintrEnbl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "chip revison register."]
    #[inline(always)]
    pub const fn gcchip_rev(self) -> crate::common::Reg<regs::GcchipRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "chip date register."]
    #[inline(always)]
    pub const fn gcchip_date(self) -> crate::common::Reg<regs::GcchipDate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "chip patch revision register."]
    #[inline(always)]
    pub const fn gcreg_hichip_patch_rev(
        self,
    ) -> crate::common::Reg<regs::GcregHichipPatchRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "product identification register."]
    #[inline(always)]
    pub const fn gc_product_id(self) -> crate::common::Reg<regs::GcProductId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "module power control register."]
    #[inline(always)]
    pub const fn gc_module_power_controls(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerControls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "module power module control register."]
    #[inline(always)]
    pub const fn gc_module_power_module_control(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerModuleControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "module power module status register."]
    #[inline(always)]
    pub const fn gc_module_power_module_status(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerModuleStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "fetch engine page table base address register."]
    #[inline(always)]
    pub const fn aqmemory_fe_page_table(
        self,
    ) -> crate::common::Reg<regs::AqmemoryFePageTable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "memory debug register."]
    #[inline(always)]
    pub const fn aqmemory_debug(
        self,
    ) -> crate::common::Reg<regs::AqmemoryDebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "timing control register."]
    #[inline(always)]
    pub const fn aqregister_timing_control(
        self,
    ) -> crate::common::Reg<regs::AqregisterTimingControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[doc = "fetch command buffer base address register."]
    #[inline(always)]
    pub const fn gcreg_fetch_address(
        self,
    ) -> crate::common::Reg<regs::GcregFetchAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "fetch control register."]
    #[inline(always)]
    pub const fn gcreg_fetch_control(
        self,
    ) -> crate::common::Reg<regs::GcregFetchControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "current fetch command address register."]
    #[inline(always)]
    pub const fn gcreg_current_fetch_address(
        self,
    ) -> crate::common::Reg<regs::GcregCurrentFetchAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
}
pub mod regs {
    #[doc = "clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqhiClockControl(pub u32);
    impl AqhiClockControl {
        #[doc = "disable 2D/VG clock."]
        #[inline(always)]
        pub const fn clk2d_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable 2D/VG clock."]
        #[inline(always)]
        pub fn set_clk2d_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "core clock frequency scale value."]
        #[inline(always)]
        pub const fn fscale_val(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[doc = "core clock frequency scale value."]
        #[inline(always)]
        pub fn set_fscale_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
        #[doc = "core clock frequency scale value enable."]
        #[inline(always)]
        pub const fn fscale_cmd_load(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "core clock frequency scale value enable."]
        #[inline(always)]
        pub fn set_fscale_cmd_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disables clock gating for rams."]
        #[inline(always)]
        pub const fn disable_ram_clock_gating(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "disables clock gating for rams."]
        #[inline(always)]
        pub fn set_disable_ram_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "disable debug registers."]
        #[inline(always)]
        pub const fn disable_debug_registers(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disable debug registers."]
        #[inline(always)]
        pub fn set_disable_debug_registers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "soft reset the IP."]
        #[inline(always)]
        pub const fn soft_reset(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "soft reset the IP."]
        #[inline(always)]
        pub fn set_soft_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "disables ram power optimization."]
        #[inline(always)]
        pub const fn disable_ram_power_optimization(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "disables ram power optimization."]
        #[inline(always)]
        pub fn set_disable_ram_power_optimization(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "3D pipe is idle or not present."]
        #[inline(always)]
        pub const fn idle3_d(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "3D pipe is idle or not present."]
        #[inline(always)]
        pub fn set_idle3_d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "2D pipe is idle or not present."]
        #[inline(always)]
        pub const fn idle2_d(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "2D pipe is idle or not present."]
        #[inline(always)]
        pub fn set_idle2_d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "vg pipe is idle."]
        #[inline(always)]
        pub const fn idle_vg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "vg pipe is idle."]
        #[inline(always)]
        pub fn set_idle_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "isolate GPU bit, used for power on/off."]
        #[inline(always)]
        pub const fn isolate_gpu(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "isolate GPU bit, used for power on/off."]
        #[inline(always)]
        pub fn set_isolate_gpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for AqhiClockControl {
        #[inline(always)]
        fn default() -> AqhiClockControl {
            AqhiClockControl(0)
        }
    }
    #[doc = "idle status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aqhildle(pub u32);
    impl Aqhildle {
        #[doc = "0: fetch engine is busy 1:fetch engine is idle."]
        #[inline(always)]
        pub const fn idle_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0: fetch engine is busy 1:fetch engine is idle."]
        #[inline(always)]
        pub fn set_idle_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DE is dile or not present."]
        #[inline(always)]
        pub const fn idle_de(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DE is dile or not present."]
        #[inline(always)]
        pub fn set_idle_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Pixel engine is idle."]
        #[inline(always)]
        pub const fn idle_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel engine is idle."]
        #[inline(always)]
        pub fn set_idle_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SH is idle or not present."]
        #[inline(always)]
        pub const fn idle_sh(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SH is idle or not present."]
        #[inline(always)]
        pub fn set_idle_sh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA is idle or not present."]
        #[inline(always)]
        pub const fn idle_pa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA is idle or not present."]
        #[inline(always)]
        pub fn set_idle_pa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SE is idle or not present."]
        #[inline(always)]
        pub const fn idle_se(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SE is idle or not present."]
        #[inline(always)]
        pub fn set_idle_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RA is idle or not present."]
        #[inline(always)]
        pub const fn idle_ra(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RA is idle or not present."]
        #[inline(always)]
        pub fn set_idle_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TX is idle or not present."]
        #[inline(always)]
        pub const fn idle_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX is idle or not present."]
        #[inline(always)]
        pub fn set_idle_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Vector Graphics Engine is idle."]
        #[inline(always)]
        pub const fn idle_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Vector Graphics Engine is idle."]
        #[inline(always)]
        pub fn set_idle_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Image Engine is idle."]
        #[inline(always)]
        pub const fn idle_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Image Engine is idle."]
        #[inline(always)]
        pub fn set_idle_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FP is idle or not present."]
        #[inline(always)]
        pub const fn idle_fp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FP is idle or not present."]
        #[inline(always)]
        pub fn set_idle_fp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tessellation Engine is idle."]
        #[inline(always)]
        pub const fn idle_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tessellation Engine is idle."]
        #[inline(always)]
        pub fn set_idle_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BLT is idle or not present."]
        #[inline(always)]
        pub const fn idle_blt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BLT is idle or not present."]
        #[inline(always)]
        pub fn set_idle_blt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "axi is in low power mode."]
        #[inline(always)]
        pub const fn axi_lp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "axi is in low power mode."]
        #[inline(always)]
        pub fn set_axi_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Aqhildle {
        #[inline(always)]
        fn default() -> Aqhildle {
            Aqhildle(0)
        }
    }
    #[doc = "interrupt acknoledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqintrAcknowledge(pub u32);
    impl AqintrAcknowledge {
        #[doc = "for each interrupt event, 0=clear,1=interrupt active."]
        #[inline(always)]
        pub const fn intr_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "for each interrupt event, 0=clear,1=interrupt active."]
        #[inline(always)]
        pub fn set_intr_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AqintrAcknowledge {
        #[inline(always)]
        fn default() -> AqintrAcknowledge {
            AqintrAcknowledge(0)
        }
    }
    #[doc = "interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqintrEnbl(pub u32);
    impl AqintrEnbl {
        #[doc = "0=disable interrupt; 1=enable interrupt."]
        #[inline(always)]
        pub const fn intr_enbl_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "0=disable interrupt; 1=enable interrupt."]
        #[inline(always)]
        pub fn set_intr_enbl_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AqintrEnbl {
        #[inline(always)]
        fn default() -> AqintrEnbl {
            AqintrEnbl(0)
        }
    }
    #[doc = "memory debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqmemoryDebug(pub u32);
    impl AqmemoryDebug {
        #[doc = "limits the total number of outstanding read requests."]
        #[inline(always)]
        pub const fn max_outstanding_reads(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "limits the total number of outstanding read requests."]
        #[inline(always)]
        pub fn set_max_outstanding_reads(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "not relevant for vector graphics IP."]
        #[inline(always)]
        pub const fn zcomp_limit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "not relevant for vector graphics IP."]
        #[inline(always)]
        pub fn set_zcomp_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for AqmemoryDebug {
        #[inline(always)]
        fn default() -> AqmemoryDebug {
            AqmemoryDebug(0)
        }
    }
    #[doc = "fetch engine page table base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqmemoryFePageTable(pub u32);
    impl AqmemoryFePageTable {
        #[doc = "base address for the FE virtual address lookup table."]
        #[inline(always)]
        pub const fn base_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "base address for the FE virtual address lookup table."]
        #[inline(always)]
        pub fn set_base_address(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AqmemoryFePageTable {
        #[inline(always)]
        fn default() -> AqmemoryFePageTable {
            AqmemoryFePageTable(0)
        }
    }
    #[doc = "timing control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqregisterTimingControl(pub u32);
    impl AqregisterTimingControl {
        #[doc = "for 1 port ram."]
        #[inline(always)]
        pub const fn for_rf1p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "for 1 port ram."]
        #[inline(always)]
        pub fn set_for_rf1p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "for 2 port ram."]
        #[inline(always)]
        pub const fn for_rf2p(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "for 2 port ram."]
        #[inline(always)]
        pub fn set_for_rf2p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RTC for fast rams."]
        #[inline(always)]
        pub const fn fast_rtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "RTC for fast rams."]
        #[inline(always)]
        pub fn set_fast_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "WTC for fast rams."]
        #[inline(always)]
        pub const fn fast_wtc(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "WTC for fast rams."]
        #[inline(always)]
        pub fn set_fast_wtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "powerdown memory."]
        #[inline(always)]
        pub const fn power_down(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "powerdown memory."]
        #[inline(always)]
        pub fn set_power_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for AqregisterTimingControl {
        #[inline(always)]
        fn default() -> AqregisterTimingControl {
            AqregisterTimingControl(0)
        }
    }
    #[doc = "module power control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerControls(pub u32);
    impl GcModulePowerControls {
        #[doc = "enable module level clock gating."]
        #[inline(always)]
        pub const fn enable_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable module level clock gating."]
        #[inline(always)]
        pub fn set_enable_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disable module level clock gating for stall condition."]
        #[inline(always)]
        pub const fn disable_stall_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable module level clock gating for stall condition."]
        #[inline(always)]
        pub fn set_disable_stall_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "disable module level clock gating for starve/idle condition."]
        #[inline(always)]
        pub const fn disable_starve_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disable module level clock gating for starve/idle condition."]
        #[inline(always)]
        pub fn set_disable_starve_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "number of clock cycle gating the module if the modules is idle for this amout of clockk cycles."]
        #[inline(always)]
        pub const fn turn_on_counter(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "number of clock cycle gating the module if the modules is idle for this amout of clockk cycles."]
        #[inline(always)]
        pub fn set_turn_on_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "counter value for clock gating the module if the module is idle for this amout of clock cycles."]
        #[inline(always)]
        pub const fn turn_off_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value for clock gating the module if the module is idle for this amout of clock cycles."]
        #[inline(always)]
        pub fn set_turn_off_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GcModulePowerControls {
        #[inline(always)]
        fn default() -> GcModulePowerControls {
            GcModulePowerControls(0)
        }
    }
    #[doc = "module power module control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerModuleControl(pub u32);
    impl GcModulePowerModuleControl {
        #[doc = "disables module level clock gating for FE."]
        #[inline(always)]
        pub const fn disable_module_clock_gating_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for FE."]
        #[inline(always)]
        pub fn set_disable_module_clock_gating_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disables module level clock gating for PE."]
        #[inline(always)]
        pub const fn disable_module_clock_gating_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for PE."]
        #[inline(always)]
        pub fn set_disable_module_clock_gating_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "disables module lelvel clock gating for VG."]
        #[inline(always)]
        pub const fn disable_module_clock_gating_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "disables module lelvel clock gating for VG."]
        #[inline(always)]
        pub fn set_disable_module_clock_gating_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "disables module level clock gating for IM."]
        #[inline(always)]
        pub const fn disable_module_clock_gating_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for IM."]
        #[inline(always)]
        pub fn set_disable_module_clock_gating_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disables module level clock gating for TS."]
        #[inline(always)]
        pub const fn disable_module_clock_gating_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for TS."]
        #[inline(always)]
        pub fn set_disable_module_clock_gating_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "disables module level clock gating for flexa, not supported for all variants."]
        #[inline(always)]
        pub const fn disable_module_clockgating_flexa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for flexa, not supported for all variants."]
        #[inline(always)]
        pub fn set_disable_module_clockgating_flexa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for GcModulePowerModuleControl {
        #[inline(always)]
        fn default() -> GcModulePowerModuleControl {
            GcModulePowerModuleControl(0)
        }
    }
    #[doc = "module power module status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerModuleStatus(pub u32);
    impl GcModulePowerModuleStatus {
        #[doc = "module level clock gating is on for FE."]
        #[inline(always)]
        pub const fn module_clock_gated_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for FE."]
        #[inline(always)]
        pub fn set_module_clock_gated_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "module level clock gating is on for PE."]
        #[inline(always)]
        pub const fn module_clock_gated_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for PE."]
        #[inline(always)]
        pub fn set_module_clock_gated_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "module level clock gating is on for VG."]
        #[inline(always)]
        pub const fn module_clock_gated_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for VG."]
        #[inline(always)]
        pub fn set_module_clock_gated_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "module level clock gating is on for IM."]
        #[inline(always)]
        pub const fn module_clock_gated_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for IM."]
        #[inline(always)]
        pub fn set_module_clock_gated_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "module level ckock gating is on for ts."]
        #[inline(always)]
        pub const fn module_clock_gated_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "module level ckock gating is on for ts."]
        #[inline(always)]
        pub fn set_module_clock_gated_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "module level ckock gating is on for flexa."]
        #[inline(always)]
        pub const fn module_clock_gated_flexa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "module level ckock gating is on for flexa."]
        #[inline(always)]
        pub fn set_module_clock_gated_flexa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for GcModulePowerModuleStatus {
        #[inline(always)]
        fn default() -> GcModulePowerModuleStatus {
            GcModulePowerModuleStatus(0)
        }
    }
    #[doc = "product identification register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcProductId(pub u32);
    impl GcProductId {
        #[doc = "0:None_no extra letter on the product name for this core 1:nano 5:nano ultra."]
        #[inline(always)]
        pub const fn grade_level(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0:None_no extra letter on the product name for this core 1:nano 5:nano ultra."]
        #[inline(always)]
        pub fn set_grade_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "product number is 265."]
        #[inline(always)]
        pub const fn num(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "product number is 265."]
        #[inline(always)]
        pub fn set_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 4usize)) | (((val as u32) & 0x000f_ffff) << 4usize);
        }
        #[doc = "product type is 3:VG."]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "product type is 3:VG."]
        #[inline(always)]
        pub fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for GcProductId {
        #[inline(always)]
        fn default() -> GcProductId {
            GcProductId(0)
        }
    }
    #[doc = "chip date register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcchipDate(pub u32);
    impl GcchipDate {
        #[doc = "date."]
        #[inline(always)]
        pub const fn date(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "date."]
        #[inline(always)]
        pub fn set_date(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcchipDate {
        #[inline(always)]
        fn default() -> GcchipDate {
            GcchipDate(0)
        }
    }
    #[doc = "chip revison register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcchipRev(pub u32);
    impl GcchipRev {
        #[doc = "revision."]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "revision."]
        #[inline(always)]
        pub fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcchipRev {
        #[inline(always)]
        fn default() -> GcchipRev {
            GcchipRev(0)
        }
    }
    #[doc = "current fetch command address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregCurrentFetchAddress(pub u32);
    impl GcregCurrentFetchAddress {
        #[doc = "address."]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "address."]
        #[inline(always)]
        pub fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcregCurrentFetchAddress {
        #[inline(always)]
        fn default() -> GcregCurrentFetchAddress {
            GcregCurrentFetchAddress(0)
        }
    }
    #[doc = "fetch command buffer base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregFetchAddress(pub u32);
    impl GcregFetchAddress {
        #[doc = "0=system 2=vritual 1=local."]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "0=system 2=vritual 1=local."]
        #[inline(always)]
        pub fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "address of command buffer."]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "address of command buffer."]
        #[inline(always)]
        pub fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for GcregFetchAddress {
        #[inline(always)]
        fn default() -> GcregFetchAddress {
            GcregFetchAddress(0)
        }
    }
    #[doc = "fetch control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregFetchControl(pub u32);
    impl GcregFetchControl {
        #[doc = "number of 64bit words to fetch."]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "number of 64bit words to fetch."]
        #[inline(always)]
        pub fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
    }
    impl Default for GcregFetchControl {
        #[inline(always)]
        fn default() -> GcregFetchControl {
            GcregFetchControl(0)
        }
    }
    #[doc = "chip patch revision register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregHichipPatchRev(pub u32);
    impl GcregHichipPatchRev {
        #[doc = "patch revision."]
        #[inline(always)]
        pub const fn patch_rev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "patch revision."]
        #[inline(always)]
        pub fn set_patch_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GcregHichipPatchRev {
        #[inline(always)]
        fn default() -> GcregHichipPatchRev {
            GcregHichipPatchRev(0)
        }
    }
}
