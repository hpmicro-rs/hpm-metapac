#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DAO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dao {
    ptr: *mut u8,
}
unsafe impl Send for Dao {}
unsafe impl Sync for Dao {}
impl Dao {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn rx_cfgr(self) -> crate::common::Reg<regs::RxCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RX Slot Control Register."]
    #[inline(always)]
    pub const fn rxslt(self) -> crate::common::Reg<regs::Rxslt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "HPF A Coef Register."]
    #[inline(always)]
    pub const fn hpf_ma(self) -> crate::common::Reg<regs::HpfMa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "HPF B Coef Register."]
    #[inline(always)]
    pub const fn hpf_b(self) -> crate::common::Reg<regs::HpfB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
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
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Enable this module to run."]
        #[must_use]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this module to run."]
        #[inline(always)]
        pub const fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd")
                .field("run", &self.run())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ run: {=bool:?}, sftrst: {=bool:?} }}",
                self.run(),
                self.sftrst()
            )
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "the module continues to consume data, but all the pads are constant, thus no audio out."]
        #[must_use]
        #[inline(always)]
        pub const fn false_run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the module continues to consume data, but all the pads are constant, thus no audio out."]
        #[inline(always)]
        pub const fn set_false_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn false_level(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled."]
        #[inline(always)]
        pub const fn set_false_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "all the outputs are inverted before sending to pad."]
        #[must_use]
        #[inline(always)]
        pub const fn invert(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "all the outputs are inverted before sending to pad."]
        #[inline(always)]
        pub const fn set_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version."]
        #[must_use]
        #[inline(always)]
        pub const fn remap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version."]
        #[inline(always)]
        pub const fn set_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asserted to enable the left channel."]
        #[must_use]
        #[inline(always)]
        pub const fn left_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the left channel."]
        #[inline(always)]
        pub const fn set_left_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Asserted to enable the right channel."]
        #[must_use]
        #[inline(always)]
        pub const fn right_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the right channel."]
        #[inline(always)]
        pub const fn set_right_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Asserted to let the left and right channel output the same value."]
        #[must_use]
        #[inline(always)]
        pub const fn mono(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to let the left and right channel output the same value."]
        #[inline(always)]
        pub const fn set_mono(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Whether HPF is enabled. This HPF is used to filter out the DC part."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Whether HPF is enabled. This HPF is used to filter out the DC part."]
        #[inline(always)]
        pub const fn set_hpf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("false_run", &self.false_run())
                .field("false_level", &self.false_level())
                .field("invert", &self.invert())
                .field("remap", &self.remap())
                .field("left_en", &self.left_en())
                .field("right_en", &self.right_en())
                .field("mono", &self.mono())
                .field("hpf_en", &self.hpf_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ false_run: {=bool:?}, false_level: {=u8:?}, invert: {=bool:?}, remap: {=bool:?}, left_en: {=bool:?}, right_en: {=bool:?}, mono: {=bool:?}, hpf_en: {=bool:?} }}" , self . false_run () , self . false_level () , self . invert () , self . remap () , self . left_en () , self . right_en () , self . mono () , self . hpf_en ())
        }
    }
    #[doc = "HPF B Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfB(pub u32);
    impl HpfB {
        #[doc = "coef B of the Order-1 HPF."]
        #[must_use]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub const fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfB {
        #[inline(always)]
        fn default() -> HpfB {
            HpfB(0)
        }
    }
    impl core::fmt::Debug for HpfB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HpfB").field("coef", &self.coef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HpfB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HpfB {{ coef: {=u32:?} }}", self.coef())
        }
    }
    #[doc = "HPF A Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfMa(pub u32);
    impl HpfMa {
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[must_use]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub const fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfMa {
        #[inline(always)]
        fn default() -> HpfMa {
            HpfMa(0)
        }
    }
    impl core::fmt::Debug for HpfMa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HpfMa").field("coef", &self.coef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HpfMa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HpfMa {{ coef: {=u32:?} }}", self.coef())
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxCfgr(pub u32);
    impl RxCfgr {
        #[doc = "CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_max(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc."]
        #[inline(always)]
        pub const fn set_ch_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
    }
    impl Default for RxCfgr {
        #[inline(always)]
        fn default() -> RxCfgr {
            RxCfgr(0)
        }
    }
    impl core::fmt::Debug for RxCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxCfgr")
                .field("ch_max", &self.ch_max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxCfgr {{ ch_max: {=u8:?} }}", self.ch_max())
        }
    }
    #[doc = "RX Slot Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxslt(pub u32);
    impl Rxslt {
        #[doc = "Slot enable for the channels."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Slot enable for the channels."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxslt {
        #[inline(always)]
        fn default() -> Rxslt {
            Rxslt(0)
        }
    }
    impl core::fmt::Debug for Rxslt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxslt").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxslt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxslt {{ en: {=u32:?} }}", self.en())
        }
    }
}
