#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DDRPHY."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddrphy {
    ptr: *mut u8,
}
unsafe impl Send for Ddrphy {}
unsafe impl Sync for Ddrphy {}
impl Ddrphy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Revision Identification Register."]
    #[inline(always)]
    pub const fn ridr(self) -> crate::common::Reg<regs::Ridr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PHY Initialization Register (PIR)."]
    #[inline(always)]
    pub const fn pir(self) -> crate::common::Reg<regs::Pir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PHY General Configuration Registers 0-1 (PGCR0- 1)."]
    #[inline(always)]
    pub const fn pgcr0(self) -> crate::common::Reg<regs::Pgcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PHY General Configuration Registers 0-1 (PGCR0- 1)."]
    #[inline(always)]
    pub const fn pgcr1(self) -> crate::common::Reg<regs::Pgcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89."]
    #[inline(always)]
    pub const fn pgsr0(self) -> crate::common::Reg<regs::Pgsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89."]
    #[inline(always)]
    pub const fn pgsr1(self) -> crate::common::Reg<regs::Pgsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "“PLL Control Register (PLLCR)” on page 91."]
    #[inline(always)]
    pub const fn pllcr(self) -> crate::common::Reg<regs::Pllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[inline(always)]
    pub const fn ptr0(self) -> crate::common::Reg<regs::Ptr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[inline(always)]
    pub const fn ptr1(self) -> crate::common::Reg<regs::Ptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[inline(always)]
    pub const fn ptr2(self) -> crate::common::Reg<regs::Ptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[inline(always)]
    pub const fn ptr3(self) -> crate::common::Reg<regs::Ptr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[inline(always)]
    pub const fn ptr4(self) -> crate::common::Reg<regs::Ptr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "“AC Master Delay Line Register (ACMDLR)” on page 96."]
    #[inline(always)]
    pub const fn acmdlr(self) -> crate::common::Reg<regs::Acmdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "“AC Bit Delay Line Register (ACBDLR)” on page 96."]
    #[inline(always)]
    pub const fn acbdlr(self) -> crate::common::Reg<regs::Acbdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "“AC I/O Configuration Register (ACIOCR)” on page 97."]
    #[inline(always)]
    pub const fn aciocr(self) -> crate::common::Reg<regs::Aciocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "“DATX8 Common Configuration Register (DXCCR)” on page 99."]
    #[inline(always)]
    pub const fn dxccr(self) -> crate::common::Reg<regs::Dxccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "“DDR System General Configuration Register (DSGCR)” on page 101."]
    #[inline(always)]
    pub const fn dsgcr(self) -> crate::common::Reg<regs::Dsgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "“DRAM Configuration Register (DCR)” on page 103."]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[inline(always)]
    pub const fn dtpr0(self) -> crate::common::Reg<regs::Dtpr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[inline(always)]
    pub const fn dtpr1(self) -> crate::common::Reg<regs::Dtpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[inline(always)]
    pub const fn dtpr2(self) -> crate::common::Reg<regs::Dtpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mr(self) -> crate::common::Reg<regs::Mr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "“Mode Register 0 (MR0)” on page 108."]
    #[inline(always)]
    pub const fn mr0(self) -> crate::common::Reg<regs::Mr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn emr(self) -> crate::common::Reg<regs::Emr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "“Mode Register 1 (MR1)” on page 111."]
    #[inline(always)]
    pub const fn mr1(self) -> crate::common::Reg<regs::Mr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn emr2(self) -> crate::common::Reg<regs::Emr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "“Mode Register 2/Extended Mode Register 2 (MR2/EMR2)” on page 114."]
    #[inline(always)]
    pub const fn mr2(self) -> crate::common::Reg<regs::Mr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn emr3(self) -> crate::common::Reg<regs::Emr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "“Mode Register 3 (MR3)” on page 116."]
    #[inline(always)]
    pub const fn mr3(self) -> crate::common::Reg<regs::Mr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "“ODT Configuration Register (ODTCR)” on page 117."]
    #[inline(always)]
    pub const fn odtcr(self) -> crate::common::Reg<regs::Odtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "“Data Training Configuration Register (DTCR)” on page 118."]
    #[inline(always)]
    pub const fn dtcr(self) -> crate::common::Reg<regs::Dtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[inline(always)]
    pub const fn dtar0(self) -> crate::common::Reg<regs::Dtar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[inline(always)]
    pub const fn dtar1(self) -> crate::common::Reg<regs::Dtar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[inline(always)]
    pub const fn dtar2(self) -> crate::common::Reg<regs::Dtar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[inline(always)]
    pub const fn dtar3(self) -> crate::common::Reg<regs::Dtar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[inline(always)]
    pub const fn dtdr0(self) -> crate::common::Reg<regs::Dtdr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[inline(always)]
    pub const fn dtdr1(self) -> crate::common::Reg<regs::Dtdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[inline(always)]
    pub const fn dtedr0(self) -> crate::common::Reg<regs::Dtedr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[inline(always)]
    pub const fn dtedr1(self) -> crate::common::Reg<regs::Dtedr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "“PHY General Configuration Register 2 (PGCR2)” on page 87."]
    #[inline(always)]
    pub const fn pgcr2(self) -> crate::common::Reg<regs::Pgcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1)."]
    #[inline(always)]
    pub const fn rdimmgcr0(self) -> crate::common::Reg<regs::Rdimmgcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1)."]
    #[inline(always)]
    pub const fn rdimmgcr1(self) -> crate::common::Reg<regs::Rdimmgcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "RDIMM Control Register 0-1 (RDIMMCR0-1)."]
    #[inline(always)]
    pub const fn rdimmcr0(self) -> crate::common::Reg<regs::Rdimmcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RDIMM Control Register 0-1 (RDIMMCR0-1)."]
    #[inline(always)]
    pub const fn rdimmcr1(self) -> crate::common::Reg<regs::Rdimmcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "“DCU Address Register (DCUAR)” on page 129."]
    #[inline(always)]
    pub const fn dcuar(self) -> crate::common::Reg<regs::Dcuar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "“DCU Data Register (DCUDR)” on page 130."]
    #[inline(always)]
    pub const fn dcudr(self) -> crate::common::Reg<regs::Dcudr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "“DCU Run Register (DCURR)” on page 130."]
    #[inline(always)]
    pub const fn dcurr(self) -> crate::common::Reg<regs::Dcurr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "“DCU Loop Register (DCULR)” on page 131."]
    #[inline(always)]
    pub const fn dculr(self) -> crate::common::Reg<regs::Dculr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "“DCU General Configuration Register (DCUGCR)” on page 132."]
    #[inline(always)]
    pub const fn dcugcr(self) -> crate::common::Reg<regs::Dcugcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "“DCU Timing Parameter Register (DCUTPR)” on page 132."]
    #[inline(always)]
    pub const fn dcutpr(self) -> crate::common::Reg<regs::Dcutpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "DCU Status Register 0-1 (DCUSR0-1)."]
    #[inline(always)]
    pub const fn dcusr0(self) -> crate::common::Reg<regs::Dcusr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "DCU Status Register 0-1 (DCUSR0-1)."]
    #[inline(always)]
    pub const fn dcusr1(self) -> crate::common::Reg<regs::Dcusr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "“BIST Run Register (BISTRR)” on page 133."]
    #[inline(always)]
    pub const fn bistrr(self) -> crate::common::Reg<regs::Bistrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "“BIST Word Count Register (BISTWCR)” on page 136."]
    #[inline(always)]
    pub const fn bistwcr(self) -> crate::common::Reg<regs::Bistwcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[inline(always)]
    pub const fn bistmskr0(self) -> crate::common::Reg<regs::Bistmskr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[inline(always)]
    pub const fn bistmskr1(self) -> crate::common::Reg<regs::Bistmskr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[inline(always)]
    pub const fn bistmskr2(self) -> crate::common::Reg<regs::Bistmskr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "“BIST LFSR Seed Register (BISTLSR)” on page 137."]
    #[inline(always)]
    pub const fn bistlsr(self) -> crate::common::Reg<regs::Bistlsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[inline(always)]
    pub const fn bistar0(self) -> crate::common::Reg<regs::Bistar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[inline(always)]
    pub const fn bistar1(self) -> crate::common::Reg<regs::Bistar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[inline(always)]
    pub const fn bistar2(self) -> crate::common::Reg<regs::Bistar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "“BIST User Data Pattern Register (BISTUDPR)” on page 138."]
    #[inline(always)]
    pub const fn bistudpr(self) -> crate::common::Reg<regs::Bistudpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "“BIST General Status Register (BISTGSR)” on page 139."]
    #[inline(always)]
    pub const fn bistgsr(self) -> crate::common::Reg<regs::Bistgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "“BIST Word Error Register (BISTWER)” on page 139."]
    #[inline(always)]
    pub const fn bistwer(self) -> crate::common::Reg<regs::Bistwer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[inline(always)]
    pub const fn bistber0(self) -> crate::common::Reg<regs::Bistber0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[inline(always)]
    pub const fn bistber1(self) -> crate::common::Reg<regs::Bistber1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[inline(always)]
    pub const fn bistber2(self) -> crate::common::Reg<regs::Bistber2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[inline(always)]
    pub const fn bistber3(self) -> crate::common::Reg<regs::Bistber3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "“BIST Word Count Status Register (BISTWCSR)” on page 141."]
    #[inline(always)]
    pub const fn bistwcsr(self) -> crate::common::Reg<regs::Bistwcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[inline(always)]
    pub const fn bistfwr0(self) -> crate::common::Reg<regs::Bistfwr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[inline(always)]
    pub const fn bistfwr1(self) -> crate::common::Reg<regs::Bistfwr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[inline(always)]
    pub const fn bistfwr2(self) -> crate::common::Reg<regs::Bistfwr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "“Anti-Aging Control Register (AACR)” on page 143."]
    #[inline(always)]
    pub const fn aacr(self) -> crate::common::Reg<regs::Aacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "General Purpose Register 0-1 (GPR0-1)."]
    #[inline(always)]
    pub const fn gpr0(self) -> crate::common::Reg<regs::Gpr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "General Purpose Register 0-1 (GPR0-1)."]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<regs::Gpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn zq(self, n: usize) -> Zq {
        assert!(n < 4usize);
        unsafe { Zq::from_ptr(self.ptr.add(0x0180usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dx(self, n: usize) -> Dx {
        assert!(n < 9usize);
        unsafe { Dx::from_ptr(self.ptr.add(0x01c0usize + n * 64usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dx {
    ptr: *mut u8,
}
unsafe impl Send for Dx {}
unsafe impl Sync for Dx {}
impl Dx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "“DATX8 General Configuration Register (DXnGCR)” on page 148."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DATX8 General Status Registers 0-2 (DXnGSR0-2)."]
    #[inline(always)]
    pub const fn gsr0(self) -> crate::common::Reg<regs::Gsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DATX8 General Status Registers 0-2 (DXnGSR0-2)."]
    #[inline(always)]
    pub const fn gsr1(self) -> crate::common::Reg<regs::Gsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn bdlr0(self) -> crate::common::Reg<regs::Bdlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn bdlr1(self) -> crate::common::Reg<regs::Bdlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn bdlr2(self) -> crate::common::Reg<regs::Bdlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn bdlr3(self) -> crate::common::Reg<regs::Bdlr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn bdlr4(self) -> crate::common::Reg<regs::Bdlr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn lcdlr0(self) -> crate::common::Reg<regs::Lcdlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn lcdlr1(self) -> crate::common::Reg<regs::Lcdlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[inline(always)]
    pub const fn lcdlr2(self) -> crate::common::Reg<regs::Lcdlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "“DATX8 Master Delay Line Register (DXnMDLR)” on page 157."]
    #[inline(always)]
    pub const fn mdlr(self) -> crate::common::Reg<regs::Mdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "“DATX8 General Timing Register (DXnGTR)” on page 159."]
    #[inline(always)]
    pub const fn gtr(self) -> crate::common::Reg<regs::Gtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "“DATX8 General Status Register 2 (DXnGSR2)” on page 152."]
    #[inline(always)]
    pub const fn gsr2(self) -> crate::common::Reg<regs::Gsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zq {
    ptr: *mut u8,
}
unsafe impl Send for Zq {}
unsafe impl Sync for Zq {}
impl Zq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Impedance Control Register 0-1 (ZQnCR0-1)."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Impedance Control Register 0-1 (ZQnCR0-1)."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Impedance Status Register 0-1 (ZQnSR0-1)."]
    #[inline(always)]
    pub const fn sr0(self) -> crate::common::Reg<regs::Sr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Impedance Status Register 0-1 (ZQnSR0-1)."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "“Anti-Aging Control Register (AACR)” on page 143."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aacr(pub u32);
    impl Aacr {
        #[doc = "Anti-Aging Toggle Rate: Defines the number of controller clock (ctl_clk) cycles after which the PUB will toggle the data going to DATX8 if the data channel between the controller/PUB and DATX8 has been idle for this long. The default value correspond to a toggling count of 4096 ctl_clk cycles. For a ctl_clk running at 533MHz the toggle rate will be approximately 7.68us. The default value may also be overridden by the macro DWC_AACR_AATR_DFLT."]
        #[inline(always)]
        pub const fn aatr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Anti-Aging Toggle Rate: Defines the number of controller clock (ctl_clk) cycles after which the PUB will toggle the data going to DATX8 if the data channel between the controller/PUB and DATX8 has been idle for this long. The default value correspond to a toggling count of 4096 ctl_clk cycles. For a ctl_clk running at 533MHz the toggle rate will be approximately 7.68us. The default value may also be overridden by the macro DWC_AACR_AATR_DFLT."]
        #[inline(always)]
        pub fn set_aatr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
        #[doc = "Anti-Aging Enable Control: Enables, if set, the automatic toggling of the data going to the DATX8 when the data channel from the controller/PUB to DATX8 is idle for programmable number of clock cycles."]
        #[inline(always)]
        pub const fn aaenc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Anti-Aging Enable Control: Enables, if set, the automatic toggling of the data going to the DATX8 when the data channel from the controller/PUB to DATX8 is idle for programmable number of clock cycles."]
        #[inline(always)]
        pub fn set_aaenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Anti-Aging PAD Output Enable Control: Enables, if set, anti-aging toggling on the pad output enable signal “ctl_oe_n” going into the DATX8s. This will increase power consumption for the anti-aging feature."]
        #[inline(always)]
        pub const fn aaoenc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Anti-Aging PAD Output Enable Control: Enables, if set, anti-aging toggling on the pad output enable signal “ctl_oe_n” going into the DATX8s. This will increase power consumption for the anti-aging feature."]
        #[inline(always)]
        pub fn set_aaoenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Aacr {
        #[inline(always)]
        fn default() -> Aacr {
            Aacr(0)
        }
    }
    #[doc = "“AC Bit Delay Line Register (ACBDLR)” on page 96."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acbdlr(pub u32);
    impl Acbdlr {
        #[doc = "CK0 Bit Delay: Delay select for the BDL on CK0."]
        #[inline(always)]
        pub const fn ck0bd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "CK0 Bit Delay: Delay select for the BDL on CK0."]
        #[inline(always)]
        pub fn set_ck0bd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "CK1 Bit Delay: Delay select for the BDL on CK1."]
        #[inline(always)]
        pub const fn ck1bd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "CK1 Bit Delay: Delay select for the BDL on CK1."]
        #[inline(always)]
        pub fn set_ck1bd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "CK2 Bit Delay: Delay select for the BDL on CK2."]
        #[inline(always)]
        pub const fn ck2bd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "CK2 Bit Delay: Delay select for the BDL on CK2."]
        #[inline(always)]
        pub fn set_ck2bd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "Address/Command Bit Delay: Delay select for the BDLs on address and command signals."]
        #[inline(always)]
        pub const fn acbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "Address/Command Bit Delay: Delay select for the BDLs on address and command signals."]
        #[inline(always)]
        pub fn set_acbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
    }
    impl Default for Acbdlr {
        #[inline(always)]
        fn default() -> Acbdlr {
            Acbdlr(0)
        }
    }
    #[doc = "“AC I/O Configuration Register (ACIOCR)” on page 97."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aciocr(pub u32);
    impl Aciocr {
        #[doc = "Address/Command I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for all address and command pins. This bit connects to bit \\[0\\]
of the IOM pin on the D3F I/Os, and for other I/O libraries, it connects to the IOM pin of the I/O."]
        #[inline(always)]
        pub const fn aciom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Address/Command I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for all address and command pins. This bit connects to bit \\[0\\]
of the IOM pin on the D3F I/Os, and for other I/O libraries, it connects to the IOM pin of the I/O."]
        #[inline(always)]
        pub fn set_aciom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/Command Output Enable: Enables, when set, the output driver on the I/O for all address and command pins."]
        #[inline(always)]
        pub const fn acoe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/Command Output Enable: Enables, when set, the output driver on the I/O for all address and command pins."]
        #[inline(always)]
        pub fn set_acoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Address/Command On-Die Termination: Enables, when set, the on-die termination on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub const fn acodt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Address/Command On-Die Termination: Enables, when set, the on-die termination on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub fn set_acodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AC Power Down Driver: Powers down, when set, the output driver on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub const fn acpdd1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AC Power Down Driver: Powers down, when set, the output driver on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub fn set_acpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AC Power Down Receiver: Powers down, when set, the input receiver on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub const fn acpdr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AC Power Down Receiver: Powers down, when set, the input receiver on the I/O for RAS#, CAS#, WE#, BA\\[2:0\\], and A\\[15:0\\]
pins."]
        #[inline(always)]
        pub fn set_acpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CK On-Die Termination: Enables, when set, the on-die termination on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub const fn ckodt(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "CK On-Die Termination: Enables, when set, the on-die termination on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub fn set_ckodt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "CK Power Down Driver: Powers down, when set, the output driver on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub const fn ckpdd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "CK Power Down Driver: Powers down, when set, the output driver on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub fn set_ckpdd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "CK Power Down Receiver: Powers down, when set, the input receiver on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub const fn ckpdr(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "CK Power Down Receiver: Powers down, when set, the input receiver on the I/O for CK\\[0\\], CK\\[1\\], and CK\\[2\\]
pins, respectively."]
        #[inline(always)]
        pub fn set_ckpdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[doc = "Rank On-Die Termination: Enables, when set, the on-die termination on the I/O for CKE\\[3:0\\], ODT\\[3:0\\], and CS#\\[3:0\\]
pins. RANKODT\\[0\\]
controls the on-die termination for CKE\\[0\\], ODT\\[0\\], and CS#\\[0\\], RANKODT\\[1\\]
controls the on-die termination for CKE\\[1\\], ODT\\[1\\], and CS#\\[1\\], and so on."]
        #[inline(always)]
        pub const fn rankodt(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "Rank On-Die Termination: Enables, when set, the on-die termination on the I/O for CKE\\[3:0\\], ODT\\[3:0\\], and CS#\\[3:0\\]
pins. RANKODT\\[0\\]
controls the on-die termination for CKE\\[0\\], ODT\\[0\\], and CS#\\[0\\], RANKODT\\[1\\]
controls the on-die termination for CKE\\[1\\], ODT\\[1\\], and CS#\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_rankodt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
        #[doc = "CS# Power Down Driver: Powers down, when set, the output driver on the I/O for CS#\\[3:0\\]
pins. CSPDD\\[0\\]
controls the power down for CS#\\[0\\], CSPDD\\[1\\]
controls the power down for CS#\\[1\\], and so on. CKE and ODT driver power down is controlled by DSGCR register."]
        #[inline(always)]
        pub const fn cspdd1(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "CS# Power Down Driver: Powers down, when set, the output driver on the I/O for CS#\\[3:0\\]
pins. CSPDD\\[0\\]
controls the power down for CS#\\[0\\], CSPDD\\[1\\]
controls the power down for CS#\\[1\\], and so on. CKE and ODT driver power down is controlled by DSGCR register."]
        #[inline(always)]
        pub fn set_cspdd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "Rank Power Down Receiver: Powers down, when set, the input receiver on the I/O CKE\\[3:0\\], ODT\\[3:0\\], and CS#\\[3:0\\]
pins. RANKPDR\\[0\\]
controls the power down for CKE\\[0\\], ODT\\[0\\], and CS#\\[0\\], RANKPDR\\[1\\]
controls the power down for CKE\\[1\\], ODT\\[1\\], and CS#\\[1\\], and so on."]
        #[inline(always)]
        pub const fn rankpdr(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "Rank Power Down Receiver: Powers down, when set, the input receiver on the I/O CKE\\[3:0\\], ODT\\[3:0\\], and CS#\\[3:0\\]
pins. RANKPDR\\[0\\]
controls the power down for CKE\\[0\\], ODT\\[0\\], and CS#\\[0\\], RANKPDR\\[1\\]
controls the power down for CKE\\[1\\], ODT\\[1\\], and CS#\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_rankpdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "SDRAM Reset On-Die Termination: Enables, when set, the on-die termination on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub const fn rstodt(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM Reset On-Die Termination: Enables, when set, the on-die termination on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub fn set_rstodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SDRAM Reset Power Down Driver: Powers down, when set, the output driver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub const fn rstpdd1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM Reset Power Down Driver: Powers down, when set, the output driver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub fn set_rstpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDRAM Reset Power Down Receiver: Powers down, when set, the input receiver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub const fn rstpdr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM Reset Power Down Receiver: Powers down, when set, the input receiver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub fn set_rstpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SDRAM Reset I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for SDRAM Reset."]
        #[inline(always)]
        pub const fn rstiom(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM Reset I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for SDRAM Reset."]
        #[inline(always)]
        pub fn set_rstiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Address/Command Slew Rate (D3F I/O Only): Selects slew rate of the I/O for all address and command pins."]
        #[inline(always)]
        pub const fn acsr(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Address/Command Slew Rate (D3F I/O Only): Selects slew rate of the I/O for all address and command pins."]
        #[inline(always)]
        pub fn set_acsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Aciocr {
        #[inline(always)]
        fn default() -> Aciocr {
            Aciocr(0)
        }
    }
    #[doc = "“AC Master Delay Line Register (ACMDLR)” on page 96."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acmdlr(pub u32);
    impl Acmdlr {
        #[doc = "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation."]
        #[inline(always)]
        pub const fn iprd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation."]
        #[inline(always)]
        pub fn set_iprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so."]
        #[inline(always)]
        pub const fn tprd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so."]
        #[inline(always)]
        pub fn set_tprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "MDL Delay: Delay select for the LCDL for the Master Delay Line."]
        #[inline(always)]
        pub const fn mdld(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "MDL Delay: Delay select for the LCDL for the Master Delay Line."]
        #[inline(always)]
        pub fn set_mdld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Acmdlr {
        #[inline(always)]
        fn default() -> Acmdlr {
            Acmdlr(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdlr0(pub u32);
    impl Bdlr0 {
        #[doc = "DQ0 Write Bit Delay: Delay select for the BDL on DQ0 write path."]
        #[inline(always)]
        pub const fn dq0wbd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ0 Write Bit Delay: Delay select for the BDL on DQ0 write path."]
        #[inline(always)]
        pub fn set_dq0wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DQ1 Write Bit Delay: Delay select for the BDL on DQ1 write path."]
        #[inline(always)]
        pub const fn dq1wbd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ1 Write Bit Delay: Delay select for the BDL on DQ1 write path."]
        #[inline(always)]
        pub fn set_dq1wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "DQ2 Write Bit Delay: Delay select for the BDL on DQ2 write path."]
        #[inline(always)]
        pub const fn dq2wbd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ2 Write Bit Delay: Delay select for the BDL on DQ2 write path."]
        #[inline(always)]
        pub fn set_dq2wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "DQ3 Write Bit Delay: Delay select for the BDL on DQ3 write path."]
        #[inline(always)]
        pub const fn dq3wbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ3 Write Bit Delay: Delay select for the BDL on DQ3 write path."]
        #[inline(always)]
        pub fn set_dq3wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "DQ4 Write Bit Delay: Delay select for the BDL on DQ4 write path."]
        #[inline(always)]
        pub const fn dq4wbd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ4 Write Bit Delay: Delay select for the BDL on DQ4 write path."]
        #[inline(always)]
        pub fn set_dq4wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Bdlr0 {
        #[inline(always)]
        fn default() -> Bdlr0 {
            Bdlr0(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdlr1(pub u32);
    impl Bdlr1 {
        #[doc = "DQ5 Write Bit Delay: Delay select for the BDL on DQ5 write path."]
        #[inline(always)]
        pub const fn dq5wbd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ5 Write Bit Delay: Delay select for the BDL on DQ5 write path."]
        #[inline(always)]
        pub fn set_dq5wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DQ6 Write Bit Delay: Delay select for the BDL on DQ6 write path."]
        #[inline(always)]
        pub const fn dq6wbd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ6 Write Bit Delay: Delay select for the BDL on DQ6 write path."]
        #[inline(always)]
        pub fn set_dq6wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "DQ7 Write Bit Delay: Delay select for the BDL on DQ7 write path."]
        #[inline(always)]
        pub const fn dq7wbd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ7 Write Bit Delay: Delay select for the BDL on DQ7 write path."]
        #[inline(always)]
        pub fn set_dq7wbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "DM Write Bit Delay: Delay select for the BDL on DM write path."]
        #[inline(always)]
        pub const fn dmwbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "DM Write Bit Delay: Delay select for the BDL on DM write path."]
        #[inline(always)]
        pub fn set_dmwbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "DQS Write Bit Delay: Delay select for the BDL on DQS write path."]
        #[inline(always)]
        pub const fn dswbd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "DQS Write Bit Delay: Delay select for the BDL on DQS write path."]
        #[inline(always)]
        pub fn set_dswbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Bdlr1 {
        #[inline(always)]
        fn default() -> Bdlr1 {
            Bdlr1(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdlr2(pub u32);
    impl Bdlr2 {
        #[doc = "DQS Output Enable Bit Delay: Delay select for the BDL on DQS output enable path."]
        #[inline(always)]
        pub const fn dsoebd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "DQS Output Enable Bit Delay: Delay select for the BDL on DQS output enable path."]
        #[inline(always)]
        pub fn set_dsoebd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DQ Output Enable Bit Delay: Delay select for the BDL on DQ/DM output enable path."]
        #[inline(always)]
        pub const fn dqoebd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ Output Enable Bit Delay: Delay select for the BDL on DQ/DM output enable path."]
        #[inline(always)]
        pub fn set_dqoebd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "DQS Read Bit Delay: Delay select for the BDL on DQS read path."]
        #[inline(always)]
        pub const fn dsrbd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "DQS Read Bit Delay: Delay select for the BDL on DQS read path."]
        #[inline(always)]
        pub fn set_dsrbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "DQSN Read Bit Delay (Type B/B1 PHY Only): Delay select for the BDL on DQSN read path."]
        #[inline(always)]
        pub const fn dsnrbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "DQSN Read Bit Delay (Type B/B1 PHY Only): Delay select for the BDL on DQSN read path."]
        #[inline(always)]
        pub fn set_dsnrbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
    }
    impl Default for Bdlr2 {
        #[inline(always)]
        fn default() -> Bdlr2 {
            Bdlr2(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdlr3(pub u32);
    impl Bdlr3 {
        #[doc = "DQ0 Read Bit Delay: Delay select for the BDL on DQ0 read path."]
        #[inline(always)]
        pub const fn dq0rbd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ0 Read Bit Delay: Delay select for the BDL on DQ0 read path."]
        #[inline(always)]
        pub fn set_dq0rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DQ1 Read Bit Delay: Delay select for the BDL on DQ1 read path."]
        #[inline(always)]
        pub const fn dq1rbd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ1 Read Bit Delay: Delay select for the BDL on DQ1 read path."]
        #[inline(always)]
        pub fn set_dq1rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "DQ2 Read Bit Delay: Delay select for the BDL on DQ2 read path."]
        #[inline(always)]
        pub const fn dq2rbd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ2 Read Bit Delay: Delay select for the BDL on DQ2 read path."]
        #[inline(always)]
        pub fn set_dq2rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "DQ3 Read Bit Delay: Delay select for the BDL on DQ3 read path."]
        #[inline(always)]
        pub const fn dq3rbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ3 Read Bit Delay: Delay select for the BDL on DQ3 read path."]
        #[inline(always)]
        pub fn set_dq3rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "DQ4 Read Bit Delay: Delay select for the BDL on DQ4 read path."]
        #[inline(always)]
        pub const fn dq4rbd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ4 Read Bit Delay: Delay select for the BDL on DQ4 read path."]
        #[inline(always)]
        pub fn set_dq4rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Bdlr3 {
        #[inline(always)]
        fn default() -> Bdlr3 {
            Bdlr3(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdlr4(pub u32);
    impl Bdlr4 {
        #[doc = "DQ5 Read Bit Delay: Delay select for the BDL on DQ5 read path."]
        #[inline(always)]
        pub const fn dq5rbd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ5 Read Bit Delay: Delay select for the BDL on DQ5 read path."]
        #[inline(always)]
        pub fn set_dq5rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DQ6 Read Bit Delay: Delay select for the BDL on DQ6 read path."]
        #[inline(always)]
        pub const fn dq6rbd(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ6 Read Bit Delay: Delay select for the BDL on DQ6 read path."]
        #[inline(always)]
        pub fn set_dq6rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "DQ7 Read Bit Delay: Delay select for the BDL on DQ7 read path."]
        #[inline(always)]
        pub const fn dq7rbd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "DQ7 Read Bit Delay: Delay select for the BDL on DQ7 read path."]
        #[inline(always)]
        pub fn set_dq7rbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "DM Read Bit Delay: Delay select for the BDL on DM read path."]
        #[inline(always)]
        pub const fn dmrbd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "DM Read Bit Delay: Delay select for the BDL on DM read path."]
        #[inline(always)]
        pub fn set_dmrbd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
    }
    impl Default for Bdlr4 {
        #[inline(always)]
        fn default() -> Bdlr4 {
            Bdlr4(0)
        }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistar0(pub u32);
    impl Bistar0 {
        #[doc = "BIST Column Address: Selects the SDRAM column address to be used during BIST. The lower bits of this address must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2."]
        #[inline(always)]
        pub const fn bcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "BIST Column Address: Selects the SDRAM column address to be used during BIST. The lower bits of this address must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2."]
        #[inline(always)]
        pub fn set_bcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "BIST Row Address: Selects the SDRAM row address to be used during BIST."]
        #[inline(always)]
        pub const fn brow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "BIST Row Address: Selects the SDRAM row address to be used during BIST."]
        #[inline(always)]
        pub fn set_brow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "BIST Bank Address: Selects the SDRAM bank address to be used during BIST."]
        #[inline(always)]
        pub const fn bbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "BIST Bank Address: Selects the SDRAM bank address to be used during BIST."]
        #[inline(always)]
        pub fn set_bbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Bistar0 {
        #[inline(always)]
        fn default() -> Bistar0 {
            Bistar0(0)
        }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistar1(pub u32);
    impl Bistar1 {
        #[doc = "BIST Rank: Selects the SDRAM rank to be used during BIST. Valid values range from 0 to maximum ranks minus 1."]
        #[inline(always)]
        pub const fn brank(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "BIST Rank: Selects the SDRAM rank to be used during BIST. Valid values range from 0 to maximum ranks minus 1."]
        #[inline(always)]
        pub fn set_brank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "BIST Maximum Rank: Specifies the maximum SDRAM rank to be used during BIST. The default value is set to maximum ranks minus 1. Example default shown here is for a 4-rank system."]
        #[inline(always)]
        pub const fn bmrank(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "BIST Maximum Rank: Specifies the maximum SDRAM rank to be used during BIST. The default value is set to maximum ranks minus 1. Example default shown here is for a 4-rank system."]
        #[inline(always)]
        pub fn set_bmrank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "BIST Address Increment: Selects the value by which the SDRAM address is incremented for each write/read access. This value must be at the beginning of a burst boundary, i.e. the lower bits must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2."]
        #[inline(always)]
        pub const fn bainc(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "BIST Address Increment: Selects the value by which the SDRAM address is incremented for each write/read access. This value must be at the beginning of a burst boundary, i.e. the lower bits must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2."]
        #[inline(always)]
        pub fn set_bainc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for Bistar1 {
        #[inline(always)]
        fn default() -> Bistar1 {
            Bistar1(0)
        }
    }
    #[doc = "BIST Address Register 0-2 (BISTAR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistar2(pub u32);
    impl Bistar2 {
        #[doc = "BIST Maximum Column Address: Specifies the maximum SDRAM column address to be used during BIST before the address increments to the next row."]
        #[inline(always)]
        pub const fn bmcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "BIST Maximum Column Address: Specifies the maximum SDRAM column address to be used during BIST before the address increments to the next row."]
        #[inline(always)]
        pub fn set_bmcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "BIST Maximum Row Address: Specifies the maximum SDRAM row address to be used during BIST before the address increments to the next bank."]
        #[inline(always)]
        pub const fn bmrow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "BIST Maximum Row Address: Specifies the maximum SDRAM row address to be used during BIST before the address increments to the next bank."]
        #[inline(always)]
        pub fn set_bmrow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "BIST Maximum Bank Address: Specifies the maximum SDRAM bank address to be used during BIST before the address increments to the next rank."]
        #[inline(always)]
        pub const fn bmbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "BIST Maximum Bank Address: Specifies the maximum SDRAM bank address to be used during BIST before the address increments to the next rank."]
        #[inline(always)]
        pub fn set_bmbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Bistar2 {
        #[inline(always)]
        fn default() -> Bistar2 {
            Bistar2(0)
        }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistber0(pub u32);
    impl Bistber0 {
        #[doc = "Address Bit Error: Each group of two bits indicate the bit error count on each of the."]
        #[inline(always)]
        pub const fn aber(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address Bit Error: Each group of two bits indicate the bit error count on each of the."]
        #[inline(always)]
        pub fn set_aber(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistber0 {
        #[inline(always)]
        fn default() -> Bistber0 {
            Bistber0(0)
        }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistber1(pub u32);
    impl Bistber1 {
        #[doc = "Bank Address Bit Error: Each group of two bits indicate the bit error count on each of the up to 3 bank address bits. \\[1:0\\]
is the error count for BA\\[0\\], \\[3:2\\]
for BA\\[1\\], and so on."]
        #[inline(always)]
        pub const fn baber(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Bank Address Bit Error: Each group of two bits indicate the bit error count on each of the up to 3 bank address bits. \\[1:0\\]
is the error count for BA\\[0\\], \\[3:2\\]
for BA\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_baber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "WE# Bit Error: Indicates the number of bit errors on WE#."]
        #[inline(always)]
        pub const fn weber(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "WE# Bit Error: Indicates the number of bit errors on WE#."]
        #[inline(always)]
        pub fn set_weber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "CKE Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CKE bits. \\[1:0\\]
is the error count for CKE\\[0\\], \\[3:2\\]
for CKE\\[1\\], and so on."]
        #[inline(always)]
        pub const fn ckeber(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "CKE Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CKE bits. \\[1:0\\]
is the error count for CKE\\[0\\], \\[3:2\\]
for CKE\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_ckeber(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "CS# Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CS# bits. \\[1:0\\]
is the error count for CS#\\[0\\], \\[3:2\\]
for CS#\\[1\\], and so on."]
        #[inline(always)]
        pub const fn csber(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "CS# Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CS# bits. \\[1:0\\]
is the error count for CS#\\[0\\], \\[3:2\\]
for CS#\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_csber(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "ODT Bit Error: Each group of two bits indicates the bit error count on each of the up to 4 ODT bits. \\[1:0\\]
is the error count for ODT\\[0\\], \\[3:2\\]
for ODT\\[1\\], and so on."]
        #[inline(always)]
        pub const fn odtber(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "ODT Bit Error: Each group of two bits indicates the bit error count on each of the up to 4 ODT bits. \\[1:0\\]
is the error count for ODT\\[0\\], \\[3:2\\]
for ODT\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_odtber(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Bistber1 {
        #[inline(always)]
        fn default() -> Bistber1 {
            Bistber1(0)
        }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistber2(pub u32);
    impl Bistber2 {
        #[doc = "Data Bit Error: The error count for even DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ\\[0\\], the second for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub const fn dqber0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Bit Error: The error count for even DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ\\[0\\], the second for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_dqber0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistber2 {
        #[inline(always)]
        fn default() -> Bistber2 {
            Bistber2(0)
        }
    }
    #[doc = "BIST Bit Error Register 0-3 (BISTBER0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistber3(pub u32);
    impl Bistber3 {
        #[doc = "Data Bit Error: The error count for odd DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ\\[0\\], the second for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub const fn dqber1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Bit Error: The error count for odd DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ\\[0\\], the second for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_dqber1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistber3 {
        #[inline(always)]
        fn default() -> Bistber3 {
            Bistber3(0)
        }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistfwr0(pub u32);
    impl Bistfwr0 {
        #[doc = "Bit status during a word error for each of the up to 16 address bits."]
        #[inline(always)]
        pub const fn awebs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit status during a word error for each of the up to 16 address bits."]
        #[inline(always)]
        pub fn set_awebs(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Bit status during a word error for each of the up to 3 bank address bits."]
        #[inline(always)]
        pub const fn bawebs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Bit status during a word error for each of the up to 3 bank address bits."]
        #[inline(always)]
        pub fn set_bawebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Bit status during a word error for the WE#."]
        #[inline(always)]
        pub const fn wewebs(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bit status during a word error for the WE#."]
        #[inline(always)]
        pub fn set_wewebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bit status during a word error for each of the up to 4 CKE bits."]
        #[inline(always)]
        pub const fn ckewebs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit status during a word error for each of the up to 4 CKE bits."]
        #[inline(always)]
        pub fn set_ckewebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Bit status during a word error for each of the up to 4 CS# bits."]
        #[inline(always)]
        pub const fn cswebs(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit status during a word error for each of the up to 4 CS# bits."]
        #[inline(always)]
        pub fn set_cswebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Bit status during a word error for each of the up to 4 ODT bits."]
        #[inline(always)]
        pub const fn odtwebs(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit status during a word error for each of the up to 4 ODT bits."]
        #[inline(always)]
        pub fn set_odtwebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Bistfwr0 {
        #[inline(always)]
        fn default() -> Bistfwr0 {
            Bistfwr0(0)
        }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistfwr1(pub u32);
    impl Bistfwr1 {
        #[doc = "Bit status during a word error for the RAS."]
        #[inline(always)]
        pub const fn raswebs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bit status during a word error for the RAS."]
        #[inline(always)]
        pub fn set_raswebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Bit status during a word error for the CAS."]
        #[inline(always)]
        pub const fn caswebs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bit status during a word error for the CAS."]
        #[inline(always)]
        pub fn set_caswebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bit status during a word error for the PAR_IN. Only for DIMM parity support."]
        #[inline(always)]
        pub const fn parwebs(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bit status during a word error for the PAR_IN. Only for DIMM parity support."]
        #[inline(always)]
        pub fn set_parwebs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Bit status during a word error for the data mask (DM) bit. DMWEBS \\[0\\]
is for the first DM beat, DMWEBS \\[1\\]
is for the second DM beat, and so on."]
        #[inline(always)]
        pub const fn dmwebs(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit status during a word error for the data mask (DM) bit. DMWEBS \\[0\\]
is for the first DM beat, DMWEBS \\[1\\]
is for the second DM beat, and so on."]
        #[inline(always)]
        pub fn set_dmwebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Bistfwr1 {
        #[inline(always)]
        fn default() -> Bistfwr1 {
            Bistfwr1(0)
        }
    }
    #[doc = "BIST Fail Word Register 0-2 (BISTFWR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistfwr2(pub u32);
    impl Bistfwr2 {
        #[doc = "Bit status during a word error for each of the 8 data (DQ) bits. The first 8 bits indicate the status of the first data beat (i.e. the status of the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 8 bits indicate the status of the second data beat (i.e. the status of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS), and so on. For each of the 8-bit group, the first bit is for DQ\\[0\\], the second bit is for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub const fn dqwebs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bit status during a word error for each of the 8 data (DQ) bits. The first 8 bits indicate the status of the first data beat (i.e. the status of the data driven out on DQ\\[7:0\\]
on the rising edge of DQS). The second 8 bits indicate the status of the second data beat (i.e. the status of the data driven out on DQ\\[7:0\\]
on the falling edge of DQS), and so on. For each of the 8-bit group, the first bit is for DQ\\[0\\], the second bit is for DQ\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_dqwebs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistfwr2 {
        #[inline(always)]
        fn default() -> Bistfwr2 {
            Bistfwr2(0)
        }
    }
    #[doc = "“BIST General Status Register (BISTGSR)” on page 139."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistgsr(pub u32);
    impl Bistgsr {
        #[doc = "BIST Done: Indicates, if set, that the BIST has finished executing. This bit is reset to zero when BIST is triggered."]
        #[inline(always)]
        pub const fn bdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Done: Indicates, if set, that the BIST has finished executing. This bit is reset to zero when BIST is triggered."]
        #[inline(always)]
        pub fn set_bdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BIST Address/Command Error: indicates, if set, that there is a data comparison error in the address/command lane."]
        #[inline(always)]
        pub const fn bacerr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Address/Command Error: indicates, if set, that there is a data comparison error in the address/command lane."]
        #[inline(always)]
        pub fn set_bacerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BIST Data Error: indicates, if set, that there is a data comparison error in the byte lane."]
        #[inline(always)]
        pub const fn bdxerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Data Error: indicates, if set, that there is a data comparison error in the byte lane."]
        #[inline(always)]
        pub fn set_bdxerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PAR_IN Bit Error (DIMM Only): Indicates the number of bit errors on PAR_IN."]
        #[inline(always)]
        pub const fn parber(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "PAR_IN Bit Error (DIMM Only): Indicates the number of bit errors on PAR_IN."]
        #[inline(always)]
        pub fn set_parber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "DM Bit Error: Indicates the number of bit errors on data mask (DM) bit. DMBER\\[1:0\\]
are for even DQS cycles first DM beat, and DMBER\\[3:2\\]
are for even DQS cycles second DM beat. Similarly, DMBER\\[5:4\\]
are for odd DQS cycles first DM beat, and DMBER\\[7:6\\]
are for odd DQS cycles second DM beat."]
        #[inline(always)]
        pub const fn dmber(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "DM Bit Error: Indicates the number of bit errors on data mask (DM) bit. DMBER\\[1:0\\]
are for even DQS cycles first DM beat, and DMBER\\[3:2\\]
are for even DQS cycles second DM beat. Similarly, DMBER\\[5:4\\]
are for odd DQS cycles first DM beat, and DMBER\\[7:6\\]
are for odd DQS cycles second DM beat."]
        #[inline(always)]
        pub fn set_dmber(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "RAS Bit Error: Indicates the number of bit errors on RAS."]
        #[inline(always)]
        pub const fn rasber(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "RAS Bit Error: Indicates the number of bit errors on RAS."]
        #[inline(always)]
        pub fn set_rasber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "CAS Bit Error: Indicates the number of bit errors on CAS."]
        #[inline(always)]
        pub const fn casber(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "CAS Bit Error: Indicates the number of bit errors on CAS."]
        #[inline(always)]
        pub fn set_casber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Bistgsr {
        #[inline(always)]
        fn default() -> Bistgsr {
            Bistgsr(0)
        }
    }
    #[doc = "“BIST LFSR Seed Register (BISTLSR)” on page 137."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistlsr(pub u32);
    impl Bistlsr {
        #[doc = "LFSR seed for pseudo-random BIST patterns."]
        #[inline(always)]
        pub const fn seed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "LFSR seed for pseudo-random BIST patterns."]
        #[inline(always)]
        pub fn set_seed(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistlsr {
        #[inline(always)]
        fn default() -> Bistlsr {
            Bistlsr(0)
        }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistmskr0(pub u32);
    impl Bistmskr0 {
        #[doc = "Mask bit for each of the up to 16 address bits."]
        #[inline(always)]
        pub const fn amsk(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Mask bit for each of the up to 16 address bits."]
        #[inline(always)]
        pub fn set_amsk(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask bit for each of the up to 3 bank address bits."]
        #[inline(always)]
        pub const fn bamsk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Mask bit for each of the up to 3 bank address bits."]
        #[inline(always)]
        pub fn set_bamsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Mask bit for the WE#."]
        #[inline(always)]
        pub const fn wemsk(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Mask bit for the WE#."]
        #[inline(always)]
        pub fn set_wemsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Mask bit for each of the up to 4 CKE bits."]
        #[inline(always)]
        pub const fn ckemsk(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask bit for each of the up to 4 CKE bits."]
        #[inline(always)]
        pub fn set_ckemsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Mask bit for each of the up to 4 CS# bits."]
        #[inline(always)]
        pub const fn csmsk(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask bit for each of the up to 4 CS# bits."]
        #[inline(always)]
        pub fn set_csmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Mask bit for each of the up to 4 ODT bits."]
        #[inline(always)]
        pub const fn odtmsk(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask bit for each of the up to 4 ODT bits."]
        #[inline(always)]
        pub fn set_odtmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Bistmskr0 {
        #[inline(always)]
        fn default() -> Bistmskr0 {
            Bistmskr0(0)
        }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistmskr1(pub u32);
    impl Bistmskr1 {
        #[doc = "Mask bit for the RAS."]
        #[inline(always)]
        pub const fn rasmsk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mask bit for the RAS."]
        #[inline(always)]
        pub fn set_rasmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mask bit for the CAS."]
        #[inline(always)]
        pub const fn casmsk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mask bit for the CAS."]
        #[inline(always)]
        pub fn set_casmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mask bit for the PAR_IN. Only for DIMM parity support and only if the design is compiled for less than 3 ranks."]
        #[inline(always)]
        pub const fn parmsk(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Mask bit for the PAR_IN. Only for DIMM parity support and only if the design is compiled for less than 3 ranks."]
        #[inline(always)]
        pub fn set_parmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Mask bit for the data mask (DM) bit."]
        #[inline(always)]
        pub const fn dmmsk(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask bit for the data mask (DM) bit."]
        #[inline(always)]
        pub fn set_dmmsk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Bistmskr1 {
        #[inline(always)]
        fn default() -> Bistmskr1 {
            Bistmskr1(0)
        }
    }
    #[doc = "BIST Mask Register 0-2 (BISTMSKR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistmskr2(pub u32);
    impl Bistmskr2 {
        #[doc = "Mask bit for each of the 8 data (DQ) bits."]
        #[inline(always)]
        pub const fn dqmsk(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mask bit for each of the 8 data (DQ) bits."]
        #[inline(always)]
        pub fn set_dqmsk(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bistmskr2 {
        #[inline(always)]
        fn default() -> Bistmskr2 {
            Bistmskr2(0)
        }
    }
    #[doc = "“BIST Run Register (BISTRR)” on page 133."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistrr(pub u32);
    impl Bistrr {
        #[doc = "BIST Instruction: Selects the BIST instruction to be executed: Valid values are: 000 = NOP: No operation 001 = Run: Triggers the running of the BIST. 010 = Stop: Stops the running of the BIST. 011 = Reset: Resets all BIST run-time registers, such as error counters. 100 – 111 Reserved."]
        #[inline(always)]
        pub const fn binst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "BIST Instruction: Selects the BIST instruction to be executed: Valid values are: 000 = NOP: No operation 001 = Run: Triggers the running of the BIST. 010 = Stop: Stops the running of the BIST. 011 = Reset: Resets all BIST run-time registers, such as error counters. 100 – 111 Reserved."]
        #[inline(always)]
        pub fn set_binst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "BIST Mode: Selects the mode in which BIST is run. Valid values are: 0 = Loopback mode: Address, commands and data loop back at the PHY I/Os. 1 = DRAM mode: Address, commands and data go to DRAM for normal memory accesses."]
        #[inline(always)]
        pub const fn bmode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Mode: Selects the mode in which BIST is run. Valid values are: 0 = Loopback mode: Address, commands and data loop back at the PHY I/Os. 1 = DRAM mode: Address, commands and data go to DRAM for normal memory accesses."]
        #[inline(always)]
        pub fn set_bmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BIST Infinite Run: Specifies, if set, that the BIST should be run indefinitely until when it is either stopped or a failure has been encountered. Otherwise BIST is run until number of BIST words specified in the BISTWCR register has been generated."]
        #[inline(always)]
        pub const fn binf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Infinite Run: Specifies, if set, that the BIST should be run indefinitely until when it is either stopped or a failure has been encountered. Otherwise BIST is run until number of BIST words specified in the BISTWCR register has been generated."]
        #[inline(always)]
        pub fn set_binf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if BSONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if BSONF is set."]
        #[inline(always)]
        pub const fn nfail(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[doc = "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if BSONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if BSONF is set."]
        #[inline(always)]
        pub fn set_nfail(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
        #[doc = "BIST Stop On Nth Fail: Specifies, if set, that the BIST should stop when an nth data word or address/command comparison error has been encountered."]
        #[inline(always)]
        pub const fn bsonf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Stop On Nth Fail: Specifies, if set, that the BIST should stop when an nth data word or address/command comparison error has been encountered."]
        #[inline(always)]
        pub fn set_bsonf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "BIST DATX8 Enable: Enables the running of BIST on the data byte lane PHYs. This bit is exclusive with BACEN, i.e. both cannot be set to ‘1’ at the same time."]
        #[inline(always)]
        pub const fn bdxen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BIST DATX8 Enable: Enables the running of BIST on the data byte lane PHYs. This bit is exclusive with BACEN, i.e. both cannot be set to ‘1’ at the same time."]
        #[inline(always)]
        pub fn set_bdxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "BIST AC Enable: Enables the running of BIST on the address/command lane PHY. This bit is exclusive with BDXEN, i.e. both cannot be set to ‘1’ at the same time."]
        #[inline(always)]
        pub const fn bacen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BIST AC Enable: Enables the running of BIST on the address/command lane PHY. This bit is exclusive with BDXEN, i.e. both cannot be set to ‘1’ at the same time."]
        #[inline(always)]
        pub fn set_bacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "BIST Data Mask Enable: Enables, if set, that the data mask BIST should be included in the BIST run, i.e. data pattern generated and loopback data compared. This is valid only for loopback mode."]
        #[inline(always)]
        pub const fn bdmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BIST Data Mask Enable: Enables, if set, that the data mask BIST should be included in the BIST run, i.e. data pattern generated and loopback data compared. This is valid only for loopback mode."]
        #[inline(always)]
        pub fn set_bdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BIST Data Pattern: Selects the data pattern used during BIST. Valid values are: 00 = Walking 0 01 = Walking 1 10 = LFSR-based pseudo-random 11 = User programmable (Not valid for AC loopback)."]
        #[inline(always)]
        pub const fn bdpat(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "BIST Data Pattern: Selects the data pattern used during BIST. Valid values are: 00 = Walking 0 01 = Walking 1 10 = LFSR-based pseudo-random 11 = User programmable (Not valid for AC loopback)."]
        #[inline(always)]
        pub fn set_bdpat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "BIST DATX8 Select: Select the byte lane for comparison of loopback/read data. Valid values are 0 to 8."]
        #[inline(always)]
        pub const fn bdxsel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "BIST DATX8 Select: Select the byte lane for comparison of loopback/read data. Valid values are 0 to 8."]
        #[inline(always)]
        pub fn set_bdxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "BIST CK Select: Selects the CK that should be used to register the AC loopback signals from the I/Os. Valid values are: 00 = CK\\[0\\]
01 = CK\\[1\\]
10 = CK\\[2\\]
11 = Reserved."]
        #[inline(always)]
        pub const fn bcksel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x03;
            val as u8
        }
        #[doc = "BIST CK Select: Selects the CK that should be used to register the AC loopback signals from the I/Os. Valid values are: 00 = CK\\[0\\]
01 = CK\\[1\\]
10 = CK\\[2\\]
11 = Reserved."]
        #[inline(always)]
        pub fn set_bcksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
        }
        #[doc = "BIST Clock Cycle Select: Selects the clock numbers on which the AC loopback data is written into the FIFO. Data is written into the loopback FIFO once every four clock cycles. Valid values are: 00 = Clock cycle 0, 4, 8, 12, etc. 01 = Clock cycle 1, 5, 9, 13, etc. 10 = Clock cycle 2, 6, 10, 14, etc. 11 = Clock cycle 3, 7, 11, 15, etc."]
        #[inline(always)]
        pub const fn bccsel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "BIST Clock Cycle Select: Selects the clock numbers on which the AC loopback data is written into the FIFO. Data is written into the loopback FIFO once every four clock cycles. Valid values are: 00 = Clock cycle 0, 4, 8, 12, etc. 01 = Clock cycle 1, 5, 9, 13, etc. 10 = Clock cycle 2, 6, 10, 14, etc. 11 = Clock cycle 3, 7, 11, 15, etc."]
        #[inline(always)]
        pub fn set_bccsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for Bistrr {
        #[inline(always)]
        fn default() -> Bistrr {
            Bistrr(0)
        }
    }
    #[doc = "“BIST User Data Pattern Register (BISTUDPR)” on page 138."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistudpr(pub u32);
    impl Bistudpr {
        #[doc = "BIST User Data Pattern 0: Data to be applied on even DQ pins during BIST."]
        #[inline(always)]
        pub const fn budp0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "BIST User Data Pattern 0: Data to be applied on even DQ pins during BIST."]
        #[inline(always)]
        pub fn set_budp0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "BIST User Data Pattern 1: Data to be applied on odd DQ pins during BIST."]
        #[inline(always)]
        pub const fn budp1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "BIST User Data Pattern 1: Data to be applied on odd DQ pins during BIST."]
        #[inline(always)]
        pub fn set_budp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bistudpr {
        #[inline(always)]
        fn default() -> Bistudpr {
            Bistudpr(0)
        }
    }
    #[doc = "“BIST Word Count Register (BISTWCR)” on page 136."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistwcr(pub u32);
    impl Bistwcr {
        #[doc = "BIST Word Count: Indicates the number of words to generate during BIST. This must be a multiple of DRAM burst length (BL) divided by 2, e.g. for BL=8, valid values are 4, 8, 12, 16, and so on."]
        #[inline(always)]
        pub const fn bwcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "BIST Word Count: Indicates the number of words to generate during BIST. This must be a multiple of DRAM burst length (BL) divided by 2, e.g. for BL=8, valid values are 4, 8, 12, 16, and so on."]
        #[inline(always)]
        pub fn set_bwcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Bistwcr {
        #[inline(always)]
        fn default() -> Bistwcr {
            Bistwcr(0)
        }
    }
    #[doc = "“BIST Word Count Status Register (BISTWCSR)” on page 141."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistwcsr(pub u32);
    impl Bistwcsr {
        #[doc = "Address/Command Word Count: Indicates the number of words received from the address/command lane."]
        #[inline(always)]
        pub const fn acwcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Address/Command Word Count: Indicates the number of words received from the address/command lane."]
        #[inline(always)]
        pub fn set_acwcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Byte Word Count: Indicates the number of words received from the byte lane."]
        #[inline(always)]
        pub const fn dxwcnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte Word Count: Indicates the number of words received from the byte lane."]
        #[inline(always)]
        pub fn set_dxwcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bistwcsr {
        #[inline(always)]
        fn default() -> Bistwcsr {
            Bistwcsr(0)
        }
    }
    #[doc = "“BIST Word Error Register (BISTWER)” on page 139."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bistwer(pub u32);
    impl Bistwer {
        #[doc = "Address/Command Word Error: Indicates the number of word errors on the address/command lane. An error on any bit of the address/command bus increments the error count."]
        #[inline(always)]
        pub const fn acwer(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Address/Command Word Error: Indicates the number of word errors on the address/command lane. An error on any bit of the address/command bus increments the error count."]
        #[inline(always)]
        pub fn set_acwer(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Byte Word Error: Indicates the number of word errors on the byte lane. An error on any bit of the data bus including the data mask bit increments the error count."]
        #[inline(always)]
        pub const fn dxwer(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte Word Error: Indicates the number of word errors on the byte lane. An error on any bit of the data bus including the data mask bit increments the error count."]
        #[inline(always)]
        pub fn set_dxwer(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Bistwer {
        #[inline(always)]
        fn default() -> Bistwer {
            Bistwer(0)
        }
    }
    #[doc = "Impedance Control Register 0-1 (ZQnCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr0(pub u32);
    impl Cr0 {
        #[doc = "Impedance Over-Ride Data: Data used to directly drive the impedance control. ZDATA field mapping for D3F I/Os is as follows: ZDATA\\[27:21\\]
is used to select the pull-up on-die termination impedance ZDATA\\[20:14\\]
is used to select the pull-down on-die termination impedance ZDATA\\[13:7\\]
is used to select the pull-up output impedance ZDATA\\[6:0\\]
is used to select the pull-down output impedance ZDATA field mapping for D3A/B/R I/Os is as follows: ZDATA\\[27:20\\]
is reserved and returns zeros on reads ZDATA\\[19:15\\]
is used to select the pull-up on-die termination impedance ZDATA\\[14:10\\]
is used to select the pull-down on-die termination impedance ZDATA\\[9:5\\]
is used to select the pull-up output impedance ZDATA\\[4:0\\]
is used to select the pull-down output impedance The default value is 0x000014A for I/O type D3C/R and 0x0001830 for I/O type D3F."]
        #[inline(always)]
        pub const fn zdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "Impedance Over-Ride Data: Data used to directly drive the impedance control. ZDATA field mapping for D3F I/Os is as follows: ZDATA\\[27:21\\]
is used to select the pull-up on-die termination impedance ZDATA\\[20:14\\]
is used to select the pull-down on-die termination impedance ZDATA\\[13:7\\]
is used to select the pull-up output impedance ZDATA\\[6:0\\]
is used to select the pull-down output impedance ZDATA field mapping for D3A/B/R I/Os is as follows: ZDATA\\[27:20\\]
is reserved and returns zeros on reads ZDATA\\[19:15\\]
is used to select the pull-up on-die termination impedance ZDATA\\[14:10\\]
is used to select the pull-down on-die termination impedance ZDATA\\[9:5\\]
is used to select the pull-up output impedance ZDATA\\[4:0\\]
is used to select the pull-down output impedance The default value is 0x000014A for I/O type D3C/R and 0x0001830 for I/O type D3F."]
        #[inline(always)]
        pub fn set_zdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "Impedance Over-ride Enable: When this bit is set, it allows users to directly drive the impedance control using the data programmed in the ZDATA field. Otherwise, the control is generated automatically by the impedance control logic."]
        #[inline(always)]
        pub const fn zden(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Over-ride Enable: When this bit is set, it allows users to directly drive the impedance control using the data programmed in the ZDATA field. Otherwise, the control is generated automatically by the impedance control logic."]
        #[inline(always)]
        pub fn set_zden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Impedance Calibration Bypass: Bypasses, if set, impedance calibration of this ZQ control block when impedance calibration is already in progress. Impedance calibration can be disabled prior to trigger by using the ZCALEN bit."]
        #[inline(always)]
        pub const fn zcalbyp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Bypass: Bypasses, if set, impedance calibration of this ZQ control block when impedance calibration is already in progress. Impedance calibration can be disabled prior to trigger by using the ZCALEN bit."]
        #[inline(always)]
        pub fn set_zcalbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Impedance Calibration Enable: Enables, if set, the impedance calibration of this ZQ control block when impedance calibration is triggered using either the ZCAL bit of PIR register or the DFI update interface."]
        #[inline(always)]
        pub const fn zcalen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Enable: Enables, if set, the impedance calibration of this ZQ control block when impedance calibration is triggered using either the ZCAL bit of PIR register or the DFI update interface."]
        #[inline(always)]
        pub fn set_zcalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ZQ Power Down: Powers down, if set, the PZQ cell."]
        #[inline(always)]
        pub const fn zqpd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ZQ Power Down: Powers down, if set, the PZQ cell."]
        #[inline(always)]
        pub fn set_zqpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr0 {
        #[inline(always)]
        fn default() -> Cr0 {
            Cr0(0)
        }
    }
    #[doc = "Impedance Control Register 0-1 (ZQnCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Impedance Divide Ratio: Selects the external resistor divide ratio to be used to set the output impedance and the on-die termination as follows: ZPROG\\[7:4\\]
= On-die termination divide select ZPROG\\[3:0\\]
= Output impedance divide select."]
        #[inline(always)]
        pub const fn zprog(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Impedance Divide Ratio: Selects the external resistor divide ratio to be used to set the output impedance and the on-die termination as follows: ZPROG\\[7:4\\]
= On-die termination divide select ZPROG\\[3:0\\]
= Output impedance divide select."]
        #[inline(always)]
        pub fn set_zprog(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DFI Controller Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 0 (channel 0) requests an update."]
        #[inline(always)]
        pub const fn dficu0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DFI Controller Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 0 (channel 0) requests an update."]
        #[inline(always)]
        pub fn set_dficu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DFI Controller Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 1 (channel 1) requests an update. Only valid in shared-AC mode."]
        #[inline(always)]
        pub const fn dficu1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DFI Controller Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 1 (channel 1) requests an update. Only valid in shared-AC mode."]
        #[inline(always)]
        pub fn set_dficu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DFI Concurrent Controller Update Interface: Sets this impedance controller to be enabled for calibration when both of the DFI controller update interfaces request an update on the same clock. This provides the ability to enable impedance calibration updates for the Address/Command lane. Only valid in shared-AC mode."]
        #[inline(always)]
        pub const fn dficcu(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DFI Concurrent Controller Update Interface: Sets this impedance controller to be enabled for calibration when both of the DFI controller update interfaces request an update on the same clock. This provides the ability to enable impedance calibration updates for the Address/Command lane. Only valid in shared-AC mode."]
        #[inline(always)]
        pub fn set_dficcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DFI Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 0 (channel 0) requests an update."]
        #[inline(always)]
        pub const fn dfipu0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DFI Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 0 (channel 0) requests an update."]
        #[inline(always)]
        pub fn set_dfipu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DFI Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 1 (channel 1) requests an update. Only valid in shared-AC mode."]
        #[inline(always)]
        pub const fn dfipu1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DFI Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 1 (channel 1) requests an update. Only valid in shared-AC mode."]
        #[inline(always)]
        pub fn set_dfipu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "“DRAM Configuration Register (DCR)” on page 103."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr(pub u32);
    impl Dcr {
        #[doc = "DDR Mode: SDRAM DDR mode. Valid values are: 000 = Reserved 001 = Reserved 010 = DDR2 011 = DDR3 100 – 111 = Reserved."]
        #[inline(always)]
        pub const fn ddrmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "DDR Mode: SDRAM DDR mode. Valid values are: 000 = Reserved 001 = Reserved 010 = DDR2 011 = DDR3 100 – 111 = Reserved."]
        #[inline(always)]
        pub fn set_ddrmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "DDR 8-Bank: Indicates, if set, that the SDRAM used has 8 banks. tRPA = tRP+1 and tFAW are used for 8-bank DRAMs, otherwise tRPA = tRP and no tFAW is used. Note that a setting of 1 for DRAMs that have fewer than 8 banks results in correct functionality, but less tight DRAM command spacing for the parameters."]
        #[inline(always)]
        pub const fn ddr8bnk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DDR 8-Bank: Indicates, if set, that the SDRAM used has 8 banks. tRPA = tRP+1 and tFAW are used for 8-bank DRAMs, otherwise tRPA = tRP and no tFAW is used. Note that a setting of 1 for DRAMs that have fewer than 8 banks results in correct functionality, but less tight DRAM command spacing for the parameters."]
        #[inline(always)]
        pub fn set_ddr8bnk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Primary DQ (DDR3 Only): Specifies the DQ pin in a byte that is designated as a primary pin for Multi-Purpose Register (MPR) reads. Valid values are 0 to 7 for DQ\\[0\\]
to DQ\\[7\\], respectively."]
        #[inline(always)]
        pub const fn pdq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Primary DQ (DDR3 Only): Specifies the DQ pin in a byte that is designated as a primary pin for Multi-Purpose Register (MPR) reads. Valid values are 0 to 7 for DQ\\[0\\]
to DQ\\[7\\], respectively."]
        #[inline(always)]
        pub fn set_pdq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Multi-Purpose Register (MPR) DQ (DDR3 Only): Specifies the value that is driven on non-primary DQ pins during MPR reads. Valid values are: 0 = Primary DQ drives out the data from MPR (0-1-0-1); non-primary DQs drive ‘0’ 1 = Primary DQ and non-primary DQs all drive the same data from MPR (0-1-0-1)."]
        #[inline(always)]
        pub const fn mprdq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Purpose Register (MPR) DQ (DDR3 Only): Specifies the value that is driven on non-primary DQ pins during MPR reads. Valid values are: 0 = Primary DQ drives out the data from MPR (0-1-0-1); non-primary DQs drive ‘0’ 1 = Primary DQ and non-primary DQs all drive the same data from MPR (0-1-0-1)."]
        #[inline(always)]
        pub fn set_mprdq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Byte Mask: Mask applied to all beats of read data on all bytes lanes during read DQS gate training. This allows training to be conducted based on selected bit(s) from the byte lanes. Valid values for each bit are: 0 = Disable compare for that bit 1 = Enable compare for that bit Note that this mask applies in DDR3 MPR operation mode as well and must be in keeping with the PDQ field setting."]
        #[inline(always)]
        pub const fn bytemask(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "Byte Mask: Mask applied to all beats of read data on all bytes lanes during read DQS gate training. This allows training to be conducted based on selected bit(s) from the byte lanes. Valid values for each bit are: 0 = Disable compare for that bit 1 = Enable compare for that bit Note that this mask applies in DDR3 MPR operation mode as well and must be in keeping with the PDQ field setting."]
        #[inline(always)]
        pub fn set_bytemask(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
        #[doc = "No Simultaneous Rank Access: Specifies, if set, that simultaneous rank access on the same clock cycle is not allowed. This means that multiple chip select signals should not be asserted at the same time. This may be required on some DIMM systems."]
        #[inline(always)]
        pub const fn nosra(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No Simultaneous Rank Access: Specifies, if set, that simultaneous rank access on the same clock cycle is not allowed. This means that multiple chip select signals should not be asserted at the same time. This may be required on some DIMM systems."]
        #[inline(always)]
        pub fn set_nosra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DDR 2T Timing: Indicates, if set, that 2T timing should be used by PUB internally generated SDRAM transactions."]
        #[inline(always)]
        pub const fn ddr2t(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DDR 2T Timing: Indicates, if set, that 2T timing should be used by PUB internally generated SDRAM transactions."]
        #[inline(always)]
        pub fn set_ddr2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Un-buffered DIMM Address Mirroring: Indicates, if set, that there is address mirroring on the second rank of an un-buffered DIMM (the rank connected to CS#\\[1\\]). In this case, the PUB re-scrambles the bank and address when sending mode register commands to the second rank. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments when using an un-buffered DIMM. DCR\\[NOSRA\\]
must be set if address mirroring is enabled."]
        #[inline(always)]
        pub const fn udimm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Un-buffered DIMM Address Mirroring: Indicates, if set, that there is address mirroring on the second rank of an un-buffered DIMM (the rank connected to CS#\\[1\\]). In this case, the PUB re-scrambles the bank and address when sending mode register commands to the second rank. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments when using an un-buffered DIMM. DCR\\[NOSRA\\]
must be set if address mirroring is enabled."]
        #[inline(always)]
        pub fn set_udimm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Dcr {
        #[inline(always)]
        fn default() -> Dcr {
            Dcr(0)
        }
    }
    #[doc = "“DCU Address Register (DCUAR)” on page 129."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcuar(pub u32);
    impl Dcuar {
        #[doc = "Cache Word Address: Address of the cache word to be accessed."]
        #[inline(always)]
        pub const fn cwaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Cache Word Address: Address of the cache word to be accessed."]
        #[inline(always)]
        pub fn set_cwaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Cache Slice Address: Address of the cache slice to be accessed."]
        #[inline(always)]
        pub const fn csaddr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Cache Slice Address: Address of the cache slice to be accessed."]
        #[inline(always)]
        pub fn set_csaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Cache Select: Selects the cache to be accessed. Valid values are: 00 = Command cache 01 = Expected data cache 10 = Read data cache 11 = Reserved."]
        #[inline(always)]
        pub const fn csel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Cache Select: Selects the cache to be accessed. Valid values are: 00 = Command cache 01 = Expected data cache 10 = Read data cache 11 = Reserved."]
        #[inline(always)]
        pub fn set_csel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Increment Address: Specifies, if set, that the cache address specified in WADDR and SADDR should be automatically incremented after each access of the cache. The increment happens in such a way that all the slices of a selected word are first accessed before going to the next word."]
        #[inline(always)]
        pub const fn inca(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Address: Specifies, if set, that the cache address specified in WADDR and SADDR should be automatically incremented after each access of the cache. The increment happens in such a way that all the slices of a selected word are first accessed before going to the next word."]
        #[inline(always)]
        pub fn set_inca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Access Type: Specifies the type of access to be performed using this address. Valid values are: 0 = Write access 1 = Read access."]
        #[inline(always)]
        pub const fn atype(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Access Type: Specifies the type of access to be performed using this address. Valid values are: 0 = Write access 1 = Read access."]
        #[inline(always)]
        pub fn set_atype(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dcuar {
        #[inline(always)]
        fn default() -> Dcuar {
            Dcuar(0)
        }
    }
    #[doc = "“DCU Data Register (DCUDR)” on page 130."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcudr(pub u32);
    impl Dcudr {
        #[doc = "Cache Data: Data to be written to or read from a cache. This data corresponds to the cache word slice specified by the DCU Address Register."]
        #[inline(always)]
        pub const fn cdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Cache Data: Data to be written to or read from a cache. This data corresponds to the cache word slice specified by the DCU Address Register."]
        #[inline(always)]
        pub fn set_cdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dcudr {
        #[inline(always)]
        fn default() -> Dcudr {
            Dcudr(0)
        }
    }
    #[doc = "“DCU General Configuration Register (DCUGCR)” on page 132."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcugcr(pub u32);
    impl Dcugcr {
        #[doc = "Read Capture Start Word: The capture and compare of read data should start after Nth word. For example setting this value to 12 will skip the first 12 read data."]
        #[inline(always)]
        pub const fn rcsw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Read Capture Start Word: The capture and compare of read data should start after Nth word. For example setting this value to 12 will skip the first 12 read data."]
        #[inline(always)]
        pub fn set_rcsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dcugcr {
        #[inline(always)]
        fn default() -> Dcugcr {
            Dcugcr(0)
        }
    }
    #[doc = "“DCU Loop Register (DCULR)” on page 131."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dculr(pub u32);
    impl Dculr {
        #[doc = "Loop Start Address: Command cache word address where the loop should start."]
        #[inline(always)]
        pub const fn lsaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Loop Start Address: Command cache word address where the loop should start."]
        #[inline(always)]
        pub fn set_lsaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Loop End Address: Command cache word address where the loop should end."]
        #[inline(always)]
        pub const fn leaddr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Loop End Address: Command cache word address where the loop should end."]
        #[inline(always)]
        pub fn set_leaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Loop Count: The number of times that the loop should be executed if LINF is not set."]
        #[inline(always)]
        pub const fn lcnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Loop Count: The number of times that the loop should be executed if LINF is not set."]
        #[inline(always)]
        pub fn set_lcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Loop Infinite: Indicates, if set, that the loop should be executed indefinitely until stopped by the STOP command. Otherwise the loop is execute LCNT times."]
        #[inline(always)]
        pub const fn linf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Infinite: Indicates, if set, that the loop should be executed indefinitely until stopped by the STOP command. Otherwise the loop is execute LCNT times."]
        #[inline(always)]
        pub fn set_linf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment DRAM Address: Indicates, if set, that DRAM addresses should be incremented every time a DRAM read/write command inside the loop is executed."]
        #[inline(always)]
        pub const fn ida(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment DRAM Address: Indicates, if set, that DRAM addresses should be incremented every time a DRAM read/write command inside the loop is executed."]
        #[inline(always)]
        pub fn set_ida(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Expected Data Loop End Address: The last expected data cache word address that contains valid expected data. Expected data should looped between 0 and this address. XLEADDR field uses only the following bits based on the cache depth: DCU expected data cache = 4, XLEADDR\\[1:0\\]
DCU expected data cache = 8, XLEADDR\\[2:0\\]
DCU expected data cache = 16, XLEADDR\\[3:0\\]."]
        #[inline(always)]
        pub const fn xleaddr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Expected Data Loop End Address: The last expected data cache word address that contains valid expected data. Expected data should looped between 0 and this address. XLEADDR field uses only the following bits based on the cache depth: DCU expected data cache = 4, XLEADDR\\[1:0\\]
DCU expected data cache = 8, XLEADDR\\[2:0\\]
DCU expected data cache = 16, XLEADDR\\[3:0\\]."]
        #[inline(always)]
        pub fn set_xleaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Dculr {
        #[inline(always)]
        fn default() -> Dculr {
            Dculr(0)
        }
    }
    #[doc = "“DCU Run Register (DCURR)” on page 130."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcurr(pub u32);
    impl Dcurr {
        #[doc = "DCU Instruction: Selects the DCU command to be executed: Valid values are: 0000 = NOP: No operation 0001 = Run: Triggers the execution of commands in the command cache. 0010 = Stop: Stops the execution of commands in the command cache. 0011 = Stop Loop: Stops the execution of an infinite loop in the command cache. 0100 = Reset: Resets all DCU run time registers. See “DCU Status” on page 255 for details. 0101 – 1111 Reserved."]
        #[inline(always)]
        pub const fn dinst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DCU Instruction: Selects the DCU command to be executed: Valid values are: 0000 = NOP: No operation 0001 = Run: Triggers the execution of commands in the command cache. 0010 = Stop: Stops the execution of commands in the command cache. 0011 = Stop Loop: Stops the execution of an infinite loop in the command cache. 0100 = Reset: Resets all DCU run time registers. See “DCU Status” on page 255 for details. 0101 – 1111 Reserved."]
        #[inline(always)]
        pub fn set_dinst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Start Address: Cache word address where the execution of commands should begin."]
        #[inline(always)]
        pub const fn saddr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Start Address: Cache word address where the execution of commands should begin."]
        #[inline(always)]
        pub fn set_saddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "End Address: Cache word address where the execution of command should end."]
        #[inline(always)]
        pub const fn eaddr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "End Address: Cache word address where the execution of command should end."]
        #[inline(always)]
        pub fn set_eaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if SONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if SONF is set. Valid values are from 0 to 254."]
        #[inline(always)]
        pub const fn nfail(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if SONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if SONF is set. Valid values are from 0 to 254."]
        #[inline(always)]
        pub fn set_nfail(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Stop On Nth Fail: Specifies, if set, that the execution of commands and the capture of read data should stop when there are N read data failures. The number of failures is specified by NFAIL. Otherwise commands execute until the end of the program or until manually stopped using a STOP command."]
        #[inline(always)]
        pub const fn sonf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Stop On Nth Fail: Specifies, if set, that the execution of commands and the capture of read data should stop when there are N read data failures. The number of failures is specified by NFAIL. Otherwise commands execute until the end of the program or until manually stopped using a STOP command."]
        #[inline(always)]
        pub fn set_sonf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Stop Capture On Full: Specifies, if set, that the capture of read data should stop when the capture cache is full."]
        #[inline(always)]
        pub const fn scof(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Capture On Full: Specifies, if set, that the capture of read data should stop when the capture cache is full."]
        #[inline(always)]
        pub fn set_scof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Read Capture Enable: Indicates, if set, that read data coming back from the SDRAM should be captured into the read data cache."]
        #[inline(always)]
        pub const fn rcen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Read Capture Enable: Indicates, if set, that read data coming back from the SDRAM should be captured into the read data cache."]
        #[inline(always)]
        pub fn set_rcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Expected Compare Enable: Indicates, if set, that read data coming back from the SDRAM should be should be compared with the expected data."]
        #[inline(always)]
        pub const fn xcen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Expected Compare Enable: Indicates, if set, that read data coming back from the SDRAM should be should be compared with the expected data."]
        #[inline(always)]
        pub fn set_xcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Dcurr {
        #[inline(always)]
        fn default() -> Dcurr {
            Dcurr(0)
        }
    }
    #[doc = "DCU Status Register 0-1 (DCUSR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcusr0(pub u32);
    impl Dcusr0 {
        #[doc = "Run Done: Indicates, if set, that the DCU has finished executing the commands in the command cache. This bit is also set to indicate that a STOP command has successfully been executed and command execution has stopped."]
        #[inline(always)]
        pub const fn rdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Run Done: Indicates, if set, that the DCU has finished executing the commands in the command cache. This bit is also set to indicate that a STOP command has successfully been executed and command execution has stopped."]
        #[inline(always)]
        pub fn set_rdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture Fail: Indicates, if set, that at least one read data word has failed."]
        #[inline(always)]
        pub const fn cfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Capture Fail: Indicates, if set, that at least one read data word has failed."]
        #[inline(always)]
        pub fn set_cfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Capture Full: Indicates, if set, that the capture cache is full."]
        #[inline(always)]
        pub const fn cfull(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture Full: Indicates, if set, that the capture cache is full."]
        #[inline(always)]
        pub fn set_cfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Dcusr0 {
        #[inline(always)]
        fn default() -> Dcusr0 {
            Dcusr0(0)
        }
    }
    #[doc = "DCU Status Register 0-1 (DCUSR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcusr1(pub u32);
    impl Dcusr1 {
        #[doc = "Read Count: Number of read words returned from the SDRAM."]
        #[inline(always)]
        pub const fn rdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Read Count: Number of read words returned from the SDRAM."]
        #[inline(always)]
        pub fn set_rdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Fail Count: Number of read words that have failed."]
        #[inline(always)]
        pub const fn flcnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Fail Count: Number of read words that have failed."]
        #[inline(always)]
        pub fn set_flcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Loop Count: Indicates the value of the loop count. This is useful when the program has stopped because of failures to assess how many reads were executed before first fail."]
        #[inline(always)]
        pub const fn lpcnt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Loop Count: Indicates the value of the loop count. This is useful when the program has stopped because of failures to assess how many reads were executed before first fail."]
        #[inline(always)]
        pub fn set_lpcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dcusr1 {
        #[inline(always)]
        fn default() -> Dcusr1 {
            Dcusr1(0)
        }
    }
    #[doc = "“DCU Timing Parameter Register (DCUTPR)” on page 132."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcutpr(pub u32);
    impl Dcutpr {
        #[doc = "DCU Generic Timing Parameter 0."]
        #[inline(always)]
        pub const fn tdcut0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DCU Generic Timing Parameter 0."]
        #[inline(always)]
        pub fn set_tdcut0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "DCU Generic Timing Parameter 1."]
        #[inline(always)]
        pub const fn tdcut1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "DCU Generic Timing Parameter 1."]
        #[inline(always)]
        pub fn set_tdcut1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "DCU Generic Timing Parameter 2."]
        #[inline(always)]
        pub const fn tdcut2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "DCU Generic Timing Parameter 2."]
        #[inline(always)]
        pub fn set_tdcut2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "DCU Generic Timing Parameter 3."]
        #[inline(always)]
        pub const fn tdcut3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "DCU Generic Timing Parameter 3."]
        #[inline(always)]
        pub fn set_tdcut3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dcutpr {
        #[inline(always)]
        fn default() -> Dcutpr {
            Dcutpr(0)
        }
    }
    #[doc = "“DDR System General Configuration Register (DSGCR)” on page 101."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dsgcr(pub u32);
    impl Dsgcr {
        #[doc = "PHY Update Request Enable: Specifies if set, that the PHY should issue PHY- initiated update request when there is DDL VT drift."]
        #[inline(always)]
        pub const fn puren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Update Request Enable: Specifies if set, that the PHY should issue PHY- initiated update request when there is DDL VT drift."]
        #[inline(always)]
        pub fn set_puren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Byte Disable Enable: Specifies, if set, that the PHY should respond to DFI byte disable request. Otherwise the byte disable from the DFI is ignored in which case bytes can only be disabled using the DXnGCR register."]
        #[inline(always)]
        pub const fn bdisen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Disable Enable: Specifies, if set, that the PHY should respond to DFI byte disable request. Otherwise the byte disable from the DFI is ignored in which case bytes can only be disabled using the DXnGCR register."]
        #[inline(always)]
        pub fn set_bdisen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Impedance Update Enable: Specifies, if set, that in addition to DDL VT update, the PHY could also perform impedance calibration (update). Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFICU0, DFICU1 and DFICCU bits to control if an impedance calibration is performed (update) with a DFI controller update request. Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFIPU0 and DFIPU1 bits to control if an impedance calibration is performed (update) with a DFI PHY update request."]
        #[inline(always)]
        pub const fn zuen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Update Enable: Specifies, if set, that in addition to DDL VT update, the PHY could also perform impedance calibration (update). Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFICU0, DFICU1 and DFICCU bits to control if an impedance calibration is performed (update) with a DFI controller update request. Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFIPU0 and DFIPU1 bits to control if an impedance calibration is performed (update) with a DFI PHY update request."]
        #[inline(always)]
        pub fn set_zuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Low Power I/O Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the I/Os of the byte."]
        #[inline(always)]
        pub const fn lpiopd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Low Power I/O Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the I/Os of the byte."]
        #[inline(always)]
        pub fn set_lpiopd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Low Power PLL Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the PLL of the byte if the wakeup time request satisfies the PLL lock time."]
        #[inline(always)]
        pub const fn lppllpd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Low Power PLL Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the PLL of the byte if the wakeup time request satisfies the PLL lock time."]
        #[inline(always)]
        pub fn set_lppllpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Controller Update Acknowledge Enable: Specifies, if set, that the PHY should issue controller update acknowledge when the DFI controller update request is asserted. By default the PHY does not acknowledge controller initiated update requests but simply does an update whenever there is a controller update request. This speeds up the update."]
        #[inline(always)]
        pub const fn cuaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Controller Update Acknowledge Enable: Specifies, if set, that the PHY should issue controller update acknowledge when the DFI controller update request is asserted. By default the PHY does not acknowledge controller initiated update requests but simply does an update whenever there is a controller update request. This speeds up the update."]
        #[inline(always)]
        pub fn set_cuaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DQS Gate Extension: Specifies, if set, that the DQS gating must be extended by two DRAM clock cycles and then re-centered, i.e. one clock cycle extension on either side."]
        #[inline(always)]
        pub const fn dqsgx(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Gate Extension: Specifies, if set, that the DQS gating must be extended by two DRAM clock cycles and then re-centered, i.e. one clock cycle extension on either side."]
        #[inline(always)]
        pub fn set_dqsgx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bypass Rise-to-Rise Mode: Indicates, if set, that the PHY bypass mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY bypass mode is running in rise-to-fall mode."]
        #[inline(always)]
        pub const fn brrmode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Rise-to-Rise Mode: Indicates, if set, that the PHY bypass mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY bypass mode is running in rise-to-fall mode."]
        #[inline(always)]
        pub fn set_brrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PHY Update Acknowledge Delay: Specifies the number of clock cycles that the indication for the completion of PHY update from the PHY to the controller should be delayed. This essentially delays, by this many clock cycles, the de-assertion of dfi_ctrlup_ack and dfi_phyupd_req signals relative to the time when the delay lines or I/Os are updated."]
        #[inline(always)]
        pub const fn puad(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "PHY Update Acknowledge Delay: Specifies the number of clock cycles that the indication for the completion of PHY update from the PHY to the controller should be delayed. This essentially delays, by this many clock cycles, the de-assertion of dfi_ctrlup_ack and dfi_phyupd_req signals relative to the time when the delay lines or I/Os are updated."]
        #[inline(always)]
        pub fn set_puad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DTO On-Die Termination: Enables, when set, the on-die termination on the I/O for DTO pins."]
        #[inline(always)]
        pub const fn dtoodt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DTO On-Die Termination: Enables, when set, the on-die termination on the I/O for DTO pins."]
        #[inline(always)]
        pub fn set_dtoodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DTO Power Down Driver: Powers down, when set, the output driver on the I/O for DTO pins."]
        #[inline(always)]
        pub const fn dtopdd1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DTO Power Down Driver: Powers down, when set, the output driver on the I/O for DTO pins."]
        #[inline(always)]
        pub fn set_dtopdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DTO Power Down Receiver: Powers down, when set, the input receiver on the I/O for DTO pins."]
        #[inline(always)]
        pub const fn dtopdr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DTO Power Down Receiver: Powers down, when set, the input receiver on the I/O for DTO pins."]
        #[inline(always)]
        pub fn set_dtopdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DTO I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DTO pins."]
        #[inline(always)]
        pub const fn dtoiom(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DTO I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DTO pins."]
        #[inline(always)]
        pub fn set_dtoiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DTO Output Enable: Enables, when set, the output driver on the I/O for DTO pins."]
        #[inline(always)]
        pub const fn dtooe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DTO Output Enable: Enables, when set, the output driver on the I/O for DTO pins."]
        #[inline(always)]
        pub fn set_dtooe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ATO Analog Test Enable: Enables, if set, the analog test output (ATO) I/O."]
        #[inline(always)]
        pub const fn atoae(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ATO Analog Test Enable: Enables, if set, the analog test output (ATO) I/O."]
        #[inline(always)]
        pub fn set_atoae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Rise-to-Rise Mode: Indicates, if set, that the PHY mission mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY mission mode is running in rise-to- fall mode."]
        #[inline(always)]
        pub const fn rrmode(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Rise-to-Rise Mode: Indicates, if set, that the PHY mission mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY mission mode is running in rise-to- fall mode."]
        #[inline(always)]
        pub fn set_rrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Single Data Rate Mode: Indicates, if set, that the external controller is configured to run in single data rate (SDR) mode. Otherwise if not set the controller is running in half data rate (HDR) mode. This bit not supported in the current version of the PUB."]
        #[inline(always)]
        pub const fn sdrmode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Single Data Rate Mode: Indicates, if set, that the external controller is configured to run in single data rate (SDR) mode. Otherwise if not set the controller is running in half data rate (HDR) mode. This bit not supported in the current version of the PUB."]
        #[inline(always)]
        pub fn set_sdrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "CKE Power Down Driver: Powers down, when set, the output driver on the I/O for CKE\\[3:0\\]
pins. CKEPDD\\[0\\]
controls the power down for CKE\\[0\\], CKEPDD\\[1\\]
controls the power down for CKE\\[1\\], and so on."]
        #[inline(always)]
        pub const fn ckepdd1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "CKE Power Down Driver: Powers down, when set, the output driver on the I/O for CKE\\[3:0\\]
pins. CKEPDD\\[0\\]
controls the power down for CKE\\[0\\], CKEPDD\\[1\\]
controls the power down for CKE\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_ckepdd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "ODT Power Down Driver: Powers down, when set, the output driver on the I/O for ODT\\[3:0\\]
pins. ODTPDD\\[0\\]
controls the power down for ODT\\[0\\], ODTPDD\\[1\\]
controls the power down for ODT\\[1\\], and so on."]
        #[inline(always)]
        pub const fn odtpdd1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "ODT Power Down Driver: Powers down, when set, the output driver on the I/O for ODT\\[3:0\\]
pins. ODTPDD\\[0\\]
controls the power down for ODT\\[0\\], ODTPDD\\[1\\]
controls the power down for ODT\\[1\\], and so on."]
        #[inline(always)]
        pub fn set_odtpdd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "SDRAM CK Output Enable: Enables, when set, the output driver on the I/O for SDRAM CK/CK# pins."]
        #[inline(always)]
        pub const fn ckoe(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM CK Output Enable: Enables, when set, the output driver on the I/O for SDRAM CK/CK# pins."]
        #[inline(always)]
        pub fn set_ckoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SDRAM ODT Output Enable: Enables, when set, the output driver on the I/O for SDRAM ODT pins."]
        #[inline(always)]
        pub const fn odtoe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM ODT Output Enable: Enables, when set, the output driver on the I/O for SDRAM ODT pins."]
        #[inline(always)]
        pub fn set_odtoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SDRAM Reset Output Enable: Enables, when set, the output driver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub const fn rstoe(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM Reset Output Enable: Enables, when set, the output driver on the I/O for SDRAM RST# pin."]
        #[inline(always)]
        pub fn set_rstoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SDRAM CKE Output Enable: Enables, when set, the output driver on the I/O for SDRAM CKE pins."]
        #[inline(always)]
        pub const fn ckeoe(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM CKE Output Enable: Enables, when set, the output driver on the I/O for SDRAM CKE pins."]
        #[inline(always)]
        pub fn set_ckeoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dsgcr {
        #[inline(always)]
        fn default() -> Dsgcr {
            Dsgcr(0)
        }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtar0(pub u32);
    impl Dtar0 {
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub const fn dtcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub fn set_dtcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub const fn dtrow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub fn set_dtrow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub const fn dtbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub fn set_dtbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Dtar0 {
        #[inline(always)]
        fn default() -> Dtar0 {
            Dtar0(0)
        }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtar1(pub u32);
    impl Dtar1 {
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub const fn dtcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub fn set_dtcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub const fn dtrow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub fn set_dtrow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub const fn dtbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub fn set_dtbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Dtar1 {
        #[inline(always)]
        fn default() -> Dtar1 {
            Dtar1(0)
        }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtar2(pub u32);
    impl Dtar2 {
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub const fn dtcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub fn set_dtcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub const fn dtrow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub fn set_dtrow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub const fn dtbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub fn set_dtbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Dtar2 {
        #[inline(always)]
        fn default() -> Dtar2 {
            Dtar2(0)
        }
    }
    #[doc = "Data Training Address Register 0-3 (DTAR0-3)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtar3(pub u32);
    impl Dtar3 {
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub const fn dtcol(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”."]
        #[inline(always)]
        pub fn set_dtcol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub const fn dtrow(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Data Training Row Address: Selects the SDRAM row address to be used during data training."]
        #[inline(always)]
        pub fn set_dtrow(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub const fn dtbank(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Data Training Bank Address: Selects the SDRAM bank address to be used during data training."]
        #[inline(always)]
        pub fn set_dtbank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Dtar3 {
        #[inline(always)]
        fn default() -> Dtar3 {
            Dtar3(0)
        }
    }
    #[doc = "“Data Training Configuration Register (DTCR)” on page 118."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtcr(pub u32);
    impl Dtcr {
        #[doc = "Data Training Repeat Number: Repeat number used to confirm stability of DDR write or read. Note: The minimum value should be 0x4 and the maximum value should be 0x14."]
        #[inline(always)]
        pub const fn dtrptn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Data Training Repeat Number: Repeat number used to confirm stability of DDR write or read. Note: The minimum value should be 0x4 and the maximum value should be 0x14."]
        #[inline(always)]
        pub fn set_dtrptn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Data Training Rank: Select the SDRAM rank to be used during Read DQS gate training, Read/Write Data Bit Deskew, Read/Write Eye Training."]
        #[inline(always)]
        pub const fn dtrank(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Data Training Rank: Select the SDRAM rank to be used during Read DQS gate training, Read/Write Data Bit Deskew, Read/Write Eye Training."]
        #[inline(always)]
        pub fn set_dtrank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Read Data Training Using MPR (DDR3 Only): Specifies, if set, that DQS gate training should use the SDRAM Multi-Purpose Register (MPR) register. Otherwise data-training is performed by first writing to some locations in the SDRAM and then reading them back."]
        #[inline(always)]
        pub const fn dtmpr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Training Using MPR (DDR3 Only): Specifies, if set, that DQS gate training should use the SDRAM Multi-Purpose Register (MPR) register. Otherwise data-training is performed by first writing to some locations in the SDRAM and then reading them back."]
        #[inline(always)]
        pub fn set_dtmpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Read Data Training Compare Data: Specifies, if set, that DQS gate training should also check if the returning read data is correct. Otherwise data-training only checks if the correct number of DQS edges were returned."]
        #[inline(always)]
        pub const fn dtcmpd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Training Compare Data: Specifies, if set, that DQS gate training should also check if the returning read data is correct. Otherwise data-training only checks if the correct number of DQS edges were returned."]
        #[inline(always)]
        pub fn set_dtcmpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Training WDQ Margin: Defines how close to 0 or how close to 2*(wdq calibration_value) the WDQ LCDL can be moved during training. Basically defines how much timing margin."]
        #[inline(always)]
        pub const fn dtwdqm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Training WDQ Margin: Defines how close to 0 or how close to 2*(wdq calibration_value) the WDQ LCDL can be moved during training. Basically defines how much timing margin."]
        #[inline(always)]
        pub fn set_dtwdqm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Data Training Write Bit Deskew Data Mask, if set, it enables write bit deskew of the data mask."]
        #[inline(always)]
        pub const fn dtwbddm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training Write Bit Deskew Data Mask, if set, it enables write bit deskew of the data mask."]
        #[inline(always)]
        pub fn set_dtwbddm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Data Training Bit Deskew Centering: Enables, if set, eye centering capability during write and read bit deskew training."]
        #[inline(always)]
        pub const fn dtbdc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training Bit Deskew Centering: Enables, if set, eye centering capability during write and read bit deskew training."]
        #[inline(always)]
        pub fn set_dtbdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Data Training WDQ Margin Override: If set, the Training WDQ Margin value specified in DTCR\\[11:8\\]
(DTWDQM) is used during data training. Otherwise the value is computed as ¼ of the ddr_clk period measurement found during calibration of the WDQ LCDL."]
        #[inline(always)]
        pub const fn dtwdqmo(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training WDQ Margin Override: If set, the Training WDQ Margin value specified in DTCR\\[11:8\\]
(DTWDQM) is used during data training. Otherwise the value is computed as ¼ of the ddr_clk period measurement found during calibration of the WDQ LCDL."]
        #[inline(always)]
        pub fn set_dtwdqmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Data Training Debug Byte Select: Selects the byte during data training single step debug mode. Note: DTDEN is not used to enable this feature."]
        #[inline(always)]
        pub const fn dtdbs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Data Training Debug Byte Select: Selects the byte during data training single step debug mode. Note: DTDEN is not used to enable this feature."]
        #[inline(always)]
        pub fn set_dtdbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Data Training Debug Enable: Enables, if set, the data training single step debug mode."]
        #[inline(always)]
        pub const fn dtden(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training Debug Enable: Enables, if set, the data training single step debug mode."]
        #[inline(always)]
        pub fn set_dtden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Data Training Debug Step: A write of 1 to this bit steps the data training algorithm through a single step. This bit is used to initiate one step of the data training algorithm in question. This bit is self-clearing. To trigger the next step, this bit must be written to again. Note: The training steps must be repeated in order to get new data in the “Data Training Eye Data Register 0-1 (DTEDR0-1)” on page 122. For example, to see the training results for a different lane, select that lane and repeat the training steps to populate DTEDR0 and DTEDR1 with the correct data."]
        #[inline(always)]
        pub const fn dtdstp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training Debug Step: A write of 1 to this bit steps the data training algorithm through a single step. This bit is used to initiate one step of the data training algorithm in question. This bit is self-clearing. To trigger the next step, this bit must be written to again. Note: The training steps must be repeated in order to get new data in the “Data Training Eye Data Register 0-1 (DTEDR0-1)” on page 122. For example, to see the training results for a different lane, select that lane and repeat the training steps to populate DTEDR0 and DTEDR1 with the correct data."]
        #[inline(always)]
        pub fn set_dtdstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Data Training Extended Write DQS: Enables, if set, an extended write DQS whereby two additional pulses of DQS are added as post-amble to a burst of writes. Generally this should only be enabled when running read bit deskew with the intention of performing read eye deskew prior to running write leveling adjustment."]
        #[inline(always)]
        pub const fn dtexd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Data Training Extended Write DQS: Enables, if set, an extended write DQS whereby two additional pulses of DQS are added as post-amble to a burst of writes. Generally this should only be enabled when running read bit deskew with the intention of performing read eye deskew prior to running write leveling adjustment."]
        #[inline(always)]
        pub fn set_dtexd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Rank Enable: Specifies the ranks that are enabled for data-training. Bit 0 controls rank 0, bit 1 controls rank 1, bit 2 controls rank 2, and bit 3 controls rank 3. Setting the bit to ‘1’ enables the rank, and setting it to ‘0’ disables the rank."]
        #[inline(always)]
        pub const fn ranken(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Rank Enable: Specifies the ranks that are enabled for data-training. Bit 0 controls rank 0, bit 1 controls rank 1, bit 2 controls rank 2, and bit 3 controls rank 3. Setting the bit to ‘1’ enables the rank, and setting it to ‘0’ disables the rank."]
        #[inline(always)]
        pub fn set_ranken(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Refresh During Training: A non-zero value specifies that a burst of refreshes equal to the number specified in this field should be sent to the SDRAM after training each rank except the last rank."]
        #[inline(always)]
        pub const fn rfshdt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Refresh During Training: A non-zero value specifies that a burst of refreshes equal to the number specified in this field should be sent to the SDRAM after training each rank except the last rank."]
        #[inline(always)]
        pub fn set_rfshdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Dtcr {
        #[inline(always)]
        fn default() -> Dtcr {
            Dtcr(0)
        }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtdr0(pub u32);
    impl Dtdr0 {
        #[doc = "Data Training Data: The first 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle."]
        #[inline(always)]
        pub const fn dtbyte0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Data: The first 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle."]
        #[inline(always)]
        pub fn set_dtbyte0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dtdr0 {
        #[inline(always)]
        fn default() -> Dtdr0 {
            Dtdr0(0)
        }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtdr1(pub u32);
    impl Dtdr1 {
        #[doc = "Data Training Data: The second 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle."]
        #[inline(always)]
        pub const fn dtbyte4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Data: The second 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle."]
        #[inline(always)]
        pub fn set_dtbyte4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dtbyte7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dtbyte7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dtdr1 {
        #[inline(always)]
        fn default() -> Dtdr1 {
            Dtdr1(0)
        }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtedr0(pub u32);
    impl Dtedr0 {
        #[doc = "Data Training WDQ LCDL Minimum."]
        #[inline(always)]
        pub const fn dtwlmn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training WDQ LCDL Minimum."]
        #[inline(always)]
        pub fn set_dtwlmn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data Training WDQ LCDL Maximum."]
        #[inline(always)]
        pub const fn dtwlmx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training WDQ LCDL Maximum."]
        #[inline(always)]
        pub fn set_dtwlmx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data Training Write BDL Shift Minimum."]
        #[inline(always)]
        pub const fn dtwbmn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Write BDL Shift Minimum."]
        #[inline(always)]
        pub fn set_dtwbmn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data Training Write BDL Shift Maximum."]
        #[inline(always)]
        pub const fn dtwbmx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Write BDL Shift Maximum."]
        #[inline(always)]
        pub fn set_dtwbmx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dtedr0 {
        #[inline(always)]
        fn default() -> Dtedr0 {
            Dtedr0(0)
        }
    }
    #[doc = "Data Training Eye Data Register 0-1 (DTEDR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtedr1(pub u32);
    impl Dtedr1 {
        #[doc = "Data Training RDQS LCDL Minimum."]
        #[inline(always)]
        pub const fn dtrlmn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training RDQS LCDL Minimum."]
        #[inline(always)]
        pub fn set_dtrlmn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Data Training RDQS LCDL Maximum."]
        #[inline(always)]
        pub const fn dtrlmx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training RDQS LCDL Maximum."]
        #[inline(always)]
        pub fn set_dtrlmx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Data Training Read BDL Shift Minimum."]
        #[inline(always)]
        pub const fn dtrbmn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Read BDL Shift Minimum."]
        #[inline(always)]
        pub fn set_dtrbmn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data Training Read BDL Shift Maximum."]
        #[inline(always)]
        pub const fn dtrbmx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training Read BDL Shift Maximum."]
        #[inline(always)]
        pub fn set_dtrbmx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dtedr1 {
        #[inline(always)]
        fn default() -> Dtedr1 {
            Dtedr1(0)
        }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtpr0(pub u32);
    impl Dtpr0 {
        #[doc = "Internal read to precharge command delay. Valid values are 2 to 15."]
        #[inline(always)]
        pub const fn trtp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Internal read to precharge command delay. Valid values are 2 to 15."]
        #[inline(always)]
        pub fn set_trtp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Internal write to read command delay. Valid values are 1 to 15."]
        #[inline(always)]
        pub const fn twtr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Internal write to read command delay. Valid values are 1 to 15."]
        #[inline(always)]
        pub fn set_twtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Precharge command period: The minimum time between a precharge command and any other command. Note that the Controller automatically derives tRPA for 8- bank DDR2 devices by adding 1 to tRP. Valid values are 2 to 15."]
        #[inline(always)]
        pub const fn trp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Precharge command period: The minimum time between a precharge command and any other command. Note that the Controller automatically derives tRPA for 8- bank DDR2 devices by adding 1 to tRP. Valid values are 2 to 15."]
        #[inline(always)]
        pub fn set_trp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Activate to read or write delay. Minimum time from when an activate command is issued to when a read or write to the activated row can be issued. Valid values are 2 to 15."]
        #[inline(always)]
        pub const fn trcd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Activate to read or write delay. Minimum time from when an activate command is issued to when a read or write to the activated row can be issued. Valid values are 2 to 15."]
        #[inline(always)]
        pub fn set_trcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Activate to precharge command delay. Valid values are 2 to 63."]
        #[inline(always)]
        pub const fn tras(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Activate to precharge command delay. Valid values are 2 to 63."]
        #[inline(always)]
        pub fn set_tras(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Activate to activate command delay (different banks). Valid values are 1 to 15."]
        #[inline(always)]
        pub const fn trrd(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "Activate to activate command delay (different banks). Valid values are 1 to 15."]
        #[inline(always)]
        pub fn set_trrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "Activate to activate command delay (same bank). Valid values are 2 to 63."]
        #[inline(always)]
        pub const fn trc(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "Activate to activate command delay (same bank). Valid values are 2 to 63."]
        #[inline(always)]
        pub fn set_trc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for Dtpr0 {
        #[inline(always)]
        fn default() -> Dtpr0 {
            Dtpr0(0)
        }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtpr1(pub u32);
    impl Dtpr1 {
        #[doc = "Load mode cycle time: The minimum time between a load mode register command and any other command. For DDR3 this is the minimum time between two load mode register commands. Valid values for DDR2 are 2 to 3. For DDR3, the value used for tMRD is 4 plus the value programmed in these bits, i.e. tMRD value for DDR3 ranges from 4 to 7."]
        #[inline(always)]
        pub const fn tmrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Load mode cycle time: The minimum time between a load mode register command and any other command. For DDR3 this is the minimum time between two load mode register commands. Valid values for DDR2 are 2 to 3. For DDR3, the value used for tMRD is 4 plus the value programmed in these bits, i.e. tMRD value for DDR3 ranges from 4 to 7."]
        #[inline(always)]
        pub fn set_tmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Load mode update delay (DDR3 only). The minimum time between a load mode register command and a non-load mode register command. Valid values are: 000 = 12 001 = 13 010 = 14 011 = 15 100 = 16 101 = 17 110 – 111 = Reserved."]
        #[inline(always)]
        pub const fn tmod(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Load mode update delay (DDR3 only). The minimum time between a load mode register command and a non-load mode register command. Valid values are: 000 = 12 001 = 13 010 = 14 011 = 15 100 = 16 101 = 17 110 – 111 = Reserved."]
        #[inline(always)]
        pub fn set_tmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "4-bank activate period. No more than 4-bank activate commands may be issued in a given tFAW period. Only applies to 8-bank devices. Valid values are 2 to 63."]
        #[inline(always)]
        pub const fn tfaw(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x3f;
            val as u8
        }
        #[doc = "4-bank activate period. No more than 4-bank activate commands may be issued in a given tFAW period. Only applies to 8-bank devices. Valid values are 2 to 63."]
        #[inline(always)]
        pub fn set_tfaw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 5usize)) | (((val as u32) & 0x3f) << 5usize);
        }
        #[doc = "Refresh-to-Refresh: Indicates the minimum time between two refresh commands or between a refresh and an active command. This is derived from the minimum refresh interval from the datasheet, tRFC(min), divided by the clock cycle time. The default number of clock cycles is for the largest JEDEC tRFC(min parameter value supported."]
        #[inline(always)]
        pub const fn trfc(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x01ff;
            val as u16
        }
        #[doc = "Refresh-to-Refresh: Indicates the minimum time between two refresh commands or between a refresh and an active command. This is derived from the minimum refresh interval from the datasheet, tRFC(min), divided by the clock cycle time. The default number of clock cycles is for the largest JEDEC tRFC(min parameter value supported."]
        #[inline(always)]
        pub fn set_trfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
        }
        #[doc = "Minimum delay from when write leveling mode is programmed to the first DQS/DQS# rising edge."]
        #[inline(always)]
        pub const fn twlmrd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "Minimum delay from when write leveling mode is programmed to the first DQS/DQS# rising edge."]
        #[inline(always)]
        pub fn set_twlmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "Write leveling output delay: Number of clock cycles from when write leveling DQS is driven high by the control block to when the results from the SDRAM on DQ is sampled by the control block. This must include the SDRAM tWLO timing parameter plus the round trip delay from control block to SDRAM back to control block."]
        #[inline(always)]
        pub const fn twlo(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x0f;
            val as u8
        }
        #[doc = "Write leveling output delay: Number of clock cycles from when write leveling DQS is driven high by the control block to when the results from the SDRAM on DQ is sampled by the control block. This must include the SDRAM tWLO timing parameter plus the round trip delay from control block to SDRAM back to control block."]
        #[inline(always)]
        pub fn set_twlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
        }
        #[doc = "ODT turn-on/turn-off delays (DDR2 only). Valid values are: 00 = 2/2.5 01 = 3/3.5 10 = 4/4.5 11 = 5/5.5 Most DDR2 devices utilize a fixed value of 2/2.5. For non-standard SDRAMs, the user must ensure that the operational Write Latency is always greater than or equal to the ODT turn-on delay. For example, a DDR2 SDRAM with CAS latency set to 3 and CAS additive latency set to 0 has a Write Latency of 2. Thus 2/2.5 can be used, but not 3/3.5 or higher."]
        #[inline(always)]
        pub const fn taond_taofd(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "ODT turn-on/turn-off delays (DDR2 only). Valid values are: 00 = 2/2.5 01 = 3/3.5 10 = 4/4.5 11 = 5/5.5 Most DDR2 devices utilize a fixed value of 2/2.5. For non-standard SDRAMs, the user must ensure that the operational Write Latency is always greater than or equal to the ODT turn-on delay. For example, a DDR2 SDRAM with CAS latency set to 3 and CAS additive latency set to 0 has a Write Latency of 2. Thus 2/2.5 can be used, but not 3/3.5 or higher."]
        #[inline(always)]
        pub fn set_taond_taofd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Dtpr1 {
        #[inline(always)]
        fn default() -> Dtpr1 {
            Dtpr1(0)
        }
    }
    #[doc = "DRAM Timing Parameters Register 0-2 (DTPR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtpr2(pub u32);
    impl Dtpr2 {
        #[doc = "Self refresh exit delay. The minimum time between a self refresh exit command and any other command. This parameter must be set to the maximum of the various minimum self refresh exit delay parameters specified in the SDRAM datasheet, i.e. max(tXSNR, tXSRD) for DDR2 and max(tXS, tXSDLL) for DDR3. Valid values are 2 to 1023."]
        #[inline(always)]
        pub const fn txs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Self refresh exit delay. The minimum time between a self refresh exit command and any other command. This parameter must be set to the maximum of the various minimum self refresh exit delay parameters specified in the SDRAM datasheet, i.e. max(tXSNR, tXSRD) for DDR2 and max(tXS, tXSDLL) for DDR3. Valid values are 2 to 1023."]
        #[inline(always)]
        pub fn set_txs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Power down exit delay. The minimum time between a power down exit command and any other command. This parameter must be set to the maximum of the various minimum power down exit delay parameters specified in the SDRAM datasheet, i.e. max(tXP, tXARD, tXARDS) for DDR2 and max(tXP, tXPDLL) for DDR3. Valid values are 2 to 31."]
        #[inline(always)]
        pub const fn txp(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "Power down exit delay. The minimum time between a power down exit command and any other command. This parameter must be set to the maximum of the various minimum power down exit delay parameters specified in the SDRAM datasheet, i.e. max(tXP, tXARD, tXARDS) for DDR2 and max(tXP, tXPDLL) for DDR3. Valid values are 2 to 31."]
        #[inline(always)]
        pub fn set_txp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "CKE minimum pulse width. Also specifies the minimum time that the SDRAM must remain in power down or self refresh mode. For DDR3 this parameter must be set to the value of tCKESR which is usually bigger than the value of tCKE. Valid values are 2 to 15."]
        #[inline(always)]
        pub const fn tcke(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x0f;
            val as u8
        }
        #[doc = "CKE minimum pulse width. Also specifies the minimum time that the SDRAM must remain in power down or self refresh mode. For DDR3 this parameter must be set to the value of tCKESR which is usually bigger than the value of tCKE. Valid values are 2 to 15."]
        #[inline(always)]
        pub fn set_tcke(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
        }
        #[doc = "DLL locking time. Valid values are 2 to 1023."]
        #[inline(always)]
        pub const fn tdllk(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "DLL locking time. Valid values are 2 to 1023."]
        #[inline(always)]
        pub fn set_tdllk(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "Read to ODT delay (DDR3 only). Specifies whether ODT can be enabled immediately after the read post-amble or one clock delay has to be added. Valid values are: 0 = ODT may be turned on immediately after read post-amble 1 = ODT may not be turned on until one clock after the read post-amble If tRTODT is set to 1, then the read-to-write latency is increased by 1 if ODT is enabled."]
        #[inline(always)]
        pub const fn trtodt(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Read to ODT delay (DDR3 only). Specifies whether ODT can be enabled immediately after the read post-amble or one clock delay has to be added. Valid values are: 0 = ODT may be turned on immediately after read post-amble 1 = ODT may not be turned on until one clock after the read post-amble If tRTODT is set to 1, then the read-to-write latency is increased by 1 if ODT is enabled."]
        #[inline(always)]
        pub fn set_trtodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Read to Write command delay. Valid values are: 0 = standard bus turn around delay 1 = add 1 clock to standard bus turn around delay This parameter allows the user to increase the delay between issuing Write commands to the SDRAM when preceded by Read commands. This provides an option to increase bus turn-around margin for high frequency systems."]
        #[inline(always)]
        pub const fn trtw(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Read to Write command delay. Valid values are: 0 = standard bus turn around delay 1 = add 1 clock to standard bus turn around delay This parameter allows the user to increase the delay between issuing Write commands to the SDRAM when preceded by Read commands. This provides an option to increase bus turn-around margin for high frequency systems."]
        #[inline(always)]
        pub fn set_trtw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Read to read and write to write command delay. Valid values are: 0 = BL/2 for DDR2 and 4 for DDR3 1 = BL/2 + 1 for DDR2 and 5 for DDR3."]
        #[inline(always)]
        pub const fn tccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Read to read and write to write command delay. Valid values are: 0 = BL/2 for DDR2 and 4 for DDR3 1 = BL/2 + 1 for DDR2 and 5 for DDR3."]
        #[inline(always)]
        pub fn set_tccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dtpr2 {
        #[inline(always)]
        fn default() -> Dtpr2 {
            Dtpr2(0)
        }
    }
    #[doc = "“DATX8 Common Configuration Register (DXCCR)” on page 99."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dxccr(pub u32);
    impl Dxccr {
        #[doc = "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the ODT configuration bit of the individual DATX8 (“DATX8 General Configuration Register (DXnGCR)” on page 148)."]
        #[inline(always)]
        pub const fn dxodt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the ODT configuration bit of the individual DATX8 (“DATX8 General Configuration Register (DXnGCR)” on page 148)."]
        #[inline(always)]
        pub fn set_dxodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the IOM configuration bit of the individual DATX8."]
        #[inline(always)]
        pub const fn dxiom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the IOM configuration bit of the individual DATX8."]
        #[inline(always)]
        pub fn set_dxiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Master Delay Line Enable: Enables, if set, all DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the MDLEN bit in the individual DATX8."]
        #[inline(always)]
        pub const fn mdlen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Master Delay Line Enable: Enables, if set, all DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the MDLEN bit in the individual DATX8."]
        #[inline(always)]
        pub fn set_mdlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDD configuration bit of the individual DATX8."]
        #[inline(always)]
        pub const fn dxpdd1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDD configuration bit of the individual DATX8."]
        #[inline(always)]
        pub fn set_dxpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDR configuration bit of the individual DATX8."]
        #[inline(always)]
        pub const fn dxpdr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDR configuration bit of the individual DATX8."]
        #[inline(always)]
        pub fn set_dxpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DQS Resistor: Selects the on-die pull-down/pull-up resistor for DQS pins. DQSRES\\[3\\]
selects pull-down (when set to 0) or pull-up (when set to 1). DQSRES\\[2:0\\]
selects the resistor value. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b."]
        #[inline(always)]
        pub const fn dqsres(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "DQS Resistor: Selects the on-die pull-down/pull-up resistor for DQS pins. DQSRES\\[3\\]
selects pull-down (when set to 0) or pull-up (when set to 1). DQSRES\\[2:0\\]
selects the resistor value. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b."]
        #[inline(always)]
        pub fn set_dqsres(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "DQS# Resistor: Selects the on-die pull-up/pull-down resistor for DQS# pins. Same encoding as DQSRES. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b."]
        #[inline(always)]
        pub const fn dqsnres(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "DQS# Resistor: Selects the on-die pull-up/pull-down resistor for DQS# pins. Same encoding as DQSRES. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b."]
        #[inline(always)]
        pub fn set_dqsnres(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "Data Slew Rate (D3F I/O Only): Selects slew rate of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros."]
        #[inline(always)]
        pub const fn dxsr(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Data Slew Rate (D3F I/O Only): Selects slew rate of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros."]
        #[inline(always)]
        pub fn set_dxsr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Most Significant Byte Unused DQs: Specifies the number of DQ bits that are not used in the most significant byte. The used (valid) bits for this byte are \\[8-MSBDQ- 1:0\\]. To disable the whole byte, use the DXnGCR.DXEN register."]
        #[inline(always)]
        pub const fn msbudq(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "Most Significant Byte Unused DQs: Specifies the number of DQ bits that are not used in the most significant byte. The used (valid) bits for this byte are \\[8-MSBDQ- 1:0\\]. To disable the whole byte, use the DXnGCR.DXEN register."]
        #[inline(always)]
        pub fn set_msbudq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "Unused DQ On-Die Termination: Enables, when set, the on-die termination on the I/O for unused DQ pins."]
        #[inline(always)]
        pub const fn udqodt(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Unused DQ On-Die Termination: Enables, when set, the on-die termination on the I/O for unused DQ pins."]
        #[inline(always)]
        pub fn set_udqodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Unused DQ Power Down Driver: Powers down, when set, the output driver on the I/O for unused DQ pins."]
        #[inline(always)]
        pub const fn udqpdd1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Unused DQ Power Down Driver: Powers down, when set, the output driver on the I/O for unused DQ pins."]
        #[inline(always)]
        pub fn set_udqpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Unused DQ Power Down Receiver: Powers down, when set, the input receiver on the I/O for unused DQ pins."]
        #[inline(always)]
        pub const fn udqpdr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Unused DQ Power Down Receiver: Powers down, when set, the input receiver on the I/O for unused DQ pins."]
        #[inline(always)]
        pub fn set_udqpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Unused DQ I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for unused DQ pins."]
        #[inline(always)]
        pub const fn udqiom(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Unused DQ I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for unused DQ pins."]
        #[inline(always)]
        pub fn set_udqiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Dynamic Data Power Down Driver: Dynamically powers down, when set, the output driver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDD and DXnGCR.DXPDD are not set to 1. Driver is powered-up on a DFI WRITE command and powered-down (twrlat + WL/2 + n) HDR cycles after the last DFI WRITE command. Note that n is defined by the register bit field DXCCR\\[27:24\\]
(DDPDDCDO)."]
        #[inline(always)]
        pub const fn dyndxpdd1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic Data Power Down Driver: Dynamically powers down, when set, the output driver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDD and DXnGCR.DXPDD are not set to 1. Driver is powered-up on a DFI WRITE command and powered-down (twrlat + WL/2 + n) HDR cycles after the last DFI WRITE command. Note that n is defined by the register bit field DXCCR\\[27:24\\]
(DDPDDCDO)."]
        #[inline(always)]
        pub fn set_dyndxpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Data Power Down Receiver: Dynamically powers down, when set, the input receiver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDR and DXnGCR.DXPDR are not set to 1. Receiver is powered-up on a DFI READ command and powered-down (trddata_en + fixed_read_latency + n) HDR cycles after the last DFI READ command. Note that n is defined by the register bit field DXCCR\\[31:28\\]
(DDPDRCDO)."]
        #[inline(always)]
        pub const fn dyndxpdr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Data Power Down Receiver: Dynamically powers down, when set, the input receiver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDR and DXnGCR.DXPDR are not set to 1. Receiver is powered-up on a DFI READ command and powered-down (trddata_en + fixed_read_latency + n) HDR cycles after the last DFI READ command. Note that n is defined by the register bit field DXCCR\\[31:28\\]
(DDPDRCDO)."]
        #[inline(always)]
        pub fn set_dyndxpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Dynamic Data Power Down Driver Count Down Offset: Offset applied in calculating window of time where driver is powered up."]
        #[inline(always)]
        pub const fn ddpddcdo(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Dynamic Data Power Down Driver Count Down Offset: Offset applied in calculating window of time where driver is powered up."]
        #[inline(always)]
        pub fn set_ddpddcdo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Dynamic Data Power Down Receiver Count Down Offset: Offset applied in calculating window of time where receiver is powered up."]
        #[inline(always)]
        pub const fn ddpdrcdo(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Dynamic Data Power Down Receiver Count Down Offset: Offset applied in calculating window of time where receiver is powered up."]
        #[inline(always)]
        pub fn set_ddpdrcdo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Dxccr {
        #[inline(always)]
        fn default() -> Dxccr {
            Dxccr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Emr(pub u32);
    impl Emr {
        #[doc = "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation."]
        #[inline(always)]
        pub const fn de(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation."]
        #[inline(always)]
        pub fn set_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output Driver Impedance Control: Controls the output drive strength. Valid values are: 0 = Full strength 1 = Reduced strength."]
        #[inline(always)]
        pub const fn dic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output Driver Impedance Control: Controls the output drive strength. Valid values are: 0 = Full strength 1 = Reduced strength."]
        #[inline(always)]
        pub fn set_dic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "On Die Termination low bit:."]
        #[inline(always)]
        pub const fn rttl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "On Die Termination low bit:."]
        #[inline(always)]
        pub fn set_rttl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 000 = 0 001 = 1 010 = 2 011 = 3 100 = 4 101 = 5 All other settings are reserved and should not be used. The maximum allowed value of AL is tRCD-1."]
        #[inline(always)]
        pub const fn al(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 000 = 0 001 = 1 010 = 2 011 = 3 100 = 4 101 = 5 All other settings are reserved and should not be used. The maximum allowed value of AL is tRCD-1."]
        #[inline(always)]
        pub fn set_al(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "On Die Termination high bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 00 = ODT disabled 01 = 75\u{f057} 10 = 150\u{f057} 11 = 50\u{f057} (some vendors)."]
        #[inline(always)]
        pub const fn rtth(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "On Die Termination high bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 00 = ODT disabled 01 = 75\u{f057} 10 = 150\u{f057} 11 = 50\u{f057} (some vendors)."]
        #[inline(always)]
        pub fn set_rtth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Off-Chip Driver (OCD) Impedance Calibration: Used to calibrate and match pull-up to pull- down impedance to 18 \u{f057} nominal (refer to the SDRAM datasheet for details). Valid values are: 000 = OCD calibration mode exit 001 = Drive (1) pull-up 010 = Drive (0) pull-down 100 = OCD enter adjust mode 111 = OCD calibration default All other settings are reserved and should not be used. Note that OCD is not supported by all vendors. Refer to the SDRAM datasheet for details on the recommended OCD settings."]
        #[inline(always)]
        pub const fn ocd(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x07;
            val as u8
        }
        #[doc = "Off-Chip Driver (OCD) Impedance Calibration: Used to calibrate and match pull-up to pull- down impedance to 18 \u{f057} nominal (refer to the SDRAM datasheet for details). Valid values are: 000 = OCD calibration mode exit 001 = Drive (1) pull-up 010 = Drive (0) pull-down 100 = OCD enter adjust mode 111 = OCD calibration default All other settings are reserved and should not be used. Note that OCD is not supported by all vendors. Refer to the SDRAM datasheet for details on the recommended OCD settings."]
        #[inline(always)]
        pub fn set_ocd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
        }
        #[doc = "DQS_b Enable/Disable: When ‘0’, DQS_b is the complement of the differential data strobe pair DQS/DQS_b. When ‘1’, DQS is used in a single-ended mode and the DQS_b pin is disabled. Also used to similarly enable/disable RDQS_b if RDQS is enabled. The Controller does not allow the user to change this bit."]
        #[inline(always)]
        pub const fn dqs(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DQS_b Enable/Disable: When ‘0’, DQS_b is the complement of the differential data strobe pair DQS/DQS_b. When ‘1’, DQS is used in a single-ended mode and the DQS_b pin is disabled. Also used to similarly enable/disable RDQS_b if RDQS is enabled. The Controller does not allow the user to change this bit."]
        #[inline(always)]
        pub fn set_dqs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RDQS Enable/Disable: When enabled (‘1’), RDQS is identical in function and timing to data strobe DQS during a read, and ignored during a write. A ‘0’ disables the SDRAM from driving RDQS. The Controller does not allow the user to change this bit."]
        #[inline(always)]
        pub const fn rdqs(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RDQS Enable/Disable: When enabled (‘1’), RDQS is identical in function and timing to data strobe DQS during a read, and ignored during a write. A ‘0’ disables the SDRAM from driving RDQS. The Controller does not allow the user to change this bit."]
        #[inline(always)]
        pub fn set_rdqs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation."]
        #[inline(always)]
        pub const fn qoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation."]
        #[inline(always)]
        pub fn set_qoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Emr {
        #[inline(always)]
        fn default() -> Emr {
            Emr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Emr2(pub u32);
    impl Emr2 {
        #[doc = "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA\\[1:0\\]
= 00 & 01) 010 = Quarter Array (BA\\[1:0\\]
= 00) 011 = Not defined 100 = 3/4 Array (BA\\[1:0\\]
= 01, 10, & 11) 101 = Half Array (BA\\[1:0\\]
= 10 & 11) 110 = Quarter Array (BA\\[1:0\\]
= 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA\\[2:0\\]
= 000, 001, 010 & 011) 010 = Quarter Array (BA\\[2:0\\]
= 000, 001) 011 = 1/8 Array (BA\\[2:0\\]
= 000) 100 = 3/4 Array (BA\\[2:0\\]
= 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA\\[2:0\\]
= 100, 101, 110 & 111) 110 = Quarter Array (BA\\[2:0\\]
= 110 & 111) 111 = 1/8 Array (BA\\[2:0\\]
111)."]
        #[inline(always)]
        pub const fn pasr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA\\[1:0\\]
= 00 & 01) 010 = Quarter Array (BA\\[1:0\\]
= 00) 011 = Not defined 100 = 3/4 Array (BA\\[1:0\\]
= 01, 10, & 11) 101 = Half Array (BA\\[1:0\\]
= 10 & 11) 110 = Quarter Array (BA\\[1:0\\]
= 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA\\[2:0\\]
= 000, 001, 010 & 011) 010 = Quarter Array (BA\\[2:0\\]
= 000, 001) 011 = 1/8 Array (BA\\[2:0\\]
= 000) 100 = 3/4 Array (BA\\[2:0\\]
= 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA\\[2:0\\]
= 100, 101, 110 & 111) 110 = Quarter Array (BA\\[2:0\\]
= 110 & 111) 111 = 1/8 Array (BA\\[2:0\\]
111)."]
        #[inline(always)]
        pub fn set_pasr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Duty Cycle Corrector: Enables, if set, duty cycle correction within SDRAM."]
        #[inline(always)]
        pub const fn dcc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Duty Cycle Corrector: Enables, if set, duty cycle correction within SDRAM."]
        #[inline(always)]
        pub fn set_dcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Self Refresh Rate: Enables, if set, high temperature self refresh rate."]
        #[inline(always)]
        pub const fn srf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Self Refresh Rate: Enables, if set, high temperature self refresh rate."]
        #[inline(always)]
        pub fn set_srf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Emr2 {
        #[inline(always)]
        fn default() -> Emr2 {
            Emr2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Emr3(pub u32);
    impl Emr3 {}
    impl Default for Emr3 {
        #[inline(always)]
        fn default() -> Emr3 {
            Emr3(0)
        }
    }
    #[doc = "“DATX8 General Configuration Register (DXnGCR)” on page 148."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "Data Byte Enable: Enables, if set, the data byte. Setting this bit to '0' disables the byte, i.e. the byte is not used in PHY initialization or training and is ignored during SDRAM read/write operations."]
        #[inline(always)]
        pub const fn dxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Byte Enable: Enables, if set, the data byte. Setting this bit to '0' disables the byte, i.e. the byte is not used in PHY initialization or training and is ignored during SDRAM read/write operations."]
        #[inline(always)]
        pub fn set_dxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DQS On-Die Termination: Enables, when set, the on-die termination on the I/O for DQS/DQS# pin of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0\\[9\\]
is '0'."]
        #[inline(always)]
        pub const fn dqsodt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DQS On-Die Termination: Enables, when set, the on-die termination on the I/O for DQS/DQS# pin of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0\\[9\\]
is '0'."]
        #[inline(always)]
        pub fn set_dqsodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ and DM pins of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0\\[10\\]
is '0'."]
        #[inline(always)]
        pub const fn dqodt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ and DM pins of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0\\[10\\]
is '0'."]
        #[inline(always)]
        pub fn set_dqodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the IOM configuration bit of the individual DATX8(see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub const fn dxiom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the IOM configuration bit of the individual DATX8(see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub fn set_dxiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDD configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub const fn dxpdd1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDD configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub fn set_dxpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub const fn dxpdr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub fn set_dxpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DQSR Power Down: Powers down, if set, the PDQSR cell. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub const fn dqsrpd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DQSR Power Down: Powers down, if set, the PDQSR cell. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99)."]
        #[inline(always)]
        pub fn set_dqsrpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Write DQS Enable: Controls whether the write DQS going to the SDRAM is enabled (toggling) or disabled (static value) and whether the DQS is inverted. DQS# is always the inversion of DQS. These values are valid only when DQS/DQS# output enable is on, otherwise the DQS/DQS# is tristated. Valid settings are: 00 = Reserved 01 = DQS toggling with normal polarity (This should be the default setting) 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub const fn dsen(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Write DQS Enable: Controls whether the write DQS going to the SDRAM is enabled (toggling) or disabled (static value) and whether the DQS is inverted. DQS# is always the inversion of DQS. These values are valid only when DQS/DQS# output enable is on, otherwise the DQS/DQS# is tristated. Valid settings are: 00 = Reserved 01 = DQS toggling with normal polarity (This should be the default setting) 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub fn set_dsen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "DQS Dynamic RTT Control: If set, the on die termination (ODT) control of the DQS/DQS# SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0\\[1\\]
(DQSODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0')."]
        #[inline(always)]
        pub const fn dqsrtt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Dynamic RTT Control: If set, the on die termination (ODT) control of the DQS/DQS# SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0\\[1\\]
(DQSODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0')."]
        #[inline(always)]
        pub fn set_dqsrtt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DQ Dynamic RTT Control: If set, the on die termination (ODT) control of the DQ/DM SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0\\[2\\]
(DQODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0')."]
        #[inline(always)]
        pub const fn dqrtt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DQ Dynamic RTT Control: If set, the on die termination (ODT) control of the DQ/DM SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0\\[2\\]
(DQODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0')."]
        #[inline(always)]
        pub fn set_dqrtt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RTT Output Hold: Indicates the number of clock cycles (from 0 to 3) after the read data postamble for which ODT control should remain set to DQSODT for DQS or DQODT for DQ/DM before disabling it (setting it to ‘0’) when using dynamic ODT control. ODT is disabled almost RTTOH clock cycles after the read postamble."]
        #[inline(always)]
        pub const fn rttoh(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "RTT Output Hold: Indicates the number of clock cycles (from 0 to 3) after the read data postamble for which ODT control should remain set to DQSODT for DQS or DQODT for DQ/DM before disabling it (setting it to ‘0’) when using dynamic ODT control. ODT is disabled almost RTTOH clock cycles after the read postamble."]
        #[inline(always)]
        pub fn set_rttoh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "RTT On Additive Latency: Indicates when the ODT control of DQ/DQS SSTL I/Os is set to the value in DQODT/DQSODT during read cycles. Valid values are: 0 = ODT control is set to DQSODT/DQODT almost two cycles before read data preamble 1 = ODT control is set to DQSODT/DQODT almost one cycle before read data preamble."]
        #[inline(always)]
        pub const fn rttoal(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RTT On Additive Latency: Indicates when the ODT control of DQ/DQS SSTL I/Os is set to the value in DQODT/DQSODT during read cycles. Valid values are: 0 = ODT control is set to DQSODT/DQODT almost two cycles before read data preamble 1 = ODT control is set to DQSODT/DQODT almost one cycle before read data preamble."]
        #[inline(always)]
        pub fn set_rttoal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Data Byte Output Enable Override: Specifies whether the output I/O output enable for the byte lane should be set to a fixed value. Valid values are: 00 = No override. Output enable is controlled by DFI transactions 01 = output enable is asserted (I/O is forced to output mode). 10 = Output enable is de-asserted (I/O is forced to input mode) 11 = Reserved."]
        #[inline(always)]
        pub const fn dxoeo(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Data Byte Output Enable Override: Specifies whether the output I/O output enable for the byte lane should be set to a fixed value. Valid values are: 00 = No override. Output enable is controlled by DFI transactions 01 = output enable is asserted (I/O is forced to output mode). 10 = Output enable is de-asserted (I/O is forced to input mode) 11 = Reserved."]
        #[inline(always)]
        pub fn set_dxoeo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "PLL Rest: Resets the byte PLL by driving the PLL reset pin. This bit is not self- clearing and a '0' must be written to de-assert the reset. This bit is ORed with the global PLLRST configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub const fn pllrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Rest: Resets the byte PLL by driving the PLL reset pin. This bit is not self- clearing and a '0' must be written to de-assert the reset. This bit is ORed with the global PLLRST configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub fn set_pllrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL Power Down: Puts the byte PLL in power down mode by driving the PLL power down pin. This bit is not self-clearing and a '0' must be written to de-assert the power-down. This bit is ORed with the global PLLPD configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub const fn pllpd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Power Down: Puts the byte PLL in power down mode by driving the PLL power down pin. This bit is not self-clearing and a '0' must be written to de-assert the power-down. This bit is ORed with the global PLLPD configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub fn set_pllpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Gear Shift: Enables, if set, rapid locking mode on the byte PLL. This bit is ORed with the global GSHIFT configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub const fn gshift(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Gear Shift: Enables, if set, rapid locking mode on the byte PLL. This bit is ORed with the global GSHIFT configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub fn set_gshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PLL Bypass: Puts the byte PLL in bypass mode by driving the PLL bypass pin. This bit is not self-clearing and a '0' must be written to de-assert the bypass. This bit is ORed with the global BYP configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub const fn pllbyp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Bypass: Puts the byte PLL in bypass mode by driving the PLL bypass pin. This bit is not self-clearing and a '0' must be written to de-assert the bypass. This bit is ORed with the global BYP configuration bit (see Table 3-10 on page 91)."]
        #[inline(always)]
        pub fn set_pllbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Write Level Rank Enable: Specifies the ranks that should be write leveled for this byte. Write leveling responses from ranks that are not enabled for write leveling for a particular byte are ignored and write leveling is flagged as done for these ranks. WLRKEN\\[0\\]
enables rank 0, \\[1\\]
enables rank 1, \\[2\\]
enables rank 2, and \\[3\\]
enables rank 3."]
        #[inline(always)]
        pub const fn wlrken(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x0f;
            val as u8
        }
        #[doc = "Write Level Rank Enable: Specifies the ranks that should be write leveled for this byte. Write leveling responses from ranks that are not enabled for write leveling for a particular byte are ignored and write leveling is flagged as done for these ranks. WLRKEN\\[0\\]
enables rank 0, \\[1\\]
enables rank 1, \\[2\\]
enables rank 2, and \\[3\\]
enables rank 3."]
        #[inline(always)]
        pub fn set_wlrken(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
        }
        #[doc = "Master Delay Line Enable: Enables, if set, the DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the common DATX8 MDL enable bit."]
        #[inline(always)]
        pub const fn mdlen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Master Delay Line Enable: Enables, if set, the DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the common DATX8 MDL enable bit."]
        #[inline(always)]
        pub fn set_mdlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Calibration Bypass: Prevents, if set, period measurement calibration from automatically triggering after PHY initialization."]
        #[inline(always)]
        pub const fn calbyp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Bypass: Prevents, if set, period measurement calibration from automatically triggering after PHY initialization."]
        #[inline(always)]
        pub fn set_calbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "General Purpose Register 0-1 (GPR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr0(pub u32);
    impl Gpr0 {
        #[doc = "General Purpose Register 0: General purpose register bits."]
        #[inline(always)]
        pub const fn gpr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "General Purpose Register 0: General purpose register bits."]
        #[inline(always)]
        pub fn set_gpr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gpr0 {
        #[inline(always)]
        fn default() -> Gpr0 {
            Gpr0(0)
        }
    }
    #[doc = "General Purpose Register 0-1 (GPR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr1(pub u32);
    impl Gpr1 {
        #[doc = "General Purpose Register 1: General purpose register bits."]
        #[inline(always)]
        pub const fn gpr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "General Purpose Register 1: General purpose register bits."]
        #[inline(always)]
        pub fn set_gpr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gpr1 {
        #[inline(always)]
        fn default() -> Gpr1 {
            Gpr1(0)
        }
    }
    #[doc = "DATX8 General Status Registers 0-2 (DXnGSR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gsr0(pub u32);
    impl Gsr0 {
        #[doc = "Write DQ Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write DQ LCDL."]
        #[inline(always)]
        pub const fn wdqcal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write DQ Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write DQ LCDL."]
        #[inline(always)]
        pub fn set_wdqcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read DQS Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS LCDL."]
        #[inline(always)]
        pub const fn rdqscal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS LCDL."]
        #[inline(always)]
        pub fn set_rdqscal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Read DQS# Calibration (Type B/B1 PHY Only): Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS# LCDL."]
        #[inline(always)]
        pub const fn rdqsncal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS# Calibration (Type B/B1 PHY Only): Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS# LCDL."]
        #[inline(always)]
        pub fn set_rdqsncal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Read DQS gating Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS gating LCDL."]
        #[inline(always)]
        pub const fn gdqscal(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS gating Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS gating LCDL."]
        #[inline(always)]
        pub fn set_gdqscal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write Leveling Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write leveling slave delay line."]
        #[inline(always)]
        pub const fn wlcal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write leveling slave delay line."]
        #[inline(always)]
        pub fn set_wlcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Write Leveling Done: Indicates, if set, that the DATX8 has completed write leveling."]
        #[inline(always)]
        pub const fn wldone(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Done: Indicates, if set, that the DATX8 has completed write leveling."]
        #[inline(always)]
        pub fn set_wldone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Write Leveling Error: Indicates, if set, that there is a write leveling error in the DATX8."]
        #[inline(always)]
        pub const fn wlerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Error: Indicates, if set, that there is a write leveling error in the DATX8."]
        #[inline(always)]
        pub fn set_wlerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Write Leveling Period: Returns the DDR clock period measured by the write leveling LCDL during calibration. The measured period is used to generate the control of the write leveling pipeline which is a function of the write-leveling delay and the clock period. This value is PVT compensated."]
        #[inline(always)]
        pub const fn wlprd(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0xff;
            val as u8
        }
        #[doc = "Write Leveling Period: Returns the DDR clock period measured by the write leveling LCDL during calibration. The measured period is used to generate the control of the write leveling pipeline which is a function of the write-leveling delay and the clock period. This value is PVT compensated."]
        #[inline(always)]
        pub fn set_wlprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 7usize)) | (((val as u32) & 0xff) << 7usize);
        }
        #[doc = "DATX8 PLL Lock: Indicates, if set, that the DATX8 PLL has locked. This is a direct status of the DATX8 PLL lock pin."]
        #[inline(always)]
        pub const fn dplock(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DATX8 PLL Lock: Indicates, if set, that the DATX8 PLL has locked. This is a direct status of the DATX8 PLL lock pin."]
        #[inline(always)]
        pub fn set_dplock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Read DQS gating Period: Returns the DDR clock period measured by the read DQS gating LCDL during calibration. This value is PVT compensated."]
        #[inline(always)]
        pub const fn gdqsprd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Read DQS gating Period: Returns the DDR clock period measured by the read DQS gating LCDL during calibration. This value is PVT compensated."]
        #[inline(always)]
        pub fn set_gdqsprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training. One bit for each of the up to 4 ranks."]
        #[inline(always)]
        pub const fn qsgerr(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training. One bit for each of the up to 4 ranks."]
        #[inline(always)]
        pub fn set_qsgerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Write Leveling DQ Status: Captures the write leveling DQ status from the DRAM during software write leveling."]
        #[inline(always)]
        pub const fn wldq(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling DQ Status: Captures the write leveling DQ status from the DRAM during software write leveling."]
        #[inline(always)]
        pub fn set_wldq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Gsr0 {
        #[inline(always)]
        fn default() -> Gsr0 {
            Gsr0(0)
        }
    }
    #[doc = "DATX8 General Status Registers 0-2 (DXnGSR0-2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gsr1(pub u32);
    impl Gsr1 {
        #[doc = "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the DATX8 delay line digital test output."]
        #[inline(always)]
        pub const fn dltdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the DATX8 delay line digital test output."]
        #[inline(always)]
        pub fn set_dltdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the DATX8 delay line digital test output."]
        #[inline(always)]
        pub const fn dltcode(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the DATX8 delay line digital test output."]
        #[inline(always)]
        pub fn set_dltcode(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 1usize)) | (((val as u32) & 0x00ff_ffff) << 1usize);
        }
    }
    impl Default for Gsr1 {
        #[inline(always)]
        fn default() -> Gsr1 {
            Gsr1(0)
        }
    }
    #[doc = "“DATX8 General Status Register 2 (DXnGSR2)” on page 152."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gsr2(pub u32);
    impl Gsr2 {
        #[doc = "Read Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read bit deskew training."]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Read Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read bit deskew training."]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read bit deskew training."]
        #[inline(always)]
        pub const fn rdwn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read bit deskew training."]
        #[inline(always)]
        pub fn set_rdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write bit deskew training."]
        #[inline(always)]
        pub const fn wderr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write bit deskew training."]
        #[inline(always)]
        pub fn set_wderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Write Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write bit deskew training."]
        #[inline(always)]
        pub const fn wdwn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Write Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write bit deskew training."]
        #[inline(always)]
        pub fn set_wdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Read Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read data eye training."]
        #[inline(always)]
        pub const fn reerr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read data eye training."]
        #[inline(always)]
        pub fn set_reerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read data eye training."]
        #[inline(always)]
        pub const fn rewn(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read data eye training."]
        #[inline(always)]
        pub fn set_rewn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Write Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write data eye training."]
        #[inline(always)]
        pub const fn weerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write data eye training."]
        #[inline(always)]
        pub fn set_weerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Write Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write data eye training."]
        #[inline(always)]
        pub const fn wewn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write data eye training."]
        #[inline(always)]
        pub fn set_wewn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Error Status: If an error occurred for this lane as indicated by RDERR, WDERR, REERR or WEERR the error status code can provide additional information regard when the error occurred during the algorithm execution."]
        #[inline(always)]
        pub const fn estat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Error Status: If an error occurred for this lane as indicated by RDERR, WDERR, REERR or WEERR the error status code can provide additional information regard when the error occurred during the algorithm execution."]
        #[inline(always)]
        pub fn set_estat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Gsr2 {
        #[inline(always)]
        fn default() -> Gsr2 {
            Gsr2(0)
        }
    }
    #[doc = "“DATX8 General Timing Register (DXnGTR)” on page 159."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtr(pub u32);
    impl Gtr {
        #[doc = "Rank n DQS Gating System Latency: This is used to increase the number of clock cycles needed to expect valid DDR read data by up to seven extra clock cycles. This is used to compensate for board delays and other system delays. Power-up default is 000 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic DQS data training but these values can be overwritten by a direct write to this register. Every three bits of this register control the latency of each of the (up to) four ranks. R0DGSL controls the latency of rank 0, R1DGSL controls rank 1, and so on. Valid values are 0 to 7:."]
        #[inline(always)]
        pub const fn r0dgsl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Rank n DQS Gating System Latency: This is used to increase the number of clock cycles needed to expect valid DDR read data by up to seven extra clock cycles. This is used to compensate for board delays and other system delays. Power-up default is 000 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic DQS data training but these values can be overwritten by a direct write to this register. Every three bits of this register control the latency of each of the (up to) four ranks. R0DGSL controls the latency of rank 0, R1DGSL controls rank 1, and so on. Valid values are 0 to 7:."]
        #[inline(always)]
        pub fn set_r0dgsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r1dgsl(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r1dgsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r2dgsl(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r2dgsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r3dgsl(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r3dgsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Rank n Write Leveling System Latency: This is used to adjust the write latency after write leveling. Power-up default is 01 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic write leveling but these values can be overwritten by a direct write to this register. Every two bits of this register control the latency of each of the (up to) four ranks. R0WLSL controls the latency of rank 0, R1WLSL controls rank 1, and so on. Valid values: 00 = Write latency = WL - 1 01 = Write latency = WL 10 = Write latency = WL + 1 11 = Reserved."]
        #[inline(always)]
        pub const fn r0wlsl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Rank n Write Leveling System Latency: This is used to adjust the write latency after write leveling. Power-up default is 01 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic write leveling but these values can be overwritten by a direct write to this register. Every two bits of this register control the latency of each of the (up to) four ranks. R0WLSL controls the latency of rank 0, R1WLSL controls rank 1, and so on. Valid values: 00 = Write latency = WL - 1 01 = Write latency = WL 10 = Write latency = WL + 1 11 = Reserved."]
        #[inline(always)]
        pub fn set_r0wlsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r1wlsl(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r1wlsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r2wlsl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r2wlsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn r3wlsl(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_r3wlsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for Gtr {
        #[inline(always)]
        fn default() -> Gtr {
            Gtr(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcdlr0(pub u32);
    impl Lcdlr0 {
        #[doc = "Rank 0 Write Leveling Delay: Rank 0 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub const fn r0wld(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 0 Write Leveling Delay: Rank 0 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub fn set_r0wld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Rank 1 Write Leveling Delay: Rank 1 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub const fn r1wld(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 1 Write Leveling Delay: Rank 1 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub fn set_r1wld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Rank 2 Write Leveling Delay: Rank 2 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub const fn r2wld(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 2 Write Leveling Delay: Rank 2 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub fn set_r2wld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Rank 3 Write Leveling Delay: Rank 3 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub const fn r3wld(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 3 Write Leveling Delay: Rank 3 delay select for the write leveling (WL) LCDL."]
        #[inline(always)]
        pub fn set_r3wld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Lcdlr0 {
        #[inline(always)]
        fn default() -> Lcdlr0 {
            Lcdlr0(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcdlr1(pub u32);
    impl Lcdlr1 {
        #[doc = "Write Data Delay: Delay select for the write data (WDQ) LCDL."]
        #[inline(always)]
        pub const fn wdqd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write Data Delay: Delay select for the write data (WDQ) LCDL."]
        #[inline(always)]
        pub fn set_wdqd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Read DQS Delay: Delay select for the read DQS (RDQS) LCDL."]
        #[inline(always)]
        pub const fn rdqsd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Read DQS Delay: Delay select for the read DQS (RDQS) LCDL."]
        #[inline(always)]
        pub fn set_rdqsd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Read DQSN Delay (Type B/B1 PHY Only): Delay select for the read DQSN (RDQS) LCDL."]
        #[inline(always)]
        pub const fn rdqsnd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Read DQSN Delay (Type B/B1 PHY Only): Delay select for the read DQSN (RDQS) LCDL."]
        #[inline(always)]
        pub fn set_rdqsnd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lcdlr1 {
        #[inline(always)]
        fn default() -> Lcdlr1 {
            Lcdlr1(0)
        }
    }
    #[doc = "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcdlr2(pub u32);
    impl Lcdlr2 {
        #[doc = "Rank 0 Read DQS Gating Delay: Rank 0 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub const fn r0dqsgd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 0 Read DQS Gating Delay: Rank 0 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub fn set_r0dqsgd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Rank 1 Read DQS Gating Delay: Rank 1 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub const fn r1dqsgd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 1 Read DQS Gating Delay: Rank 1 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub fn set_r1dqsgd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Rank 2 Read DQS Gating Delay: Rank 2 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub const fn r2dqsgd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 2 Read DQS Gating Delay: Rank 2 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub fn set_r2dqsgd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Rank 3 Read DQS Gating Delay: Rank 3 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub const fn r3dqsgd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Rank 3 Read DQS Gating Delay: Rank 3 delay select for the read DQS gating (DQSG) LCDL."]
        #[inline(always)]
        pub fn set_r3dqsgd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Lcdlr2 {
        #[inline(always)]
        fn default() -> Lcdlr2 {
            Lcdlr2(0)
        }
    }
    #[doc = "“DATX8 Master Delay Line Register (DXnMDLR)” on page 157."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mdlr(pub u32);
    impl Mdlr {
        #[doc = "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation."]
        #[inline(always)]
        pub const fn iprd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation."]
        #[inline(always)]
        pub fn set_iprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so."]
        #[inline(always)]
        pub const fn tprd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so."]
        #[inline(always)]
        pub fn set_tprd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "MDL Delay: Delay select for the LCDL for the Master Delay Line."]
        #[inline(always)]
        pub const fn mdld(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "MDL Delay: Delay select for the LCDL for the Master Delay Line."]
        #[inline(always)]
        pub fn set_mdld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Mdlr {
        #[inline(always)]
        fn default() -> Mdlr {
            Mdlr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mr(pub u32);
    impl Mr {
        #[doc = "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: 010 = 4 011 = 8 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: 010 = 4 011 = 8 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1)."]
        #[inline(always)]
        pub const fn bt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1)."]
        #[inline(always)]
        pub fn set_bt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 010 = 2 011 = 3 100 = 4 101 = 5 110 = 6 111 = 7 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub const fn cl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 010 = 2 011 = 3 100 = 4 101 = 5 110 = 6 111 = 7 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub fn set_cl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used."]
        #[inline(always)]
        pub const fn tm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used."]
        #[inline(always)]
        pub fn set_tm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued."]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued."]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 001 = 2 010 = 3 011 = 4 100 = 5 101 = 6 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued."]
        #[inline(always)]
        pub const fn wr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 001 = 2 010 = 3 011 = 4 100 = 5 101 = 6 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued."]
        #[inline(always)]
        pub fn set_wr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Fast exit 1 = Slow exit."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Fast exit 1 = Slow exit."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mr {
        #[inline(always)]
        fn default() -> Mr {
            Mr(0)
        }
    }
    #[doc = "“Mode Register 0 (MR0)” on page 108."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mr0(pub u32);
    impl Mr0 {
        #[doc = "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: Valid values for DDR3 are: 00 = 8 (Fixed) 01 = 4 or 8 (On the fly) 10 = 4 (Fixed) 11 = Reserved."]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: Valid values for DDR3 are: 00 = 8 (Fixed) 01 = 4 or 8 (On the fly) 10 = 4 (Fixed) 11 = Reserved."]
        #[inline(always)]
        pub fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CAS Latency low bit."]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CAS Latency low bit."]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1)."]
        #[inline(always)]
        pub const fn bt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1)."]
        #[inline(always)]
        pub fn set_bt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 0010 = 5 0100 = 6 0110 = 7 1000 = 8 1010 = 9 1100 = 10 1110 = 11 0001 = 12 0011 = 13 0101 = 14 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub const fn clh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 0010 = 5 0100 = 6 0110 = 7 1000 = 8 1010 = 9 1100 = 10 1110 = 11 0001 = 12 0011 = 13 0101 = 14 All other settings are reserved and should not be used."]
        #[inline(always)]
        pub fn set_clh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used."]
        #[inline(always)]
        pub const fn tm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used."]
        #[inline(always)]
        pub fn set_tm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued."]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued."]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 000 = 16 001 = 5 010 = 6 011 = 7 100 = 8 101 = 10 110 = 12 111 = 14 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued."]
        #[inline(always)]
        pub const fn wr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 000 = 16 001 = 5 010 = 6 011 = 7 100 = 8 101 = 10 110 = 12 111 = 14 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued."]
        #[inline(always)]
        pub fn set_wr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Slow exit (DLL off) 1 = Fast exit (DLL on)."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Slow exit (DLL off) 1 = Fast exit (DLL on)."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mr0 {
        #[inline(always)]
        fn default() -> Mr0 {
            Mr0(0)
        }
    }
    #[doc = "“Mode Register 1 (MR1)” on page 111."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mr1(pub u32);
    impl Mr1 {
        #[doc = "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation. Note: SDRAM DLL off mode is not supported."]
        #[inline(always)]
        pub const fn de(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation. Note: SDRAM DLL off mode is not supported."]
        #[inline(always)]
        pub fn set_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output Driver Impedance Control low bit."]
        #[inline(always)]
        pub const fn dicl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output Driver Impedance Control low bit."]
        #[inline(always)]
        pub fn set_dicl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "On Die Termination low bit."]
        #[inline(always)]
        pub const fn rttl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "On Die Termination low bit."]
        #[inline(always)]
        pub fn set_rttl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 00 = 0 (AL disabled) 01 = CL - 1 10 = CL - 2 11 = Reserved."]
        #[inline(always)]
        pub const fn al(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 00 = 0 (AL disabled) 01 = CL - 1 10 = CL - 2 11 = Reserved."]
        #[inline(always)]
        pub fn set_al(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Output Driver Impedance Control high bit: Controls the output drive strength. Valid values are: 00 = RZQ/6 01 = RZQ7 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub const fn dich(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Output Driver Impedance Control high bit: Controls the output drive strength. Valid values are: 00 = RZQ/6 01 = RZQ7 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub fn set_dich(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "On Die Termination mid bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 000 = ODT disabled 001 = RZQ/4 010 = RZQ/2 011 = RZQ/6 100 = RZQ/12 101 = RZQ/8 All other settings are reserved and should not be used. Bit on \\[9, 6,2\\]."]
        #[inline(always)]
        pub const fn rttm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "On Die Termination mid bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 000 = ODT disabled 001 = RZQ/4 010 = RZQ/2 011 = RZQ/6 100 = RZQ/12 101 = RZQ/8 All other settings are reserved and should not be used. Bit on \\[9, 6,2\\]."]
        #[inline(always)]
        pub fn set_rttm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Write Leveling Enable: Enables write-leveling when set."]
        #[inline(always)]
        pub const fn level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Enable: Enables write-leveling when set."]
        #[inline(always)]
        pub fn set_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "On Die Termination high bit."]
        #[inline(always)]
        pub const fn rtth(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "On Die Termination high bit."]
        #[inline(always)]
        pub fn set_rtth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Termination Data Strobe: When enabled (‘1’) TDQS provides additional termination resistance outputs that may be useful in some system configurations. Refer to the SDRAM datasheet for details."]
        #[inline(always)]
        pub const fn tdqs(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Termination Data Strobe: When enabled (‘1’) TDQS provides additional termination resistance outputs that may be useful in some system configurations. Refer to the SDRAM datasheet for details."]
        #[inline(always)]
        pub fn set_tdqs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation."]
        #[inline(always)]
        pub const fn qoff(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation."]
        #[inline(always)]
        pub fn set_qoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Mr1 {
        #[inline(always)]
        fn default() -> Mr1 {
            Mr1(0)
        }
    }
    #[doc = "“Mode Register 2/Extended Mode Register 2 (MR2/EMR2)” on page 114."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mr2(pub u32);
    impl Mr2 {
        #[doc = "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA\\[1:0\\]
= 00 & 01) 010 = Quarter Array (BA\\[1:0\\]
= 00) 011 = Not defined 100 = 3/4 Array (BA\\[1:0\\]
= 01, 10, & 11) 101 = Half Array (BA\\[1:0\\]
= 10 & 11) 110 = Quarter Array (BA\\[1:0\\]
= 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA\\[2:0\\]
= 000, 001, 010 & 011) 010 = Quarter Array (BA\\[2:0\\]
= 000, 001) 011 = 1/8 Array (BA\\[2:0\\]
= 000) 100 = 3/4 Array (BA\\[2:0\\]
= 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA\\[2:0\\]
= 100, 101, 110 & 111) 110 = Quarter Array (BA\\[2:0\\]
= 110 & 111) 111 = 1/8 Array (BA\\[2:0\\]
111)."]
        #[inline(always)]
        pub const fn pasr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA\\[1:0\\]
= 00 & 01) 010 = Quarter Array (BA\\[1:0\\]
= 00) 011 = Not defined 100 = 3/4 Array (BA\\[1:0\\]
= 01, 10, & 11) 101 = Half Array (BA\\[1:0\\]
= 10 & 11) 110 = Quarter Array (BA\\[1:0\\]
= 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA\\[2:0\\]
= 000, 001, 010 & 011) 010 = Quarter Array (BA\\[2:0\\]
= 000, 001) 011 = 1/8 Array (BA\\[2:0\\]
= 000) 100 = 3/4 Array (BA\\[2:0\\]
= 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA\\[2:0\\]
= 100, 101, 110 & 111) 110 = Quarter Array (BA\\[2:0\\]
= 110 & 111) 111 = 1/8 Array (BA\\[2:0\\]
111)."]
        #[inline(always)]
        pub fn set_pasr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "CAS Write Latency: The delay between when the SDRAM registers a write command to when write data is available. Valid values are: 000 = 5 (tCK > 2.5ns) 001 = 6 (2.5ns > tCK > 1.875ns) 010 = 7 (1.875ns > tCK> 1.5ns) 011 = 8 (1.5ns > tCK > 1.25ns) 100 = 9 (1.25ns > tCK > 1.07ns) 101 = 10 (1.07ns > tCK > 0.935ns) 110 = 11 (0.935ns > tCK > 0.833ns) 111 = 12 (0.833ns > tCK > 0.75ns) All other settings are reserved and should not be used."]
        #[inline(always)]
        pub const fn cwl(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "CAS Write Latency: The delay between when the SDRAM registers a write command to when write data is available. Valid values are: 000 = 5 (tCK > 2.5ns) 001 = 6 (2.5ns > tCK > 1.875ns) 010 = 7 (1.875ns > tCK> 1.5ns) 011 = 8 (1.5ns > tCK > 1.25ns) 100 = 9 (1.25ns > tCK > 1.07ns) 101 = 10 (1.07ns > tCK > 0.935ns) 110 = 11 (0.935ns > tCK > 0.833ns) 111 = 12 (0.833ns > tCK > 0.75ns) All other settings are reserved and should not be used."]
        #[inline(always)]
        pub fn set_cwl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Auto Self-Refresh: When enabled (‘1’), SDRAM automatically provides self-refresh power management functions for all supported operating temperature values. Otherwise the SRT bit must be programmed to indicate the temperature range."]
        #[inline(always)]
        pub const fn asr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Self-Refresh: When enabled (‘1’), SDRAM automatically provides self-refresh power management functions for all supported operating temperature values. Otherwise the SRT bit must be programmed to indicate the temperature range."]
        #[inline(always)]
        pub fn set_asr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Self-Refresh Temperature Range: Selects either normal (‘0’) or extended (‘1’) operating temperature range during self-refresh."]
        #[inline(always)]
        pub const fn srt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Self-Refresh Temperature Range: Selects either normal (‘0’) or extended (‘1’) operating temperature range during self-refresh."]
        #[inline(always)]
        pub fn set_srt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Dynamic ODT: Selects RTT for dynamic ODT. Valid values are: 00 = Dynamic ODT off 01 = RZQ/4 10 = RZQ/2 11 = Reserved."]
        #[inline(always)]
        pub const fn rttwr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Dynamic ODT: Selects RTT for dynamic ODT. Valid values are: 00 = Dynamic ODT off 01 = RZQ/4 10 = RZQ/2 11 = Reserved."]
        #[inline(always)]
        pub fn set_rttwr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
    }
    impl Default for Mr2 {
        #[inline(always)]
        fn default() -> Mr2 {
            Mr2(0)
        }
    }
    #[doc = "“Mode Register 3 (MR3)” on page 116."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mr3(pub u32);
    impl Mr3 {
        #[doc = "Multi-Purpose Register (MPR) Location: Selects MPR data location: Valid value are: 00 = Predefined pattern for system calibration All other settings are reserved and should not be used."]
        #[inline(always)]
        pub const fn mprloc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Multi-Purpose Register (MPR) Location: Selects MPR data location: Valid value are: 00 = Predefined pattern for system calibration All other settings are reserved and should not be used."]
        #[inline(always)]
        pub fn set_mprloc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Multi-Purpose Register Enable: Enables, if set, that read data should come from the Multi-Purpose Register. Otherwise read data come from the DRAM array."]
        #[inline(always)]
        pub const fn mpr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Purpose Register Enable: Enables, if set, that read data should come from the Multi-Purpose Register. Otherwise read data come from the DRAM array."]
        #[inline(always)]
        pub fn set_mpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Mr3 {
        #[inline(always)]
        fn default() -> Mr3 {
            Mr3(0)
        }
    }
    #[doc = "“ODT Configuration Register (ODTCR)” on page 117."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odtcr(pub u32);
    impl Odtcr {
        #[doc = "Read ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a read command is sent to rank n. RDODT0, RDODT1, RDODT2, and RDODT3 specify ODT settings when a read is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to disable ODT during reads."]
        #[inline(always)]
        pub const fn rdodt0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a read command is sent to rank n. RDODT0, RDODT1, RDODT2, and RDODT3 specify ODT settings when a read is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to disable ODT during reads."]
        #[inline(always)]
        pub fn set_rdodt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn rdodt1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_rdodt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn rdodt2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_rdodt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn rdodt3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_rdodt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Write ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a write command is sent to rank n. WRODT0, WRODT1, WRODT2, and WRODT3 specify ODT settings when a write is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to enable ODT only on rank being written to."]
        #[inline(always)]
        pub const fn wrodt0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Write ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a write command is sent to rank n. WRODT0, WRODT1, WRODT2, and WRODT3 specify ODT settings when a write is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to enable ODT only on rank being written to."]
        #[inline(always)]
        pub fn set_wrodt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn wrodt1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_wrodt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn wrodt2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_wrodt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn wrodt3(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_wrodt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Odtcr {
        #[inline(always)]
        fn default() -> Odtcr {
            Odtcr(0)
        }
    }
    #[doc = "PHY General Configuration Registers 0-1 (PGCR0- 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pgcr0(pub u32);
    impl Pgcr0 {
        #[doc = "Write Leveling LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write leveling LCDL."]
        #[inline(always)]
        pub const fn wllvt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write leveling LCDL."]
        #[inline(always)]
        pub fn set_wllvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write DQ LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write DQ LCDL."]
        #[inline(always)]
        pub const fn wdlvt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write DQ LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write DQ LCDL."]
        #[inline(always)]
        pub fn set_wdlvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Read DQS LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS LCDL."]
        #[inline(always)]
        pub const fn rdlvt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS LCDL."]
        #[inline(always)]
        pub fn set_rdlvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Read DQS Gating LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS gating LCDL."]
        #[inline(always)]
        pub const fn rglvt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS Gating LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS gating LCDL."]
        #[inline(always)]
        pub fn set_rglvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write Data BDL VT Compensation: Enables, if set, the VT drift compensation of the write data bit delay lines."]
        #[inline(always)]
        pub const fn wdbvt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data BDL VT Compensation: Enables, if set, the VT drift compensation of the write data bit delay lines."]
        #[inline(always)]
        pub fn set_wdbvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read Data BDL VT Compensation: Enables, if set, the VT drift compensation of the read data bit delay lines."]
        #[inline(always)]
        pub const fn rdbvt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data BDL VT Compensation: Enables, if set, the VT drift compensation of the read data bit delay lines."]
        #[inline(always)]
        pub fn set_rdbvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Delay Line Test Mode: Selects, if set, the delay line oscillator test mode. Setting this bit also clears all delay line register values. For DL oscillator testing, first set this bit, then apply desired non-zero LCDL and BDL register programmings."]
        #[inline(always)]
        pub const fn dltmode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Delay Line Test Mode: Selects, if set, the delay line oscillator test mode. Setting this bit also clears all delay line register values. For DL oscillator testing, first set this bit, then apply desired non-zero LCDL and BDL register programmings."]
        #[inline(always)]
        pub fn set_dltmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Delay Line Test Start: A write of '1' to this bit will trigger delay line oscillator mode period measurement. This bit is not self clearing and needs to be reset to '0' before the measurement can be re-triggered."]
        #[inline(always)]
        pub const fn dltst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Delay Line Test Start: A write of '1' to this bit will trigger delay line oscillator mode period measurement. This bit is not self clearing and needs to be reset to '0' before the measurement can be re-triggered."]
        #[inline(always)]
        pub fn set_dltst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Oscillator Enable: Enables, if set, the delay line oscillation."]
        #[inline(always)]
        pub const fn oscen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Oscillator Enable: Enables, if set, the delay line oscillation."]
        #[inline(always)]
        pub fn set_oscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Oscillator Mode Division: Specifies the factor by which the delay line oscillator mode output is divided down before it is output on the delay line digital test output pin dl_dto. Valid values are: 000 = Divide by 1 001 = Divide by 256 010 = Divide by 512 011 = Divide by 1024 100 = Divide by 2048 101 = Divide by 4096 110 = Divide by 8192 111 = Divide by 65536."]
        #[inline(always)]
        pub const fn oscdiv(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Oscillator Mode Division: Specifies the factor by which the delay line oscillator mode output is divided down before it is output on the delay line digital test output pin dl_dto. Valid values are: 000 = Divide by 1 001 = Divide by 256 010 = Divide by 512 011 = Divide by 1024 100 = Divide by 2048 101 = Divide by 4096 110 = Divide by 8192 111 = Divide by 65536."]
        #[inline(always)]
        pub fn set_oscdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Oscillator Mode Write-Leveling Delay Line Select: Selects which of the two write leveling LCDLs is active. The delay select value of the inactive LCDL is set to zero while the delay select value of the active LCDL can be varied by the input write leveling delay select pin. Valid values are: 00 = No WL LCDL is active 01 = DDR WL LCDL is active 10 = SDR WL LCDL is active 11 = Both LCDLs are active."]
        #[inline(always)]
        pub const fn oscwdl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Oscillator Mode Write-Leveling Delay Line Select: Selects which of the two write leveling LCDLs is active. The delay select value of the inactive LCDL is set to zero while the delay select value of the active LCDL can be varied by the input write leveling delay select pin. Valid values are: 00 = No WL LCDL is active 01 = DDR WL LCDL is active 10 = SDR WL LCDL is active 11 = Both LCDLs are active."]
        #[inline(always)]
        pub fn set_oscwdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Digital Test Output Select: Selects the PHY digital test output that is driven onto PHY digital test output (phy_dto) pin: Valid values are: 00000 = DATX8 0 PLL digital test output 00001 = DATX8 1 PLL digital test output 00010 = DATX8 2 PLL digital test output 00011 = DATX8 3 PLL digital test output 00100 = DATX8 4 PLL digital test output 00101 = DATX8 5 PLL digital test output 00110 = DATX8 6 PLL digital test output 00111 = DATX8 7 PLL digital test output 01000 = DATX8 8 PLL digital test output 01001 = AC PLL digital test output 01010 – 01111 = Reserved 10000 = DATX8 0 delay line digital test output 10001 = DATX8 1 delay line digital test output 10010 = DATX8 2 delay line digital test output 10011 = DATX8 3 delay line digital test output 10100 = DATX8 4 delay line digital test output 10101 = DATX8 5 delay line digital test output 10110 = DATX8 6 delay line digital test output 10111 = DATX8 7 delay line digital test output 11000 = DATX8 8 delay line digital test output 11001 = AC delay line digital test output 11010 – 11111 = Reserved."]
        #[inline(always)]
        pub const fn dtosel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x1f;
            val as u8
        }
        #[doc = "Digital Test Output Select: Selects the PHY digital test output that is driven onto PHY digital test output (phy_dto) pin: Valid values are: 00000 = DATX8 0 PLL digital test output 00001 = DATX8 1 PLL digital test output 00010 = DATX8 2 PLL digital test output 00011 = DATX8 3 PLL digital test output 00100 = DATX8 4 PLL digital test output 00101 = DATX8 5 PLL digital test output 00110 = DATX8 6 PLL digital test output 00111 = DATX8 7 PLL digital test output 01000 = DATX8 8 PLL digital test output 01001 = AC PLL digital test output 01010 – 01111 = Reserved 10000 = DATX8 0 delay line digital test output 10001 = DATX8 1 delay line digital test output 10010 = DATX8 2 delay line digital test output 10011 = DATX8 3 delay line digital test output 10100 = DATX8 4 delay line digital test output 10101 = DATX8 5 delay line digital test output 10110 = DATX8 6 delay line digital test output 10111 = DATX8 7 delay line digital test output 11000 = DATX8 8 delay line digital test output 11001 = AC delay line digital test output 11010 – 11111 = Reserved."]
        #[inline(always)]
        pub fn set_dtosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
        }
        #[doc = "Enables, if set, the PUB to control the interface to the PHY and SDRAM. In this mode the DFI commands from the controller are ignored. The bit must be set to 0 after the system determines it is convenient to pass control of the DFI bus to the controller. When set to 0 the DFI interface has control of the PHY and SDRAM interface except when triggering pub operations such as BIST, DCU or data training."]
        #[inline(always)]
        pub const fn pubmode(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Enables, if set, the PUB to control the interface to the PHY and SDRAM. In this mode the DFI commands from the controller are ignored. The bit must be set to 0 after the system determines it is convenient to pass control of the DFI bus to the controller. When set to 0 the DFI interface has control of the PHY and SDRAM interface except when triggering pub operations such as BIST, DCU or data training."]
        #[inline(always)]
        pub fn set_pubmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CK Enable: Controls whether the CK going to the SDRAM is enabled (toggling) or disabled (static value) and whether the CK is inverted. Two bits for each of the up to three CK pairs. Valid values for the two bits are: 00 = CK disabled (Driven to constant 0) 01 = CK toggling with inverted polarity 10 = CK toggling with normal polarity (This should be the default setting) 11 = CK disabled (Driven to constant 1)."]
        #[inline(always)]
        pub const fn cken(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "CK Enable: Controls whether the CK going to the SDRAM is enabled (toggling) or disabled (static value) and whether the CK is inverted. Two bits for each of the up to three CK pairs. Valid values for the two bits are: 00 = CK disabled (Driven to constant 0) 01 = CK toggling with inverted polarity 10 = CK toggling with normal polarity (This should be the default setting) 11 = CK disabled (Driven to constant 1)."]
        #[inline(always)]
        pub fn set_cken(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for Pgcr0 {
        #[inline(always)]
        fn default() -> Pgcr0 {
            Pgcr0(0)
        }
    }
    #[doc = "PHY General Configuration Registers 0-1 (PGCR0- 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pgcr1(pub u32);
    impl Pgcr1 {
        #[doc = "Power Down Disabled Byte: Indicates, if set, that the PLL and I/Os of a disabled byte should be powered down."]
        #[inline(always)]
        pub const fn pddisdx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power Down Disabled Byte: Indicates, if set, that the PLL and I/Os of a disabled byte should be powered down."]
        #[inline(always)]
        pub fn set_pddisdx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write Leveling (Software) Mode: Indicates, if set, that the PUB is in software write leveling mode in which software executes single steps of DQS pulsing by writing '1' to PIR.WL. The write leveling DQ status from the DRAM is captured in DXnGSR0.WLDQ."]
        #[inline(always)]
        pub const fn wlmode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling (Software) Mode: Indicates, if set, that the PUB is in software write leveling mode in which software executes single steps of DQS pulsing by writing '1' to PIR.WL. The write leveling DQ status from the DRAM is captured in DXnGSR0.WLDQ."]
        #[inline(always)]
        pub fn set_wlmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write Leveling Step: Specifies the number of delay step-size increments during each step of write leveling. Valid values are: 0 = computed to be 1/2 of the associated lane's DXnGSR0.WLPRD value 1 = 1 step size."]
        #[inline(always)]
        pub const fn wlstep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Step: Specifies the number of delay step-size increments during each step of write leveling. Valid values are: 0 = computed to be 1/2 of the associated lane's DXnGSR0.WLPRD value 1 = 1 step size."]
        #[inline(always)]
        pub fn set_wlstep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Write System Latency Optimization: controls the insertion of a pipeline stage on the AC signals from the DFI interface to the PHY to cater for a negative write system latency (WSL) value (only -1 possible). 0x0 = A pipeline stage is inserted only if WL2 training results in a WSL of -1 for any rank 0x1 = Inserts a pipeline stage."]
        #[inline(always)]
        pub const fn wslopt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write System Latency Optimization: controls the insertion of a pipeline stage on the AC signals from the DFI interface to the PHY to cater for a negative write system latency (WSL) value (only -1 possible). 0x0 = A pipeline stage is inserted only if WL2 training results in a WSL of -1 for any rank 0x1 = Inserts a pipeline stage."]
        #[inline(always)]
        pub fn set_wslopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AC PHY High-Speed Reset: a Write of '0' to this bit resets the AC macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset."]
        #[inline(always)]
        pub const fn achrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AC PHY High-Speed Reset: a Write of '0' to this bit resets the AC macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset."]
        #[inline(always)]
        pub fn set_achrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Write Leveling Select Type: Selects the encoding type for the write leveling select signal depending on the desired setup/hold margins for the internal pipelines. Refer to the DDR PHY Databook for details of how the select type is used. Valid values are: 0 = Type 1: Setup margin of 90 degrees and hold margin of 90 degrees 1 = Type 2: Setup margin of 135 degrees and hold margin of 45 degrees."]
        #[inline(always)]
        pub const fn wlselt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Select Type: Selects the encoding type for the write leveling select signal depending on the desired setup/hold margins for the internal pipelines. Refer to the DDR PHY Databook for details of how the select type is used. Valid values are: 0 = Type 1: Setup margin of 90 degrees and hold margin of 90 degrees 1 = Type 2: Setup margin of 135 degrees and hold margin of 45 degrees."]
        #[inline(always)]
        pub fn set_wlselt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O DDR Mode (D3F I/O Only): Selects the DDR mode for the I/Os. These bits connect to bits \\[2:1\\]
of the IOM pin of the SSTL I/O. For more information, refer to the SSTL I/O chapter in the DWC DDR PHY Databook."]
        #[inline(always)]
        pub const fn ioddrm(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "I/O DDR Mode (D3F I/O Only): Selects the DDR mode for the I/Os. These bits connect to bits \\[2:1\\]
of the IOM pin of the SSTL I/O. For more information, refer to the SSTL I/O chapter in the DWC DDR PHY Databook."]
        #[inline(always)]
        pub fn set_ioddrm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Master Delay Line Enable: Enables, if set, the AC master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high."]
        #[inline(always)]
        pub const fn mdlen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Master Delay Line Enable: Enables, if set, the AC master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high."]
        #[inline(always)]
        pub fn set_mdlen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Low-Pass Filter Enable: Enables, if set, the low pass filtering of MDL period measurements."]
        #[inline(always)]
        pub const fn lpfen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Pass Filter Enable: Enables, if set, the low pass filtering of MDL period measurements."]
        #[inline(always)]
        pub fn set_lpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Low-Pass Filter Depth: Specifies the number of measurements over which MDL period measurements are filtered. This determines the time constant of the low pass filter. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16."]
        #[inline(always)]
        pub const fn lpfdepth(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Low-Pass Filter Depth: Specifies the number of measurements over which MDL period measurements are filtered. This determines the time constant of the low pass filter. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16."]
        #[inline(always)]
        pub fn set_lpfdepth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Filter Depth: Specifies the number of measurements over which all AC and DATX8 initial period measurements, that happen after reset or when calibration is manually triggered, are averaged. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16."]
        #[inline(always)]
        pub const fn fdepth(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Filter Depth: Specifies the number of measurements over which all AC and DATX8 initial period measurements, that happen after reset or when calibration is manually triggered, are averaged. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16."]
        #[inline(always)]
        pub fn set_fdepth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Delay Line VT Drift Limit: Specifies the minimum change in the delay line VT drift in one direction which should result in the assertion of the delay line VT drift status signal (vt_drift). The limit is specified in terms of delay select values. A value of 0 disables the assertion of delay line VT drift status signal."]
        #[inline(always)]
        pub const fn dldlmt(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0xff;
            val as u8
        }
        #[doc = "Delay Line VT Drift Limit: Specifies the minimum change in the delay line VT drift in one direction which should result in the assertion of the delay line VT drift status signal (vt_drift). The limit is specified in terms of delay select values. A value of 0 disables the assertion of delay line VT drift status signal."]
        #[inline(always)]
        pub fn set_dldlmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 15usize)) | (((val as u32) & 0xff) << 15usize);
        }
        #[doc = "Impedance Clock Divider Select: Selects the divide ratio for the clock used by the impedance control logic relative to the clock used by the memory controller and SDRAM. Valid values are: 00 = Divide by 2 01 = Divide by 8 10 = Divide by 32 11 = Divide by 64 For more information, refer to “Impedance Calibration” on page 174."]
        #[inline(always)]
        pub const fn zcksel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x03;
            val as u8
        }
        #[doc = "Impedance Clock Divider Select: Selects the divide ratio for the clock used by the impedance control logic relative to the clock used by the memory controller and SDRAM. Valid values are: 00 = Divide by 2 01 = Divide by 8 10 = Divide by 32 11 = Divide by 64 For more information, refer to “Impedance Calibration” on page 174."]
        #[inline(always)]
        pub fn set_zcksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
        }
        #[doc = "DX PHY High-Speed Reset: a Write of '0' to this bit resets the DX macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset."]
        #[inline(always)]
        pub const fn dxhrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "DX PHY High-Speed Reset: a Write of '0' to this bit resets the DX macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset."]
        #[inline(always)]
        pub fn set_dxhrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "VT Calculation Inhibit: Inhibits calculation of the next VT compensated delay line values. A value of 1 will inhibit the VT calculation. This bit should be set to 1 during writes to the delay line registers."]
        #[inline(always)]
        pub const fn inhvt(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "VT Calculation Inhibit: Inhibits calculation of the next VT compensated delay line values. A value of 1 will inhibit the VT calculation. This bit should be set to 1 during writes to the delay line registers."]
        #[inline(always)]
        pub fn set_inhvt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "I/O Loop-Back Select: Selects where inside the I/O the loop-back of signals happens. Valid values are: 0 = Loopback is after output buffer; output enable must be asserted 1 = Loopback is before output buffer; output enable is don’t care."]
        #[inline(always)]
        pub const fn iolb(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "I/O Loop-Back Select: Selects where inside the I/O the loop-back of signals happens. Valid values are: 0 = Loopback is after output buffer; output enable must be asserted 1 = Loopback is before output buffer; output enable is don’t care."]
        #[inline(always)]
        pub fn set_iolb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Loopback DQS Shift: Selects how the read DQS is shifted during loopback to ensure that the read DQS is centered into the read data eye. Valid values are: 1b0 = PUB sets the read DQS LCDL to 0 (internally). DQS is already shifted 90 degrees by write path 1b1 = The read DQS shift is set manually through software."]
        #[inline(always)]
        pub const fn lbdqss(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback DQS Shift: Selects how the read DQS is shifted during loopback to ensure that the read DQS is centered into the read data eye. Valid values are: 1b0 = PUB sets the read DQS LCDL to 0 (internally). DQS is already shifted 90 degrees by write path 1b1 = The read DQS shift is set manually through software."]
        #[inline(always)]
        pub fn set_lbdqss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Loopback DQS Gating: Selects the DQS gating mode that should be used when the PHY is in loopback mode, including BIST loopback mode. Valid values are: 00 = DQS gate is always on 01 = DQS gate training will be triggered on the PUB 10 = DQS gate is set manually using software 11 = Reserved."]
        #[inline(always)]
        pub const fn lbgdqs(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Loopback DQS Gating: Selects the DQS gating mode that should be used when the PHY is in loopback mode, including BIST loopback mode. Valid values are: 00 = DQS gate is always on 01 = DQS gate training will be triggered on the PUB 10 = DQS gate is set manually using software 11 = Reserved."]
        #[inline(always)]
        pub fn set_lbgdqs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "Loopback Mode: Indicates, if set, that the PHY/PUB is in loopback mode."]
        #[inline(always)]
        pub const fn lbmode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Mode: Indicates, if set, that the PHY/PUB is in loopback mode."]
        #[inline(always)]
        pub fn set_lbmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pgcr1 {
        #[inline(always)]
        fn default() -> Pgcr1 {
            Pgcr1(0)
        }
    }
    #[doc = "“PHY General Configuration Register 2 (PGCR2)” on page 87."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pgcr2(pub u32);
    impl Pgcr2 {
        #[doc = "Refresh Period: Indicates the period, after which the PUB has to issue a refresh command to the SDRAM. This is derived from the maximum refresh interval from the datasheet, tRFC(max) or REFI, divided by the clock cycle time. A further 400 clocks must be subtracted from the derived number to account for command flow and missed slots of refreshes in the internal PUB blocks. The default corresponds to DDR3 9*7.8us at 1066MHz when a burst of 9 refreshes are issued at every refresh interval."]
        #[inline(always)]
        pub const fn trefprd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Refresh Period: Indicates the period, after which the PUB has to issue a refresh command to the SDRAM. This is derived from the maximum refresh interval from the datasheet, tRFC(max) or REFI, divided by the clock cycle time. A further 400 clocks must be subtracted from the derived number to account for command flow and missed slots of refreshes in the internal PUB blocks. The default corresponds to DDR3 9*7.8us at 1066MHz when a burst of 9 refreshes are issued at every refresh interval."]
        #[inline(always)]
        pub fn set_trefprd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "No Bubbles: Specified whether reads should be returned to the controller with no bubbles. Enabling no-bubble reads increases the read latency. Valid values are: 0 = Bubbles are allowed during reads 1 = Bubbles are not allowed during reads."]
        #[inline(always)]
        pub const fn nobub(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No Bubbles: Specified whether reads should be returned to the controller with no bubbles. Enabling no-bubble reads increases the read latency. Valid values are: 0 = Bubbles are allowed during reads 1 = Bubbles are not allowed during reads."]
        #[inline(always)]
        pub fn set_nobub(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fixed Latency: Specified whether all reads should be returned to the controller with a fixed read latency. Enabling fixed read latency increases the read latency. Valid values are: 0 = Disable fixed read latency 1 = Enable fixed read latency Fixed read latency is calculated as (12 + (maximum DXnGTR.RxDGSL)/2) HDR clock cycles."]
        #[inline(always)]
        pub const fn fxdlat(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed Latency: Specified whether all reads should be returned to the controller with a fixed read latency. Enabling fixed read latency increases the read latency. Valid values are: 0 = Disable fixed read latency 1 = Enable fixed read latency Fixed read latency is calculated as (12 + (maximum DXnGTR.RxDGSL)/2) HDR clock cycles."]
        #[inline(always)]
        pub fn set_fxdlat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Data Training PUB Mode Timer Exit: Specifies the number of controller clocks to wait when entering and exiting pub mode data training. The default value ensures controller refreshes do not cause memory model errors when entering and exiting data training. The value should be increased if controller initiated SDRAM ZQ short or long operation may occur just before or just after the execution of data training."]
        #[inline(always)]
        pub const fn dtpmxtmr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0xff;
            val as u8
        }
        #[doc = "Data Training PUB Mode Timer Exit: Specifies the number of controller clocks to wait when entering and exiting pub mode data training. The default value ensures controller refreshes do not cause memory model errors when entering and exiting data training. The value should be increased if controller initiated SDRAM ZQ short or long operation may occur just before or just after the execution of data training."]
        #[inline(always)]
        pub fn set_dtpmxtmr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
        }
        #[doc = "Shared-AC mode: set to 1 to enable shared address/command mode with two independent data channels – available only if shared address/command mode support is compiled in."]
        #[inline(always)]
        pub const fn shrac(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Shared-AC mode: set to 1 to enable shared address/command mode with two independent data channels – available only if shared address/command mode support is compiled in."]
        #[inline(always)]
        pub fn set_shrac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "AC Power-Down with Dual Channels: Set to 1 to power-down address/command lane when both data channels are powered-down. Only valid in shared-AC mode."]
        #[inline(always)]
        pub const fn acpddc(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "AC Power-Down with Dual Channels: Set to 1 to power-down address/command lane when both data channels are powered-down. Only valid in shared-AC mode."]
        #[inline(always)]
        pub fn set_acpddc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Low-Power Master Channel 0: set to 1 to have channel 0 act as master to drive channel 1 low-power functions simultaneously. Only valid in shared-AC mode."]
        #[inline(always)]
        pub const fn lpmstrc0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Master Channel 0: set to 1 to have channel 0 act as master to drive channel 1 low-power functions simultaneously. Only valid in shared-AC mode."]
        #[inline(always)]
        pub fn set_lpmstrc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Dynamic AC Power Down Driver: Powers down, when set, the output driver on I/O for ADDR and BA. This bit is ORed with bit ACIOCR\\[3\\]
(ACPDD)."]
        #[inline(always)]
        pub const fn dynacpdd1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic AC Power Down Driver: Powers down, when set, the output driver on I/O for ADDR and BA. This bit is ORed with bit ACIOCR\\[3\\]
(ACPDD)."]
        #[inline(always)]
        pub fn set_dynacpdd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pgcr2 {
        #[inline(always)]
        fn default() -> Pgcr2 {
            Pgcr2(0)
        }
    }
    #[doc = "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pgsr0(pub u32);
    impl Pgsr0 {
        #[doc = "Initialization Done: Indicates, if set, that the DDR system initialization has completed. This bit is set after all the selected initialization routines in PIR register have completed."]
        #[inline(always)]
        pub const fn idone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Done: Indicates, if set, that the DDR system initialization has completed. This bit is set after all the selected initialization routines in PIR register have completed."]
        #[inline(always)]
        pub fn set_idone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLL Lock Done: Indicates, if set, that PLL locking has completed."]
        #[inline(always)]
        pub const fn pldone(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Lock Done: Indicates, if set, that PLL locking has completed."]
        #[inline(always)]
        pub fn set_pldone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Digital Delay Line (DDL) Calibration Done: Indicates, if set, that DDL calibration has completed."]
        #[inline(always)]
        pub const fn dcdone(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Delay Line (DDL) Calibration Done: Indicates, if set, that DDL calibration has completed."]
        #[inline(always)]
        pub fn set_dcdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Impedance Calibration Done: Indicates, if set, that impedance calibration has completed."]
        #[inline(always)]
        pub const fn zcdone(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Done: Indicates, if set, that impedance calibration has completed."]
        #[inline(always)]
        pub fn set_zcdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DRAM Initialization Done: Indicates, if set, that DRAM initialization has completed."]
        #[inline(always)]
        pub const fn didone(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DRAM Initialization Done: Indicates, if set, that DRAM initialization has completed."]
        #[inline(always)]
        pub fn set_didone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Write Leveling Done: Indicates, if set, that write leveling has completed."]
        #[inline(always)]
        pub const fn wldone(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Done: Indicates, if set, that write leveling has completed."]
        #[inline(always)]
        pub fn set_wldone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Read DQS Gate Training Done: Indicates, if set, that DQS gate training has completed."]
        #[inline(always)]
        pub const fn qsgdone(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS Gate Training Done: Indicates, if set, that DQS gate training has completed."]
        #[inline(always)]
        pub fn set_qsgdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Write Leveling Adjustment Done: Indicates, if set, that write leveling adjustment has completed."]
        #[inline(always)]
        pub const fn wladone(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Adjustment Done: Indicates, if set, that write leveling adjustment has completed."]
        #[inline(always)]
        pub fn set_wladone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Read Data Bit Deskew Done: Indicates, if set, that read bit deskew has completed."]
        #[inline(always)]
        pub const fn rddone(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Bit Deskew Done: Indicates, if set, that read bit deskew has completed."]
        #[inline(always)]
        pub fn set_rddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Write Data Bit Deskew Done: Indicates, if set, that write bit deskew has completed."]
        #[inline(always)]
        pub const fn wddone(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Bit Deskew Done: Indicates, if set, that write bit deskew has completed."]
        #[inline(always)]
        pub fn set_wddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read Data Eye Training Done: Indicates, if set, that read eye training has completed."]
        #[inline(always)]
        pub const fn redone(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Eye Training Done: Indicates, if set, that read eye training has completed."]
        #[inline(always)]
        pub fn set_redone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Write Data Eye Training Done: Indicates, if set, that write eye training has completed."]
        #[inline(always)]
        pub const fn wedone(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Eye Training Done: Indicates, if set, that write eye training has completed."]
        #[inline(always)]
        pub fn set_wedone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Impedance Calibration Error: Indicates, if set, that there is an error in impedance calibration."]
        #[inline(always)]
        pub const fn zcerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Error: Indicates, if set, that there is an error in impedance calibration."]
        #[inline(always)]
        pub fn set_zcerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Write Leveling Error: Indicates, if set, that there is an error in write leveling."]
        #[inline(always)]
        pub const fn wlerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Error: Indicates, if set, that there is an error in write leveling."]
        #[inline(always)]
        pub fn set_wlerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Read DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training."]
        #[inline(always)]
        pub const fn qsgerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training."]
        #[inline(always)]
        pub fn set_qsgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Write Data Leveling Adjustment Error: Indicates, if set, that there is an error in write leveling adjustment."]
        #[inline(always)]
        pub const fn wlaerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Leveling Adjustment Error: Indicates, if set, that there is an error in write leveling adjustment."]
        #[inline(always)]
        pub fn set_wlaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Read Data Bit Deskew Error: Indicates, if set, that there is an error in read bit deskew."]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Bit Deskew Error: Indicates, if set, that there is an error in read bit deskew."]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Write Data Bit Deskew Error: Indicates, if set, that there is an error in write bit deskew."]
        #[inline(always)]
        pub const fn wderr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Bit Deskew Error: Indicates, if set, that there is an error in write bit deskew."]
        #[inline(always)]
        pub fn set_wderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Read Data Eye Training Error: Indicates, if set, that there is an error in read eye training."]
        #[inline(always)]
        pub const fn reerr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Eye Training Error: Indicates, if set, that there is an error in read eye training."]
        #[inline(always)]
        pub fn set_reerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Write Eye Training Error: Indicates, if set, that there is an error in write eye training."]
        #[inline(always)]
        pub const fn weerr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Write Eye Training Error: Indicates, if set, that there is an error in write eye training."]
        #[inline(always)]
        pub fn set_weerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLL Lock Done per Channel: Indicates PLL locking has completed for each underlying channel. Bit 28 represents channel 0 while bit 29 represents channel 1."]
        #[inline(always)]
        pub const fn pldone_chn(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "PLL Lock Done per Channel: Indicates PLL locking has completed for each underlying channel. Bit 28 represents channel 0 while bit 29 represents channel 1."]
        #[inline(always)]
        pub fn set_pldone_chn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "AC PLL Lock: Indicates, if set, that AC PLL has locked. This is a direct status of the AC PLL lock pin."]
        #[inline(always)]
        pub const fn aplock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AC PLL Lock: Indicates, if set, that AC PLL has locked. This is a direct status of the AC PLL lock pin."]
        #[inline(always)]
        pub fn set_aplock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pgsr0 {
        #[inline(always)]
        fn default() -> Pgsr0 {
            Pgsr0(0)
        }
    }
    #[doc = "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pgsr1(pub u32);
    impl Pgsr1 {
        #[doc = "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the AC delay line digital test output."]
        #[inline(always)]
        pub const fn dltdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the AC delay line digital test output."]
        #[inline(always)]
        pub fn set_dltdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the AC delay line digital test output."]
        #[inline(always)]
        pub const fn dltcode(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the AC delay line digital test output."]
        #[inline(always)]
        pub fn set_dltcode(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 1usize)) | (((val as u32) & 0x00ff_ffff) << 1usize);
        }
        #[doc = "VT Stop: Indicates, if set, that the VT calculation logic has stopped computing the next values for the VT compensated delay line values. After assertion of the PGCR.INHVT, the VTSTOP bit should be read to ensure all VT compensation logic has stopped computations before writing to the delay line registers."]
        #[inline(always)]
        pub const fn vtstop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "VT Stop: Indicates, if set, that the VT calculation logic has stopped computing the next values for the VT compensated delay line values. After assertion of the PGCR.INHVT, the VTSTOP bit should be read to ensure all VT compensation logic has stopped computations before writing to the delay line registers."]
        #[inline(always)]
        pub fn set_vtstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "RDIMM Parity Error: Indicates, if set, that there was a parity error (i.e. err_out_n was sampled low) during one of the transactions to the RDIMM buffer chip. This bit remains asserted until cleared by the PIR.CLRSR."]
        #[inline(always)]
        pub const fn parerr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Parity Error: Indicates, if set, that there was a parity error (i.e. err_out_n was sampled low) during one of the transactions to the RDIMM buffer chip. This bit remains asserted until cleared by the PIR.CLRSR."]
        #[inline(always)]
        pub fn set_parerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pgsr1 {
        #[inline(always)]
        fn default() -> Pgsr1 {
            Pgsr1(0)
        }
    }
    #[doc = "PHY Initialization Register (PIR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pir(pub u32);
    impl Pir {
        #[doc = "Initialization Trigger: A write of '1' to this bit triggers the DDR system initialization, including PHY initialization, DRAM initialization, and PHY training. The exact initialization steps to be executed are specified in bits 1 to 15 of this register. A bit setting of 1 means the step will be executed as part of the initialization sequence, while a setting of ‘0’ means the step will be bypassed. The initialization trigger bit is self-clearing."]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Trigger: A write of '1' to this bit triggers the DDR system initialization, including PHY initialization, DRAM initialization, and PHY training. The exact initialization steps to be executed are specified in bits 1 to 15 of this register. A bit setting of 1 means the step will be executed as part of the initialization sequence, while a setting of ‘0’ means the step will be bypassed. The initialization trigger bit is self-clearing."]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Impedance Calibration: Performs PHY impedance calibration. When set the impedance calibration will be performed in parallel with PHY initialization (PLL initialization + DDL calibration + PHY reset)."]
        #[inline(always)]
        pub const fn zcal(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration: Performs PHY impedance calibration. When set the impedance calibration will be performed in parallel with PHY initialization (PLL initialization + DDL calibration + PHY reset)."]
        #[inline(always)]
        pub fn set_zcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PLL Initialization: Executes the PLL initialization sequence which includes correct driving of PLL power-down, reset and gear shift pins, and then waiting for the PHY PLLs to lock."]
        #[inline(always)]
        pub const fn pllinit(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Initialization: Executes the PLL initialization sequence which includes correct driving of PLL power-down, reset and gear shift pins, and then waiting for the PHY PLLs to lock."]
        #[inline(always)]
        pub fn set_pllinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Digital Delay Line (DDL) Calibration: Performs PHY delay line calibration."]
        #[inline(always)]
        pub const fn dcal(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Delay Line (DDL) Calibration: Performs PHY delay line calibration."]
        #[inline(always)]
        pub fn set_dcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PHY Reset: Resets the AC and DATX8 modules by asserting the AC/DATX8 reset pin."]
        #[inline(always)]
        pub const fn phyrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Reset: Resets the AC and DATX8 modules by asserting the AC/DATX8 reset pin."]
        #[inline(always)]
        pub fn set_phyrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DRAM Reset (DDR3 Only): Issues a reset to the DRAM (by driving the DRAM reset pin low) and wait 200us. This can be triggered in isolation or with the full DRAM initialization (DRAMINIT). For the later case, the reset is issued and 200us is waited before starting the full initialization sequence."]
        #[inline(always)]
        pub const fn dramrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DRAM Reset (DDR3 Only): Issues a reset to the DRAM (by driving the DRAM reset pin low) and wait 200us. This can be triggered in isolation or with the full DRAM initialization (DRAMINIT). For the later case, the reset is issued and 200us is waited before starting the full initialization sequence."]
        #[inline(always)]
        pub fn set_dramrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DRAM Initialization: Executes the DRAM initialization sequence."]
        #[inline(always)]
        pub const fn draminit(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DRAM Initialization: Executes the DRAM initialization sequence."]
        #[inline(always)]
        pub fn set_draminit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Write Leveling (DDR3 Only): Executes a PUB write leveling routine."]
        #[inline(always)]
        pub const fn wl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling (DDR3 Only): Executes a PUB write leveling routine."]
        #[inline(always)]
        pub fn set_wl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read DQS Gate Training: Executes a PUB training routine to determine the optimum position of the read data DQS strobe for maximum system timing margins."]
        #[inline(always)]
        pub const fn qsgate(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read DQS Gate Training: Executes a PUB training routine to determine the optimum position of the read data DQS strobe for maximum system timing margins."]
        #[inline(always)]
        pub fn set_qsgate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Write Leveling Adjust (DDR3 Only): Executes a PUB training routine that re- adjusts the write latency used during write in case the write leveling routine changed the expected latency. Note: Ensure that the DCU command cache is cleared prior to running WLADJ."]
        #[inline(always)]
        pub const fn wladj(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Write Leveling Adjust (DDR3 Only): Executes a PUB training routine that re- adjusts the write latency used during write in case the write leveling routine changed the expected latency. Note: Ensure that the DCU command cache is cleared prior to running WLADJ."]
        #[inline(always)]
        pub fn set_wladj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Read Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during read."]
        #[inline(always)]
        pub const fn rddskw(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during read."]
        #[inline(always)]
        pub fn set_rddskw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Write Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during write."]
        #[inline(always)]
        pub const fn wrdskw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during write."]
        #[inline(always)]
        pub fn set_wrdskw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Read Data Eye Training: Executes a PUB training routine to maximize the read data eye."]
        #[inline(always)]
        pub const fn rdeye(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Read Data Eye Training: Executes a PUB training routine to maximize the read data eye."]
        #[inline(always)]
        pub fn set_rdeye(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Write Data Eye Training: Executes a PUB training routine to maximize the write data eye."]
        #[inline(always)]
        pub const fn wreye(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Write Data Eye Training: Executes a PUB training routine to maximize the write data eye."]
        #[inline(always)]
        pub fn set_wreye(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Initialization Complete Pin Configuration: Specifies how the DFI initialization complete output pin (dfi_init_complete) should be used to indicate the status of initialization. Valid value are: 0 = Asserted after PHY initialization (DLL locking and impedance calibration) is complete. 1 = Asserted after PHY initialization is complete and the triggered the PUB initialization (DRAM initialization, data training, or initialization trigger with no selected initialization) is complete."]
        #[inline(always)]
        pub const fn icpc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Complete Pin Configuration: Specifies how the DFI initialization complete output pin (dfi_init_complete) should be used to indicate the status of initialization. Valid value are: 0 = Asserted after PHY initialization (DLL locking and impedance calibration) is complete. 1 = Asserted after PHY initialization is complete and the triggered the PUB initialization (DRAM initialization, data training, or initialization trigger with no selected initialization) is complete."]
        #[inline(always)]
        pub fn set_icpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL Bypass: A setting of 1 on this bit will put all PHY PLLs in bypass mode."]
        #[inline(always)]
        pub const fn pllbyp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Bypass: A setting of 1 on this bit will put all PHY PLLs in bypass mode."]
        #[inline(always)]
        pub fn set_pllbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Controller DRAM Initialization: Indicates, if set, that DRAM initialization will be performed by the controller. Otherwise if not set it indicates that DRAM initialization will be performed using the built-in initialization sequence or using software through the configuration port."]
        #[inline(always)]
        pub const fn ctldinit(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Controller DRAM Initialization: Indicates, if set, that DRAM initialization will be performed by the controller. Otherwise if not set it indicates that DRAM initialization will be performed using the built-in initialization sequence or using software through the configuration port."]
        #[inline(always)]
        pub fn set_ctldinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "RDIMM Initialization: Executes the RDIMM buffer chip initialization before executing DRAM initialization. The RDIMM buffer chip initialization is run after the DRAM is reset and CKE have been driven high by the DRAM initialization sequence."]
        #[inline(always)]
        pub const fn rdimminit(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Initialization: Executes the RDIMM buffer chip initialization before executing DRAM initialization. The RDIMM buffer chip initialization is run after the DRAM is reset and CKE have been driven high by the DRAM initialization sequence."]
        #[inline(always)]
        pub fn set_rdimminit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear Status Registers: Writing 1 to this bit clears (reset to 0) select status bits in register PGSR0. This bit is primarily for debug purposes and is typically not needed during normal functional operation. It can be used when PGSR.IDONE=1, to manually clear a selection of the PGSR status bits, although starting a new initialization process (PIR\\[0\\].INIT = 1’b1) automatically clears the PGSR status bits associated with the initialization steps enabled. The following list describes which bits within the PGSR0 register are cleared when CLRSR is set to 1’b1 and which bits are not cleared: The following bits are not cleared by PIR\\[27\\]
(CLRSR): PGSR0\\[31\\]
(APLOCK) PGSR0\\[29:28\\]
(PLDONE_CHN) PGSR0\\[23\\]
(WLAERR) PGSR0\\[21\\]
(WLERR) PGSR0\\[4\\]
(DIDONE) PGSR0\\[2\\]
(DCDONE) PGSR0\\[1\\]
(PLDONE) PGSR0\\[0\\]
(IDONE) The following bits are always zero: PGSR0\\[30\\]
(reserved) PGSR0\\[19:12\\]
(reserved) The following bits are cleared unconditionally by PIR\\[27\\]
(CLRSR): PGSR0\\[27\\]
(WEERR) PGSR0\\[26\\]
(REERR) PGSR0\\[25\\]
(WDERR) PGSR0\\[24\\]
(RDERR) - PGSR0\\[22\\]
(QSGERR) - PGSR0\\[20\\]
(ZCERR) - PGSR0\\[11\\]
(WEDONE) - PGSR0\\[10\\]
(REDONE) - PGSR0\\[9\\]
(WDDONE) - PGSR0\\[8\\]
(RDDONE) - PGSR0\\[7\\]
(WLADONE) - PGSR0\\[6\\]
(QSGDONE) - PGSR0\\[5\\]
(WLDONE) - PGSR0\\[3\\]
(ZCDONE)."]
        #[inline(always)]
        pub const fn clrsr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Status Registers: Writing 1 to this bit clears (reset to 0) select status bits in register PGSR0. This bit is primarily for debug purposes and is typically not needed during normal functional operation. It can be used when PGSR.IDONE=1, to manually clear a selection of the PGSR status bits, although starting a new initialization process (PIR\\[0\\].INIT = 1’b1) automatically clears the PGSR status bits associated with the initialization steps enabled. The following list describes which bits within the PGSR0 register are cleared when CLRSR is set to 1’b1 and which bits are not cleared: The following bits are not cleared by PIR\\[27\\]
(CLRSR): PGSR0\\[31\\]
(APLOCK) PGSR0\\[29:28\\]
(PLDONE_CHN) PGSR0\\[23\\]
(WLAERR) PGSR0\\[21\\]
(WLERR) PGSR0\\[4\\]
(DIDONE) PGSR0\\[2\\]
(DCDONE) PGSR0\\[1\\]
(PLDONE) PGSR0\\[0\\]
(IDONE) The following bits are always zero: PGSR0\\[30\\]
(reserved) PGSR0\\[19:12\\]
(reserved) The following bits are cleared unconditionally by PIR\\[27\\]
(CLRSR): PGSR0\\[27\\]
(WEERR) PGSR0\\[26\\]
(REERR) PGSR0\\[25\\]
(WDERR) PGSR0\\[24\\]
(RDERR) - PGSR0\\[22\\]
(QSGERR) - PGSR0\\[20\\]
(ZCERR) - PGSR0\\[11\\]
(WEDONE) - PGSR0\\[10\\]
(REDONE) - PGSR0\\[9\\]
(WDDONE) - PGSR0\\[8\\]
(RDDONE) - PGSR0\\[7\\]
(WLADONE) - PGSR0\\[6\\]
(QSGDONE) - PGSR0\\[5\\]
(WLDONE) - PGSR0\\[3\\]
(ZCDONE)."]
        #[inline(always)]
        pub fn set_clrsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLL Lock Bypass: Bypasses or stops, if set, the waiting of PLLs to lock. PLL lock wait is automatically triggered after reset. PLL lock wait may be triggered manually using INIT and PLLINIT bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub const fn lockbyp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Lock Bypass: Bypasses or stops, if set, the waiting of PLLs to lock. PLL lock wait is automatically triggered after reset. PLL lock wait may be triggered manually using INIT and PLLINIT bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub fn set_lockbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Digital Delay Line (DDL) Calibration Bypass: Bypasses or stops, if set, DDL calibration that automatically triggers after reset. DDL calibration may be triggered manually using INIT and DCAL bits of the PIR register. This bit is self- clearing."]
        #[inline(always)]
        pub const fn dcalbyp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Delay Line (DDL) Calibration Bypass: Bypasses or stops, if set, DDL calibration that automatically triggers after reset. DDL calibration may be triggered manually using INIT and DCAL bits of the PIR register. This bit is self- clearing."]
        #[inline(always)]
        pub fn set_dcalbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Impedance Calibration Bypass: Bypasses or stops, if set, impedance calibration of all ZQ control blocks that automatically triggers after reset. Impedance calibration may be triggered manually using INIT and ZCAL bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub const fn zcalbyp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Bypass: Bypasses or stops, if set, impedance calibration of all ZQ control blocks that automatically triggers after reset. Impedance calibration may be triggered manually using INIT and ZCAL bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub fn set_zcalbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Initialization Bypass: Bypasses or stops, if set, all initialization routines currently running, including PHY initialization, DRAM initialization, and PHY training. Initialization may be triggered manually using INIT and the other relevant bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub const fn initbyp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Bypass: Bypasses or stops, if set, all initialization routines currently running, including PHY initialization, DRAM initialization, and PHY training. Initialization may be triggered manually using INIT and the other relevant bits of the PIR register. This bit is self-clearing."]
        #[inline(always)]
        pub fn set_initbyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pir {
        #[inline(always)]
        fn default() -> Pir {
            Pir(0)
        }
    }
    #[doc = "“PLL Control Register (PLLCR)” on page 91."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcr(pub u32);
    impl Pllcr {
        #[doc = "Digital Test Control: Selects various PLL digital test signals and other test mode signals to be brought out via bit \\[1\\]
of the PLL digital test output (pll_dto\\[1\\]). Valid values are: 00 = ‘0’ (Test output is disabled) 01 = PLL x1 clock (X1) 10 = PLL reference (input) clock (REF_CLK) 11 = PLL feedback clock (FB_X1)."]
        #[inline(always)]
        pub const fn dtc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Digital Test Control: Selects various PLL digital test signals and other test mode signals to be brought out via bit \\[1\\]
of the PLL digital test output (pll_dto\\[1\\]). Valid values are: 00 = ‘0’ (Test output is disabled) 01 = PLL x1 clock (X1) 10 = PLL reference (input) clock (REF_CLK) 11 = PLL feedback clock (FB_X1)."]
        #[inline(always)]
        pub fn set_dtc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Analog Test Control: Selects various PLL analog test signals to be brought out via PLL analog test output pin (pll_ato). Valid values are: 0000 = Reserved 0001 = vdd_ckin 0010 = vrfbf 0011 = vdd_cko 0100 = vp_cp 0101 = vpfil(vp) 0110 = Reserved 0111 = gd 1000 = vcntrl_atb 1001 = vref_atb 1010 = vpsf_atb 1011 – 1111 = Reserved."]
        #[inline(always)]
        pub const fn atc(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Analog Test Control: Selects various PLL analog test signals to be brought out via PLL analog test output pin (pll_ato). Valid values are: 0000 = Reserved 0001 = vdd_ckin 0010 = vrfbf 0011 = vdd_cko 0100 = vp_cp 0101 = vpfil(vp) 0110 = Reserved 0111 = gd 1000 = vcntrl_atb 1001 = vref_atb 1010 = vpsf_atb 1011 – 1111 = Reserved."]
        #[inline(always)]
        pub fn set_atc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Analog Test Enable (ATOEN): Selects the analog test signal that is driven on the analog test output pin. Otherwise the analog test output is tri-stated. This allows analog test output pins from multiple PLLs to be connected together. Valid values are: 0000 = All PLL analog test signals are tri-stated 0001 = AC PLL analog test signal is driven out 0010 = DATX8 0 PLL analog test signal is driven out 0011 = DATX8 1 PLL analog test signal is driven out 0100 = DATX8 2 PLL analog test signal is driven out 0101 = DATX8 3 PLL analog test signal is driven out 0110 = DATX8 4 PLL analog test signal is driven out 0111 = DATX8 5 PLL analog test signal is driven out 1000 = DATX8 6 PLL analog test signal is driven out 1001 = DATX8 7 PLL analog test signal is driven out 1010 = DATX8 8 PLL analog test signal is driven out 1011 – 1111 = Reserved."]
        #[inline(always)]
        pub const fn atoen(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "Analog Test Enable (ATOEN): Selects the analog test signal that is driven on the analog test output pin. Otherwise the analog test output is tri-stated. This allows analog test output pins from multiple PLLs to be connected together. Valid values are: 0000 = All PLL analog test signals are tri-stated 0001 = AC PLL analog test signal is driven out 0010 = DATX8 0 PLL analog test signal is driven out 0011 = DATX8 1 PLL analog test signal is driven out 0100 = DATX8 2 PLL analog test signal is driven out 0101 = DATX8 3 PLL analog test signal is driven out 0110 = DATX8 4 PLL analog test signal is driven out 0111 = DATX8 5 PLL analog test signal is driven out 1000 = DATX8 6 PLL analog test signal is driven out 1001 = DATX8 7 PLL analog test signal is driven out 1010 = DATX8 8 PLL analog test signal is driven out 1011 – 1111 = Reserved."]
        #[inline(always)]
        pub fn set_atoen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "Gear Shift: Enables, if set, rapid locking mode."]
        #[inline(always)]
        pub const fn gshift(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Gear Shift: Enables, if set, rapid locking mode."]
        #[inline(always)]
        pub fn set_gshift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Charge Pump Integrating Current Control."]
        #[inline(always)]
        pub const fn cpic(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Charge Pump Integrating Current Control."]
        #[inline(always)]
        pub fn set_cpic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Charge Pump Proportional Current Control."]
        #[inline(always)]
        pub const fn cppc(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "Charge Pump Proportional Current Control."]
        #[inline(always)]
        pub fn set_cppc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "PLL Quadrature Phase Mode: Enables, if set, the quadrature phase clock outputs. This mode is not used in this version of the PHY."]
        #[inline(always)]
        pub const fn qpmode(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Quadrature Phase Mode: Enables, if set, the quadrature phase clock outputs. This mode is not used in this version of the PHY."]
        #[inline(always)]
        pub fn set_qpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL Frequency Select: Selects the operating range of the PLL. Valid values for PHYs that go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 335MHz to 533MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 225MHz to 385MHz 10 = Reserved 11 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 275MHz Valid values for PHYs that don’t go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 250MHz to 400MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 300MHz 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub const fn frqsel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "PLL Frequency Select: Selects the operating range of the PLL. Valid values for PHYs that go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 335MHz to 533MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 225MHz to 385MHz 10 = Reserved 11 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 275MHz Valid values for PHYs that don’t go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 250MHz to 400MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 300MHz 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub fn set_frqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "PLL Power Down: Puts the PLLs in power down mode by driving the PLL power down pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the power-down."]
        #[inline(always)]
        pub const fn pllpd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Power Down: Puts the PLLs in power down mode by driving the PLL power down pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the power-down."]
        #[inline(always)]
        pub fn set_pllpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PLL Rest: Resets the PLLs by driving the PLL reset pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the reset."]
        #[inline(always)]
        pub const fn pllrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Rest: Resets the PLLs by driving the PLL reset pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the reset."]
        #[inline(always)]
        pub fn set_pllrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "PLL Bypass: Bypasses the PLL, if set, to 1."]
        #[inline(always)]
        pub const fn byp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Bypass: Bypasses the PLL, if set, to 1."]
        #[inline(always)]
        pub fn set_byp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pllcr {
        #[inline(always)]
        fn default() -> Pllcr {
            Pllcr(0)
        }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptr0(pub u32);
    impl Ptr0 {
        #[doc = "PHY Reset Time: Number of configuration or APB clock cycles that the PHY reset must remain asserted after PHY calibration is done before the reset to the PHY is de-asserted. This is used to extend the reset to the PHY so that the reset is asserted for some clock cycles after the clocks are stable. Valid values are from 1 to 63 (the value must be non-zero)."]
        #[inline(always)]
        pub const fn tphyrst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "PHY Reset Time: Number of configuration or APB clock cycles that the PHY reset must remain asserted after PHY calibration is done before the reset to the PHY is de-asserted. This is used to extend the reset to the PHY so that the reset is asserted for some clock cycles after the clocks are stable. Valid values are from 1 to 63 (the value must be non-zero)."]
        #[inline(always)]
        pub fn set_tphyrst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "PLL Gear Shift Time: Number of configuration or APB clock cycles from when the PLL reset pin is de-asserted to when the PLL gear shift pin is de-asserted. This must correspond to a value that is equal to or more than 4us. Default value corresponds to 4us."]
        #[inline(always)]
        pub const fn tpllgs(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x7fff;
            val as u16
        }
        #[doc = "PLL Gear Shift Time: Number of configuration or APB clock cycles from when the PLL reset pin is de-asserted to when the PLL gear shift pin is de-asserted. This must correspond to a value that is equal to or more than 4us. Default value corresponds to 4us."]
        #[inline(always)]
        pub fn set_tpllgs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 6usize)) | (((val as u32) & 0x7fff) << 6usize);
        }
        #[doc = "PLL Power-Down Time: Number of configuration or APB clock cycles that the PLL must remain in power-down mode, i.e. number of clock cycles from when PLL power-down pin is asserted to when PLL power-down pin is de-asserted. This must correspond to a value that is equal to or more than 1us. Default value corresponds to 1us."]
        #[inline(always)]
        pub const fn tpllpd(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "PLL Power-Down Time: Number of configuration or APB clock cycles that the PLL must remain in power-down mode, i.e. number of clock cycles from when PLL power-down pin is asserted to when PLL power-down pin is de-asserted. This must correspond to a value that is equal to or more than 1us. Default value corresponds to 1us."]
        #[inline(always)]
        pub fn set_tpllpd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Ptr0 {
        #[inline(always)]
        fn default() -> Ptr0 {
            Ptr0(0)
        }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptr1(pub u32);
    impl Ptr1 {
        #[doc = "PLL Reset Time: Number of configuration or APB clock cycles that the PLL must remain in reset mode, i.e. number of clock cycles from when PLL power-down pin is de-asserted and PLL reset pin is asserted to when PLL reset pin is de-asserted. The setting must correspond to a value that is equal to, or greater than, 3us."]
        #[inline(always)]
        pub const fn tpllrst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "PLL Reset Time: Number of configuration or APB clock cycles that the PLL must remain in reset mode, i.e. number of clock cycles from when PLL power-down pin is de-asserted and PLL reset pin is asserted to when PLL reset pin is de-asserted. The setting must correspond to a value that is equal to, or greater than, 3us."]
        #[inline(always)]
        pub fn set_tpllrst(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "PLL Lock Time: Number of configuration or APB clock cycles for the PLL to stabilize and lock, i.e. number of clock cycles from when the PLL reset pin is de-asserted to when the PLL has lock and is ready for use. This must correspond to a value that is equal to or more than 100us. Default value corresponds to 100us."]
        #[inline(always)]
        pub const fn tplllock(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "PLL Lock Time: Number of configuration or APB clock cycles for the PLL to stabilize and lock, i.e. number of clock cycles from when the PLL reset pin is de-asserted to when the PLL has lock and is ready for use. This must correspond to a value that is equal to or more than 100us. Default value corresponds to 100us."]
        #[inline(always)]
        pub fn set_tplllock(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Ptr1 {
        #[inline(always)]
        fn default() -> Ptr1 {
            Ptr1(0)
        }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptr2(pub u32);
    impl Ptr2 {
        #[doc = "Calibration On Time: Number of clock cycles that the calibration clock is enabled (cal_clk_en asserted)."]
        #[inline(always)]
        pub const fn tcalon(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Calibration On Time: Number of clock cycles that the calibration clock is enabled (cal_clk_en asserted)."]
        #[inline(always)]
        pub fn set_tcalon(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Calibration Setup Time: Number of controller clock cycles from when calibration is enabled (cal_en asserted) to when the calibration clock is asserted again (cal_clk_en asserted)."]
        #[inline(always)]
        pub const fn tcals(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "Calibration Setup Time: Number of controller clock cycles from when calibration is enabled (cal_en asserted) to when the calibration clock is asserted again (cal_clk_en asserted)."]
        #[inline(always)]
        pub fn set_tcals(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "Calibration Hold Time: Number of controller clock cycles from when the clock was disabled (cal_clk_en deasserted) to when calibration is enable (cal_en asserted)."]
        #[inline(always)]
        pub const fn tcalh(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "Calibration Hold Time: Number of controller clock cycles from when the clock was disabled (cal_clk_en deasserted) to when calibration is enable (cal_en asserted)."]
        #[inline(always)]
        pub fn set_tcalh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "Write Leveling Delay Settling Time: Number of controller clock cycles from when a new value of the write leveling delay is applies to the LCDL to when to DQS high is driven high. This allows the delay to settle."]
        #[inline(always)]
        pub const fn twldlys(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "Write Leveling Delay Settling Time: Number of controller clock cycles from when a new value of the write leveling delay is applies to the LCDL to when to DQS high is driven high. This allows the delay to settle."]
        #[inline(always)]
        pub fn set_twldlys(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
    }
    impl Default for Ptr2 {
        #[inline(always)]
        fn default() -> Ptr2 {
            Ptr2(0)
        }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptr3(pub u32);
    impl Ptr3 {
        #[doc = "DRAM Initialization Time 0: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE low time with power and clock stable (500 us) DDR2 = CKE low time with power and clock stable (200 us) Default value corresponds to DDR3 500 us at 1066 MHz. During Verilog simulations, it is recommended that this value is changed to a much smaller value in order to avoid long simulation times. However, this may cause a memory model error, due to a violation of the CKE setup sequence. This violation is expected if this value is not programmed to the required SDRAM CKE low time, but memory models should be able to tolerate this violation without malfunction of the model."]
        #[inline(always)]
        pub const fn tdinit0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "DRAM Initialization Time 0: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE low time with power and clock stable (500 us) DDR2 = CKE low time with power and clock stable (200 us) Default value corresponds to DDR3 500 us at 1066 MHz. During Verilog simulations, it is recommended that this value is changed to a much smaller value in order to avoid long simulation times. However, this may cause a memory model error, due to a violation of the CKE setup sequence. This violation is expected if this value is not programmed to the required SDRAM CKE low time, but memory models should be able to tolerate this violation without malfunction of the model."]
        #[inline(always)]
        pub fn set_tdinit0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "DRAM Initialization Time 1: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE high time to first command (tRFC + 10 ns or 5 tCK, whichever is bigger) DDR2 = CKE high time to first command (400 ns) Default value corresponds to DDR3 tRFC of 360ns at 1066 MHz."]
        #[inline(always)]
        pub const fn tdinit1(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[doc = "DRAM Initialization Time 1: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE high time to first command (tRFC + 10 ns or 5 tCK, whichever is bigger) DDR2 = CKE high time to first command (400 ns) Default value corresponds to DDR3 tRFC of 360ns at 1066 MHz."]
        #[inline(always)]
        pub fn set_tdinit1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
        }
    }
    impl Default for Ptr3 {
        #[inline(always)]
        fn default() -> Ptr3 {
            Ptr3(0)
        }
    }
    #[doc = "PHY Timing Registers 0-4 (PTR0-4)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptr4(pub u32);
    impl Ptr4 {
        #[doc = "DRAM Initialization Time 2: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Reset low time (200 us on power-up or 100 ns after power-up) Default value corresponds to DDR3 200 us at 1066 MHz."]
        #[inline(always)]
        pub const fn tdinit2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DRAM Initialization Time 2: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Reset low time (200 us on power-up or 100 ns after power-up) Default value corresponds to DDR3 200 us at 1066 MHz."]
        #[inline(always)]
        pub fn set_tdinit2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "DRAM Initialization Time 3: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Time from ZQ initialization command to first command (1 us) Default value corresponds to the DDR3 640ns at 1066 MHz."]
        #[inline(always)]
        pub const fn tdinit3(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x03ff;
            val as u16
        }
        #[doc = "DRAM Initialization Time 3: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Time from ZQ initialization command to first command (1 us) Default value corresponds to the DDR3 640ns at 1066 MHz."]
        #[inline(always)]
        pub fn set_tdinit3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 18usize)) | (((val as u32) & 0x03ff) << 18usize);
        }
    }
    impl Default for Ptr4 {
        #[inline(always)]
        fn default() -> Ptr4 {
            Ptr4(0)
        }
    }
    #[doc = "RDIMM Control Register 0-1 (RDIMMCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdimmcr0(pub u32);
    impl Rdimmcr0 {
        #[doc = "Control Word 0 (Global Features Control Word): Bit definitions are: RC0\\[0\\]: 0 = Output inversion enabled, 1 = Output inversion disabled. RC0\\[1\\]: 0 = Floating outputs disabled, 1 = Floating outputs enabled. RC0\\[2\\]: 0 = A outputs enabled, 1 = A outputs disabled. RC0\\[3\\]: 0 = B outputs enabled, 1 = B outputs disabled."]
        #[inline(always)]
        pub const fn rc0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 0 (Global Features Control Word): Bit definitions are: RC0\\[0\\]: 0 = Output inversion enabled, 1 = Output inversion disabled. RC0\\[1\\]: 0 = Floating outputs disabled, 1 = Floating outputs enabled. RC0\\[2\\]: 0 = A outputs enabled, 1 = A outputs disabled. RC0\\[3\\]: 0 = B outputs enabled, 1 = B outputs disabled."]
        #[inline(always)]
        pub fn set_rc0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Control Word 1 (Clock Driver Enable Control Word): Bit definitions are: RC1\\[0\\]: 0 = Y0/Y0# clock enabled, 1 = Y0/Y0# clock disabled. RC1\\[1\\]: 0 = Y1/Y1# clock enabled, 1 = Y1/Y1# clock disabled. RC1\\[2\\]: 0 = Y2/Y2# clock enabled, 1 = Y2/Y2# clock disabled. RC1\\[3\\]: 0 = Y3/Y3# clock enabled, 1 = Y3/Y3# clock disabled."]
        #[inline(always)]
        pub const fn rc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 1 (Clock Driver Enable Control Word): Bit definitions are: RC1\\[0\\]: 0 = Y0/Y0# clock enabled, 1 = Y0/Y0# clock disabled. RC1\\[1\\]: 0 = Y1/Y1# clock enabled, 1 = Y1/Y1# clock disabled. RC1\\[2\\]: 0 = Y2/Y2# clock enabled, 1 = Y2/Y2# clock disabled. RC1\\[3\\]: 0 = Y3/Y3# clock enabled, 1 = Y3/Y3# clock disabled."]
        #[inline(always)]
        pub fn set_rc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Control Word 2 (Timing Control Word): Bit definitions are: RC2\\[0\\]: 0 = Standard (1/2 clock) pre-launch, 1 = Prelaunch controlled by RC12. RC2\\[1\\]: 0 = Reserved. RC2\\[2\\]: 0 = 100 Ohm input bus termination, 1 = 150 Ohm input bus termination. RC2\\[3\\]: 0 = Operation frequency band 1, 1 = Test mode frequency band 2."]
        #[inline(always)]
        pub const fn rc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 2 (Timing Control Word): Bit definitions are: RC2\\[0\\]: 0 = Standard (1/2 clock) pre-launch, 1 = Prelaunch controlled by RC12. RC2\\[1\\]: 0 = Reserved. RC2\\[2\\]: 0 = 100 Ohm input bus termination, 1 = 150 Ohm input bus termination. RC2\\[3\\]: 0 = Operation frequency band 1, 1 = Test mode frequency band 2."]
        #[inline(always)]
        pub fn set_rc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Control Word 3 (Command/Address Signals Driver Characteristics Control Word): RC3\\[1:0\\]
is driver settings for command/address A outputs, and RC3\\[3:2\\]
is driver settings for command/address B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved."]
        #[inline(always)]
        pub const fn rc3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 3 (Command/Address Signals Driver Characteristics Control Word): RC3\\[1:0\\]
is driver settings for command/address A outputs, and RC3\\[3:2\\]
is driver settings for command/address B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved."]
        #[inline(always)]
        pub fn set_rc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Control Word 4 (Control Signals Driver Characteristics Control Word): RC4\\[1:0\\]
is driver settings for control A outputs, and RC4\\[3:2\\]
is driver settings for control B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub const fn rc4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 4 (Control Signals Driver Characteristics Control Word): RC4\\[1:0\\]
is driver settings for control A outputs, and RC4\\[3:2\\]
is driver settings for control B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Reserved 11 = Reserved."]
        #[inline(always)]
        pub fn set_rc4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Control Word 5 (CK Driver Characteristics Control Word): RC5\\[1:0\\]
is driver settings for clock Y1, Y1#, Y3, and Y3# outputs, and RC5\\[3:2\\]
is driver settings for clock Y0, Y0#, Y2, and Y2# outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved."]
        #[inline(always)]
        pub const fn rc5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 5 (CK Driver Characteristics Control Word): RC5\\[1:0\\]
is driver settings for clock Y1, Y1#, Y3, and Y3# outputs, and RC5\\[3:2\\]
is driver settings for clock Y0, Y0#, Y2, and Y2# outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved."]
        #[inline(always)]
        pub fn set_rc5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Control Word 6: Reserved, free to use by vendor."]
        #[inline(always)]
        pub const fn rc6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 6: Reserved, free to use by vendor."]
        #[inline(always)]
        pub fn set_rc6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Control Word 7: Reserved, free to use by vendor."]
        #[inline(always)]
        pub const fn rc7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 7: Reserved, free to use by vendor."]
        #[inline(always)]
        pub fn set_rc7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Rdimmcr0 {
        #[inline(always)]
        fn default() -> Rdimmcr0 {
            Rdimmcr0(0)
        }
    }
    #[doc = "RDIMM Control Register 0-1 (RDIMMCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdimmcr1(pub u32);
    impl Rdimmcr1 {
        #[doc = "Control Word 8 (Additional Input Bus Termination Setting Control Word): RC8\\[2:0\\]
is Input Bus Termination (IBT) setting as follows: 000 = IBT as defined in RC2. 001 = Reserved 010 = 200 Ohm 011 = Reserved 100 = 300 Ohm 101 = Reserved 110 = Reserved 111 = Off RC8\\[3\\]: 0 = IBT off when MIRROR is HIGH, 1 = IBT on when MIRROR is high."]
        #[inline(always)]
        pub const fn rc8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 8 (Additional Input Bus Termination Setting Control Word): RC8\\[2:0\\]
is Input Bus Termination (IBT) setting as follows: 000 = IBT as defined in RC2. 001 = Reserved 010 = 200 Ohm 011 = Reserved 100 = 300 Ohm 101 = Reserved 110 = Reserved 111 = Off RC8\\[3\\]: 0 = IBT off when MIRROR is HIGH, 1 = IBT on when MIRROR is high."]
        #[inline(always)]
        pub fn set_rc8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Control Word 9 (Power Saving Settings Control Word): Bit definitions are: RC9\\[0\\]: 0 = Floating outputs as defined in RC0, 1 = Weak drive enabled. RC9\\[1\\]: 0 = Reserved. RC9\\[2\\]: 0 = CKE power down with IBT ON, QxODT is a function of DxODT, 1 = CKE power down with IBT off, QxODT held LOW. RC9\\[2\\]
is valid only when RC9\\[3\\]
is 1. RC9\\[3\\]: 0 = CKE power down mode disabled, 1 = CKE power down mode enabled."]
        #[inline(always)]
        pub const fn rc9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 9 (Power Saving Settings Control Word): Bit definitions are: RC9\\[0\\]: 0 = Floating outputs as defined in RC0, 1 = Weak drive enabled. RC9\\[1\\]: 0 = Reserved. RC9\\[2\\]: 0 = CKE power down with IBT ON, QxODT is a function of DxODT, 1 = CKE power down with IBT off, QxODT held LOW. RC9\\[2\\]
is valid only when RC9\\[3\\]
is 1. RC9\\[3\\]: 0 = CKE power down mode disabled, 1 = CKE power down mode enabled."]
        #[inline(always)]
        pub fn set_rc9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Control Word 10 (RDIMM Operating Speed Control Word): RC10\\[2:0\\]
is RDIMM operating speed setting as follows: 000 = DDR3/DDR3L-800 001 = DDR3/DDR3L-1066 010 = DDR3/DDR3L-1333 011 = DDR3/DDR3L-1600 100 = Reserved 101 = Reserved 110 = Reserved 111 = Reserved RC10\\[3\\]: Don’t care."]
        #[inline(always)]
        pub const fn rc10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 10 (RDIMM Operating Speed Control Word): RC10\\[2:0\\]
is RDIMM operating speed setting as follows: 000 = DDR3/DDR3L-800 001 = DDR3/DDR3L-1066 010 = DDR3/DDR3L-1333 011 = DDR3/DDR3L-1600 100 = Reserved 101 = Reserved 110 = Reserved 111 = Reserved RC10\\[3\\]: Don’t care."]
        #[inline(always)]
        pub fn set_rc10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Control Word 11 (Operating Voltage VDD Control Word): RC10\\[1:0\\]
is VDD operating voltage setting as follows: 00 = DDR3 1.5V mode 01 = DDR3L 1.35V mode 10 = Reserved 11 = Reserved RC10\\[3:2\\]: Reserved."]
        #[inline(always)]
        pub const fn rc11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 11 (Operating Voltage VDD Control Word): RC10\\[1:0\\]
is VDD operating voltage setting as follows: 00 = DDR3 1.5V mode 01 = DDR3L 1.35V mode 10 = Reserved 11 = Reserved RC10\\[3:2\\]: Reserved."]
        #[inline(always)]
        pub fn set_rc11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Control Word 12: Reserved for future use."]
        #[inline(always)]
        pub const fn rc12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 12: Reserved for future use."]
        #[inline(always)]
        pub fn set_rc12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Control Word 13: Reserved for future use."]
        #[inline(always)]
        pub const fn rc13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 13: Reserved for future use."]
        #[inline(always)]
        pub fn set_rc13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Control Word 14: Reserved for future use."]
        #[inline(always)]
        pub const fn rc14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 14: Reserved for future use."]
        #[inline(always)]
        pub fn set_rc14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Control Word 15: Reserved for future use."]
        #[inline(always)]
        pub const fn rc15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Control Word 15: Reserved for future use."]
        #[inline(always)]
        pub fn set_rc15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Rdimmcr1 {
        #[inline(always)]
        fn default() -> Rdimmcr1 {
            Rdimmcr1(0)
        }
    }
    #[doc = "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdimmgcr0(pub u32);
    impl Rdimmgcr0 {
        #[doc = "Registered DIMM: Indicates, if set, that a registered DIMM is used. In this case, the PUB increases the SDRAM write and read latencies (WL/RL) by 1 and also enforces that accesses adhere to RDIMM buffer chip. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments to WL/RL when using a registered DIMM. The DCR.NOSRA register bit must be set to ‘1’ if using the standard RDIMM buffer chip so that normal DRAM accesses do not assert multiple chip select bits at the same time."]
        #[inline(always)]
        pub const fn rdimm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Registered DIMM: Indicates, if set, that a registered DIMM is used. In this case, the PUB increases the SDRAM write and read latencies (WL/RL) by 1 and also enforces that accesses adhere to RDIMM buffer chip. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments to WL/RL when using a registered DIMM. The DCR.NOSRA register bit must be set to ‘1’ if using the standard RDIMM buffer chip so that normal DRAM accesses do not assert multiple chip select bits at the same time."]
        #[inline(always)]
        pub fn set_rdimm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Parity Error No Registering: Indicates, if set, that parity error signal from the RDIMM should be passed to the DFI controller without any synchronization or registering. Otherwise, the error signal is synchronized as shown in Figure 4-30 on page 262."]
        #[inline(always)]
        pub const fn errnoreg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error No Registering: Indicates, if set, that parity error signal from the RDIMM should be passed to the DFI controller without any synchronization or registering. Otherwise, the error signal is synchronized as shown in Figure 4-30 on page 262."]
        #[inline(always)]
        pub fn set_errnoreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop On Parity Error: Indicates, if set, that the PUB is to stop driving commands to the DRAM upon encountering a parity error. Transactions can resume only after status is cleared via PIR.CLRSR."]
        #[inline(always)]
        pub const fn soperr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Stop On Parity Error: Indicates, if set, that the PUB is to stop driving commands to the DRAM upon encountering a parity error. Transactions can resume only after status is cleared via PIR.CLRSR."]
        #[inline(always)]
        pub fn set_soperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PAR_IN On-Die Termination: Enables, when set, the on-die termination on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub const fn parinodt(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PAR_IN On-Die Termination: Enables, when set, the on-die termination on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub fn set_parinodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PAR_IN Power Down Driver: Powers down, when set, the output driver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub const fn parinpdd(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PAR_IN Power Down Driver: Powers down, when set, the output driver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub fn set_parinpdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PAR_IN Power Down Receiver: Powers down, when set, the input receiver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub const fn parinpdr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PAR_IN Power Down Receiver: Powers down, when set, the input receiver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub fn set_parinpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PAR_IN I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for PAR_IN pin."]
        #[inline(always)]
        pub const fn pariniom(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PAR_IN I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for PAR_IN pin."]
        #[inline(always)]
        pub fn set_pariniom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PAR_IN Output Enable: Enables, when set, the output driver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub const fn parinoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PAR_IN Output Enable: Enables, when set, the output driver on the I/O for PAR_IN pin."]
        #[inline(always)]
        pub fn set_parinoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ERROUT# On-Die Termination: Enables, when set, the on-die termination on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub const fn erroutodt(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ERROUT# On-Die Termination: Enables, when set, the on-die termination on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub fn set_erroutodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ERROUT# Power Down Driver: Powers down, when set, the output driver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub const fn erroutpdd(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ERROUT# Power Down Driver: Powers down, when set, the output driver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub fn set_erroutpdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ERROUT# Power Down Receiver: Powers down, when set, the input receiver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub const fn erroutpdr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ERROUT# Power Down Receiver: Powers down, when set, the input receiver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub fn set_erroutpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ERROUT# I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for ERROUT# pin."]
        #[inline(always)]
        pub const fn erroutiom(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ERROUT# I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for ERROUT# pin."]
        #[inline(always)]
        pub fn set_erroutiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ERROUT# Output Enable: Enables, when set, the output driver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub const fn erroutoe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ERROUT# Output Enable: Enables, when set, the output driver on the I/O for ERROUT# pin."]
        #[inline(always)]
        pub fn set_erroutoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "RDIMM Outputs On-Die Termination: Enables, when set, the on-die termination on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub const fn rdimmodt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Outputs On-Die Termination: Enables, when set, the on-die termination on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub fn set_rdimmodt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "RDIMM Outputs Power Down Driver: Powers down, when set, the output driver on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub const fn rdimmpdd(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Outputs Power Down Driver: Powers down, when set, the output driver on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub fn set_rdimmpdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "RDIMM Outputs Power Down Receiver: Powers down, when set, the input receiver on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub const fn rdimmpdr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Outputs Power Down Receiver: Powers down, when set, the input receiver on the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub fn set_rdimmpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "RDIMM Outputs I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub const fn rdimmiom(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Outputs I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for QCSEN# and MIRROR pins."]
        #[inline(always)]
        pub fn set_rdimmiom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "QCSEN# Output Enable: Enables, when set, the output driver on the I/O for QCSEN# pin."]
        #[inline(always)]
        pub const fn qcsenoe(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "QCSEN# Output Enable: Enables, when set, the output driver on the I/O for QCSEN# pin."]
        #[inline(always)]
        pub fn set_qcsenoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "MIRROR Output Enable: Enables, when set, the output driver on the I/O for MIRROR pin."]
        #[inline(always)]
        pub const fn mirroroe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "MIRROR Output Enable: Enables, when set, the output driver on the I/O for MIRROR pin."]
        #[inline(always)]
        pub fn set_mirroroe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "RDMIMM Quad CS Enable: Enables, if set, the Quad CS mode for the RDIMM registering buffer chip. This register bit controls the buffer chip QCSEN# signal."]
        #[inline(always)]
        pub const fn qcsen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RDMIMM Quad CS Enable: Enables, if set, the Quad CS mode for the RDIMM registering buffer chip. This register bit controls the buffer chip QCSEN# signal."]
        #[inline(always)]
        pub fn set_qcsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "RDIMM Mirror: Selects between two different ballouts of the RDIMM buffer chip for front or back operation. This register bit controls the buffer chip MIRROR signal."]
        #[inline(always)]
        pub const fn mirror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RDIMM Mirror: Selects between two different ballouts of the RDIMM buffer chip for front or back operation. This register bit controls the buffer chip MIRROR signal."]
        #[inline(always)]
        pub fn set_mirror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rdimmgcr0 {
        #[inline(always)]
        fn default() -> Rdimmgcr0 {
            Rdimmgcr0(0)
        }
    }
    #[doc = "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdimmgcr1(pub u32);
    impl Rdimmgcr1 {
        #[doc = "Stabilization time: Number of DRAM clock cycles for the RDIMM buffer chip to stabilize. This parameter corresponds to the buffer chip tSTAB parameter. Default value is in decimal format and corresponds to 6us at 533MHz."]
        #[inline(always)]
        pub const fn tbcstab(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Stabilization time: Number of DRAM clock cycles for the RDIMM buffer chip to stabilize. This parameter corresponds to the buffer chip tSTAB parameter. Default value is in decimal format and corresponds to 6us at 533MHz."]
        #[inline(always)]
        pub fn set_tbcstab(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Command word to command word programming delay: Number of DRAM clock cycles between two RDIMM buffer chip command programming accesses. The value used for tBCMRD is 8 plus the value programmed in these bits, i.e. tBCMRD value ranges from 8 to 15. This parameter corresponds to the buffer chip tMRD parameter."]
        #[inline(always)]
        pub const fn tbcmrd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Command word to command word programming delay: Number of DRAM clock cycles between two RDIMM buffer chip command programming accesses. The value used for tBCMRD is 8 plus the value programmed in these bits, i.e. tBCMRD value ranges from 8 to 15. This parameter corresponds to the buffer chip tMRD parameter."]
        #[inline(always)]
        pub fn set_tbcmrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Control Registers Initialization Enable: Indicates which RDIMM buffer chip control registers (RC0 to RC15) should be initialized (written) when the PUB is triggered to initialize the buffer chip. A setting of ‘1’ on CRINIT\\[n\\]
bit means that CRn should be written during initialization."]
        #[inline(always)]
        pub const fn crinit(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Control Registers Initialization Enable: Indicates which RDIMM buffer chip control registers (RC0 to RC15) should be initialized (written) when the PUB is triggered to initialize the buffer chip. A setting of ‘1’ on CRINIT\\[n\\]
bit means that CRn should be written during initialization."]
        #[inline(always)]
        pub fn set_crinit(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Rdimmgcr1 {
        #[inline(always)]
        fn default() -> Rdimmgcr1 {
            Rdimmgcr1(0)
        }
    }
    #[doc = "Revision Identification Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ridr(pub u32);
    impl Ridr {
        #[doc = "PUB Minor Revision: Indicates minor update of the PUB such as bug fixes. Normally no new features are included."]
        #[inline(always)]
        pub const fn pubmnr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PUB Minor Revision: Indicates minor update of the PUB such as bug fixes. Normally no new features are included."]
        #[inline(always)]
        pub fn set_pubmnr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PUB Moderate Revision: Indicates moderate revision of the PUB such as addition of new features. Normally the new version is still compatible with previous versions."]
        #[inline(always)]
        pub const fn pubmdr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PUB Moderate Revision: Indicates moderate revision of the PUB such as addition of new features. Normally the new version is still compatible with previous versions."]
        #[inline(always)]
        pub fn set_pubmdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "PUB Major Revision: Indicates major revision of the PUB such addition of the features that make the new version not compatible with previous versions."]
        #[inline(always)]
        pub const fn pubmjr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "PUB Major Revision: Indicates major revision of the PUB such addition of the features that make the new version not compatible with previous versions."]
        #[inline(always)]
        pub fn set_pubmjr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "PHY Minor Revision: Indicates minor update of the PHY such as bug fixes. Normally no new features are included."]
        #[inline(always)]
        pub const fn phymnr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "PHY Minor Revision: Indicates minor update of the PHY such as bug fixes. Normally no new features are included."]
        #[inline(always)]
        pub fn set_phymnr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "PHY Moderate Revision: Indicates moderate revision of the PHY such as addition of new features. Normally the new version is still compatible with previous versions."]
        #[inline(always)]
        pub const fn phymdr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "PHY Moderate Revision: Indicates moderate revision of the PHY such as addition of new features. Normally the new version is still compatible with previous versions."]
        #[inline(always)]
        pub fn set_phymdr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "PHY Major Revision: Indicates major revision of the PHY such addition of the features that make the new version not compatible with previous versions."]
        #[inline(always)]
        pub const fn phymjr(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "PHY Major Revision: Indicates major revision of the PHY such addition of the features that make the new version not compatible with previous versions."]
        #[inline(always)]
        pub fn set_phymjr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "User-Defined Revision ID: General purpose revision identification set by the user."]
        #[inline(always)]
        pub const fn udrid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "User-Defined Revision ID: General purpose revision identification set by the user."]
        #[inline(always)]
        pub fn set_udrid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ridr {
        #[inline(always)]
        fn default() -> Ridr {
            Ridr(0)
        }
    }
    #[doc = "Impedance Status Register 0-1 (ZQnSR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr0(pub u32);
    impl Sr0 {
        #[doc = "Impedance Control: Current value of impedance control. ZCTRL field mapping for D3F I/Os is as follows: ZCTRL\\[27:21\\]
is used to select the pull-up on-die termination impedance ZCTRL\\[20:14\\]
is used to select the pull-down on-die termination impedance ZCTRL\\[13:7\\]
is used to select the pull-up output impedance ZCTRL\\[6:0\\]
is used to select the pull-down output impedance ZCTRL field mapping for D3A/B/R I/Os is as follows: ZCTRL\\[27:20\\]
is reserved and returns zeros on reads ZCTRL\\[19:15\\]
is used to select the pull-up on-die termination impedance ZCTRL\\[14:10\\]
is used to select the pull-down on-die termination impedance ZCTRL\\[9:5\\]
is used to select the pull-up output impedance ZCTRL\\[4:0\\]
is used to select the pull-down output impedance Note: The default value is 0x000014A for I/O type D3C/D3R and 0x0001839 for I/O type D3F."]
        #[inline(always)]
        pub const fn zctrl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "Impedance Control: Current value of impedance control. ZCTRL field mapping for D3F I/Os is as follows: ZCTRL\\[27:21\\]
is used to select the pull-up on-die termination impedance ZCTRL\\[20:14\\]
is used to select the pull-down on-die termination impedance ZCTRL\\[13:7\\]
is used to select the pull-up output impedance ZCTRL\\[6:0\\]
is used to select the pull-down output impedance ZCTRL field mapping for D3A/B/R I/Os is as follows: ZCTRL\\[27:20\\]
is reserved and returns zeros on reads ZCTRL\\[19:15\\]
is used to select the pull-up on-die termination impedance ZCTRL\\[14:10\\]
is used to select the pull-down on-die termination impedance ZCTRL\\[9:5\\]
is used to select the pull-up output impedance ZCTRL\\[4:0\\]
is used to select the pull-down output impedance Note: The default value is 0x000014A for I/O type D3C/D3R and 0x0001839 for I/O type D3F."]
        #[inline(always)]
        pub fn set_zctrl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "Impedance Calibration Error: If set, indicates that there was an error during impedance calibration."]
        #[inline(always)]
        pub const fn zerr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Error: If set, indicates that there was an error during impedance calibration."]
        #[inline(always)]
        pub fn set_zerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Impedance Calibration Done: Indicates that impedance calibration has completed."]
        #[inline(always)]
        pub const fn zdone(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Impedance Calibration Done: Indicates that impedance calibration has completed."]
        #[inline(always)]
        pub fn set_zdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sr0 {
        #[inline(always)]
        fn default() -> Sr0 {
            Sr0(0)
        }
    }
    #[doc = "Impedance Status Register 0-1 (ZQnSR0-1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Output impedance pull-down calibration status. Valid status encodings are: 00 = Completed with no errors 01 = Overflow error 10 = Underflow error 11 = Calibration in progress."]
        #[inline(always)]
        pub const fn zpd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Output impedance pull-down calibration status. Valid status encodings are: 00 = Completed with no errors 01 = Overflow error 10 = Underflow error 11 = Calibration in progress."]
        #[inline(always)]
        pub fn set_zpd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Output impedance pull-up calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub const fn zpu(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Output impedance pull-up calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub fn set_zpu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "On-die termination (ODT) pull-down calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub const fn opd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "On-die termination (ODT) pull-down calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub fn set_opd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "On-die termination (ODT) pull-up calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub const fn opu(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "On-die termination (ODT) pull-up calibration status. Similar status encodings as ZPD."]
        #[inline(always)]
        pub fn set_opu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
}
