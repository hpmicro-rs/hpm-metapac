#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PDGO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdgo {
    ptr: *mut u8,
}
unsafe impl Send for Pdgo {}
unsafe impl Sync for Pdgo {}
impl Pdgo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "trunoff control."]
    #[inline(always)]
    pub const fn dgo_turnoff(self) -> crate::common::Reg<regs::DgoTurnoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RC32K CLOCK."]
    #[inline(always)]
    pub const fn dgo_rc32k_cfg(self) -> crate::common::Reg<regs::DgoRc32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Generic control 0."]
    #[inline(always)]
    pub const fn dgo_gpr00(self) -> crate::common::Reg<regs::DgoGpr00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Generic control 1."]
    #[inline(always)]
    pub const fn dgo_gpr01(self) -> crate::common::Reg<regs::DgoGpr01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Generic control 2."]
    #[inline(always)]
    pub const fn dgo_gpr02(self) -> crate::common::Reg<regs::DgoGpr02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "Generic control 3."]
    #[inline(always)]
    pub const fn dgo_gpr03(self) -> crate::common::Reg<regs::DgoGpr03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "control register 0."]
    #[inline(always)]
    pub const fn dgo_ctr0(self) -> crate::common::Reg<regs::DgoCtr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn dgo_ctr1(self) -> crate::common::Reg<regs::DgoCtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn dgo_ctr2(self) -> crate::common::Reg<regs::DgoCtr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "control register 3."]
    #[inline(always)]
    pub const fn dgo_ctr3(self) -> crate::common::Reg<regs::DgoCtr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "control register 4."]
    #[inline(always)]
    pub const fn dgo_ctr4(self) -> crate::common::Reg<regs::DgoCtr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
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
    #[doc = "control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr0(pub u32);
    impl DgoCtr0 {
        #[doc = "dgo register status retenion."]
        #[must_use]
        #[inline(always)]
        pub const fn retention(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "dgo register status retenion."]
        #[inline(always)]
        pub const fn set_retention(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for DgoCtr0 {
        #[inline(always)]
        fn default() -> DgoCtr0 {
            DgoCtr0(0)
        }
    }
    impl core::fmt::Debug for DgoCtr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoCtr0")
                .field("retention", &self.retention())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoCtr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoCtr0 {{ retention: {=bool:?} }}", self.retention())
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr1(pub u32);
    impl DgoCtr1 {
        #[doc = "wakeup pin status."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_wakeup_status(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup pin status."]
        #[inline(always)]
        pub const fn set_pin_wakeup_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "permit wakeup pin or software wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "permit wakeup pin or software wakeup."]
        #[inline(always)]
        pub const fn set_wakeup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "software wakeup： 0 : wakeup once； 1：auto wakeup Continuously."]
        #[must_use]
        #[inline(always)]
        pub const fn aoto_sys_wakeup(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software wakeup： 0 : wakeup once； 1：auto wakeup Continuously."]
        #[inline(always)]
        pub const fn set_aoto_sys_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DgoCtr1 {
        #[inline(always)]
        fn default() -> DgoCtr1 {
            DgoCtr1(0)
        }
    }
    impl core::fmt::Debug for DgoCtr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoCtr1")
                .field("pin_wakeup_status", &self.pin_wakeup_status())
                .field("wakeup_en", &self.wakeup_en())
                .field("aoto_sys_wakeup", &self.aoto_sys_wakeup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoCtr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DgoCtr1 {{ pin_wakeup_status: {=bool:?}, wakeup_en: {=bool:?}, aoto_sys_wakeup: {=bool:?} }}" , self . pin_wakeup_status () , self . wakeup_en () , self . aoto_sys_wakeup ())
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr2(pub u32);
    impl DgoCtr2 {
        #[doc = "wakeup pin pull down disable."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup_pulldn_disable(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup pin pull down disable."]
        #[inline(always)]
        pub const fn set_wakeup_pulldn_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "resetn pin pull up disable."]
        #[must_use]
        #[inline(always)]
        pub const fn resetn_pullup_disable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "resetn pin pull up disable."]
        #[inline(always)]
        pub const fn set_resetn_pullup_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for DgoCtr2 {
        #[inline(always)]
        fn default() -> DgoCtr2 {
            DgoCtr2(0)
        }
    }
    impl core::fmt::Debug for DgoCtr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoCtr2")
                .field("wakeup_pulldn_disable", &self.wakeup_pulldn_disable())
                .field("resetn_pullup_disable", &self.resetn_pullup_disable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoCtr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DgoCtr2 {{ wakeup_pulldn_disable: {=bool:?}, resetn_pullup_disable: {=bool:?} }}",
                self.wakeup_pulldn_disable(),
                self.resetn_pullup_disable()
            )
        }
    }
    #[doc = "control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr3(pub u32);
    impl DgoCtr3 {
        #[doc = "software wakeup counter."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup_counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software wakeup counter."]
        #[inline(always)]
        pub const fn set_wakeup_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoCtr3 {
        #[inline(always)]
        fn default() -> DgoCtr3 {
            DgoCtr3(0)
        }
    }
    impl core::fmt::Debug for DgoCtr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoCtr3")
                .field("wakeup_counter", &self.wakeup_counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoCtr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DgoCtr3 {{ wakeup_counter: {=u32:?} }}",
                self.wakeup_counter()
            )
        }
    }
    #[doc = "control register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr4(pub u32);
    impl DgoCtr4 {
        #[doc = "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bandgap_lp_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode."]
        #[inline(always)]
        pub const fn set_bandgap_lp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bandgap_less_power(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode."]
        #[inline(always)]
        pub const fn set_bandgap_less_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DgoCtr4 {
        #[inline(always)]
        fn default() -> DgoCtr4 {
            DgoCtr4(0)
        }
    }
    impl core::fmt::Debug for DgoCtr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoCtr4")
                .field("bandgap_lp_mode", &self.bandgap_lp_mode())
                .field("bandgap_less_power", &self.bandgap_less_power())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoCtr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DgoCtr4 {{ bandgap_lp_mode: {=bool:?}, bandgap_less_power: {=bool:?} }}",
                self.bandgap_lp_mode(),
                self.bandgap_less_power()
            )
        }
    }
    #[doc = "Generic control 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr00(pub u32);
    impl DgoGpr00 {
        #[doc = "Generic control."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr00 {
        #[inline(always)]
        fn default() -> DgoGpr00 {
            DgoGpr00(0)
        }
    }
    impl core::fmt::Debug for DgoGpr00 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoGpr00")
                .field("gpr", &self.gpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoGpr00 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoGpr00 {{ gpr: {=u32:?} }}", self.gpr())
        }
    }
    #[doc = "Generic control 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr01(pub u32);
    impl DgoGpr01 {
        #[doc = "Generic control."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr01 {
        #[inline(always)]
        fn default() -> DgoGpr01 {
            DgoGpr01(0)
        }
    }
    impl core::fmt::Debug for DgoGpr01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoGpr01")
                .field("gpr", &self.gpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoGpr01 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoGpr01 {{ gpr: {=u32:?} }}", self.gpr())
        }
    }
    #[doc = "Generic control 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr02(pub u32);
    impl DgoGpr02 {
        #[doc = "Generic control."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr02 {
        #[inline(always)]
        fn default() -> DgoGpr02 {
            DgoGpr02(0)
        }
    }
    impl core::fmt::Debug for DgoGpr02 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoGpr02")
                .field("gpr", &self.gpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoGpr02 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoGpr02 {{ gpr: {=u32:?} }}", self.gpr())
        }
    }
    #[doc = "Generic control 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr03(pub u32);
    impl DgoGpr03 {
        #[doc = "Generic control."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr03 {
        #[inline(always)]
        fn default() -> DgoGpr03 {
            DgoGpr03(0)
        }
    }
    impl core::fmt::Debug for DgoGpr03 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoGpr03")
                .field("gpr", &self.gpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoGpr03 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoGpr03 {{ gpr: {=u32:?} }}", self.gpr())
        }
    }
    #[doc = "RC32K CLOCK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoRc32kCfg(pub u32);
    impl DgoRc32kCfg {
        #[doc = "capacitor trim bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cap_trim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "capacitor trim bits."]
        #[inline(always)]
        pub const fn set_cap_trim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "IRC32K bit 6."]
        #[must_use]
        #[inline(always)]
        pub const fn capex6_trim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K bit 6."]
        #[inline(always)]
        pub const fn set_capex6_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IRC32K bit 7."]
        #[must_use]
        #[inline(always)]
        pub const fn capex7_trim(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K bit 7."]
        #[inline(always)]
        pub const fn set_capex7_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed."]
        #[must_use]
        #[inline(always)]
        pub const fn irc_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed."]
        #[inline(always)]
        pub const fn set_irc_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DgoRc32kCfg {
        #[inline(always)]
        fn default() -> DgoRc32kCfg {
            DgoRc32kCfg(0)
        }
    }
    impl core::fmt::Debug for DgoRc32kCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoRc32kCfg")
                .field("cap_trim", &self.cap_trim())
                .field("capex6_trim", &self.capex6_trim())
                .field("capex7_trim", &self.capex7_trim())
                .field("irc_trimmed", &self.irc_trimmed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoRc32kCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DgoRc32kCfg {{ cap_trim: {=u16:?}, capex6_trim: {=bool:?}, capex7_trim: {=bool:?}, irc_trimmed: {=bool:?} }}" , self . cap_trim () , self . capex6_trim () , self . capex7_trim () , self . irc_trimmed ())
        }
    }
    #[doc = "trunoff control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoTurnoff(pub u32);
    impl DgoTurnoff {
        #[doc = "trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoTurnoff {
        #[inline(always)]
        fn default() -> DgoTurnoff {
            DgoTurnoff(0)
        }
    }
    impl core::fmt::Debug for DgoTurnoff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DgoTurnoff")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DgoTurnoff {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DgoTurnoff {{ counter: {=u32:?} }}", self.counter())
        }
    }
}
