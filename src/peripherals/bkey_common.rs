#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BKEY."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkey {
    ptr: *mut u8,
}
unsafe impl Send for Bkey {}
unsafe impl Sync for Bkey {}
impl Bkey {
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
    pub const fn key(self, n: usize) -> Key {
        assert!(n < 2usize);
        unsafe { Key::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ecc(self, n: usize) -> crate::common::Reg<regs::Ecc, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Key selection."]
    #[inline(always)]
    pub const fn select(self) -> crate::common::Reg<regs::Select, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    ptr: *mut u8,
}
unsafe impl Send for Key {}
unsafe impl Sync for Key {}
impl Key {
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
    pub const fn data(self, n: usize) -> crate::common::Reg<regs::Data, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "security key data."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "security key data."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecc(pub u32);
    impl Ecc {
        #[doc = "Parity check bits for key0."]
        #[inline(always)]
        pub const fn ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Parity check bits for key0."]
        #[inline(always)]
        pub fn set_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "read lock to key0 0: key read enable 1: key always read as 0."]
        #[inline(always)]
        pub const fn rlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "read lock to key0 0: key read enable 1: key always read as 0."]
        #[inline(always)]
        pub fn set_rlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "write lock to key0 0: write enable 1: write ignored."]
        #[inline(always)]
        pub const fn wlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "write lock to key0 0: write enable 1: write ignored."]
        #[inline(always)]
        pub fn set_wlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ecc {
        #[inline(always)]
        fn default() -> Ecc {
            Ecc(0)
        }
    }
    #[doc = "Key selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Select(pub u32);
    impl Select {
        #[doc = "select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode."]
        #[inline(always)]
        pub const fn select(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode."]
        #[inline(always)]
        pub fn set_select(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Select {
        #[inline(always)]
        fn default() -> Select {
            Select(0)
        }
    }
}
