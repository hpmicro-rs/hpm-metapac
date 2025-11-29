#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OTP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otp {
    ptr: *mut u8,
}
unsafe impl Send for Otp {}
unsafe impl Sync for Otp {}
impl Otp {
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
    pub const fn shadow(self, n: usize) -> crate::common::Reg<regs::Shadow, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn shadow_lock(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ShadowLock, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn fuse(self, n: usize) -> crate::common::Reg<regs::Fuse, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn fuse_lock(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FuseLock, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
        }
    }
    #[doc = "UNLOCK."]
    #[inline(always)]
    pub const fn unlock(self) -> crate::common::Reg<regs::Unlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "DATA."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "ADDR."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "CMD."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "LOAD Request."]
    #[inline(always)]
    pub const fn load_req(self) -> crate::common::Reg<regs::LoadReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "LOAD complete."]
    #[inline(always)]
    pub const fn load_comp(self) -> crate::common::Reg<regs::LoadComp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn region(self, n: usize) -> crate::common::Reg<regs::Region, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a20usize + n * 4usize) as _)
        }
    }
    #[doc = "interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
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
    #[doc = "ADDR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "word address to be read or write."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "word address to be read or write."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addr {{ addr: {=u8:?} }}", self.addr())
        }
    }
    #[doc = "CMD."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "command to access fure array \"BLOW\" will update fuse word at ADDR to value hold in DATA \"READ\" will fetch fuse value in at ADDR to DATA register."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "command to access fure array \"BLOW\" will update fuse word at ADDR to value hold in DATA \"READ\" will fetch fuse value in at ADDR to DATA register."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd").field("cmd", &self.cmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmd {{ cmd: {=u32:?} }}", self.cmd())
        }
    }
    #[doc = "DATA."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "data register for non-blocking access this register hold dat read from fuse array or data to by programmed to fuse array."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "data register for non-blocking access this register hold dat read from fuse array or data to by programmed to fuse array."]
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
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fuse(pub u32);
    impl Fuse {
        #[doc = "fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)."]
        #[must_use]
        #[inline(always)]
        pub const fn fuse(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)."]
        #[inline(always)]
        pub const fn set_fuse(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fuse {
        #[inline(always)]
        fn default() -> Fuse {
            Fuse(0)
        }
    }
    impl core::fmt::Debug for Fuse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fuse").field("fuse", &self.fuse()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fuse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fuse {{ fuse: {=u32:?} }}", self.fuse())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FuseLock(pub u32);
    impl FuseLock {
        #[doc = "lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FuseLock {
        #[inline(always)]
        fn default() -> FuseLock {
            FuseLock(0)
        }
    }
    impl core::fmt::Debug for FuseLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FuseLock")
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FuseLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FuseLock {{ lock: {=u32:?} }}", self.lock())
        }
    }
    #[doc = "interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable."]
        #[must_use]
        #[inline(always)]
        pub const fn load(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable."]
        #[inline(always)]
        pub const fn set_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable."]
        #[must_use]
        #[inline(always)]
        pub const fn read(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable."]
        #[inline(always)]
        pub const fn set_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable."]
        #[must_use]
        #[inline(always)]
        pub const fn write(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable."]
        #[inline(always)]
        pub const fn set_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
                .field("load", &self.load())
                .field("read", &self.read())
                .field("write", &self.write())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntEn {{ load: {=bool:?}, read: {=bool:?}, write: {=bool:?} }}",
                self.load(),
                self.read(),
                self.write()
            )
        }
    }
    #[doc = "interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u32);
    impl IntFlag {
        #[doc = "fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded."]
        #[must_use]
        #[inline(always)]
        pub const fn load(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded."]
        #[inline(always)]
        pub const fn set_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register."]
        #[must_use]
        #[inline(always)]
        pub const fn read(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register."]
        #[inline(always)]
        pub const fn set_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse."]
        #[must_use]
        #[inline(always)]
        pub const fn write(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse."]
        #[inline(always)]
        pub const fn set_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for IntFlag {
        #[inline(always)]
        fn default() -> IntFlag {
            IntFlag(0)
        }
    }
    impl core::fmt::Debug for IntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntFlag")
                .field("load", &self.load())
                .field("read", &self.read())
                .field("write", &self.write())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntFlag {{ load: {=bool:?}, read: {=bool:?}, write: {=bool:?} }}",
                self.load(),
                self.read(),
                self.write()
            )
        }
    }
    #[doc = "LOAD complete."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LoadComp(pub u32);
    impl LoadComp {
        #[doc = "reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3."]
        #[must_use]
        #[inline(always)]
        pub const fn complete(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3."]
        #[inline(always)]
        pub const fn set_complete(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for LoadComp {
        #[inline(always)]
        fn default() -> LoadComp {
            LoadComp(0)
        }
    }
    impl core::fmt::Debug for LoadComp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LoadComp")
                .field("complete", &self.complete())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LoadComp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LoadComp {{ complete: {=u8:?} }}", self.complete())
        }
    }
    #[doc = "LOAD Request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LoadReq(pub u32);
    impl LoadReq {
        #[doc = "reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3."]
        #[must_use]
        #[inline(always)]
        pub const fn request(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3."]
        #[inline(always)]
        pub const fn set_request(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for LoadReq {
        #[inline(always)]
        fn default() -> LoadReq {
            LoadReq(0)
        }
    }
    impl core::fmt::Debug for LoadReq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LoadReq")
                .field("request", &self.request())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LoadReq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LoadReq {{ request: {=u8:?} }}", self.request())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Region(pub u32);
    impl Region {
        #[doc = "start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for Region {
        #[inline(always)]
        fn default() -> Region {
            Region(0)
        }
    }
    impl core::fmt::Debug for Region {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Region")
                .field("start", &self.start())
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Region {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Region {{ start: {=u8:?}, stop: {=u8:?} }}",
                self.start(),
                self.stop()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shadow(pub u32);
    impl Shadow {
        #[doc = "shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128."]
        #[must_use]
        #[inline(always)]
        pub const fn shadow(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128."]
        #[inline(always)]
        pub const fn set_shadow(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Shadow {
        #[inline(always)]
        fn default() -> Shadow {
            Shadow(0)
        }
    }
    impl core::fmt::Debug for Shadow {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shadow")
                .field("shadow", &self.shadow())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shadow {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Shadow {{ shadow: {=u32:?} }}", self.shadow())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShadowLock(pub u32);
    impl ShadowLock {
        #[doc = "lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ShadowLock {
        #[inline(always)]
        fn default() -> ShadowLock {
            ShadowLock(0)
        }
    }
    impl core::fmt::Debug for ShadowLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ShadowLock")
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ShadowLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ShadowLock {{ lock: {=u32:?} }}", self.lock())
        }
    }
    #[doc = "UNLOCK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Unlock(pub u32);
    impl Unlock {
        #[doc = "unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly."]
        #[must_use]
        #[inline(always)]
        pub const fn unlock(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly."]
        #[inline(always)]
        pub const fn set_unlock(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Unlock {
        #[inline(always)]
        fn default() -> Unlock {
            Unlock(0)
        }
    }
    impl core::fmt::Debug for Unlock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Unlock")
                .field("unlock", &self.unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Unlock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Unlock {{ unlock: {=u32:?} }}", self.unlock())
        }
    }
}
