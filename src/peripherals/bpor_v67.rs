#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BPOR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpor {
    ptr: *mut u8,
}
unsafe impl Send for Bpor {}
unsafe impl Sync for Bpor {}
impl Bpor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power on cause."]
    #[inline(always)]
    pub const fn por_cause(self) -> crate::common::Reg<regs::PorCause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power on select."]
    #[inline(always)]
    pub const fn por_select(self) -> crate::common::Reg<regs::PorSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Power on reset config."]
    #[inline(always)]
    pub const fn por_config(self) -> crate::common::Reg<regs::PorConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Power down control."]
    #[inline(always)]
    pub const fn por_control(self) -> crate::common::Reg<regs::PorControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
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
    #[doc = "Power on cause."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PorCause(pub u32);
    impl PorCause {
        #[doc = "Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO."]
        #[must_use]
        #[inline(always)]
        pub const fn cause(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO."]
        #[inline(always)]
        pub const fn set_cause(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for PorCause {
        #[inline(always)]
        fn default() -> PorCause {
            PorCause(0)
        }
    }
    impl core::fmt::Debug for PorCause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PorCause")
                .field("cause", &self.cause())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PorCause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PorCause {{ cause: {=u8:?} }}", self.cause())
        }
    }
    #[doc = "Power on reset config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PorConfig(pub u32);
    impl PorConfig {
        #[doc = "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen."]
        #[must_use]
        #[inline(always)]
        pub const fn retention(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen."]
        #[inline(always)]
        pub const fn set_retention(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PorConfig {
        #[inline(always)]
        fn default() -> PorConfig {
            PorConfig(0)
        }
    }
    impl core::fmt::Debug for PorConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PorConfig")
                .field("retention", &self.retention())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PorConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PorConfig {{ retention: {=bool:?} }}", self.retention())
        }
    }
    #[doc = "Power down control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PorControl(pub u32);
    impl PorControl {
        #[doc = "Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PorControl {
        #[inline(always)]
        fn default() -> PorControl {
            PorControl(0)
        }
    }
    impl core::fmt::Debug for PorControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PorControl")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PorControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PorControl {{ counter: {=u16:?} }}", self.counter())
        }
    }
    #[doc = "Power on select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PorSelect(pub u32);
    impl PorSelect {
        #[doc = "Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO."]
        #[must_use]
        #[inline(always)]
        pub const fn select(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO."]
        #[inline(always)]
        pub const fn set_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for PorSelect {
        #[inline(always)]
        fn default() -> PorSelect {
            PorSelect(0)
        }
    }
    impl core::fmt::Debug for PorSelect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PorSelect")
                .field("select", &self.select())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PorSelect {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PorSelect {{ select: {=u8:?} }}", self.select())
        }
    }
}
