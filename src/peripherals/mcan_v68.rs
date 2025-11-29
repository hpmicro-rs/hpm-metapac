#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MCAN0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcan {
    ptr: *mut u8,
}
unsafe impl Send for Mcan {}
unsafe impl Sync for Mcan {}
impl Mcan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "endian register."]
    #[inline(always)]
    pub const fn endn(self) -> crate::common::Reg<regs::Endn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set."]
    #[inline(always)]
    pub const fn dbtp(self) -> crate::common::Reg<regs::Dbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "test register."]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ram watchdog."]
    #[inline(always)]
    pub const fn rwd(self) -> crate::common::Reg<regs::Rwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CC control register."]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "nominal bit timing and prescaler register."]
    #[inline(always)]
    pub const fn nbtp(self) -> crate::common::Reg<regs::Nbtp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "timestamp counter configuration."]
    #[inline(always)]
    pub const fn tscc(self) -> crate::common::Reg<regs::Tscc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "timestamp counter value."]
    #[inline(always)]
    pub const fn tscv(self) -> crate::common::Reg<regs::Tscv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "timeout counter configuration."]
    #[inline(always)]
    pub const fn tocc(self) -> crate::common::Reg<regs::Tocc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "timeout counter value."]
    #[inline(always)]
    pub const fn tocv(self) -> crate::common::Reg<regs::Tocv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "error counter register."]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "protocol status register."]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "transmitter delay compensation."]
    #[inline(always)]
    pub const fn tdcr(self) -> crate::common::Reg<regs::Tdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "interrupt register."]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<regs::Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "interrupt line select."]
    #[inline(always)]
    pub const fn ils(self) -> crate::common::Reg<regs::Ils, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "interrupt line enable."]
    #[inline(always)]
    pub const fn ile(self) -> crate::common::Reg<regs::Ile, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "global filter configuration."]
    #[inline(always)]
    pub const fn gfc(self) -> crate::common::Reg<regs::Gfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "standard ID filter configuration."]
    #[inline(always)]
    pub const fn sidfc(self) -> crate::common::Reg<regs::Sidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "extended ID filter configuration."]
    #[inline(always)]
    pub const fn xidfc(self) -> crate::common::Reg<regs::Xidfc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "extended id and mask."]
    #[inline(always)]
    pub const fn xidam(self) -> crate::common::Reg<regs::Xidam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "high priority message status."]
    #[inline(always)]
    pub const fn hpms(self) -> crate::common::Reg<regs::Hpms, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "new data1."]
    #[inline(always)]
    pub const fn ndat1(self) -> crate::common::Reg<regs::Ndat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "new data2."]
    #[inline(always)]
    pub const fn ndat2(self) -> crate::common::Reg<regs::Ndat2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "rx fifo 0 configuration."]
    #[inline(always)]
    pub const fn rxf0c(self) -> crate::common::Reg<regs::Rxf0c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "rx fifo 0 status."]
    #[inline(always)]
    pub const fn rxf0s(self) -> crate::common::Reg<regs::Rxf0s, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "rx fifo0 acknowledge."]
    #[inline(always)]
    pub const fn rxf0a(self) -> crate::common::Reg<regs::Rxf0a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "rx buffer configuration."]
    #[inline(always)]
    pub const fn rxbc(self) -> crate::common::Reg<regs::Rxbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "rx fifo1 configuration."]
    #[inline(always)]
    pub const fn rxf1c(self) -> crate::common::Reg<regs::Rxf1c, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "rx fifo1 status."]
    #[inline(always)]
    pub const fn rxf1s(self) -> crate::common::Reg<regs::Rxf1s, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "rx fifo 1 acknowledge."]
    #[inline(always)]
    pub const fn rxf1a(self) -> crate::common::Reg<regs::Rxf1a, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "rx buffer/fifo element size configuration."]
    #[inline(always)]
    pub const fn rxesc(self) -> crate::common::Reg<regs::Rxesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "tx buffer configuration."]
    #[inline(always)]
    pub const fn txbc(self) -> crate::common::Reg<regs::Txbc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "tx fifo/queue status."]
    #[inline(always)]
    pub const fn txfqs(self) -> crate::common::Reg<regs::Txfqs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "tx buffer element size configuration."]
    #[inline(always)]
    pub const fn txesc(self) -> crate::common::Reg<regs::Txesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "tx buffer request pending."]
    #[inline(always)]
    pub const fn txbrp(self) -> crate::common::Reg<regs::Txbrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "tx buffer add request."]
    #[inline(always)]
    pub const fn txbar(self) -> crate::common::Reg<regs::Txbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "tx buffer cancellation request."]
    #[inline(always)]
    pub const fn txbcr(self) -> crate::common::Reg<regs::Txbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "tx buffer transmission occurred."]
    #[inline(always)]
    pub const fn txbto(self) -> crate::common::Reg<regs::Txbto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "tx buffer cancellation finished."]
    #[inline(always)]
    pub const fn txbcf(self) -> crate::common::Reg<regs::Txbcf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "tx buffer transmission interrupt enable."]
    #[inline(always)]
    pub const fn txbtie(self) -> crate::common::Reg<regs::Txbtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "tx buffer cancellation finished interrupt enable."]
    #[inline(always)]
    pub const fn txbcie(self) -> crate::common::Reg<regs::Txbcie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "tx event fifo configuration."]
    #[inline(always)]
    pub const fn txefc(self) -> crate::common::Reg<regs::Txefc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "tx event fifo status."]
    #[inline(always)]
    pub const fn txefs(self) -> crate::common::Reg<regs::Txefs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "tx event fifo acknowledge."]
    #[inline(always)]
    pub const fn txefa(self) -> crate::common::Reg<regs::Txefa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ts_sel(self, n: usize) -> crate::common::Reg<regs::TsSel, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "core release register."]
    #[inline(always)]
    pub const fn crel(self) -> crate::common::Reg<regs::Crel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "timestamp configuration."]
    #[inline(always)]
    pub const fn tscfg(self) -> crate::common::Reg<regs::Tscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "timestamp status1."]
    #[inline(always)]
    pub const fn tss1(self) -> crate::common::Reg<regs::Tss1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "timestamp status2."]
    #[inline(always)]
    pub const fn tss2(self) -> crate::common::Reg<regs::Tss2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "actual timebase."]
    #[inline(always)]
    pub const fn atb(self) -> crate::common::Reg<regs::Atb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "actual timebase high."]
    #[inline(always)]
    pub const fn atbh(self) -> crate::common::Reg<regs::Atbh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "global control."]
    #[inline(always)]
    pub const fn glb_ctl(self) -> crate::common::Reg<regs::GlbCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "global status."]
    #[inline(always)]
    pub const fn glb_status(self) -> crate::common::Reg<regs::GlbStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn message_buff(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MessageBuff, crate::common::RW> {
        assert!(n < 640usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 4usize) as _)
        }
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
    #[doc = "actual timebase."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atb(pub u32);
    impl Atb {
        #[doc = "timebase for timestamp generation 31-0."]
        #[must_use]
        #[inline(always)]
        pub const fn tb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "timebase for timestamp generation 31-0."]
        #[inline(always)]
        pub const fn set_tb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Atb {
        #[inline(always)]
        fn default() -> Atb {
            Atb(0)
        }
    }
    impl core::fmt::Debug for Atb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atb").field("tb", &self.tb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Atb {{ tb: {=u32:?} }}", self.tb())
        }
    }
    #[doc = "actual timebase high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atbh(pub u32);
    impl Atbh {
        #[doc = "timebase for timestamp generation 63-32."]
        #[must_use]
        #[inline(always)]
        pub const fn tbh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "timebase for timestamp generation 63-32."]
        #[inline(always)]
        pub const fn set_tbh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Atbh {
        #[inline(always)]
        fn default() -> Atbh {
            Atbh(0)
        }
    }
    impl core::fmt::Debug for Atbh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atbh").field("tbh", &self.tbh()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atbh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Atbh {{ tbh: {=u32:?} }}", self.tbh())
        }
    }
    #[doc = "CC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "Initialization 0= Normal Operation 1= Initialization is started."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization 0= Normal Operation 1= Initialization is started."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)."]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’)."]
        #[inline(always)]
        pub const fn set_cce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active."]
        #[must_use]
        #[inline(always)]
        pub const fn asm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active."]
        #[inline(always)]
        pub const fn set_asm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_CAN may be set in power down by stopping m_can_hclk and m_can_cclk."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_CAN may be set in power down by stopping m_can_hclk and m_can_cclk."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
        #[must_use]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
        #[inline(always)]
        pub const fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled."]
        #[inline(always)]
        pub const fn set_mon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled."]
        #[inline(always)]
        pub const fn set_dar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn test(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled."]
        #[inline(always)]
        pub const fn set_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FD Operation Enable 0= FD operation disabled 1= FD operation enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn fdoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD Operation Enable 0= FD operation disabled 1= FD operation enabled."]
        #[inline(always)]
        pub const fn set_fdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
        #[must_use]
        #[inline(always)]
        pub const fn brse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated."]
        #[inline(always)]
        pub const fn set_brse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
        #[must_use]
        #[inline(always)]
        pub const fn utsu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis."]
        #[inline(always)]
        pub const fn set_utsu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn wmm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO."]
        #[inline(always)]
        pub const fn set_wmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
        #[must_use]
        #[inline(always)]
        pub const fn pxhd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition."]
        #[inline(always)]
        pub const fn set_pxhd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn efbi(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization."]
        #[inline(always)]
        pub const fn set_efbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn txp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled."]
        #[inline(always)]
        pub const fn set_txp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
        #[must_use]
        #[inline(always)]
        pub const fn niso(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015."]
        #[inline(always)]
        pub const fn set_niso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    impl core::fmt::Debug for Cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccr")
                .field("init", &self.init())
                .field("cce", &self.cce())
                .field("asm", &self.asm())
                .field("csa", &self.csa())
                .field("csr", &self.csr())
                .field("mon", &self.mon())
                .field("dar", &self.dar())
                .field("test", &self.test())
                .field("fdoe", &self.fdoe())
                .field("brse", &self.brse())
                .field("utsu", &self.utsu())
                .field("wmm", &self.wmm())
                .field("pxhd", &self.pxhd())
                .field("efbi", &self.efbi())
                .field("txp", &self.txp())
                .field("niso", &self.niso())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cccr {{ init: {=bool:?}, cce: {=bool:?}, asm: {=bool:?}, csa: {=bool:?}, csr: {=bool:?}, mon: {=bool:?}, dar: {=bool:?}, test: {=bool:?}, fdoe: {=bool:?}, brse: {=bool:?}, utsu: {=bool:?}, wmm: {=bool:?}, pxhd: {=bool:?}, efbi: {=bool:?}, txp: {=bool:?}, niso: {=bool:?} }}" , self . init () , self . cce () , self . asm () , self . csa () , self . csr () , self . mon () , self . dar () , self . test () , self . fdoe () , self . brse () , self . utsu () , self . wmm () , self . pxhd () , self . efbi () , self . txp () , self . niso ())
        }
    }
    #[doc = "core release register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crel(pub u32);
    impl Crel {
        #[doc = "Timestamp Day Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
        #[must_use]
        #[inline(always)]
        pub const fn day(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Day Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
        #[inline(always)]
        pub const fn set_day(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Timestamp Month Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
        #[must_use]
        #[inline(always)]
        pub const fn mon(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timestamp Month Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
        #[inline(always)]
        pub const fn set_mon(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Timestamp Year One digit, BCD-coded. This field is set by generic parameter on synthesis."]
        #[must_use]
        #[inline(always)]
        pub const fn year(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Year One digit, BCD-coded. This field is set by generic parameter on synthesis."]
        #[inline(always)]
        pub const fn set_year(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Sub-step of Core Release One digit, BCD-coded."]
        #[must_use]
        #[inline(always)]
        pub const fn substep(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Sub-step of Core Release One digit, BCD-coded."]
        #[inline(always)]
        pub const fn set_substep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Step of Core Release One digit, BCD-coded."]
        #[must_use]
        #[inline(always)]
        pub const fn step(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Step of Core Release One digit, BCD-coded."]
        #[inline(always)]
        pub const fn set_step(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Core Release One digit, BCD-coded."]
        #[must_use]
        #[inline(always)]
        pub const fn rel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Core Release One digit, BCD-coded."]
        #[inline(always)]
        pub const fn set_rel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Crel {
        #[inline(always)]
        fn default() -> Crel {
            Crel(0)
        }
    }
    impl core::fmt::Debug for Crel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crel")
                .field("day", &self.day())
                .field("mon", &self.mon())
                .field("year", &self.year())
                .field("substep", &self.substep())
                .field("step", &self.step())
                .field("rel", &self.rel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Crel {{ day: {=u8:?}, mon: {=u8:?}, year: {=u8:?}, substep: {=u8:?}, step: {=u8:?}, rel: {=u8:?} }}" , self . day () , self . mon () , self . year () , self . substep () , self . step () , self . rel ())
        }
    }
    #[doc = "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbtp(pub u32);
    impl Dbtp {
        #[doc = "Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[must_use]
        #[inline(always)]
        pub const fn dsjw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub const fn set_dsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[must_use]
        #[inline(always)]
        pub const fn dtseg2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub const fn set_dtseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[must_use]
        #[inline(always)]
        pub const fn dtseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub const fn set_dtseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[must_use]
        #[inline(always)]
        pub const fn dbrp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub const fn set_dbrp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tdc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled."]
        #[inline(always)]
        pub const fn set_tdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Dbtp {
        #[inline(always)]
        fn default() -> Dbtp {
            Dbtp(0)
        }
    }
    impl core::fmt::Debug for Dbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbtp")
                .field("dsjw", &self.dsjw())
                .field("dtseg2", &self.dtseg2())
                .field("dtseg1", &self.dtseg1())
                .field("dbrp", &self.dbrp())
                .field("tdc", &self.tdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbtp {{ dsjw: {=u8:?}, dtseg2: {=u8:?}, dtseg1: {=u8:?}, dbrp: {=u8:?}, tdc: {=bool:?} }}" , self . dsjw () , self . dtseg2 () , self . dtseg1 () , self . dbrp () , self . tdc ())
        }
    }
    #[doc = "error counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecr(pub u32);
    impl Ecr {
        #[doc = "Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255 Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
        #[must_use]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255 Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
        #[inline(always)]
        pub const fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127."]
        #[must_use]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127."]
        #[inline(always)]
        pub const fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128."]
        #[must_use]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128."]
        #[inline(always)]
        pub const fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CAN Error Logging The counter is incremented each time when a CAN protocol error causes the 8-bit Transmit Error Counter TEC or the 7-bit Receive Error Counter REC to be incremented. The counter is also incremented when the Bus_Off limit is reached. It is not incremented when only RP is set without changing REC. The increment of CEL follows after the increment of REC or TEC. The counter is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: Byte access: Reading byte 2 will reset CEL to zero, reading bytes 3/1/0 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn cel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "CAN Error Logging The counter is incremented each time when a CAN protocol error causes the 8-bit Transmit Error Counter TEC or the 7-bit Receive Error Counter REC to be incremented. The counter is also incremented when the Bus_Off limit is reached. It is not incremented when only RP is set without changing REC. The increment of CEL follows after the increment of REC or TEC. The counter is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: Byte access: Reading byte 2 will reset CEL to zero, reading bytes 3/1/0 has no impact."]
        #[inline(always)]
        pub const fn set_cel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ecr {
        #[inline(always)]
        fn default() -> Ecr {
            Ecr(0)
        }
    }
    impl core::fmt::Debug for Ecr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecr")
                .field("tec", &self.tec())
                .field("rec", &self.rec())
                .field("rp", &self.rp())
                .field("cel", &self.cel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ecr {{ tec: {=u8:?}, rec: {=u8:?}, rp: {=bool:?}, cel: {=u8:?} }}",
                self.tec(),
                self.rec(),
                self.rp(),
                self.cel()
            )
        }
    }
    #[doc = "endian register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endn(pub u32);
    impl Endn {
        #[doc = "Endianness Test Value The endianness test value is 0x87654321."]
        #[must_use]
        #[inline(always)]
        pub const fn evt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Endianness Test Value The endianness test value is 0x87654321."]
        #[inline(always)]
        pub const fn set_evt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Endn {
        #[inline(always)]
        fn default() -> Endn {
            Endn(0)
        }
    }
    impl core::fmt::Debug for Endn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Endn").field("evt", &self.evt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Endn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Endn {{ evt: {=u32:?} }}", self.evt())
        }
    }
    #[doc = "global filter configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gfc(pub u32);
    impl Gfc {
        #[doc = "Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs."]
        #[must_use]
        #[inline(always)]
        pub const fn rrfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs."]
        #[inline(always)]
        pub const fn set_rrfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs."]
        #[must_use]
        #[inline(always)]
        pub const fn rrfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs."]
        #[inline(always)]
        pub const fn set_rrfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject."]
        #[must_use]
        #[inline(always)]
        pub const fn anfe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject."]
        #[inline(always)]
        pub const fn set_anfe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject."]
        #[must_use]
        #[inline(always)]
        pub const fn anfs(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject."]
        #[inline(always)]
        pub const fn set_anfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Gfc {
        #[inline(always)]
        fn default() -> Gfc {
            Gfc(0)
        }
    }
    impl core::fmt::Debug for Gfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gfc")
                .field("rrfe", &self.rrfe())
                .field("rrfs", &self.rrfs())
                .field("anfe", &self.anfe())
                .field("anfs", &self.anfs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gfc {{ rrfe: {=bool:?}, rrfs: {=bool:?}, anfe: {=u8:?}, anfs: {=u8:?} }}",
                self.rrfe(),
                self.rrfs(),
                self.anfe(),
                self.anfs()
            )
        }
    }
    #[doc = "global control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCtl(pub u32);
    impl GlbCtl {
        #[doc = "external timestamp select. each CAN block has 4 timestamp input, this register is used to select one of them as timestame if TSCFG.TBCS is set to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tsu_tbin_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "external timestamp select. each CAN block has 4 timestamp input, this register is used to select one of them as timestame if TSCFG.TBCS is set to 1."]
        #[inline(always)]
        pub const fn set_tsu_tbin_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "standby polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn stby_pol(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "standby polarity selection."]
        #[inline(always)]
        pub const fn set_stby_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0."]
        #[must_use]
        #[inline(always)]
        pub const fn stby_clr_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0."]
        #[inline(always)]
        pub const fn set_stby_clr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "m_can standby control."]
        #[must_use]
        #[inline(always)]
        pub const fn m_can_stby(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "m_can standby control."]
        #[inline(always)]
        pub const fn set_m_can_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GlbCtl {
        #[inline(always)]
        fn default() -> GlbCtl {
            GlbCtl(0)
        }
    }
    impl core::fmt::Debug for GlbCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlbCtl")
                .field("tsu_tbin_sel", &self.tsu_tbin_sel())
                .field("stby_pol", &self.stby_pol())
                .field("stby_clr_en", &self.stby_clr_en())
                .field("m_can_stby", &self.m_can_stby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GlbCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GlbCtl {{ tsu_tbin_sel: {=u8:?}, stby_pol: {=bool:?}, stby_clr_en: {=bool:?}, m_can_stby: {=bool:?} }}" , self . tsu_tbin_sel () , self . stby_pol () , self . stby_clr_en () , self . m_can_stby ())
        }
    }
    #[doc = "global status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbStatus(pub u32);
    impl GlbStatus {
        #[doc = "m_can interrupt status0."]
        #[must_use]
        #[inline(always)]
        pub const fn m_can_int0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "m_can interrupt status0."]
        #[inline(always)]
        pub const fn set_m_can_int0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "m_can interrupt status1."]
        #[must_use]
        #[inline(always)]
        pub const fn m_can_int1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "m_can interrupt status1."]
        #[inline(always)]
        pub const fn set_m_can_int1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for GlbStatus {
        #[inline(always)]
        fn default() -> GlbStatus {
            GlbStatus(0)
        }
    }
    impl core::fmt::Debug for GlbStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlbStatus")
                .field("m_can_int0", &self.m_can_int0())
                .field("m_can_int1", &self.m_can_int1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GlbStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GlbStatus {{ m_can_int0: {=bool:?}, m_can_int1: {=bool:?} }}",
                self.m_can_int0(),
                self.m_can_int1()
            )
        }
    }
    #[doc = "high priority message status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpms(pub u32);
    impl Hpms {
        #[doc = "Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= ‘1’."]
        #[must_use]
        #[inline(always)]
        pub const fn bidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI\\[1\\]
= ‘1’."]
        #[inline(always)]
        pub const fn set_bidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1."]
        #[must_use]
        #[inline(always)]
        pub const fn msi(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1."]
        #[inline(always)]
        pub const fn set_msi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List."]
        #[must_use]
        #[inline(always)]
        pub const fn flst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List."]
        #[inline(always)]
        pub const fn set_flst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Hpms {
        #[inline(always)]
        fn default() -> Hpms {
            Hpms(0)
        }
    }
    impl core::fmt::Debug for Hpms {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hpms")
                .field("bidx", &self.bidx())
                .field("msi", &self.msi())
                .field("fidx", &self.fidx())
                .field("flst", &self.flst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hpms {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hpms {{ bidx: {=u8:?}, msi: {=u8:?}, fidx: {=u8:?}, flst: {=bool:?} }}",
                self.bidx(),
                self.msi(),
                self.fidx(),
                self.flst()
            )
        }
    }
    #[doc = "interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ie(pub u32);
    impl Ie {
        #[doc = "Rx FIFO 0 New Message Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0ne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 New Message Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf0ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0we(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf0we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx FIFO 0 Full Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0fe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Full Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf0fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0le(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf0le(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1ne(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf1ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1we(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf1we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx FIFO 1 Full Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1fe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Full Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf1fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1le(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rf1le(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High Priority Message Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hpme(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Interrupt Enable."]
        #[inline(always)]
        pub const fn set_hpme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tce(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcfe(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tcfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfee(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tfee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tefne(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tefne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tefwe(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tefwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn teffe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Interrupt Enable."]
        #[inline(always)]
        pub const fn set_teffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Event Lost Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tefle(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Event Lost Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tefle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tswe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tswe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mrafe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Interrupt Enable."]
        #[inline(always)]
        pub const fn set_mrafe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tooe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn drxe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable."]
        #[inline(always)]
        pub const fn set_drxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bece(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Enable."]
        #[inline(always)]
        pub const fn set_bece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn beue(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Enable."]
        #[inline(always)]
        pub const fn set_beue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eloe(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_eloe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn epe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Enable."]
        #[inline(always)]
        pub const fn set_epe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ewe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ewe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn boe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status Interrupt Enable."]
        #[inline(always)]
        pub const fn set_boe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Enable."]
        #[inline(always)]
        pub const fn set_wdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn peae(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Enable."]
        #[inline(always)]
        pub const fn set_peae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pede(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Enable."]
        #[inline(always)]
        pub const fn set_pede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arae(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Enable."]
        #[inline(always)]
        pub const fn set_arae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ie {
        #[inline(always)]
        fn default() -> Ie {
            Ie(0)
        }
    }
    impl core::fmt::Debug for Ie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ie")
                .field("rf0ne", &self.rf0ne())
                .field("rf0we", &self.rf0we())
                .field("rf0fe", &self.rf0fe())
                .field("rf0le", &self.rf0le())
                .field("rf1ne", &self.rf1ne())
                .field("rf1we", &self.rf1we())
                .field("rf1fe", &self.rf1fe())
                .field("rf1le", &self.rf1le())
                .field("hpme", &self.hpme())
                .field("tce", &self.tce())
                .field("tcfe", &self.tcfe())
                .field("tfee", &self.tfee())
                .field("tefne", &self.tefne())
                .field("tefwe", &self.tefwe())
                .field("teffe", &self.teffe())
                .field("tefle", &self.tefle())
                .field("tswe", &self.tswe())
                .field("mrafe", &self.mrafe())
                .field("tooe", &self.tooe())
                .field("drxe", &self.drxe())
                .field("bece", &self.bece())
                .field("beue", &self.beue())
                .field("eloe", &self.eloe())
                .field("epe", &self.epe())
                .field("ewe", &self.ewe())
                .field("boe", &self.boe())
                .field("wdie", &self.wdie())
                .field("peae", &self.peae())
                .field("pede", &self.pede())
                .field("arae", &self.arae())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ie {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ie {{ rf0ne: {=bool:?}, rf0we: {=bool:?}, rf0fe: {=bool:?}, rf0le: {=bool:?}, rf1ne: {=bool:?}, rf1we: {=bool:?}, rf1fe: {=bool:?}, rf1le: {=bool:?}, hpme: {=bool:?}, tce: {=bool:?}, tcfe: {=bool:?}, tfee: {=bool:?}, tefne: {=bool:?}, tefwe: {=bool:?}, teffe: {=bool:?}, tefle: {=bool:?}, tswe: {=bool:?}, mrafe: {=bool:?}, tooe: {=bool:?}, drxe: {=bool:?}, bece: {=bool:?}, beue: {=bool:?}, eloe: {=bool:?}, epe: {=bool:?}, ewe: {=bool:?}, boe: {=bool:?}, wdie: {=bool:?}, peae: {=bool:?}, pede: {=bool:?}, arae: {=bool:?} }}" , self . rf0ne () , self . rf0we () , self . rf0fe () , self . rf0le () , self . rf1ne () , self . rf1we () , self . rf1fe () , self . rf1le () , self . hpme () , self . tce () , self . tcfe () , self . tfee () , self . tefne () , self . tefwe () , self . teffe () , self . tefle () , self . tswe () , self . mrafe () , self . tooe () , self . drxe () , self . bece () , self . beue () , self . eloe () , self . epe () , self . ewe () , self . boe () , self . wdie () , self . peae () , self . pede () , self . arae ())
        }
    }
    #[doc = "interrupt line enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ile(pub u32);
    impl Ile {
        #[doc = "Enable Interrupt Line 0 0= Interrupt line m_can_int0 disabled 1= Interrupt line m_can_int0 enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn eint0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 0 0= Interrupt line m_can_int0 disabled 1= Interrupt line m_can_int0 enabled."]
        #[inline(always)]
        pub const fn set_eint0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Interrupt Line 1 0= Interrupt line m_can_int1 disabled 1= Interrupt line m_can_int1 enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn eint1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt Line 1 0= Interrupt line m_can_int1 disabled 1= Interrupt line m_can_int1 enabled."]
        #[inline(always)]
        pub const fn set_eint1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ile {
        #[inline(always)]
        fn default() -> Ile {
            Ile(0)
        }
    }
    impl core::fmt::Debug for Ile {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ile")
                .field("eint0", &self.eint0())
                .field("eint1", &self.eint1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ile {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ile {{ eint0: {=bool:?}, eint1: {=bool:?} }}",
                self.eint0(),
                self.eint1()
            )
        }
    }
    #[doc = "interrupt line select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ils(pub u32);
    impl Ils {
        #[doc = "Rx FIFO 0 New Message Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0nl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 New Message Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf0nl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0wl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf0wl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx FIFO 0 Full Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0fl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Full Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf0fl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0ll(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf0ll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1nl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf1nl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1wl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf1wl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx FIFO 1 Full Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1fl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Full Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf1fl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1ll(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Line."]
        #[inline(always)]
        pub const fn set_rf1ll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High Priority Message Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn hpml(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message Interrupt Line."]
        #[inline(always)]
        pub const fn set_hpml(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tcl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed Interrupt Line."]
        #[inline(always)]
        pub const fn set_tcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tcfl(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line."]
        #[inline(always)]
        pub const fn set_tcfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tfel(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty Interrupt Line."]
        #[inline(always)]
        pub const fn set_tfel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tefnl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line."]
        #[inline(always)]
        pub const fn set_tefnl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tefwl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line."]
        #[inline(always)]
        pub const fn set_tefwl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn teffl(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full Interrupt Line."]
        #[inline(always)]
        pub const fn set_teffl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Event Lost Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tefll(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Event Lost Interrupt Line."]
        #[inline(always)]
        pub const fn set_tefll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tswl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound Interrupt Line."]
        #[inline(always)]
        pub const fn set_tswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn mrafl(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure Interrupt Line."]
        #[inline(always)]
        pub const fn set_mrafl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn tool(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred Interrupt Line."]
        #[inline(always)]
        pub const fn set_tool(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn drxl(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line."]
        #[inline(always)]
        pub const fn set_drxl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn becl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Interrupt Line."]
        #[inline(always)]
        pub const fn set_becl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn beul(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Interrupt Line."]
        #[inline(always)]
        pub const fn set_beul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn elol(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow Interrupt Line."]
        #[inline(always)]
        pub const fn set_elol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn epl(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Line."]
        #[inline(always)]
        pub const fn set_epl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn ewl(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status Interrupt Line."]
        #[inline(always)]
        pub const fn set_ewl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn bol(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status Interrupt Line."]
        #[inline(always)]
        pub const fn set_bol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt Line."]
        #[must_use]
        #[inline(always)]
        pub const fn wdil(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt Line."]
        #[inline(always)]
        pub const fn set_wdil(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase Line."]
        #[must_use]
        #[inline(always)]
        pub const fn peal(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase Line."]
        #[inline(always)]
        pub const fn set_peal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase Line."]
        #[must_use]
        #[inline(always)]
        pub const fn pedl(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase Line."]
        #[inline(always)]
        pub const fn set_pedl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address Line."]
        #[must_use]
        #[inline(always)]
        pub const fn aral(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address Line."]
        #[inline(always)]
        pub const fn set_aral(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ils {
        #[inline(always)]
        fn default() -> Ils {
            Ils(0)
        }
    }
    impl core::fmt::Debug for Ils {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ils")
                .field("rf0nl", &self.rf0nl())
                .field("rf0wl", &self.rf0wl())
                .field("rf0fl", &self.rf0fl())
                .field("rf0ll", &self.rf0ll())
                .field("rf1nl", &self.rf1nl())
                .field("rf1wl", &self.rf1wl())
                .field("rf1fl", &self.rf1fl())
                .field("rf1ll", &self.rf1ll())
                .field("hpml", &self.hpml())
                .field("tcl", &self.tcl())
                .field("tcfl", &self.tcfl())
                .field("tfel", &self.tfel())
                .field("tefnl", &self.tefnl())
                .field("tefwl", &self.tefwl())
                .field("teffl", &self.teffl())
                .field("tefll", &self.tefll())
                .field("tswl", &self.tswl())
                .field("mrafl", &self.mrafl())
                .field("tool", &self.tool())
                .field("drxl", &self.drxl())
                .field("becl", &self.becl())
                .field("beul", &self.beul())
                .field("elol", &self.elol())
                .field("epl", &self.epl())
                .field("ewl", &self.ewl())
                .field("bol", &self.bol())
                .field("wdil", &self.wdil())
                .field("peal", &self.peal())
                .field("pedl", &self.pedl())
                .field("aral", &self.aral())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ils {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ils {{ rf0nl: {=bool:?}, rf0wl: {=bool:?}, rf0fl: {=bool:?}, rf0ll: {=bool:?}, rf1nl: {=bool:?}, rf1wl: {=bool:?}, rf1fl: {=bool:?}, rf1ll: {=bool:?}, hpml: {=bool:?}, tcl: {=bool:?}, tcfl: {=bool:?}, tfel: {=bool:?}, tefnl: {=bool:?}, tefwl: {=bool:?}, teffl: {=bool:?}, tefll: {=bool:?}, tswl: {=bool:?}, mrafl: {=bool:?}, tool: {=bool:?}, drxl: {=bool:?}, becl: {=bool:?}, beul: {=bool:?}, elol: {=bool:?}, epl: {=bool:?}, ewl: {=bool:?}, bol: {=bool:?}, wdil: {=bool:?}, peal: {=bool:?}, pedl: {=bool:?}, aral: {=bool:?} }}" , self . rf0nl () , self . rf0wl () , self . rf0fl () , self . rf0ll () , self . rf1nl () , self . rf1wl () , self . rf1fl () , self . rf1ll () , self . hpml () , self . tcl () , self . tcfl () , self . tfel () , self . tefnl () , self . tefwl () , self . teffl () , self . tefll () , self . tswl () , self . mrafl () , self . tool () , self . drxl () , self . becl () , self . beul () , self . elol () , self . epl () , self . ewl () , self . bol () , self . wdil () , self . peal () , self . pedl () , self . aral ())
        }
    }
    #[doc = "interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ir(pub u32);
    impl Ir {
        #[doc = "Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0n(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0."]
        #[inline(always)]
        pub const fn set_rf0n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0w(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark."]
        #[inline(always)]
        pub const fn set_rf0w(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full."]
        #[inline(always)]
        pub const fn set_rf0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0l(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero."]
        #[inline(always)]
        pub const fn set_rf0l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1n(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1."]
        #[inline(always)]
        pub const fn set_rf1n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1w(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark."]
        #[inline(always)]
        pub const fn set_rf1w(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1f(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full."]
        #[inline(always)]
        pub const fn set_rf1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1l(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero."]
        #[inline(always)]
        pub const fn set_rf1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "High Priority Message 0= No high priority message received 1= High priority message received."]
        #[must_use]
        #[inline(always)]
        pub const fn hpm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "High Priority Message 0= No high priority message received 1= High priority message received."]
        #[inline(always)]
        pub const fn set_hpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmission Completed 0= No transmission completed 1= Transmission completed."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Completed 0= No transmission completed 1= Transmission completed."]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished."]
        #[must_use]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished."]
        #[inline(always)]
        pub const fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty."]
        #[inline(always)]
        pub const fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element."]
        #[must_use]
        #[inline(always)]
        pub const fn tefn(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element."]
        #[inline(always)]
        pub const fn set_tefn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn tefw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark."]
        #[inline(always)]
        pub const fn set_tefw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn teff(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full."]
        #[inline(always)]
        pub const fn set_teff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
        #[inline(always)]
        pub const fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around."]
        #[must_use]
        #[inline(always)]
        pub const fn tsw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around."]
        #[inline(always)]
        pub const fn set_tsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Message RAM Access Failure The flag is set, when the Rx Handler .has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. .was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_CAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn mraf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Message RAM Access Failure The flag is set, when the Rx Handler .has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. .was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_CAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred."]
        #[inline(always)]
        pub const fn set_mraf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Timeout Occurred 0= No timeout 1= Timeout reached."]
        #[must_use]
        #[inline(always)]
        pub const fn too(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout Occurred 0= No timeout 1= Timeout reached."]
        #[inline(always)]
        pub const fn set_too(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into an Rx Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn drx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into an Rx Buffer."]
        #[inline(always)]
        pub const fn set_drx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_can_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)."]
        #[must_use]
        #[inline(always)]
        pub const fn bec(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_can_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)."]
        #[inline(always)]
        pub const fn set_bec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_can_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to ‘1’. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)."]
        #[must_use]
        #[inline(always)]
        pub const fn beu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_can_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to ‘1’. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)."]
        #[inline(always)]
        pub const fn set_beu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn elo(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred."]
        #[inline(always)]
        pub const fn set_elo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed."]
        #[must_use]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed."]
        #[inline(always)]
        pub const fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed."]
        #[must_use]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed."]
        #[inline(always)]
        pub const fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed."]
        #[must_use]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed."]
        #[inline(always)]
        pub const fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY."]
        #[must_use]
        #[inline(always)]
        pub const fn wdi(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY."]
        #[inline(always)]
        pub const fn set_wdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC ≠ 0,7)."]
        #[must_use]
        #[inline(always)]
        pub const fn pea(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC ≠ 0,7)."]
        #[inline(always)]
        pub const fn set_pea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC ≠ 0,7)."]
        #[must_use]
        #[inline(always)]
        pub const fn ped(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC ≠ 0,7)."]
        #[inline(always)]
        pub const fn set_ped(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Access to Reserved Address 0= No access to reserved address occurred 1= Access to reserved address occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ara(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Access to Reserved Address 0= No access to reserved address occurred 1= Access to reserved address occurred."]
        #[inline(always)]
        pub const fn set_ara(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ir {
        #[inline(always)]
        fn default() -> Ir {
            Ir(0)
        }
    }
    impl core::fmt::Debug for Ir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ir")
                .field("rf0n", &self.rf0n())
                .field("rf0w", &self.rf0w())
                .field("rf0f", &self.rf0f())
                .field("rf0l", &self.rf0l())
                .field("rf1n", &self.rf1n())
                .field("rf1w", &self.rf1w())
                .field("rf1f", &self.rf1f())
                .field("rf1l", &self.rf1l())
                .field("hpm", &self.hpm())
                .field("tc", &self.tc())
                .field("tcf", &self.tcf())
                .field("tfe", &self.tfe())
                .field("tefn", &self.tefn())
                .field("tefw", &self.tefw())
                .field("teff", &self.teff())
                .field("tefl", &self.tefl())
                .field("tsw", &self.tsw())
                .field("mraf", &self.mraf())
                .field("too", &self.too())
                .field("drx", &self.drx())
                .field("bec", &self.bec())
                .field("beu", &self.beu())
                .field("elo", &self.elo())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("wdi", &self.wdi())
                .field("pea", &self.pea())
                .field("ped", &self.ped())
                .field("ara", &self.ara())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ir {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ir {{ rf0n: {=bool:?}, rf0w: {=bool:?}, rf0f: {=bool:?}, rf0l: {=bool:?}, rf1n: {=bool:?}, rf1w: {=bool:?}, rf1f: {=bool:?}, rf1l: {=bool:?}, hpm: {=bool:?}, tc: {=bool:?}, tcf: {=bool:?}, tfe: {=bool:?}, tefn: {=bool:?}, tefw: {=bool:?}, teff: {=bool:?}, tefl: {=bool:?}, tsw: {=bool:?}, mraf: {=bool:?}, too: {=bool:?}, drx: {=bool:?}, bec: {=bool:?}, beu: {=bool:?}, elo: {=bool:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, wdi: {=bool:?}, pea: {=bool:?}, ped: {=bool:?}, ara: {=bool:?} }}" , self . rf0n () , self . rf0w () , self . rf0f () , self . rf0l () , self . rf1n () , self . rf1w () , self . rf1f () , self . rf1l () , self . hpm () , self . tc () , self . tcf () , self . tfe () , self . tefn () , self . tefw () , self . teff () , self . tefl () , self . tsw () , self . mraf () , self . too () , self . drx () , self . bec () , self . beu () , self . elo () , self . ep () , self . ew () , self . bo () , self . wdi () , self . pea () , self . ped () , self . ara ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MessageBuff(pub u32);
    impl MessageBuff {
        #[doc = "m_can message buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "m_can message buffer."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MessageBuff {
        #[inline(always)]
        fn default() -> MessageBuff {
            MessageBuff(0)
        }
    }
    impl core::fmt::Debug for MessageBuff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MessageBuff")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MessageBuff {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MessageBuff {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "nominal bit timing and prescaler register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nbtp(pub u32);
    impl Nbtp {
        #[doc = "Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[must_use]
        #[inline(always)]
        pub const fn ntseg2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub const fn set_ntseg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[must_use]
        #[inline(always)]
        pub const fn ntseg1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub const fn set_ntseg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[must_use]
        #[inline(always)]
        pub const fn nbrp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub const fn set_nbrp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
        #[doc = "Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[must_use]
        #[inline(always)]
        pub const fn nsjw(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub const fn set_nsjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for Nbtp {
        #[inline(always)]
        fn default() -> Nbtp {
            Nbtp(0)
        }
    }
    impl core::fmt::Debug for Nbtp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nbtp")
                .field("ntseg2", &self.ntseg2())
                .field("ntseg1", &self.ntseg1())
                .field("nbrp", &self.nbrp())
                .field("nsjw", &self.nsjw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nbtp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nbtp {{ ntseg2: {=u8:?}, ntseg1: {=u8:?}, nbrp: {=u16:?}, nsjw: {=u8:?} }}",
                self.ntseg2(),
                self.ntseg1(),
                self.nbrp(),
                self.nsjw()
            )
        }
    }
    #[doc = "new data1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat1(pub u32);
    impl Ndat1 {
        #[doc = "New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message."]
        #[must_use]
        #[inline(always)]
        pub const fn nd1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New Data\\[31:0\\]
The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message."]
        #[inline(always)]
        pub const fn set_nd1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat1 {
        #[inline(always)]
        fn default() -> Ndat1 {
            Ndat1(0)
        }
    }
    impl core::fmt::Debug for Ndat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ndat1").field("nd1", &self.nd1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ndat1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ndat1 {{ nd1: {=u32:?} }}", self.nd1())
        }
    }
    #[doc = "new data2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat2(pub u32);
    impl Ndat2 {
        #[doc = "New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message."]
        #[must_use]
        #[inline(always)]
        pub const fn nd2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New Data\\[63:32\\]
The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message."]
        #[inline(always)]
        pub const fn set_nd2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ndat2 {
        #[inline(always)]
        fn default() -> Ndat2 {
            Ndat2(0)
        }
    }
    impl core::fmt::Debug for Ndat2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ndat2").field("nd2", &self.nd2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ndat2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ndat2 {{ nd2: {=u32:?} }}", self.nd2())
        }
    }
    #[doc = "protocol status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psr(pub u32);
    impl Psr {
        #[doc = "Last Error Code The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to ‘0’when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_CAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value ‘1’), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value‘0’), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to ‘7’. When the LEC shows the value ‘7’, no CAN bus event was detected since the last CPU read access to the Protocol Status Register. Note: When a frame in CAN FD format has reached the data phase with BRS flag set, the next CAN event (error or valid frame) will be shown in DLEC instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence will be shown as a Form Error, not Stuff Error. Note: The Bus_Off recovery sequence (see ISO 11898-1:2015) cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus_Off, it will set CCCR.INIT of its own accord,stopping all bus activities. Once CCCR.INIT has been cleared by the CPU, the device will then wait for 129 occurrences of Bus Idle (129 * 11 consecutive recessive bits) before resuming normal operation. At the end of the Bus_Off recovery sequence, the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT, each time a sequence of 11 recessive bits has been monitored, a Bit0Error code is written to PSR.LEC, enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus_Off recovery sequence. ECR.REC is used to count these sequences. Note: Byte access: Reading byte 0 will set LEC to “111”, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn lec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Last Error Code The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to ‘0’when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_CAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value ‘1’), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value‘0’), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to ‘7’. When the LEC shows the value ‘7’, no CAN bus event was detected since the last CPU read access to the Protocol Status Register. Note: When a frame in CAN FD format has reached the data phase with BRS flag set, the next CAN event (error or valid frame) will be shown in DLEC instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence will be shown as a Form Error, not Stuff Error. Note: The Bus_Off recovery sequence (see ISO 11898-1:2015) cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus_Off, it will set CCCR.INIT of its own accord,stopping all bus activities. Once CCCR.INIT has been cleared by the CPU, the device will then wait for 129 occurrences of Bus Idle (129 * 11 consecutive recessive bits) before resuming normal operation. At the end of the Bus_Off recovery sequence, the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT, each time a sequence of 11 recessive bits has been monitored, a Bit0Error code is written to PSR.LEC, enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus_Off recovery sequence. ECR.REC is used to count these sequences. Note: Byte access: Reading byte 0 will set LEC to “111”, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_lec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Activity Monitors the module’s CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter Note: ACT is set to “00” by a Protocol Exception Event."]
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Activity Monitors the module’s CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter Note: ACT is set to “00” by a Protocol Exception Event."]
        #[inline(always)]
        pub const fn set_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state."]
        #[must_use]
        #[inline(always)]
        pub const fn ep(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state."]
        #[inline(always)]
        pub const fn set_ep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96."]
        #[must_use]
        #[inline(always)]
        pub const fn ew(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96."]
        #[inline(always)]
        pub const fn set_ew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state."]
        #[must_use]
        #[inline(always)]
        pub const fn bo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state."]
        #[inline(always)]
        pub const fn set_bo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data Phase Last Error Code Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set.Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error. Note: Byte access: Reading byte 0 will set DLEC to “111”, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn dlec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Data Phase Last Error Code Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set.Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error. Note: Byte access: Reading byte 0 will set DLEC to “111”, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_dlec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "ESI flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set Note: Byte access: Reading byte 0 will reset RESI, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn resi(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ESI flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set Note: Byte access: Reading byte 0 will reset RESI, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_resi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BRS flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set Note: Byte access: Reading byte 0 will reset RBRS, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn rbrs(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BRS flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set Note: Byte access: Reading byte 0 will reset RBRS, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_rbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Received a CAN FD Message This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received Note: Byte access: Reading byte 0 will reset RFDF, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn rfdf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Received a CAN FD Message This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received Note: Byte access: Reading byte 0 will reset RFDF, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_rfdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Protocol Exception Event 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred Note: Byte access: Reading byte 0 will reset PXE, reading bytes 3/2/1 has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn pxe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol Exception Event 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred Note: Byte access: Reading byte 0 will reset PXE, reading bytes 3/2/1 has no impact."]
        #[inline(always)]
        pub const fn set_pxe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitter Delay Compensation Value Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Value Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[inline(always)]
        pub const fn set_tdcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Psr {
        #[inline(always)]
        fn default() -> Psr {
            Psr(0)
        }
    }
    impl core::fmt::Debug for Psr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Psr")
                .field("lec", &self.lec())
                .field("act", &self.act())
                .field("ep", &self.ep())
                .field("ew", &self.ew())
                .field("bo", &self.bo())
                .field("dlec", &self.dlec())
                .field("resi", &self.resi())
                .field("rbrs", &self.rbrs())
                .field("rfdf", &self.rfdf())
                .field("pxe", &self.pxe())
                .field("tdcv", &self.tdcv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Psr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Psr {{ lec: {=u8:?}, act: {=u8:?}, ep: {=bool:?}, ew: {=bool:?}, bo: {=bool:?}, dlec: {=u8:?}, resi: {=bool:?}, rbrs: {=bool:?}, rfdf: {=bool:?}, pxe: {=bool:?}, tdcv: {=u8:?} }}" , self . lec () , self . act () , self . ep () , self . ew () , self . bo () , self . dlec () , self . resi () , self . rbrs () , self . rfdf () , self . pxe () , self . tdcv ())
        }
    }
    #[doc = "ram watchdog."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rwd(pub u32);
    impl Rwd {
        #[doc = "Watchdog Configuration Start value of the Message RAM Watchdog Counter. With the reset value of “00” the counter is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn wdc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog Configuration Start value of the Message RAM Watchdog Counter. With the reset value of “00” the counter is disabled."]
        #[inline(always)]
        pub const fn set_wdc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Watchdog Value Actual Message RAM Watchdog Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn wdv(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog Value Actual Message RAM Watchdog Counter Value."]
        #[inline(always)]
        pub const fn set_wdv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Rwd {
        #[inline(always)]
        fn default() -> Rwd {
            Rwd(0)
        }
    }
    impl core::fmt::Debug for Rwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rwd")
                .field("wdc", &self.wdc())
                .field("wdv", &self.wdv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rwd {{ wdc: {=u8:?}, wdv: {=u8:?} }}",
                self.wdc(),
                self.wdv()
            )
        }
    }
    #[doc = "rx buffer configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxbc(pub u32);
    impl Rxbc {
        #[doc = "Rx Buffer Start Address Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address).Also used to reference debug messages A,B,C."]
        #[must_use]
        #[inline(always)]
        pub const fn rbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx Buffer Start Address Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address).Also used to reference debug messages A,B,C."]
        #[inline(always)]
        pub const fn set_rbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
    }
    impl Default for Rxbc {
        #[inline(always)]
        fn default() -> Rxbc {
            Rxbc(0)
        }
    }
    impl core::fmt::Debug for Rxbc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxbc").field("rbsa", &self.rbsa()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxbc {{ rbsa: {=u16:?} }}", self.rbsa())
        }
    }
    #[doc = "rx buffer/fifo element size configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxesc(pub u32);
    impl Rxesc {
        #[doc = "Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn f0ds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored."]
        #[inline(always)]
        pub const fn set_f0ds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field."]
        #[must_use]
        #[inline(always)]
        pub const fn f1ds(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field."]
        #[inline(always)]
        pub const fn set_f1ds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field."]
        #[must_use]
        #[inline(always)]
        pub const fn rbds(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field."]
        #[inline(always)]
        pub const fn set_rbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Rxesc {
        #[inline(always)]
        fn default() -> Rxesc {
            Rxesc(0)
        }
    }
    impl core::fmt::Debug for Rxesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxesc")
                .field("f0ds", &self.f0ds())
                .field("f1ds", &self.f1ds())
                .field("rbds", &self.rbds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxesc {{ f0ds: {=u8:?}, f1ds: {=u8:?}, rbds: {=u8:?} }}",
                self.f0ds(),
                self.f1ds(),
                self.rbds()
            )
        }
    }
    #[doc = "rx fifo0 acknowledge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf0a(pub u32);
    impl Rxf0a {
        #[doc = "Rx FIFO 0 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
        #[must_use]
        #[inline(always)]
        pub const fn f0ai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
        #[inline(always)]
        pub const fn set_f0ai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Rxf0a {
        #[inline(always)]
        fn default() -> Rxf0a {
            Rxf0a(0)
        }
    }
    impl core::fmt::Debug for Rxf0a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf0a").field("f0ai", &self.f0ai()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf0a {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxf0a {{ f0ai: {=u8:?} }}", self.f0ai())
        }
    }
    #[doc = "rx fifo 0 configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf0c(pub u32);
    impl Rxf0c {
        #[doc = "Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)."]
        #[must_use]
        #[inline(always)]
        pub const fn f0sa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address)."]
        #[inline(always)]
        pub const fn set_f0sa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1."]
        #[must_use]
        #[inline(always)]
        pub const fn f0s(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1."]
        #[inline(always)]
        pub const fn set_f0s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn f0wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled."]
        #[inline(always)]
        pub const fn set_f0wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode."]
        #[must_use]
        #[inline(always)]
        pub const fn f0om(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode."]
        #[inline(always)]
        pub const fn set_f0om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rxf0c {
        #[inline(always)]
        fn default() -> Rxf0c {
            Rxf0c(0)
        }
    }
    impl core::fmt::Debug for Rxf0c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf0c")
                .field("f0sa", &self.f0sa())
                .field("f0s", &self.f0s())
                .field("f0wm", &self.f0wm())
                .field("f0om", &self.f0om())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf0c {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxf0c {{ f0sa: {=u16:?}, f0s: {=u8:?}, f0wm: {=u8:?}, f0om: {=bool:?} }}",
                self.f0sa(),
                self.f0s(),
                self.f0wm(),
                self.f0om()
            )
        }
    }
    #[doc = "rx fifo 0 status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf0s(pub u32);
    impl Rxf0s {
        #[doc = "Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
        #[must_use]
        #[inline(always)]
        pub const fn f0fl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64."]
        #[inline(always)]
        pub const fn set_f0fl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63."]
        #[must_use]
        #[inline(always)]
        pub const fn f0gi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63."]
        #[inline(always)]
        pub const fn set_f0gi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
        #[must_use]
        #[inline(always)]
        pub const fn f0pi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63."]
        #[inline(always)]
        pub const fn set_f0pi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full."]
        #[must_use]
        #[inline(always)]
        pub const fn f0f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full."]
        #[inline(always)]
        pub const fn set_f0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = ‘1’ will not set this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rf0l(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = ‘1’ will not set this flag."]
        #[inline(always)]
        pub const fn set_rf0l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Rxf0s {
        #[inline(always)]
        fn default() -> Rxf0s {
            Rxf0s(0)
        }
    }
    impl core::fmt::Debug for Rxf0s {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf0s")
                .field("f0fl", &self.f0fl())
                .field("f0gi", &self.f0gi())
                .field("f0pi", &self.f0pi())
                .field("f0f", &self.f0f())
                .field("rf0l", &self.rf0l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf0s {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxf0s {{ f0fl: {=u8:?}, f0gi: {=u8:?}, f0pi: {=u8:?}, f0f: {=bool:?}, rf0l: {=bool:?} }}" , self . f0fl () , self . f0gi () , self . f0pi () , self . f0f () , self . rf0l ())
        }
    }
    #[doc = "rx fifo 1 acknowledge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf1a(pub u32);
    impl Rxf1a {
        #[doc = "Rx FIFO 1 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
        #[must_use]
        #[inline(always)]
        pub const fn f1ai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
        #[inline(always)]
        pub const fn set_f1ai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Rxf1a {
        #[inline(always)]
        fn default() -> Rxf1a {
            Rxf1a(0)
        }
    }
    impl core::fmt::Debug for Rxf1a {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf1a").field("f1ai", &self.f1ai()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf1a {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxf1a {{ f1ai: {=u8:?} }}", self.f1ai())
        }
    }
    #[doc = "rx fifo1 configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf1c(pub u32);
    impl Rxf1c {
        #[doc = "Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
        #[must_use]
        #[inline(always)]
        pub const fn f1sa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
        #[inline(always)]
        pub const fn set_f1sa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1."]
        #[must_use]
        #[inline(always)]
        pub const fn f1s(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1."]
        #[inline(always)]
        pub const fn set_f1s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn f1wm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled."]
        #[inline(always)]
        pub const fn set_f1wm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
        #[doc = "FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode."]
        #[must_use]
        #[inline(always)]
        pub const fn f1om(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode."]
        #[inline(always)]
        pub const fn set_f1om(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rxf1c {
        #[inline(always)]
        fn default() -> Rxf1c {
            Rxf1c(0)
        }
    }
    impl core::fmt::Debug for Rxf1c {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf1c")
                .field("f1sa", &self.f1sa())
                .field("f1s", &self.f1s())
                .field("f1wm", &self.f1wm())
                .field("f1om", &self.f1om())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf1c {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxf1c {{ f1sa: {=u16:?}, f1s: {=u8:?}, f1wm: {=u8:?}, f1om: {=bool:?} }}",
                self.f1sa(),
                self.f1s(),
                self.f1wm(),
                self.f1om()
            )
        }
    }
    #[doc = "rx fifo1 status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxf1s(pub u32);
    impl Rxf1s {
        #[doc = "Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
        #[must_use]
        #[inline(always)]
        pub const fn f1fl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64."]
        #[inline(always)]
        pub const fn set_f1fl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63."]
        #[must_use]
        #[inline(always)]
        pub const fn f1gi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63."]
        #[inline(always)]
        pub const fn set_f1gi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
        #[must_use]
        #[inline(always)]
        pub const fn f1pi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63."]
        #[inline(always)]
        pub const fn set_f1pi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full."]
        #[must_use]
        #[inline(always)]
        pub const fn f1f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full."]
        #[inline(always)]
        pub const fn set_f1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = ‘1’ will not set this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rf1l(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = ‘1’ will not set this flag."]
        #[inline(always)]
        pub const fn set_rf1l(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set."]
        #[must_use]
        #[inline(always)]
        pub const fn dms(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set."]
        #[inline(always)]
        pub const fn set_dms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Rxf1s {
        #[inline(always)]
        fn default() -> Rxf1s {
            Rxf1s(0)
        }
    }
    impl core::fmt::Debug for Rxf1s {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxf1s")
                .field("f1fl", &self.f1fl())
                .field("f1gi", &self.f1gi())
                .field("f1pi", &self.f1pi())
                .field("f1f", &self.f1f())
                .field("rf1l", &self.rf1l())
                .field("dms", &self.dms())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxf1s {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rxf1s {{ f1fl: {=u8:?}, f1gi: {=u8:?}, f1pi: {=u8:?}, f1f: {=bool:?}, rf1l: {=bool:?}, dms: {=u8:?} }}" , self . f1fl () , self . f1gi () , self . f1pi () , self . f1f () , self . rf1l () , self . dms ())
        }
    }
    #[doc = "standard ID filter configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidfc(pub u32);
    impl Sidfc {
        #[doc = "Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)."]
        #[must_use]
        #[inline(always)]
        pub const fn flssa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address)."]
        #[inline(always)]
        pub const fn set_flssa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128."]
        #[must_use]
        #[inline(always)]
        pub const fn lss(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128."]
        #[inline(always)]
        pub const fn set_lss(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Sidfc {
        #[inline(always)]
        fn default() -> Sidfc {
            Sidfc(0)
        }
    }
    impl core::fmt::Debug for Sidfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sidfc")
                .field("flssa", &self.flssa())
                .field("lss", &self.lss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sidfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sidfc {{ flssa: {=u16:?}, lss: {=u8:?} }}",
                self.flssa(),
                self.lss()
            )
        }
    }
    #[doc = "transmitter delay compensation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdcr(pub u32);
    impl Tdcr {
        #[doc = "Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
        #[must_use]
        #[inline(always)]
        pub const fn tdcf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq."]
        #[inline(always)]
        pub const fn set_tdcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[must_use]
        #[inline(always)]
        pub const fn tdco(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[inline(always)]
        pub const fn set_tdco(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for Tdcr {
        #[inline(always)]
        fn default() -> Tdcr {
            Tdcr(0)
        }
    }
    impl core::fmt::Debug for Tdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdcr")
                .field("tdcf", &self.tdcf())
                .field("tdco", &self.tdco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tdcr {{ tdcf: {=u8:?}, tdco: {=u8:?} }}",
                self.tdcf(),
                self.tdco()
            )
        }
    }
    #[doc = "test register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn lbck(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled."]
        #[inline(always)]
        pub const fn set_lbck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx."]
        #[must_use]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx."]
        #[inline(always)]
        pub const fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Receive Pin Monitors the actual value of pin m_can_rx 0= The CAN bus is dominant (m_can_rx = ‘0’) 1= The CAN bus is recessive (m_can_rx = ‘1’)."]
        #[must_use]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Pin Monitors the actual value of pin m_can_rx 0= The CAN bus is dominant (m_can_rx = ‘0’) 1= The CAN bus is recessive (m_can_rx = ‘1’)."]
        #[inline(always)]
        pub const fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Tx Buffer Number Prepared Tx Buffer number of message that is ready for transmission. Valid when PVAL is set.Valid values are 0 to 31."]
        #[must_use]
        #[inline(always)]
        pub const fn txbnp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx Buffer Number Prepared Tx Buffer number of message that is ready for transmission. Valid when PVAL is set.Valid values are 0 to 31."]
        #[inline(always)]
        pub const fn set_txbnp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Prepared Valid 0= Value of TXBNP not valid 1= Value of TXBNP valid."]
        #[must_use]
        #[inline(always)]
        pub const fn pval(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Prepared Valid 0= Value of TXBNP not valid 1= Value of TXBNP valid."]
        #[inline(always)]
        pub const fn set_pval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Tx Buffer Number Started Tx Buffer number of message whose transmission was started last. Valid when SVAL is set. Valid values are 0 to 31."]
        #[must_use]
        #[inline(always)]
        pub const fn txbns(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx Buffer Number Started Tx Buffer number of message whose transmission was started last. Valid when SVAL is set. Valid values are 0 to 31."]
        #[inline(always)]
        pub const fn set_txbns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Started Valid 0= Value of TXBNS not valid 1= Value of TXBNS valid."]
        #[must_use]
        #[inline(always)]
        pub const fn sval(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Started Valid 0= Value of TXBNS not valid 1= Value of TXBNS valid."]
        #[inline(always)]
        pub const fn set_sval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Test {
        #[inline(always)]
        fn default() -> Test {
            Test(0)
        }
    }
    impl core::fmt::Debug for Test {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Test")
                .field("lbck", &self.lbck())
                .field("tx", &self.tx())
                .field("rx", &self.rx())
                .field("txbnp", &self.txbnp())
                .field("pval", &self.pval())
                .field("txbns", &self.txbns())
                .field("sval", &self.sval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Test {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Test {{ lbck: {=bool:?}, tx: {=u8:?}, rx: {=bool:?}, txbnp: {=u8:?}, pval: {=bool:?}, txbns: {=u8:?}, sval: {=bool:?} }}" , self . lbck () , self . tx () , self . rx () , self . txbnp () , self . pval () , self . txbns () , self . sval ())
        }
    }
    #[doc = "timeout counter configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocc(pub u32);
    impl Tocc {
        #[doc = "Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled."]
        #[inline(always)]
        pub const fn set_rp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tos(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1."]
        #[inline(always)]
        pub const fn set_tos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
        #[must_use]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period."]
        #[inline(always)]
        pub const fn set_top(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tocc {
        #[inline(always)]
        fn default() -> Tocc {
            Tocc(0)
        }
    }
    impl core::fmt::Debug for Tocc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tocc")
                .field("rp", &self.rp())
                .field("tos", &self.tos())
                .field("top", &self.top())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tocc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tocc {{ rp: {=bool:?}, tos: {=u8:?}, top: {=u16:?} }}",
                self.rp(),
                self.tos(),
                self.top()
            )
        }
    }
    #[doc = "timeout counter value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tocv(pub u32);
    impl Tocv {
        #[doc = "Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS. Note: Byte access: when TOCC.TOS = “00，writing one of the register bytes 3/2/1/0 will preset the Timeout Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn toc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS. Note: Byte access: when TOCC.TOS = “00，writing one of the register bytes 3/2/1/0 will preset the Timeout Counter."]
        #[inline(always)]
        pub const fn set_toc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tocv {
        #[inline(always)]
        fn default() -> Tocv {
            Tocv(0)
        }
    }
    impl core::fmt::Debug for Tocv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tocv").field("toc", &self.toc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tocv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tocv {{ toc: {=u16:?} }}", self.toc())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsSel(pub u32);
    impl TsSel {
        #[doc = "Timestamp Word TS default can save 16 timestamps with 32bit; if ts64_en is set, then work at 64bit mode, can save 8 timestamps with 01/23/45…."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timestamp Word TS default can save 16 timestamps with 32bit; if ts64_en is set, then work at 64bit mode, can save 8 timestamps with 01/23/45…."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsSel {
        #[inline(always)]
        fn default() -> TsSel {
            TsSel(0)
        }
    }
    impl core::fmt::Debug for TsSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsSel").field("ts", &self.ts()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsSel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsSel {{ ts: {=u32:?} }}", self.ts())
        }
    }
    #[doc = "timestamp counter configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscc(pub u32);
    impl Tscc {
        #[doc = "timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”."]
        #[must_use]
        #[inline(always)]
        pub const fn tss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”."]
        #[inline(always)]
        pub const fn set_tss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[must_use]
        #[inline(always)]
        pub const fn tcp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times \\[1…16\\]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub const fn set_tcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Tscc {
        #[inline(always)]
        fn default() -> Tscc {
            Tscc(0)
        }
    }
    impl core::fmt::Debug for Tscc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscc")
                .field("tss", &self.tss())
                .field("tcp", &self.tcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tscc {{ tss: {=u8:?}, tcp: {=u8:?} }}",
                self.tss(),
                self.tcp()
            )
        }
    }
    #[doc = "timestamp configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscfg(pub u32);
    impl Tscfg {
        #[doc = "Timestamp Unit Enable 0: TSU disabled 1: TSU enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tsue(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp Unit Enable 0: TSU disabled 1: TSU enabled."]
        #[inline(always)]
        pub const fn set_tsue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn tbcs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB\\[31:0\\]
is the internal timbase counter 1: Timestamp value captured from input tsu_tbin\\[31:0\\],ATB.TB\\[31:0\\]
is tsu_tbin\\[31:0\\]."]
        #[inline(always)]
        pub const fn set_tbcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF."]
        #[must_use]
        #[inline(always)]
        pub const fn scp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF."]
        #[inline(always)]
        pub const fn set_scp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one."]
        #[must_use]
        #[inline(always)]
        pub const fn en64(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one."]
        #[inline(always)]
        pub const fn set_en64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
        #[must_use]
        #[inline(always)]
        pub const fn tbpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE\\[7:0\\]
is fixed to 0x00, the Timestamp Prescaler is not used."]
        #[inline(always)]
        pub const fn set_tbpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Tscfg {
        #[inline(always)]
        fn default() -> Tscfg {
            Tscfg(0)
        }
    }
    impl core::fmt::Debug for Tscfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscfg")
                .field("tsue", &self.tsue())
                .field("tbcs", &self.tbcs())
                .field("scp", &self.scp())
                .field("en64", &self.en64())
                .field("tbpre", &self.tbpre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tscfg {{ tsue: {=bool:?}, tbcs: {=bool:?}, scp: {=bool:?}, en64: {=bool:?}, tbpre: {=u8:?} }}" , self . tsue () , self . tbcs () , self . scp () , self . en64 () , self . tbpre ())
        }
    }
    #[doc = "timestamp counter value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tscv(pub u32);
    impl Tscv {
        #[doc = "Timestamp Counter The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx).When TSCC.TSS = “01”, the Timestamp Counter is incremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = “10”, TSC reflects the external Timestamp Counter value. A write access has no impact."]
        #[must_use]
        #[inline(always)]
        pub const fn tsc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp Counter The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx).When TSCC.TSS = “01”, the Timestamp Counter is incremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = “10”, TSC reflects the external Timestamp Counter value. A write access has no impact."]
        #[inline(always)]
        pub const fn set_tsc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tscv {
        #[inline(always)]
        fn default() -> Tscv {
            Tscv(0)
        }
    }
    impl core::fmt::Debug for Tscv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tscv").field("tsc", &self.tsc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tscv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tscv {{ tsc: {=u16:?} }}", self.tsc())
        }
    }
    #[doc = "timestamp status1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tss1(pub u32);
    impl Tss1 {
        #[doc = "Timestamp New Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when a timestamp was stored in the related Timestamp register. Reading a Timestamp register resets the related bit."]
        #[must_use]
        #[inline(always)]
        pub const fn tsn(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp New Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when a timestamp was stored in the related Timestamp register. Reading a Timestamp register resets the related bit."]
        #[inline(always)]
        pub const fn set_tsn(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Timestamp Lost Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when the timestamp stored in the related Timestamp register was overwritten before it was read. Reading a Timestamp register resets the related bit."]
        #[must_use]
        #[inline(always)]
        pub const fn tsl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Timestamp Lost Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when the timestamp stored in the related Timestamp register was overwritten before it was read. Reading a Timestamp register resets the related bit."]
        #[inline(always)]
        pub const fn set_tsl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tss1 {
        #[inline(always)]
        fn default() -> Tss1 {
            Tss1(0)
        }
    }
    impl core::fmt::Debug for Tss1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tss1")
                .field("tsn", &self.tsn())
                .field("tsl", &self.tsl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tss1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tss1 {{ tsn: {=u16:?}, tsl: {=u16:?} }}",
                self.tsn(),
                self.tsl()
            )
        }
    }
    #[doc = "timestamp status2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tss2(pub u32);
    impl Tss2 {
        #[doc = "Timestamp Pointer The Timestamp Pointer is incremented by one each time a timestamp is captured. From its maximum value (3, 7, or 15 depending on number_ts_g), it is incremented to 0. Value also signalled on output m_can_tsp\\[3:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn tsp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Timestamp Pointer The Timestamp Pointer is incremented by one each time a timestamp is captured. From its maximum value (3, 7, or 15 depending on number_ts_g), it is incremented to 0. Value also signalled on output m_can_tsp\\[3:0\\]."]
        #[inline(always)]
        pub const fn set_tsp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Tss2 {
        #[inline(always)]
        fn default() -> Tss2 {
            Tss2(0)
        }
    }
    impl core::fmt::Debug for Tss2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tss2").field("tsp", &self.tsp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tss2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tss2 {{ tsp: {=u8:?} }}", self.tsp())
        }
    }
    #[doc = "tx buffer add request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbar(pub u32);
    impl Txbar {
        #[doc = "Add Request Each Tx Buffer has its own Add Request bit. Writing a ‘1’ will set the corresponding Add Request bit; writing a ‘0’ has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0= No transmission request added 1= Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn ar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Add Request Each Tx Buffer has its own Add Request bit. Writing a ‘1’ will set the corresponding Add Request bit; writing a ‘0’ has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0= No transmission request added 1= Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored."]
        #[inline(always)]
        pub const fn set_ar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbar {
        #[inline(always)]
        fn default() -> Txbar {
            Txbar(0)
        }
    }
    impl core::fmt::Debug for Txbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbar").field("ar", &self.ar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbar {{ ar: {=u32:?} }}", self.ar())
        }
    }
    #[doc = "tx buffer configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbc(pub u32);
    impl Txbc {
        #[doc = "Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
        #[must_use]
        #[inline(always)]
        pub const fn tbsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers."]
        #[inline(always)]
        pub const fn set_tbsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32."]
        #[must_use]
        #[inline(always)]
        pub const fn ndtb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32."]
        #[inline(always)]
        pub const fn set_ndtb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32."]
        #[must_use]
        #[inline(always)]
        pub const fn tfqs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32."]
        #[inline(always)]
        pub const fn set_tfqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation."]
        #[must_use]
        #[inline(always)]
        pub const fn tfqm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation."]
        #[inline(always)]
        pub const fn set_tfqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Txbc {
        #[inline(always)]
        fn default() -> Txbc {
            Txbc(0)
        }
    }
    impl core::fmt::Debug for Txbc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbc")
                .field("tbsa", &self.tbsa())
                .field("ndtb", &self.ndtb())
                .field("tfqs", &self.tfqs())
                .field("tfqm", &self.tfqm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txbc {{ tbsa: {=u16:?}, ndtb: {=u8:?}, tfqs: {=u8:?}, tfqm: {=bool:?} }}",
                self.tbsa(),
                self.ndtb(),
                self.tfqs(),
                self.tfqm()
            )
        }
    }
    #[doc = "tx buffer cancellation finished."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcf(pub u32);
    impl Txbcf {
        #[doc = "Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished."]
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished."]
        #[inline(always)]
        pub const fn set_cf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbcf {
        #[inline(always)]
        fn default() -> Txbcf {
            Txbcf(0)
        }
    }
    impl core::fmt::Debug for Txbcf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcf").field("cf", &self.cf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbcf {{ cf: {=u32:?} }}", self.cf())
        }
    }
    #[doc = "tx buffer cancellation finished interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcie(pub u32);
    impl Txbcie {
        #[doc = "Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cfie(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled."]
        #[inline(always)]
        pub const fn set_cfie(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbcie {
        #[inline(always)]
        fn default() -> Txbcie {
            Txbcie(0)
        }
    }
    impl core::fmt::Debug for Txbcie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcie")
                .field("cfie", &self.cfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbcie {{ cfie: {=u32:?} }}", self.cfie())
        }
    }
    #[doc = "tx buffer cancellation request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbcr(pub u32);
    impl Txbcr {
        #[doc = "Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbcr {
        #[inline(always)]
        fn default() -> Txbcr {
            Txbcr(0)
        }
    }
    impl core::fmt::Debug for Txbcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbcr").field("cr", &self.cr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbcr {{ cr: {=u32:?} }}", self.cr())
        }
    }
    #[doc = "tx buffer request pending."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbrp(pub u32);
    impl Txbrp {
        #[doc = "Transmission Request Pending Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR.The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan (see Section 3.5, Tx Handling) is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signalled via TXBCF ? after successful transmission together with the corresponding TXBTO bit ? when the transmission has not yet been started at the point of cancellation ? when the transmission has been aborted due to lost arbitration ? when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0= No transmission request pending 1= Transmission request pending Note: TXBRP bits which are set while a Tx scan is in progress are not considered during this particular Tx scan. In case a cancellation is requested for such a Tx Buffer, this Add Request is cancelled immediately, the corresponding TXBRP bit is reset."]
        #[must_use]
        #[inline(always)]
        pub const fn trp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmission Request Pending Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR.The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan (see Section 3.5, Tx Handling) is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signalled via TXBCF ? after successful transmission together with the corresponding TXBTO bit ? when the transmission has not yet been started at the point of cancellation ? when the transmission has been aborted due to lost arbitration ? when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0= No transmission request pending 1= Transmission request pending Note: TXBRP bits which are set while a Tx scan is in progress are not considered during this particular Tx scan. In case a cancellation is requested for such a Tx Buffer, this Add Request is cancelled immediately, the corresponding TXBRP bit is reset."]
        #[inline(always)]
        pub const fn set_trp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbrp {
        #[inline(always)]
        fn default() -> Txbrp {
            Txbrp(0)
        }
    }
    impl core::fmt::Debug for Txbrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbrp").field("trp", &self.trp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbrp {{ trp: {=u32:?} }}", self.trp())
        }
    }
    #[doc = "tx buffer transmission interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbtie(pub u32);
    impl Txbtie {
        #[doc = "Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbtie {
        #[inline(always)]
        fn default() -> Txbtie {
            Txbtie(0)
        }
    }
    impl core::fmt::Debug for Txbtie {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbtie").field("tie", &self.tie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbtie {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbtie {{ tie: {=u32:?} }}", self.tie())
        }
    }
    #[doc = "tx buffer transmission occurred."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txbto(pub u32);
    impl Txbto {
        #[doc = "Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn to(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred."]
        #[inline(always)]
        pub const fn set_to(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txbto {
        #[inline(always)]
        fn default() -> Txbto {
            Txbto(0)
        }
    }
    impl core::fmt::Debug for Txbto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txbto").field("to", &self.to()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txbto {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txbto {{ to: {=u32:?} }}", self.to())
        }
    }
    #[doc = "tx event fifo acknowledge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefa(pub u32);
    impl Txefa {
        #[doc = "Event FIFO Acknowledge Index After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
        #[must_use]
        #[inline(always)]
        pub const fn efai(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Acknowledge Index After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
        #[inline(always)]
        pub const fn set_efai(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Txefa {
        #[inline(always)]
        fn default() -> Txefa {
            Txefa(0)
        }
    }
    impl core::fmt::Debug for Txefa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefa").field("efai", &self.efai()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txefa {{ efai: {=u8:?} }}", self.efai())
        }
    }
    #[doc = "tx event fifo configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefc(pub u32);
    impl Txefc {
        #[doc = "Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
        #[must_use]
        #[inline(always)]
        pub const fn efsa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
        #[inline(always)]
        pub const fn set_efsa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1."]
        #[must_use]
        #[inline(always)]
        pub const fn efs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1."]
        #[inline(always)]
        pub const fn set_efs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn efwm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled."]
        #[inline(always)]
        pub const fn set_efwm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Txefc {
        #[inline(always)]
        fn default() -> Txefc {
            Txefc(0)
        }
    }
    impl core::fmt::Debug for Txefc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefc")
                .field("efsa", &self.efsa())
                .field("efs", &self.efs())
                .field("efwm", &self.efwm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txefc {{ efsa: {=u16:?}, efs: {=u8:?}, efwm: {=u8:?} }}",
                self.efsa(),
                self.efs(),
                self.efwm()
            )
        }
    }
    #[doc = "tx event fifo status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txefs(pub u32);
    impl Txefs {
        #[doc = "Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
        #[must_use]
        #[inline(always)]
        pub const fn effl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32."]
        #[inline(always)]
        pub const fn set_effl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
        #[must_use]
        #[inline(always)]
        pub const fn efgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31."]
        #[inline(always)]
        pub const fn set_efgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
        #[must_use]
        #[inline(always)]
        pub const fn efpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31."]
        #[inline(always)]
        pub const fn set_efpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn eff(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full."]
        #[inline(always)]
        pub const fn set_eff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
        #[must_use]
        #[inline(always)]
        pub const fn tefl(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
        #[inline(always)]
        pub const fn set_tefl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Txefs {
        #[inline(always)]
        fn default() -> Txefs {
            Txefs(0)
        }
    }
    impl core::fmt::Debug for Txefs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txefs")
                .field("effl", &self.effl())
                .field("efgi", &self.efgi())
                .field("efpi", &self.efpi())
                .field("eff", &self.eff())
                .field("tefl", &self.tefl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txefs {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Txefs {{ effl: {=u8:?}, efgi: {=u8:?}, efpi: {=u8:?}, eff: {=bool:?}, tefl: {=bool:?} }}" , self . effl () , self . efgi () , self . efpi () , self . eff () , self . tefl ())
        }
    }
    #[doc = "tx buffer element size configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txesc(pub u32);
    impl Txesc {
        #[doc = "Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as “0xCC” (padding bytes)."]
        #[must_use]
        #[inline(always)]
        pub const fn tbds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as “0xCC” (padding bytes)."]
        #[inline(always)]
        pub const fn set_tbds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Txesc {
        #[inline(always)]
        fn default() -> Txesc {
            Txesc(0)
        }
    }
    impl core::fmt::Debug for Txesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txesc").field("tbds", &self.tbds()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txesc {{ tbds: {=u8:?} }}", self.tbds())
        }
    }
    #[doc = "tx fifo/queue status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfqs(pub u32);
    impl Txfqs {
        #[doc = "Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’) Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn tffl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’) Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
        #[inline(always)]
        pub const fn set_tffl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’)."]
        #[must_use]
        #[inline(always)]
        pub const fn tfgi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’)."]
        #[inline(always)]
        pub const fn set_tfgi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
        #[must_use]
        #[inline(always)]
        pub const fn tfqpi(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31."]
        #[inline(always)]
        pub const fn set_tfqpi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full."]
        #[must_use]
        #[inline(always)]
        pub const fn tfqf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full."]
        #[inline(always)]
        pub const fn set_tfqf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Txfqs {
        #[inline(always)]
        fn default() -> Txfqs {
            Txfqs(0)
        }
    }
    impl core::fmt::Debug for Txfqs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txfqs")
                .field("tffl", &self.tffl())
                .field("tfgi", &self.tfgi())
                .field("tfqpi", &self.tfqpi())
                .field("tfqf", &self.tfqf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txfqs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txfqs {{ tffl: {=u8:?}, tfgi: {=u8:?}, tfqpi: {=u8:?}, tfqf: {=bool:?} }}",
                self.tffl(),
                self.tfgi(),
                self.tfqpi(),
                self.tfqf()
            )
        }
    }
    #[doc = "extended id and mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidam(pub u32);
    impl Xidam {
        #[doc = "Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
        #[must_use]
        #[inline(always)]
        pub const fn eidm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
        #[inline(always)]
        pub const fn set_eidm(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for Xidam {
        #[inline(always)]
        fn default() -> Xidam {
            Xidam(0)
        }
    }
    impl core::fmt::Debug for Xidam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xidam").field("eidm", &self.eidm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xidam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Xidam {{ eidm: {=u32:?} }}", self.eidm())
        }
    }
    #[doc = "extended ID filter configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xidfc(pub u32);
    impl Xidfc {
        #[doc = "Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
        #[must_use]
        #[inline(always)]
        pub const fn flesa(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
        #[inline(always)]
        pub const fn set_flesa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64."]
        #[must_use]
        #[inline(always)]
        pub const fn lse(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64."]
        #[inline(always)]
        pub const fn set_lse(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Xidfc {
        #[inline(always)]
        fn default() -> Xidfc {
            Xidfc(0)
        }
    }
    impl core::fmt::Debug for Xidfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xidfc")
                .field("flesa", &self.flesa())
                .field("lse", &self.lse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xidfc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xidfc {{ flesa: {=u16:?}, lse: {=u8:?} }}",
                self.flesa(),
                self.lse()
            )
        }
    }
}
