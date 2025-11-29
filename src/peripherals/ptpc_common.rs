#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PTPC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptpc {
    ptr: *mut u8,
}
unsafe impl Send for Ptpc {}
unsafe impl Sync for Ptpc {}
impl Ptpc {
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
    pub const fn ptpc(self, n: usize) -> PtpcPtpc {
        assert!(n < 2usize);
        unsafe { PtpcPtpc::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4096usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn time_sel(self) -> crate::common::Reg<regs::TimeSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_sts(self) -> crate::common::Reg<regs::IntSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ptpc_can_ts_sel(
        self,
    ) -> crate::common::Reg<regs::PtpcCanTsSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PtpcPtpc {
    ptr: *mut u8,
}
unsafe impl Send for PtpcPtpc {}
unsafe impl Sync for PtpcPtpc {}
impl PtpcPtpc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register 0."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register 1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "timestamp high."]
    #[inline(always)]
    pub const fn timeh(self) -> crate::common::Reg<regs::Timeh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "timestamp low."]
    #[inline(always)]
    pub const fn timel(self) -> crate::common::Reg<regs::Timel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "timestamp update high."]
    #[inline(always)]
    pub const fn ts_updth(self) -> crate::common::Reg<regs::TsUpdth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "timestamp update low."]
    #[inline(always)]
    pub const fn ts_updtl(self) -> crate::common::Reg<regs::TsUpdtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn addend(self) -> crate::common::Reg<regs::Addend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tarh(self) -> crate::common::Reg<regs::Tarh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tarl(self) -> crate::common::Reg<regs::Tarl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn pps_ctrl(self) -> crate::common::Reg<regs::PpsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn capt_snaph(self) -> crate::common::Reg<regs::CaptSnaph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn capt_snapl(self) -> crate::common::Reg<regs::CaptSnapl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
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
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addend(pub u32);
    impl Addend {
        #[doc = "used in fine update mode only."]
        #[must_use]
        #[inline(always)]
        pub const fn addend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "used in fine update mode only."]
        #[inline(always)]
        pub const fn set_addend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addend {
        #[inline(always)]
        fn default() -> Addend {
            Addend(0)
        }
    }
    impl core::fmt::Debug for Addend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addend")
                .field("addend", &self.addend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addend {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addend {{ addend: {=u32:?} }}", self.addend())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CaptSnaph(pub u32);
    impl CaptSnaph {
        #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_snap_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8."]
        #[inline(always)]
        pub const fn set_capt_snap_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CaptSnaph {
        #[inline(always)]
        fn default() -> CaptSnaph {
            CaptSnaph(0)
        }
    }
    impl core::fmt::Debug for CaptSnaph {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CaptSnaph")
                .field("capt_snap_high", &self.capt_snap_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CaptSnaph {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CaptSnaph {{ capt_snap_high: {=u32:?} }}",
                self.capt_snap_high()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CaptSnapl(pub u32);
    impl CaptSnapl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_snap_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capt_snap_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CaptSnapl {
        #[inline(always)]
        fn default() -> CaptSnapl {
            CaptSnapl(0)
        }
    }
    impl core::fmt::Debug for CaptSnapl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CaptSnapl")
                .field("capt_snap_low", &self.capt_snap_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CaptSnapl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CaptSnapl {{ capt_snap_low: {=u32:?} }}",
                self.capt_snap_low()
            )
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
        pub const fn timer_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timer_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: coarse update, ns counter add ss_incr\\[7:0\\]
each clk 1: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn fine_coarse_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: coarse update, ns counter add ss_incr\\[7:0\\]
each clk 1: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow."]
        #[inline(always)]
        pub const fn set_fine_coarse_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "initial timer with ts_updt, pulse, clear after set."]
        #[must_use]
        #[inline(always)]
        pub const fn init_timer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "initial timer with ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub const fn set_init_timer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "update timer with +/- ts_updt, pulse, clear after set."]
        #[must_use]
        #[inline(always)]
        pub const fn update_timer(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "update timer with +/- ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub const fn set_update_timer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "set to enable compare, will be cleared by HW when compare event triggered."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable compare, will be cleared by HW when compare event triggered."]
        #[inline(always)]
        pub const fn set_comp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_snap_neg_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capt_snap_neg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "set will use posege of input capture signal to latch timestamp value."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_snap_pos_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set will use posege of input capture signal to latch timestamp value."]
        #[inline(always)]
        pub const fn set_capt_snap_pos_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_snap_keep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event."]
        #[inline(always)]
        pub const fn set_capt_snap_keep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF."]
        #[must_use]
        #[inline(always)]
        pub const fn subsec_digital_rollover(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF."]
        #[inline(always)]
        pub const fn set_subsec_digital_rollover(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("timer_enable", &self.timer_enable())
                .field("fine_coarse_sel", &self.fine_coarse_sel())
                .field("init_timer", &self.init_timer())
                .field("update_timer", &self.update_timer())
                .field("comp_en", &self.comp_en())
                .field("capt_snap_neg_en", &self.capt_snap_neg_en())
                .field("capt_snap_pos_en", &self.capt_snap_pos_en())
                .field("capt_snap_keep", &self.capt_snap_keep())
                .field("subsec_digital_rollover", &self.subsec_digital_rollover())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl0 {{ timer_enable: {=bool:?}, fine_coarse_sel: {=bool:?}, init_timer: {=bool:?}, update_timer: {=bool:?}, comp_en: {=bool:?}, capt_snap_neg_en: {=bool:?}, capt_snap_pos_en: {=bool:?}, capt_snap_keep: {=bool:?}, subsec_digital_rollover: {=bool:?} }}" , self . timer_enable () , self . fine_coarse_sel () , self . init_timer () , self . update_timer () , self . comp_en () , self . capt_snap_neg_en () , self . capt_snap_pos_en () , self . capt_snap_keep () , self . subsec_digital_rollover ())
        }
    }
    #[doc = "Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_incr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20."]
        #[inline(always)]
        pub const fn set_ss_incr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    impl core::fmt::Debug for Ctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl1")
                .field("ss_incr", &self.ss_incr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctrl1 {{ ss_incr: {=u8:?} }}", self.ss_incr())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_int_sts0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pps_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capture_int_sts0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capture_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_int_sts0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_comp_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_int_sts1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pps_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capture_int_sts1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capture_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_int_sts1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_comp_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
                .field("pps_int_sts0", &self.pps_int_sts0())
                .field("capture_int_sts0", &self.capture_int_sts0())
                .field("comp_int_sts0", &self.comp_int_sts0())
                .field("pps_int_sts1", &self.pps_int_sts1())
                .field("capture_int_sts1", &self.capture_int_sts1())
                .field("comp_int_sts1", &self.comp_int_sts1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntEn {{ pps_int_sts0: {=bool:?}, capture_int_sts0: {=bool:?}, comp_int_sts0: {=bool:?}, pps_int_sts1: {=bool:?}, capture_int_sts1: {=bool:?}, comp_int_sts1: {=bool:?} }}" , self . pps_int_sts0 () , self . capture_int_sts0 () , self . comp_int_sts0 () , self . pps_int_sts1 () , self . capture_int_sts1 () , self . comp_int_sts1 ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSts(pub u32);
    impl IntSts {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_int_sts0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pps_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capture_int_sts0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capture_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_int_sts0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_comp_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_int_sts1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pps_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn capture_int_sts1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_capture_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_int_sts1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_comp_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntSts {
        #[inline(always)]
        fn default() -> IntSts {
            IntSts(0)
        }
    }
    impl core::fmt::Debug for IntSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntSts")
                .field("pps_int_sts0", &self.pps_int_sts0())
                .field("capture_int_sts0", &self.capture_int_sts0())
                .field("comp_int_sts0", &self.comp_int_sts0())
                .field("pps_int_sts1", &self.pps_int_sts1())
                .field("capture_int_sts1", &self.capture_int_sts1())
                .field("comp_int_sts1", &self.comp_int_sts1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntSts {{ pps_int_sts0: {=bool:?}, capture_int_sts0: {=bool:?}, comp_int_sts0: {=bool:?}, pps_int_sts1: {=bool:?}, capture_int_sts1: {=bool:?}, comp_int_sts1: {=bool:?} }}" , self . pps_int_sts0 () , self . capture_int_sts0 () , self . comp_int_sts0 () , self . pps_int_sts1 () , self . capture_int_sts1 () , self . comp_int_sts1 ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsCtrl(pub u32);
    impl PpsCtrl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_ctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pps_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
                .field("pps_ctrl", &self.pps_ctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PpsCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PpsCtrl {{ pps_ctrl: {=u8:?} }}", self.pps_ctrl())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpcCanTsSel(pub u32);
    impl PtpcCanTsSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tsu_tbin0_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tsu_tbin0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tsu_tbin1_sel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tsu_tbin1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 14usize)) | (((val as u32) & 0x3f) << 14usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tsu_tbin2_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tsu_tbin2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tsu_tbin3_sel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tsu_tbin3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for PtpcCanTsSel {
        #[inline(always)]
        fn default() -> PtpcCanTsSel {
            PtpcCanTsSel(0)
        }
    }
    impl core::fmt::Debug for PtpcCanTsSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpcCanTsSel")
                .field("tsu_tbin0_sel", &self.tsu_tbin0_sel())
                .field("tsu_tbin1_sel", &self.tsu_tbin1_sel())
                .field("tsu_tbin2_sel", &self.tsu_tbin2_sel())
                .field("tsu_tbin3_sel", &self.tsu_tbin3_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpcCanTsSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PtpcCanTsSel {{ tsu_tbin0_sel: {=u8:?}, tsu_tbin1_sel: {=u8:?}, tsu_tbin2_sel: {=u8:?}, tsu_tbin3_sel: {=u8:?} }}" , self . tsu_tbin0_sel () , self . tsu_tbin1_sel () , self . tsu_tbin2_sel () , self . tsu_tbin3_sel ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tarh(pub u32);
    impl Tarh {
        #[doc = "used for generate compare signal if enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "used for generate compare signal if enabled."]
        #[inline(always)]
        pub const fn set_target_time_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tarh {
        #[inline(always)]
        fn default() -> Tarh {
            Tarh(0)
        }
    }
    impl core::fmt::Debug for Tarh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tarh")
                .field("target_time_high", &self.target_time_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tarh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tarh {{ target_time_high: {=u32:?} }}",
                self.target_time_high()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tarl(pub u32);
    impl Tarl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_target_time_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tarl {
        #[inline(always)]
        fn default() -> Tarl {
            Tarl(0)
        }
    }
    impl core::fmt::Debug for Tarl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tarl")
                .field("target_time_low", &self.target_time_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tarl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tarl {{ target_time_low: {=u32:?} }}",
                self.target_time_low()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimeSel(pub u32);
    impl TimeSel {
        #[doc = "set to use ptpc1 for canx clr to use ptpc0 for canx."]
        #[must_use]
        #[inline(always)]
        pub const fn can0_time_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to use ptpc1 for canx clr to use ptpc0 for canx."]
        #[inline(always)]
        pub const fn set_can0_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn can1_time_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_can1_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn can2_time_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_can2_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn can3_time_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_can3_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for TimeSel {
        #[inline(always)]
        fn default() -> TimeSel {
            TimeSel(0)
        }
    }
    impl core::fmt::Debug for TimeSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimeSel")
                .field("can0_time_sel", &self.can0_time_sel())
                .field("can1_time_sel", &self.can1_time_sel())
                .field("can2_time_sel", &self.can2_time_sel())
                .field("can3_time_sel", &self.can3_time_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimeSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TimeSel {{ can0_time_sel: {=bool:?}, can1_time_sel: {=bool:?}, can2_time_sel: {=bool:?}, can3_time_sel: {=bool:?} }}" , self . can0_time_sel () , self . can1_time_sel () , self . can2_time_sel () , self . can3_time_sel ())
        }
    }
    #[doc = "timestamp high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timeh(pub u32);
    impl Timeh {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timestamp_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timeh {
        #[inline(always)]
        fn default() -> Timeh {
            Timeh(0)
        }
    }
    impl core::fmt::Debug for Timeh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timeh")
                .field("timestamp_high", &self.timestamp_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timeh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timeh {{ timestamp_high: {=u32:?} }}",
                self.timestamp_high()
            )
        }
    }
    #[doc = "timestamp low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timel(pub u32);
    impl Timel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timestamp_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timel {
        #[inline(always)]
        fn default() -> Timel {
            Timel(0)
        }
    }
    impl core::fmt::Debug for Timel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timel")
                .field("timestamp_low", &self.timestamp_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timel {{ timestamp_low: {=u32:?} }}",
                self.timestamp_low()
            )
        }
    }
    #[doc = "timestamp update high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsUpdth(pub u32);
    impl TsUpdth {
        #[doc = "together with ts_updtl, used to initial or update timestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn sec_update(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "together with ts_updtl, used to initial or update timestamp."]
        #[inline(always)]
        pub const fn set_sec_update(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsUpdth {
        #[inline(always)]
        fn default() -> TsUpdth {
            TsUpdth(0)
        }
    }
    impl core::fmt::Debug for TsUpdth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsUpdth")
                .field("sec_update", &self.sec_update())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsUpdth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsUpdth {{ sec_update: {=u32:?} }}", self.sec_update())
        }
    }
    #[doc = "timestamp update low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsUpdtl(pub u32);
    impl TsUpdtl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ns_update(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ns_update(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "1 for sub; 0 for add, used only at update."]
        #[must_use]
        #[inline(always)]
        pub const fn add_sub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 for sub; 0 for add, used only at update."]
        #[inline(always)]
        pub const fn set_add_sub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsUpdtl {
        #[inline(always)]
        fn default() -> TsUpdtl {
            TsUpdtl(0)
        }
    }
    impl core::fmt::Debug for TsUpdtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsUpdtl")
                .field("ns_update", &self.ns_update())
                .field("add_sub", &self.add_sub())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsUpdtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsUpdtl {{ ns_update: {=u32:?}, add_sub: {=bool:?} }}",
                self.ns_update(),
                self.add_sub()
            )
        }
    }
}
