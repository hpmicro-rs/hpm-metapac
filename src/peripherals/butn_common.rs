#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BUTN."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Butn {
    ptr: *mut u8,
}
unsafe impl Send for Butn {}
unsafe impl Sync for Butn {}
impl Butn {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Button status."]
    #[inline(always)]
    pub const fn btn_status(self) -> crate::common::Reg<regs::BtnStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Button interrupt mask."]
    #[inline(always)]
    pub const fn btn_irq_mask(self) -> crate::common::Reg<regs::BtnIrqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Debounce setting."]
    #[inline(always)]
    pub const fn led_intense(self) -> crate::common::Reg<regs::LedIntense, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Button interrupt mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtnIrqMask(pub u32);
    impl BtnIrqMask {
        #[doc = "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn pbtn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_pbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn wbtn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_wbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn dbtn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_dbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn pclick(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_pclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn xpclick(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_xpclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn wclick(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_wclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn xwclick(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_xwclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for BtnIrqMask {
        #[inline(always)]
        fn default() -> BtnIrqMask {
            BtnIrqMask(0)
        }
    }
    #[doc = "Button status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtnStatus(pub u32);
    impl BtnStatus {
        #[doc = "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn pbtn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_pbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn wbtn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_wbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn dbtn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub fn set_dbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn pclick(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_pclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn xpclick(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_xpclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn wclick(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_wclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn xwclick(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub fn set_xwclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for BtnStatus {
        #[inline(always)]
        fn default() -> BtnStatus {
            BtnStatus(0)
        }
    }
    #[doc = "Debounce setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LedIntense(pub u32);
    impl LedIntense {
        #[doc = "Pbutton brightness 0."]
        #[inline(always)]
        pub const fn pled(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pbutton brightness 0."]
        #[inline(always)]
        pub fn set_pled(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Rbutton brightness 0."]
        #[inline(always)]
        pub const fn rled(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Rbutton brightness 0."]
        #[inline(always)]
        pub fn set_rled(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LedIntense {
        #[inline(always)]
        fn default() -> LedIntense {
            LedIntense(0)
        }
    }
}
