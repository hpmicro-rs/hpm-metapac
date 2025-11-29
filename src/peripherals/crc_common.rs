#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chn {
    ptr: *mut u8,
}
unsafe impl Send for Chn {}
unsafe impl Sync for Chn {}
impl Chn {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "&index0 pre set for crc setting."]
    #[inline(always)]
    pub const fn pre_set(self) -> crate::common::Reg<regs::PreSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "chn&index0 clear crc result and setting."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "chn&index0 poly."]
    #[inline(always)]
    pub const fn poly(self) -> crate::common::Reg<regs::Poly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "chn&index0 init_data."]
    #[inline(always)]
    pub const fn init_data(self) -> crate::common::Reg<regs::InitData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "chn&index0 xorout."]
    #[inline(always)]
    pub const fn xorout(self) -> crate::common::Reg<regs::Xorout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "chn&index0 misc_setting."]
    #[inline(always)]
    pub const fn misc_setting(self) -> crate::common::Reg<regs::MiscSetting, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "chn&index0 data."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "chn&index0 result."]
    #[inline(always)]
    pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "CRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
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
    pub const fn chn(self, n: usize) -> Chn {
        assert!(n < 8usize);
        unsafe { Chn::from_ptr(self.ptr.wrapping_add(0x0usize + n * 64usize) as _) }
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
    #[doc = "chn&index0 clear crc result and setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "write 1 to clr crc setting and result for its channel. always read 0."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "write 1 to clr crc setting and result for its channel. always read 0."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    impl core::fmt::Debug for Clr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clr").field("clr", &self.clr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clr {{ clr: {=bool:?} }}", self.clr())
        }
    }
    #[doc = "chn&index0 data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "data for crc."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "data for crc."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "chn&index0 init_data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InitData(pub u32);
    impl InitData {
        #[doc = "initial data of CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn init_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "initial data of CRC."]
        #[inline(always)]
        pub const fn set_init_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for InitData {
        #[inline(always)]
        fn default() -> InitData {
            InitData(0)
        }
    }
    impl core::fmt::Debug for InitData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InitData")
                .field("init_data", &self.init_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InitData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "InitData {{ init_data: {=u32:?} }}", self.init_data())
        }
    }
    #[doc = "chn&index0 misc_setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscSetting(pub u32);
    impl MiscSetting {
        #[doc = "crc data length."]
        #[must_use]
        #[inline(always)]
        pub const fn poly_width(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "crc data length."]
        #[inline(always)]
        pub const fn set_poly_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "0: no wrap input bit order 1: wrap input bit order."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_in(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0: no wrap input bit order 1: wrap input bit order."]
        #[inline(always)]
        pub const fn set_rev_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0: no wrap output bit order 1: wrap output bit order."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_out(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "0: no wrap output bit order 1: wrap output bit order."]
        #[inline(always)]
        pub const fn set_rev_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "0: no wrap input byte order 1: wrap input byte order."]
        #[must_use]
        #[inline(always)]
        pub const fn byte_rev(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "0: no wrap input byte order 1: wrap input byte order."]
        #[inline(always)]
        pub const fn set_byte_rev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for MiscSetting {
        #[inline(always)]
        fn default() -> MiscSetting {
            MiscSetting(0)
        }
    }
    impl core::fmt::Debug for MiscSetting {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiscSetting")
                .field("poly_width", &self.poly_width())
                .field("rev_in", &self.rev_in())
                .field("rev_out", &self.rev_out())
                .field("byte_rev", &self.byte_rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiscSetting {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MiscSetting {{ poly_width: {=u8:?}, rev_in: {=bool:?}, rev_out: {=bool:?}, byte_rev: {=bool:?} }}" , self . poly_width () , self . rev_in () , self . rev_out () , self . byte_rev ())
        }
    }
    #[doc = "chn&index0 poly."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Poly(pub u32);
    impl Poly {
        #[doc = "poly setting."]
        #[must_use]
        #[inline(always)]
        pub const fn poly(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "poly setting."]
        #[inline(always)]
        pub const fn set_poly(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Poly {
        #[inline(always)]
        fn default() -> Poly {
            Poly(0)
        }
    }
    impl core::fmt::Debug for Poly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poly").field("poly", &self.poly()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Poly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Poly {{ poly: {=u32:?} }}", self.poly())
        }
    }
    #[doc = "&index0 pre set for crc setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PreSet(pub u32);
    impl PreSet {
        #[doc = "0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb."]
        #[must_use]
        #[inline(always)]
        pub const fn pre_set(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb."]
        #[inline(always)]
        pub const fn set_pre_set(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PreSet {
        #[inline(always)]
        fn default() -> PreSet {
            PreSet(0)
        }
    }
    impl core::fmt::Debug for PreSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PreSet")
                .field("pre_set", &self.pre_set())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PreSet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PreSet {{ pre_set: {=u8:?} }}", self.pre_set())
        }
    }
    #[doc = "chn&index0 result."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Result(pub u32);
    impl Result {
        #[doc = "crc result."]
        #[must_use]
        #[inline(always)]
        pub const fn result(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "crc result."]
        #[inline(always)]
        pub const fn set_result(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Result {
        #[inline(always)]
        fn default() -> Result {
            Result(0)
        }
    }
    impl core::fmt::Debug for Result {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Result")
                .field("result", &self.result())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Result {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Result {{ result: {=u32:?} }}", self.result())
        }
    }
    #[doc = "chn&index0 xorout."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xorout(pub u32);
    impl Xorout {
        #[doc = "XOR for CRC result."]
        #[must_use]
        #[inline(always)]
        pub const fn xorout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "XOR for CRC result."]
        #[inline(always)]
        pub const fn set_xorout(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Xorout {
        #[inline(always)]
        fn default() -> Xorout {
            Xorout(0)
        }
    }
    impl core::fmt::Debug for Xorout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xorout")
                .field("xorout", &self.xorout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xorout {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Xorout {{ xorout: {=u32:?} }}", self.xorout())
        }
    }
}
