#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RNG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Error Registers."]
    #[inline(always)]
    pub const fn err(self) -> crate::common::Reg<regs::Err, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FIFO out to bus/cpu."]
    #[inline(always)]
    pub const fn fo2b(self) -> crate::common::Reg<regs::Fo2b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn r2sk(self, n: usize) -> crate::common::Reg<regs::R2sk, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Self Test, when both ST and GS triggered, ST first and GS next."]
        #[inline(always)]
        pub const fn slfchk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Self Test, when both ST and GS triggered, ST first and GS next."]
        #[inline(always)]
        pub fn set_slfchk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Generate Seed, when both ST and GS triggered, ST first and GS next."]
        #[inline(always)]
        pub const fn gensd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Generate Seed, when both ST and GS triggered, ST first and GS next."]
        #[inline(always)]
        pub fn set_gensd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt."]
        #[inline(always)]
        pub const fn clrint(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt."]
        #[inline(always)]
        pub fn set_clrint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
        #[inline(always)]
        pub const fn clrerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
        #[inline(always)]
        pub fn set_clrerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
        #[inline(always)]
        pub const fn fufmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
        #[inline(always)]
        pub fn set_fufmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Auto Reseed."]
        #[inline(always)]
        pub const fn autrsd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Reseed."]
        #[inline(always)]
        pub fn set_autrsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
        #[inline(always)]
        pub const fn mirqdn(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
        #[inline(always)]
        pub fn set_mirqdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mask Interrupt Request for Error."]
        #[inline(always)]
        pub const fn mirqerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mask Interrupt Request for Error."]
        #[inline(always)]
        pub fn set_mirqerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Error Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Err(pub u32);
    impl Err {
        #[doc = "Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]."]
        #[inline(always)]
        pub const fn sckerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]."]
        #[inline(always)]
        pub fn set_sckerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FIFO access error(underflow)."]
        #[inline(always)]
        pub const fn fufe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO access error(underflow)."]
        #[inline(always)]
        pub fn set_fufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Err {
        #[inline(always)]
        fn default() -> Err {
            Err(0)
        }
    }
    #[doc = "FIFO out to bus/cpu."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fo2b(pub u32);
    impl Fo2b {
        #[doc = "SW read the FIFO output."]
        #[inline(always)]
        pub const fn fo2b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SW read the FIFO output."]
        #[inline(always)]
        pub fn set_fo2b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fo2b {
        #[inline(always)]
        fn default() -> Fo2b {
            Fo2b(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R2sk(pub u32);
    impl R2sk {
        #[doc = "FIFO out to KMAN, will be SDP engine key."]
        #[inline(always)]
        pub const fn fo2s0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO out to KMAN, will be SDP engine key."]
        #[inline(always)]
        pub fn set_fo2s0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for R2sk {
        #[inline(always)]
        fn default() -> R2sk {
            R2sk(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "when 1, means the RNG engine is busy for seeding or random number generation, self test and so on."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "when 1, means the RNG engine is busy for seeding or random number generation, self test and so on."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again."]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again."]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD\\[GS\\], or automatically if the CTRL\\[ARS\\]
is set."]
        #[inline(always)]
        pub const fn rsdreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD\\[GS\\], or automatically if the CTRL\\[ARS\\]
is set."]
        #[inline(always)]
        pub fn set_rsdreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD\\[ST\\]. 0 Self test not completed 1 Completed a self test since the last reset."]
        #[inline(always)]
        pub const fn scdn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD\\[ST\\]. 0 Self test not completed 1 Completed a self test since the last reset."]
        #[inline(always)]
        pub fn set_scdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1st Seed done When \"1\", Indicates that the RNG generated the first seed."]
        #[inline(always)]
        pub const fn fsddn(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1st Seed done When \"1\", Indicates that the RNG generated the first seed."]
        #[inline(always)]
        pub fn set_fsddn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "New seed done."]
        #[inline(always)]
        pub const fn nsddn(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "New seed done."]
        #[inline(always)]
        pub fn set_nsddn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Fifo Level, Indicates the number of random words currently in the output FIFO."]
        #[inline(always)]
        pub const fn frnnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Fifo Level, Indicates the number of random words currently in the output FIFO."]
        #[inline(always)]
        pub fn set_frnnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Fifo Size, it is 5 in this design."]
        #[inline(always)]
        pub const fn fsize(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Fifo Size, it is 5 in this design."]
        #[inline(always)]
        pub fn set_fsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Error was detected, check ESR register for details."]
        #[inline(always)]
        pub const fn funcerr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error was detected, check ESR register for details."]
        #[inline(always)]
        pub fn set_funcerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Self Check Pass Fail."]
        #[inline(always)]
        pub const fn scpf(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "Self Check Pass Fail."]
        #[inline(always)]
        pub fn set_scpf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
}
