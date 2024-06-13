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
        unsafe { PtpcPtpc::from_ptr(self.ptr.add(0x0usize + n * 4096usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn time_sel(self) -> crate::common::Reg<regs::TimeSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_sts(self) -> crate::common::Reg<regs::IntSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ptpc_can_ts_sel(
        self,
    ) -> crate::common::Reg<regs::PtpcCanTsSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3000usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control Register 1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "timestamp high."]
    #[inline(always)]
    pub const fn timeh(self) -> crate::common::Reg<regs::Timeh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "timestamp low."]
    #[inline(always)]
    pub const fn timel(self) -> crate::common::Reg<regs::Timel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "timestamp update high."]
    #[inline(always)]
    pub const fn ts_updth(self) -> crate::common::Reg<regs::TsUpdth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "timestamp update low."]
    #[inline(always)]
    pub const fn ts_updtl(self) -> crate::common::Reg<regs::TsUpdtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn addend(self) -> crate::common::Reg<regs::Addend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tarh(self) -> crate::common::Reg<regs::Tarh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tarl(self) -> crate::common::Reg<regs::Tarl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn pps_ctrl(self) -> crate::common::Reg<regs::PpsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn capt_snaph(self) -> crate::common::Reg<regs::CaptSnaph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn capt_snapl(self) -> crate::common::Reg<regs::CaptSnapl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addend(pub u32);
    impl Addend {
        #[doc = "used in fine update mode only."]
        #[inline(always)]
        pub const fn addend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "used in fine update mode only."]
        #[inline(always)]
        pub fn set_addend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addend {
        #[inline(always)]
        fn default() -> Addend {
            Addend(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CaptSnaph(pub u32);
    impl CaptSnaph {
        #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8."]
        #[inline(always)]
        pub const fn capt_snap_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8."]
        #[inline(always)]
        pub fn set_capt_snap_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CaptSnaph {
        #[inline(always)]
        fn default() -> CaptSnaph {
            CaptSnaph(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CaptSnapl(pub u32);
    impl CaptSnapl {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capt_snap_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capt_snap_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CaptSnapl {
        #[inline(always)]
        fn default() -> CaptSnapl {
            CaptSnapl(0)
        }
    }
    #[doc = "Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn timer_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_timer_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: coarse update, ns counter add ss_incr\\[7:0\\]
each clk 1: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow."]
        #[inline(always)]
        pub const fn fine_coarse_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: coarse update, ns counter add ss_incr\\[7:0\\]
each clk 1: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow."]
        #[inline(always)]
        pub fn set_fine_coarse_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "initial timer with ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub const fn init_timer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "initial timer with ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub fn set_init_timer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "update timer with +/- ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub const fn update_timer(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "update timer with +/- ts_updt, pulse, clear after set."]
        #[inline(always)]
        pub fn set_update_timer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "set to enable compare, will be cleared by HW when compare event triggered."]
        #[inline(always)]
        pub const fn comp_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable compare, will be cleared by HW when compare event triggered."]
        #[inline(always)]
        pub fn set_comp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capt_snap_neg_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capt_snap_neg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "set will use posege of input capture signal to latch timestamp value."]
        #[inline(always)]
        pub const fn capt_snap_pos_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set will use posege of input capture signal to latch timestamp value."]
        #[inline(always)]
        pub fn set_capt_snap_pos_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event."]
        #[inline(always)]
        pub const fn capt_snap_keep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event."]
        #[inline(always)]
        pub fn set_capt_snap_keep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF."]
        #[inline(always)]
        pub const fn subsec_digital_rollover(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF."]
        #[inline(always)]
        pub fn set_subsec_digital_rollover(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    #[doc = "Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20."]
        #[inline(always)]
        pub const fn ss_incr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20."]
        #[inline(always)]
        pub fn set_ss_incr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pps_int_sts0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pps_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capture_int_sts0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capture_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn comp_int_sts0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_comp_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pps_int_sts1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pps_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capture_int_sts1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capture_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn comp_int_sts1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_comp_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSts(pub u32);
    impl IntSts {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pps_int_sts0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pps_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capture_int_sts0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capture_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn comp_int_sts0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_comp_int_sts0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pps_int_sts1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pps_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn capture_int_sts1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_capture_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn comp_int_sts1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_comp_int_sts1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for IntSts {
        #[inline(always)]
        fn default() -> IntSts {
            IntSts(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PpsCtrl(pub u32);
    impl PpsCtrl {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pps_ctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pps_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PpsCtrl {
        #[inline(always)]
        fn default() -> PpsCtrl {
            PpsCtrl(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpcCanTsSel(pub u32);
    impl PtpcCanTsSel {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn tsu_tbin0_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_tsu_tbin0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn tsu_tbin1_sel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_tsu_tbin1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 14usize)) | (((val as u32) & 0x3f) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn tsu_tbin2_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_tsu_tbin2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn tsu_tbin3_sel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_tsu_tbin3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for PtpcCanTsSel {
        #[inline(always)]
        fn default() -> PtpcCanTsSel {
            PtpcCanTsSel(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tarh(pub u32);
    impl Tarh {
        #[doc = "used for generate compare signal if enabled."]
        #[inline(always)]
        pub const fn target_time_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "used for generate compare signal if enabled."]
        #[inline(always)]
        pub fn set_target_time_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tarh {
        #[inline(always)]
        fn default() -> Tarh {
            Tarh(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tarl(pub u32);
    impl Tarl {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn target_time_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_target_time_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tarl {
        #[inline(always)]
        fn default() -> Tarl {
            Tarl(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimeSel(pub u32);
    impl TimeSel {
        #[doc = "set to use ptpc1 for canx clr to use ptpc0 for canx."]
        #[inline(always)]
        pub const fn can0_time_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to use ptpc1 for canx clr to use ptpc0 for canx."]
        #[inline(always)]
        pub fn set_can0_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn can1_time_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_can1_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn can2_time_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_can2_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn can3_time_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_can3_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for TimeSel {
        #[inline(always)]
        fn default() -> TimeSel {
            TimeSel(0)
        }
    }
    #[doc = "timestamp high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timeh(pub u32);
    impl Timeh {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn timestamp_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_timestamp_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timeh {
        #[inline(always)]
        fn default() -> Timeh {
            Timeh(0)
        }
    }
    #[doc = "timestamp low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timel(pub u32);
    impl Timel {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn timestamp_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_timestamp_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timel {
        #[inline(always)]
        fn default() -> Timel {
            Timel(0)
        }
    }
    #[doc = "timestamp update high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsUpdth(pub u32);
    impl TsUpdth {
        #[doc = "together with ts_updtl, used to initial or update timestamp."]
        #[inline(always)]
        pub const fn sec_update(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "together with ts_updtl, used to initial or update timestamp."]
        #[inline(always)]
        pub fn set_sec_update(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsUpdth {
        #[inline(always)]
        fn default() -> TsUpdth {
            TsUpdth(0)
        }
    }
    #[doc = "timestamp update low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsUpdtl(pub u32);
    impl TsUpdtl {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ns_update(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ns_update(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "1 for sub; 0 for add, used only at update."]
        #[inline(always)]
        pub const fn add_sub(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 for sub; 0 for add, used only at update."]
        #[inline(always)]
        pub fn set_add_sub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsUpdtl {
        #[inline(always)]
        fn default() -> TsUpdtl {
            TsUpdtl(0)
        }
    }
}
