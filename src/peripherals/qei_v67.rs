#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count {
    ptr: *mut u8,
}
unsafe impl Send for Count {}
unsafe impl Sync for Count {}
impl Count {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Z counter."]
    #[inline(always)]
    pub const fn z(self) -> crate::common::Reg<regs::Z, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Phase counter."]
    #[inline(always)]
    pub const fn ph(self) -> crate::common::Reg<regs::Ph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Speed counter."]
    #[inline(always)]
    pub const fn spd(self) -> crate::common::Reg<regs::Spd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer counter."]
    #[inline(always)]
    pub const fn tmr(self) -> crate::common::Reg<regs::Tmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "QEI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qei {
    ptr: *mut u8,
}
unsafe impl Send for Qei {}
unsafe impl Sync for Qei {}
impl Qei {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Phase configure register."]
    #[inline(always)]
    pub const fn phcfg(self) -> crate::common::Reg<regs::Phcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Watchdog configure register."]
    #[inline(always)]
    pub const fn wdgcfg(self) -> crate::common::Reg<regs::Wdgcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Phase index register."]
    #[inline(always)]
    pub const fn phidx(self) -> crate::common::Reg<regs::Phidx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Tigger output enable register."]
    #[inline(always)]
    pub const fn trgoen(self) -> crate::common::Reg<regs::Trgoen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Read event enable register."]
    #[inline(always)]
    pub const fn readen(self) -> crate::common::Reg<regs::Readen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Z comparator."]
    #[inline(always)]
    pub const fn zcmp(self) -> crate::common::Reg<regs::Zcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Phase comparator."]
    #[inline(always)]
    pub const fn phcmp(self) -> crate::common::Reg<regs::Phcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Speed comparator."]
    #[inline(always)]
    pub const fn spdcmp(self) -> crate::common::Reg<regs::Spdcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DMA request enable register."]
    #[inline(always)]
    pub const fn dmaen(self) -> crate::common::Reg<regs::Dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt request register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn count(self, n: usize) -> Count {
        assert!(n < 4usize);
        unsafe { Count::from_ptr(self.ptr.add(0x30usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn spdhis(self, n: usize) -> crate::common::Reg<regs::Spdhis, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "00-abz; 01-pd; 10-ud; 11-reserved."]
        #[inline(always)]
        pub const fn enctyp(&self) -> super::vals::WorkMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::WorkMode::from_bits(val as u8)
        }
        #[doc = "00-abz; 01-pd; 10-ud; 11-reserved."]
        #[inline(always)]
        pub fn set_enctyp(&mut self, val: super::vals::WorkMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx."]
        #[inline(always)]
        pub const fn rstcnt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx."]
        #[inline(always)]
        pub fn set_rstcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub const fn snapen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub fn set_snapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)."]
        #[inline(always)]
        pub const fn hfdir0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)."]
        #[inline(always)]
        pub fn set_hfdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub const fn hfdir1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub fn set_hfdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)."]
        #[inline(always)]
        pub const fn hrdir0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)."]
        #[inline(always)]
        pub fn set_hrdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub const fn hrdir1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub fn set_hrdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- pause zcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pausez(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause zcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pausez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1- pause phcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pauseph(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause phcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pauseph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1- pause spdcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pausespd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause spdcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pausespd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1- reset zcnt when H assert."]
        #[inline(always)]
        pub const fn hrstz(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset zcnt when H assert."]
        #[inline(always)]
        pub fn set_hrstz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "1- reset phcnt when H assert."]
        #[inline(always)]
        pub const fn hrstph(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset phcnt when H assert."]
        #[inline(always)]
        pub fn set_hrstph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "1- reset spdcnt when H assert."]
        #[inline(always)]
        pub const fn hrstspd(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset spdcnt when H assert."]
        #[inline(always)]
        pub fn set_hrstspd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
        #[inline(always)]
        pub const fn read(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
        #[inline(always)]
        pub fn set_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "DMA request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaen(pub u32);
    impl Dmaen {
        #[doc = "1- generate dma request when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate dma request when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate dma request when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmaen {
        #[inline(always)]
        fn default() -> Dmaen {
            Dmaen(0)
        }
    }
    #[doc = "Interrupt request register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "1- generate interrupt when zphf flag set."]
        #[inline(always)]
        pub const fn zphie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when zphf flag set."]
        #[inline(always)]
        pub fn set_zphie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate interrupt when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate interrupt when homef flag set."]
        #[inline(always)]
        pub const fn homeie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when homef flag set."]
        #[inline(always)]
        pub fn set_homeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate interrupt when wdg flag set."]
        #[inline(always)]
        pub const fn wdgie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Irqen {
        #[inline(always)]
        fn default() -> Irqen {
            Irqen(0)
        }
    }
    #[doc = "Phase counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ph(pub u32);
    impl Ph {
        #[doc = "phcnt value."]
        #[inline(always)]
        pub const fn phcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "phcnt value."]
        #[inline(always)]
        pub fn set_phcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub const fn bstat(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub fn set_bstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub const fn astat(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub fn set_astat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ph {
        #[inline(always)]
        fn default() -> Ph {
            Ph(0)
        }
    }
    #[doc = "Phase configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcfg(pub u32);
    impl Phcfg {
        #[doc = "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax."]
        #[inline(always)]
        pub const fn phmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax."]
        #[inline(always)]
        pub fn set_phmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[doc = "1- phcnt will set to phidx when Z input assert."]
        #[inline(always)]
        pub const fn phcaliz(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- phcnt will set to phidx when Z input assert."]
        #[inline(always)]
        pub fn set_phcaliz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert."]
        #[inline(always)]
        pub const fn zcntcfg(&self) -> super::vals::ZCntMode {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::ZCntMode::from_bits(val as u8)
        }
        #[doc = "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert."]
        #[inline(always)]
        pub fn set_zcntcfg(&mut self, val: super::vals::ZCntMode) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Phcfg {
        #[inline(always)]
        fn default() -> Phcfg {
            Phcfg(0)
        }
    }
    #[doc = "Phase comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcmp(pub u32);
    impl Phcmp {
        #[doc = "phcnt position compare value."]
        #[inline(always)]
        pub const fn phcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "phcnt position compare value."]
        #[inline(always)]
        pub fn set_phcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[doc = "0- position compare need positive rotation 1- position compare need negative rotation."]
        #[inline(always)]
        pub const fn dircmp(&self) -> super::vals::Dir {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "0- position compare need positive rotation 1- position compare need negative rotation."]
        #[inline(always)]
        pub fn set_dircmp(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "1- postion compare not include rotation direction."]
        #[inline(always)]
        pub const fn dircmpdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- postion compare not include rotation direction."]
        #[inline(always)]
        pub fn set_dircmpdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- postion compare not include zcnt."]
        #[inline(always)]
        pub const fn zcmpdis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- postion compare not include zcnt."]
        #[inline(always)]
        pub fn set_zcmpdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Phcmp {
        #[inline(always)]
        fn default() -> Phcmp {
            Phcmp(0)
        }
    }
    #[doc = "Phase index register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phidx(pub u32);
    impl Phidx {
        #[doc = "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1."]
        #[inline(always)]
        pub const fn phidx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1."]
        #[inline(always)]
        pub fn set_phidx(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
    }
    impl Default for Phidx {
        #[inline(always)]
        fn default() -> Phidx {
            Phidx(0)
        }
    }
    #[doc = "Read event enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Readen(pub u32);
    impl Readen {
        #[doc = "1- load counters to their read registers when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- load counters to their read registers when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- load counters to their read registers when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Readen {
        #[inline(always)]
        fn default() -> Readen {
            Readen(0)
        }
    }
    #[doc = "Speed counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spd(pub u32);
    impl Spd {
        #[doc = "spdcnt value."]
        #[inline(always)]
        pub const fn spdcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "spdcnt value."]
        #[inline(always)]
        pub fn set_spdcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub const fn bstat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub fn set_bstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub const fn astat(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub fn set_astat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Spd {
        #[inline(always)]
        fn default() -> Spd {
            Spd(0)
        }
    }
    #[doc = "Speed comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spdcmp(pub u32);
    impl Spdcmp {
        #[doc = "spdcnt position compare value."]
        #[inline(always)]
        pub const fn spdcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "spdcnt position compare value."]
        #[inline(always)]
        pub fn set_spdcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Spdcmp {
        #[inline(always)]
        fn default() -> Spdcmp {
            Spdcmp(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spdhis(pub u32);
    impl Spdhis {
        #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low."]
        #[inline(always)]
        pub const fn spdhis0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low."]
        #[inline(always)]
        pub fn set_spdhis0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Spdhis {
        #[inline(always)]
        fn default() -> Spdhis {
            Spdhis(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "z input flag."]
        #[inline(always)]
        pub const fn zphf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "z input flag."]
        #[inline(always)]
        pub fn set_zphf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "postion compare match flag."]
        #[inline(always)]
        pub const fn poscmpf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "postion compare match flag."]
        #[inline(always)]
        pub fn set_poscmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "home flag."]
        #[inline(always)]
        pub const fn homef(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "home flag."]
        #[inline(always)]
        pub fn set_homef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "watchdog flag."]
        #[inline(always)]
        pub const fn wdgf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog flag."]
        #[inline(always)]
        pub fn set_wdgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Timer counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr(pub u32);
    impl Tmr {
        #[doc = "32 bit free run timer."]
        #[inline(always)]
        pub const fn tmrcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32 bit free run timer."]
        #[inline(always)]
        pub fn set_tmrcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tmr {
        #[inline(always)]
        fn default() -> Tmr {
            Tmr(0)
        }
    }
    #[doc = "Tigger output enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgoen(pub u32);
    impl Trgoen {
        #[doc = "1- enable trigger output when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- enable trigger output when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- enable trigger output when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Trgoen {
        #[inline(always)]
        fn default() -> Trgoen {
            Trgoen(0)
        }
    }
    #[doc = "Watchdog configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgcfg(pub u32);
    impl Wdgcfg {
        #[doc = "watch dog timeout value."]
        #[inline(always)]
        pub const fn wdgto(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "watch dog timeout value."]
        #[inline(always)]
        pub fn set_wdgto(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "1- enable wdog counter."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable wdog counter."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wdgcfg {
        #[inline(always)]
        fn default() -> Wdgcfg {
            Wdgcfg(0)
        }
    }
    #[doc = "Z counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Z(pub u32);
    impl Z {
        #[doc = "zcnt value."]
        #[inline(always)]
        pub const fn zcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "zcnt value."]
        #[inline(always)]
        pub fn set_zcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Z {
        #[inline(always)]
        fn default() -> Z {
            Z(0)
        }
    }
    #[doc = "Z comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zcmp(pub u32);
    impl Zcmp {
        #[doc = "zcnt postion compare value."]
        #[inline(always)]
        pub const fn zcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "zcnt postion compare value."]
        #[inline(always)]
        pub fn set_zcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Zcmp {
        #[inline(always)]
        fn default() -> Zcmp {
            Zcmp(0)
        }
    }
}
pub mod vals {
    #[doc = "Rotation direction."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "Forward"]
        FORWARD = 0x0,
        #[doc = "Reverse"]
        REVERSE = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[doc = "Decoder work mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WorkMode {
        #[doc = "ABZ."]
        ABZ = 0x0,
        #[doc = "PD mode, Pluse + Direction."]
        PD = 0x01,
        #[doc = "UD mode, Up pluse + Down pluse."]
        UD = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl WorkMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WorkMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WorkMode {
        #[inline(always)]
        fn from(val: u8) -> WorkMode {
            WorkMode::from_bits(val)
        }
    }
    impl From<WorkMode> for u8 {
        #[inline(always)]
        fn from(val: WorkMode) -> u8 {
            WorkMode::to_bits(val)
        }
    }
    #[doc = "Z counter inc mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ZCntMode {
        #[doc = "Z counter."]
        ON_Z_INPUT = 0x0,
        #[doc = "Z counter with phase."]
        ON_PHASE_COUNT_MAX = 0x01,
    }
    impl ZCntMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ZCntMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ZCntMode {
        #[inline(always)]
        fn from(val: u8) -> ZCntMode {
            ZCntMode::from_bits(val)
        }
    }
    impl From<ZCntMode> for u8 {
        #[inline(always)]
        fn from(val: ZCntMode) -> u8 {
            ZCntMode::to_bits(val)
        }
    }
}
