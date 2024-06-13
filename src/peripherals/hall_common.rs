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
    #[doc = "W counter."]
    #[inline(always)]
    pub const fn w(self) -> crate::common::Reg<regs::W, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "V counter."]
    #[inline(always)]
    pub const fn v(self) -> crate::common::Reg<regs::V, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "U counter."]
    #[inline(always)]
    pub const fn u(self) -> crate::common::Reg<regs::U, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer counter."]
    #[inline(always)]
    pub const fn tmr(self) -> crate::common::Reg<regs::Tmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "HALL0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hall {
    ptr: *mut u8,
}
unsafe impl Send for Hall {}
unsafe impl Sync for Hall {}
impl Hall {
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
    #[doc = "U,V,W configure register."]
    #[inline(always)]
    pub const fn uvwcfg(self) -> crate::common::Reg<regs::Uvwcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Trigger output enable register."]
    #[inline(always)]
    pub const fn trgoen(self) -> crate::common::Reg<regs::Trgoen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Read event enable register."]
    #[inline(always)]
    pub const fn readen(self) -> crate::common::Reg<regs::Readen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DMA enable register."]
    #[inline(always)]
    pub const fn dmaen(self) -> crate::common::Reg<regs::Dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt request enable register."]
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
    pub const fn his(self, n: usize) -> His {
        assert!(n < 3usize);
        unsafe { His::from_ptr(self.ptr.add(0x70usize + n * 8usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct His {
    ptr: *mut u8,
}
unsafe impl Send for His {}
unsafe impl Sync for His {}
impl His {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "history register 0."]
    #[inline(always)]
    pub const fn his0(self) -> crate::common::Reg<regs::His0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "history register 1."]
    #[inline(always)]
    pub const fn his1(self) -> crate::common::Reg<regs::His1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "set to reset all counter and related snapshots."]
        #[inline(always)]
        pub const fn rstcnt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to reset all counter and related snapshots."]
        #[inline(always)]
        pub fn set_rstcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub const fn snapen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub fn set_snapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
        #[inline(always)]
        pub const fn read(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
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
    #[doc = "DMA enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaen(pub u32);
    impl Dmaen {
        #[doc = "1- generate dma request when w flag set."]
        #[inline(always)]
        pub const fn wfen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when w flag set."]
        #[inline(always)]
        pub fn set_wfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- generate dma request when v flag set."]
        #[inline(always)]
        pub const fn vfen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when v flag set."]
        #[inline(always)]
        pub fn set_vfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1- generate dma request when u flag set."]
        #[inline(always)]
        pub const fn ufen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when u flag set."]
        #[inline(always)]
        pub fn set_ufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "1- generate dma request when phdly flag set."]
        #[inline(always)]
        pub const fn phdlyen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when phdly flag set."]
        #[inline(always)]
        pub fn set_phdlyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate dma request when phpre flag set."]
        #[inline(always)]
        pub const fn phpreen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when phpre flag set."]
        #[inline(always)]
        pub fn set_phpreen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate dma request when phupt flag set."]
        #[inline(always)]
        pub const fn phupten(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when phupt flag set."]
        #[inline(always)]
        pub fn set_phupten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmaen {
        #[inline(always)]
        fn default() -> Dmaen {
            Dmaen(0)
        }
    }
    #[doc = "history register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct His0(pub u32);
    impl His0 {
        #[doc = "copy of ucnt when u signal transition from 0 to 1."]
        #[inline(always)]
        pub const fn uhis0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "copy of ucnt when u signal transition from 0 to 1."]
        #[inline(always)]
        pub fn set_uhis0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for His0 {
        #[inline(always)]
        fn default() -> His0 {
            His0(0)
        }
    }
    #[doc = "history register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct His1(pub u32);
    impl His1 {
        #[doc = "copy of ucnt when u signal transition from 1 to 0."]
        #[inline(always)]
        pub const fn uhis1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "copy of ucnt when u signal transition from 1 to 0."]
        #[inline(always)]
        pub fn set_uhis1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for His1 {
        #[inline(always)]
        fn default() -> His1 {
            His1(0)
        }
    }
    #[doc = "Interrupt request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "1- generate interrupt request when w flag set."]
        #[inline(always)]
        pub const fn wfie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when w flag set."]
        #[inline(always)]
        pub fn set_wfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- generate interrupt request when v flag set."]
        #[inline(always)]
        pub const fn vfie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when v flag set."]
        #[inline(always)]
        pub fn set_vfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1- generate interrupt request when u flag set."]
        #[inline(always)]
        pub const fn ufie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when u flag set."]
        #[inline(always)]
        pub fn set_ufie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "1- generate interrupt request when phdly flag set."]
        #[inline(always)]
        pub const fn phdlyie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when phdly flag set."]
        #[inline(always)]
        pub fn set_phdlyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate interrupt request when phpre flag set."]
        #[inline(always)]
        pub const fn phpreie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when phpre flag set."]
        #[inline(always)]
        pub fn set_phpreie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate interrupt request when phupt flag set."]
        #[inline(always)]
        pub const fn phuptie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when phupt flag set."]
        #[inline(always)]
        pub fn set_phuptie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate interrupt request when wdg flag set."]
        #[inline(always)]
        pub const fn wdgie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when wdg flag set."]
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
    #[doc = "Phase configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcfg(pub u32);
    impl Phcfg {
        #[doc = "delay clock cycles number."]
        #[inline(always)]
        pub const fn dlycnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "delay clock cycles number."]
        #[inline(always)]
        pub fn set_dlycnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle."]
        #[inline(always)]
        pub const fn dlysel(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle."]
        #[inline(always)]
        pub fn set_dlysel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Phcfg {
        #[inline(always)]
        fn default() -> Phcfg {
            Phcfg(0)
        }
    }
    #[doc = "Read event enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Readen(pub u32);
    impl Readen {
        #[doc = "1- load counters to their read registers when w flag set."]
        #[inline(always)]
        pub const fn wfen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when w flag set."]
        #[inline(always)]
        pub fn set_wfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- load counters to their read registers when v flag set."]
        #[inline(always)]
        pub const fn vfen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when v flag set."]
        #[inline(always)]
        pub fn set_vfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1- load counters to their read registers when u flag set."]
        #[inline(always)]
        pub const fn ufen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when u flag set."]
        #[inline(always)]
        pub fn set_ufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "1- load counters to their read registers when phdly flag set."]
        #[inline(always)]
        pub const fn phdlyen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when phdly flag set."]
        #[inline(always)]
        pub fn set_phdlyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- load counters to their read registers when phpre flag set."]
        #[inline(always)]
        pub const fn phpreen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when phpre flag set."]
        #[inline(always)]
        pub fn set_phpreen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- load counters to their read registers when phupt flag set."]
        #[inline(always)]
        pub const fn phupten(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when phupt flag set."]
        #[inline(always)]
        pub fn set_phupten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Readen {
        #[inline(always)]
        fn default() -> Readen {
            Readen(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "w flag, will set when w signal toggle."]
        #[inline(always)]
        pub const fn wf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "w flag, will set when w signal toggle."]
        #[inline(always)]
        pub fn set_wf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "v flag, will set when v signal toggle."]
        #[inline(always)]
        pub const fn vf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "v flag, will set when v signal toggle."]
        #[inline(always)]
        pub fn set_vf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "u flag, will set when u signal toggle."]
        #[inline(always)]
        pub const fn uf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "u flag, will set when u signal toggle."]
        #[inline(always)]
        pub fn set_uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting."]
        #[inline(always)]
        pub const fn phdlyf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting."]
        #[inline(always)]
        pub fn set_phdlyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle."]
        #[inline(always)]
        pub const fn phpref(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle."]
        #[inline(always)]
        pub fn set_phpref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "phase update flag, will set when any of u, v, w signal toggle."]
        #[inline(always)]
        pub const fn phuptf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "phase update flag, will set when any of u, v, w signal toggle."]
        #[inline(always)]
        pub fn set_phuptf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "watchdog count timeout flag."]
        #[inline(always)]
        pub const fn wdgf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog count timeout flag."]
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
        pub const fn timer(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32 bit free run timer."]
        #[inline(always)]
        pub fn set_timer(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tmr {
        #[inline(always)]
        fn default() -> Tmr {
            Tmr(0)
        }
    }
    #[doc = "Trigger output enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgoen(pub u32);
    impl Trgoen {
        #[doc = "1- enable trigger output when w flag set."]
        #[inline(always)]
        pub const fn wfen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when w flag set."]
        #[inline(always)]
        pub fn set_wfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- enable trigger output when v flag set."]
        #[inline(always)]
        pub const fn vfen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when v flag set."]
        #[inline(always)]
        pub fn set_vfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1- enable trigger output when u flag set."]
        #[inline(always)]
        pub const fn ufen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when u flag set."]
        #[inline(always)]
        pub fn set_ufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "1- enable trigger output when phdly flag set."]
        #[inline(always)]
        pub const fn phdlyen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when phdly flag set."]
        #[inline(always)]
        pub fn set_phdlyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- enable trigger output when phpre flag set."]
        #[inline(always)]
        pub const fn phpreen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when phpre flag set."]
        #[inline(always)]
        pub fn set_phpreen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- enable trigger output when phupt flag set."]
        #[inline(always)]
        pub const fn phupten(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when phupt flag set."]
        #[inline(always)]
        pub fn set_phupten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Trgoen {
        #[inline(always)]
        fn default() -> Trgoen {
            Trgoen(0)
        }
    }
    #[doc = "U counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct U(pub u32);
    impl U {
        #[doc = "ucnt counter."]
        #[inline(always)]
        pub const fn ucnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "ucnt counter."]
        #[inline(always)]
        pub fn set_ucnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "this bit indicate W state."]
        #[inline(always)]
        pub const fn wstat(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "this bit indicate W state."]
        #[inline(always)]
        pub fn set_wstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "this bit indicate V state."]
        #[inline(always)]
        pub const fn vstat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "this bit indicate V state."]
        #[inline(always)]
        pub fn set_vstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "this bit indicate U state."]
        #[inline(always)]
        pub const fn ustat(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "this bit indicate U state."]
        #[inline(always)]
        pub fn set_ustat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for U {
        #[inline(always)]
        fn default() -> U {
            U(0)
        }
    }
    #[doc = "U,V,W configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uvwcfg(pub u32);
    impl Uvwcfg {
        #[doc = "the clock cycle number which the pre flag will set before the next uvw transition."]
        #[inline(always)]
        pub const fn precnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "the clock cycle number which the pre flag will set before the next uvw transition."]
        #[inline(always)]
        pub fn set_precnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Uvwcfg {
        #[inline(always)]
        fn default() -> Uvwcfg {
            Uvwcfg(0)
        }
    }
    #[doc = "V counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct V(pub u32);
    impl V {
        #[doc = "vcnt counter."]
        #[inline(always)]
        pub const fn vcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "vcnt counter."]
        #[inline(always)]
        pub fn set_vcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for V {
        #[inline(always)]
        fn default() -> V {
            V(0)
        }
    }
    #[doc = "W counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct W(pub u32);
    impl W {
        #[doc = "wcnt counter."]
        #[inline(always)]
        pub const fn wcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "wcnt counter."]
        #[inline(always)]
        pub fn set_wcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for W {
        #[inline(always)]
        fn default() -> W {
            W(0)
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
}
