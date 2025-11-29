#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FEMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Femc {
    ptr: *mut u8,
}
unsafe impl Send for Femc {}
unsafe impl Sync for Femc {}
impl Femc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IO Mux Control Register."]
    #[inline(always)]
    pub const fn ioctrl(self) -> crate::common::Reg<regs::Ioctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Bus (AXI) Weight Control Register 0."]
    #[inline(always)]
    pub const fn bmw0(self) -> crate::common::Reg<regs::Bmw0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Bus (AXI) Weight Control Register 1."]
    #[inline(always)]
    pub const fn bmw1(self) -> crate::common::Reg<regs::Bmw1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn br(self, n: usize) -> crate::common::Reg<regs::Br, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SDRAM Control Register 0."]
    #[inline(always)]
    pub const fn sdrctrl0(self) -> crate::common::Reg<regs::Sdrctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SDRAM Control Register 1."]
    #[inline(always)]
    pub const fn sdrctrl1(self) -> crate::common::Reg<regs::Sdrctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SDRAM Control Register 2."]
    #[inline(always)]
    pub const fn sdrctrl2(self) -> crate::common::Reg<regs::Sdrctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SDRAM Control Register 3."]
    #[inline(always)]
    pub const fn sdrctrl3(self) -> crate::common::Reg<regs::Sdrctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SRAM control register 0."]
    #[inline(always)]
    pub const fn srctrl0(self) -> crate::common::Reg<regs::Srctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SRAM control register 1."]
    #[inline(always)]
    pub const fn srctrl1(self) -> crate::common::Reg<regs::Srctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "IP Command Control Register 0."]
    #[inline(always)]
    pub const fn saddr(self) -> crate::common::Reg<regs::Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "IP Command Control Register 1."]
    #[inline(always)]
    pub const fn datsz(self) -> crate::common::Reg<regs::Datsz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Command Control Register 2."]
    #[inline(always)]
    pub const fn bytemsk(self) -> crate::common::Reg<regs::Bytemsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "IP Command Register."]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::common::Reg<regs::Ipcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "TX DATA Register."]
    #[inline(always)]
    pub const fn iptx(self) -> crate::common::Reg<regs::Iptx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RX DATA Register."]
    #[inline(always)]
    pub const fn iprx(self) -> crate::common::Reg<regs::Iprx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Status Register 0."]
    #[inline(always)]
    pub const fn stat0(self) -> crate::common::Reg<regs::Stat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Delay Line Config Register."]
    #[inline(always)]
    pub const fn dlycfg(self) -> crate::common::Reg<regs::Dlycfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
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
    #[doc = "Bus (AXI) Weight Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmw0(pub u32);
    impl Bmw0 {
        #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
        #[must_use]
        #[inline(always)]
        pub const fn qos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
        #[inline(always)]
        pub const fn set_qos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
        #[must_use]
        #[inline(always)]
        pub const fn age(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
        #[inline(always)]
        pub const fn set_age(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
        #[must_use]
        #[inline(always)]
        pub const fn sh(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
        #[inline(always)]
        pub const fn set_sh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
        #[must_use]
        #[inline(always)]
        pub const fn rws(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
        #[inline(always)]
        pub const fn set_rws(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bmw0 {
        #[inline(always)]
        fn default() -> Bmw0 {
            Bmw0(0)
        }
    }
    impl core::fmt::Debug for Bmw0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmw0")
                .field("qos", &self.qos())
                .field("age", &self.age())
                .field("sh", &self.sh())
                .field("rws", &self.rws())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmw0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bmw0 {{ qos: {=u8:?}, age: {=u8:?}, sh: {=u8:?}, rws: {=u8:?} }}",
                self.qos(),
                self.age(),
                self.sh(),
                self.rws()
            )
        }
    }
    #[doc = "Bus (AXI) Weight Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmw1(pub u32);
    impl Bmw1 {
        #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
        #[must_use]
        #[inline(always)]
        pub const fn qos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
        #[inline(always)]
        pub const fn set_qos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
        #[must_use]
        #[inline(always)]
        pub const fn age(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
        #[inline(always)]
        pub const fn set_age(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
        #[must_use]
        #[inline(always)]
        pub const fn ph(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
        #[inline(always)]
        pub const fn set_ph(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
        #[must_use]
        #[inline(always)]
        pub const fn rws(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
        #[inline(always)]
        pub const fn set_rws(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Weight of Bank Rotation. This weight score is valid when queue command's bank is not same as current executing command."]
        #[must_use]
        #[inline(always)]
        pub const fn br(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Weight of Bank Rotation. This weight score is valid when queue command's bank is not same as current executing command."]
        #[inline(always)]
        pub const fn set_br(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bmw1 {
        #[inline(always)]
        fn default() -> Bmw1 {
            Bmw1(0)
        }
    }
    impl core::fmt::Debug for Bmw1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmw1")
                .field("qos", &self.qos())
                .field("age", &self.age())
                .field("ph", &self.ph())
                .field("rws", &self.rws())
                .field("br", &self.br())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmw1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bmw1 {{ qos: {=u8:?}, age: {=u8:?}, ph: {=u8:?}, rws: {=u8:?}, br: {=u8:?} }}",
                self.qos(),
                self.age(),
                self.ph(),
                self.rws(),
                self.br()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Br(pub u32);
    impl Br {
        #[doc = "Valid."]
        #[must_use]
        #[inline(always)]
        pub const fn vld(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Valid."]
        #[inline(always)]
        pub const fn set_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::MemorySize {
            let val = (self.0 >> 1usize) & 0x1f;
            super::vals::MemorySize::from_bits(val as u8)
        }
        #[doc = "Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::MemorySize) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
        }
        #[doc = "Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Br {
        #[inline(always)]
        fn default() -> Br {
            Br(0)
        }
    }
    impl core::fmt::Debug for Br {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Br")
                .field("vld", &self.vld())
                .field("size", &self.size())
                .field("base", &self.base())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Br {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Br {{ vld: {=bool:?}, size: {:?}, base: {=u32:?} }}",
                self.vld(),
                self.size(),
                self.base()
            )
        }
    }
    #[doc = "IP Command Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bytemsk(pub u32);
    impl Bytemsk {
        #[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked."]
        #[must_use]
        #[inline(always)]
        pub const fn bm0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked."]
        #[inline(always)]
        pub const fn set_bm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked."]
        #[must_use]
        #[inline(always)]
        pub const fn bm1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked."]
        #[inline(always)]
        pub const fn set_bm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked."]
        #[must_use]
        #[inline(always)]
        pub const fn bm2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked."]
        #[inline(always)]
        pub const fn set_bm2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked."]
        #[must_use]
        #[inline(always)]
        pub const fn bm3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked."]
        #[inline(always)]
        pub const fn set_bm3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Bytemsk {
        #[inline(always)]
        fn default() -> Bytemsk {
            Bytemsk(0)
        }
    }
    impl core::fmt::Debug for Bytemsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bytemsk")
                .field("bm0", &self.bm0())
                .field("bm1", &self.bm1())
                .field("bm2", &self.bm2())
                .field("bm3", &self.bm3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bytemsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bytemsk {{ bm0: {=bool:?}, bm1: {=bool:?}, bm2: {=bool:?}, bm3: {=bool:?} }}",
                self.bm0(),
                self.bm1(),
                self.bm2(),
                self.bm3()
            )
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Software Reset Reset all internal logic in SEMC except configuration register."]
        #[must_use]
        #[inline(always)]
        pub const fn rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset Reset all internal logic in SEMC except configuration register."]
        #[inline(always)]
        pub const fn set_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Module Disable 0b - Module enabled 1b - Module disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Module Disable 0b - Module enabled 1b - Module disabled."]
        #[inline(always)]
        pub const fn set_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad."]
        #[must_use]
        #[inline(always)]
        pub const fn dqs(&self) -> super::vals::Dqs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Dqs::from_bits(val as u8)
        }
        #[doc = "DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad."]
        #[inline(always)]
        pub const fn set_dqs(&mut self, val: super::vals::Dqs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn cto(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
        #[inline(always)]
        pub const fn set_cto(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31."]
        #[must_use]
        #[inline(always)]
        pub const fn bto(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31."]
        #[inline(always)]
        pub const fn set_bto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("rst", &self.rst())
                .field("dis", &self.dis())
                .field("dqs", &self.dqs())
                .field("cto", &self.cto())
                .field("bto", &self.bto())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ rst: {=bool:?}, dis: {=bool:?}, dqs: {:?}, cto: {=u8:?}, bto: {=u8:?} }}",
                self.rst(),
                self.dis(),
                self.dqs(),
                self.cto(),
                self.bto()
            )
        }
    }
    #[doc = "IP Command Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datsz(pub u32);
    impl Datsz {
        #[doc = "Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4."]
        #[must_use]
        #[inline(always)]
        pub const fn datsz(&self) -> super::vals::DataSize {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::DataSize::from_bits(val as u8)
        }
        #[doc = "Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4."]
        #[inline(always)]
        pub const fn set_datsz(&mut self, val: super::vals::DataSize) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Datsz {
        #[inline(always)]
        fn default() -> Datsz {
            Datsz(0)
        }
    }
    impl core::fmt::Debug for Datsz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Datsz")
                .field("datsz", &self.datsz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Datsz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Datsz {{ datsz: {:?} }}", self.datsz())
        }
    }
    #[doc = "Delay Line Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlycfg(pub u32);
    impl Dlycfg {
        #[doc = "delay line enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "delay line enable."]
        #[inline(always)]
        pub const fn set_dlyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "delay line select, 0 for 1 cell, 31 for all 32 cells."]
        #[must_use]
        #[inline(always)]
        pub const fn dlysel(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "delay line select, 0 for 1 cell, 31 for all 32 cells."]
        #[inline(always)]
        pub const fn set_dlysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "delay clock output enable, should be set after setting DLYEN and DLYSEL."]
        #[must_use]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "delay clock output enable, should be set after setting DLYEN and DLYSEL."]
        #[inline(always)]
        pub const fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Dlycfg {
        #[inline(always)]
        fn default() -> Dlycfg {
            Dlycfg(0)
        }
    }
    impl core::fmt::Debug for Dlycfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlycfg")
                .field("dlyen", &self.dlyen())
                .field("dlysel", &self.dlysel())
                .field("oe", &self.oe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlycfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlycfg {{ dlyen: {=bool:?}, dlysel: {=u8:?}, oe: {=bool:?} }}",
                self.dlyen(),
                self.dlysel(),
                self.oe()
            )
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
        #[doc = "IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ipcmddone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[inline(always)]
        pub const fn set_ipcmddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ipcmderr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[inline(always)]
        pub const fn set_ipcmderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn axicmderr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[inline(always)]
        pub const fn set_axicmderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn axibuserr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled."]
        #[inline(always)]
        pub const fn set_axibuserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Inten {
        #[inline(always)]
        fn default() -> Inten {
            Inten(0)
        }
    }
    impl core::fmt::Debug for Inten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inten")
                .field("ipcmddone", &self.ipcmddone())
                .field("ipcmderr", &self.ipcmderr())
                .field("axicmderr", &self.axicmderr())
                .field("axibuserr", &self.axibuserr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inten {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Inten {{ ipcmddone: {=bool:?}, ipcmderr: {=bool:?}, axicmderr: {=bool:?}, axibuserr: {=bool:?} }}" , self . ipcmddone () , self . ipcmderr () , self . axicmderr () , self . axibuserr ())
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intr(pub u32);
    impl Intr {
        #[doc = "IP command normal done interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ipcmddone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IP command normal done interrupt."]
        #[inline(always)]
        pub const fn set_ipcmddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command."]
        #[must_use]
        #[inline(always)]
        pub const fn ipcmderr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command."]
        #[inline(always)]
        pub const fn set_ipcmderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn axicmderr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
        #[inline(always)]
        pub const fn set_axicmderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read."]
        #[must_use]
        #[inline(always)]
        pub const fn axibuserr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read."]
        #[inline(always)]
        pub const fn set_axibuserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Intr {
        #[inline(always)]
        fn default() -> Intr {
            Intr(0)
        }
    }
    impl core::fmt::Debug for Intr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intr")
                .field("ipcmddone", &self.ipcmddone())
                .field("ipcmderr", &self.ipcmderr())
                .field("axicmderr", &self.axicmderr())
                .field("axibuserr", &self.axibuserr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intr {{ ipcmddone: {=bool:?}, ipcmderr: {=bool:?}, axicmderr: {=bool:?}, axibuserr: {=bool:?} }}" , self . ipcmddone () , self . ipcmderr () , self . axicmderr () , self . axibuserr ())
        }
    }
    #[doc = "IO Mux Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioctrl(pub u32);
    impl Ioctrl {
        #[doc = "IO_CSX output selection 0001b - SDRAM CS1 0110b - SRAM CE#."]
        #[must_use]
        #[inline(always)]
        pub const fn io_csx(&self) -> super::vals::IoCsx {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::IoCsx::from_bits(val as u8)
        }
        #[doc = "IO_CSX output selection 0001b - SDRAM CS1 0110b - SRAM CE#."]
        #[inline(always)]
        pub const fn set_io_csx(&mut self, val: super::vals::IoCsx) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Ioctrl {
        #[inline(always)]
        fn default() -> Ioctrl {
            Ioctrl(0)
        }
    }
    impl core::fmt::Debug for Ioctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ioctrl")
                .field("io_csx", &self.io_csx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ioctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ioctrl {{ io_csx: {:?} }}", self.io_csx())
        }
    }
    #[doc = "IP Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipcmd(pub u32);
    impl Ipcmd {
        #[doc = "SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> super::vals::SdramCmd {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::SdramCmd::from_bits(val as u16)
        }
        #[doc = "SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: super::vals::SdramCmd) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
        #[doc = "This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ipcmd {
        #[inline(always)]
        fn default() -> Ipcmd {
            Ipcmd(0)
        }
    }
    impl core::fmt::Debug for Ipcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipcmd")
                .field("cmd", &self.cmd())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipcmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ipcmd {{ cmd: {:?}, key: {=u16:?} }}",
                self.cmd(),
                self.key()
            )
        }
    }
    #[doc = "RX DATA Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprx(pub u32);
    impl Iprx {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn dat(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_dat(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprx {
        #[inline(always)]
        fn default() -> Iprx {
            Iprx(0)
        }
    }
    impl core::fmt::Debug for Iprx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprx").field("dat", &self.dat()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iprx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iprx {{ dat: {=u32:?} }}", self.dat())
        }
    }
    #[doc = "TX DATA Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iptx(pub u32);
    impl Iptx {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn dat(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_dat(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iptx {
        #[inline(always)]
        fn default() -> Iptx {
            Iptx(0)
        }
    }
    impl core::fmt::Debug for Iptx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iptx").field("dat", &self.dat()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iptx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iptx {{ dat: {=u32:?} }}", self.dat())
        }
    }
    #[doc = "IP Command Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Saddr(pub u32);
    impl Saddr {
        #[doc = "Slave address."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Slave address."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Saddr {
        #[inline(always)]
        fn default() -> Saddr {
            Saddr(0)
        }
    }
    impl core::fmt::Debug for Saddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Saddr").field("sa", &self.sa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Saddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Saddr {{ sa: {=u32:?} }}", self.sa())
        }
    }
    #[doc = "SDRAM Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrctrl0(pub u32);
    impl Sdrctrl0 {
        #[doc = "Port Size 00b - 8bit 01b - 16bit 10b - 32bit."]
        #[must_use]
        #[inline(always)]
        pub const fn portsz(&self) -> super::vals::SdramPortSize {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::SdramPortSize::from_bits(val as u8)
        }
        #[doc = "Port Size 00b - 8bit 01b - 16bit 10b - 32bit."]
        #[inline(always)]
        pub const fn set_portsz(&mut self, val: super::vals::SdramPortSize) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)."]
        #[must_use]
        #[inline(always)]
        pub const fn highband(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)."]
        #[inline(always)]
        pub const fn set_highband(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8."]
        #[must_use]
        #[inline(always)]
        pub const fn burstlen(&self) -> super::vals::BurstLen {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::BurstLen::from_bits(val as u8)
        }
        #[doc = "Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8."]
        #[inline(always)]
        pub const fn set_burstlen(&mut self, val: super::vals::BurstLen) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Merged COL and COL8 register"]
        #[must_use]
        #[inline(always)]
        pub const fn col(&self) -> super::vals::ColAddrBits {
            let val = (self.0 >> 7usize) & 0x07;
            super::vals::ColAddrBits::from_bits(val as u8)
        }
        #[doc = "Merged COL and COL8 register"]
        #[inline(always)]
        pub const fn set_col(&mut self, val: super::vals::ColAddrBits) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
        }
        #[doc = "CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3."]
        #[must_use]
        #[inline(always)]
        pub const fn cas(&self) -> super::vals::CasLatency {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::CasLatency::from_bits(val as u8)
        }
        #[doc = "CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3."]
        #[inline(always)]
        pub const fn set_cas(&mut self, val: super::vals::CasLatency) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
        #[must_use]
        #[inline(always)]
        pub const fn bank2(&self) -> super::vals::Bank2Sel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Bank2Sel::from_bits(val as u8)
        }
        #[doc = "2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
        #[inline(always)]
        pub const fn set_bank2(&mut self, val: super::vals::Bank2Sel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sdrctrl0 {
        #[inline(always)]
        fn default() -> Sdrctrl0 {
            Sdrctrl0(0)
        }
    }
    impl core::fmt::Debug for Sdrctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrctrl0")
                .field("portsz", &self.portsz())
                .field("highband", &self.highband())
                .field("burstlen", &self.burstlen())
                .field("col", &self.col())
                .field("cas", &self.cas())
                .field("bank2", &self.bank2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdrctrl0 {{ portsz: {:?}, highband: {=bool:?}, burstlen: {:?}, col: {:?}, cas: {:?}, bank2: {:?} }}" , self . portsz () , self . highband () , self . burstlen () , self . col () , self . cas () , self . bank2 ())
        }
    }
    #[doc = "SDRAM Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrctrl1(pub u32);
    impl Sdrctrl1 {
        #[doc = "PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn pre2act(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_pre2act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn act2rw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_act2rw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn rfrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_rfrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn wrc(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_wrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ckeoff(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
        #[inline(always)]
        pub const fn set_ckeoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
        #[must_use]
        #[inline(always)]
        pub const fn act2pre(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
        #[inline(always)]
        pub const fn set_act2pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Sdrctrl1 {
        #[inline(always)]
        fn default() -> Sdrctrl1 {
            Sdrctrl1(0)
        }
    }
    impl core::fmt::Debug for Sdrctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrctrl1")
                .field("pre2act", &self.pre2act())
                .field("act2rw", &self.act2rw())
                .field("rfrc", &self.rfrc())
                .field("wrc", &self.wrc())
                .field("ckeoff", &self.ckeoff())
                .field("act2pre", &self.act2pre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdrctrl1 {{ pre2act: {=u8:?}, act2rw: {=u8:?}, rfrc: {=u8:?}, wrc: {=u8:?}, ckeoff: {=u8:?}, act2pre: {=u8:?} }}" , self . pre2act () , self . act2rw () , self . rfrc () , self . wrc () , self . ckeoff () , self . act2pre ())
        }
    }
    #[doc = "SDRAM Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrctrl2(pub u32);
    impl Sdrctrl2 {
        #[doc = "Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
        #[must_use]
        #[inline(always)]
        pub const fn srrc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
        #[inline(always)]
        pub const fn set_srrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn ref2ref(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_ref2ref(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
        #[must_use]
        #[inline(always)]
        pub const fn act2act(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
        #[inline(always)]
        pub const fn set_act2act(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
        #[must_use]
        #[inline(always)]
        pub const fn ito(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
        #[inline(always)]
        pub const fn set_ito(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sdrctrl2 {
        #[inline(always)]
        fn default() -> Sdrctrl2 {
            Sdrctrl2(0)
        }
    }
    impl core::fmt::Debug for Sdrctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrctrl2")
                .field("srrc", &self.srrc())
                .field("ref2ref", &self.ref2ref())
                .field("act2act", &self.act2act())
                .field("ito", &self.ito())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrctrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sdrctrl2 {{ srrc: {=u8:?}, ref2ref: {=u8:?}, act2act: {=u8:?}, ito: {=u8:?} }}",
                self.srrc(),
                self.ref2ref(),
                self.act2act(),
                self.ito()
            )
        }
    }
    #[doc = "SDRAM Control Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdrctrl3(pub u32);
    impl Sdrctrl3 {
        #[doc = "Refresh enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh enable."]
        #[inline(always)]
        pub const fn set_ren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8."]
        #[must_use]
        #[inline(always)]
        pub const fn rebl(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8."]
        #[inline(always)]
        pub const fn set_rebl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn prescale(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles."]
        #[inline(always)]
        pub const fn set_prescale(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period."]
        #[must_use]
        #[inline(always)]
        pub const fn rt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period."]
        #[inline(always)]
        pub const fn set_rt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period."]
        #[must_use]
        #[inline(always)]
        pub const fn ut(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period."]
        #[inline(always)]
        pub const fn set_ut(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sdrctrl3 {
        #[inline(always)]
        fn default() -> Sdrctrl3 {
            Sdrctrl3(0)
        }
    }
    impl core::fmt::Debug for Sdrctrl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sdrctrl3")
                .field("ren", &self.ren())
                .field("rebl", &self.rebl())
                .field("prescale", &self.prescale())
                .field("rt", &self.rt())
                .field("ut", &self.ut())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sdrctrl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sdrctrl3 {{ ren: {=bool:?}, rebl: {=u8:?}, prescale: {=u8:?}, rt: {=u8:?}, ut: {=u8:?} }}" , self . ren () , self . rebl () , self . prescale () , self . rt () , self . ut ())
        }
    }
    #[doc = "SRAM control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srctrl0(pub u32);
    impl Srctrl0 {
        #[doc = "port size 0b - 8bit 1b - 16bit."]
        #[must_use]
        #[inline(always)]
        pub const fn portsz(&self) -> super::vals::SramPortSize {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::SramPortSize::from_bits(val as u8)
        }
        #[doc = "port size 0b - 8bit 1b - 16bit."]
        #[inline(always)]
        pub const fn set_portsz(&mut self, val: super::vals::SramPortSize) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "address data mode 00b - address and data MUX mode 11b - address and data non-MUX mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adm(&self) -> super::vals::AddressDataMux {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::AddressDataMux::from_bits(val as u8)
        }
        #[doc = "address data mode 00b - address and data MUX mode 11b - address and data non-MUX mode."]
        #[inline(always)]
        pub const fn set_adm(&mut self, val: super::vals::AddressDataMux) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "ADV polarity 0b - ADV is active low 1b - ADV is active high."]
        #[must_use]
        #[inline(always)]
        pub const fn advp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADV polarity 0b - ADV is active low 1b - ADV is active high."]
        #[inline(always)]
        pub const fn set_advp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ADV hold state 0b - ADV is high during address hold state 1b - ADV is low during address hold state."]
        #[must_use]
        #[inline(always)]
        pub const fn advh(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ADV hold state 0b - ADV is high during address hold state 1b - ADV is low during address hold state."]
        #[inline(always)]
        pub const fn set_advh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Srctrl0 {
        #[inline(always)]
        fn default() -> Srctrl0 {
            Srctrl0(0)
        }
    }
    impl core::fmt::Debug for Srctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srctrl0")
                .field("portsz", &self.portsz())
                .field("adm", &self.adm())
                .field("advp", &self.advp())
                .field("advh", &self.advh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Srctrl0 {{ portsz: {:?}, adm: {:?}, advp: {=bool:?}, advh: {=bool:?} }}",
                self.portsz(),
                self.adm(),
                self.advp(),
                self.advh()
            )
        }
    }
    #[doc = "SRAM control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srctrl1(pub u32);
    impl Srctrl1 {
        #[doc = "Chip enable setup time, is CES+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ces(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Chip enable setup time, is CES+1 clock cycles."]
        #[inline(always)]
        pub const fn set_ces(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Chip enable hold time, is CEH+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ceh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Chip enable hold time, is CEH+1 clock cycles."]
        #[inline(always)]
        pub const fn set_ceh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Address setup time, is AS+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn as_(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup time, is AS+1 clock cycles."]
        #[inline(always)]
        pub const fn set_as_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Address hold time, is AH+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ah(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Address hold time, is AH+1 clock cycles."]
        #[inline(always)]
        pub const fn set_ah(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "WE low time, is WEL+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn wel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "WE low time, is WEL+1 clock cycles."]
        #[inline(always)]
        pub const fn set_wel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "WE high time, is WEH+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn weh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "WE high time, is WEH+1 clock cycles."]
        #[inline(always)]
        pub const fn set_weh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "OE low time, is OEL+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn oel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OE low time, is OEL+1 clock cycles."]
        #[inline(always)]
        pub const fn set_oel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "OE high time, is OEH+1 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn oeh(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "OE high time, is OEH+1 clock cycles."]
        #[inline(always)]
        pub const fn set_oeh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Srctrl1 {
        #[inline(always)]
        fn default() -> Srctrl1 {
            Srctrl1(0)
        }
    }
    impl core::fmt::Debug for Srctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srctrl1")
                .field("ces", &self.ces())
                .field("ceh", &self.ceh())
                .field("as_", &self.as_())
                .field("ah", &self.ah())
                .field("wel", &self.wel())
                .field("weh", &self.weh())
                .field("oel", &self.oel())
                .field("oeh", &self.oeh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Srctrl1 {{ ces: {=u8:?}, ceh: {=u8:?}, as_: {=u8:?}, ah: {=u8:?}, wel: {=u8:?}, weh: {=u8:?}, oel: {=u8:?}, oeh: {=u8:?} }}" , self . ces () , self . ceh () , self . as_ () , self . ah () , self . wel () , self . weh () , self . oel () , self . oeh ())
        }
    }
    #[doc = "Status Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat0(pub u32);
    impl Stat0 {
        #[doc = "Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Stat0 {
        #[inline(always)]
        fn default() -> Stat0 {
            Stat0(0)
        }
    }
    impl core::fmt::Debug for Stat0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat0").field("idle", &self.idle()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stat0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Stat0 {{ idle: {=bool:?} }}", self.idle())
        }
    }
}
pub mod vals {
    #[doc = "address data mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AddressDataMux {
        #[doc = "address and data MUX mode"]
        MUX = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "address and data non-MUX mode"]
        NONE = 0x03,
    }
    impl AddressDataMux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AddressDataMux {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AddressDataMux {
        #[inline(always)]
        fn from(val: u8) -> AddressDataMux {
            AddressDataMux::from_bits(val)
        }
    }
    impl From<AddressDataMux> for u8 {
        #[inline(always)]
        fn from(val: AddressDataMux) -> u8 {
            AddressDataMux::to_bits(val)
        }
    }
    #[doc = "2 Bank selection bit."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bank2Sel {
        #[doc = "SDRAM device has 4 banks"]
        BANK_NUM_4 = 0x0,
        #[doc = "SDRAM device has 2 banks"]
        BANK_NUM_2 = 0x01,
    }
    impl Bank2Sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bank2Sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bank2Sel {
        #[inline(always)]
        fn from(val: u8) -> Bank2Sel {
            Bank2Sel::from_bits(val)
        }
    }
    impl From<Bank2Sel> for u8 {
        #[inline(always)]
        fn from(val: Bank2Sel) -> u8 {
            Bank2Sel::to_bits(val)
        }
    }
    #[doc = "Burst Length."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BurstLen {
        _1 = 0x0,
        _2 = 0x01,
        _4 = 0x02,
        _8 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl BurstLen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BurstLen {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BurstLen {
        #[inline(always)]
        fn from(val: u8) -> BurstLen {
            BurstLen::from_bits(val)
        }
    }
    impl From<BurstLen> for u8 {
        #[inline(always)]
        fn from(val: BurstLen) -> u8 {
            BurstLen::to_bits(val)
        }
    }
    #[doc = "CAS Latency."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CasLatency {
        _1 = 0x0,
        _RESERVED_1 = 0x01,
        _2 = 0x02,
        _3 = 0x03,
    }
    impl CasLatency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CasLatency {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CasLatency {
        #[inline(always)]
        fn from(val: u8) -> CasLatency {
            CasLatency::from_bits(val)
        }
    }
    impl From<CasLatency> for u8 {
        #[inline(always)]
        fn from(val: CasLatency) -> u8 {
            CasLatency::to_bits(val)
        }
    }
    #[doc = "Column address bit number. merged with COL8"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ColAddrBits {
        #[doc = "12 bit"]
        _12BIT = 0x0,
        #[doc = "8 bit (COL8)"]
        _8BIT = 0x01,
        #[doc = "11 bit"]
        _11BIT = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "10 bit"]
        _10BIT = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "9 bit"]
        _9BIT = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl ColAddrBits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ColAddrBits {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ColAddrBits {
        #[inline(always)]
        fn from(val: u8) -> ColAddrBits {
            ColAddrBits::from_bits(val)
        }
    }
    impl From<ColAddrBits> for u8 {
        #[inline(always)]
        fn from(val: ColAddrBits) -> u8 {
            ColAddrBits::to_bits(val)
        }
    }
    #[doc = "Data Size. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DataSize {
        _RESERVED_0 = 0x0,
        _8BIT = 0x01,
        _16BIT = 0x02,
        _24BIT = 0x03,
        #[doc = "32bit, and other variants"]
        _32BIT = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl DataSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataSize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataSize {
        #[inline(always)]
        fn from(val: u8) -> DataSize {
            DataSize::from_bits(val)
        }
    }
    impl From<DataSize> for u8 {
        #[inline(always)]
        fn from(val: DataSize) -> u8 {
            DataSize::to_bits(val)
        }
    }
    #[doc = "DQS (read strobe) mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dqs {
        #[doc = "Dummy read strobe loopbacked internally"]
        INTERNAL = 0x0,
        #[doc = "Dummy read strobe loopbacked from DQS pad"]
        FROM_PAD = 0x01,
    }
    impl Dqs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dqs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dqs {
        #[inline(always)]
        fn from(val: u8) -> Dqs {
            Dqs::from_bits(val)
        }
    }
    impl From<Dqs> for u8 {
        #[inline(always)]
        fn from(val: Dqs) -> u8 {
            Dqs::to_bits(val)
        }
    }
    #[doc = "IO_CSX output selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IoCsx {
        _RESERVED_0 = 0x0,
        #[doc = "SDRAM CS1"]
        SDRAM_CS1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "SRAM CE#"]
        SRAM_CE = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl IoCsx {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IoCsx {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IoCsx {
        #[inline(always)]
        fn from(val: u8) -> IoCsx {
            IoCsx::from_bits(val)
        }
    }
    impl From<IoCsx> for u8 {
        #[inline(always)]
        fn from(val: IoCsx) -> u8 {
            IoCsx::to_bits(val)
        }
    }
    #[doc = "Memory size."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum MemorySize {
        #[doc = "4KB"]
        _4KB = 0x0,
        #[doc = "8KB"]
        _8KB = 0x01,
        #[doc = "16KB"]
        _16KB = 0x02,
        #[doc = "32KB"]
        _32KB = 0x03,
        #[doc = "64KB"]
        _64KB = 0x04,
        #[doc = "128KB"]
        _128KB = 0x05,
        #[doc = "256KB"]
        _256KB = 0x06,
        #[doc = "512KB"]
        _512KB = 0x07,
        #[doc = "1MB"]
        _1MB = 0x08,
        #[doc = "2MB"]
        _2MB = 0x09,
        #[doc = "4MB"]
        _4MB = 0x0a,
        #[doc = "8MB"]
        _8MB = 0x0b,
        #[doc = "16MB"]
        _16MB = 0x0c,
        #[doc = "32MB"]
        _32MB = 0x0d,
        #[doc = "64MB"]
        _64MB = 0x0e,
        #[doc = "128MB"]
        _128MB = 0x0f,
        #[doc = "256MB"]
        _256MB = 0x10,
        #[doc = "512MB"]
        _512MB = 0x11,
        #[doc = "1GB"]
        _1GB = 0x12,
        #[doc = "2GB"]
        _2GB = 0x13,
        #[doc = "4GB"]
        _4GB = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl MemorySize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemorySize {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemorySize {
        #[inline(always)]
        fn from(val: u8) -> MemorySize {
            MemorySize::from_bits(val)
        }
    }
    impl From<MemorySize> for u8 {
        #[inline(always)]
        fn from(val: MemorySize) -> u8 {
            MemorySize::to_bits(val)
        }
    }
    #[doc = "SDRAM Commands."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SdramCmd(u16);
    impl SdramCmd {
        #[doc = "READ"]
        pub const READ: Self = Self(0x08);
        #[doc = "WRITE"]
        pub const WRITE: Self = Self(0x09);
        #[doc = "MODE_SET"]
        pub const MODE_SET: Self = Self(0x0a);
        #[doc = "ACTIVE"]
        pub const ACTIVE: Self = Self(0x0b);
        #[doc = "AUTO REFRESH"]
        pub const AUTO_REFRESH: Self = Self(0x0c);
        #[doc = "SELF REFRESH"]
        pub const SELF_REFRESH: Self = Self(0x0d);
        #[doc = "PRECHARGE"]
        pub const PRECHARGE: Self = Self(0x0e);
        #[doc = "PRECHARGE ALL"]
        pub const PRECHARGE_ALL: Self = Self(0x0f);
    }
    impl SdramCmd {
        pub const fn from_bits(val: u16) -> SdramCmd {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for SdramCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x08 => f.write_str("READ"),
                0x09 => f.write_str("WRITE"),
                0x0a => f.write_str("MODE_SET"),
                0x0b => f.write_str("ACTIVE"),
                0x0c => f.write_str("AUTO_REFRESH"),
                0x0d => f.write_str("SELF_REFRESH"),
                0x0e => f.write_str("PRECHARGE"),
                0x0f => f.write_str("PRECHARGE_ALL"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SdramCmd {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x08 => defmt::write!(f, "READ"),
                0x09 => defmt::write!(f, "WRITE"),
                0x0a => defmt::write!(f, "MODE_SET"),
                0x0b => defmt::write!(f, "ACTIVE"),
                0x0c => defmt::write!(f, "AUTO_REFRESH"),
                0x0d => defmt::write!(f, "SELF_REFRESH"),
                0x0e => defmt::write!(f, "PRECHARGE"),
                0x0f => defmt::write!(f, "PRECHARGE_ALL"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for SdramCmd {
        #[inline(always)]
        fn from(val: u16) -> SdramCmd {
            SdramCmd::from_bits(val)
        }
    }
    impl From<SdramCmd> for u16 {
        #[inline(always)]
        fn from(val: SdramCmd) -> u16 {
            SdramCmd::to_bits(val)
        }
    }
    #[doc = "Port Size."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SdramPortSize {
        #[doc = "8bit"]
        _8BIT = 0x0,
        #[doc = "16bit"]
        _16BIT = 0x01,
        #[doc = "32bit"]
        _32BIT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl SdramPortSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SdramPortSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SdramPortSize {
        #[inline(always)]
        fn from(val: u8) -> SdramPortSize {
            SdramPortSize::from_bits(val)
        }
    }
    impl From<SdramPortSize> for u8 {
        #[inline(always)]
        fn from(val: SdramPortSize) -> u8 {
            SdramPortSize::to_bits(val)
        }
    }
    #[doc = "port size."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SramPortSize {
        #[doc = "8bit"]
        _8BIT = 0x0,
        #[doc = "16bit"]
        _16BIT = 0x01,
    }
    impl SramPortSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SramPortSize {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SramPortSize {
        #[inline(always)]
        fn from(val: u8) -> SramPortSize {
            SramPortSize::from_bits(val)
        }
    }
    impl From<SramPortSize> for u8 {
        #[inline(always)]
        fn from(val: SramPortSize) -> u8 {
            SramPortSize::to_bits(val)
        }
    }
}
