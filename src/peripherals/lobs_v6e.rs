#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LOBS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lobs {
    ptr: *mut u8,
}
unsafe impl Send for Lobs {}
unsafe impl Sync for Lobs {}
impl Lobs {
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
    #[doc = "Stream Control Register."]
    #[inline(always)]
    pub const fn streamctrl(self) -> crate::common::Reg<regs::Streamctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pre-trigger Action Register."]
    #[inline(always)]
    pub const fn ptaction(self) -> crate::common::Reg<regs::Ptaction, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Start Address Register."]
    #[inline(always)]
    pub const fn startaddr(self) -> crate::common::Reg<regs::Startaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "End Address Register."]
    #[inline(always)]
    pub const fn endaddr(self) -> crate::common::Reg<regs::Endaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Current Trigger State Register."]
    #[inline(always)]
    pub const fn ctsr(self) -> crate::common::Reg<regs::Ctsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Current Counter Value Register."]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Current Action Value Register."]
    #[inline(always)]
    pub const fn cavr(self) -> crate::common::Reg<regs::Cavr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Fifo State Register."]
    #[inline(always)]
    pub const fn fifostate(self) -> crate::common::Reg<regs::Fifostate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Final Address Register."]
    #[inline(always)]
    pub const fn finaladdr(self) -> crate::common::Reg<regs::Finaladdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Group Select Register."]
    #[inline(always)]
    pub const fn grpsela(self) -> crate::common::Reg<regs::Grpsela, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Group Enable Register."]
    #[inline(always)]
    pub const fn grpena(self) -> crate::common::Reg<regs::Grpena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Signal Select1 Register."]
    #[inline(always)]
    pub const fn sigsela1(self) -> crate::common::Reg<regs::Sigsela1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Signal Select2 Register."]
    #[inline(always)]
    pub const fn sigsela2(self) -> crate::common::Reg<regs::Sigsela2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Signal Enable Register."]
    #[inline(always)]
    pub const fn sigena(self) -> crate::common::Reg<regs::Sigena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn state(self, n: usize) -> State {
        assert!(n < 5usize);
        unsafe { State::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "Lock Access Register."]
    #[inline(always)]
    pub const fn lar(self) -> crate::common::Reg<regs::Lar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb0usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    ptr: *mut u8,
}
unsafe impl Send for State {}
unsafe impl Sync for State {}
impl State {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Signal Select Register."]
    #[inline(always)]
    pub const fn sigsel(self) -> crate::common::Reg<regs::Sigsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Trigger Control Register."]
    #[inline(always)]
    pub const fn trigctrl(self) -> crate::common::Reg<regs::Trigctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Next State Register."]
    #[inline(always)]
    pub const fn nextstate(self) -> crate::common::Reg<regs::Nextstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Action Register."]
    #[inline(always)]
    pub const fn action(self) -> crate::common::Reg<regs::Action, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Counter Compare Register."]
    #[inline(always)]
    pub const fn countcomp(self) -> crate::common::Reg<regs::Countcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "External Mask Register."]
    #[inline(always)]
    pub const fn extmask(self) -> crate::common::Reg<regs::Extmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "External Compare Register."]
    #[inline(always)]
    pub const fn extcomp(self) -> crate::common::Reg<regs::Extcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Signal Mask Register."]
    #[inline(always)]
    pub const fn sigmask(self) -> crate::common::Reg<regs::Sigmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Compare Enable register."]
    #[inline(always)]
    pub const fn compen(self) -> crate::common::Reg<regs::Compen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Signal Compare Register0."]
    #[inline(always)]
    pub const fn sigcomp0(self) -> crate::common::Reg<regs::Sigcomp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Signal Compare Register1."]
    #[inline(always)]
    pub const fn sigcomp1(self) -> crate::common::Reg<regs::Sigcomp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Signal Compare Register2."]
    #[inline(always)]
    pub const fn sigcomp2(self) -> crate::common::Reg<regs::Sigcomp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Signal Compare Register3."]
    #[inline(always)]
    pub const fn sigcomp3(self) -> crate::common::Reg<regs::Sigcomp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
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
    #[doc = "Action Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Action(pub u32);
    impl Action {
        #[doc = "Trace active. 0b0 Trace disable. 0b1 Trace enable."]
        #[must_use]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Trace active. 0b0 Trace disable. 0b1 Trace enable."]
        #[inline(always)]
        pub const fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Action {
        #[inline(always)]
        fn default() -> Action {
            Action(0)
        }
    }
    impl core::fmt::Debug for Action {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Action")
                .field("trace", &self.trace())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Action {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Action {{ trace: {=bool:?} }}", self.trace())
        }
    }
    #[doc = "Current Action Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cavr(pub u32);
    impl Cavr {
        #[doc = "Trace active. 0b0 Trace is not active. 0b1 Trace is active."]
        #[must_use]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Trace active. 0b0 Trace is not active. 0b1 Trace is active."]
        #[inline(always)]
        pub const fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cavr {
        #[inline(always)]
        fn default() -> Cavr {
            Cavr(0)
        }
    }
    impl core::fmt::Debug for Cavr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cavr")
                .field("trace", &self.trace())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cavr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cavr {{ trace: {=bool:?} }}", self.trace())
        }
    }
    #[doc = "Current Counter Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "Returns the counter value when the CTSR was last read. If the CTSR has never been read, then the value in the CCVR is undefined."]
        #[must_use]
        #[inline(always)]
        pub const fn ccvr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Returns the counter value when the CTSR was last read. If the CTSR has never been read, then the value in the CCVR is undefined."]
        #[inline(always)]
        pub const fn set_ccvr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ccvr {
        #[inline(always)]
        fn default() -> Ccvr {
            Ccvr(0)
        }
    }
    impl core::fmt::Debug for Ccvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvr").field("ccvr", &self.ccvr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccvr {{ ccvr: {=u32:?} }}", self.ccvr())
        }
    }
    #[doc = "Compare Enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Compen(pub u32);
    impl Compen {
        #[doc = "Select compare signal number0-3."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Select compare signal number0-3."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Compen {
        #[inline(always)]
        fn default() -> Compen {
            Compen(0)
        }
    }
    impl core::fmt::Debug for Compen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Compen").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Compen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Compen {{ en: {=u8:?} }}", self.en())
        }
    }
    #[doc = "Counter Compare Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countcomp(pub u32);
    impl Countcomp {
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Countcomp {
        #[inline(always)]
        fn default() -> Countcomp {
            Countcomp(0)
        }
    }
    impl core::fmt::Debug for Countcomp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Countcomp")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Countcomp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Countcomp {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Run control. 0 LOBS disabled. Register programming permitted. 1 LOBS enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Run control. 0 LOBS disabled. Register programming permitted. 1 LOBS enabled."]
        #[inline(always)]
        pub const fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Ctrl").field("run", &self.run()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctrl {{ run: {=bool:?} }}", self.run())
        }
    }
    #[doc = "Current Trigger State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctsr(pub u32);
    impl Ctsr {
        #[doc = "Reads current Trigger State. This is a one-hot encoded field. When CTRL.RUN: 0 RAZ. 1 Returns current Trigger State. If FINALSTATE is 1, then the CTSR field gives the Trigger State when FINALSTATE became 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ctsr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Reads current Trigger State. This is a one-hot encoded field. When CTRL.RUN: 0 RAZ. 1 Returns current Trigger State. If FINALSTATE is 1, then the CTSR field gives the Trigger State when FINALSTATE became 1."]
        #[inline(always)]
        pub const fn set_ctsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "0 LOBS is still tracing. 1 Indicates that the LOBS has stopped advancing Trigger States and stopped trace. FINALSTATE can be set by TRIGCTRL.COUNTBRK reaching the final loop count, or by programming NEXTSTATEto zero."]
        #[must_use]
        #[inline(always)]
        pub const fn finalstate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "0 LOBS is still tracing. 1 Indicates that the LOBS has stopped advancing Trigger States and stopped trace. FINALSTATE can be set by TRIGCTRL.COUNTBRK reaching the final loop count, or by programming NEXTSTATEto zero."]
        #[inline(always)]
        pub const fn set_finalstate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctsr {
        #[inline(always)]
        fn default() -> Ctsr {
            Ctsr(0)
        }
    }
    impl core::fmt::Debug for Ctsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctsr")
                .field("ctsr", &self.ctsr())
                .field("finalstate", &self.finalstate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctsr {{ ctsr: {=u8:?}, finalstate: {=bool:?} }}",
                self.ctsr(),
                self.finalstate()
            )
        }
    }
    #[doc = "End Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endaddr(pub u32);
    impl Endaddr {
        #[doc = "End address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "End address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Endaddr {
        #[inline(always)]
        fn default() -> Endaddr {
            Endaddr(0)
        }
    }
    impl core::fmt::Debug for Endaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Endaddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Endaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Endaddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "External Compare Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extcomp(pub u32);
    impl Extcomp {
        #[doc = "External Compare."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "External Compare."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Extcomp {
        #[inline(always)]
        fn default() -> Extcomp {
            Extcomp(0)
        }
    }
    impl core::fmt::Debug for Extcomp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extcomp")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extcomp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Extcomp {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "External Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extmask(pub u32);
    impl Extmask {
        #[doc = "External Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "External Mask."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Extmask {
        #[inline(always)]
        fn default() -> Extmask {
            Extmask(0)
        }
    }
    impl core::fmt::Debug for Extmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extmask")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extmask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Extmask {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "Fifo State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifostate(pub u32);
    impl Fifostate {
        #[doc = "FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Fifostate {
        #[inline(always)]
        fn default() -> Fifostate {
            Fifostate(0)
        }
    }
    impl core::fmt::Debug for Fifostate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifostate")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifostate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fifostate {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "Final Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Finaladdr(pub u32);
    impl Finaladdr {
        #[doc = "Final address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Final address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Finaladdr {
        #[inline(always)]
        fn default() -> Finaladdr {
            Finaladdr(0)
        }
    }
    impl core::fmt::Debug for Finaladdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Finaladdr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Finaladdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Finaladdr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Group Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grpena(pub u32);
    impl Grpena {
        #[doc = "Enable sample group number1."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable sample group number1."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable sample group number2."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable sample group number2."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Grpena {
        #[inline(always)]
        fn default() -> Grpena {
            Grpena(0)
        }
    }
    impl core::fmt::Debug for Grpena {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grpena")
                .field("en1", &self.en1())
                .field("en2", &self.en2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grpena {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grpena {{ en1: {=bool:?}, en2: {=bool:?} }}",
                self.en1(),
                self.en2()
            )
        }
    }
    #[doc = "Group Select Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grpsela(pub u32);
    impl Grpsela {
        #[doc = "Select sample group number1."]
        #[must_use]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Select sample group number1."]
        #[inline(always)]
        pub const fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Select sample group number2."]
        #[must_use]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Select sample group number2."]
        #[inline(always)]
        pub const fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Grpsela {
        #[inline(always)]
        fn default() -> Grpsela {
            Grpsela(0)
        }
    }
    impl core::fmt::Debug for Grpsela {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grpsela")
                .field("num1", &self.num1())
                .field("num2", &self.num2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grpsela {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grpsela {{ num1: {=u8:?}, num2: {=u8:?} }}",
                self.num1(),
                self.num2()
            )
        }
    }
    #[doc = "Lock Access Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lar(pub u32);
    impl Lar {
        #[doc = "Lock Access Value."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Lock Access Value."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lar {
        #[inline(always)]
        fn default() -> Lar {
            Lar(0)
        }
    }
    impl core::fmt::Debug for Lar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lar").field("value", &self.value()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lar {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "Next State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nextstate(pub u32);
    impl Nextstate {
        #[doc = "Selects the next state to move to after the Trigger Condition has been met in the current state. 0x0 Do not change state. This is the final Trigger State. 0x1 Selects Trigger State 0. 0x2 Selects Trigger State 1. 0x4 Selects Trigger State 2. 0x8 Selects Trigger State 3. 0x10 Selects Trigger State 4."]
        #[must_use]
        #[inline(always)]
        pub const fn nextstate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Selects the next state to move to after the Trigger Condition has been met in the current state. 0x0 Do not change state. This is the final Trigger State. 0x1 Selects Trigger State 0. 0x2 Selects Trigger State 1. 0x4 Selects Trigger State 2. 0x8 Selects Trigger State 3. 0x10 Selects Trigger State 4."]
        #[inline(always)]
        pub const fn set_nextstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Nextstate {
        #[inline(always)]
        fn default() -> Nextstate {
            Nextstate(0)
        }
    }
    impl core::fmt::Debug for Nextstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nextstate")
                .field("nextstate", &self.nextstate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nextstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nextstate {{ nextstate: {=u8:?} }}", self.nextstate())
        }
    }
    #[doc = "Pre-trigger Action Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptaction(pub u32);
    impl Ptaction {
        #[doc = "Enables trace."]
        #[must_use]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enables trace."]
        #[inline(always)]
        pub const fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ptaction {
        #[inline(always)]
        fn default() -> Ptaction {
            Ptaction(0)
        }
    }
    impl core::fmt::Debug for Ptaction {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptaction")
                .field("trace", &self.trace())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptaction {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptaction {{ trace: {=bool:?} }}", self.trace())
        }
    }
    #[doc = "Signal Compare Register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp0(pub u32);
    impl Sigcomp0 {
        #[doc = "Compare golden value for Signal Group signals\\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn value0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[31:0\\]."]
        #[inline(always)]
        pub const fn set_value0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp0 {
        #[inline(always)]
        fn default() -> Sigcomp0 {
            Sigcomp0(0)
        }
    }
    impl core::fmt::Debug for Sigcomp0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigcomp0")
                .field("value0", &self.value0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigcomp0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigcomp0 {{ value0: {=u32:?} }}", self.value0())
        }
    }
    #[doc = "Signal Compare Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp1(pub u32);
    impl Sigcomp1 {
        #[doc = "Compare golden value for Signal Group signals\\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn value1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[63:32\\]."]
        #[inline(always)]
        pub const fn set_value1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp1 {
        #[inline(always)]
        fn default() -> Sigcomp1 {
            Sigcomp1(0)
        }
    }
    impl core::fmt::Debug for Sigcomp1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigcomp1")
                .field("value1", &self.value1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigcomp1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigcomp1 {{ value1: {=u32:?} }}", self.value1())
        }
    }
    #[doc = "Signal Compare Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp2(pub u32);
    impl Sigcomp2 {
        #[doc = "Compare golden value for Signal Group signals\\[95:64\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn value2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[95:64\\]."]
        #[inline(always)]
        pub const fn set_value2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp2 {
        #[inline(always)]
        fn default() -> Sigcomp2 {
            Sigcomp2(0)
        }
    }
    impl core::fmt::Debug for Sigcomp2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigcomp2")
                .field("value2", &self.value2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigcomp2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigcomp2 {{ value2: {=u32:?} }}", self.value2())
        }
    }
    #[doc = "Signal Compare Register3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp3(pub u32);
    impl Sigcomp3 {
        #[doc = "Compare golden value for Signal Group signals\\[127:96\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn value3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[127:96\\]."]
        #[inline(always)]
        pub const fn set_value3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp3 {
        #[inline(always)]
        fn default() -> Sigcomp3 {
            Sigcomp3(0)
        }
    }
    impl core::fmt::Debug for Sigcomp3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigcomp3")
                .field("value3", &self.value3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigcomp3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigcomp3 {{ value3: {=u32:?} }}", self.value3())
        }
    }
    #[doc = "Signal Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigena(pub u32);
    impl Sigena {
        #[doc = "Enable sample signal number1."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable sample signal number1."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Enable sample signal number2."]
        #[must_use]
        #[inline(always)]
        pub const fn en2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable sample signal number2."]
        #[inline(always)]
        pub const fn set_en2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Sigena {
        #[inline(always)]
        fn default() -> Sigena {
            Sigena(0)
        }
    }
    impl core::fmt::Debug for Sigena {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigena")
                .field("en1", &self.en1())
                .field("en2", &self.en2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigena {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sigena {{ en1: {=u8:?}, en2: {=u8:?} }}",
                self.en1(),
                self.en2()
            )
        }
    }
    #[doc = "Signal Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigmask(pub u32);
    impl Sigmask {
        #[doc = "Select compare signal number0."]
        #[must_use]
        #[inline(always)]
        pub const fn num0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number0."]
        #[inline(always)]
        pub const fn set_num0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select compare signal number1."]
        #[must_use]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number1."]
        #[inline(always)]
        pub const fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select compare signal number2."]
        #[must_use]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number2."]
        #[inline(always)]
        pub const fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select compare signal number3."]
        #[must_use]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number3."]
        #[inline(always)]
        pub const fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigmask {
        #[inline(always)]
        fn default() -> Sigmask {
            Sigmask(0)
        }
    }
    impl core::fmt::Debug for Sigmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigmask")
                .field("num0", &self.num0())
                .field("num1", &self.num1())
                .field("num2", &self.num2())
                .field("num3", &self.num3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigmask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sigmask {{ num0: {=u8:?}, num1: {=u8:?}, num2: {=u8:?}, num3: {=u8:?} }}",
                self.num0(),
                self.num1(),
                self.num2(),
                self.num3()
            )
        }
    }
    #[doc = "Signal Select Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsel(pub u32);
    impl Sigsel {
        #[doc = "Selects Signal Group. 0x1 Selects Signal Group 0. 0x2 Selects Signal Group 1. 0x4 Selects Signal Group 2. 0x8 Selects Signal Group 3. 0x10 Selects Signal Group 4. 0x20 Selects Signal Group 5. 0x40 Selects Signal Group 6. 0x80 Selects Signal Group 7. 0x100 Selects Signal Group 8. 0x200 Selects Signal Group 9. 0x400 Selects Signal Group 10. 0x800 Selects Signal Group 11."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Selects Signal Group. 0x1 Selects Signal Group 0. 0x2 Selects Signal Group 1. 0x4 Selects Signal Group 2. 0x8 Selects Signal Group 3. 0x10 Selects Signal Group 4. 0x20 Selects Signal Group 5. 0x40 Selects Signal Group 6. 0x80 Selects Signal Group 7. 0x100 Selects Signal Group 8. 0x200 Selects Signal Group 9. 0x400 Selects Signal Group 10. 0x800 Selects Signal Group 11."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Sigsel {
        #[inline(always)]
        fn default() -> Sigsel {
            Sigsel(0)
        }
    }
    impl core::fmt::Debug for Sigsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigsel").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigsel {{ en: {=u16:?} }}", self.en())
        }
    }
    #[doc = "Signal Select1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela1(pub u32);
    impl Sigsela1 {
        #[doc = "Select sample signal bit number1 in first group."]
        #[must_use]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number1 in first group."]
        #[inline(always)]
        pub const fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select sample signal bit number2 in first group."]
        #[must_use]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number2 in first group."]
        #[inline(always)]
        pub const fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select sample signal bit number3 in first group."]
        #[must_use]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number3 in first group."]
        #[inline(always)]
        pub const fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select sample signal bit number4 in first group."]
        #[must_use]
        #[inline(always)]
        pub const fn num4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number4 in first group."]
        #[inline(always)]
        pub const fn set_num4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela1 {
        #[inline(always)]
        fn default() -> Sigsela1 {
            Sigsela1(0)
        }
    }
    impl core::fmt::Debug for Sigsela1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigsela1")
                .field("num1", &self.num1())
                .field("num2", &self.num2())
                .field("num3", &self.num3())
                .field("num4", &self.num4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigsela1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sigsela1 {{ num1: {=u8:?}, num2: {=u8:?}, num3: {=u8:?}, num4: {=u8:?} }}",
                self.num1(),
                self.num2(),
                self.num3(),
                self.num4()
            )
        }
    }
    #[doc = "Signal Select2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela2(pub u32);
    impl Sigsela2 {
        #[doc = "Select sample signal bit number1 in second group."]
        #[must_use]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number1 in second group."]
        #[inline(always)]
        pub const fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select sample signal bit number2 in second group."]
        #[must_use]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number2 in second group."]
        #[inline(always)]
        pub const fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select sample signal bit number3 in second group."]
        #[must_use]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number3 in second group."]
        #[inline(always)]
        pub const fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select sample signal bit number4 in second group."]
        #[must_use]
        #[inline(always)]
        pub const fn num4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number4 in second group."]
        #[inline(always)]
        pub const fn set_num4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela2 {
        #[inline(always)]
        fn default() -> Sigsela2 {
            Sigsela2(0)
        }
    }
    impl core::fmt::Debug for Sigsela2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigsela2")
                .field("num1", &self.num1())
                .field("num2", &self.num2())
                .field("num3", &self.num3())
                .field("num4", &self.num4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigsela2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sigsela2 {{ num1: {=u8:?}, num2: {=u8:?}, num3: {=u8:?}, num4: {=u8:?} }}",
                self.num1(),
                self.num2(),
                self.num3(),
                self.num4()
            )
        }
    }
    #[doc = "Start Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Startaddr(pub u32);
    impl Startaddr {
        #[doc = "Start address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Startaddr {
        #[inline(always)]
        fn default() -> Startaddr {
            Startaddr(0)
        }
    }
    impl core::fmt::Debug for Startaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Startaddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Startaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Startaddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Stream Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Streamctrl(pub u32);
    impl Streamctrl {
        #[doc = "Burst Cfg 3b011 Incr4 3b101 Incr8 3b111 Incr16."]
        #[must_use]
        #[inline(always)]
        pub const fn burst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Cfg 3b011 Incr4 3b101 Incr8 3b111 Incr16."]
        #[inline(always)]
        pub const fn set_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Sample Rate 4 take one every 5 5 take one every 6 6 take one every 7."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Sample Rate 4 take one every 5 5 take one every 6 6 take one every 7."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "FIFO Overflow Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn full_clear(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Overflow Clear."]
        #[inline(always)]
        pub const fn set_full_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Signal Group Select 0 128bit from one group 1 from 2 groups, 4bit in each group."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Signal Group Select 0 128bit from one group 1 from 2 groups, 4bit in each group."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Streamctrl {
        #[inline(always)]
        fn default() -> Streamctrl {
            Streamctrl(0)
        }
    }
    impl core::fmt::Debug for Streamctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Streamctrl")
                .field("burst", &self.burst())
                .field("sample", &self.sample())
                .field("full_clear", &self.full_clear())
                .field("sel", &self.sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Streamctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Streamctrl {{ burst: {=u8:?}, sample: {=u8:?}, full_clear: {=bool:?}, sel: {=bool:?} }}" , self . burst () , self . sample () , self . full_clear () , self . sel ())
        }
    }
    #[doc = "Trigger Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigctrl(pub u32);
    impl Trigctrl {
        #[doc = "Trigger Signal Comparison type select. 0b000 Trigger Signal Comparisons disabled. The enabled counters count clocks immediately after the Trigger State has been entered and generate a programmable Output Action and transition to the next Trigger State when the Counter Compare Register count is reached, that is when a Trigger Counter Comparison match occurs. 0b001 Compare type is equal (==). 0b010 Compare type is greater than (>). 0b011 Compare type is greater than or equal (>=). 0b101 Compare type is not equal (!=). 0b110 Compare type is less than (<). 0b111 Compare type is less than or equal (<=)."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger Signal Comparison type select. 0b000 Trigger Signal Comparisons disabled. The enabled counters count clocks immediately after the Trigger State has been entered and generate a programmable Output Action and transition to the next Trigger State when the Counter Compare Register count is reached, that is when a Trigger Counter Comparison match occurs. 0b001 Compare type is equal (==). 0b010 Compare type is greater than (>). 0b011 Compare type is greater than or equal (>=). 0b101 Compare type is not equal (!=). 0b110 Compare type is less than (<). 0b111 Compare type is less than or equal (<=)."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Comparison mode. Acts as both a counter enable and a select for the comparison mode. 0b0 Disable counters and select Trigger Signal Comparison mode. 0b1 Enable counters and select Trigger Counter Comparison mode."]
        #[must_use]
        #[inline(always)]
        pub const fn compsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison mode. Acts as both a counter enable and a select for the comparison mode. 0b0 Disable counters and select Trigger Signal Comparison mode. 0b1 Enable counters and select Trigger Counter Comparison mode."]
        #[inline(always)]
        pub const fn set_compsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Trace capture control. 0b10 Trace is captured every ELACLK cycle. others Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn trace(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Trace capture control. 0b10 Trace is captured every ELACLK cycle. others Reserved."]
        #[inline(always)]
        pub const fn set_trace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Trigctrl {
        #[inline(always)]
        fn default() -> Trigctrl {
            Trigctrl(0)
        }
    }
    impl core::fmt::Debug for Trigctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trigctrl")
                .field("comp", &self.comp())
                .field("compsel", &self.compsel())
                .field("trace", &self.trace())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trigctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trigctrl {{ comp: {=u8:?}, compsel: {=bool:?}, trace: {=u8:?} }}",
                self.comp(),
                self.compsel(),
                self.trace()
            )
        }
    }
}
