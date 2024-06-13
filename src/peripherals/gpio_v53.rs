#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct As {
    ptr: *mut u8,
}
unsafe impl Send for As {}
unsafe impl Sync for As {}
impl As {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt asynchronous value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::AsValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO interrupt asynchronous set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::AsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO interrupt asynchronous clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::AsClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO interrupt asynchronous toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::AsToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Di {
    ptr: *mut u8,
}
unsafe impl Send for Di {}
unsafe impl Sync for Di {}
impl Di {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO input value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::DiValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Do {
    ptr: *mut u8,
}
unsafe impl Send for Do {}
unsafe impl Sync for Do {}
impl Do {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO output value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::DoValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO output set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::DoSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO output clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::DoClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO output toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::DoToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "FGPIO, GPIO0, PGPIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
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
    pub const fn di(self, n: usize) -> Di {
        assert!(n < 15usize);
        unsafe { Di::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn do_(self, n: usize) -> Do {
        assert!(n < 15usize);
        unsafe { Do::from_ptr(self.ptr.add(0x0100usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn oe(self, n: usize) -> Oe {
        assert!(n < 15usize);
        unsafe { Oe::from_ptr(self.ptr.add(0x0200usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn if_(self, n: usize) -> If {
        assert!(n < 15usize);
        unsafe { If::from_ptr(self.ptr.add(0x0300usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ie(self, n: usize) -> Ie {
        assert!(n < 15usize);
        unsafe { Ie::from_ptr(self.ptr.add(0x0400usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pl(self, n: usize) -> Pl {
        assert!(n < 15usize);
        unsafe { Pl::from_ptr(self.ptr.add(0x0500usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tp(self, n: usize) -> Tp {
        assert!(n < 15usize);
        unsafe { Tp::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn as_(self, n: usize) -> As {
        assert!(n < 15usize);
        unsafe { As::from_ptr(self.ptr.add(0x0700usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pd(self, n: usize) -> Pd {
        assert!(n < 15usize);
        unsafe { Pd::from_ptr(self.ptr.add(0x0800usize + n * 16usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie {
    ptr: *mut u8,
}
unsafe impl Send for Ie {}
unsafe impl Sync for Ie {}
impl Ie {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt enable value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::IeValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO interrupt enable set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::IeSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO interrupt enable clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::IeClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO interrupt enable toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::IeToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct If {
    ptr: *mut u8,
}
unsafe impl Send for If {}
unsafe impl Sync for If {}
impl If {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt flag value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::IfValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oe {
    ptr: *mut u8,
}
unsafe impl Send for Oe {}
unsafe impl Sync for Oe {}
impl Oe {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO direction value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::OeValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO direction set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::OeSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO direction clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::OeClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO direction toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::OeToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pd {
    ptr: *mut u8,
}
unsafe impl Send for Pd {}
unsafe impl Sync for Pd {}
impl Pd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO dual edge interrupt enable value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::PdValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO dual edge interrupt enable set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::PdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO dual edge interrupt enable clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::PdClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO dual edge interrupt enable toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::PdToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl {
    ptr: *mut u8,
}
unsafe impl Send for Pl {}
unsafe impl Sync for Pl {}
impl Pl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt polarity value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::PlValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO interrupt polarity set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::PlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO interrupt polarity clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::PlClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO interrupt polarity toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::PlToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tp {
    ptr: *mut u8,
}
unsafe impl Send for Tp {}
unsafe impl Sync for Tp {}
impl Tp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt type value."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::TpValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO interrupt type set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::TpSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO interrupt type clear."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::TpClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO interrupt type toggle."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::TpToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "GPIO interrupt asynchronous clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsClear(pub u32);
    impl AsClear {
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub const fn irq_async(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub fn set_irq_async(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AsClear {
        #[inline(always)]
        fn default() -> AsClear {
            AsClear(0)
        }
    }
    #[doc = "GPIO interrupt asynchronous set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsSet(pub u32);
    impl AsSet {
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub const fn irq_async(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub fn set_irq_async(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AsSet {
        #[inline(always)]
        fn default() -> AsSet {
            AsSet(0)
        }
    }
    #[doc = "GPIO interrupt asynchronous toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsToggle(pub u32);
    impl AsToggle {
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub const fn irq_async(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub fn set_irq_async(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AsToggle {
        #[inline(always)]
        fn default() -> AsToggle {
            AsToggle(0)
        }
    }
    #[doc = "GPIO interrupt asynchronous value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsValue(pub u32);
    impl AsValue {
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub const fn irq_async(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise."]
        #[inline(always)]
        pub fn set_irq_async(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AsValue {
        #[inline(always)]
        fn default() -> AsValue {
            AsValue(0)
        }
    }
    #[doc = "GPIO input value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DiValue(pub u32);
    impl DiValue {
        #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin."]
        #[inline(always)]
        pub const fn input(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin."]
        #[inline(always)]
        pub fn set_input(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DiValue {
        #[inline(always)]
        fn default() -> DiValue {
            DiValue(0)
        }
    }
    #[doc = "GPIO output clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DoClear(pub u32);
    impl DoClear {
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub const fn output(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub fn set_output(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DoClear {
        #[inline(always)]
        fn default() -> DoClear {
            DoClear(0)
        }
    }
    #[doc = "GPIO output set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DoSet(pub u32);
    impl DoSet {
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub const fn output(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub fn set_output(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DoSet {
        #[inline(always)]
        fn default() -> DoSet {
            DoSet(0)
        }
    }
    #[doc = "GPIO output toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DoToggle(pub u32);
    impl DoToggle {
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub const fn output(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub fn set_output(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DoToggle {
        #[inline(always)]
        fn default() -> DoToggle {
            DoToggle(0)
        }
    }
    #[doc = "GPIO output value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DoValue(pub u32);
    impl DoValue {
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub const fn output(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output."]
        #[inline(always)]
        pub fn set_output(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DoValue {
        #[inline(always)]
        fn default() -> DoValue {
            DoValue(0)
        }
    }
    #[doc = "GPIO interrupt enable clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IeClear(pub u32);
    impl IeClear {
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub const fn irq_en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IeClear {
        #[inline(always)]
        fn default() -> IeClear {
            IeClear(0)
        }
    }
    #[doc = "GPIO interrupt enable set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IeSet(pub u32);
    impl IeSet {
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub const fn irq_en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IeSet {
        #[inline(always)]
        fn default() -> IeSet {
            IeSet(0)
        }
    }
    #[doc = "GPIO interrupt enable toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IeToggle(pub u32);
    impl IeToggle {
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub const fn irq_en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IeToggle {
        #[inline(always)]
        fn default() -> IeToggle {
            IeToggle(0)
        }
    }
    #[doc = "GPIO interrupt enable value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IeValue(pub u32);
    impl IeValue {
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub const fn irq_en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IeValue {
        #[inline(always)]
        fn default() -> IeValue {
            IeValue(0)
        }
    }
    #[doc = "GPIO interrupt flag value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IfValue(pub u32);
    impl IfValue {
        #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending."]
        #[inline(always)]
        pub const fn irq_flag(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending."]
        #[inline(always)]
        pub fn set_irq_flag(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IfValue {
        #[inline(always)]
        fn default() -> IfValue {
            IfValue(0)
        }
    }
    #[doc = "GPIO direction clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OeClear(pub u32);
    impl OeClear {
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub const fn direction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub fn set_direction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OeClear {
        #[inline(always)]
        fn default() -> OeClear {
            OeClear(0)
        }
    }
    #[doc = "GPIO direction set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OeSet(pub u32);
    impl OeSet {
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub const fn direction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub fn set_direction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OeSet {
        #[inline(always)]
        fn default() -> OeSet {
            OeSet(0)
        }
    }
    #[doc = "GPIO direction toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OeToggle(pub u32);
    impl OeToggle {
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub const fn direction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub fn set_direction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OeToggle {
        #[inline(always)]
        fn default() -> OeToggle {
            OeToggle(0)
        }
    }
    #[doc = "GPIO direction value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OeValue(pub u32);
    impl OeValue {
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub const fn direction(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output."]
        #[inline(always)]
        pub fn set_direction(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OeValue {
        #[inline(always)]
        fn default() -> OeValue {
            OeValue(0)
        }
    }
    #[doc = "GPIO dual edge interrupt enable clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdClear(pub u32);
    impl PdClear {
        #[doc = "GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable."]
        #[inline(always)]
        pub const fn irq_dual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable."]
        #[inline(always)]
        pub fn set_irq_dual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PdClear {
        #[inline(always)]
        fn default() -> PdClear {
            PdClear(0)
        }
    }
    #[doc = "GPIO dual edge interrupt enable set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdSet(pub u32);
    impl PdSet {
        #[doc = "GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable."]
        #[inline(always)]
        pub const fn irq_dual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable."]
        #[inline(always)]
        pub fn set_irq_dual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PdSet {
        #[inline(always)]
        fn default() -> PdSet {
            PdSet(0)
        }
    }
    #[doc = "GPIO dual edge interrupt enable toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdToggle(pub u32);
    impl PdToggle {
        #[doc = "GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one."]
        #[inline(always)]
        pub const fn irq_dual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one."]
        #[inline(always)]
        pub fn set_irq_dual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PdToggle {
        #[inline(always)]
        fn default() -> PdToggle {
            PdToggle(0)
        }
    }
    #[doc = "GPIO dual edge interrupt enable value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdValue(pub u32);
    impl PdValue {
        #[doc = "GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable."]
        #[inline(always)]
        pub const fn irq_dual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable."]
        #[inline(always)]
        pub fn set_irq_dual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PdValue {
        #[inline(always)]
        fn default() -> PdValue {
            PdValue(0)
        }
    }
    #[doc = "GPIO interrupt polarity clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlClear(pub u32);
    impl PlClear {
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub const fn irq_pol(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub fn set_irq_pol(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PlClear {
        #[inline(always)]
        fn default() -> PlClear {
            PlClear(0)
        }
    }
    #[doc = "GPIO interrupt polarity set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlSet(pub u32);
    impl PlSet {
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub const fn irq_pol(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub fn set_irq_pol(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PlSet {
        #[inline(always)]
        fn default() -> PlSet {
            PlSet(0)
        }
    }
    #[doc = "GPIO interrupt polarity toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlToggle(pub u32);
    impl PlToggle {
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub const fn irq_pol(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub fn set_irq_pol(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PlToggle {
        #[inline(always)]
        fn default() -> PlToggle {
            PlToggle(0)
        }
    }
    #[doc = "GPIO interrupt polarity value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PlValue(pub u32);
    impl PlValue {
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub const fn irq_pol(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge."]
        #[inline(always)]
        pub fn set_irq_pol(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PlValue {
        #[inline(always)]
        fn default() -> PlValue {
            PlValue(0)
        }
    }
    #[doc = "GPIO interrupt type clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpClear(pub u32);
    impl TpClear {
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub const fn irq_type(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub fn set_irq_type(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpClear {
        #[inline(always)]
        fn default() -> TpClear {
            TpClear(0)
        }
    }
    #[doc = "GPIO interrupt type set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpSet(pub u32);
    impl TpSet {
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub const fn irq_type(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub fn set_irq_type(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpSet {
        #[inline(always)]
        fn default() -> TpSet {
            TpSet(0)
        }
    }
    #[doc = "GPIO interrupt type toggle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpToggle(pub u32);
    impl TpToggle {
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub const fn irq_type(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub fn set_irq_type(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpToggle {
        #[inline(always)]
        fn default() -> TpToggle {
            TpToggle(0)
        }
    }
    #[doc = "GPIO interrupt type value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TpValue(pub u32);
    impl TpValue {
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub const fn irq_type(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge."]
        #[inline(always)]
        pub fn set_irq_type(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TpValue {
        #[inline(always)]
        fn default() -> TpValue {
            TpValue(0)
        }
    }
}
