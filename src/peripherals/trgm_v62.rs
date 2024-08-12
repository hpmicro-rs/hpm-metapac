#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TRGM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgm {
    ptr: *mut u8,
}
unsafe impl Send for Trgm {}
unsafe impl Sync for Trgm {}
impl Trgm {
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
    pub const fn filtcfg(self, n: usize) -> crate::common::Reg<regs::Filtcfg, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgocfg(self, n: usize) -> crate::common::Reg<regs::Trgocfg, crate::common::RW> {
        assert!(n < 68usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dmacfg(self, n: usize) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[doc = "General Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub const fn dmasrcsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub fn set_dmasrcsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtcfg(pub u32);
    impl Filtcfg {
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub const fn filtlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub fn set_filtlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::FilterMode {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::FilterMode::from_bits(val as u8)
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::FilterMode) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Filtcfg {
        #[inline(always)]
        fn default() -> Filtcfg {
            Filtcfg(0)
        }
    }
    #[doc = "General Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub const fn trgopen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub fn set_trgopen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgocfg(pub u32);
    impl Trgocfg {
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub const fn trigosel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub fn set_trigosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn redg2pen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub fn set_redg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn fedg2pen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub fn set_fedg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Trgocfg {
        #[inline(always)]
        fn default() -> Trgocfg {
            Trgocfg(0)
        }
    }
}
pub mod vals {
    #[doc = "Filter mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FilterMode {
        #[doc = "Bypass"]
        BYPASS = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Rapid change mode"]
        RAPID_CHANGE = 0x04,
        #[doc = "Delay filter mode"]
        DELAY = 0x05,
        #[doc = "Stable low mode"]
        STABLE_LOW = 0x06,
        #[doc = "Stable high mode"]
        STABLE_HIGH = 0x07,
    }
    impl FilterMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterMode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterMode {
        #[inline(always)]
        fn from(val: u8) -> FilterMode {
            FilterMode::from_bits(val)
        }
    }
    impl From<FilterMode> for u8 {
        #[inline(always)]
        fn from(val: FilterMode) -> u8 {
            FilterMode::to_bits(val)
        }
    }
}
