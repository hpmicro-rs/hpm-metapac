#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PSEC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psec {
    ptr: *mut u8,
}
unsafe impl Send for Psec {}
unsafe impl Sync for Psec {}
impl Psec {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "secure state configuration."]
    #[inline(always)]
    pub const fn secure_state_config(
        self,
    ) -> crate::common::Reg<regs::SecureStateConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Security violation config."]
    #[inline(always)]
    pub const fn violation_config(
        self,
    ) -> crate::common::Reg<regs::ViolationConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Escalate behavior on security event."]
    #[inline(always)]
    pub const fn escalate_config(
        self,
    ) -> crate::common::Reg<regs::EscalateConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Event and escalate status."]
    #[inline(always)]
    pub const fn event(self) -> crate::common::Reg<regs::Event, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Lifecycle."]
    #[inline(always)]
    pub const fn lifecycle(self) -> crate::common::Reg<regs::Lifecycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "Escalate behavior on security event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscalateConfig(pub u32);
    impl EscalateConfig {
        #[doc = "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub const fn sec_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub fn set_sec_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn lock_sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub fn set_lock_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub const fn nsc_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate."]
        #[inline(always)]
        pub fn set_nsc_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[doc = "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn lock_nsc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub fn set_lock_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EscalateConfig {
        #[inline(always)]
        fn default() -> EscalateConfig {
            EscalateConfig(0)
        }
    }
    #[doc = "Event and escalate status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Event(pub u32);
    impl Event {
        #[doc = "PMIC is escalting secure event."]
        #[inline(always)]
        pub const fn pmic_esc_sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC is escalting secure event."]
        #[inline(always)]
        pub fn set_pmic_esc_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PMIC is escalating non-secure event."]
        #[inline(always)]
        pub const fn pmic_esc_nsc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC is escalating non-secure event."]
        #[inline(always)]
        pub fn set_pmic_esc_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "local event statue, each bit represents one security event."]
        #[inline(always)]
        pub const fn event(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "local event statue, each bit represents one security event."]
        #[inline(always)]
        pub fn set_event(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Event {
        #[inline(always)]
        fn default() -> Event {
            Event(0)
        }
    }
    #[doc = "Lifecycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lifecycle(pub u32);
    impl Lifecycle {
        #[doc = "lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow."]
        #[inline(always)]
        pub const fn lifecycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow."]
        #[inline(always)]
        pub fn set_lifecycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Lifecycle {
        #[inline(always)]
        fn default() -> Lifecycle {
            Lifecycle(0)
        }
    }
    #[doc = "Secure state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecureState(pub u32);
    impl SecureState {
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state."]
        #[inline(always)]
        pub const fn pmic_ins(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state."]
        #[inline(always)]
        pub fn set_pmic_ins(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state."]
        #[inline(always)]
        pub const fn pmic_sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state."]
        #[inline(always)]
        pub fn set_pmic_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state."]
        #[inline(always)]
        pub const fn pmic_nsc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state."]
        #[inline(always)]
        pub fn set_pmic_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state."]
        #[inline(always)]
        pub const fn pmic_fail(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state."]
        #[inline(always)]
        pub fn set_pmic_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state."]
        #[inline(always)]
        pub const fn allow_sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state."]
        #[inline(always)]
        pub fn set_allow_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state."]
        #[inline(always)]
        pub const fn allow_nsc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state."]
        #[inline(always)]
        pub fn set_allow_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for SecureState {
        #[inline(always)]
        fn default() -> SecureState {
            SecureState(0)
        }
    }
    #[doc = "secure state configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecureStateConfig(pub u32);
    impl SecureStateConfig {
        #[doc = "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state."]
        #[inline(always)]
        pub const fn allow_restart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state."]
        #[inline(always)]
        pub fn set_allow_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SecureStateConfig {
        #[inline(always)]
        fn default() -> SecureStateConfig {
            SecureStateConfig(0)
        }
    }
    #[doc = "Security violation config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ViolationConfig(pub u32);
    impl ViolationConfig {
        #[doc = "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub const fn sec_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub fn set_sec_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn lock_sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub fn set_lock_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub const fn nsc_vio_cfg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation."]
        #[inline(always)]
        pub fn set_nsc_vio_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
        #[doc = "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub const fn lock_nsc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored."]
        #[inline(always)]
        pub fn set_lock_nsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ViolationConfig {
        #[inline(always)]
        fn default() -> ViolationConfig {
            ViolationConfig(0)
        }
    }
}
