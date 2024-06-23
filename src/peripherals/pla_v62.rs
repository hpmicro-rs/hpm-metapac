#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chn {
    ptr: *mut u8,
}
unsafe impl Send for Chn {}
unsafe impl Sync for Chn {}
impl Chn {
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
    pub const fn aoi_16to8(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Aoi16to8, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "CHN&index0 AOI_16to8_00_01 OR logic cfg."]
    #[inline(always)]
    pub const fn aoi_8to7_00_01(self) -> crate::common::Reg<regs::Aoi8to70001, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "CHN&index0 AOI_16to8_02_03 OR logic cfg."]
    #[inline(always)]
    pub const fn aoi_8to7_02_03(self) -> crate::common::Reg<regs::Aoi8to70203, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "CHN&index0 AOI_16to8_04_05 OR logic cfg."]
    #[inline(always)]
    pub const fn aoi_8to7_04_05(self) -> crate::common::Reg<regs::Aoi8to70405, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "CHN&index0 AOI_16to8_06 OR logic cfg."]
    #[inline(always)]
    pub const fn aoi_8to7_06(self) -> crate::common::Reg<regs::Aoi8to706, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn filter_2nd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Filter2nd, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn filter_3rd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Filter3rd, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "CHN&index0 cfg ff."]
    #[inline(always)]
    pub const fn cfg_ff(self) -> crate::common::Reg<regs::CfgFf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
}
#[doc = "PLA0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pla {
    ptr: *mut u8,
}
unsafe impl Send for Pla {}
unsafe impl Sync for Pla {}
impl Pla {
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
    pub const fn chn(self, n: usize) -> Chn {
        assert!(n < 8usize);
        unsafe { Chn::from_ptr(self.ptr.add(0x0usize + n * 112usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn filter_1st_pla_in(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Filter1stPlaIn, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn filter_1st_pla_out(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Filter1stPlaOut, crate::common::RW> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn chn_cfg_active(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChnCfgActive, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aoi16to8(pub u32);
    impl Aoi16to8 {
        #[doc = "select value for AOI_16to8_0. 0: 0. 1: 1st_filter_out\\[0\\]. 2: ~1st_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_0. 0: 0. 1: 1st_filter_out\\[0\\]. 2: ~1st_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "select value for AOI_16to8_1. 0: 0. 1: 1st_filter_out\\[1\\]. 2: ~1st_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_1. 0: 0. 1: 1st_filter_out\\[1\\]. 2: ~1st_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "select value for AOI_16to8_2. 0: 0. 1: 1st_filter_out\\[2\\]. 2: ~1st_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_2. 0: 0. 1: 1st_filter_out\\[2\\]. 2: ~1st_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "select value for AOI_16to8_3. 0: 0. 1: 1st_filter_out\\[3\\]. 2: ~1st_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_3. 0: 0. 1: 1st_filter_out\\[3\\]. 2: ~1st_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "select value for AOI_16to8_4. 0: 0. 1: 1st_filter_out\\[4\\]. 2: ~1st_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_4. 0: 0. 1: 1st_filter_out\\[4\\]. 2: ~1st_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "select value for AOI_16to8_5. 0: 0. 1: 1st_filter_out\\[5\\]. 2: ~1st_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_5. 0: 0. 1: 1st_filter_out\\[5\\]. 2: ~1st_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "select value for AOI_16to8_6. 0: 0. 1: 1st_filter_out\\[6\\]. 2: ~1st_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_6. 0: 0. 1: 1st_filter_out\\[6\\]. 2: ~1st_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "select value for AOI_16to8_7. 0: 0. 1: 1st_filter_out\\[7\\]. 2: ~1st_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_7. 0: 0. 1: 1st_filter_out\\[7\\]. 2: ~1st_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "select value for AOI_16to8_8. 0: 0. 1: 1st_filter_out\\[8\\]. 2: ~1st_filter_out\\[8\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_8. 0: 0. 1: 1st_filter_out\\[8\\]. 2: ~1st_filter_out\\[8\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "select value for AOI_16to8_9. 0: 0. 1: 1st_filter_out\\[9\\]. 2: ~1st_filter_out\\[9\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_9(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_9. 0: 0. 1: 1st_filter_out\\[9\\]. 2: ~1st_filter_out\\[9\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "select value for AOI_16to8_10. 0: 0. 1: 1st_filter_out\\[10\\]. 2: ~1st_filter_out\\[10\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_10(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_10. 0: 0. 1: 1st_filter_out\\[10\\]. 2: ~1st_filter_out\\[10\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "select value for AOI_16to8_11. 0: 0. 1: 1st_filter_out\\[11\\]. 2: ~1st_filter_out\\[11\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_11(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_11. 0: 0. 1: 1st_filter_out\\[11\\]. 2: ~1st_filter_out\\[11\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "select value for AOI_16to8_12. 0: 0. 1: 1st_filter_out\\[12\\]. 2: ~1st_filter_out\\[12\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_12. 0: 0. 1: 1st_filter_out\\[12\\]. 2: ~1st_filter_out\\[12\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "select value for AOI_16to8_13. 0: 0. 1: 1st_filter_out\\[13\\]. 2: ~1st_filter_out\\[13\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_13(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_13. 0: 0. 1: 1st_filter_out\\[13\\]. 2: ~1st_filter_out\\[13\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "select value for AOI_16to8_14. 0: 0. 1: 1st_filter_out\\[14\\]. 2: ~1st_filter_out\\[14\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_14(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_14. 0: 0. 1: 1st_filter_out\\[14\\]. 2: ~1st_filter_out\\[14\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "select value for AOI_16to8_15. 0: 0. 1: 1st_filter_out\\[15\\]. 2: ~1st_filter_out\\[15\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_16to8_15(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_16to8_15. 0: 0. 1: 1st_filter_out\\[15\\]. 2: ~1st_filter_out\\[15\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_16to8_15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Aoi16to8 {
        #[inline(always)]
        fn default() -> Aoi16to8 {
            Aoi16to8(0)
        }
    }
    #[doc = "CHN&index0 AOI_16to8_00_01 OR logic cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aoi8to70001(pub u32);
    impl Aoi8to70001 {
        #[doc = "select value for AOI_8to7_00_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "select value for AOI_8to7_00_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "select value for AOI_8to7_00_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "select value for AOI_8to7_00_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "select value for AOI_8to7_00_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "select value for AOI_8to7_00_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "select value for AOI_8to7_00_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "select value for AOI_8to7_00_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_00_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_00_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_00_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "select value for AOI_8to7_01_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "select value for AOI_8to7_01_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "select value for AOI_8to7_01_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "select value for AOI_8to7_01_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_3(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "select value for AOI_8to7_01_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "select value for AOI_8to7_01_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_5(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "select value for AOI_8to7_01_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "select value for AOI_8to7_01_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_01_7(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_01_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_01_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Aoi8to70001 {
        #[inline(always)]
        fn default() -> Aoi8to70001 {
            Aoi8to70001(0)
        }
    }
    #[doc = "CHN&index0 AOI_16to8_02_03 OR logic cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aoi8to70203(pub u32);
    impl Aoi8to70203 {
        #[doc = "select value for AOI_8to7_02_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "select value for AOI_8to7_02_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "select value for AOI_8to7_02_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "select value for AOI_8to7_02_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "select value for AOI_8to7_02_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "select value for AOI_8to7_02_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "select value for AOI_8to7_02_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "select value for AOI_8to7_02_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_02_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_02_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_02_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "select value for AOI_8to7_03_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "select value for AOI_8to7_03_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "select value for AOI_8to7_03_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "select value for AOI_8to7_03_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_3(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "select value for AOI_8to7_03_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "select value for AOI_8to7_03_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_5(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "select value for AOI_8to7_03_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "select value for AOI_8to7_03_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_03_7(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_03_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_03_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Aoi8to70203 {
        #[inline(always)]
        fn default() -> Aoi8to70203 {
            Aoi8to70203(0)
        }
    }
    #[doc = "CHN&index0 AOI_16to8_04_05 OR logic cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aoi8to70405(pub u32);
    impl Aoi8to70405 {
        #[doc = "select value for AOI_8to7_04_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "select value for AOI_8to7_04_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "select value for AOI_8to7_04_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "select value for AOI_8to7_04_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "select value for AOI_8to7_04_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "select value for AOI_8to7_04_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "select value for AOI_8to7_04_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "select value for AOI_8to7_04_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_04_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_04_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_04_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "select value for AOI_8to7_05_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "select value for AOI_8to7_05_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "select value for AOI_8to7_05_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_2(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "select value for AOI_8to7_05_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_3(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "select value for AOI_8to7_05_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "select value for AOI_8to7_05_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_5(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "select value for AOI_8to7_05_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "select value for AOI_8to7_05_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_05_7(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_05_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_05_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Aoi8to70405 {
        #[inline(always)]
        fn default() -> Aoi8to70405 {
            Aoi8to70405(0)
        }
    }
    #[doc = "CHN&index0 AOI_16to8_06 OR logic cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aoi8to706(pub u32);
    impl Aoi8to706 {
        #[doc = "select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_5(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_6(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub const fn aoi_8to7_06_7(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1."]
        #[inline(always)]
        pub fn set_aoi_8to7_06_7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Aoi8to706 {
        #[inline(always)]
        fn default() -> Aoi8to706 {
            Aoi8to706(0)
        }
    }
    #[doc = "CHN&index0 cfg ff."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgFf(pub u32);
    impl CfgFf {
        #[doc = "cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
        #[inline(always)]
        pub const fn sel_cfg_ff_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus."]
        #[inline(always)]
        pub fn set_sel_cfg_ff_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
        #[inline(always)]
        pub const fn sel_clk_source(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock."]
        #[inline(always)]
        pub fn set_sel_clk_source(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
        #[inline(always)]
        pub const fn sel_adder_minus(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active."]
        #[inline(always)]
        pub fn set_sel_adder_minus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
        #[inline(always)]
        pub const fn dis_osc_loop_clamp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp."]
        #[inline(always)]
        pub fn set_dis_osc_loop_clamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
        #[inline(always)]
        pub const fn osc_loop_clamp_value(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1."]
        #[inline(always)]
        pub fn set_osc_loop_clamp_value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for CfgFf {
        #[inline(always)]
        fn default() -> CfgFf {
            CfgFf(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChnCfgActive(pub u32);
    impl ChnCfgActive {
        #[doc = "write 0xF00D to enable all setting. Otherwire, all setting inactive."]
        #[inline(always)]
        pub const fn cfg_active(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "write 0xF00D to enable all setting. Otherwire, all setting inactive."]
        #[inline(always)]
        pub fn set_cfg_active(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ChnCfgActive {
        #[inline(always)]
        fn default() -> ChnCfgActive {
            ChnCfgActive(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filter1stPlaIn(pub u32);
    impl Filter1stPlaIn {
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn sync_edge_filter_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_sync_edge_filter_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub const fn software_inject(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub fn set_software_inject(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub const fn filter_reverse(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub fn set_filter_reverse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub const fn edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub fn set_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn nege_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_nege_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn pose_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_pose_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub const fn filter_sync_level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub fn set_filter_sync_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub const fn filter_ext_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub fn set_filter_ext_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub const fn filter_ext_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub fn set_filter_ext_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub const fn filter_ext_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub fn set_filter_ext_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Filter1stPlaIn {
        #[inline(always)]
        fn default() -> Filter1stPlaIn {
            Filter1stPlaIn(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filter1stPlaOut(pub u32);
    impl Filter1stPlaOut {
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn sync_edge_filter_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_sync_edge_filter_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub const fn software_inject(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub fn set_software_inject(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub const fn filter_reverse(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub fn set_filter_reverse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub const fn edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub fn set_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn nege_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_nege_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn pose_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_pose_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub const fn filter_sync_level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub fn set_filter_sync_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub const fn filter_ext_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub fn set_filter_ext_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub const fn filter_ext_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub fn set_filter_ext_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub const fn filter_ext_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub fn set_filter_ext_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Filter1stPlaOut {
        #[inline(always)]
        fn default() -> Filter1stPlaOut {
            Filter1stPlaOut(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filter2nd(pub u32);
    impl Filter2nd {
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn sync_edge_filter_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_sync_edge_filter_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub const fn software_inject(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub fn set_software_inject(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub const fn filter_reverse(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub fn set_filter_reverse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub const fn edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub fn set_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn nege_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_nege_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn pose_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_pose_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub const fn filter_sync_level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub fn set_filter_sync_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub const fn filter_ext_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub fn set_filter_ext_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub const fn filter_ext_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub fn set_filter_ext_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub const fn filter_ext_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub fn set_filter_ext_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Filter2nd {
        #[inline(always)]
        fn default() -> Filter2nd {
            Filter2nd(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filter3rd(pub u32);
    impl Filter3rd {
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn sync_edge_filter_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sync and edge detector filter. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_sync_edge_filter_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub const fn software_inject(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
        #[inline(always)]
        pub fn set_software_inject(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub const fn filter_reverse(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
        #[inline(always)]
        pub fn set_filter_reverse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub const fn edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
        #[inline(always)]
        pub fn set_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn nege_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "nege edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_nege_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn pose_edge_dect_enable(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "pose edge detector enable. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_pose_edge_dect_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub const fn filter_sync_level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "synchroniser level. 0: 2 level sync. 1: 3 level sync."]
        #[inline(always)]
        pub fn set_filter_sync_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub const fn filter_ext_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
        #[inline(always)]
        pub fn set_filter_ext_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub const fn filter_ext_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
        #[inline(always)]
        pub fn set_filter_ext_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub const fn filter_ext_counter(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period."]
        #[inline(always)]
        pub fn set_filter_ext_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Filter3rd {
        #[inline(always)]
        fn default() -> Filter3rd {
            Filter3rd(0)
        }
    }
}
