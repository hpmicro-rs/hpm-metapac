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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Stream Control Register."]
    #[inline(always)]
    pub const fn streamctrl(self) -> crate::common::Reg<regs::Streamctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Pre-trigger Action Register."]
    #[inline(always)]
    pub const fn ptaction(self) -> crate::common::Reg<regs::Ptaction, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Start Address Register."]
    #[inline(always)]
    pub const fn startaddr(self) -> crate::common::Reg<regs::Startaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "End Address Register."]
    #[inline(always)]
    pub const fn endaddr(self) -> crate::common::Reg<regs::Endaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Current Trigger State Register."]
    #[inline(always)]
    pub const fn ctsr(self) -> crate::common::Reg<regs::Ctsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Current Counter Value Register."]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Current Action Value Register."]
    #[inline(always)]
    pub const fn cavr(self) -> crate::common::Reg<regs::Cavr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Fifo State Register."]
    #[inline(always)]
    pub const fn fifostate(self) -> crate::common::Reg<regs::Fifostate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Final Address Register."]
    #[inline(always)]
    pub const fn finaladdr(self) -> crate::common::Reg<regs::Finaladdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Group Select Register."]
    #[inline(always)]
    pub const fn grpsela(self) -> crate::common::Reg<regs::Grpsela, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Group Enable Register."]
    #[inline(always)]
    pub const fn grpena(self) -> crate::common::Reg<regs::Grpena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Signal Select1 Register."]
    #[inline(always)]
    pub const fn sigsela1(self) -> crate::common::Reg<regs::Sigsela1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Signal Select2 Register."]
    #[inline(always)]
    pub const fn sigsela2(self) -> crate::common::Reg<regs::Sigsela2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Signal Enable Register."]
    #[inline(always)]
    pub const fn sigena(self) -> crate::common::Reg<regs::Sigena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn state(self, n: usize) -> State {
        assert!(n < 5usize);
        unsafe { State::from_ptr(self.ptr.add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "Lock Access Register."]
    #[inline(always)]
    pub const fn lar(self) -> crate::common::Reg<regs::Lar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fb0usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Trigger Control Register."]
    #[inline(always)]
    pub const fn trigctrl(self) -> crate::common::Reg<regs::Trigctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Next State Register."]
    #[inline(always)]
    pub const fn nextstate(self) -> crate::common::Reg<regs::Nextstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Action Register."]
    #[inline(always)]
    pub const fn action(self) -> crate::common::Reg<regs::Action, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Counter Compare Register."]
    #[inline(always)]
    pub const fn countcomp(self) -> crate::common::Reg<regs::Countcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "External Mask Register."]
    #[inline(always)]
    pub const fn extmask(self) -> crate::common::Reg<regs::Extmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "External Compare Register."]
    #[inline(always)]
    pub const fn extcomp(self) -> crate::common::Reg<regs::Extcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Signal Mask Register."]
    #[inline(always)]
    pub const fn sigmask(self) -> crate::common::Reg<regs::Sigmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Compare Enable register."]
    #[inline(always)]
    pub const fn compen(self) -> crate::common::Reg<regs::Compen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Signal Compare Register0."]
    #[inline(always)]
    pub const fn sigcomp0(self) -> crate::common::Reg<regs::Sigcomp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Signal Compare Register1."]
    #[inline(always)]
    pub const fn sigcomp1(self) -> crate::common::Reg<regs::Sigcomp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Signal Compare Register2."]
    #[inline(always)]
    pub const fn sigcomp2(self) -> crate::common::Reg<regs::Sigcomp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Signal Compare Register3."]
    #[inline(always)]
    pub const fn sigcomp3(self) -> crate::common::Reg<regs::Sigcomp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Action Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Action(pub u32);
    impl Action {
        #[doc = "Trace active. 0b0 Trace disable. 0b1 Trace enable."]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Trace active. 0b0 Trace disable. 0b1 Trace enable."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Action {
        #[inline(always)]
        fn default() -> Action {
            Action(0)
        }
    }
    #[doc = "Current Action Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cavr(pub u32);
    impl Cavr {
        #[doc = "Trace active. 0b0 Trace is not active. 0b1 Trace is active."]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Trace active. 0b0 Trace is not active. 0b1 Trace is active."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cavr {
        #[inline(always)]
        fn default() -> Cavr {
            Cavr(0)
        }
    }
    #[doc = "Current Counter Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "Returns the counter value when the CTSR was last read. If the CTSR has never been read, then the value in the CCVR is undefined."]
        #[inline(always)]
        pub const fn ccvr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Returns the counter value when the CTSR was last read. If the CTSR has never been read, then the value in the CCVR is undefined."]
        #[inline(always)]
        pub fn set_ccvr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ccvr {
        #[inline(always)]
        fn default() -> Ccvr {
            Ccvr(0)
        }
    }
    #[doc = "Compare Enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Compen(pub u32);
    impl Compen {
        #[doc = "Select compare signal number0-3."]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Select compare signal number0-3."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Compen {
        #[inline(always)]
        fn default() -> Compen {
            Compen(0)
        }
    }
    #[doc = "Counter Compare Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countcomp(pub u32);
    impl Countcomp {
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Countcomp {
        #[inline(always)]
        fn default() -> Countcomp {
            Countcomp(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Run control. 0 LOBS disabled. Register programming permitted. 1 LOBS enabled."]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Run control. 0 LOBS disabled. Register programming permitted. 1 LOBS enabled."]
        #[inline(always)]
        pub fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Current Trigger State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctsr(pub u32);
    impl Ctsr {
        #[doc = "Reads current Trigger State. This is a one-hot encoded field. When CTRL.RUN: 0 RAZ. 1 Returns current Trigger State. If FINALSTATE is 1, then the CTSR field gives the Trigger State when FINALSTATE became 1."]
        #[inline(always)]
        pub const fn ctsr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Reads current Trigger State. This is a one-hot encoded field. When CTRL.RUN: 0 RAZ. 1 Returns current Trigger State. If FINALSTATE is 1, then the CTSR field gives the Trigger State when FINALSTATE became 1."]
        #[inline(always)]
        pub fn set_ctsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "0 LOBS is still tracing. 1 Indicates that the LOBS has stopped advancing Trigger States and stopped trace. FINALSTATE can be set by TRIGCTRL.COUNTBRK reaching the final loop count, or by programming NEXTSTATEto zero."]
        #[inline(always)]
        pub const fn finalstate(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "0 LOBS is still tracing. 1 Indicates that the LOBS has stopped advancing Trigger States and stopped trace. FINALSTATE can be set by TRIGCTRL.COUNTBRK reaching the final loop count, or by programming NEXTSTATEto zero."]
        #[inline(always)]
        pub fn set_finalstate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctsr {
        #[inline(always)]
        fn default() -> Ctsr {
            Ctsr(0)
        }
    }
    #[doc = "End Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Endaddr(pub u32);
    impl Endaddr {
        #[doc = "End address."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "End address."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Endaddr {
        #[inline(always)]
        fn default() -> Endaddr {
            Endaddr(0)
        }
    }
    #[doc = "External Compare Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extcomp(pub u32);
    impl Extcomp {
        #[doc = "External Compare."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "External Compare."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Extcomp {
        #[inline(always)]
        fn default() -> Extcomp {
            Extcomp(0)
        }
    }
    #[doc = "External Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extmask(pub u32);
    impl Extmask {
        #[doc = "External Mask."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "External Mask."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Extmask {
        #[inline(always)]
        fn default() -> Extmask {
            Extmask(0)
        }
    }
    #[doc = "Fifo State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifostate(pub u32);
    impl Fifostate {
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO empty."]
        #[inline(always)]
        pub fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO full."]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full."]
        #[inline(always)]
        pub fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Fifostate {
        #[inline(always)]
        fn default() -> Fifostate {
            Fifostate(0)
        }
    }
    #[doc = "Final Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Finaladdr(pub u32);
    impl Finaladdr {
        #[doc = "Final address."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Final address."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Finaladdr {
        #[inline(always)]
        fn default() -> Finaladdr {
            Finaladdr(0)
        }
    }
    #[doc = "Group Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grpena(pub u32);
    impl Grpena {
        #[doc = "Enable sample group number1."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable sample group number1."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable sample group number2."]
        #[inline(always)]
        pub const fn en2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable sample group number2."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Grpena {
        #[inline(always)]
        fn default() -> Grpena {
            Grpena(0)
        }
    }
    #[doc = "Group Select Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grpsela(pub u32);
    impl Grpsela {
        #[doc = "Select sample group number1."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Select sample group number1."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Select sample group number2."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Select sample group number2."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Grpsela {
        #[inline(always)]
        fn default() -> Grpsela {
            Grpsela(0)
        }
    }
    #[doc = "Lock Access Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lar(pub u32);
    impl Lar {
        #[doc = "Lock Access Value."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Lock Access Value."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lar {
        #[inline(always)]
        fn default() -> Lar {
            Lar(0)
        }
    }
    #[doc = "Next State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nextstate(pub u32);
    impl Nextstate {
        #[doc = "Selects the next state to move to after the Trigger Condition has been met in the current state. 0x0 Do not change state. This is the final Trigger State. 0x1 Selects Trigger State 0. 0x2 Selects Trigger State 1. 0x4 Selects Trigger State 2. 0x8 Selects Trigger State 3. 0x10 Selects Trigger State 4."]
        #[inline(always)]
        pub const fn nextstate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Selects the next state to move to after the Trigger Condition has been met in the current state. 0x0 Do not change state. This is the final Trigger State. 0x1 Selects Trigger State 0. 0x2 Selects Trigger State 1. 0x4 Selects Trigger State 2. 0x8 Selects Trigger State 3. 0x10 Selects Trigger State 4."]
        #[inline(always)]
        pub fn set_nextstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Nextstate {
        #[inline(always)]
        fn default() -> Nextstate {
            Nextstate(0)
        }
    }
    #[doc = "Pre-trigger Action Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptaction(pub u32);
    impl Ptaction {
        #[doc = "Enables trace."]
        #[inline(always)]
        pub const fn trace(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enables trace."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ptaction {
        #[inline(always)]
        fn default() -> Ptaction {
            Ptaction(0)
        }
    }
    #[doc = "Signal Compare Register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp0(pub u32);
    impl Sigcomp0 {
        #[doc = "Compare golden value for Signal Group signals\\[31:0\\]."]
        #[inline(always)]
        pub const fn value0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[31:0\\]."]
        #[inline(always)]
        pub fn set_value0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp0 {
        #[inline(always)]
        fn default() -> Sigcomp0 {
            Sigcomp0(0)
        }
    }
    #[doc = "Signal Compare Register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp1(pub u32);
    impl Sigcomp1 {
        #[doc = "Compare golden value for Signal Group signals\\[63:32\\]."]
        #[inline(always)]
        pub const fn value1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[63:32\\]."]
        #[inline(always)]
        pub fn set_value1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp1 {
        #[inline(always)]
        fn default() -> Sigcomp1 {
            Sigcomp1(0)
        }
    }
    #[doc = "Signal Compare Register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp2(pub u32);
    impl Sigcomp2 {
        #[doc = "Compare golden value for Signal Group signals\\[95:64\\]."]
        #[inline(always)]
        pub const fn value2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[95:64\\]."]
        #[inline(always)]
        pub fn set_value2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp2 {
        #[inline(always)]
        fn default() -> Sigcomp2 {
            Sigcomp2(0)
        }
    }
    #[doc = "Signal Compare Register3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigcomp3(pub u32);
    impl Sigcomp3 {
        #[doc = "Compare golden value for Signal Group signals\\[127:96\\]."]
        #[inline(always)]
        pub const fn value3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare golden value for Signal Group signals\\[127:96\\]."]
        #[inline(always)]
        pub fn set_value3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sigcomp3 {
        #[inline(always)]
        fn default() -> Sigcomp3 {
            Sigcomp3(0)
        }
    }
    #[doc = "Signal Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigena(pub u32);
    impl Sigena {
        #[doc = "Enable sample signal number1."]
        #[inline(always)]
        pub const fn en1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable sample signal number1."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Enable sample signal number2."]
        #[inline(always)]
        pub const fn en2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable sample signal number2."]
        #[inline(always)]
        pub fn set_en2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Sigena {
        #[inline(always)]
        fn default() -> Sigena {
            Sigena(0)
        }
    }
    #[doc = "Signal Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigmask(pub u32);
    impl Sigmask {
        #[doc = "Select compare signal number0."]
        #[inline(always)]
        pub const fn num0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number0."]
        #[inline(always)]
        pub fn set_num0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select compare signal number1."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number1."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select compare signal number2."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number2."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select compare signal number3."]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select compare signal number3."]
        #[inline(always)]
        pub fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigmask {
        #[inline(always)]
        fn default() -> Sigmask {
            Sigmask(0)
        }
    }
    #[doc = "Signal Select Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsel(pub u32);
    impl Sigsel {
        #[doc = "Selects Signal Group. 0x1 Selects Signal Group 0. 0x2 Selects Signal Group 1. 0x4 Selects Signal Group 2. 0x8 Selects Signal Group 3. 0x10 Selects Signal Group 4. 0x20 Selects Signal Group 5. 0x40 Selects Signal Group 6. 0x80 Selects Signal Group 7. 0x100 Selects Signal Group 8. 0x200 Selects Signal Group 9. 0x400 Selects Signal Group 10. 0x800 Selects Signal Group 11."]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Selects Signal Group. 0x1 Selects Signal Group 0. 0x2 Selects Signal Group 1. 0x4 Selects Signal Group 2. 0x8 Selects Signal Group 3. 0x10 Selects Signal Group 4. 0x20 Selects Signal Group 5. 0x40 Selects Signal Group 6. 0x80 Selects Signal Group 7. 0x100 Selects Signal Group 8. 0x200 Selects Signal Group 9. 0x400 Selects Signal Group 10. 0x800 Selects Signal Group 11."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Sigsel {
        #[inline(always)]
        fn default() -> Sigsel {
            Sigsel(0)
        }
    }
    #[doc = "Signal Select1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela1(pub u32);
    impl Sigsela1 {
        #[doc = "Select sample signal bit number1 in first group."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number1 in first group."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select sample signal bit number2 in first group."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number2 in first group."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select sample signal bit number3 in first group."]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number3 in first group."]
        #[inline(always)]
        pub fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select sample signal bit number4 in first group."]
        #[inline(always)]
        pub const fn num4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number4 in first group."]
        #[inline(always)]
        pub fn set_num4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela1 {
        #[inline(always)]
        fn default() -> Sigsela1 {
            Sigsela1(0)
        }
    }
    #[doc = "Signal Select2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela2(pub u32);
    impl Sigsela2 {
        #[doc = "Select sample signal bit number1 in second group."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number1 in second group."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select sample signal bit number2 in second group."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number2 in second group."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select sample signal bit number3 in second group."]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number3 in second group."]
        #[inline(always)]
        pub fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select sample signal bit number4 in second group."]
        #[inline(always)]
        pub const fn num4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select sample signal bit number4 in second group."]
        #[inline(always)]
        pub fn set_num4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela2 {
        #[inline(always)]
        fn default() -> Sigsela2 {
            Sigsela2(0)
        }
    }
    #[doc = "Start Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Startaddr(pub u32);
    impl Startaddr {
        #[doc = "Start address."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start address."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Startaddr {
        #[inline(always)]
        fn default() -> Startaddr {
            Startaddr(0)
        }
    }
    #[doc = "Stream Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Streamctrl(pub u32);
    impl Streamctrl {
        #[doc = "Burst Cfg 3b011 Incr4 3b101 Incr8 3b111 Incr16."]
        #[inline(always)]
        pub const fn burst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Cfg 3b011 Incr4 3b101 Incr8 3b111 Incr16."]
        #[inline(always)]
        pub fn set_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Sample Rate 4 take one every 5 5 take one every 6 6 take one every 7."]
        #[inline(always)]
        pub const fn sample(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Sample Rate 4 take one every 5 5 take one every 6 6 take one every 7."]
        #[inline(always)]
        pub fn set_sample(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "FIFO Overflow Clear."]
        #[inline(always)]
        pub const fn full_clear(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Overflow Clear."]
        #[inline(always)]
        pub fn set_full_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Signal Group Select 0 128bit from one group 1 from 2 groups, 4bit in each group."]
        #[inline(always)]
        pub const fn sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Signal Group Select 0 128bit from one group 1 from 2 groups, 4bit in each group."]
        #[inline(always)]
        pub fn set_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Streamctrl {
        #[inline(always)]
        fn default() -> Streamctrl {
            Streamctrl(0)
        }
    }
    #[doc = "Trigger Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigctrl(pub u32);
    impl Trigctrl {
        #[doc = "Trigger Signal Comparison type select. 0b000 Trigger Signal Comparisons disabled. The enabled counters count clocks immediately after the Trigger State has been entered and generate a programmable Output Action and transition to the next Trigger State when the Counter Compare Register count is reached, that is when a Trigger Counter Comparison match occurs. 0b001 Compare type is equal (==). 0b010 Compare type is greater than (>). 0b011 Compare type is greater than or equal (>=). 0b101 Compare type is not equal (!=). 0b110 Compare type is less than (<). 0b111 Compare type is less than or equal (<=)."]
        #[inline(always)]
        pub const fn comp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger Signal Comparison type select. 0b000 Trigger Signal Comparisons disabled. The enabled counters count clocks immediately after the Trigger State has been entered and generate a programmable Output Action and transition to the next Trigger State when the Counter Compare Register count is reached, that is when a Trigger Counter Comparison match occurs. 0b001 Compare type is equal (==). 0b010 Compare type is greater than (>). 0b011 Compare type is greater than or equal (>=). 0b101 Compare type is not equal (!=). 0b110 Compare type is less than (<). 0b111 Compare type is less than or equal (<=)."]
        #[inline(always)]
        pub fn set_comp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Comparison mode. Acts as both a counter enable and a select for the comparison mode. 0b0 Disable counters and select Trigger Signal Comparison mode. 0b1 Enable counters and select Trigger Counter Comparison mode."]
        #[inline(always)]
        pub const fn compsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison mode. Acts as both a counter enable and a select for the comparison mode. 0b0 Disable counters and select Trigger Signal Comparison mode. 0b1 Enable counters and select Trigger Counter Comparison mode."]
        #[inline(always)]
        pub fn set_compsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Trace capture control. 0b10 Trace is captured every ELACLK cycle. others Reserved."]
        #[inline(always)]
        pub const fn trace(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Trace capture control. 0b10 Trace is captured every ELACLK cycle. others Reserved."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Trigctrl {
        #[inline(always)]
        fn default() -> Trigctrl {
            Trigctrl(0)
        }
    }
}
