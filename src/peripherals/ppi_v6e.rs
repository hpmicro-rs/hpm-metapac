#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd {
    ptr: *mut u8,
}
unsafe impl Send for Cmd {}
unsafe impl Sync for Cmd {}
impl Cmd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "cmd_cfg."]
    #[inline(always)]
    pub const fn cmd_cfg(self) -> crate::common::Reg<regs::CmdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ad_cfg."]
    #[inline(always)]
    pub const fn ad_cfg(self) -> crate::common::Reg<regs::AdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ctrl_cfg."]
    #[inline(always)]
    pub const fn ctrl_cfg(self) -> crate::common::Reg<regs::CtrlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs {
    ptr: *mut u8,
}
unsafe impl Send for Cs {}
unsafe impl Sync for Cs {}
impl Cs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "cfg0."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "cfg1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "cfg2."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "cfg3."]
    #[inline(always)]
    pub const fn cfg3(self) -> crate::common::Reg<regs::Cfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "cfg4."]
    #[inline(always)]
    pub const fn cfg4(self) -> crate::common::Reg<regs::Cfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
#[doc = "PPI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppi {
    ptr: *mut u8,
}
unsafe impl Send for Ppi {}
unsafe impl Sync for Ppi {}
impl Ppi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "glb_cfg."]
    #[inline(always)]
    pub const fn glb_cfg(self) -> crate::common::Reg<regs::GlbCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "pad_cfg."]
    #[inline(always)]
    pub const fn pad_cfg(self) -> crate::common::Reg<regs::PadCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "clkpin_cfg."]
    #[inline(always)]
    pub const fn clkpin_cfg(self) -> crate::common::Reg<regs::ClkpinCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "tm_cfg."]
    #[inline(always)]
    pub const fn tm_cfg(self) -> crate::common::Reg<regs::TmCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "irq_sts."]
    #[inline(always)]
    pub const fn irq_sts(self) -> crate::common::Reg<regs::IrqSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "irq_en."]
    #[inline(always)]
    pub const fn irq_en(self) -> crate::common::Reg<regs::IrqEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cs(self, n: usize) -> Cs {
        assert!(n < 4usize);
        unsafe { Cs::from_ptr(self.ptr.add(0x40usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmd(self, n: usize) -> Cmd {
        assert!(n < 64usize);
        unsafe { Cmd::from_ptr(self.ptr.add(0x0400usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "ad_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdCfg(pub u32);
    impl AdCfg {
        #[doc = "select one of the 4 bytes(11 for 31:24, 10 for 23:16, 01 for 15:8, 00 for 7:0)."]
        #[inline(always)]
        pub const fn byte_sel0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "select one of the 4 bytes(11 for 31:24, 10 for 23:16, 01 for 15:8, 00 for 7:0)."]
        #[inline(always)]
        pub fn set_byte_sel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "0 for data; 1 for address."]
        #[inline(always)]
        pub const fn ad_sel0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0 for data; 1 for address."]
        #[inline(always)]
        pub fn set_ad_sel0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0 for OUT; 1 for IN."]
        #[inline(always)]
        pub const fn dir0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0 for OUT; 1 for IN."]
        #[inline(always)]
        pub fn set_dir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn byte_sel1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_byte_sel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ad_sel1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ad_sel1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dir1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn byte_sel2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_byte_sel2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ad_sel2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ad_sel2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dir2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dir2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn byte_sel3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_byte_sel3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ad_sel3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ad_sel3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dir3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dir3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for AdCfg {
        #[inline(always)]
        fn default() -> AdCfg {
            AdCfg(0)
        }
    }
    #[doc = "cfg0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "addr_start and addr_end config the address slot for CS0, use high 12bit, the minimun slot is 1Mbyte(addr_start==addr_end)."]
        #[inline(always)]
        pub const fn addr_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "addr_start and addr_end config the address slot for CS0, use high 12bit, the minimun slot is 1Mbyte(addr_start==addr_end)."]
        #[inline(always)]
        pub fn set_addr_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn addr_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_addr_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    #[doc = "cfg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "gennerally should be configured according to port size, 0 for 8bit; 1 for 16bit; 2 for 32bit;."]
        #[inline(always)]
        pub const fn addr_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "gennerally should be configured according to port size, 0 for 8bit; 1 for 16bit; 2 for 32bit;."]
        #[inline(always)]
        pub fn set_addr_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "the high AHB address will AND with {cs0_mask\\[15:0\\], 16'hFFFF}, shift right with addr_shift, then output as real address."]
        #[inline(always)]
        pub const fn addr_mask(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "the high AHB address will AND with {cs0_mask\\[15:0\\], 16'hFFFF}, shift right with addr_shift, then output as real address."]
        #[inline(always)]
        pub fn set_addr_mask(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    #[doc = "cfg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "CS enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "00-8bit; 01-16bit; 10-32bit; 11-reserved."]
        #[inline(always)]
        pub const fn port_size(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "00-8bit; 01-16bit; 10-32bit; 11-reserved."]
        #[inline(always)]
        pub fn set_port_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "set to none-zero value, will add delay between each command sequence for burst cmd, or splited transfer cmd sequence(such as transfer 32bit on 16bit port), CS will be de-assert during the delay."]
        #[inline(always)]
        pub const fn inter_cmd_dly(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "set to none-zero value, will add delay between each command sequence for burst cmd, or splited transfer cmd sequence(such as transfer 32bit on 16bit port), CS will be de-assert during the delay."]
        #[inline(always)]
        pub fn set_inter_cmd_dly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: use two stage sync; 1: use one stage sync."]
        #[inline(always)]
        pub const fn ready_in_sel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "0: use two stage sync; 1: use one stage sync."]
        #[inline(always)]
        pub fn set_ready_in_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CS assert at when clk_div_cnt equal to sync_clk_sel."]
        #[inline(always)]
        pub const fn sync_clk_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "CS assert at when clk_div_cnt equal to sync_clk_sel."]
        #[inline(always)]
        pub fn set_sync_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "set to enable CS pin sync with clock counter. Clr if use async mode(no clk pin), or not care the CS start time with clk pin."]
        #[inline(always)]
        pub const fn cs_sync_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable CS pin sync with clock counter. Clr if use async mode(no clk pin), or not care the CS start time with clk pin."]
        #[inline(always)]
        pub fn set_cs_sync_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    #[doc = "cfg3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg3(pub u32);
    impl Cfg3 {
        #[doc = "first read cmd start index."]
        #[inline(always)]
        pub const fn rcmd_start0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "first read cmd start index."]
        #[inline(always)]
        pub fn set_rcmd_start0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "first read cmd end index."]
        #[inline(always)]
        pub const fn rcmd_end0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "first read cmd end index."]
        #[inline(always)]
        pub fn set_rcmd_end0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "sequential read cmd start index."]
        #[inline(always)]
        pub const fn rcmd_start1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "sequential read cmd start index."]
        #[inline(always)]
        pub fn set_rcmd_start1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "sequential read cmd end index."]
        #[inline(always)]
        pub const fn rcmd_end1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "sequential read cmd end index."]
        #[inline(always)]
        pub fn set_rcmd_end1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Cfg3 {
        #[inline(always)]
        fn default() -> Cfg3 {
            Cfg3(0)
        }
    }
    #[doc = "cfg4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg4(pub u32);
    impl Cfg4 {
        #[doc = "first write cmd start index."]
        #[inline(always)]
        pub const fn wcmd_start0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "first write cmd start index."]
        #[inline(always)]
        pub fn set_wcmd_start0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "first write cmd end index."]
        #[inline(always)]
        pub const fn wcmd_end0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "first write cmd end index."]
        #[inline(always)]
        pub fn set_wcmd_end0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "sequential write cmd start index."]
        #[inline(always)]
        pub const fn wcmd_start1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "sequential write cmd start index."]
        #[inline(always)]
        pub fn set_wcmd_start1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "sequential write cmd end index."]
        #[inline(always)]
        pub const fn wcmd_end1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "sequential write cmd end index."]
        #[inline(always)]
        pub fn set_wcmd_end1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Cfg4 {
        #[inline(always)]
        fn default() -> Cfg4 {
            Cfg4(0)
        }
    }
    #[doc = "clkpin_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkpinCfg(pub u32);
    impl ClkpinCfg {
        #[doc = "set to enable clock logic."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable clock logic."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: use clk_gate in cmd sequence for whether output clock 1: always enable clock output;."]
        #[inline(always)]
        pub const fn aon(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: use clk_gate in cmd sequence for whether output clock 1: always enable clock output;."]
        #[inline(always)]
        pub fn set_aon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set to invert clock output."]
        #[inline(always)]
        pub const fn invert(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to invert clock output."]
        #[inline(always)]
        pub fn set_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "clock low number."]
        #[inline(always)]
        pub const fn low(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "clock low number."]
        #[inline(always)]
        pub fn set_low(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "clock high numer."]
        #[inline(always)]
        pub const fn high(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "clock high numer."]
        #[inline(always)]
        pub fn set_high(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "there will be a system counter run from 0 to cycle, clk output will be set to high when counter is clk_high, and low when counter is clk_low. The output will be system clock if cycle is 0. All 4 CS share same clock configuration(one clock pin with configured frequency). different CS can be assert at different counter value."]
        #[inline(always)]
        pub const fn cycle(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "there will be a system counter run from 0 to cycle, clk output will be set to high when counter is clk_high, and low when counter is clk_low. The output will be system clock if cycle is 0. All 4 CS share same clock configuration(one clock pin with configured frequency). different CS can be assert at different counter value."]
        #[inline(always)]
        pub fn set_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for ClkpinCfg {
        #[inline(always)]
        fn default() -> ClkpinCfg {
            ClkpinCfg(0)
        }
    }
    #[doc = "cmd_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdCfg(pub u32);
    impl CmdCfg {
        #[doc = "cmd clock cycles."]
        #[inline(always)]
        pub const fn cycle_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "cmd clock cycles."]
        #[inline(always)]
        pub fn set_cycle_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the clock gate enable signal, set to output clock signal."]
        #[inline(always)]
        pub const fn clk_gate(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "the clock gate enable signal, set to output clock signal."]
        #[inline(always)]
        pub fn set_clk_gate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "cs value in current cmd."]
        #[inline(always)]
        pub const fn cs_val(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "cs value in current cmd."]
        #[inline(always)]
        pub fn set_cs_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for CmdCfg {
        #[inline(always)]
        fn default() -> CmdCfg {
            CmdCfg(0)
        }
    }
    #[doc = "ctrl_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlCfg(pub u32);
    impl CtrlCfg {
        #[doc = "for OUT, it defines the output value(0 or 1); for IN, it defines whether to wait for ready(ready polarity is defined in ctrl_pad_pol)."]
        #[inline(always)]
        pub const fn io_cfg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "for OUT, it defines the output value(0 or 1); for IN, it defines whether to wait for ready(ready polarity is defined in ctrl_pad_pol)."]
        #[inline(always)]
        pub fn set_io_cfg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg3(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg4(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg5(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg6(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn io_cfg7(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_io_cfg7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for CtrlCfg {
        #[inline(always)]
        fn default() -> CtrlCfg {
            CtrlCfg(0)
        }
    }
    #[doc = "glb_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCfg(pub u32);
    impl GlbCfg {
        #[doc = "software reset."]
        #[inline(always)]
        pub const fn soft_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "software reset."]
        #[inline(always)]
        pub fn set_soft_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: register output, one cycle delay; 1: direct output， no delay but may have timing issue."]
        #[inline(always)]
        pub const fn pad_out_reg_enj(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: register output, one cycle delay; 1: direct output， no delay but may have timing issue."]
        #[inline(always)]
        pub fn set_pad_out_reg_enj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for GlbCfg {
        #[inline(always)]
        fn default() -> GlbCfg {
            GlbCfg(0)
        }
    }
    #[doc = "irq_en."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEn(pub u32);
    impl IrqEn {
        #[doc = "timeout interrupt enable."]
        #[inline(always)]
        pub const fn irq_tmout_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "timeout interrupt enable."]
        #[inline(always)]
        pub fn set_irq_tmout_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IrqEn {
        #[inline(always)]
        fn default() -> IrqEn {
            IrqEn(0)
        }
    }
    #[doc = "irq_sts."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSts(pub u32);
    impl IrqSts {
        #[doc = "tiemout interrupt status, write 1 to clear."]
        #[inline(always)]
        pub const fn irq_tmout_sts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "tiemout interrupt status, write 1 to clear."]
        #[inline(always)]
        pub fn set_irq_tmout_sts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IrqSts {
        #[inline(always)]
        fn default() -> IrqSts {
            IrqSts(0)
        }
    }
    #[doc = "pad_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PadCfg(pub u32);
    impl PadCfg {
        #[doc = "for OUT pad: 0: output the value in cmd 1: output reversed value in cmd for IN pad, defines the signal active value, when ctrl_cfg.io_cfg is set, will wait the active value for ready(generally read or write ready)."]
        #[inline(always)]
        pub const fn ctrl_pad_pol(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "for OUT pad: 0: output the value in cmd 1: output reversed value in cmd for IN pad, defines the signal active value, when ctrl_cfg.io_cfg is set, will wait the active value for ready(generally read or write ready)."]
        #[inline(always)]
        pub fn set_ctrl_pad_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the pad output enable signal. 0 for IN; 1 for OUT. NOTE: for unused pads, set both ctrl_pad_oe and ctrl_pad_pol to 0."]
        #[inline(always)]
        pub const fn ctrl_pad_oe(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the pad output enable signal. 0 for IN; 1 for OUT. NOTE: for unused pads, set both ctrl_pad_oe and ctrl_pad_pol to 0."]
        #[inline(always)]
        pub fn set_ctrl_pad_oe(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "dm pin polarity."]
        #[inline(always)]
        pub const fn dm_pad_pol(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "dm pin polarity."]
        #[inline(always)]
        pub fn set_dm_pad_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "cs pin idle state, default high for active low."]
        #[inline(always)]
        pub const fn cs_idle_st(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "cs pin idle state, default high for active low."]
        #[inline(always)]
        pub fn set_cs_idle_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for PadCfg {
        #[inline(always)]
        fn default() -> PadCfg {
            PadCfg(0)
        }
    }
    #[doc = "tm_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TmCfg(pub u32);
    impl TmCfg {
        #[doc = "timeout value, max 20us at 200MHz clock."]
        #[inline(always)]
        pub const fn tm_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "timeout value, max 20us at 200MHz clock."]
        #[inline(always)]
        pub fn set_tm_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "timeout enable. if enabled, then if each AHB transfer time exceed tm_cfg clock cycles, will assert irq."]
        #[inline(always)]
        pub const fn tm_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "timeout enable. if enabled, then if each AHB transfer time exceed tm_cfg clock cycles, will assert irq."]
        #[inline(always)]
        pub fn set_tm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for TmCfg {
        #[inline(always)]
        fn default() -> TmCfg {
            TmCfg(0)
        }
    }
}
