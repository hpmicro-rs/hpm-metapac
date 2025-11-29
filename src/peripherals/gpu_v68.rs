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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "idle status register."]
    #[inline(always)]
    pub const fn aqhildle(self) -> crate::common::Reg<regs::Aqhildle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "interrupt acknoledge register."]
    #[inline(always)]
    pub const fn aqintr_acknowledge(
        self,
    ) -> crate::common::Reg<regs::AqintrAcknowledge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "interrupt enable register."]
    #[inline(always)]
    pub const fn aqintr_enbl(self) -> crate::common::Reg<regs::AqintrEnbl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "chip revison register."]
    #[inline(always)]
    pub const fn gcchip_rev(self) -> crate::common::Reg<regs::GcchipRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "chip date register."]
    #[inline(always)]
    pub const fn gcchip_date(self) -> crate::common::Reg<regs::GcchipDate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "chip patch revision register."]
    #[inline(always)]
    pub const fn gcreg_hichip_patch_rev(
        self,
    ) -> crate::common::Reg<regs::GcregHichipPatchRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "product identification register."]
    #[inline(always)]
    pub const fn gc_product_id(self) -> crate::common::Reg<regs::GcProductId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "module power control register."]
    #[inline(always)]
    pub const fn gc_module_power_controls(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerControls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "module power module control register."]
    #[inline(always)]
    pub const fn gc_module_power_module_control(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerModuleControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "module power module status register."]
    #[inline(always)]
    pub const fn gc_module_power_module_status(
        self,
    ) -> crate::common::Reg<regs::GcModulePowerModuleStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "fetch engine page table base address register."]
    #[inline(always)]
    pub const fn aqmemory_fe_page_table(
        self,
    ) -> crate::common::Reg<regs::AqmemoryFePageTable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "memory debug register."]
    #[inline(always)]
    pub const fn aqmemory_debug(
        self,
    ) -> crate::common::Reg<regs::AqmemoryDebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "timing control register."]
    #[inline(always)]
    pub const fn aqregister_timing_control(
        self,
    ) -> crate::common::Reg<regs::AqregisterTimingControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "fetch command buffer base address register."]
    #[inline(always)]
    pub const fn gcreg_fetch_address(
        self,
    ) -> crate::common::Reg<regs::GcregFetchAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "fetch control register."]
    #[inline(always)]
    pub const fn gcreg_fetch_control(
        self,
    ) -> crate::common::Reg<regs::GcregFetchControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "current fetch command address register."]
    #[inline(always)]
    pub const fn gcreg_current_fetch_address(
        self,
    ) -> crate::common::Reg<regs::GcregCurrentFetchAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
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
    #[doc = "clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqhiClockControl(pub u32);
    impl AqhiClockControl {
        #[doc = "disable 2D/VG clock."]
        #[must_use]
        #[inline(always)]
        pub const fn clk2d_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable 2D/VG clock."]
        #[inline(always)]
        pub const fn set_clk2d_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "core clock frequency scale value."]
        #[must_use]
        #[inline(always)]
        pub const fn fscale_val(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[doc = "core clock frequency scale value."]
        #[inline(always)]
        pub const fn set_fscale_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
        #[doc = "core clock frequency scale value enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fscale_cmd_load(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "core clock frequency scale value enable."]
        #[inline(always)]
        pub const fn set_fscale_cmd_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disables clock gating for rams."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_ram_clock_gating(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "disables clock gating for rams."]
        #[inline(always)]
        pub const fn set_disable_ram_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "disable debug registers."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_debug_registers(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disable debug registers."]
        #[inline(always)]
        pub const fn set_disable_debug_registers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "soft reset the IP."]
        #[must_use]
        #[inline(always)]
        pub const fn soft_reset(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "soft reset the IP."]
        #[inline(always)]
        pub const fn set_soft_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "disables ram power optimization."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_ram_power_optimization(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "disables ram power optimization."]
        #[inline(always)]
        pub const fn set_disable_ram_power_optimization(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "3D pipe is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle3_d(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "3D pipe is idle or not present."]
        #[inline(always)]
        pub const fn set_idle3_d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "2D pipe is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle2_d(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "2D pipe is idle or not present."]
        #[inline(always)]
        pub const fn set_idle2_d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "vg pipe is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_vg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "vg pipe is idle."]
        #[inline(always)]
        pub const fn set_idle_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "isolate GPU bit, used for power on/off."]
        #[must_use]
        #[inline(always)]
        pub const fn isolate_gpu(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "isolate GPU bit, used for power on/off."]
        #[inline(always)]
        pub const fn set_isolate_gpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for AqhiClockControl {
        #[inline(always)]
        fn default() -> AqhiClockControl {
            AqhiClockControl(0)
        }
    }
    impl core::fmt::Debug for AqhiClockControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqhiClockControl")
                .field("clk2d_dis", &self.clk2d_dis())
                .field("fscale_val", &self.fscale_val())
                .field("fscale_cmd_load", &self.fscale_cmd_load())
                .field("disable_ram_clock_gating", &self.disable_ram_clock_gating())
                .field("disable_debug_registers", &self.disable_debug_registers())
                .field("soft_reset", &self.soft_reset())
                .field(
                    "disable_ram_power_optimization",
                    &self.disable_ram_power_optimization(),
                )
                .field("idle3_d", &self.idle3_d())
                .field("idle2_d", &self.idle2_d())
                .field("idle_vg", &self.idle_vg())
                .field("isolate_gpu", &self.isolate_gpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqhiClockControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AqhiClockControl {{ clk2d_dis: {=bool:?}, fscale_val: {=u8:?}, fscale_cmd_load: {=bool:?}, disable_ram_clock_gating: {=bool:?}, disable_debug_registers: {=bool:?}, soft_reset: {=bool:?}, disable_ram_power_optimization: {=bool:?}, idle3_d: {=bool:?}, idle2_d: {=bool:?}, idle_vg: {=bool:?}, isolate_gpu: {=bool:?} }}" , self . clk2d_dis () , self . fscale_val () , self . fscale_cmd_load () , self . disable_ram_clock_gating () , self . disable_debug_registers () , self . soft_reset () , self . disable_ram_power_optimization () , self . idle3_d () , self . idle2_d () , self . idle_vg () , self . isolate_gpu ())
        }
    }
    #[doc = "idle status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aqhildle(pub u32);
    impl Aqhildle {
        #[doc = "0: fetch engine is busy 1:fetch engine is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0: fetch engine is busy 1:fetch engine is idle."]
        #[inline(always)]
        pub const fn set_idle_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DE is dile or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_de(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DE is dile or not present."]
        #[inline(always)]
        pub const fn set_idle_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Pixel engine is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Pixel engine is idle."]
        #[inline(always)]
        pub const fn set_idle_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SH is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_sh(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SH is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_sh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_pa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_pa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SE is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_se(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SE is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_se(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RA is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_ra(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RA is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TX is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Vector Graphics Engine is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Vector Graphics Engine is idle."]
        #[inline(always)]
        pub const fn set_idle_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Image Engine is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Image Engine is idle."]
        #[inline(always)]
        pub const fn set_idle_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FP is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_fp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FP is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_fp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tessellation Engine is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tessellation Engine is idle."]
        #[inline(always)]
        pub const fn set_idle_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BLT is idle or not present."]
        #[must_use]
        #[inline(always)]
        pub const fn idle_blt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BLT is idle or not present."]
        #[inline(always)]
        pub const fn set_idle_blt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "axi is in low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_lp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "axi is in low power mode."]
        #[inline(always)]
        pub const fn set_axi_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Aqhildle {
        #[inline(always)]
        fn default() -> Aqhildle {
            Aqhildle(0)
        }
    }
    impl core::fmt::Debug for Aqhildle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aqhildle")
                .field("idle_fe", &self.idle_fe())
                .field("idle_de", &self.idle_de())
                .field("idle_pe", &self.idle_pe())
                .field("idle_sh", &self.idle_sh())
                .field("idle_pa", &self.idle_pa())
                .field("idle_se", &self.idle_se())
                .field("idle_ra", &self.idle_ra())
                .field("idle_tx", &self.idle_tx())
                .field("idle_vg", &self.idle_vg())
                .field("idle_im", &self.idle_im())
                .field("idle_fp", &self.idle_fp())
                .field("idle_ts", &self.idle_ts())
                .field("idle_blt", &self.idle_blt())
                .field("axi_lp", &self.axi_lp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aqhildle {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Aqhildle {{ idle_fe: {=bool:?}, idle_de: {=bool:?}, idle_pe: {=bool:?}, idle_sh: {=bool:?}, idle_pa: {=bool:?}, idle_se: {=bool:?}, idle_ra: {=bool:?}, idle_tx: {=bool:?}, idle_vg: {=bool:?}, idle_im: {=bool:?}, idle_fp: {=bool:?}, idle_ts: {=bool:?}, idle_blt: {=bool:?}, axi_lp: {=bool:?} }}" , self . idle_fe () , self . idle_de () , self . idle_pe () , self . idle_sh () , self . idle_pa () , self . idle_se () , self . idle_ra () , self . idle_tx () , self . idle_vg () , self . idle_im () , self . idle_fp () , self . idle_ts () , self . idle_blt () , self . axi_lp ())
        }
    }
    #[doc = "interrupt acknoledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqintrAcknowledge(pub u32);
    impl AqintrAcknowledge {
        #[doc = "for each interrupt event, 0=clear,1=interrupt active."]
        #[must_use]
        #[inline(always)]
        pub const fn intr_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "for each interrupt event, 0=clear,1=interrupt active."]
        #[inline(always)]
        pub const fn set_intr_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AqintrAcknowledge {
        #[inline(always)]
        fn default() -> AqintrAcknowledge {
            AqintrAcknowledge(0)
        }
    }
    impl core::fmt::Debug for AqintrAcknowledge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqintrAcknowledge")
                .field("intr_vec", &self.intr_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqintrAcknowledge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AqintrAcknowledge {{ intr_vec: {=u32:?} }}",
                self.intr_vec()
            )
        }
    }
    #[doc = "interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqintrEnbl(pub u32);
    impl AqintrEnbl {
        #[doc = "0=disable interrupt; 1=enable interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn intr_enbl_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "0=disable interrupt; 1=enable interrupt."]
        #[inline(always)]
        pub const fn set_intr_enbl_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AqintrEnbl {
        #[inline(always)]
        fn default() -> AqintrEnbl {
            AqintrEnbl(0)
        }
    }
    impl core::fmt::Debug for AqintrEnbl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqintrEnbl")
                .field("intr_enbl_vec", &self.intr_enbl_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqintrEnbl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AqintrEnbl {{ intr_enbl_vec: {=u32:?} }}",
                self.intr_enbl_vec()
            )
        }
    }
    #[doc = "memory debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqmemoryDebug(pub u32);
    impl AqmemoryDebug {
        #[doc = "limits the total number of outstanding read requests."]
        #[must_use]
        #[inline(always)]
        pub const fn max_outstanding_reads(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "limits the total number of outstanding read requests."]
        #[inline(always)]
        pub const fn set_max_outstanding_reads(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "not relevant for vector graphics IP."]
        #[must_use]
        #[inline(always)]
        pub const fn zcomp_limit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "not relevant for vector graphics IP."]
        #[inline(always)]
        pub const fn set_zcomp_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for AqmemoryDebug {
        #[inline(always)]
        fn default() -> AqmemoryDebug {
            AqmemoryDebug(0)
        }
    }
    impl core::fmt::Debug for AqmemoryDebug {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqmemoryDebug")
                .field("max_outstanding_reads", &self.max_outstanding_reads())
                .field("zcomp_limit", &self.zcomp_limit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqmemoryDebug {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AqmemoryDebug {{ max_outstanding_reads: {=u8:?}, zcomp_limit: {=u8:?} }}",
                self.max_outstanding_reads(),
                self.zcomp_limit()
            )
        }
    }
    #[doc = "fetch engine page table base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqmemoryFePageTable(pub u32);
    impl AqmemoryFePageTable {
        #[doc = "base address for the FE virtual address lookup table."]
        #[must_use]
        #[inline(always)]
        pub const fn base_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "base address for the FE virtual address lookup table."]
        #[inline(always)]
        pub const fn set_base_address(&mut self, val: u32) {
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
    impl core::fmt::Debug for AqmemoryFePageTable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqmemoryFePageTable")
                .field("base_address", &self.base_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqmemoryFePageTable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AqmemoryFePageTable {{ base_address: {=u32:?} }}",
                self.base_address()
            )
        }
    }
    #[doc = "timing control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AqregisterTimingControl(pub u32);
    impl AqregisterTimingControl {
        #[doc = "for 1 port ram."]
        #[must_use]
        #[inline(always)]
        pub const fn for_rf1p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "for 1 port ram."]
        #[inline(always)]
        pub const fn set_for_rf1p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "for 2 port ram."]
        #[must_use]
        #[inline(always)]
        pub const fn for_rf2p(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "for 2 port ram."]
        #[inline(always)]
        pub const fn set_for_rf2p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RTC for fast rams."]
        #[must_use]
        #[inline(always)]
        pub const fn fast_rtc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "RTC for fast rams."]
        #[inline(always)]
        pub const fn set_fast_rtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "WTC for fast rams."]
        #[must_use]
        #[inline(always)]
        pub const fn fast_wtc(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "WTC for fast rams."]
        #[inline(always)]
        pub const fn set_fast_wtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "powerdown memory."]
        #[must_use]
        #[inline(always)]
        pub const fn power_down(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "powerdown memory."]
        #[inline(always)]
        pub const fn set_power_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for AqregisterTimingControl {
        #[inline(always)]
        fn default() -> AqregisterTimingControl {
            AqregisterTimingControl(0)
        }
    }
    impl core::fmt::Debug for AqregisterTimingControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AqregisterTimingControl")
                .field("for_rf1p", &self.for_rf1p())
                .field("for_rf2p", &self.for_rf2p())
                .field("fast_rtc", &self.fast_rtc())
                .field("fast_wtc", &self.fast_wtc())
                .field("power_down", &self.power_down())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AqregisterTimingControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AqregisterTimingControl {{ for_rf1p: {=u8:?}, for_rf2p: {=u8:?}, fast_rtc: {=u8:?}, fast_wtc: {=u8:?}, power_down: {=bool:?} }}" , self . for_rf1p () , self . for_rf2p () , self . fast_rtc () , self . fast_wtc () , self . power_down ())
        }
    }
    #[doc = "module power control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerControls(pub u32);
    impl GcModulePowerControls {
        #[doc = "enable module level clock gating."]
        #[must_use]
        #[inline(always)]
        pub const fn enable_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable module level clock gating."]
        #[inline(always)]
        pub const fn set_enable_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disable module level clock gating for stall condition."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_stall_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable module level clock gating for stall condition."]
        #[inline(always)]
        pub const fn set_disable_stall_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "disable module level clock gating for starve/idle condition."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_starve_module_clock_gating(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disable module level clock gating for starve/idle condition."]
        #[inline(always)]
        pub const fn set_disable_starve_module_clock_gating(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "number of clock cycle gating the module if the modules is idle for this amout of clockk cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn turn_on_counter(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "number of clock cycle gating the module if the modules is idle for this amout of clockk cycles."]
        #[inline(always)]
        pub const fn set_turn_on_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "counter value for clock gating the module if the module is idle for this amout of clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn turn_off_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value for clock gating the module if the module is idle for this amout of clock cycles."]
        #[inline(always)]
        pub const fn set_turn_off_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GcModulePowerControls {
        #[inline(always)]
        fn default() -> GcModulePowerControls {
            GcModulePowerControls(0)
        }
    }
    impl core::fmt::Debug for GcModulePowerControls {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcModulePowerControls")
                .field(
                    "enable_module_clock_gating",
                    &self.enable_module_clock_gating(),
                )
                .field(
                    "disable_stall_module_clock_gating",
                    &self.disable_stall_module_clock_gating(),
                )
                .field(
                    "disable_starve_module_clock_gating",
                    &self.disable_starve_module_clock_gating(),
                )
                .field("turn_on_counter", &self.turn_on_counter())
                .field("turn_off_counter", &self.turn_off_counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcModulePowerControls {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GcModulePowerControls {{ enable_module_clock_gating: {=bool:?}, disable_stall_module_clock_gating: {=bool:?}, disable_starve_module_clock_gating: {=bool:?}, turn_on_counter: {=u8:?}, turn_off_counter: {=u16:?} }}" , self . enable_module_clock_gating () , self . disable_stall_module_clock_gating () , self . disable_starve_module_clock_gating () , self . turn_on_counter () , self . turn_off_counter ())
        }
    }
    #[doc = "module power module control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerModuleControl(pub u32);
    impl GcModulePowerModuleControl {
        #[doc = "disables module level clock gating for FE."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clock_gating_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for FE."]
        #[inline(always)]
        pub const fn set_disable_module_clock_gating_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disables module level clock gating for PE."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clock_gating_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for PE."]
        #[inline(always)]
        pub const fn set_disable_module_clock_gating_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "disables module lelvel clock gating for VG."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clock_gating_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "disables module lelvel clock gating for VG."]
        #[inline(always)]
        pub const fn set_disable_module_clock_gating_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "disables module level clock gating for IM."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clock_gating_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for IM."]
        #[inline(always)]
        pub const fn set_disable_module_clock_gating_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disables module level clock gating for TS."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clock_gating_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for TS."]
        #[inline(always)]
        pub const fn set_disable_module_clock_gating_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "disables module level clock gating for flexa, not supported for all variants."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_module_clockgating_flexa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "disables module level clock gating for flexa, not supported for all variants."]
        #[inline(always)]
        pub const fn set_disable_module_clockgating_flexa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for GcModulePowerModuleControl {
        #[inline(always)]
        fn default() -> GcModulePowerModuleControl {
            GcModulePowerModuleControl(0)
        }
    }
    impl core::fmt::Debug for GcModulePowerModuleControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcModulePowerModuleControl")
                .field(
                    "disable_module_clock_gating_fe",
                    &self.disable_module_clock_gating_fe(),
                )
                .field(
                    "disable_module_clock_gating_pe",
                    &self.disable_module_clock_gating_pe(),
                )
                .field(
                    "disable_module_clock_gating_vg",
                    &self.disable_module_clock_gating_vg(),
                )
                .field(
                    "disable_module_clock_gating_im",
                    &self.disable_module_clock_gating_im(),
                )
                .field(
                    "disable_module_clock_gating_ts",
                    &self.disable_module_clock_gating_ts(),
                )
                .field(
                    "disable_module_clockgating_flexa",
                    &self.disable_module_clockgating_flexa(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcModulePowerModuleControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GcModulePowerModuleControl {{ disable_module_clock_gating_fe: {=bool:?}, disable_module_clock_gating_pe: {=bool:?}, disable_module_clock_gating_vg: {=bool:?}, disable_module_clock_gating_im: {=bool:?}, disable_module_clock_gating_ts: {=bool:?}, disable_module_clockgating_flexa: {=bool:?} }}" , self . disable_module_clock_gating_fe () , self . disable_module_clock_gating_pe () , self . disable_module_clock_gating_vg () , self . disable_module_clock_gating_im () , self . disable_module_clock_gating_ts () , self . disable_module_clockgating_flexa ())
        }
    }
    #[doc = "module power module status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcModulePowerModuleStatus(pub u32);
    impl GcModulePowerModuleStatus {
        #[doc = "module level clock gating is on for FE."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_fe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for FE."]
        #[inline(always)]
        pub const fn set_module_clock_gated_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "module level clock gating is on for PE."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for PE."]
        #[inline(always)]
        pub const fn set_module_clock_gated_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "module level clock gating is on for VG."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_vg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for VG."]
        #[inline(always)]
        pub const fn set_module_clock_gated_vg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "module level clock gating is on for IM."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "module level clock gating is on for IM."]
        #[inline(always)]
        pub const fn set_module_clock_gated_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "module level ckock gating is on for ts."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_ts(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "module level ckock gating is on for ts."]
        #[inline(always)]
        pub const fn set_module_clock_gated_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "module level ckock gating is on for flexa."]
        #[must_use]
        #[inline(always)]
        pub const fn module_clock_gated_flexa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "module level ckock gating is on for flexa."]
        #[inline(always)]
        pub const fn set_module_clock_gated_flexa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for GcModulePowerModuleStatus {
        #[inline(always)]
        fn default() -> GcModulePowerModuleStatus {
            GcModulePowerModuleStatus(0)
        }
    }
    impl core::fmt::Debug for GcModulePowerModuleStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcModulePowerModuleStatus")
                .field("module_clock_gated_fe", &self.module_clock_gated_fe())
                .field("module_clock_gated_pe", &self.module_clock_gated_pe())
                .field("module_clock_gated_vg", &self.module_clock_gated_vg())
                .field("module_clock_gated_im", &self.module_clock_gated_im())
                .field("module_clock_gated_ts", &self.module_clock_gated_ts())
                .field("module_clock_gated_flexa", &self.module_clock_gated_flexa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcModulePowerModuleStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GcModulePowerModuleStatus {{ module_clock_gated_fe: {=bool:?}, module_clock_gated_pe: {=bool:?}, module_clock_gated_vg: {=bool:?}, module_clock_gated_im: {=bool:?}, module_clock_gated_ts: {=bool:?}, module_clock_gated_flexa: {=bool:?} }}" , self . module_clock_gated_fe () , self . module_clock_gated_pe () , self . module_clock_gated_vg () , self . module_clock_gated_im () , self . module_clock_gated_ts () , self . module_clock_gated_flexa ())
        }
    }
    #[doc = "product identification register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcProductId(pub u32);
    impl GcProductId {
        #[doc = "0:None_no extra letter on the product name for this core 1:nano 5:nano ultra."]
        #[must_use]
        #[inline(always)]
        pub const fn grade_level(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0:None_no extra letter on the product name for this core 1:nano 5:nano ultra."]
        #[inline(always)]
        pub const fn set_grade_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "product number is 265."]
        #[must_use]
        #[inline(always)]
        pub const fn num(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "product number is 265."]
        #[inline(always)]
        pub const fn set_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 4usize)) | (((val as u32) & 0x000f_ffff) << 4usize);
        }
        #[doc = "product type is 3:VG."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "product type is 3:VG."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for GcProductId {
        #[inline(always)]
        fn default() -> GcProductId {
            GcProductId(0)
        }
    }
    impl core::fmt::Debug for GcProductId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcProductId")
                .field("grade_level", &self.grade_level())
                .field("num", &self.num())
                .field("type_", &self.type_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcProductId {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GcProductId {{ grade_level: {=u8:?}, num: {=u32:?}, type_: {=u8:?} }}",
                self.grade_level(),
                self.num(),
                self.type_()
            )
        }
    }
    #[doc = "chip date register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcchipDate(pub u32);
    impl GcchipDate {
        #[doc = "date."]
        #[must_use]
        #[inline(always)]
        pub const fn date(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "date."]
        #[inline(always)]
        pub const fn set_date(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcchipDate {
        #[inline(always)]
        fn default() -> GcchipDate {
            GcchipDate(0)
        }
    }
    impl core::fmt::Debug for GcchipDate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcchipDate")
                .field("date", &self.date())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcchipDate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GcchipDate {{ date: {=u32:?} }}", self.date())
        }
    }
    #[doc = "chip revison register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcchipRev(pub u32);
    impl GcchipRev {
        #[doc = "revision."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "revision."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcchipRev {
        #[inline(always)]
        fn default() -> GcchipRev {
            GcchipRev(0)
        }
    }
    impl core::fmt::Debug for GcchipRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcchipRev")
                .field("rev", &self.rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcchipRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GcchipRev {{ rev: {=u32:?} }}", self.rev())
        }
    }
    #[doc = "current fetch command address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregCurrentFetchAddress(pub u32);
    impl GcregCurrentFetchAddress {
        #[doc = "address."]
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "address."]
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GcregCurrentFetchAddress {
        #[inline(always)]
        fn default() -> GcregCurrentFetchAddress {
            GcregCurrentFetchAddress(0)
        }
    }
    impl core::fmt::Debug for GcregCurrentFetchAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcregCurrentFetchAddress")
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcregCurrentFetchAddress {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GcregCurrentFetchAddress {{ address: {=u32:?} }}",
                self.address()
            )
        }
    }
    #[doc = "fetch command buffer base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregFetchAddress(pub u32);
    impl GcregFetchAddress {
        #[doc = "0=system 2=vritual 1=local."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "0=system 2=vritual 1=local."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "address of command buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "address of command buffer."]
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for GcregFetchAddress {
        #[inline(always)]
        fn default() -> GcregFetchAddress {
            GcregFetchAddress(0)
        }
    }
    impl core::fmt::Debug for GcregFetchAddress {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcregFetchAddress")
                .field("type_", &self.type_())
                .field("address", &self.address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcregFetchAddress {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GcregFetchAddress {{ type_: {=u8:?}, address: {=u32:?} }}",
                self.type_(),
                self.address()
            )
        }
    }
    #[doc = "fetch control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregFetchControl(pub u32);
    impl GcregFetchControl {
        #[doc = "number of 64bit words to fetch."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "number of 64bit words to fetch."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
    }
    impl Default for GcregFetchControl {
        #[inline(always)]
        fn default() -> GcregFetchControl {
            GcregFetchControl(0)
        }
    }
    impl core::fmt::Debug for GcregFetchControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcregFetchControl")
                .field("count", &self.count())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcregFetchControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GcregFetchControl {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "chip patch revision register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GcregHichipPatchRev(pub u32);
    impl GcregHichipPatchRev {
        #[doc = "patch revision."]
        #[must_use]
        #[inline(always)]
        pub const fn patch_rev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "patch revision."]
        #[inline(always)]
        pub const fn set_patch_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for GcregHichipPatchRev {
        #[inline(always)]
        fn default() -> GcregHichipPatchRev {
            GcregHichipPatchRev(0)
        }
    }
    impl core::fmt::Debug for GcregHichipPatchRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GcregHichipPatchRev")
                .field("patch_rev", &self.patch_rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GcregHichipPatchRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GcregHichipPatchRev {{ patch_rev: {=u8:?} }}",
                self.patch_rev()
            )
        }
    }
}
