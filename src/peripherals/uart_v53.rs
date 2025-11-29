#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "UART0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Idle Configuration Register."]
    #[inline(always)]
    pub const fn idle_cfg(self) -> crate::common::Reg<regs::IdleCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "address match config register."]
    #[inline(always)]
    pub const fn addr_cfg(self) -> crate::common::Reg<regs::AddrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Identification Register2."]
    #[inline(always)]
    pub const fn iir2(self) -> crate::common::Reg<regs::Iir2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Over Sample Control Register."]
    #[inline(always)]
    pub const fn oscr(self) -> crate::common::Reg<regs::Oscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "FIFO Control Register config."]
    #[inline(always)]
    pub const fn fcrr(self) -> crate::common::Reg<regs::Fcrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "moto system control register."]
    #[inline(always)]
    pub const fn moto_cfg(self) -> crate::common::Reg<regs::MotoCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Divisor Latch LSB (when DLAB = 1)."]
    #[inline(always)]
    pub const fn dll(self) -> crate::common::Reg<regs::Dll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Receiver Buffer Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Transmitter Holding Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Divisor Latch MSB (when DLAB = 1)."]
    #[inline(always)]
    pub const fn dlm(self) -> crate::common::Reg<regs::Dlm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "FIFO Control Register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Identification Register."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Line Control Register."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Modem Control Register (."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Line Status Register."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Modem Status Register."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPR Register."]
    #[inline(always)]
    pub const fn gpr(self) -> crate::common::Reg<regs::Gpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
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
    #[doc = "address match config register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AddrCfg(pub u32);
    impl AddrCfg {
        #[doc = "address 0 field."]
        #[must_use]
        #[inline(always)]
        pub const fn addr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "address 0 field."]
        #[inline(always)]
        pub const fn set_addr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80."]
        #[must_use]
        #[inline(always)]
        pub const fn addr1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "address 1 fileld. in 9bit mode, this is the full address byte. For other mode(8/7/6/5bit), MSB should be set for address flag. If want address==0 to be matched at 8bit mode, should set addr1=0x80."]
        #[inline(always)]
        pub const fn set_addr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "enable addr0 compare for the first character."]
        #[must_use]
        #[inline(always)]
        pub const fn a0_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "enable addr0 compare for the first character."]
        #[inline(always)]
        pub const fn set_a0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature."]
        #[must_use]
        #[inline(always)]
        pub const fn a1_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "enable addr1 compare for the first character. If a1_en OR a0_en, then do not receive data if address not match. If ~a1_en AND ~a0_en, the receive all data like before. NOTE: should set idle_tmout_en if enable address match feature."]
        #[inline(always)]
        pub const fn set_a1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "set to use 9bit mode for receiver, only valid if rxen_addr_msb is set."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen_9bit(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "set to use 9bit mode for receiver, only valid if rxen_addr_msb is set."]
        #[inline(always)]
        pub const fn set_rxen_9bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen_addr_msb(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "set to use MSB as address flag at receiver(actually this is done by software set correct MSB in addr0/addr1). Clr to use first character as address. Only needed if enable address match feature."]
        #[inline(always)]
        pub const fn set_rxen_addr_msb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others."]
        #[must_use]
        #[inline(always)]
        pub const fn txen_9bit(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "set to use 9bit mode for transmitter, will set the MSB for the first character as address flag, keep 0 for others."]
        #[inline(always)]
        pub const fn set_txen_9bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for AddrCfg {
        #[inline(always)]
        fn default() -> AddrCfg {
            AddrCfg(0)
        }
    }
    impl core::fmt::Debug for AddrCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AddrCfg")
                .field("addr0", &self.addr0())
                .field("addr1", &self.addr1())
                .field("a0_en", &self.a0_en())
                .field("a1_en", &self.a1_en())
                .field("rxen_9bit", &self.rxen_9bit())
                .field("rxen_addr_msb", &self.rxen_addr_msb())
                .field("txen_9bit", &self.txen_9bit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AddrCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AddrCfg {{ addr0: {=u8:?}, addr1: {=u8:?}, a0_en: {=bool:?}, a1_en: {=bool:?}, rxen_9bit: {=bool:?}, rxen_addr_msb: {=bool:?}, txen_9bit: {=bool:?} }}" , self . addr0 () , self . addr1 () , self . a0_en () , self . a1_en () , self . rxen_9bit () , self . rxen_addr_msb () , self . txen_9bit ())
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO."]
        #[inline(always)]
        pub const fn set_fifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("fifosize", &self.fifosize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfg {{ fifosize: {=u8:?} }}", self.fifosize())
        }
    }
    #[doc = "Divisor Latch LSB (when DLAB = 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dll(pub u32);
    impl Dll {
        #[doc = "Least significant byte of the Divisor Latch."]
        #[must_use]
        #[inline(always)]
        pub const fn dll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Least significant byte of the Divisor Latch."]
        #[inline(always)]
        pub const fn set_dll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dll {
        #[inline(always)]
        fn default() -> Dll {
            Dll(0)
        }
    }
    impl core::fmt::Debug for Dll {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dll").field("dll", &self.dll()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dll {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dll {{ dll: {=u8:?} }}", self.dll())
        }
    }
    #[doc = "Divisor Latch MSB (when DLAB = 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlm(pub u32);
    impl Dlm {
        #[doc = "Most significant byte of the Divisor Latch."]
        #[must_use]
        #[inline(always)]
        pub const fn dlm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Most significant byte of the Divisor Latch."]
        #[inline(always)]
        pub const fn set_dlm(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dlm {
        #[inline(always)]
        fn default() -> Dlm {
            Dlm(0)
        }
    }
    impl core::fmt::Debug for Dlm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlm").field("dlm", &self.dlm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlm {{ dlm: {=u8:?} }}", self.dlm())
        }
    }
    #[doc = "FIFO Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[must_use]
        #[inline(always)]
        pub const fn fifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[inline(always)]
        pub const fn set_fifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn rfiforst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn set_rfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn tfiforst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn set_tfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[must_use]
        #[inline(always)]
        pub const fn tfifot(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[inline(always)]
        pub const fn set_tfifot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Receiver FIFO trigger level."]
        #[must_use]
        #[inline(always)]
        pub const fn rfifot(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Receiver FIFO trigger level."]
        #[inline(always)]
        pub const fn set_rfifot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("fifoe", &self.fifoe())
                .field("rfiforst", &self.rfiforst())
                .field("tfiforst", &self.tfiforst())
                .field("dmae", &self.dmae())
                .field("tfifot", &self.tfifot())
                .field("rfifot", &self.rfifot())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcr {{ fifoe: {=bool:?}, rfiforst: {=bool:?}, tfiforst: {=bool:?}, dmae: {=bool:?}, tfifot: {=u8:?}, rfifot: {=u8:?} }}" , self . fifoe () , self . rfiforst () , self . tfiforst () , self . dmae () , self . tfifot () , self . rfifot ())
        }
    }
    #[doc = "FIFO Control Register config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcrr(pub u32);
    impl Fcrr {
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[must_use]
        #[inline(always)]
        pub const fn fifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[inline(always)]
        pub const fn set_fifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn rfiforst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn set_rfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn tfiforst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn set_tfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[must_use]
        #[inline(always)]
        pub const fn tfifot(&self) -> super::vals::TxFifoTrigger {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::TxFifoTrigger::from_bits(val as u8)
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[inline(always)]
        pub const fn set_tfifot(&mut self, val: super::vals::TxFifoTrigger) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Receiver FIFO trigger level."]
        #[must_use]
        #[inline(always)]
        pub const fn rfifot(&self) -> super::vals::RxFifoTrigger {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::RxFifoTrigger::from_bits(val as u8)
        }
        #[doc = "Receiver FIFO trigger level."]
        #[inline(always)]
        pub const fn set_rfifot(&mut self, val: super::vals::RxFifoTrigger) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "rxfifo threshold(0 for 1byte, 0xF for 16bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rfifot4(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "rxfifo threshold(0 for 1byte, 0xF for 16bytes). Uart will send rx_dma_req if data in fifo reachs the threshold, also will set the rxdata irq if enabled."]
        #[inline(always)]
        pub const fn set_rfifot4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "txfifo threshold(0 for 1byte, 0xF for 16bytes), uart will send tx_dma_req when data in fifo is less than threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn tfifot4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "txfifo threshold(0 for 1byte, 0xF for 16bytes), uart will send tx_dma_req when data in fifo is less than threshold."]
        #[inline(always)]
        pub const fn set_tfifot4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)."]
        #[must_use]
        #[inline(always)]
        pub const fn fifot4en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "set to use new 4bit fifo threshold(TFIFOT4 and RFIFOT4) clr to use 2bit(TFIFOT and RFIFOT)."]
        #[inline(always)]
        pub const fn set_fifot4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Fcrr {
        #[inline(always)]
        fn default() -> Fcrr {
            Fcrr(0)
        }
    }
    impl core::fmt::Debug for Fcrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcrr")
                .field("fifoe", &self.fifoe())
                .field("rfiforst", &self.rfiforst())
                .field("tfiforst", &self.tfiforst())
                .field("dmae", &self.dmae())
                .field("tfifot", &self.tfifot())
                .field("rfifot", &self.rfifot())
                .field("rfifot4", &self.rfifot4())
                .field("tfifot4", &self.tfifot4())
                .field("fifot4en", &self.fifot4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcrr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcrr {{ fifoe: {=bool:?}, rfiforst: {=bool:?}, tfiforst: {=bool:?}, dmae: {=bool:?}, tfifot: {:?}, rfifot: {:?}, rfifot4: {=u8:?}, tfifot4: {=u8:?}, fifot4en: {=bool:?} }}" , self . fifoe () , self . rfiforst () , self . tfiforst () , self . dmae () , self . tfifot () , self . rfifot () , self . rfifot4 () , self . tfifot4 () , self . fifot4en ())
        }
    }
    #[doc = "GPR Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr(pub u32);
    impl Gpr {
        #[doc = "A one-byte storage register."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "A one-byte storage register."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Gpr {
        #[inline(always)]
        fn default() -> Gpr {
            Gpr(0)
        }
    }
    impl core::fmt::Debug for Gpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpr {{ data: {=u8:?} }}", self.data())
        }
    }
    #[doc = "Idle Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdleCfg(pub u32);
    impl IdleCfg {
        #[doc = "Threshold for UART Receive Idle detection (in terms of bits)."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_idle_thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Threshold for UART Receive Idle detection (in terms of bits)."]
        #[inline(always)]
        pub const fn set_rx_idle_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_idle_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature."]
        #[inline(always)]
        pub const fn set_rx_idle_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_idle_cond(&self) -> super::vals::RxIdleCond {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::RxIdleCond::from_bits(val as u8)
        }
        #[doc = "IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle."]
        #[inline(always)]
        pub const fn set_rx_idle_cond(&mut self, val: super::vals::RxIdleCond) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UART receive enable. 0 - hold RX input to high, avoide wrong data input when config pinmux 1 - bypass RX input from PIN software should set it after config pinmux."]
        #[inline(always)]
        pub const fn set_rxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Threshold for UART transmit Idle detection (in terms of bits)."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_idle_thr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Threshold for UART transmit Idle detection (in terms of bits)."]
        #[inline(always)]
        pub const fn set_tx_idle_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "UART TX Idle Detect Enable 0 - Disable 1 - Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_idle_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "UART TX Idle Detect Enable 0 - Disable 1 - Enable."]
        #[inline(always)]
        pub const fn set_tx_idle_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_idle_cond(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE Detection Condition 0 - Treat as idle if TX pin is logic one 1 - Treat as idle if UART state machine state is idle."]
        #[inline(always)]
        pub const fn set_tx_idle_cond(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for IdleCfg {
        #[inline(always)]
        fn default() -> IdleCfg {
            IdleCfg(0)
        }
    }
    impl core::fmt::Debug for IdleCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IdleCfg")
                .field("rx_idle_thr", &self.rx_idle_thr())
                .field("rx_idle_en", &self.rx_idle_en())
                .field("rx_idle_cond", &self.rx_idle_cond())
                .field("rxen", &self.rxen())
                .field("tx_idle_thr", &self.tx_idle_thr())
                .field("tx_idle_en", &self.tx_idle_en())
                .field("tx_idle_cond", &self.tx_idle_cond())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IdleCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IdleCfg {{ rx_idle_thr: {=u8:?}, rx_idle_en: {=bool:?}, rx_idle_cond: {:?}, rxen: {=bool:?}, tx_idle_thr: {=u8:?}, tx_idle_en: {=bool:?}, tx_idle_cond: {=bool:?} }}" , self . rx_idle_thr () , self . rx_idle_en () , self . rx_idle_cond () , self . rxen () , self . tx_idle_thr () , self . tx_idle_en () , self . tx_idle_cond ())
        }
    }
    #[doc = "Interrupt Enable Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn erbi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_erbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable transmitter holding register interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ethei(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable transmitter holding register interrupt."]
        #[inline(always)]
        pub const fn set_ethei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable receiver line status interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn elsi(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable receiver line status interrupt."]
        #[inline(always)]
        pub const fn set_elsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
        #[must_use]
        #[inline(always)]
        pub const fn emsi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
        #[inline(always)]
        pub const fn set_emsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable DATA_LOST interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn edatlost(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "enable DATA_LOST interrupt."]
        #[inline(always)]
        pub const fn set_edatlost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "enable ADDR_MATCH_IDLE interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn eaddrm_idle(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "enable ADDR_MATCH_IDLE interrupt."]
        #[inline(always)]
        pub const fn set_eaddrm_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "enable ADDR_MATCH interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn eaddrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "enable ADDR_MATCH interrupt."]
        #[inline(always)]
        pub const fn set_eaddrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "enable transmit idle interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn etxidle(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "enable transmit idle interrupt."]
        #[inline(always)]
        pub const fn set_etxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn erxidle(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt."]
        #[inline(always)]
        pub const fn set_erxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("erbi", &self.erbi())
                .field("ethei", &self.ethei())
                .field("elsi", &self.elsi())
                .field("emsi", &self.emsi())
                .field("edatlost", &self.edatlost())
                .field("eaddrm_idle", &self.eaddrm_idle())
                .field("eaddrm", &self.eaddrm())
                .field("etxidle", &self.etxidle())
                .field("erxidle", &self.erxidle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ erbi: {=bool:?}, ethei: {=bool:?}, elsi: {=bool:?}, emsi: {=bool:?}, edatlost: {=bool:?}, eaddrm_idle: {=bool:?}, eaddrm: {=bool:?}, etxidle: {=bool:?}, erxidle: {=bool:?} }}" , self . erbi () , self . ethei () , self . elsi () , self . emsi () , self . edatlost () , self . eaddrm_idle () , self . eaddrm () , self . etxidle () , self . erxidle ())
        }
    }
    #[doc = "Interrupt Identification Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir(pub u32);
    impl Iir {
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[must_use]
        #[inline(always)]
        pub const fn intrid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[inline(always)]
        pub const fn set_intrid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn fifoed(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[inline(always)]
        pub const fn set_fifoed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle_flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR."]
        #[inline(always)]
        pub const fn set_rxidle_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Iir {
        #[inline(always)]
        fn default() -> Iir {
            Iir(0)
        }
    }
    impl core::fmt::Debug for Iir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iir")
                .field("intrid", &self.intrid())
                .field("fifoed", &self.fifoed())
                .field("rxidle_flag", &self.rxidle_flag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iir {{ intrid: {=u8:?}, fifoed: {=u8:?}, rxidle_flag: {=bool:?} }}",
                self.intrid(),
                self.fifoed(),
                self.rxidle_flag()
            )
        }
    }
    #[doc = "Interrupt Identification Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir2(pub u32);
    impl Iir2 {
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[must_use]
        #[inline(always)]
        pub const fn intrid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[inline(always)]
        pub const fn set_intrid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn fifoed(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[inline(always)]
        pub const fn set_fifoed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "assert if data lost before address match status, write one clear; It will not assert if no address match occurs."]
        #[must_use]
        #[inline(always)]
        pub const fn data_lost(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "assert if data lost before address match status, write one clear; It will not assert if no address match occurs."]
        #[inline(always)]
        pub const fn set_data_lost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "address match and idle irq status, assert at rx bus idle if address match event triggered. Write one clear;."]
        #[must_use]
        #[inline(always)]
        pub const fn addr_match_idle(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "address match and idle irq status, assert at rx bus idle if address match event triggered. Write one clear;."]
        #[inline(always)]
        pub const fn set_addr_match_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "address match irq status, assert if either address match(and enabled). Write one clear NOTE: the address byte may not moved by DMA at this point. User can wait next addr_match_idle irq for the whole data include address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr_match(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "address match irq status, assert if either address match(and enabled). Write one clear NOTE: the address byte may not moved by DMA at this point. User can wait next addr_match_idle irq for the whole data include address."]
        #[inline(always)]
        pub const fn set_addr_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART TX IDLE Flag, assert after txd high and then tx idle timeout, write one clear 0 - UART TX is busy 1 - UART TX is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn txidle_flag(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART TX IDLE Flag, assert after txd high and then tx idle timeout, write one clear 0 - UART TX is busy 1 - UART TX is idle."]
        #[inline(always)]
        pub const fn set_txidle_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART RX IDLE Flag, assert after rxd high and then rx idle timeout, write one clear 0 - UART RX is busy 1 - UART RX is idle."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle_flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART RX IDLE Flag, assert after rxd high and then rx idle timeout, write one clear 0 - UART RX is busy 1 - UART RX is idle."]
        #[inline(always)]
        pub const fn set_rxidle_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Iir2 {
        #[inline(always)]
        fn default() -> Iir2 {
            Iir2(0)
        }
    }
    impl core::fmt::Debug for Iir2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iir2")
                .field("intrid", &self.intrid())
                .field("fifoed", &self.fifoed())
                .field("data_lost", &self.data_lost())
                .field("addr_match_idle", &self.addr_match_idle())
                .field("addr_match", &self.addr_match())
                .field("txidle_flag", &self.txidle_flag())
                .field("rxidle_flag", &self.rxidle_flag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iir2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Iir2 {{ intrid: {=u8:?}, fifoed: {=u8:?}, data_lost: {=bool:?}, addr_match_idle: {=bool:?}, addr_match: {=bool:?}, txidle_flag: {=bool:?}, rxidle_flag: {=bool:?} }}" , self . intrid () , self . fifoed () , self . data_lost () , self . addr_match_idle () , self . addr_match () , self . txidle_flag () , self . rxidle_flag ())
        }
    }
    #[doc = "Line Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u32);
    impl Lcr {
        #[doc = "Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn wls(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits."]
        #[inline(always)]
        pub const fn set_wls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn stb(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits."]
        #[inline(always)]
        pub const fn set_stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
        #[must_use]
        #[inline(always)]
        pub const fn pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
        #[inline(always)]
        pub const fn set_pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
        #[must_use]
        #[inline(always)]
        pub const fn eps(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
        #[inline(always)]
        pub const fn set_eps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
        #[must_use]
        #[inline(always)]
        pub const fn sps(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
        #[inline(always)]
        pub const fn set_sps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break control."]
        #[must_use]
        #[inline(always)]
        pub const fn bc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Break control."]
        #[inline(always)]
        pub const fn set_bc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Divisor latch access bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Divisor latch access bit."]
        #[inline(always)]
        pub const fn set_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    impl core::fmt::Debug for Lcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcr")
                .field("wls", &self.wls())
                .field("stb", &self.stb())
                .field("pen", &self.pen())
                .field("eps", &self.eps())
                .field("sps", &self.sps())
                .field("bc", &self.bc())
                .field("dlab", &self.dlab())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lcr {{ wls: {=u8:?}, stb: {=bool:?}, pen: {=bool:?}, eps: {=bool:?}, sps: {=bool:?}, bc: {=bool:?}, dlab: {=bool:?} }}" , self . wls () , self . stb () , self . pen () , self . eps () , self . sps () , self . bc () , self . dlab ())
        }
    }
    #[doc = "Line Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u32);
    impl Lsr {
        #[doc = "Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun."]
        #[must_use]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun."]
        #[inline(always)]
        pub const fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error This bit is set when the received parity does not match with the parity selected in the LCR\\[5:4\\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error This bit is set when the received parity does not match with the parity selected in the LCR\\[5:4\\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn lbreak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn set_lbreak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1."]
        #[must_use]
        #[inline(always)]
        pub const fn thre(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1."]
        #[inline(always)]
        pub const fn set_thre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero."]
        #[must_use]
        #[inline(always)]
        pub const fn temt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero."]
        #[inline(always)]
        pub const fn set_temt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO."]
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "data bytes in txfifo not sent."]
        #[must_use]
        #[inline(always)]
        pub const fn tfifo_num(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "data bytes in txfifo not sent."]
        #[inline(always)]
        pub const fn set_tfifo_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "data bytes in rxfifo not read."]
        #[must_use]
        #[inline(always)]
        pub const fn rfifo_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "data bytes in rxfifo not read."]
        #[inline(always)]
        pub const fn set_rfifo_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "txidle after timeout, clear after tx idle condition not match."]
        #[must_use]
        #[inline(always)]
        pub const fn txidle(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "txidle after timeout, clear after tx idle condition not match."]
        #[inline(always)]
        pub const fn set_txidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "rxidle after timeout, clear after rx idle condition not match."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "rxidle after timeout, clear after rx idle condition not match."]
        #[inline(always)]
        pub const fn set_rxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Lsr {
        #[inline(always)]
        fn default() -> Lsr {
            Lsr(0)
        }
    }
    impl core::fmt::Debug for Lsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lsr")
                .field("dr", &self.dr())
                .field("oe", &self.oe())
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("lbreak", &self.lbreak())
                .field("thre", &self.thre())
                .field("temt", &self.temt())
                .field("errf", &self.errf())
                .field("tfifo_num", &self.tfifo_num())
                .field("rfifo_num", &self.rfifo_num())
                .field("txidle", &self.txidle())
                .field("rxidle", &self.rxidle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lsr {{ dr: {=bool:?}, oe: {=bool:?}, pe: {=bool:?}, fe: {=bool:?}, lbreak: {=bool:?}, thre: {=bool:?}, temt: {=bool:?}, errf: {=bool:?}, tfifo_num: {=u8:?}, rfifo_num: {=u8:?}, txidle: {=bool:?}, rxidle: {=bool:?} }}" , self . dr () , self . oe () , self . pe () , self . fe () , self . lbreak () , self . thre () , self . temt () , self . errf () , self . tfifo_num () , self . rfifo_num () , self . txidle () , self . rxidle ())
        }
    }
    #[doc = "Modem Control Register (."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW."]
        #[must_use]
        #[inline(always)]
        pub const fn rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW."]
        #[inline(always)]
        pub const fn set_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable loopback mode 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop_(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable loopback mode 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_loop_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS."]
        #[must_use]
        #[inline(always)]
        pub const fn afe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS."]
        #[inline(always)]
        pub const fn set_afe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    impl core::fmt::Debug for Mcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr")
                .field("rts", &self.rts())
                .field("loop_", &self.loop_())
                .field("afe", &self.afe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mcr {{ rts: {=bool:?}, loop_: {=bool:?}, afe: {=bool:?} }}",
                self.rts(),
                self.loop_(),
                self.afe()
            )
        }
    }
    #[doc = "moto system control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MotoCfg(pub u32);
    impl MotoCfg {
        #[doc = "set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set."]
        #[must_use]
        #[inline(always)]
        pub const fn txstop_insert(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to insert STOP bits between each tx byte till tx fifo empty. NOTE: there will be no 1.5/2 STOP bits if enabled this feature, LCR.STB should be set to 0 if this bit is set."]
        #[inline(always)]
        pub const fn set_txstop_insert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_clr_rfifo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable the feature that, clear rxfifo at tx trigger(sw or hw), avoid unexpected data in rxfifo."]
        #[inline(always)]
        pub const fn set_trg_clr_rfifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable trigger mode. software should push needed data into txbuffer frist, uart will not start transmission at this time. User should send trigger signal(by hw or sw), uart will send all data in txfifo till empty NOTE: the hw_trigger should be pulse signal from trig mux."]
        #[inline(always)]
        pub const fn set_trg_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "set to enable hardware trigger(trigger from moto is shared by other UART)."]
        #[must_use]
        #[inline(always)]
        pub const fn hwtrg_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable hardware trigger(trigger from moto is shared by other UART)."]
        #[inline(always)]
        pub const fn set_hwtrg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits."]
        #[must_use]
        #[inline(always)]
        pub const fn txstp_bits(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "if TXSTOP_INSERT is enabled, the STOP bits to be inserted between each byte. 0 for 1 bit; 0xFF for 256bits."]
        #[inline(always)]
        pub const fn set_txstp_bits(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "software trigger. User should avoid use sw/hw trigger at same time, otherwise result unknown. Hardware auto reset."]
        #[must_use]
        #[inline(always)]
        pub const fn swtrg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software trigger. User should avoid use sw/hw trigger at same time, otherwise result unknown. Hardware auto reset."]
        #[inline(always)]
        pub const fn set_swtrg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MotoCfg {
        #[inline(always)]
        fn default() -> MotoCfg {
            MotoCfg(0)
        }
    }
    impl core::fmt::Debug for MotoCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MotoCfg")
                .field("txstop_insert", &self.txstop_insert())
                .field("trg_clr_rfifo", &self.trg_clr_rfifo())
                .field("trg_mode", &self.trg_mode())
                .field("hwtrg_en", &self.hwtrg_en())
                .field("txstp_bits", &self.txstp_bits())
                .field("swtrg", &self.swtrg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MotoCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MotoCfg {{ txstop_insert: {=bool:?}, trg_clr_rfifo: {=bool:?}, trg_mode: {=bool:?}, hwtrg_en: {=bool:?}, txstp_bits: {=u8:?}, swtrg: {=bool:?} }}" , self . txstop_insert () , self . trg_clr_rfifo () , self . trg_mode () , self . hwtrg_en () , self . txstp_bits () , self . swtrg ())
        }
    }
    #[doc = "Modem Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u32);
    impl Msr {
        #[doc = "Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
        #[must_use]
        #[inline(always)]
        pub const fn dcts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
        #[inline(always)]
        pub const fn set_dcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    impl core::fmt::Debug for Msr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msr")
                .field("dcts", &self.dcts())
                .field("cts", &self.cts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Msr {{ dcts: {=bool:?}, cts: {=bool:?} }}",
                self.dcts(),
                self.cts()
            )
        }
    }
    #[doc = "Over Sample Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oscr(pub u32);
    impl Oscr {
        #[doc = "Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC."]
        #[must_use]
        #[inline(always)]
        pub const fn osc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC."]
        #[inline(always)]
        pub const fn set_osc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Oscr {
        #[inline(always)]
        fn default() -> Oscr {
            Oscr(0)
        }
    }
    impl core::fmt::Debug for Oscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oscr").field("osc", &self.osc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oscr {{ osc: {=u8:?} }}", self.osc())
        }
    }
    #[doc = "Receiver Buffer Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u32);
    impl Rbr {
        #[doc = "Receive data read port."]
        #[must_use]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive data read port."]
        #[inline(always)]
        pub const fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rbr {
        #[inline(always)]
        fn default() -> Rbr {
            Rbr(0)
        }
    }
    impl core::fmt::Debug for Rbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rbr").field("rbr", &self.rbr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rbr {{ rbr: {=u8:?} }}", self.rbr())
        }
    }
    #[doc = "Transmitter Holding Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Thr(pub u32);
    impl Thr {
        #[doc = "Transmit data write port."]
        #[must_use]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit data write port."]
        #[inline(always)]
        pub const fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Thr {
        #[inline(always)]
        fn default() -> Thr {
            Thr(0)
        }
    }
    impl core::fmt::Debug for Thr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Thr").field("thr", &self.thr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Thr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Thr {{ thr: {=u8:?} }}", self.thr())
        }
    }
}
pub mod vals {
    #[doc = "Transmitter FIFO trigger level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RxFifoTrigger {
        #[doc = "1 byte"]
        NOT_EMPTY = 0x0,
        #[doc = "1/4 full"]
        GT_ONE_QUARTER = 0x01,
        #[doc = "1/2 full"]
        GT_HALF = 0x02,
        #[doc = "3/4 full"]
        GT_THREE_QUARTER = 0x03,
    }
    impl RxFifoTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RxFifoTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RxFifoTrigger {
        #[inline(always)]
        fn from(val: u8) -> RxFifoTrigger {
            RxFifoTrigger::from_bits(val)
        }
    }
    impl From<RxFifoTrigger> for u8 {
        #[inline(always)]
        fn from(val: RxFifoTrigger) -> u8 {
            RxFifoTrigger::to_bits(val)
        }
    }
    #[doc = "IDLE Detection Condition."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RxIdleCond {
        #[doc = "Treat as idle if RX pin is logic one"]
        RXLINE_LOGIC_ONE = 0x0,
        #[doc = "Treat as idle if UART state machine state is idle"]
        STATE_MACHINE_IDLE = 0x01,
    }
    impl RxIdleCond {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RxIdleCond {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RxIdleCond {
        #[inline(always)]
        fn from(val: u8) -> RxIdleCond {
            RxIdleCond::from_bits(val)
        }
    }
    impl From<RxIdleCond> for u8 {
        #[inline(always)]
        fn from(val: RxIdleCond) -> u8 {
            RxIdleCond::to_bits(val)
        }
    }
    #[doc = "Receiver FIFO trigger level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TxFifoTrigger {
        #[doc = "1 byte or more empty space in fifo"]
        NOT_FULL = 0x0,
        #[doc = "3/4 full"]
        LT_THREE_QUARTER = 0x01,
        #[doc = "1/2 full"]
        LT_HALF = 0x02,
        #[doc = "1/4 full"]
        LT_ONE_QUARTER = 0x03,
    }
    impl TxFifoTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TxFifoTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TxFifoTrigger {
        #[inline(always)]
        fn from(val: u8) -> TxFifoTrigger {
            TxFifoTrigger::from_bits(val)
        }
    }
    impl From<TxFifoTrigger> for u8 {
        #[inline(always)]
        fn from(val: TxFifoTrigger) -> u8 {
            TxFifoTrigger::to_bits(val)
        }
    }
}
