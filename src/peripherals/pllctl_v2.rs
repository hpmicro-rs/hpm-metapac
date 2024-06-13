#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll {
    ptr: *mut u8,
}
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PLL0 multiple register."]
    #[inline(always)]
    pub const fn mfi(self) -> crate::common::Reg<regs::Mfi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PLL0 fraction numerator register."]
    #[inline(always)]
    pub const fn mfn(self) -> crate::common::Reg<regs::Mfn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PLL0 fraction demoninator register."]
    #[inline(always)]
    pub const fn mfd(self) -> crate::common::Reg<regs::Mfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PLL0 spread spectrum step register."]
    #[inline(always)]
    pub const fn ss_step(self) -> crate::common::Reg<regs::SsStep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PLL0 spread spectrum stop register."]
    #[inline(always)]
    pub const fn ss_stop(self) -> crate::common::Reg<regs::SsStop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PLL0 confguration register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PLL0 lock time register."]
    #[inline(always)]
    pub const fn locktime(self) -> crate::common::Reg<regs::Locktime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PLL0 step time register."]
    #[inline(always)]
    pub const fn steptime(self) -> crate::common::Reg<regs::Steptime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PLL0 advance configuration register."]
    #[inline(always)]
    pub const fn advanced(self) -> crate::common::Reg<regs::Advanced, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn div(self, n: usize) -> crate::common::Reg<regs::Div, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
}
#[doc = "PLLCTLV2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllctlv2 {
    ptr: *mut u8,
}
unsafe impl Send for Pllctlv2 {}
unsafe impl Sync for Pllctlv2 {}
impl Pllctlv2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OSC configuration."]
    #[inline(always)]
    pub const fn xtal(self) -> crate::common::Reg<regs::Xtal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pll(self, n: usize) -> Pll {
        assert!(n < 5usize);
        unsafe { Pll::from_ptr(self.ptr.add(0x80usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "PLL0 advance configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Advanced(pub u32);
    impl Advanced {
        #[doc = "Enable dither function."]
        #[inline(always)]
        pub const fn dither(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable dither function."]
        #[inline(always)]
        pub fn set_dither(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us."]
        #[inline(always)]
        pub const fn slow(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us."]
        #[inline(always)]
        pub fn set_slow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Advanced {
        #[inline(always)]
        fn default() -> Advanced {
            Advanced(0)
        }
    }
    #[doc = "PLL0 confguration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M."]
        #[inline(always)]
        pub const fn refsel(&self) -> super::vals::Refsel {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Refsel::from_bits(val as u8)
        }
        #[doc = "Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M."]
        #[inline(always)]
        pub fn set_refsel(&mut self, val: super::vals::Refsel) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable spread spectrum function. This field supports changing during PLL running."]
        #[inline(always)]
        pub const fn spread(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable spread spectrum function. This field supports changing during PLL running."]
        #[inline(always)]
        pub fn set_spread(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div(pub u32);
    impl Div {
        #[doc = "Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Divider enable status 0: Divider is off 1: Divider is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Divider enable status 0: Divider is off 1: Divider is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Divider response status 0: Divider is not stable 1: Divider is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Divider response status 0: Divider is not stable 1: Divider is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Div {
        #[inline(always)]
        fn default() -> Div {
            Div(0)
        }
    }
    #[doc = "PLL0 lock time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Locktime(pub u32);
    impl Locktime {
        #[doc = "Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
        #[inline(always)]
        pub const fn locktime(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
        #[inline(always)]
        pub fn set_locktime(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Locktime {
        #[inline(always)]
        fn default() -> Locktime {
            Locktime(0)
        }
    }
    #[doc = "PLL0 fraction demoninator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfd(pub u32);
    impl Mfd {
        #[doc = "Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
        #[inline(always)]
        pub const fn mfd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled."]
        #[inline(always)]
        pub fn set_mfd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for Mfd {
        #[inline(always)]
        fn default() -> Mfd {
            Mfd(0)
        }
    }
    #[doc = "PLL0 multiple register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfi(pub u32);
    impl Mfi {
        #[doc = "loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid."]
        #[inline(always)]
        pub const fn mfi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid."]
        #[inline(always)]
        pub fn set_mfi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "PLL enable status 0: PLL is off 1: PLL is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable status 0: PLL is off 1: PLL is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PLL status 0: PLL is not stable 1: PLL is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLL status 0: PLL is not stable 1: PLL is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: PLL is stable or shutdown 1: PLL is changing status."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: PLL is stable or shutdown 1: PLL is changing status."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mfi {
        #[inline(always)]
        fn default() -> Mfi {
            Mfi(0)
        }
    }
    #[doc = "PLL0 fraction numerator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mfn(pub u32);
    impl Mfn {
        #[doc = "Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
        #[inline(always)]
        pub const fn mfn(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running."]
        #[inline(always)]
        pub fn set_mfn(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for Mfn {
        #[inline(always)]
        fn default() -> Mfn {
            Mfn(0)
        }
    }
    #[doc = "PLL0 spread spectrum step register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SsStep(pub u32);
    impl SsStep {
        #[doc = "Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
        #[inline(always)]
        pub const fn step(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
        #[inline(always)]
        pub fn set_step(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for SsStep {
        #[inline(always)]
        fn default() -> SsStep {
            SsStep(0)
        }
    }
    #[doc = "PLL0 spread spectrum stop register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SsStop(pub u32);
    impl SsStop {
        #[doc = "Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
        #[inline(always)]
        pub const fn stop(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for SsStop {
        #[inline(always)]
        fn default() -> SsStop {
            SsStop(0)
        }
    }
    #[doc = "PLL0 step time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Steptime(pub u32);
    impl Steptime {
        #[doc = "Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
        #[inline(always)]
        pub const fn steptime(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500."]
        #[inline(always)]
        pub fn set_steptime(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Steptime {
        #[inline(always)]
        fn default() -> Steptime {
            Steptime(0)
        }
    }
    #[doc = "OSC configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal(pub u32);
    impl Xtal {
        #[doc = "Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[inline(always)]
        pub const fn ramp_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[inline(always)]
        pub fn set_ramp_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Xtal {
        #[inline(always)]
        fn default() -> Xtal {
            Xtal(0)
        }
    }
}
pub mod vals {
    #[doc = "Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Refsel {
        #[doc = "XTAL24M"]
        XTAL24M = 0x0,
        #[doc = "IRC24M"]
        IRC24M = 0x01,
    }
    impl Refsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Refsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Refsel {
        #[inline(always)]
        fn from(val: u8) -> Refsel {
            Refsel::from_bits(val)
        }
    }
    impl From<Refsel> for u8 {
        #[inline(always)]
        fn from(val: Refsel) -> u8 {
            Refsel::to_bits(val)
        }
    }
}
