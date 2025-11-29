#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BSEC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsec {
    ptr: *mut u8,
}
unsafe impl Send for Bsec {}
unsafe impl Sync for Bsec {}
impl Bsec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Secure state."]
    #[inline(always)]
    pub const fn secure_state(self) -> crate::common::Reg<regs::SecureState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "secure state configuration."]
    #[inline(always)]
    pub const fn secure_state_config(
        self,
    ) -> crate::common::Reg<regs::SecureStateConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Security violation config."]
    #[inline(always)]
    pub const fn violation_config(
        self,
    ) -> crate::common::Reg<regs::ViolationConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Escalate behavior on security event."]
    #[inline(always)]
    pub const fn escalate_config(
        self,
    ) -> crate::common::Reg<regs::EscalateConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Event and escalate status."]
    #[inline(always)]
    pub const fn event(self) -> crate::common::Reg<regs::Event, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
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
    #[doc = "Escalate behavior on security event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscalateConfig(pub u32);
    impl EscalateConfig {
        #[doc = "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[must_use]
        #[inline(always)]
        pub const fn sec_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub const fn set_sec_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn set_lock_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[must_use]
        #[inline(always)]
        pub const fn nsc_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub const fn set_nsc_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[doc = "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_nsc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn set_lock_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EscalateConfig {
        #[inline(always)]
        fn default() -> EscalateConfig {
            EscalateConfig(0)
        }
    }
    impl core::fmt::Debug for EscalateConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscalateConfig")
                .field("sec_vio_cfg", &self.sec_vio_cfg())
                .field("lock_sec", &self.lock_sec())
                .field("nsc_vio_cfg", &self.nsc_vio_cfg())
                .field("lock_nsc", &self.lock_nsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscalateConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EscalateConfig {{ sec_vio_cfg: {=u16:?}, lock_sec: {=bool:?}, nsc_vio_cfg: {=u16:?}, lock_nsc: {=bool:?} }}" , self . sec_vio_cfg () , self . lock_sec () , self . nsc_vio_cfg () , self . lock_nsc ())
        }
    }
    #[doc = "Event and escalate status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Event(pub u32);
    impl Event {
        #[doc = "BATT is escalting ssecure event."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_esc_sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BATT is escalting ssecure event."]
        #[inline(always)]
        pub const fn set_batt_esc_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BATT is escalating non-secure event."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_esc_nsc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BATT is escalating non-secure event."]
        #[inline(always)]
        pub const fn set_batt_esc_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "local event statue, each bit represents one security event."]
        #[must_use]
        #[inline(always)]
        pub const fn event(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "local event statue, each bit represents one security event."]
        #[inline(always)]
        pub const fn set_event(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Event {
        #[inline(always)]
        fn default() -> Event {
            Event(0)
        }
    }
    impl core::fmt::Debug for Event {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Event")
                .field("batt_esc_sec", &self.batt_esc_sec())
                .field("batt_esc_nsc", &self.batt_esc_nsc())
                .field("event", &self.event())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Event {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Event {{ batt_esc_sec: {=bool:?}, batt_esc_nsc: {=bool:?}, event: {=u16:?} }}",
                self.batt_esc_sec(),
                self.batt_esc_nsc(),
                self.event()
            )
        }
    }
    #[doc = "Secure state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecureState(pub u32);
    impl SecureState {
        #[doc = "BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_ins(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state."]
        #[inline(always)]
        pub const fn set_batt_ins(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state."]
        #[inline(always)]
        pub const fn set_batt_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_nsc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state."]
        #[inline(always)]
        pub const fn set_batt_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state."]
        #[must_use]
        #[inline(always)]
        pub const fn batt_fail(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state."]
        #[inline(always)]
        pub const fn set_batt_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state."]
        #[must_use]
        #[inline(always)]
        pub const fn allow_sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state."]
        #[inline(always)]
        pub const fn set_allow_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state."]
        #[must_use]
        #[inline(always)]
        pub const fn allow_nsc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state."]
        #[inline(always)]
        pub const fn set_allow_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for SecureState {
        #[inline(always)]
        fn default() -> SecureState {
            SecureState(0)
        }
    }
    impl core::fmt::Debug for SecureState {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecureState")
                .field("batt_ins", &self.batt_ins())
                .field("batt_sec", &self.batt_sec())
                .field("batt_nsc", &self.batt_nsc())
                .field("batt_fail", &self.batt_fail())
                .field("allow_sec", &self.allow_sec())
                .field("allow_nsc", &self.allow_nsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecureState {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SecureState {{ batt_ins: {=bool:?}, batt_sec: {=bool:?}, batt_nsc: {=bool:?}, batt_fail: {=bool:?}, allow_sec: {=bool:?}, allow_nsc: {=bool:?} }}" , self . batt_ins () , self . batt_sec () , self . batt_nsc () , self . batt_fail () , self . allow_sec () , self . allow_nsc ())
        }
    }
    #[doc = "secure state configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecureStateConfig(pub u32);
    impl SecureStateConfig {
        #[doc = "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state."]
        #[must_use]
        #[inline(always)]
        pub const fn allow_restart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state."]
        #[inline(always)]
        pub const fn set_allow_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SecureStateConfig {
        #[inline(always)]
        fn default() -> SecureStateConfig {
            SecureStateConfig(0)
        }
    }
    impl core::fmt::Debug for SecureStateConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecureStateConfig")
                .field("allow_restart", &self.allow_restart())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecureStateConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SecureStateConfig {{ allow_restart: {=bool:?}, lock: {=bool:?} }}",
                self.allow_restart(),
                self.lock()
            )
        }
    }
    #[doc = "Security violation config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ViolationConfig(pub u32);
    impl ViolationConfig {
        #[doc = "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[must_use]
        #[inline(always)]
        pub const fn sec_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub const fn set_sec_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn set_lock_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[must_use]
        #[inline(always)]
        pub const fn nsc_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub const fn set_nsc_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[doc = "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_nsc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn set_lock_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ViolationConfig {
        #[inline(always)]
        fn default() -> ViolationConfig {
            ViolationConfig(0)
        }
    }
    impl core::fmt::Debug for ViolationConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ViolationConfig")
                .field("sec_vio_cfg", &self.sec_vio_cfg())
                .field("lock_sec", &self.lock_sec())
                .field("nsc_vio_cfg", &self.nsc_vio_cfg())
                .field("lock_nsc", &self.lock_nsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ViolationConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ViolationConfig {{ sec_vio_cfg: {=u16:?}, lock_sec: {=bool:?}, nsc_vio_cfg: {=u16:?}, lock_nsc: {=bool:?} }}" , self . sec_vio_cfg () , self . lock_sec () , self . nsc_vio_cfg () , self . lock_nsc ())
        }
    }
}
