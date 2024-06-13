#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg {
    ptr: *mut u8,
}
unsafe impl Send for Cfg {}
unsafe impl Sync for Cfg {}
impl Cfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "OPAMP0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp {
    ptr: *mut u8,
}
unsafe impl Send for Opamp {}
unsafe impl Sync for Opamp {}
impl Opamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control reg."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "status reg."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control reg1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cfg(self, n: usize) -> Cfg {
        assert!(n < 8usize);
        unsafe { Cfg::from_ptr(self.ptr.add(0x10usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vip_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vip_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vim_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vim_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn miller_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_miller_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn disable_pm_cap(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_disable_pm_cap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn gain_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_gain_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vbypass_lv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vbypass_lv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn en_lv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_en_lv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to enable hardware trigger from moto system. NOTE: when sw_preset is enabled, this bit will not take effert."]
        #[inline(always)]
        pub const fn hw_trig_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable hardware trigger from moto system. NOTE: when sw_preset is enabled, this bit will not take effert."]
        #[inline(always)]
        pub fn set_hw_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn channel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_channel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    #[doc = "control reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vip_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vip_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vbypass(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vbypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn miller_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_miller_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn disable_pm_cap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_disable_pm_cap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn gain_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_gain_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn vim_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_vim_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn en_lv(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_en_lv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    #[doc = "control reg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sw_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sw_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "set to use preset defined by sw_sel. NOTE: when set, the hardware trigger will not be used."]
        #[inline(always)]
        pub const fn sw_preset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to use preset defined by sw_sel. NOTE: when set, the hardware trigger will not be used."]
        #[inline(always)]
        pub fn set_sw_preset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    #[doc = "status reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "current selected preset."]
        #[inline(always)]
        pub const fn cur_preset(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "current selected preset."]
        #[inline(always)]
        pub fn set_cur_preset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "1 for preset active; one of cur_preset is selected for OPAMP; 0 for no preset, OPAMP use cfg0 parameters."]
        #[inline(always)]
        pub const fn preset_act(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "1 for preset active; one of cur_preset is selected for OPAMP; 0 for no preset, OPAMP use cfg0 parameters."]
        #[inline(always)]
        pub fn set_preset_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "if more than one hardware trigger is set, will put all trigger input here; write any value to clear."]
        #[inline(always)]
        pub const fn trig_conflict(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "if more than one hardware trigger is set, will put all trigger input here; write any value to clear."]
        #[inline(always)]
        pub fn set_trig_conflict(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
}
