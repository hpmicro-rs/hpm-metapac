#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PLB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plb {
    ptr: *mut u8,
}
unsafe impl Send for Plb {}
unsafe impl Sync for Plb {}
impl Plb {
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
    pub const fn type_a(self, n: usize) -> TypeA {
        assert!(n < 4usize);
        unsafe { TypeA::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn type_b(self, n: usize) -> TypeB {
        assert!(n < 4usize);
        unsafe { TypeB::from_ptr(self.ptr.add(0x0400usize + n * 32usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TypeA {
    ptr: *mut u8,
}
unsafe impl Send for TypeA {}
unsafe impl Sync for TypeA {}
impl TypeA {
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
    pub const fn lookup_table(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LookupTable, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "TYPE A CHN&index0 software inject."]
    #[inline(always)]
    pub const fn sw_inject(self) -> crate::common::Reg<regs::TypeASwInject, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TypeB {
    ptr: *mut u8,
}
unsafe impl Send for TypeB {}
unsafe impl Sync for TypeB {}
impl TypeB {
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
    pub const fn lut(self, n: usize) -> crate::common::Reg<regs::Lut, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "TYPE B CHN&index0 mode ctrl."]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TYPE B CHN&index0 software inject."]
    #[inline(always)]
    pub const fn sw_inject(self) -> crate::common::Reg<regs::TypeBSwInject, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "cmp value, using as data unit operation."]
        #[inline(always)]
        pub const fn cmp_value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cmp value, using as data unit operation."]
        #[inline(always)]
        pub fn set_cmp_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LookupTable(pub u32);
    impl LookupTable {
        #[doc = "using 4 bit trig_in as lookup index. software can program this register as trig_in's true table."]
        #[inline(always)]
        pub const fn lookup_table(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "using 4 bit trig_in as lookup index. software can program this register as trig_in's true table."]
        #[inline(always)]
        pub fn set_lookup_table(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LookupTable {
        #[inline(always)]
        fn default() -> LookupTable {
            LookupTable(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lut(pub u32);
    impl Lut {
        #[doc = "lut0 and lut1 union as 64bit, consider each 4bit as one slice. then, total 16 slice. slice0 as bit3:0, slice1 as bit7:4...etc. using 4bit trig in as index of slice. the operate sel in data unit of type B channle is decided by which slice value choosed by trig_in."]
        #[inline(always)]
        pub const fn lookup_table(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "lut0 and lut1 union as 64bit, consider each 4bit as one slice. then, total 16 slice. slice0 as bit3:0, slice1 as bit7:4...etc. using 4bit trig in as index of slice. the operate sel in data unit of type B channle is decided by which slice value choosed by trig_in."]
        #[inline(always)]
        pub fn set_lookup_table(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lut {
        #[inline(always)]
        fn default() -> Lut {
            Lut(0)
        }
    }
    #[doc = "TYPE B CHN&index0 mode ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mode(pub u32);
    impl Mode {
        #[doc = "trig out 0 output type in current channel."]
        #[inline(always)]
        pub const fn out0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "trig out 0 output type in current channel."]
        #[inline(always)]
        pub fn set_out0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "trig out 1 output type in current channel."]
        #[inline(always)]
        pub const fn out1_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "trig out 1 output type in current channel."]
        #[inline(always)]
        pub fn set_out1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "trig out 2 output type in current channel."]
        #[inline(always)]
        pub const fn out2_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "trig out 2 output type in current channel."]
        #[inline(always)]
        pub fn set_out2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "trig out 3 output type in current channel."]
        #[inline(always)]
        pub const fn out3_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "trig out 3 output type in current channel."]
        #[inline(always)]
        pub fn set_out3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "operation selection in data unit."]
        #[inline(always)]
        pub const fn opt_sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "operation selection in data unit."]
        #[inline(always)]
        pub fn set_opt_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mode {
        #[inline(always)]
        fn default() -> Mode {
            Mode(0)
        }
    }
    #[doc = "TYPE A CHN&index0 software inject."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TypeASwInject(pub u32);
    impl TypeASwInject {
        #[doc = "software can inject value to TYPEA's output."]
        #[inline(always)]
        pub const fn sw_inject(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "software can inject value to TYPEA's output."]
        #[inline(always)]
        pub fn set_sw_inject(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for TypeASwInject {
        #[inline(always)]
        fn default() -> TypeASwInject {
            TypeASwInject(0)
        }
    }
    #[doc = "TYPE B CHN&index0 software inject."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TypeBSwInject(pub u32);
    impl TypeBSwInject {
        #[doc = "data unit value can be changed if program this register."]
        #[inline(always)]
        pub const fn software_inject(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "data unit value can be changed if program this register."]
        #[inline(always)]
        pub fn set_software_inject(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TypeBSwInject {
        #[inline(always)]
        fn default() -> TypeBSwInject {
            TypeBSwInject(0)
        }
    }
}
