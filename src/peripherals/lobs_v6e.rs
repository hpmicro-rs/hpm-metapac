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
    #[doc = "Signal Select0 Register."]
    #[inline(always)]
    pub const fn sigsela0(self) -> crate::common::Reg<regs::Sigsela0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Signal Select1 Register."]
    #[inline(always)]
    pub const fn sigsela1(self) -> crate::common::Reg<regs::Sigsela1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Signal Enable0 Register."]
    #[inline(always)]
    pub const fn sigena0(self) -> crate::common::Reg<regs::Sigena0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Signal Enable1 Register."]
    #[inline(always)]
    pub const fn sigena1(self) -> crate::common::Reg<regs::Sigena1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Signal Select Register."]
    #[inline(always)]
    pub const fn sigsel0(self) -> crate::common::Reg<regs::Sigsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Trigger Control Register."]
    #[inline(always)]
    pub const fn trigctrl0(self) -> crate::common::Reg<regs::Trigctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Next State Register."]
    #[inline(always)]
    pub const fn nextstate0(self) -> crate::common::Reg<regs::Nextstate0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Action Register."]
    #[inline(always)]
    pub const fn action0(self) -> crate::common::Reg<regs::Action0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Signal Mask Register."]
    #[inline(always)]
    pub const fn countcomp0(self) -> crate::common::Reg<regs::Countcomp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Counter Compare Register."]
    #[inline(always)]
    pub const fn sigmask0(self) -> crate::common::Reg<regs::Sigmask0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Compare Enable register."]
    #[inline(always)]
    pub const fn compen0(self) -> crate::common::Reg<regs::Compen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
}
pub mod regs {
    #[doc = "Action Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Action0(pub u32);
    impl Action0 {
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
    impl Default for Action0 {
        #[inline(always)]
        fn default() -> Action0 {
            Action0(0)
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
    pub struct Compen0(pub u32);
    impl Compen0 {}
    impl Default for Compen0 {
        #[inline(always)]
        fn default() -> Compen0 {
            Compen0(0)
        }
    }
    #[doc = "Signal Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countcomp0(pub u32);
    impl Countcomp0 {
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[inline(always)]
        pub const fn trace(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Countcomp0 {
        #[inline(always)]
        fn default() -> Countcomp0 {
            Countcomp0(0)
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
        #[doc = "Enable trace group number0-1."]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Enable trace group number0-1."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
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
        #[doc = "Select trace group number0."]
        #[inline(always)]
        pub const fn num0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Select trace group number0."]
        #[inline(always)]
        pub fn set_num0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Select trace group number1."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Select trace group number1."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Grpsela {
        #[inline(always)]
        fn default() -> Grpsela {
            Grpsela(0)
        }
    }
    #[doc = "Next State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nextstate0(pub u32);
    impl Nextstate0 {
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
    impl Default for Nextstate0 {
        #[inline(always)]
        fn default() -> Nextstate0 {
            Nextstate0(0)
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
    #[doc = "Signal Enable0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigena0(pub u32);
    impl Sigena0 {
        #[doc = "Enable trace signal number0-3."]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable trace signal number0-3."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Sigena0 {
        #[inline(always)]
        fn default() -> Sigena0 {
            Sigena0(0)
        }
    }
    #[doc = "Signal Enable1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigena1(pub u32);
    impl Sigena1 {
        #[doc = "Enable trace signal number0-3."]
        #[inline(always)]
        pub const fn en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Enable trace signal number0-3."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Sigena1 {
        #[inline(always)]
        fn default() -> Sigena1 {
            Sigena1(0)
        }
    }
    #[doc = "Counter Compare Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigmask0(pub u32);
    impl Sigmask0 {
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
    impl Default for Sigmask0 {
        #[inline(always)]
        fn default() -> Sigmask0 {
            Sigmask0(0)
        }
    }
    #[doc = "Signal Select Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsel0(pub u32);
    impl Sigsel0 {
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
    impl Default for Sigsel0 {
        #[inline(always)]
        fn default() -> Sigsel0 {
            Sigsel0(0)
        }
    }
    #[doc = "Signal Select0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela0(pub u32);
    impl Sigsela0 {
        #[doc = "Select trace signal number0 in group0."]
        #[inline(always)]
        pub const fn num0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number0 in group0."]
        #[inline(always)]
        pub fn set_num0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select trace signal number1 in group0."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number1 in group0."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select trace signal number2 in group0."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number2 in group0."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select trace signal number3 in group0."]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number3 in group0."]
        #[inline(always)]
        pub fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela0 {
        #[inline(always)]
        fn default() -> Sigsela0 {
            Sigsela0(0)
        }
    }
    #[doc = "Signal Select1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigsela1(pub u32);
    impl Sigsela1 {
        #[doc = "Select trace signal number0 in group1."]
        #[inline(always)]
        pub const fn num0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number0 in group1."]
        #[inline(always)]
        pub fn set_num0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Select trace signal number1 in group1."]
        #[inline(always)]
        pub const fn num1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number1 in group1."]
        #[inline(always)]
        pub fn set_num1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Select trace signal number2 in group1."]
        #[inline(always)]
        pub const fn num2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number2 in group1."]
        #[inline(always)]
        pub fn set_num2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Select trace signal number3 in group1."]
        #[inline(always)]
        pub const fn num3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Select trace signal number3 in group1."]
        #[inline(always)]
        pub fn set_num3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sigsela1 {
        #[inline(always)]
        fn default() -> Sigsela1 {
            Sigsela1(0)
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
        #[doc = "Burst Cfg 0b011 Incr4 0b101 Incr8 0b111 Incr16."]
        #[inline(always)]
        pub const fn burst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Cfg 0b011 Incr4 0b101 Incr8 0b111 Incr16."]
        #[inline(always)]
        pub fn set_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Sample Rate 0b110 Incr4 take one every four 0b101 Incr8 take one every five 0b100 Incr16 take one every six."]
        #[inline(always)]
        pub const fn sample(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Sample Rate 0b110 Incr4 take one every four 0b101 Incr8 take one every five 0b100 Incr16 take one every six."]
        #[inline(always)]
        pub fn set_sample(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "FIFO Overflow Clear."]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Overflow Clear."]
        #[inline(always)]
        pub fn set_clear(&mut self, val: bool) {
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
    pub struct Trigctrl0(pub u32);
    impl Trigctrl0 {
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
        #[doc = "Counter reset. 0b0 Do not reset the counter after a Trigger Signal Comparison match. 0b1 Reset the counter after a Trigger Signal Comparison match The counter acts like an activity watchdog timer, only allowing advancement to the next Trigger State when the Trigger Counter Comparison is reached. The counter is reset by a signal comparison."]
        #[inline(always)]
        pub const fn watchrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Counter reset. 0b0 Do not reset the counter after a Trigger Signal Comparison match. 0b1 Reset the counter after a Trigger Signal Comparison match The counter acts like an activity watchdog timer, only allowing advancement to the next Trigger State when the Trigger Counter Comparison is reached. The counter is reset by a signal comparison."]
        #[inline(always)]
        pub fn set_watchrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Counter source select. 0b0 Counter is incremented every ELACLK cycle. 0b1 Counter is incremented when Trigger Signal Comparison matches."]
        #[inline(always)]
        pub const fn countsrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Counter source select. 0b0 Counter is incremented every ELACLK cycle. 0b1 Counter is incremented when Trigger Signal Comparison matches."]
        #[inline(always)]
        pub fn set_countsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trace capture control. 0b00 Trace is captured when Trigger Signal Comparison succeeds. 0b01 Trace is captured when Trigger Counter Comparison succeeds. 0b10 Trace is captured every ELACLK cycle. 0b11 Reserved."]
        #[inline(always)]
        pub const fn trace(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Trace capture control. 0b00 Trace is captured when Trigger Signal Comparison succeeds. 0b01 Trace is captured when Trigger Counter Comparison succeeds. 0b10 Trace is captured every ELACLK cycle. 0b11 Reserved."]
        #[inline(always)]
        pub fn set_trace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Counter clear. 0b0 Do not clear the counter value when moving to a different NEXTSTATE. 0b1 Clear the counter value when moving to a different NEXTSTATE. Note TRIGCTRL.WATCHRST must be 0b0 when using this feature."]
        #[inline(always)]
        pub const fn countclr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Counter clear. 0b0 Do not clear the counter value when moving to a different NEXTSTATE. 0b1 Clear the counter value when moving to a different NEXTSTATE. Note TRIGCTRL.WATCHRST must be 0b0 when using this feature."]
        #[inline(always)]
        pub fn set_countclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop counter break. The loop counter break uses the Trigger State counter to break loops between Trigger States after a Trigger Counter Comparison. When the counter comparison matches, the Trigger State goes into a final state,which stops trace writes and leaves the output actions at the previous Trigger State ACTION value. 0b0 Normal operation. 0b1 Break Trigger State loop: A counter comparison match causes a transition to the final state, otherwise go to the NEXTSTATE Trigger State as the counter increments."]
        #[inline(always)]
        pub const fn counterbrk(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop counter break. The loop counter break uses the Trigger State counter to break loops between Trigger States after a Trigger Counter Comparison. When the counter comparison matches, the Trigger State goes into a final state,which stops trace writes and leaves the output actions at the previous Trigger State ACTION value. 0b0 Normal operation. 0b1 Break Trigger State loop: A counter comparison match causes a transition to the final state, otherwise go to the NEXTSTATE Trigger State as the counter increments."]
        #[inline(always)]
        pub fn set_counterbrk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Trigctrl0 {
        #[inline(always)]
        fn default() -> Trigctrl0 {
            Trigctrl0(0)
        }
    }
}
