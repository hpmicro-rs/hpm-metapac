#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BCFG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcfg {
    ptr: *mut u8,
}
unsafe impl Send for Bcfg {}
unsafe impl Sync for Bcfg {}
impl Bcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bandgap config."]
    #[inline(always)]
    pub const fn vbg_cfg(self) -> crate::common::Reg<regs::VbgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "On-chip 32k oscillator config."]
    #[inline(always)]
    pub const fn irc32k_cfg(self) -> crate::common::Reg<regs::Irc32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "XTAL 32K config."]
    #[inline(always)]
    pub const fn xtal32k_cfg(self) -> crate::common::Reg<regs::Xtal32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Clock config."]
    #[inline(always)]
    pub const fn clk_cfg(self) -> crate::common::Reg<regs::ClkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
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
    #[doc = "Clock config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfg(pub u32);
    impl ClkCfg {
        #[doc = "force switch to crystal."]
        #[must_use]
        #[inline(always)]
        pub const fn force_xtal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force switch to crystal."]
        #[inline(always)]
        pub const fn set_force_xtal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force irc32k run."]
        #[must_use]
        #[inline(always)]
        pub const fn keep_irc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force irc32k run."]
        #[inline(always)]
        pub const fn set_keep_irc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "crystal selected."]
        #[must_use]
        #[inline(always)]
        pub const fn xtal_sel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "crystal selected."]
        #[inline(always)]
        pub const fn set_xtal_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for ClkCfg {
        #[inline(always)]
        fn default() -> ClkCfg {
            ClkCfg(0)
        }
    }
    impl core::fmt::Debug for ClkCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkCfg")
                .field("force_xtal", &self.force_xtal())
                .field("keep_irc", &self.keep_irc())
                .field("xtal_sel", &self.xtal_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClkCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClkCfg {{ force_xtal: {=bool:?}, keep_irc: {=bool:?}, xtal_sel: {=bool:?} }}",
                self.force_xtal(),
                self.keep_irc(),
                self.xtal_sel()
            )
        }
    }
    #[doc = "On-chip 32k oscillator config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irc32kCfg(pub u32);
    impl Irc32kCfg {
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
    impl Default for Irc32kCfg {
        #[inline(always)]
        fn default() -> Irc32kCfg {
            Irc32kCfg(0)
        }
    }
    impl core::fmt::Debug for Irc32kCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irc32kCfg")
                .field("cap_trim", &self.cap_trim())
                .field("capex6_trim", &self.capex6_trim())
                .field("capex7_trim", &self.capex7_trim())
                .field("irc_trimmed", &self.irc_trimmed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irc32kCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Irc32kCfg {{ cap_trim: {=u16:?}, capex6_trim: {=bool:?}, capex7_trim: {=bool:?}, irc_trimmed: {=bool:?} }}" , self . cap_trim () , self . capex6_trim () , self . capex7_trim () , self . irc_trimmed ())
        }
    }
    #[doc = "Bandgap config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbgCfg(pub u32);
    impl VbgCfg {
        #[doc = "Bandgap 0.50V output trim."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_p50(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 0.50V output trim."]
        #[inline(always)]
        pub const fn set_vbg_p50(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Bandgap 0.65V output trim."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_p65(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 0.65V output trim."]
        #[inline(always)]
        pub const fn set_vbg_p65(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Bandgap 1.0V output trim."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_1p0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 1.0V output trim."]
        #[inline(always)]
        pub const fn set_vbg_1p0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode."]
        #[must_use]
        #[inline(always)]
        pub const fn power_save(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode."]
        #[inline(always)]
        pub const fn set_power_save(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[inline(always)]
        pub const fn set_vbg_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for VbgCfg {
        #[inline(always)]
        fn default() -> VbgCfg {
            VbgCfg(0)
        }
    }
    impl core::fmt::Debug for VbgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VbgCfg")
                .field("vbg_p50", &self.vbg_p50())
                .field("vbg_p65", &self.vbg_p65())
                .field("vbg_1p0", &self.vbg_1p0())
                .field("power_save", &self.power_save())
                .field("vbg_trimmed", &self.vbg_trimmed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VbgCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VbgCfg {{ vbg_p50: {=u8:?}, vbg_p65: {=u8:?}, vbg_1p0: {=u8:?}, power_save: {=bool:?}, vbg_trimmed: {=bool:?} }}" , self . vbg_p50 () , self . vbg_p65 () , self . vbg_1p0 () , self . power_save () , self . vbg_trimmed ())
        }
    }
    #[doc = "XTAL 32K config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32kCfg(pub u32);
    impl Xtal32kCfg {
        #[doc = "crystal 32k amplifier."]
        #[must_use]
        #[inline(always)]
        pub const fn amp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "crystal 32k amplifier."]
        #[inline(always)]
        pub const fn set_amp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "crystal 32k config."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "crystal 32k config."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "crystal 32k gm selection."]
        #[must_use]
        #[inline(always)]
        pub const fn gmsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "crystal 32k gm selection."]
        #[inline(always)]
        pub const fn set_gmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "crystal 32k hysteres enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "crystal 32k hysteres enable."]
        #[inline(always)]
        pub const fn set_hyst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Xtal32kCfg {
        #[inline(always)]
        fn default() -> Xtal32kCfg {
            Xtal32kCfg(0)
        }
    }
    impl core::fmt::Debug for Xtal32kCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal32kCfg")
                .field("amp", &self.amp())
                .field("cfg", &self.cfg())
                .field("gmsel", &self.gmsel())
                .field("hyst_en", &self.hyst_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xtal32kCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xtal32kCfg {{ amp: {=u8:?}, cfg: {=bool:?}, gmsel: {=u8:?}, hyst_en: {=bool:?} }}",
                self.amp(),
                self.cfg(),
                self.gmsel(),
                self.hyst_en()
            )
        }
    }
}
