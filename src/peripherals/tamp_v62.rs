#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TAMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamp {
    ptr: *mut u8,
}
unsafe impl Send for Tamp {}
unsafe impl Sync for Tamp {}
impl Tamp {
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
    pub const fn tamp(self, n: usize) -> TampTamp {
        assert!(n < 4usize);
        unsafe { TampTamp::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Tamper flag."]
    #[inline(always)]
    pub const fn tamp_flag(self) -> crate::common::Reg<regs::TampFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Tamper interrupt enable."]
    #[inline(always)]
    pub const fn irq_en(self) -> crate::common::Reg<regs::IrqEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TampTamp {
    ptr: *mut u8,
}
unsafe impl Send for TampTamp {}
unsafe impl Sync for TampTamp {}
impl TampTamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Tamper n control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Tamper n Polynomial of LFSR."]
    #[inline(always)]
    pub const fn poly(self) -> crate::common::Reg<regs::Poly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Tamper n LFSR shift register."]
    #[inline(always)]
    pub const fn lfsr(self) -> crate::common::Reg<regs::Lfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Tamper n control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u32);
    impl Control {
        #[doc = "enable tamper 0: tamper disableed 1: tamper enabled."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable tamper 0: tamper disableed 1: tamper enabled."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "select active or passive tamper 0: passive tamper 1: active tamper."]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "select active or passive tamper 0: passive tamper 1: active tamper."]
        #[inline(always)]
        pub fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover."]
        #[inline(always)]
        pub const fn recover(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover."]
        #[inline(always)]
        pub fn set_recover(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second."]
        #[inline(always)]
        pub const fn speed(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second."]
        #[inline(always)]
        pub fn set_speed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "pin value for passive tamper."]
        #[inline(always)]
        pub const fn value(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "pin value for passive tamper."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle."]
        #[inline(always)]
        pub const fn filter(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle."]
        #[inline(always)]
        pub fn set_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used."]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used."]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    #[doc = "Tamper interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEn(pub u32);
    impl IrqEn {
        #[doc = "interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled."]
        #[inline(always)]
        pub const fn irq_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqEn {
        #[inline(always)]
        fn default() -> IrqEn {
            IrqEn(0)
        }
    }
    #[doc = "Tamper n LFSR shift register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lfsr(pub u32);
    impl Lfsr {
        #[doc = "LFSR for active tamper, write only register, always read 0."]
        #[inline(always)]
        pub const fn lfsr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "LFSR for active tamper, write only register, always read 0."]
        #[inline(always)]
        pub fn set_lfsr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lfsr {
        #[inline(always)]
        fn default() -> Lfsr {
            Lfsr(0)
        }
    }
    #[doc = "Tamper n Polynomial of LFSR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poly(pub u32);
    impl Poly {
        #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\"."]
        #[inline(always)]
        pub const fn poly(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\"."]
        #[inline(always)]
        pub fn set_poly(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Poly {
        #[inline(always)]
        fn default() -> Poly {
            Poly(0)
        }
    }
    #[doc = "Tamper flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TampFlag(pub u32);
    impl TampFlag {
        #[doc = "tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disappeared."]
        #[inline(always)]
        pub const fn flag(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disappeared."]
        #[inline(always)]
        pub fn set_flag(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for TampFlag {
        #[inline(always)]
        fn default() -> TampFlag {
            TampFlag(0)
        }
    }
}
