#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DDRCTL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddrctl {
    ptr: *mut u8,
}
unsafe impl Send for Ddrctl {}
unsafe impl Sync for Ddrctl {}
impl Ddrctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description: Master Register."]
    #[inline(always)]
    pub const fn mstr(self) -> crate::common::Reg<regs::Mstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Description: Operating Mode Status Register."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Description: Mode Register Read/Write Control Register 0."]
    #[inline(always)]
    pub const fn mrctrl0(self) -> crate::common::Reg<regs::Mrctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Description: Mode Register Read/Write Control Register 1."]
    #[inline(always)]
    pub const fn mrctrl1(self) -> crate::common::Reg<regs::Mrctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Description: Mode Register Read/Write Status Register."]
    #[inline(always)]
    pub const fn mrstat(self) -> crate::common::Reg<regs::Mrstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Description: Low Power Control Register."]
    #[inline(always)]
    pub const fn pwrctl(self) -> crate::common::Reg<regs::Pwrctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Description: Low Power Timing Register."]
    #[inline(always)]
    pub const fn pwrtmg(self) -> crate::common::Reg<regs::Pwrtmg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Description: Hardware Low Power Control Register."]
    #[inline(always)]
    pub const fn hwlpctl(self) -> crate::common::Reg<regs::Hwlpctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Description: Refresh Control Register 0."]
    #[inline(always)]
    pub const fn rfshctl0(self) -> crate::common::Reg<regs::Rfshctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Description: Refresh Control Register 1."]
    #[inline(always)]
    pub const fn rfshctl1(self) -> crate::common::Reg<regs::Rfshctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Description: Refresh Control Register 0."]
    #[inline(always)]
    pub const fn rfshctl3(self) -> crate::common::Reg<regs::Rfshctl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Description: Refresh Timing Register."]
    #[inline(always)]
    pub const fn rfshtmg(self) -> crate::common::Reg<regs::Rfshtmg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Description: ECC Uncorrected Error Address Register 0."]
    #[inline(always)]
    pub const fn eccuaddr0(self) -> crate::common::Reg<regs::Eccuaddr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Description: CRC Parity Control Register0."]
    #[inline(always)]
    pub const fn crcparctl0(self) -> crate::common::Reg<regs::Crcparctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Description: CRC Parity Status Register."]
    #[inline(always)]
    pub const fn crcparstat(self) -> crate::common::Reg<regs::Crcparstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Description: SDRAM Initialization Register 0."]
    #[inline(always)]
    pub const fn init0(self) -> crate::common::Reg<regs::Init0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Description: SDRAM Initialization Register 1."]
    #[inline(always)]
    pub const fn init1(self) -> crate::common::Reg<regs::Init1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Description: SDRAM Initialization Register 3."]
    #[inline(always)]
    pub const fn init3(self) -> crate::common::Reg<regs::Init3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Description: SDRAM Initialization Register 4."]
    #[inline(always)]
    pub const fn init4(self) -> crate::common::Reg<regs::Init4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Description: SDRAM Initialization Register 5."]
    #[inline(always)]
    pub const fn init5(self) -> crate::common::Reg<regs::Init5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Description: DIMM Control Register."]
    #[inline(always)]
    pub const fn dimmctl(self) -> crate::common::Reg<regs::Dimmctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Description: Rank Control Register."]
    #[inline(always)]
    pub const fn rankctl(self) -> crate::common::Reg<regs::Rankctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 0."]
    #[inline(always)]
    pub const fn dramtmg0(self) -> crate::common::Reg<regs::Dramtmg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 1."]
    #[inline(always)]
    pub const fn dramtmg1(self) -> crate::common::Reg<regs::Dramtmg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 2."]
    #[inline(always)]
    pub const fn dramtmg2(self) -> crate::common::Reg<regs::Dramtmg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 3."]
    #[inline(always)]
    pub const fn dramtmg3(self) -> crate::common::Reg<regs::Dramtmg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 4."]
    #[inline(always)]
    pub const fn dramtmg4(self) -> crate::common::Reg<regs::Dramtmg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 5."]
    #[inline(always)]
    pub const fn dramtmg5(self) -> crate::common::Reg<regs::Dramtmg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Description: SDRAM Timing Register 8."]
    #[inline(always)]
    pub const fn dramtmg8(self) -> crate::common::Reg<regs::Dramtmg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Description: ZQ Control Register 0."]
    #[inline(always)]
    pub const fn zqctl0(self) -> crate::common::Reg<regs::Zqctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Description: ZQ Control Register 1."]
    #[inline(always)]
    pub const fn zqctl1(self) -> crate::common::Reg<regs::Zqctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Description: ZQ Status Register."]
    #[inline(always)]
    pub const fn zqstat(self) -> crate::common::Reg<regs::Zqstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Description: DFI Timing Register 0."]
    #[inline(always)]
    pub const fn dfitmg0(self) -> crate::common::Reg<regs::Dfitmg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Description: DFI Timing Register 1."]
    #[inline(always)]
    pub const fn dfitmg1(self) -> crate::common::Reg<regs::Dfitmg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Description: DFI Low Power Configuration Register 0."]
    #[inline(always)]
    pub const fn dfilpcfg0(self) -> crate::common::Reg<regs::Dfilpcfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Description: DFI Update Register 0."]
    #[inline(always)]
    pub const fn dfiupd0(self) -> crate::common::Reg<regs::Dfiupd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Description: DFI Update Register 1."]
    #[inline(always)]
    pub const fn dfiupd1(self) -> crate::common::Reg<regs::Dfiupd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Description: DFI Update Register 2."]
    #[inline(always)]
    pub const fn dfiupd2(self) -> crate::common::Reg<regs::Dfiupd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Description: DFI Update Register 3."]
    #[inline(always)]
    pub const fn dfiupd3(self) -> crate::common::Reg<regs::Dfiupd3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Description: DFI Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn dfimisc(self) -> crate::common::Reg<regs::Dfimisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Description: DFI Timing Register 2."]
    #[inline(always)]
    pub const fn dfitmg2(self) -> crate::common::Reg<regs::Dfitmg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Description: Address Map Register 0."]
    #[inline(always)]
    pub const fn addrmap0(self) -> crate::common::Reg<regs::Addrmap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Description: Address Map Register 1."]
    #[inline(always)]
    pub const fn addrmap1(self) -> crate::common::Reg<regs::Addrmap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Description: Address Map Register 2."]
    #[inline(always)]
    pub const fn addrmap2(self) -> crate::common::Reg<regs::Addrmap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Description: Address Map Register 3."]
    #[inline(always)]
    pub const fn addrmap3(self) -> crate::common::Reg<regs::Addrmap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Description: Address Map Register 4."]
    #[inline(always)]
    pub const fn addrmap4(self) -> crate::common::Reg<regs::Addrmap4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Description: Address Map Register 5."]
    #[inline(always)]
    pub const fn addrmap5(self) -> crate::common::Reg<regs::Addrmap5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Description: Address Map Register 6."]
    #[inline(always)]
    pub const fn addrmap6(self) -> crate::common::Reg<regs::Addrmap6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Description: ODT Configuration Register."]
    #[inline(always)]
    pub const fn odtcfg(self) -> crate::common::Reg<regs::Odtcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Description: ODT/Rank Map Register."]
    #[inline(always)]
    pub const fn odtmap(self) -> crate::common::Reg<regs::Odtmap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Description: Scheduler Control Register."]
    #[inline(always)]
    pub const fn sched(self) -> crate::common::Reg<regs::Sched, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Description: Scheduler Control Register 1."]
    #[inline(always)]
    pub const fn sched1(self) -> crate::common::Reg<regs::Sched1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Description: High Priority Read CAM Register 1."]
    #[inline(always)]
    pub const fn perfhpr1(self) -> crate::common::Reg<regs::Perfhpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Description: Low Priority Read CAM Register 1."]
    #[inline(always)]
    pub const fn perflpr1(self) -> crate::common::Reg<regs::Perflpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Description: Write CAM Register 1."]
    #[inline(always)]
    pub const fn perfwr1(self) -> crate::common::Reg<regs::Perfwr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Description: Variable Priority Read CAM Register 1."]
    #[inline(always)]
    pub const fn perfvpr1(self) -> crate::common::Reg<regs::Perfvpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Description: Variable Priority Write CAM Register 1."]
    #[inline(always)]
    pub const fn perfvpw1(self) -> crate::common::Reg<regs::Perfvpw1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Description: Debug Register 0."]
    #[inline(always)]
    pub const fn dbg0(self) -> crate::common::Reg<regs::Dbg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Description: Debug Register 1."]
    #[inline(always)]
    pub const fn dbg1(self) -> crate::common::Reg<regs::Dbg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Description: CAM Debug Register."]
    #[inline(always)]
    pub const fn dbgcam(self) -> crate::common::Reg<regs::Dbgcam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Description: Command Debug Register."]
    #[inline(always)]
    pub const fn dbgcmd(self) -> crate::common::Reg<regs::Dbgcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Description: Status Debug Register."]
    #[inline(always)]
    pub const fn dbgstat(self) -> crate::common::Reg<regs::Dbgstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Description: Port Status Register."]
    #[inline(always)]
    pub const fn pstat(self) -> crate::common::Reg<regs::Pstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Description: Port Common Configuration Register."]
    #[inline(always)]
    pub const fn pccfg(self) -> crate::common::Reg<regs::Pccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pcfg(self, n: usize) -> Pcfg {
        assert!(n < 16usize);
        unsafe { Pcfg::from_ptr(self.ptr.wrapping_add(0x0404usize + n * 176usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn sar(self, n: usize) -> Sar {
        assert!(n < 4usize);
        unsafe { Sar::from_ptr(self.ptr.wrapping_add(0x0f04usize + n * 8usize) as _) }
    }
    #[doc = "Description: Scrubber Control Register."]
    #[inline(always)]
    pub const fn sbrctl(self) -> crate::common::Reg<regs::Sbrctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f24usize) as _) }
    }
    #[doc = "Description: Scrubber Status Register."]
    #[inline(always)]
    pub const fn sbrstat(self) -> crate::common::Reg<regs::Sbrstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f28usize) as _) }
    }
    #[doc = "Description: Scrubber Write Data Pattern0."]
    #[inline(always)]
    pub const fn sbrwdata0(self) -> crate::common::Reg<regs::Sbrwdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f2cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id {
    ptr: *mut u8,
}
unsafe impl Send for Id {}
unsafe impl Sync for Id {}
impl Id {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description: Port n Channel m Configuration ID Mask Register."]
    #[inline(always)]
    pub const fn maskch(self) -> crate::common::Reg<regs::Maskch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Description: Port n Channel m Configuration ID Value Register."]
    #[inline(always)]
    pub const fn valuech(self) -> crate::common::Reg<regs::Valuech, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcfg {
    ptr: *mut u8,
}
unsafe impl Send for Pcfg {}
unsafe impl Sync for Pcfg {}
impl Pcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description: Port n Configuration Read Register."]
    #[inline(always)]
    pub const fn r(self) -> crate::common::Reg<regs::R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Description: Port n Configuration Write Register."]
    #[inline(always)]
    pub const fn w(self) -> crate::common::Reg<regs::W, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Description: Port n Common Configuration Register."]
    #[inline(always)]
    pub const fn c(self) -> crate::common::Reg<regs::C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn id(self, n: usize) -> Id {
        assert!(n < 16usize);
        unsafe { Id::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 8usize) as _) }
    }
    #[doc = "Description: Port n Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Description: Port n Read QoS Configuration Register 0."]
    #[inline(always)]
    pub const fn qos0(self) -> crate::common::Reg<regs::Qos0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Description: Port n Read QoS Configuration Register 1."]
    #[inline(always)]
    pub const fn qos1(self) -> crate::common::Reg<regs::Qos1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Description: Port n Write QoS Configuration Register 0."]
    #[inline(always)]
    pub const fn wqos0(self) -> crate::common::Reg<regs::Wqos0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Description: Port n Write QoS Configuration Register 1."]
    #[inline(always)]
    pub const fn wqos1(self) -> crate::common::Reg<regs::Wqos1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sar {
    ptr: *mut u8,
}
unsafe impl Send for Sar {}
unsafe impl Sync for Sar {}
impl Sar {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Description: SAR Base Address Register n."]
    #[inline(always)]
    pub const fn base(self) -> crate::common::Reg<regs::Base, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Description: SAR Size Register n."]
    #[inline(always)]
    pub const fn size(self) -> crate::common::Reg<regs::Size, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
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
    #[doc = "Description: Address Map Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap0(pub u32);
    impl Addrmap0 {
        #[doc = "Description: Selects the HIF address bit used as rank address bit 0. Valid Range: 0 to 27, and 31 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, rank address bit 0 is set to 0. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_cs_bit0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as rank address bit 0. Valid Range: 0 to 27, and 31 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, rank address bit 0 is set to 0. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_addrmap_cs_bit0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Addrmap0 {
        #[inline(always)]
        fn default() -> Addrmap0 {
            Addrmap0(0)
        }
    }
    impl core::fmt::Debug for Addrmap0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap0")
                .field("addrmap_cs_bit0", &self.addrmap_cs_bit0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Addrmap0 {{ addrmap_cs_bit0: {=u8:?} }}",
                self.addrmap_cs_bit0()
            )
        }
    }
    #[doc = "Description: Address Map Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap1(pub u32);
    impl Addrmap1 {
        #[doc = "Description: Selects the HIF address bits used as bank address bit 0. Valid Range: 0 to 30 Internal Base: 2 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_bank_b0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bits used as bank address bit 0. Valid Range: 0 to 30 Internal Base: 2 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_bank_b0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Description: Selects the HIF address bits used as bank address bit 1. Valid Range: 0 to 30 Internal Base: 3 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_bank_b1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bits used as bank address bit 1. Valid Range: 0 to 30 Internal Base: 3 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_bank_b1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Description: Selects the HIF address bit used as bank address bit 2. Valid Range: 0 to 29 and 31 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, bank address bit 2 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_bank_b2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as bank address bit 2. Valid Range: 0 to 29 and 31 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, bank address bit 2 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_bank_b2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Addrmap1 {
        #[inline(always)]
        fn default() -> Addrmap1 {
            Addrmap1(0)
        }
    }
    impl core::fmt::Debug for Addrmap1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap1")
                .field("addrmap_bank_b0", &self.addrmap_bank_b0())
                .field("addrmap_bank_b1", &self.addrmap_bank_b1())
                .field("addrmap_bank_b2", &self.addrmap_bank_b2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Addrmap1 {{ addrmap_bank_b0: {=u8:?}, addrmap_bank_b1: {=u8:?}, addrmap_bank_b2: {=u8:?} }}" , self . addrmap_bank_b0 () , self . addrmap_bank_b1 () , self . addrmap_bank_b2 ())
        }
    }
    #[doc = "Description: Address Map Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap2(pub u32);
    impl Addrmap2 {
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 2 (if MEMC_BURST_LENGTH = 4) or 3 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 2 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 2 (if MEMC_BURST_LENGTH = 4) or 3 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 2 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 3 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 3 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 5 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b5(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 5 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Addrmap2 {
        #[inline(always)]
        fn default() -> Addrmap2 {
            Addrmap2(0)
        }
    }
    impl core::fmt::Debug for Addrmap2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap2")
                .field("addrmap_col_b2", &self.addrmap_col_b2())
                .field("addrmap_col_b3", &self.addrmap_col_b3())
                .field("addrmap_col_b4", &self.addrmap_col_b4())
                .field("addrmap_col_b5", &self.addrmap_col_b5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Addrmap2 {{ addrmap_col_b2: {=u8:?}, addrmap_col_b3: {=u8:?}, addrmap_col_b4: {=u8:?}, addrmap_col_b5: {=u8:?} }}" , self . addrmap_col_b2 () , self . addrmap_col_b3 () , self . addrmap_col_b4 () , self . addrmap_col_b5 ())
        }
    }
    #[doc = "Description: Address Map Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap3(pub u32);
    impl Addrmap3 {
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b6(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 7 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b7(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 7 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 8 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b8(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 8 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8) Half bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). (Column address bit 11 in LPDDR2/LPDDR3 mode) Quarter bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 9 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b9(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8) Half bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). (Column address bit 11 in LPDDR2/LPDDR3 mode) Quarter bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 9 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Addrmap3 {
        #[inline(always)]
        fn default() -> Addrmap3 {
            Addrmap3(0)
        }
    }
    impl core::fmt::Debug for Addrmap3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap3")
                .field("addrmap_col_b6", &self.addrmap_col_b6())
                .field("addrmap_col_b7", &self.addrmap_col_b7())
                .field("addrmap_col_b8", &self.addrmap_col_b8())
                .field("addrmap_col_b9", &self.addrmap_col_b9())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Addrmap3 {{ addrmap_col_b6: {=u8:?}, addrmap_col_b7: {=u8:?}, addrmap_col_b8: {=u8:?}, addrmap_col_b9: {=u8:?} }}" , self . addrmap_col_b6 () , self . addrmap_col_b7 () , self . addrmap_col_b8 () , self . addrmap_col_b9 ())
        }
    }
    #[doc = "Description: Address Map Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap4(pub u32);
    impl Addrmap4 {
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8) Quarter bus width mode: UNUSED. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 10 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8) Quarter bus width mode: UNUSED. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 10 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Half bus width mode: Unused. To make it unused, this should be tied to 4'hF. Quarter bus width mode: Unused. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 11 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_col_b11(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Full bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Half bus width mode: Unused. To make it unused, this should be tied to 4'hF. Quarter bus width mode: Unused. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 11 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_col_b11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Addrmap4 {
        #[inline(always)]
        fn default() -> Addrmap4 {
            Addrmap4(0)
        }
    }
    impl core::fmt::Debug for Addrmap4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap4")
                .field("addrmap_col_b10", &self.addrmap_col_b10())
                .field("addrmap_col_b11", &self.addrmap_col_b11())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Addrmap4 {{ addrmap_col_b10: {=u8:?}, addrmap_col_b11: {=u8:?} }}",
                self.addrmap_col_b10(),
                self.addrmap_col_b11()
            )
        }
    }
    #[doc = "Description: Address Map Register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap5(pub u32);
    impl Addrmap5 {
        #[doc = "Description: Selects the HIF address bits used as row address bit 0. Valid Range: 0 to 11 Internal Base: 6 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bits used as row address bit 0. Valid Range: 0 to 11 Internal Base: 6 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Selects the HIF address bits used as row address bit 1. Valid Range: 0 to 11 Internal Base: 7 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bits used as row address bit 1. Valid Range: 0 to 11 Internal Base: 7 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Selects the HIF address bits used as row address bits 2 to 10. Valid Range: 0 to 11 Internal Base: 8 (for row address bit 2), 9 (for row address bit 3), 10 (for row address bit 4) etc increasing to 16 (for row address bit 10) The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b2_10(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bits used as row address bits 2 to 10. Valid Range: 0 to 11 Internal Base: 8 (for row address bit 2), 9 (for row address bit 3), 10 (for row address bit 4) etc increasing to 16 (for row address bit 10) The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b2_10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 11. Valid Range: 0 to 11, and 15 Internal Base: 17 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 11 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b11(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 11. Valid Range: 0 to 11, and 15 Internal Base: 17 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 11 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Addrmap5 {
        #[inline(always)]
        fn default() -> Addrmap5 {
            Addrmap5(0)
        }
    }
    impl core::fmt::Debug for Addrmap5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap5")
                .field("addrmap_row_b0", &self.addrmap_row_b0())
                .field("addrmap_row_b1", &self.addrmap_row_b1())
                .field("addrmap_row_b2_10", &self.addrmap_row_b2_10())
                .field("addrmap_row_b11", &self.addrmap_row_b11())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Addrmap5 {{ addrmap_row_b0: {=u8:?}, addrmap_row_b1: {=u8:?}, addrmap_row_b2_10: {=u8:?}, addrmap_row_b11: {=u8:?} }}" , self . addrmap_row_b0 () , self . addrmap_row_b1 () , self . addrmap_row_b2_10 () , self . addrmap_row_b11 ())
        }
    }
    #[doc = "Description: Address Map Register 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrmap6(pub u32);
    impl Addrmap6 {
        #[doc = "Description: Selects the HIF address bit used as row address bit 12. Valid Range: 0 to 11, and 15 Internal Base: 18 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 12 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b12(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 12. Valid Range: 0 to 11, and 15 Internal Base: 18 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 12 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 13. Valid Range: 0 to 11, and 15 Internal Base: 19 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 13 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b13(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 13. Valid Range: 0 to 11, and 15 Internal Base: 19 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 13 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 14. Valid Range: 0 to 11, and 15 Internal Base: 20 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 14 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b14(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 14. Valid Range: 0 to 11, and 15 Internal Base: 20 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 14 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 15. Valid Range: 0 to 11, and 15 Internal Base: 21 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 15 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn addrmap_row_b15(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Selects the HIF address bit used as row address bit 15. Valid Range: 0 to 11, and 15 Internal Base: 21 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 15 is set to 0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_addrmap_row_b15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Addrmap6 {
        #[inline(always)]
        fn default() -> Addrmap6 {
            Addrmap6(0)
        }
    }
    impl core::fmt::Debug for Addrmap6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrmap6")
                .field("addrmap_row_b12", &self.addrmap_row_b12())
                .field("addrmap_row_b13", &self.addrmap_row_b13())
                .field("addrmap_row_b14", &self.addrmap_row_b14())
                .field("addrmap_row_b15", &self.addrmap_row_b15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrmap6 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Addrmap6 {{ addrmap_row_b12: {=u8:?}, addrmap_row_b13: {=u8:?}, addrmap_row_b14: {=u8:?}, addrmap_row_b15: {=u8:?} }}" , self . addrmap_row_b12 () , self . addrmap_row_b13 () , self . addrmap_row_b14 () , self . addrmap_row_b15 ())
        }
    }
    #[doc = "Description: SAR Base Address Register n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Base(pub u32);
    impl Base {
        #[doc = "Description: Base address for address region n specified as awaddr\\[UMCTL2_A_ADDRW-1:x\\]
and araddr\\[UMCTL2_A_ADDRW-1:x\\]
where x is determined by the minimum block size parameter UMCTL2_SARMINSIZE: (x=log2(block size)). Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn base_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Description: Base address for address region n specified as awaddr\\[UMCTL2_A_ADDRW-1:x\\]
and araddr\\[UMCTL2_A_ADDRW-1:x\\]
where x is determined by the minimum block size parameter UMCTL2_SARMINSIZE: (x=log2(block size)). Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_base_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Base {
        #[inline(always)]
        fn default() -> Base {
            Base(0)
        }
    }
    impl core::fmt::Debug for Base {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Base")
                .field("base_addr", &self.base_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Base {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Base {{ base_addr: {=u32:?} }}", self.base_addr())
        }
    }
    #[doc = "Description: Port n Common Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C(pub u32);
    impl C {
        #[doc = "Description: If set to 0, enables support for little endian on the AHB port. If set to 1, enables support for big endian (BE- 32) on the AHB port. If set to 2, enables support for big endian (BE-A) on the AHB port. Value After Reset: 0x0 Exists: UMCTL2_A_AHB_n==1."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb_endianness(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Description: If set to 0, enables support for little endian on the AHB port. If set to 1, enables support for big endian (BE- 32) on the AHB port. If set to 2, enables support for big endian (BE-A) on the AHB port. Value After Reset: 0x0 Exists: UMCTL2_A_AHB_n==1."]
        #[inline(always)]
        pub const fn set_ahb_endianness(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for C {
        #[inline(always)]
        fn default() -> C {
            C(0)
        }
    }
    impl core::fmt::Debug for C {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C")
                .field("ahb_endianness", &self.ahb_endianness())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "C {{ ahb_endianness: {=u8:?} }}", self.ahb_endianness())
        }
    }
    #[doc = "Description: CRC Parity Control Register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcparctl0(pub u32);
    impl Crcparctl0 {
        #[doc = "Description: Interrupt enable bit for DFI alert error. If this bit is set, any parity/CRC error detected on the dfi_alert_n input will result in an interrupt being set on CRCPARSTAT.dfi_alert_err_int. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_alert_err_int_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Interrupt enable bit for DFI alert error. If this bit is set, any parity/CRC error detected on the dfi_alert_n input will result in an interrupt being set on CRCPARSTAT.dfi_alert_err_int. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_alert_err_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Interrupt clear bit for DFI alert error. If this bit is set, the alert error interrupt on CRCPARSTAT.dfi_alert_err_int will be cleared. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_alert_err_int_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Interrupt clear bit for DFI alert error. If this bit is set, the alert error interrupt on CRCPARSTAT.dfi_alert_err_int will be cleared. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_alert_err_int_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: DFI alert error count clear. Clear bit for DFI alert error counter. Asserting this bit will clear the DFI alert error counter, CRCPARSTAT.dfi_alert_err_cnt. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_alert_err_cnt_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Description: DFI alert error count clear. Clear bit for DFI alert error counter. Asserting this bit will clear the DFI alert error counter, CRCPARSTAT.dfi_alert_err_cnt. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_alert_err_cnt_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Crcparctl0 {
        #[inline(always)]
        fn default() -> Crcparctl0 {
            Crcparctl0(0)
        }
    }
    impl core::fmt::Debug for Crcparctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcparctl0")
                .field("dfi_alert_err_int_en", &self.dfi_alert_err_int_en())
                .field("dfi_alert_err_int_clr", &self.dfi_alert_err_int_clr())
                .field("dfi_alert_err_cnt_clr", &self.dfi_alert_err_cnt_clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcparctl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Crcparctl0 {{ dfi_alert_err_int_en: {=bool:?}, dfi_alert_err_int_clr: {=bool:?}, dfi_alert_err_cnt_clr: {=bool:?} }}" , self . dfi_alert_err_int_en () , self . dfi_alert_err_int_clr () , self . dfi_alert_err_cnt_clr ())
        }
    }
    #[doc = "Description: CRC Parity Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcparstat(pub u32);
    impl Crcparstat {
        #[doc = "Description: DFI alert error count. If a parity/CRC error is detected on dfi_alert_n, this counter be incremented. This is independent of the setting of CRCPARCTL0.dfi_alert_err_int_en. It will saturate at 0xFFFF, and can be cleared by asserting CRCPARCTL0.dfi_alert_err_cnt_clr. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_alert_err_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: DFI alert error count. If a parity/CRC error is detected on dfi_alert_n, this counter be incremented. This is independent of the setting of CRCPARCTL0.dfi_alert_err_int_en. It will saturate at 0xFFFF, and can be cleared by asserting CRCPARCTL0.dfi_alert_err_cnt_clr. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_alert_err_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: DFI alert error interrupt. If a parity/CRC error is detected on dfi_alert_n, and the interrupt is enabled by CRCPARCTL0.dfi_alert_err_int_en, this interrupt bit will be set. It will remain set until cleared by CRCPARCTL0.dfi_alert_err_int_clr Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_alert_err_int(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Description: DFI alert error interrupt. If a parity/CRC error is detected on dfi_alert_n, and the interrupt is enabled by CRCPARCTL0.dfi_alert_err_int_en, this interrupt bit will be set. It will remain set until cleared by CRCPARCTL0.dfi_alert_err_int_clr Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_alert_err_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Crcparstat {
        #[inline(always)]
        fn default() -> Crcparstat {
            Crcparstat(0)
        }
    }
    impl core::fmt::Debug for Crcparstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcparstat")
                .field("dfi_alert_err_cnt", &self.dfi_alert_err_cnt())
                .field("dfi_alert_err_int", &self.dfi_alert_err_int())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcparstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Crcparstat {{ dfi_alert_err_cnt: {=u16:?}, dfi_alert_err_int: {=bool:?} }}",
                self.dfi_alert_err_cnt(),
                self.dfi_alert_err_int()
            )
        }
    }
    #[doc = "Description: Port n Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Description: Enables port n. Value After Reset: \"UMCTL2_PORT_EN_RESET_VALUE\" Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enables port n. Value After Reset: \"UMCTL2_PORT_EN_RESET_VALUE\" Exists: Always."]
        #[inline(always)]
        pub const fn set_port_en(&mut self, val: bool) {
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
            f.debug_struct("Ctrl")
                .field("port_en", &self.port_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctrl {{ port_en: {=bool:?} }}", self.port_en())
        }
    }
    #[doc = "Description: Debug Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbg0(pub u32);
    impl Dbg0 {
        #[doc = "Description: When 1, disable write combine. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_wc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When 1, disable write combine. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_wc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Only present in designs supporting read bypass. When 1, disable bypass path for high priority read page hits FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_rd_bypass(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Only present in designs supporting read bypass. When 1, disable bypass path for high priority read page hits FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1."]
        #[inline(always)]
        pub const fn set_dis_rd_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: Only present in designs supporting activate bypass. When 1, disable bypass path for high priority read activates FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_act_bypass(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Only present in designs supporting activate bypass. When 1, disable bypass path for high priority read activates FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1."]
        #[inline(always)]
        pub const fn set_dis_act_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Description: When this is set to '0', auto-precharge is disabled for the flushed command in a collision case. Collision cases are write followed by read to same address, read followed by write to same address, or write followed by write to same address with DBG0.dis_wc bit = 1 (where same address comparisons exclude the two address bits representing critical word). FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_collision_page_opt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When this is set to '0', auto-precharge is disabled for the flushed command in a collision case. Collision cases are write followed by read to same address, read followed by write to same address, or write followed by write to same address with DBG0.dis_wc bit = 1 (where same address comparisons exclude the two address bits representing critical word). FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_collision_page_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Dbg0 {
        #[inline(always)]
        fn default() -> Dbg0 {
            Dbg0(0)
        }
    }
    impl core::fmt::Debug for Dbg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbg0")
                .field("dis_wc", &self.dis_wc())
                .field("dis_rd_bypass", &self.dis_rd_bypass())
                .field("dis_act_bypass", &self.dis_act_bypass())
                .field("dis_collision_page_opt", &self.dis_collision_page_opt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbg0 {{ dis_wc: {=bool:?}, dis_rd_bypass: {=bool:?}, dis_act_bypass: {=bool:?}, dis_collision_page_opt: {=bool:?} }}" , self . dis_wc () , self . dis_rd_bypass () , self . dis_act_bypass () , self . dis_collision_page_opt ())
        }
    }
    #[doc = "Description: Debug Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbg1(pub u32);
    impl Dbg1 {
        #[doc = "Description: When 1, uMCTL2 will not de-queue any transactions from the CAM. Bypass is also disabled. All transactions are queued in the CAM. No reads or writes are issued to SDRAM as long as this is asserted. This bit may be used to prevent reads or writes being issued by the uMCTL2, which makes it safe to modify certain register fields associated with reads and writes (see User Guide for details). After setting this bit, it is strongly recommended to poll DBGCAM.wr_data_pipeline_empty and DBGCAM.rd_data_pipeline_empty, before making changes to any registers which affect reads and writes. This will ensure that the relevant logic in the DDRC is idle. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_dq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When 1, uMCTL2 will not de-queue any transactions from the CAM. Bypass is also disabled. All transactions are queued in the CAM. No reads or writes are issued to SDRAM as long as this is asserted. This bit may be used to prevent reads or writes being issued by the uMCTL2, which makes it safe to modify certain register fields associated with reads and writes (see User Guide for details). After setting this bit, it is strongly recommended to poll DBGCAM.wr_data_pipeline_empty and DBGCAM.rd_data_pipeline_empty, before making changes to any registers which affect reads and writes. This will ensure that the relevant logic in the DDRC is idle. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_dq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: When 1, uMCTL2 asserts the HIF command ih_co_stall. uMCTL2 will ignore the co_ih_rxcmd_valid and all other associated request signals. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_hif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When 1, uMCTL2 asserts the HIF command ih_co_stall. uMCTL2 will ignore the co_ih_rxcmd_valid and all other associated request signals. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_hif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dbg1 {
        #[inline(always)]
        fn default() -> Dbg1 {
            Dbg1(0)
        }
    }
    impl core::fmt::Debug for Dbg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbg1")
                .field("dis_dq", &self.dis_dq())
                .field("dis_hif", &self.dis_hif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbg1 {{ dis_dq: {=bool:?}, dis_hif: {=bool:?} }}",
                self.dis_dq(),
                self.dis_hif()
            )
        }
    }
    #[doc = "Description: CAM Debug Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcam(pub u32);
    impl Dbgcam {
        #[doc = "Description: High priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 6 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_hpr_q_depth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: High priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 6 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_hpr_q_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Description: Low priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 14 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_lpr_q_depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: Low priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 14 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_lpr_q_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Description: Write queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 22 is reserved. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_w_q_depth(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: Write queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 22 is reserved. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_w_q_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Description: Stall FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_stall(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Stall FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Description: When 1, all the Read command queues and Read data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_rd_q_empty(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When 1, all the Read command queues and Read data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_rd_q_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Description: When 1, all the Write command queues and Write data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_wr_q_empty(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When 1, all the Write command queues and Write data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dbg_wr_q_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Description: This bit indicates that the read data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_data_pipeline_empty(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Description: This bit indicates that the read data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_data_pipeline_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Description: This bit indicates that the write data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_data_pipeline_empty(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Description: This bit indicates that the write data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_data_pipeline_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Dbgcam {
        #[inline(always)]
        fn default() -> Dbgcam {
            Dbgcam(0)
        }
    }
    impl core::fmt::Debug for Dbgcam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcam")
                .field("dbg_hpr_q_depth", &self.dbg_hpr_q_depth())
                .field("dbg_lpr_q_depth", &self.dbg_lpr_q_depth())
                .field("dbg_w_q_depth", &self.dbg_w_q_depth())
                .field("dbg_stall", &self.dbg_stall())
                .field("dbg_rd_q_empty", &self.dbg_rd_q_empty())
                .field("dbg_wr_q_empty", &self.dbg_wr_q_empty())
                .field("rd_data_pipeline_empty", &self.rd_data_pipeline_empty())
                .field("wr_data_pipeline_empty", &self.wr_data_pipeline_empty())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbgcam {{ dbg_hpr_q_depth: {=u8:?}, dbg_lpr_q_depth: {=u8:?}, dbg_w_q_depth: {=u8:?}, dbg_stall: {=bool:?}, dbg_rd_q_empty: {=bool:?}, dbg_wr_q_empty: {=bool:?}, rd_data_pipeline_empty: {=bool:?}, wr_data_pipeline_empty: {=bool:?} }}" , self . dbg_hpr_q_depth () , self . dbg_lpr_q_depth () , self . dbg_w_q_depth () , self . dbg_stall () , self . dbg_rd_q_empty () , self . dbg_wr_q_empty () , self . rd_data_pipeline_empty () , self . wr_data_pipeline_empty ())
        }
    }
    #[doc = "Description: Command Debug Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcmd(pub u32);
    impl Dbgcmd {
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 0. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rank0_refresh(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 0. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rank0_refresh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 1. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn rank1_refresh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 1. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_rank1_refresh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a ZQCS (ZQ calibration short) command to the SDRAM. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when ZQCTL0.dis_auto_zq=1. It is recommended NOT to set this register bit if in Init operating mode. This register bit is ignored when in Self-Refresh and Deep power-down operating modes. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn zq_calib_short(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a ZQCS (ZQ calibration short) command to the SDRAM. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when ZQCTL0.dis_auto_zq=1. It is recommended NOT to set this register bit if in Init operating mode. This register bit is ignored when in Self-Refresh and Deep power-down operating modes. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_zq_calib_short(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a dfi_ctrlupd_req to the PHY. When this request is stored in uMCTL2, the bit is automatically cleared. This operation must only be performed when DFIUPD0.dis_auto_ctrlupd=1. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrlupd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a dfi_ctrlupd_req to the PHY. When this request is stored in uMCTL2, the bit is automatically cleared. This operation must only be performed when DFIUPD0.dis_auto_ctrlupd=1. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_ctrlupd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Dbgcmd {
        #[inline(always)]
        fn default() -> Dbgcmd {
            Dbgcmd(0)
        }
    }
    impl core::fmt::Debug for Dbgcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcmd")
                .field("rank0_refresh", &self.rank0_refresh())
                .field("rank1_refresh", &self.rank1_refresh())
                .field("zq_calib_short", &self.zq_calib_short())
                .field("ctrlupd", &self.ctrlupd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcmd {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbgcmd {{ rank0_refresh: {=bool:?}, rank1_refresh: {=bool:?}, zq_calib_short: {=bool:?}, ctrlupd: {=bool:?} }}" , self . rank0_refresh () , self . rank1_refresh () , self . zq_calib_short () , self . ctrlupd ())
        }
    }
    #[doc = "Description: Status Debug Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgstat(pub u32);
    impl Dbgstat {
        #[doc = "Description: SoC core may initiate a rank0_refresh operation (refresh operation to rank 0) only if this signal is low. This signal goes high in the clock after DBGCMD.rank0_refresh is set to one. It goes low when the rank0_refresh operation is stored in uMCTL2. It is recommended not to perform rank0_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank0_refresh operation 1 - Indicates that rank0_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rank0_refresh_busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: SoC core may initiate a rank0_refresh operation (refresh operation to rank 0) only if this signal is low. This signal goes high in the clock after DBGCMD.rank0_refresh is set to one. It goes low when the rank0_refresh operation is stored in uMCTL2. It is recommended not to perform rank0_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank0_refresh operation 1 - Indicates that rank0_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rank0_refresh_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: SoC core may initiate a rank1_refresh operation (refresh operation to rank 1) only if this signal is low. This signal goes high in the clock after DBGCMD.rank1_refresh is set to one. It goes low when the rank1_refresh operation is stored in uMCTL2. It is recommended not to perform rank1_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank1_refresh operation 1 - Indicates that rank1_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn rank1_refresh_busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: SoC core may initiate a rank1_refresh operation (refresh operation to rank 1) only if this signal is low. This signal goes high in the clock after DBGCMD.rank1_refresh is set to one. It goes low when the rank1_refresh operation is stored in uMCTL2. It is recommended not to perform rank1_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank1_refresh operation 1 - Indicates that rank1_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_rank1_refresh_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: SoC core may initiate a ZQCS (ZQ calibration short) operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQCS request. It goes low when the ZQCS operation is initiated in uMCTL2. It is recommended not to perform ZQCS operations when this signal is high. 0 - Indicates that the SoC core can initiate a ZQCS operation 1 - Indicates that ZQCS operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn zq_calib_short_busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Description: SoC core may initiate a ZQCS (ZQ calibration short) operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQCS request. It goes low when the ZQCS operation is initiated in uMCTL2. It is recommended not to perform ZQCS operations when this signal is high. 0 - Indicates that the SoC core can initiate a ZQCS operation 1 - Indicates that ZQCS operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_zq_calib_short_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Description: SoC core may initiate a ctrlupd operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ctrlupd request. It goes low when the ctrlupd operation is initiated in uMCTL2. It is recommended not to perform ctrlupd operations when this signal is high. 0 - Indicates that the SoC core can initiate a ctrlupd operation 1 - Indicates that ctrlupd operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrlupd_busy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Description: SoC core may initiate a ctrlupd operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ctrlupd request. It goes low when the ctrlupd operation is initiated in uMCTL2. It is recommended not to perform ctrlupd operations when this signal is high. 0 - Indicates that the SoC core can initiate a ctrlupd operation 1 - Indicates that ctrlupd operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_ctrlupd_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Dbgstat {
        #[inline(always)]
        fn default() -> Dbgstat {
            Dbgstat(0)
        }
    }
    impl core::fmt::Debug for Dbgstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgstat")
                .field("rank0_refresh_busy", &self.rank0_refresh_busy())
                .field("rank1_refresh_busy", &self.rank1_refresh_busy())
                .field("zq_calib_short_busy", &self.zq_calib_short_busy())
                .field("ctrlupd_busy", &self.ctrlupd_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgstat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dbgstat {{ rank0_refresh_busy: {=bool:?}, rank1_refresh_busy: {=bool:?}, zq_calib_short_busy: {=bool:?}, ctrlupd_busy: {=bool:?} }}" , self . rank0_refresh_busy () , self . rank1_refresh_busy () , self . zq_calib_short_busy () , self . ctrlupd_busy ())
        }
    }
    #[doc = "Description: DFI Low Power Configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfilpcfg0(pub u32);
    impl Dfilpcfg0 {
        #[doc = "Description: Enables DFI Low Power interface handshaking during Power Down Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_lp_en_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enables DFI Low Power interface handshaking during Power Down Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_lp_en_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Value to drive on dfi_lp_wakeup signal when Power Down mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_lp_wakeup_pd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Value to drive on dfi_lp_wakeup signal when Power Down mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_lp_wakeup_pd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Description: Enables DFI Low Power interface handshaking during Self Refresh Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_lp_en_sr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enables DFI Low Power interface handshaking during Self Refresh Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_lp_en_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Description: Value to drive on dfi_lp_wakeup signal when Self Refresh mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_lp_wakeup_sr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Value to drive on dfi_lp_wakeup signal when Self Refresh mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_lp_wakeup_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Description: Setting for DFI's tlp_resp time. Same value is used for both Power Down, Self Refresh, Deep Power Down and Maximum Power Saving modes. DFI 2.1 specification onwards, recommends using a fixed value of 7 always. Value After Reset: 0x7 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_tlp_resp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Setting for DFI's tlp_resp time. Same value is used for both Power Down, Self Refresh, Deep Power Down and Maximum Power Saving modes. DFI 2.1 specification onwards, recommends using a fixed value of 7 always. Value After Reset: 0x7 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_tlp_resp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Dfilpcfg0 {
        #[inline(always)]
        fn default() -> Dfilpcfg0 {
            Dfilpcfg0(0)
        }
    }
    impl core::fmt::Debug for Dfilpcfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfilpcfg0")
                .field("dfi_lp_en_pd", &self.dfi_lp_en_pd())
                .field("dfi_lp_wakeup_pd", &self.dfi_lp_wakeup_pd())
                .field("dfi_lp_en_sr", &self.dfi_lp_en_sr())
                .field("dfi_lp_wakeup_sr", &self.dfi_lp_wakeup_sr())
                .field("dfi_tlp_resp", &self.dfi_tlp_resp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfilpcfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfilpcfg0 {{ dfi_lp_en_pd: {=bool:?}, dfi_lp_wakeup_pd: {=u8:?}, dfi_lp_en_sr: {=bool:?}, dfi_lp_wakeup_sr: {=u8:?}, dfi_tlp_resp: {=u8:?} }}" , self . dfi_lp_en_pd () , self . dfi_lp_wakeup_pd () , self . dfi_lp_en_sr () , self . dfi_lp_wakeup_sr () , self . dfi_tlp_resp ())
        }
    }
    #[doc = "Description: DFI Miscellaneous Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfimisc(pub u32);
    impl Dfimisc {
        #[doc = "Description: PHY initialization complete enable signal. When asserted the dfi_init_complete signal can be used to trigger SDRAM initialisation Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_init_complete_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: PHY initialization complete enable signal. When asserted the dfi_init_complete signal can be used to trigger SDRAM initialisation Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_init_complete_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dfimisc {
        #[inline(always)]
        fn default() -> Dfimisc {
            Dfimisc(0)
        }
    }
    impl core::fmt::Debug for Dfimisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfimisc")
                .field("dfi_init_complete_en", &self.dfi_init_complete_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfimisc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfimisc {{ dfi_init_complete_en: {=bool:?} }}",
                self.dfi_init_complete_en()
            )
        }
    }
    #[doc = "Description: DFI Timing Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfitmg0(pub u32);
    impl Dfitmg0 {
        #[doc = "Description: Write latency Number of clocks from the write command to write data enable (dfi_wrdata_en). This corresponds to the DFI timing parameter tphy_wrlat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value.Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of tphy_wrlat. This is to compensate for the extra cycle of latency through the RDIMM. Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_tphy_wrlat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Write latency Number of clocks from the write command to write data enable (dfi_wrdata_en). This corresponds to the DFI timing parameter tphy_wrlat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value.Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of tphy_wrlat. This is to compensate for the extra cycle of latency through the RDIMM. Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_tphy_wrlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Description: Specifies the number of clock cycles between when dfi_wrdata_en is asserted to when the associated write data is driven on the dfi_wrdata signal. This corresponds to the DFI timing parameter tphy_wrdata. Refer to PHY specification for correct value. Note, max supported value is 8. Unit: Clocks Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_tphy_wrdata(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Specifies the number of clock cycles between when dfi_wrdata_en is asserted to when the associated write data is driven on the dfi_wrdata signal. This corresponds to the DFI timing parameter tphy_wrdata. Refer to PHY specification for correct value. Note, max supported value is 8. Unit: Clocks Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_tphy_wrdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Description: Defines whether dfi_wrdata_en/dfi_wrdata/dfi_wrdata_mask is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_tphy_wrlat is in terms of SDR or HDR clock cycles Selects whether value in DFITMG0.dfi_tphy_wrdata is in terms of SDR or HDR clock cycles 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_wrdata_use_sdr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Defines whether dfi_wrdata_en/dfi_wrdata/dfi_wrdata_mask is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_tphy_wrlat is in terms of SDR or HDR clock cycles Selects whether value in DFITMG0.dfi_tphy_wrdata is in terms of SDR or HDR clock cycles 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2."]
        #[inline(always)]
        pub const fn set_dfi_wrdata_use_sdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Description: Time from the assertion of a read command on the DFI interface to the assertion of the dfi_rddata_en signal. Refer to PHY specification for correct value. This corresponds to the DFI parameter trddata_en. Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of trddata_en. This is to compensate for the extra cycle of latency through the RDIMM. Unit: Clocks Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_rddata_en(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Time from the assertion of a read command on the DFI interface to the assertion of the dfi_rddata_en signal. Refer to PHY specification for correct value. This corresponds to the DFI parameter trddata_en. Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of trddata_en. This is to compensate for the extra cycle of latency through the RDIMM. Unit: Clocks Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_rddata_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Description: Defines whether dfi_rddata_en/dfi_rddata/dfi_rddata_valid is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_t_rddata_en is in terms of SDR or HDR clock cycles: 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_rddata_use_sdr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Defines whether dfi_rddata_en/dfi_rddata/dfi_rddata_valid is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_t_rddata_en is in terms of SDR or HDR clock cycles: 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2."]
        #[inline(always)]
        pub const fn set_dfi_rddata_use_sdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Description: Specifies the number of DFI clock cycles after an assertion or de-assertion of the DFI control signals that the control signals at the PHY-DRAM interface reflect the assertion or de-assertion. If the DFI clock and the memory clock are not phase-aligned, this timing parameter should be rounded up to the next integer value. Note that if using RDIMM, depending on the PHY, it may be necessary to increment this parameter by 1. This is to compensate for the extra cycle of latency through the RDIMM Value After Reset: 0x7 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_ctrl_delay(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Specifies the number of DFI clock cycles after an assertion or de-assertion of the DFI control signals that the control signals at the PHY-DRAM interface reflect the assertion or de-assertion. If the DFI clock and the memory clock are not phase-aligned, this timing parameter should be rounded up to the next integer value. Note that if using RDIMM, depending on the PHY, it may be necessary to increment this parameter by 1. This is to compensate for the extra cycle of latency through the RDIMM Value After Reset: 0x7 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_ctrl_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Dfitmg0 {
        #[inline(always)]
        fn default() -> Dfitmg0 {
            Dfitmg0(0)
        }
    }
    impl core::fmt::Debug for Dfitmg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfitmg0")
                .field("dfi_tphy_wrlat", &self.dfi_tphy_wrlat())
                .field("dfi_tphy_wrdata", &self.dfi_tphy_wrdata())
                .field("dfi_wrdata_use_sdr", &self.dfi_wrdata_use_sdr())
                .field("dfi_t_rddata_en", &self.dfi_t_rddata_en())
                .field("dfi_rddata_use_sdr", &self.dfi_rddata_use_sdr())
                .field("dfi_t_ctrl_delay", &self.dfi_t_ctrl_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfitmg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfitmg0 {{ dfi_tphy_wrlat: {=u8:?}, dfi_tphy_wrdata: {=u8:?}, dfi_wrdata_use_sdr: {=bool:?}, dfi_t_rddata_en: {=u8:?}, dfi_rddata_use_sdr: {=bool:?}, dfi_t_ctrl_delay: {=u8:?} }}" , self . dfi_tphy_wrlat () , self . dfi_tphy_wrdata () , self . dfi_wrdata_use_sdr () , self . dfi_t_rddata_en () , self . dfi_rddata_use_sdr () , self . dfi_t_ctrl_delay ())
        }
    }
    #[doc = "Description: DFI Timing Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfitmg1(pub u32);
    impl Dfitmg1 {
        #[doc = "Description: Specifies the number of DFI clock cycles from the de-assertion of the dfi_dram_clk_disable signal on the DFI until the first valid rising edge of the clock to the DRAM memory devices, at the PHY-DRAM boundary. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_dram_clk_enable(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Specifies the number of DFI clock cycles from the de-assertion of the dfi_dram_clk_disable signal on the DFI until the first valid rising edge of the clock to the DRAM memory devices, at the PHY-DRAM boundary. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_dram_clk_enable(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Specifies the number of DFI clock cycles from the assertion of the dfi_dram_clk_disable signal on the DFI until the clock to the DRAM memory devices, at the PHY- DRAM boundary, maintains a low value. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_dram_clk_disable(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Specifies the number of DFI clock cycles from the assertion of the dfi_dram_clk_disable signal on the DFI until the clock to the DRAM memory devices, at the PHY- DRAM boundary, maintains a low value. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_dram_clk_disable(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Specifies the number of DFI clocks between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus. This corresponds to the DFI timing parameter twrdata_delay. Refer to PHY specification for correct value. For DFI 3.0 PHY, set to twrdata_delay, a new timing parameter introduced in DFI 3.0. For DFI 2.1 PHY, set to tphy_wrdata + (delay of DFI write data to the DRAM). Value to be programmed is in terms of DFI clocks, not PHY clocks. In FREQ_RATIO=2, divide PHY's value by 2 and round up to next integer. If using DFITMG0.dfi_wrdata_use_sdr=1, add 1 to the value. Unit: Clocks Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_wrdata_delay(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Specifies the number of DFI clocks between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus. This corresponds to the DFI timing parameter twrdata_delay. Refer to PHY specification for correct value. For DFI 3.0 PHY, set to twrdata_delay, a new timing parameter introduced in DFI 3.0. For DFI 2.1 PHY, set to tphy_wrdata + (delay of DFI write data to the DRAM). Value to be programmed is in terms of DFI clocks, not PHY clocks. In FREQ_RATIO=2, divide PHY's value by 2 and round up to next integer. If using DFITMG0.dfi_wrdata_use_sdr=1, add 1 to the value. Unit: Clocks Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_wrdata_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Dfitmg1 {
        #[inline(always)]
        fn default() -> Dfitmg1 {
            Dfitmg1(0)
        }
    }
    impl core::fmt::Debug for Dfitmg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfitmg1")
                .field("dfi_t_dram_clk_enable", &self.dfi_t_dram_clk_enable())
                .field("dfi_t_dram_clk_disable", &self.dfi_t_dram_clk_disable())
                .field("dfi_t_wrdata_delay", &self.dfi_t_wrdata_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfitmg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfitmg1 {{ dfi_t_dram_clk_enable: {=u8:?}, dfi_t_dram_clk_disable: {=u8:?}, dfi_t_wrdata_delay: {=u8:?} }}" , self . dfi_t_dram_clk_enable () , self . dfi_t_dram_clk_disable () , self . dfi_t_wrdata_delay ())
        }
    }
    #[doc = "Description: DFI Timing Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfitmg2(pub u32);
    impl Dfitmg2 {
        #[doc = "Description: Number of clocks between when a write command is sent on the DFI control interface and when the associated dfi_wrdata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_wrcslat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_tphy_wrcslat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Number of clocks between when a write command is sent on the DFI control interface and when the associated dfi_wrdata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_wrcslat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_tphy_wrcslat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Description: Number of clocks between when a read command is sent on the DFI control interface and when the associated dfi_rddata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_rdcslat. Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_tphy_rdcslat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Number of clocks between when a read command is sent on the DFI control interface and when the associated dfi_rddata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_rdcslat. Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_tphy_rdcslat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for Dfitmg2 {
        #[inline(always)]
        fn default() -> Dfitmg2 {
            Dfitmg2(0)
        }
    }
    impl core::fmt::Debug for Dfitmg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfitmg2")
                .field("dfi_tphy_wrcslat", &self.dfi_tphy_wrcslat())
                .field("dfi_tphy_rdcslat", &self.dfi_tphy_rdcslat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfitmg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfitmg2 {{ dfi_tphy_wrcslat: {=u8:?}, dfi_tphy_rdcslat: {=u8:?} }}",
                self.dfi_tphy_wrcslat(),
                self.dfi_tphy_rdcslat()
            )
        }
    }
    #[doc = "Description: DFI Update Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfiupd0(pub u32);
    impl Dfiupd0 {
        #[doc = "Description: Specifies the minimum number of clock cycles that the dfi_ctrlupd_req signal must be asserted. The uMCTL2 expects the PHY to respond within this time. If the PHY does not respond, the uMCTL2 will de-assert dfi_ctrlupd_req after dfi_t_ctrlup_min + 2 cycles. Lowest value to assign to this variable is 0x3. Unit: Clocks Value After Reset: 0x3 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_ctrlup_min(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Specifies the minimum number of clock cycles that the dfi_ctrlupd_req signal must be asserted. The uMCTL2 expects the PHY to respond within this time. If the PHY does not respond, the uMCTL2 will de-assert dfi_ctrlupd_req after dfi_t_ctrlup_min + 2 cycles. Lowest value to assign to this variable is 0x3. Unit: Clocks Value After Reset: 0x3 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_ctrlup_min(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: Specifies the maximum number of clock cycles that the dfi_ctrlupd_req signal can assert. Lowest value to assign to this variable is 0x40. Unit: Clocks Value After Reset: 0x40 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_ctrlup_max(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Specifies the maximum number of clock cycles that the dfi_ctrlupd_req signal can assert. Lowest value to assign to this variable is 0x40. Unit: Clocks Value After Reset: 0x40 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_ctrlup_max(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Description: When '1', disable the automatic dfi_ctrlupd_req generation by the uMCTL2. The core must issue the dfi_ctrlupd_req signal using register reg_ddrc_ctrlupd. This register field is changeable on the fly. When '0', uMCTL2 issues dfi_ctrlupd_req periodically. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_auto_ctrlupd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When '1', disable the automatic dfi_ctrlupd_req generation by the uMCTL2. The core must issue the dfi_ctrlupd_req signal using register reg_ddrc_ctrlupd. This register field is changeable on the fly. When '0', uMCTL2 issues dfi_ctrlupd_req periodically. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_auto_ctrlupd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dfiupd0 {
        #[inline(always)]
        fn default() -> Dfiupd0 {
            Dfiupd0(0)
        }
    }
    impl core::fmt::Debug for Dfiupd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfiupd0")
                .field("dfi_t_ctrlup_min", &self.dfi_t_ctrlup_min())
                .field("dfi_t_ctrlup_max", &self.dfi_t_ctrlup_max())
                .field("dis_auto_ctrlupd", &self.dis_auto_ctrlupd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfiupd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfiupd0 {{ dfi_t_ctrlup_min: {=u16:?}, dfi_t_ctrlup_max: {=u16:?}, dis_auto_ctrlupd: {=bool:?} }}" , self . dfi_t_ctrlup_min () , self . dfi_t_ctrlup_max () , self . dis_auto_ctrlupd ())
        }
    }
    #[doc = "Description: DFI Update Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfiupd1(pub u32);
    impl Dfiupd1 {
        #[doc = "Description: This is the maximum amount of time between uMCTL2 initiated DFI update requests. This timer resets with each update request; when the timer expires dfi_ctrlupd_req is sent and traffic is blocked until the dfi_ctrlupd_ackx is received. PHY can use this idle time to recalibrate the delay lines to the DLLs. The DFI controller update is also used to reset PHY FIFO pointers in case of data capture errors. Updates are required to maintain calibration over PVT, but frequent updates may impact performance. Note: Value programmed for DFIUPD1.dfi_t_ctrlupd_interval_max_x1024 must be greater than DFIUPD1.dfi_t_ctrlupd_interval_min_x1024. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_ctrlupd_interval_max_x1024(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Description: This is the maximum amount of time between uMCTL2 initiated DFI update requests. This timer resets with each update request; when the timer expires dfi_ctrlupd_req is sent and traffic is blocked until the dfi_ctrlupd_ackx is received. PHY can use this idle time to recalibrate the delay lines to the DLLs. The DFI controller update is also used to reset PHY FIFO pointers in case of data capture errors. Updates are required to maintain calibration over PVT, but frequent updates may impact performance. Note: Value programmed for DFIUPD1.dfi_t_ctrlupd_interval_max_x1024 must be greater than DFIUPD1.dfi_t_ctrlupd_interval_min_x1024. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_ctrlupd_interval_max_x1024(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Description: This is the minimum amount of time between uMCTL2 initiated DFI update requests (which is executed whenever the uMCTL2 is idle). Set this number higher to reduce the frequency of update requests, which can have a small impact on the latency of the first read request when the uMCTL2 is idle. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_t_ctrlupd_interval_min_x1024(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Description: This is the minimum amount of time between uMCTL2 initiated DFI update requests (which is executed whenever the uMCTL2 is idle). Set this number higher to reduce the frequency of update requests, which can have a small impact on the latency of the first read request when the uMCTL2 is idle. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_t_ctrlupd_interval_min_x1024(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dfiupd1 {
        #[inline(always)]
        fn default() -> Dfiupd1 {
            Dfiupd1(0)
        }
    }
    impl core::fmt::Debug for Dfiupd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfiupd1")
                .field(
                    "dfi_t_ctrlupd_interval_max_x1024",
                    &self.dfi_t_ctrlupd_interval_max_x1024(),
                )
                .field(
                    "dfi_t_ctrlupd_interval_min_x1024",
                    &self.dfi_t_ctrlupd_interval_min_x1024(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfiupd1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfiupd1 {{ dfi_t_ctrlupd_interval_max_x1024: {=u8:?}, dfi_t_ctrlupd_interval_min_x1024: {=u8:?} }}" , self . dfi_t_ctrlupd_interval_max_x1024 () , self . dfi_t_ctrlupd_interval_min_x1024 ())
        }
    }
    #[doc = "Description: DFI Update Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfiupd2(pub u32);
    impl Dfiupd2 {
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b00. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_phyupd_type0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b00. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_phyupd_type0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b01. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_phyupd_type1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b01. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_phyupd_type1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Description: Enables the support for acknowledging PHY- initiated updates: 0 - Disabled 1 - Enabled Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_phyupd_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enables the support for acknowledging PHY- initiated updates: 0 - Disabled 1 - Enabled Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_phyupd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dfiupd2 {
        #[inline(always)]
        fn default() -> Dfiupd2 {
            Dfiupd2(0)
        }
    }
    impl core::fmt::Debug for Dfiupd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfiupd2")
                .field("dfi_phyupd_type0", &self.dfi_phyupd_type0())
                .field("dfi_phyupd_type1", &self.dfi_phyupd_type1())
                .field("dfi_phyupd_en", &self.dfi_phyupd_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfiupd2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dfiupd2 {{ dfi_phyupd_type0: {=u16:?}, dfi_phyupd_type1: {=u16:?}, dfi_phyupd_en: {=bool:?} }}" , self . dfi_phyupd_type0 () , self . dfi_phyupd_type1 () , self . dfi_phyupd_en ())
        }
    }
    #[doc = "Description: DFI Update Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dfiupd3(pub u32);
    impl Dfiupd3 {
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b10. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_phyupd_type2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b10. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_phyupd_type2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b11. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dfi_phyupd_type3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b11. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_dfi_phyupd_type3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Dfiupd3 {
        #[inline(always)]
        fn default() -> Dfiupd3 {
            Dfiupd3(0)
        }
    }
    impl core::fmt::Debug for Dfiupd3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dfiupd3")
                .field("dfi_phyupd_type2", &self.dfi_phyupd_type2())
                .field("dfi_phyupd_type3", &self.dfi_phyupd_type3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dfiupd3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dfiupd3 {{ dfi_phyupd_type2: {=u16:?}, dfi_phyupd_type3: {=u16:?} }}",
                self.dfi_phyupd_type2(),
                self.dfi_phyupd_type3()
            )
        }
    }
    #[doc = "Description: DIMM Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dimmctl(pub u32);
    impl Dimmctl {
        #[doc = "Description: Staggering enable for multi-rank accesses (for multi-rank UDIMM and RDIMM implementations only). This is not supported for DDR4, mDDR, LPDDR2 or LPDDR3 SDRAMs. 1 - Stagger accesses to even and odd ranks 0 - Do not stagger accesses Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dimm_stagger_cs_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Staggering enable for multi-rank accesses (for multi-rank UDIMM and RDIMM implementations only). This is not supported for DDR4, mDDR, LPDDR2 or LPDDR3 SDRAMs. 1 - Stagger accesses to even and odd ranks 0 - Do not stagger accesses Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dimm_stagger_cs_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Address Mirroring Enable (for multi-rank UDIMM implementations and multi-rank DDR4 RDIMM implementations). Some UDIMMs and DDR4 RDIMMs implement address mirroring for odd ranks, which means that the following address, bank address and bank group bits are swapped: (A3, A4), (A5, A6), (A7, A8), (BA0, BA1) and also (A11, A13), (BG0, BG1) for the DDR4. Setting this bit ensures that, for mode register accesses during the automatic initialization routine, these bits are swapped within the uMCTL2 to compensate for this UDIMM/RDIMM swapping. In addition to the automatic initialization routine, in case of DDR4 UDIMM/RDIMM, they are swapped during the automatic MRS access to enable/disable of a particular DDR4 feature. Note: This has no effect on the address of any other memory accesses, or of software-driven mode register accesses. This is not supported for mDDR, LPDDR2 or LPDDR3 SDRAMs. Note: In case of x16 DDR4 DIMMs, BG1 output of MRS for the odd ranks is same as BG0 because BG1 is invalid, hence dimm_dis_bg_mirroring register must be set to 1. 1 - For odd ranks, implement address mirroring for MRS commands to during initialization and for any automatic DDR4 MRS commands (to be used if UDIMM/RDIMM implements address mirroring) 0 - Do not implement address mirroring Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dimm_addr_mirr_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Address Mirroring Enable (for multi-rank UDIMM implementations and multi-rank DDR4 RDIMM implementations). Some UDIMMs and DDR4 RDIMMs implement address mirroring for odd ranks, which means that the following address, bank address and bank group bits are swapped: (A3, A4), (A5, A6), (A7, A8), (BA0, BA1) and also (A11, A13), (BG0, BG1) for the DDR4. Setting this bit ensures that, for mode register accesses during the automatic initialization routine, these bits are swapped within the uMCTL2 to compensate for this UDIMM/RDIMM swapping. In addition to the automatic initialization routine, in case of DDR4 UDIMM/RDIMM, they are swapped during the automatic MRS access to enable/disable of a particular DDR4 feature. Note: This has no effect on the address of any other memory accesses, or of software-driven mode register accesses. This is not supported for mDDR, LPDDR2 or LPDDR3 SDRAMs. Note: In case of x16 DDR4 DIMMs, BG1 output of MRS for the odd ranks is same as BG0 because BG1 is invalid, hence dimm_dis_bg_mirroring register must be set to 1. 1 - For odd ranks, implement address mirroring for MRS commands to during initialization and for any automatic DDR4 MRS commands (to be used if UDIMM/RDIMM implements address mirroring) 0 - Do not implement address mirroring Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dimm_addr_mirr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dimmctl {
        #[inline(always)]
        fn default() -> Dimmctl {
            Dimmctl(0)
        }
    }
    impl core::fmt::Debug for Dimmctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dimmctl")
                .field("dimm_stagger_cs_en", &self.dimm_stagger_cs_en())
                .field("dimm_addr_mirr_en", &self.dimm_addr_mirr_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dimmctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dimmctl {{ dimm_stagger_cs_en: {=bool:?}, dimm_addr_mirr_en: {=bool:?} }}",
                self.dimm_stagger_cs_en(),
                self.dimm_addr_mirr_en()
            )
        }
    }
    #[doc = "Description: SDRAM Timing Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg0(pub u32);
    impl Dramtmg0 {
        #[doc = "Description: tRAS(min): Minimum time between activate and precharge to the same bank. For configurations with MEMC_FREQ_RATIO=2, 1T mode, program this to tRAS(min)/2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, program this to (tRAS(min)/2 + 1). No rounding up of the division operation. Unit: Clocks Value After Reset: 0xf Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_ras_min(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: tRAS(min): Minimum time between activate and precharge to the same bank. For configurations with MEMC_FREQ_RATIO=2, 1T mode, program this to tRAS(min)/2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, program this to (tRAS(min)/2 + 1). No rounding up of the division operation. Unit: Clocks Value After Reset: 0xf Exists: Always."]
        #[inline(always)]
        pub const fn set_t_ras_min(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Description: tRAS(max): Maximum time between activate and precharge to same bank. This is the maximum time that a page can be kept open Minimum value of this register is 1. Zero is invalid. For configurations with MEMC_FREQ_RATIO=2, program this to (tRAS(max)-1)/2. No rounding up. Unit: Multiples of 1024 clocks. Value After Reset: 0x1b Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_ras_max(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: tRAS(max): Maximum time between activate and precharge to same bank. This is the maximum time that a page can be kept open Minimum value of this register is 1. Zero is invalid. For configurations with MEMC_FREQ_RATIO=2, program this to (tRAS(max)-1)/2. No rounding up. Unit: Multiples of 1024 clocks. Value After Reset: 0x1b Exists: Always."]
        #[inline(always)]
        pub const fn set_t_ras_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Description: tFAW Valid only when 8 or more banks(or banks x bank groups) are present. In 8-bank design, at most 4 banks must be activated in a rolling window of tFAW cycles. For configurations with MEMC_FREQ_RATIO=2, program this to (tFAW/2) and round up to next integer value. In a 4-bank design, set this register to 0x1 independent of the MEMC_FREQ_RATIO configuration. Unit: Clocks Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_faw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: tFAW Valid only when 8 or more banks(or banks x bank groups) are present. In 8-bank design, at most 4 banks must be activated in a rolling window of tFAW cycles. For configurations with MEMC_FREQ_RATIO=2, program this to (tFAW/2) and round up to next integer value. In a 4-bank design, set this register to 0x1 independent of the MEMC_FREQ_RATIO configuration. Unit: Clocks Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_faw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Description: Minimum time between write and precharge to same bank. Unit: Clocks Specifications: WL + BL/2 + tWR = approximately 8 cycles + 15 ns = 14 clocks @400MHz and less for lower frequencies where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM. BST (burst terminate) is not supported at present. tWR = Write recovery time. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 for this parameter. For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up. Value After Reset: 0xf Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr2pre(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: Minimum time between write and precharge to same bank. Unit: Clocks Specifications: WL + BL/2 + tWR = approximately 8 cycles + 15 ns = 14 clocks @400MHz and less for lower frequencies where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM. BST (burst terminate) is not supported at present. tWR = Write recovery time. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 for this parameter. For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up. Value After Reset: 0xf Exists: Always."]
        #[inline(always)]
        pub const fn set_wr2pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Dramtmg0 {
        #[inline(always)]
        fn default() -> Dramtmg0 {
            Dramtmg0(0)
        }
    }
    impl core::fmt::Debug for Dramtmg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg0")
                .field("t_ras_min", &self.t_ras_min())
                .field("t_ras_max", &self.t_ras_max())
                .field("t_faw", &self.t_faw())
                .field("wr2pre", &self.wr2pre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dramtmg0 {{ t_ras_min: {=u8:?}, t_ras_max: {=u8:?}, t_faw: {=u8:?}, wr2pre: {=u8:?} }}" , self . t_ras_min () , self . t_ras_max () , self . t_faw () , self . wr2pre ())
        }
    }
    #[doc = "Description: SDRAM Timing Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg1(pub u32);
    impl Dramtmg1 {
        #[doc = "Description: tRC: Minimum time between activates to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRC/2) and round up to next integer value. Unit: Clocks. Value After Reset: 0x14 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: tRC: Minimum time between activates to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRC/2) and round up to next integer value. Unit: Clocks. Value After Reset: 0x14 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Description: tRTP: Minimum time from read to precharge of same bank. DDR2: tAL + BL/2 + max(tRTP, 2) - 2 DDR3: tAL + max (tRTP, 4) DDR4: Max of following two equations: tAL + max (tRTP, 4) or, RL + BL/2 - tRP. mDDR: BL/2 LPDDR2: Depends on if it's LPDDR2-S2 or LPDDR2-S4: LPDDR2-S2: BL/2 + tRTP - 1. LPDDR2-S4: BL/2 + max(tRTP,2) - 2. LPDDR3: BL/2 + max(tRTP,4) - 4 For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up of division operation. Unit: Clocks. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd2pre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: tRTP: Minimum time from read to precharge of same bank. DDR2: tAL + BL/2 + max(tRTP, 2) - 2 DDR3: tAL + max (tRTP, 4) DDR4: Max of following two equations: tAL + max (tRTP, 4) or, RL + BL/2 - tRP. mDDR: BL/2 LPDDR2: Depends on if it's LPDDR2-S2 or LPDDR2-S4: LPDDR2-S2: BL/2 + tRTP - 1. LPDDR2-S4: BL/2 + max(tRTP,2) - 2. LPDDR3: BL/2 + max(tRTP,4) - 4 For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up of division operation. Unit: Clocks. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd2pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Description: tXP: Minimum time after power-down exit to any operation. For DDR3, this should be programmed to tXPDLL if slow powerdown exit is selected in MR0\\[12\\]. If C/A parity for DDR4 is used, set to (tXP+PL) instead. For configurations with MEMC_FREQ_RATIO=2, program this to (tXP/2) and round it up to the next integer value. Units: Clocks Value After Reset: 0x8 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_xp(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: tXP: Minimum time after power-down exit to any operation. For DDR3, this should be programmed to tXPDLL if slow powerdown exit is selected in MR0\\[12\\]. If C/A parity for DDR4 is used, set to (tXP+PL) instead. For configurations with MEMC_FREQ_RATIO=2, program this to (tXP/2) and round it up to the next integer value. Units: Clocks Value After Reset: 0x8 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_xp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Dramtmg1 {
        #[inline(always)]
        fn default() -> Dramtmg1 {
            Dramtmg1(0)
        }
    }
    impl core::fmt::Debug for Dramtmg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg1")
                .field("t_rc", &self.t_rc())
                .field("rd2pre", &self.rd2pre())
                .field("t_xp", &self.t_xp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dramtmg1 {{ t_rc: {=u8:?}, rd2pre: {=u8:?}, t_xp: {=u8:?} }}",
                self.t_rc(),
                self.rd2pre(),
                self.t_xp()
            )
        }
    }
    #[doc = "Description: SDRAM Timing Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg2(pub u32);
    impl Dramtmg2 {
        #[doc = "Description: DDR4: WL + BL/2 + tWTR_L Others: WL + BL/2 + tWTR In DDR4, minimum time from write command to read command for same bank group. In others, minimum time from write command to read command. Includes time for bus turnaround, recovery times, and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM tWTR_L = internal write to read command delay for same bank group. This comes directly from the SDRAM specification. tWTR = internal write to read command delay. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 operation. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0xd Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr2rd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: DDR4: WL + BL/2 + tWTR_L Others: WL + BL/2 + tWTR In DDR4, minimum time from write command to read command for same bank group. In others, minimum time from write command to read command. Includes time for bus turnaround, recovery times, and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM tWTR_L = internal write to read command delay for same bank group. This comes directly from the SDRAM specification. tWTR = internal write to read command delay. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 operation. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0xd Exists: Always."]
        #[inline(always)]
        pub const fn set_wr2rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Description: DDR2/3/mDDR: RL + BL/2 + 2 - WL DDR4: RL + BL/2 + 1 + WR_PREAMBLE - WL LPDDR2/LPDDR3: RL + BL/2 + RU(tDQSCKmax/tCK) + 1 - WL. Minimum time from read command to write command. Include time for bus turnaround and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM RL = read latency = CAS latency WR_PREAMBLE = write preamble. This is unique to DDR4. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0x6 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd2wr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: DDR2/3/mDDR: RL + BL/2 + 2 - WL DDR4: RL + BL/2 + 1 + WR_PREAMBLE - WL LPDDR2/LPDDR3: RL + BL/2 + RU(tDQSCKmax/tCK) + 1 - WL. Minimum time from read command to write command. Include time for bus turnaround and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM RL = read latency = CAS latency WR_PREAMBLE = write preamble. This is unique to DDR4. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0x6 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd2wr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Dramtmg2 {
        #[inline(always)]
        fn default() -> Dramtmg2 {
            Dramtmg2(0)
        }
    }
    impl core::fmt::Debug for Dramtmg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg2")
                .field("wr2rd", &self.wr2rd())
                .field("rd2wr", &self.rd2wr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dramtmg2 {{ wr2rd: {=u8:?}, rd2wr: {=u8:?} }}",
                self.wr2rd(),
                self.rd2wr()
            )
        }
    }
    #[doc = "Description: SDRAM Timing Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg3(pub u32);
    impl Dramtmg3 {
        #[doc = "Description: tMOD: Present if MEMC_DDR3_OR_4 = 1. Cycles between load mode command and following non-load mode command. This is required to be programmed even when a design that supports DDR3/4 is running in DDR2 mode. If C/A parity for DDR4 is used, set to tMOD_PAR(tMOD+PL) instead Set to tMOD if MEMC_FREQ_RATIO=1, or tMOD/2 (rounded up to next integer) if MEMC_FREQ_RATIO=2. Note that if using RDIMM, depending on the PHY, it may be necessary to use a value of tMOD + 1 or (tMOD + 1)/2 to compensate for the extra cycle of latency applied to mode register writes by the RDIMM chip Value After Reset: \"(MEMC_DDR3_EN==1 || MEMC_DDR4_EN==1 ) ? 0xc : 0x0\" Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_mod(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: tMOD: Present if MEMC_DDR3_OR_4 = 1. Cycles between load mode command and following non-load mode command. This is required to be programmed even when a design that supports DDR3/4 is running in DDR2 mode. If C/A parity for DDR4 is used, set to tMOD_PAR(tMOD+PL) instead Set to tMOD if MEMC_FREQ_RATIO=1, or tMOD/2 (rounded up to next integer) if MEMC_FREQ_RATIO=2. Note that if using RDIMM, depending on the PHY, it may be necessary to use a value of tMOD + 1 or (tMOD + 1)/2 to compensate for the extra cycle of latency applied to mode register writes by the RDIMM chip Value After Reset: \"(MEMC_DDR3_EN==1 || MEMC_DDR4_EN==1 ) ? 0xc : 0x0\" Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[inline(always)]
        pub const fn set_t_mod(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: tMRD: Cycles between load mode commands. If MEMC_DDR3_OR_4 = 0, this parameter is also used to define the cycles between load mode command and following non-load mode command. For configurations with MEMC_FREQ_RATIO=2, program this to (tMRD/2) and round it up to the next integer value. If C/A parity for DDR4 is used, set to tMRD_PAR(tMOD+PL) instead Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_mrd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: tMRD: Cycles between load mode commands. If MEMC_DDR3_OR_4 = 0, this parameter is also used to define the cycles between load mode command and following non-load mode command. For configurations with MEMC_FREQ_RATIO=2, program this to (tMRD/2) and round it up to the next integer value. If C/A parity for DDR4 is used, set to tMRD_PAR(tMOD+PL) instead Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_mrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
    }
    impl Default for Dramtmg3 {
        #[inline(always)]
        fn default() -> Dramtmg3 {
            Dramtmg3(0)
        }
    }
    impl core::fmt::Debug for Dramtmg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg3")
                .field("t_mod", &self.t_mod())
                .field("t_mrd", &self.t_mrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dramtmg3 {{ t_mod: {=u16:?}, t_mrd: {=u8:?} }}",
                self.t_mod(),
                self.t_mrd()
            )
        }
    }
    #[doc = "Description: SDRAM Timing Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg4(pub u32);
    impl Dramtmg4 {
        #[doc = "Description: tRP: Minimum time from precharge to activate of same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRP/2 + 1). No round up of the fraction. Unit: Clocks. Value After Reset: 0x5 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: tRP: Minimum time from precharge to activate of same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRP/2 + 1). No round up of the fraction. Unit: Clocks. Value After Reset: 0x5 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Description: DDR4: tRRD_L: Minimum time between activates from bank \"a\" to bank \"b\" for same bank group. Others: tRRD: Minimum time between activates from bank \"a\" to bank \"b\" For configurations with MEMC_FREQ_RATIO=2, program this to (tRRD_L/2 or tRRD/2) and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rrd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: DDR4: tRRD_L: Minimum time between activates from bank \"a\" to bank \"b\" for same bank group. Others: tRRD: Minimum time between activates from bank \"a\" to bank \"b\" For configurations with MEMC_FREQ_RATIO=2, program this to (tRRD_L/2 or tRRD/2) and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: DDR4: tCCD_L: This is the minimum time between two reads or two writes for same bank group. Others: tCCD: This is the minimum time between two reads or two writes. For configurations with MEMC_FREQ_RATIO=2, program this to (tCCD_L/2 or tCCD/2) and round it up to the next integer value. Unit: clocks. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_ccd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Description: DDR4: tCCD_L: This is the minimum time between two reads or two writes for same bank group. Others: tCCD: This is the minimum time between two reads or two writes. For configurations with MEMC_FREQ_RATIO=2, program this to (tCCD_L/2 or tCCD/2) and round it up to the next integer value. Unit: clocks. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_ccd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Description: tRCD - tAL: Minimum time from activate to read or write command to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to ((tRCD - tAL)/2) and round it up to the next integer value. Minimum value allowed for this register is 1, which implies minimum (tRCD - tAL) value to be 2 in configurations with MEMC_FREQ_RATIO=2. Unit: Clocks. Value After Reset: 0x5 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rcd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: tRCD - tAL: Minimum time from activate to read or write command to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to ((tRCD - tAL)/2) and round it up to the next integer value. Minimum value allowed for this register is 1, which implies minimum (tRCD - tAL) value to be 2 in configurations with MEMC_FREQ_RATIO=2. Unit: Clocks. Value After Reset: 0x5 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rcd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Dramtmg4 {
        #[inline(always)]
        fn default() -> Dramtmg4 {
            Dramtmg4(0)
        }
    }
    impl core::fmt::Debug for Dramtmg4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg4")
                .field("t_rp", &self.t_rp())
                .field("t_rrd", &self.t_rrd())
                .field("t_ccd", &self.t_ccd())
                .field("t_rcd", &self.t_rcd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dramtmg4 {{ t_rp: {=u8:?}, t_rrd: {=u8:?}, t_ccd: {=u8:?}, t_rcd: {=u8:?} }}",
                self.t_rp(),
                self.t_rrd(),
                self.t_ccd(),
                self.t_rcd()
            )
        }
    }
    #[doc = "Description: SDRAM Timing Register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg5(pub u32);
    impl Dramtmg5 {
        #[doc = "Description: Minimum number of cycles of CKE HIGH/LOW during power-down and self refresh. LPDDR2/LPDDR3 mode: Set this to the larger of tCKE or tCKESR Non-LPDDR2/non-LPDDR3 designs: Set this to tCKE value. For configurations with MEMC_FREQ_RATIO=2, program this to (value described above)/2 and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x3 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_cke(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: Minimum number of cycles of CKE HIGH/LOW during power-down and self refresh. LPDDR2/LPDDR3 mode: Set this to the larger of tCKE or tCKESR Non-LPDDR2/non-LPDDR3 designs: Set this to tCKE value. For configurations with MEMC_FREQ_RATIO=2, program this to (value described above)/2 and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x3 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_cke(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Description: Minimum CKE low width for Self refresh entry to exit timing im memory clock cycles. Recommended settings: mDDR: tRFC LPDDR2: tCKESR LPDDR3: tCKESR DDR2: tCKE DDR3: tCKE + 1 DDR4: tCKE + 1 For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_ckesr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Minimum CKE low width for Self refresh entry to exit timing im memory clock cycles. Recommended settings: mDDR: tRFC LPDDR2: tCKESR LPDDR3: tCKESR DDR2: tCKE DDR3: tCKE + 1 DDR4: tCKE + 1 For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_ckesr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Description: This is the time after Self Refresh Down Entry that CK is maintained as a valid clock. Specifies the clock disable delay after SRE. Recommended settings: mDDR: 0 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: max (10 ns, 5 tCK) DDR4: max (10 ns, 5 tCK) For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_cksre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: This is the time after Self Refresh Down Entry that CK is maintained as a valid clock. Specifies the clock disable delay after SRE. Recommended settings: mDDR: 0 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: max (10 ns, 5 tCK) DDR4: max (10 ns, 5 tCK) For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_cksre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: This is the time before Self Refresh Exit that CK is maintained as a valid clock before issuing SRX. Specifies the clock stable time before SRX. Recommended settings: mDDR: 1 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: tCKSRX DDR4: tCKSRX For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_cksrx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: This is the time before Self Refresh Exit that CK is maintained as a valid clock before issuing SRX. Specifies the clock stable time before SRX. Recommended settings: mDDR: 1 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: tCKSRX DDR4: tCKSRX For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_cksrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Dramtmg5 {
        #[inline(always)]
        fn default() -> Dramtmg5 {
            Dramtmg5(0)
        }
    }
    impl core::fmt::Debug for Dramtmg5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg5")
                .field("t_cke", &self.t_cke())
                .field("t_ckesr", &self.t_ckesr())
                .field("t_cksre", &self.t_cksre())
                .field("t_cksrx", &self.t_cksrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dramtmg5 {{ t_cke: {=u8:?}, t_ckesr: {=u8:?}, t_cksre: {=u8:?}, t_cksrx: {=u8:?} }}" , self . t_cke () , self . t_ckesr () , self . t_cksre () , self . t_cksrx ())
        }
    }
    #[doc = "Description: SDRAM Timing Register 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dramtmg8(pub u32);
    impl Dramtmg8 {
        #[doc = "Description: tXS: Exit Self Refresh to commands not requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x5 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_xs_x32(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: tXS: Exit Self Refresh to commands not requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x5 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_xs_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Description: tXSDLL: Exit Self Refresh to commands requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x44 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_xs_dll_x32(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: tXSDLL: Exit Self Refresh to commands requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x44 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_xs_dll_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for Dramtmg8 {
        #[inline(always)]
        fn default() -> Dramtmg8 {
            Dramtmg8(0)
        }
    }
    impl core::fmt::Debug for Dramtmg8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dramtmg8")
                .field("t_xs_x32", &self.t_xs_x32())
                .field("t_xs_dll_x32", &self.t_xs_dll_x32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dramtmg8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dramtmg8 {{ t_xs_x32: {=u8:?}, t_xs_dll_x32: {=u8:?} }}",
                self.t_xs_x32(),
                self.t_xs_dll_x32()
            )
        }
    }
    #[doc = "Description: ECC Uncorrected Error Address Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccuaddr0(pub u32);
    impl Eccuaddr0 {
        #[doc = "Description: Page/row number of a read resulting in an uncorrected ECC error. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn ecc_uncorr_row(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Description: Page/row number of a read resulting in an uncorrected ECC error. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_ecc_uncorr_row(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "Description: Rank number of a read resulting in an uncorrected ECC error Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn ecc_uncorr_rank(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Description: Rank number of a read resulting in an uncorrected ECC error Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_ecc_uncorr_rank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for Eccuaddr0 {
        #[inline(always)]
        fn default() -> Eccuaddr0 {
            Eccuaddr0(0)
        }
    }
    impl core::fmt::Debug for Eccuaddr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccuaddr0")
                .field("ecc_uncorr_row", &self.ecc_uncorr_row())
                .field("ecc_uncorr_rank", &self.ecc_uncorr_rank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccuaddr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eccuaddr0 {{ ecc_uncorr_row: {=u32:?}, ecc_uncorr_rank: {=u8:?} }}",
                self.ecc_uncorr_row(),
                self.ecc_uncorr_rank()
            )
        }
    }
    #[doc = "Description: Hardware Low Power Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwlpctl(pub u32);
    impl Hwlpctl {
        #[doc = "Description: Enable for Hardware Low Power Interface. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn hw_lp_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enable for Hardware Low Power Interface. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_hw_lp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: When this bit is programmed to 1 the cactive_in_ddrc pin of the DDRC can be used to exit from the automatic clock stop, automatic power down or automatic self-refresh modes. Note, it will not cause exit of Self-Refresh that was caused by Hardware Low Power Interface and/or Software (PWRCTL.selfref_sw). Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn hw_lp_exit_idle_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When this bit is programmed to 1 the cactive_in_ddrc pin of the DDRC can be used to exit from the automatic clock stop, automatic power down or automatic self-refresh modes. Note, it will not cause exit of Self-Refresh that was caused by Hardware Low Power Interface and/or Software (PWRCTL.selfref_sw). Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_hw_lp_exit_idle_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: Hardware idle period. The cactive_ddrc output is driven low if the system is idle for hw_lp_idle * 32 cycles if not in INIT or DPD/MPSM operating_mode. The hardware idle function is disabled when hw_lp_idle_x32=0. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn hw_lp_idle_x32(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Hardware idle period. The cactive_ddrc output is driven low if the system is idle for hw_lp_idle * 32 cycles if not in INIT or DPD/MPSM operating_mode. The hardware idle function is disabled when hw_lp_idle_x32=0. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_hw_lp_idle_x32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Hwlpctl {
        #[inline(always)]
        fn default() -> Hwlpctl {
            Hwlpctl(0)
        }
    }
    impl core::fmt::Debug for Hwlpctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwlpctl")
                .field("hw_lp_en", &self.hw_lp_en())
                .field("hw_lp_exit_idle_en", &self.hw_lp_exit_idle_en())
                .field("hw_lp_idle_x32", &self.hw_lp_idle_x32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwlpctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hwlpctl {{ hw_lp_en: {=bool:?}, hw_lp_exit_idle_en: {=bool:?}, hw_lp_idle_x32: {=u16:?} }}" , self . hw_lp_en () , self . hw_lp_exit_idle_en () , self . hw_lp_idle_x32 ())
        }
    }
    #[doc = "Description: SDRAM Initialization Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init0(pub u32);
    impl Init0 {
        #[doc = "Description: Cycles to wait after reset before driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clock cycles. DDR2 specifications typically require this to be programmed for a delay of >= 200 us. LPDDR2/LPDDR3: tINIT1 of 100 ns (min) For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x4e Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn pre_cke_x1024(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Cycles to wait after reset before driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clock cycles. DDR2 specifications typically require this to be programmed for a delay of >= 200 us. LPDDR2/LPDDR3: tINIT1 of 100 ns (min) For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x4e Exists: Always."]
        #[inline(always)]
        pub const fn set_pre_cke_x1024(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: Cycles to wait after driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clocks. DDR2 typically requires a 400 ns delay, requiring this value to be programmed to 2 at all clock speeds. LPDDR2/LPDDR3 typically requires this to be programmed for a delay of 200 us. For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn post_cke_x1024(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Cycles to wait after driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clocks. DDR2 typically requires a 400 ns delay, requiring this value to be programmed to 2 at all clock speeds. LPDDR2/LPDDR3 typically requires this to be programmed for a delay of 200 us. For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_post_cke_x1024(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Description: If lower bit is enabled the SDRAM initialization routine is skipped. The upper bit decides what state the controller starts up in when reset is removed 00 - SDRAM Initialization routine is run after power-up 01 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Normal Mode 11 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Self-refresh Mode 10 - SDRAM Initialization routine is run after power-up. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn skip_dram_init(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Description: If lower bit is enabled the SDRAM initialization routine is skipped. The upper bit decides what state the controller starts up in when reset is removed 00 - SDRAM Initialization routine is run after power-up 01 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Normal Mode 11 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Self-refresh Mode 10 - SDRAM Initialization routine is run after power-up. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_skip_dram_init(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Init0 {
        #[inline(always)]
        fn default() -> Init0 {
            Init0(0)
        }
    }
    impl core::fmt::Debug for Init0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init0")
                .field("pre_cke_x1024", &self.pre_cke_x1024())
                .field("post_cke_x1024", &self.post_cke_x1024())
                .field("skip_dram_init", &self.skip_dram_init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Init0 {{ pre_cke_x1024: {=u16:?}, post_cke_x1024: {=u16:?}, skip_dram_init: {=u8:?} }}" , self . pre_cke_x1024 () , self . post_cke_x1024 () , self . skip_dram_init ())
        }
    }
    #[doc = "Description: SDRAM Initialization Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init1(pub u32);
    impl Init1 {
        #[doc = "Description: Wait period before driving the OCD complete command to SDRAM. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn pre_ocd_x32(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Wait period before driving the OCD complete command to SDRAM. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_pre_ocd_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Cycles to wait after completing the SDRAM initialization sequence before starting the dynamic scheduler. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn final_wait_x32(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: Cycles to wait after completing the SDRAM initialization sequence before starting the dynamic scheduler. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_final_wait_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Description: Number of cycles to assert SDRAM reset signal during init sequence. This is only present for designs supporting DDR3/DDR4 devices. For use with a Synopsys DDR PHY, this should be set to a minimum of 1 Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dram_rstn_x1024(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Description: Number of cycles to assert SDRAM reset signal during init sequence. This is only present for designs supporting DDR3/DDR4 devices. For use with a Synopsys DDR PHY, this should be set to a minimum of 1 Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[inline(always)]
        pub const fn set_dram_rstn_x1024(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Init1 {
        #[inline(always)]
        fn default() -> Init1 {
            Init1(0)
        }
    }
    impl core::fmt::Debug for Init1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init1")
                .field("pre_ocd_x32", &self.pre_ocd_x32())
                .field("final_wait_x32", &self.final_wait_x32())
                .field("dram_rstn_x1024", &self.dram_rstn_x1024())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Init1 {{ pre_ocd_x32: {=u8:?}, final_wait_x32: {=u8:?}, dram_rstn_x1024: {=u8:?} }}" , self . pre_ocd_x32 () , self . final_wait_x32 () , self . dram_rstn_x1024 ())
        }
    }
    #[doc = "Description: SDRAM Initialization Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init3(pub u32);
    impl Init3 {
        #[doc = "Description: DDR2: Value to write to EMR register. Bits 9:7 are for OCD and the setting in this register is ignored. The uMCTL2 sets those bits appropriately. DDR3/DDR4: Value to write to MR1 register Set bit 7 to 0. If PHY-evaluation mode training is enabled, this bit is set appropriately by the uMCTL2 during write leveling. mDDR: Value to write to EMR register. LPDDR2/LPDDR3 - Value to write to MR2 register Value After Reset: 0x510 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn emr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: DDR2: Value to write to EMR register. Bits 9:7 are for OCD and the setting in this register is ignored. The uMCTL2 sets those bits appropriately. DDR3/DDR4: Value to write to MR1 register Set bit 7 to 0. If PHY-evaluation mode training is enabled, this bit is set appropriately by the uMCTL2 during write leveling. mDDR: Value to write to EMR register. LPDDR2/LPDDR3 - Value to write to MR2 register Value After Reset: 0x510 Exists: Always."]
        #[inline(always)]
        pub const fn set_emr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: DDR2: Value to write to MR register. Bit 8 is for DLL and the setting here is ignored. The uMCTL2 sets this bit appropriately. DDR3/DDR4: Value loaded into MR0 register. mDDR: Value to write to MR register. LPDDR2/LPDDR3 - Value to write to MR1 register Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: DDR2: Value to write to MR register. Bit 8 is for DLL and the setting here is ignored. The uMCTL2 sets this bit appropriately. DDR3/DDR4: Value loaded into MR0 register. mDDR: Value to write to MR register. LPDDR2/LPDDR3 - Value to write to MR1 register Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_mr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Init3 {
        #[inline(always)]
        fn default() -> Init3 {
            Init3(0)
        }
    }
    impl core::fmt::Debug for Init3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init3")
                .field("emr", &self.emr())
                .field("mr", &self.mr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Init3 {{ emr: {=u16:?}, mr: {=u16:?} }}",
                self.emr(),
                self.mr()
            )
        }
    }
    #[doc = "Description: SDRAM Initialization Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init4(pub u32);
    impl Init4 {
        #[doc = "Description: DDR2: Value to write to EMR3 register. DDR3/DDR4: Value to write to MR3 register mDDR/LPDDR2/LPDDR3: Unused Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn emr3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: DDR2: Value to write to EMR3 register. DDR3/DDR4: Value to write to MR3 register mDDR/LPDDR2/LPDDR3: Unused Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_emr3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: DDR2: Value to write to EMR2 register. DDR3/DDR4: Value to write to MR2 register LPDDR2/LPDDR3: Value to write to MR3 register mDDR: Unused Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn emr2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: DDR2: Value to write to EMR2 register. DDR3/DDR4: Value to write to MR2 register LPDDR2/LPDDR3: Value to write to MR3 register mDDR: Unused Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_emr2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Init4 {
        #[inline(always)]
        fn default() -> Init4 {
            Init4(0)
        }
    }
    impl core::fmt::Debug for Init4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init4")
                .field("emr3", &self.emr3())
                .field("emr2", &self.emr2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Init4 {{ emr3: {=u16:?}, emr2: {=u16:?} }}",
                self.emr3(),
                self.emr2()
            )
        }
    }
    #[doc = "Description: SDRAM Initialization Register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init5(pub u32);
    impl Init5 {
        #[doc = "Description: ZQ initial calibration, tZQINIT. Present only in designs configured to support DDR3 or DDR4 or LPDDR2/LPDDR3. Unit: 32 clock cycles. DDR3 typically requires 512 clocks. DDR4 requires 1024 clocks. LPDDR2/LPDDR3 requires 1 us. Value After Reset: 0x10 Exists: MEMC_DDR3==1 || MEMC_DDR4 == 1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_zqinit_x32(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Description: ZQ initial calibration, tZQINIT. Present only in designs configured to support DDR3 or DDR4 or LPDDR2/LPDDR3. Unit: 32 clock cycles. DDR3 typically requires 512 clocks. DDR4 requires 1024 clocks. LPDDR2/LPDDR3 requires 1 us. Value After Reset: 0x10 Exists: MEMC_DDR3==1 || MEMC_DDR4 == 1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_dev_zqinit_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Init5 {
        #[inline(always)]
        fn default() -> Init5 {
            Init5(0)
        }
    }
    impl core::fmt::Debug for Init5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init5")
                .field("dev_zqinit_x32", &self.dev_zqinit_x32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Init5 {{ dev_zqinit_x32: {=u8:?} }}",
                self.dev_zqinit_x32()
            )
        }
    }
    #[doc = "Description: Port n Channel m Configuration ID Mask Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskch(pub u32);
    impl Maskch {
        #[doc = "Description: Determines the mask used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn id_mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Description: Determines the mask used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_id_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maskch {
        #[inline(always)]
        fn default() -> Maskch {
            Maskch(0)
        }
    }
    impl core::fmt::Debug for Maskch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maskch")
                .field("id_mask", &self.id_mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maskch {{ id_mask: {=u32:?} }}", self.id_mask())
        }
    }
    #[doc = "Description: Mode Register Read/Write Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrctrl0(pub u32);
    impl Mrctrl0 {
        #[doc = "Description: Controls which rank is accessed by MRCTRL0.mr_wr. Normally, it is desired to access all ranks, so all bits should be set to 1. However, for multi-rank UDIMMs/RDIMMs which implement address mirroring, it may be necessary to access ranks individually. Examples (assume uMCTL2 is configured for 4 ranks): 0x1 - select rank 0 only 0x2 - select rank 1 only 0x5 - select ranks 0 and 2 0xA - select ranks 1 and 3 0xF - select ranks 0, 1, 2 and 3 Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_rank(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Controls which rank is accessed by MRCTRL0.mr_wr. Normally, it is desired to access all ranks, so all bits should be set to 1. However, for multi-rank UDIMMs/RDIMMs which implement address mirroring, it may be necessary to access ranks individually. Examples (assume uMCTL2 is configured for 4 ranks): 0x1 - select rank 0 only 0x2 - select rank 1 only 0x5 - select ranks 0 and 2 0xA - select ranks 1 and 3 0xF - select ranks 0, 1, 2 and 3 Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: Always."]
        #[inline(always)]
        pub const fn set_mr_rank(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Description: Address of the mode register that is to be written to. 0000 - MR0 0001 - MR1 0010 - MR2 0011 - MR3 0100 - MR4 0101 - MR5 0110 - MR6 0111 - MR7 Don't Care for LPDDR2/LPDDR3 (see MRCTRL1.mr_data for mode register addressing in LPDDR2/LPDDR3) This signal is also used for writing to control words of RDIMMs. In that case, it corresponds to the bank address bits sent to the RDIMM In case of DDR4, the bit\\[3:2\\]
corresponds to the bank group bits. Therefore, the bit\\[3\\]
as well as the bit\\[2:0\\]
must be set to an appropriate value which is considered both the Address Mirroring of UDIMMs/RDIMMs and the Output Inversion of RDIMMs. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_addr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Address of the mode register that is to be written to. 0000 - MR0 0001 - MR1 0010 - MR2 0011 - MR3 0100 - MR4 0101 - MR5 0110 - MR6 0111 - MR7 Don't Care for LPDDR2/LPDDR3 (see MRCTRL1.mr_data for mode register addressing in LPDDR2/LPDDR3) This signal is also used for writing to control words of RDIMMs. In that case, it corresponds to the bank address bits sent to the RDIMM In case of DDR4, the bit\\[3:2\\]
corresponds to the bank group bits. Therefore, the bit\\[3\\]
as well as the bit\\[2:0\\]
must be set to an appropriate value which is considered both the Address Mirroring of UDIMMs/RDIMMs and the Output Inversion of RDIMMs. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_mr_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Description: Setting this register bit to 1 triggers a mode register read or write operation. When the MR operation is complete, the uMCTL2 automatically clears this bit. The other register fields of this register must be written in a separate APB transaction, before setting this mr_wr bit. It is recommended NOT to set this signal if in Init, Deep power- down or MPSM operating modes. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_wr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Setting this register bit to 1 triggers a mode register read or write operation. When the MR operation is complete, the uMCTL2 automatically clears this bit. The other register fields of this register must be written in a separate APB transaction, before setting this mr_wr bit. It is recommended NOT to set this signal if in Init, Deep power- down or MPSM operating modes. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_mr_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mrctrl0 {
        #[inline(always)]
        fn default() -> Mrctrl0 {
            Mrctrl0(0)
        }
    }
    impl core::fmt::Debug for Mrctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrctrl0")
                .field("mr_rank", &self.mr_rank())
                .field("mr_addr", &self.mr_addr())
                .field("mr_wr", &self.mr_wr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mrctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mrctrl0 {{ mr_rank: {=u8:?}, mr_addr: {=u8:?}, mr_wr: {=bool:?} }}",
                self.mr_rank(),
                self.mr_addr(),
                self.mr_wr()
            )
        }
    }
    #[doc = "Description: Mode Register Read/Write Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrctrl1(pub u32);
    impl Mrctrl1 {
        #[doc = "Description: Mode register write data for all non- LPDDR2/non-LPDDR3 modes. For LPDDR2/LPDDR3, MRCTRL1\\[15:0\\]
are interpreted as \\[15:8\\]
MR Address and \\[7:0\\]
MR data for writes, don't care for reads. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Description: Mode register write data for all non- LPDDR2/non-LPDDR3 modes. For LPDDR2/LPDDR3, MRCTRL1\\[15:0\\]
are interpreted as \\[15:8\\]
MR Address and \\[7:0\\]
MR data for writes, don't care for reads. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_mr_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Mrctrl1 {
        #[inline(always)]
        fn default() -> Mrctrl1 {
            Mrctrl1(0)
        }
    }
    impl core::fmt::Debug for Mrctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrctrl1")
                .field("mr_data", &self.mr_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mrctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mrctrl1 {{ mr_data: {=u32:?} }}", self.mr_data())
        }
    }
    #[doc = "Description: Mode Register Read/Write Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mrstat(pub u32);
    impl Mrstat {
        #[doc = "Description: The SoC core may initiate a MR write operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the MRW/MRR request. It goes low when the MRW/MRR command is issued to the SDRAM. It is recommended not to perform MRW/MRR commands when 'MRSTAT.mr_wr_busy' is high. 0 - Indicates that the SoC core can initiate a mode register write operation 1 - Indicates that mode register write operation is in progress Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn mr_wr_busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: The SoC core may initiate a MR write operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the MRW/MRR request. It goes low when the MRW/MRR command is issued to the SDRAM. It is recommended not to perform MRW/MRR commands when 'MRSTAT.mr_wr_busy' is high. 0 - Indicates that the SoC core can initiate a mode register write operation 1 - Indicates that mode register write operation is in progress Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_mr_wr_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Mrstat {
        #[inline(always)]
        fn default() -> Mrstat {
            Mrstat(0)
        }
    }
    impl core::fmt::Debug for Mrstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mrstat")
                .field("mr_wr_busy", &self.mr_wr_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mrstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mrstat {{ mr_wr_busy: {=bool:?} }}", self.mr_wr_busy())
        }
    }
    #[doc = "Description: Master Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mstr(pub u32);
    impl Mstr {
        #[doc = "Description: Select DDR3 SDRAM 1 - DDR3 SDRAM device in use 0 - non-DDR3 SDRAM device in use Only present in designs that support DDR3. Value After Reset: \"(MEMC_DDR3_EN==1) ? 0x1 : 0x0\" Exists: MEMC_DDR3==1."]
        #[must_use]
        #[inline(always)]
        pub const fn ddr3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Select DDR3 SDRAM 1 - DDR3 SDRAM device in use 0 - non-DDR3 SDRAM device in use Only present in designs that support DDR3. Value After Reset: \"(MEMC_DDR3_EN==1) ? 0x1 : 0x0\" Exists: MEMC_DDR3==1."]
        #[inline(always)]
        pub const fn set_ddr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: When set, enable burst-chop in DDR3/DDR4. This is only supported in full bus width mode (MSTR.data_bus_width = 00). If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), burst chop is not supported, and this bit must be set to '0' Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn burstchop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When set, enable burst-chop in DDR3/DDR4. This is only supported in full bus width mode (MSTR.data_bus_width = 00). If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), burst chop is not supported, and this bit must be set to '0' Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1."]
        #[inline(always)]
        pub const fn set_burstchop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Description: If 1, then uMCTL2 uses 2T timing. Otherwise, uses 1T timing. In 2T timing, all command signals (except chip select) are held for 2 clocks on the SDRAM bus. Chip select is asserted on the second cycle of the command Note: 2T timing is not supported in LPDDR2/LPDDR3 mode Note: 2T timing is not supported if the configuration parameter MEMC_CMD_RTN2IDLE is set Note: 2T timing is not supported in DDR4 geardown mode. Value After Reset: 0x0 Exists: MEMC_CMD_RTN2IDLE==0."]
        #[must_use]
        #[inline(always)]
        pub const fn en_2t_timing_mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If 1, then uMCTL2 uses 2T timing. Otherwise, uses 1T timing. In 2T timing, all command signals (except chip select) are held for 2 clocks on the SDRAM bus. Chip select is asserted on the second cycle of the command Note: 2T timing is not supported in LPDDR2/LPDDR3 mode Note: 2T timing is not supported if the configuration parameter MEMC_CMD_RTN2IDLE is set Note: 2T timing is not supported in DDR4 geardown mode. Value After Reset: 0x0 Exists: MEMC_CMD_RTN2IDLE==0."]
        #[inline(always)]
        pub const fn set_en_2t_timing_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Description: Selects proportion of DQ bus width that is used by the SDRAM 00 - Full DQ bus width to SDRAM 01 - Half DQ bus width to SDRAM 10 - Quarter DQ bus width to SDRAM 11 - Reserved. Note that half bus width mode is only supported when the SDRAM bus width is a multiple of 16, and quarter bus width mode is only supported when the SDRAM bus width is a multiple of 32 and the configuration parameter MEMC_QBUS_SUPPORT is set. Bus width refers to DQ bus width (excluding any ECC width). Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn data_bus_width(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Description: Selects proportion of DQ bus width that is used by the SDRAM 00 - Full DQ bus width to SDRAM 01 - Half DQ bus width to SDRAM 10 - Quarter DQ bus width to SDRAM 11 - Reserved. Note that half bus width mode is only supported when the SDRAM bus width is a multiple of 16, and quarter bus width mode is only supported when the SDRAM bus width is a multiple of 32 and the configuration parameter MEMC_QBUS_SUPPORT is set. Bus width refers to DQ bus width (excluding any ECC width). Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_data_bus_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Description: Set to 1 when uMCTL2 and DRAM has to be put in DLL-off mode for low frequency operation. Set to 0 to put uMCTL2 and DRAM in DLL-on mode for normal frequency operation. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dll_off_mode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Set to 1 when uMCTL2 and DRAM has to be put in DLL-off mode for low frequency operation. Set to 0 to put uMCTL2 and DRAM in DLL-on mode for normal frequency operation. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4==1."]
        #[inline(always)]
        pub const fn set_dll_off_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Description: SDRAM burst length used: 0001 - Burst length of 2 (only supported for mDDR) 0010 - Burst length of 4 0100 - Burst length of 8 1000 - Burst length of 16 (only supported for mDDR and LPDDR2) All other values are reserved. This controls the burst size used to access the SDRAM. This must match the burst length mode register setting in the SDRAM. Burst length of 2 is not supported with AXI ports when MEMC_BURST_LENGTH is 8. Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn burst_rdwr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: SDRAM burst length used: 0001 - Burst length of 2 (only supported for mDDR) 0010 - Burst length of 4 0100 - Burst length of 8 1000 - Burst length of 16 (only supported for mDDR and LPDDR2) All other values are reserved. This controls the burst size used to access the SDRAM. This must match the burst length mode register setting in the SDRAM. Burst length of 2 is not supported with AXI ports when MEMC_BURST_LENGTH is 8. Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_burst_rdwr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Description: Only present for multi-rank configurations. Each bit represents one rank. For two-rank configurations, only bits\\[25:24\\]
are present. 1 - populated 0 - unpopulated LSB is the lowest rank number. For 2 ranks following combinations are legal: 01 - One rank 11 - Two ranks Others - Reserved. For 4 ranks following combinations are legal: 0001 - One rank 0011 - Two ranks 1111 - Four ranks Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn active_ranks(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Only present for multi-rank configurations. Each bit represents one rank. For two-rank configurations, only bits\\[25:24\\]
are present. 1 - populated 0 - unpopulated LSB is the lowest rank number. For 2 ranks following combinations are legal: 01 - One rank 11 - Two ranks Others - Reserved. For 4 ranks following combinations are legal: 0001 - One rank 0011 - Two ranks 1111 - Four ranks Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_active_ranks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Mstr {
        #[inline(always)]
        fn default() -> Mstr {
            Mstr(0)
        }
    }
    impl core::fmt::Debug for Mstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mstr")
                .field("ddr3", &self.ddr3())
                .field("burstchop", &self.burstchop())
                .field("en_2t_timing_mode", &self.en_2t_timing_mode())
                .field("data_bus_width", &self.data_bus_width())
                .field("dll_off_mode", &self.dll_off_mode())
                .field("burst_rdwr", &self.burst_rdwr())
                .field("active_ranks", &self.active_ranks())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mstr {{ ddr3: {=bool:?}, burstchop: {=bool:?}, en_2t_timing_mode: {=bool:?}, data_bus_width: {=u8:?}, dll_off_mode: {=bool:?}, burst_rdwr: {=u8:?}, active_ranks: {=u8:?} }}" , self . ddr3 () , self . burstchop () , self . en_2t_timing_mode () , self . data_bus_width () , self . dll_off_mode () , self . burst_rdwr () , self . active_ranks ())
        }
    }
    #[doc = "Description: ODT Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odtcfg(pub u32);
    impl Odtcfg {
        #[doc = "Description: The delay, in clock cycles, from issuing a read command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CL + AL < 4), then 0. If (CL + AL >= 4), then (CL + AL - 4) DDR3 (CL - CWL) DDR4 If CAL mode is enabled, CL - CWL + DFITMG1.dfi_t_cmd_lat If CAL mode is not enabled, CL - CWL -1, or 0 if CL - CWL < 1 LPDDR3, MEMC_FREQ_RATIO=2 CL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_odt_delay(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: The delay, in clock cycles, from issuing a read command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CL + AL < 4), then 0. If (CL + AL >= 4), then (CL + AL - 4) DDR3 (CL - CWL) DDR4 If CAL mode is enabled, CL - CWL + DFITMG1.dfi_t_cmd_lat If CAL mode is not enabled, CL - CWL -1, or 0 if CL - CWL < 1 LPDDR3, MEMC_FREQ_RATIO=2 CL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_odt_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "Description: Cycles to hold ODT for a read command. The minimum supported value is 2. Recommended values: DDR2/DDR3 BL8 - 0x6 BL4 - 0x4 DDR4 - 0x6, but needs to be reduced to 0x5 in CAL mode to avoid overlap of read and write ODT LPDDR3 - RU(tDQSCKmax/tCK) + 4 + 1 Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_odt_hold(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Cycles to hold ODT for a read command. The minimum supported value is 2. Recommended values: DDR2/DDR3 BL8 - 0x6 BL4 - 0x4 DDR4 - 0x6, but needs to be reduced to 0x5 in CAL mode to avoid overlap of read and write ODT LPDDR3 - RU(tDQSCKmax/tCK) + 4 + 1 Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_odt_hold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: The delay, in clock cycles, from issuing a write command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CWL + AL < 3), then 0. If (CWL + AL >= 3), then (CWL + AL - 3) DDR3 - 0 DDR4 - DFITMG1.dfi_t_cmd_lat (to adjust for CAL mode) LPDDR3 - (CWL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_odt_delay(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: The delay, in clock cycles, from issuing a write command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CWL + AL < 3), then 0. If (CWL + AL >= 3), then (CWL + AL - 3) DDR3 - 0 DDR4 - DFITMG1.dfi_t_cmd_lat (to adjust for CAL mode) LPDDR3 - (CWL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_odt_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Description: Cycles to hold ODT for a write command. The minimum supported value is 2. DDR2/DDR3/DDR4 BL8 - 0x6 BL4 - 0x4 LPDDR3 - RU(tDQSSmax/tCK) + 4 Value After Reset: 0x4 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_odt_hold(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Cycles to hold ODT for a write command. The minimum supported value is 2. DDR2/DDR3/DDR4 BL8 - 0x6 BL4 - 0x4 LPDDR3 - RU(tDQSSmax/tCK) + 4 Value After Reset: 0x4 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_odt_hold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Odtcfg {
        #[inline(always)]
        fn default() -> Odtcfg {
            Odtcfg(0)
        }
    }
    impl core::fmt::Debug for Odtcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odtcfg")
                .field("rd_odt_delay", &self.rd_odt_delay())
                .field("rd_odt_hold", &self.rd_odt_hold())
                .field("wr_odt_delay", &self.wr_odt_delay())
                .field("wr_odt_hold", &self.wr_odt_hold())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odtcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Odtcfg {{ rd_odt_delay: {=u8:?}, rd_odt_hold: {=u8:?}, wr_odt_delay: {=u8:?}, wr_odt_hold: {=u8:?} }}" , self . rd_odt_delay () , self . rd_odt_hold () , self . wr_odt_delay () , self . wr_odt_hold ())
        }
    }
    #[doc = "Description: ODT/Rank Map Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odtmap(pub u32);
    impl Odtmap {
        #[doc = "Description: Indicates which remote ODTs must be turned on during a write to rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rank0_wr_odt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a write to rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_rank0_wr_odt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a read from rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rank0_rd_odt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a read from rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_rank0_rd_odt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a write to rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn rank1_wr_odt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a write to rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_rank1_wr_odt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a read from rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn rank1_rd_odt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Indicates which remote ODTs must be turned on during a read from rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_rank1_rd_odt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Odtmap {
        #[inline(always)]
        fn default() -> Odtmap {
            Odtmap(0)
        }
    }
    impl core::fmt::Debug for Odtmap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odtmap")
                .field("rank0_wr_odt", &self.rank0_wr_odt())
                .field("rank0_rd_odt", &self.rank0_rd_odt())
                .field("rank1_wr_odt", &self.rank1_wr_odt())
                .field("rank1_rd_odt", &self.rank1_rd_odt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odtmap {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Odtmap {{ rank0_wr_odt: {=u8:?}, rank0_rd_odt: {=u8:?}, rank1_wr_odt: {=u8:?}, rank1_rd_odt: {=u8:?} }}" , self . rank0_wr_odt () , self . rank0_rd_odt () , self . rank1_wr_odt () , self . rank1_rd_odt ())
        }
    }
    #[doc = "Description: Port Common Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pccfg(pub u32);
    impl Pccfg {
        #[doc = "Description: If set to 1 (enabled), sets co_gs_go2critical_wr and co_gs_go2critical_rd signals going to DDRC based on urgent input (awurgent, arurgent) coming from AXI master. If set to 0 (disabled), co_gs_go2critical_wr and co_gs_go2critical_rd signals at DDRC are driven to 1b'0. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn go2critical_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1 (enabled), sets co_gs_go2critical_wr and co_gs_go2critical_rd signals going to DDRC based on urgent input (awurgent, arurgent) coming from AXI master. If set to 0 (disabled), co_gs_go2critical_wr and co_gs_go2critical_rd signals at DDRC are driven to 1b'0. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_go2critical_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Page match four limit. If set to 1, limits the number of consecutive same page DDRC transactions that can be granted by the Port Arbiter to four when Page Match feature is enabled. If set to 0, there is no limit imposed on number of consecutive same page DDRC transactions. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn pagematch_limit(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Page match four limit. If set to 1, limits the number of consecutive same page DDRC transactions that can be granted by the Port Arbiter to four when Page Match feature is enabled. If set to 0, there is no limit imposed on number of consecutive same page DDRC transactions. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_pagematch_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pccfg {
        #[inline(always)]
        fn default() -> Pccfg {
            Pccfg(0)
        }
    }
    impl core::fmt::Debug for Pccfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pccfg")
                .field("go2critical_en", &self.go2critical_en())
                .field("pagematch_limit", &self.pagematch_limit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pccfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pccfg {{ go2critical_en: {=bool:?}, pagematch_limit: {=bool:?} }}",
                self.go2critical_en(),
                self.pagematch_limit()
            )
        }
    }
    #[doc = "Description: High Priority Read CAM Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perfhpr1(pub u32);
    impl Perfhpr1 {
        #[doc = "Description: Number of clocks that the HPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn hpr_max_starve(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: Number of clocks that the HPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_hpr_max_starve(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: Number of transactions that are serviced once the HPR queue goes critical is the smaller of: This number Number of transactions available Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn hpr_xact_run_length(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Description: Number of transactions that are serviced once the HPR queue goes critical is the smaller of: This number Number of transactions available Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[inline(always)]
        pub const fn set_hpr_xact_run_length(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Perfhpr1 {
        #[inline(always)]
        fn default() -> Perfhpr1 {
            Perfhpr1(0)
        }
    }
    impl core::fmt::Debug for Perfhpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perfhpr1")
                .field("hpr_max_starve", &self.hpr_max_starve())
                .field("hpr_xact_run_length", &self.hpr_xact_run_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perfhpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perfhpr1 {{ hpr_max_starve: {=u16:?}, hpr_xact_run_length: {=u8:?} }}",
                self.hpr_max_starve(),
                self.hpr_xact_run_length()
            )
        }
    }
    #[doc = "Description: Low Priority Read CAM Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perflpr1(pub u32);
    impl Perflpr1 {
        #[doc = "Description: Number of clocks that the LPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn lpr_max_starve(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: Number of clocks that the LPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always."]
        #[inline(always)]
        pub const fn set_lpr_max_starve(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: Number of transactions that are serviced once the LPR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn lpr_xact_run_length(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Description: Number of transactions that are serviced once the LPR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[inline(always)]
        pub const fn set_lpr_xact_run_length(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Perflpr1 {
        #[inline(always)]
        fn default() -> Perflpr1 {
            Perflpr1(0)
        }
    }
    impl core::fmt::Debug for Perflpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perflpr1")
                .field("lpr_max_starve", &self.lpr_max_starve())
                .field("lpr_xact_run_length", &self.lpr_xact_run_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perflpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perflpr1 {{ lpr_max_starve: {=u16:?}, lpr_xact_run_length: {=u8:?} }}",
                self.lpr_max_starve(),
                self.lpr_xact_run_length()
            )
        }
    }
    #[doc = "Description: Variable Priority Read CAM Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perfvpr1(pub u32);
    impl Perfvpr1 {
        #[doc = "Description: Indicates the range of the timeout value that is used for grouping the expired VPR commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPR commands whose timeout counters are 15 or below will be considered as expired-VPR commands when the timeout value of any of the VPR commands reach 0. The expired-VPR commands, when present, are given higher priority than HPR commands. The VPR commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPR commands that come in to DDRC will time-out right-away and will be considered as expired-VPR. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPR. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPR_EN==1."]
        #[must_use]
        #[inline(always)]
        pub const fn vpr_timeout_range(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Description: Indicates the range of the timeout value that is used for grouping the expired VPR commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPR commands whose timeout counters are 15 or below will be considered as expired-VPR commands when the timeout value of any of the VPR commands reach 0. The expired-VPR commands, when present, are given higher priority than HPR commands. The VPR commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPR commands that come in to DDRC will time-out right-away and will be considered as expired-VPR. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPR. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPR_EN==1."]
        #[inline(always)]
        pub const fn set_vpr_timeout_range(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Perfvpr1 {
        #[inline(always)]
        fn default() -> Perfvpr1 {
            Perfvpr1(0)
        }
    }
    impl core::fmt::Debug for Perfvpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perfvpr1")
                .field("vpr_timeout_range", &self.vpr_timeout_range())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perfvpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perfvpr1 {{ vpr_timeout_range: {=u16:?} }}",
                self.vpr_timeout_range()
            )
        }
    }
    #[doc = "Description: Variable Priority Write CAM Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perfvpw1(pub u32);
    impl Perfvpw1 {
        #[doc = "Description: Indicates the range of the timeout value that is used for grouping the expired VPW commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPW commands whose timeout counters are 15 or below will be considered as expired-VPW commands when the timeout value of any of the VPW commands reach 0. The expired-VPW commands, when present, are given higher priority than normal Write commands. The VPW commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPW commands that come in to DDRC will time-out right-away and will be considered as expired-VPW. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPW. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPW_EN==1."]
        #[must_use]
        #[inline(always)]
        pub const fn vpw_timeout_range(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Description: Indicates the range of the timeout value that is used for grouping the expired VPW commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPW commands whose timeout counters are 15 or below will be considered as expired-VPW commands when the timeout value of any of the VPW commands reach 0. The expired-VPW commands, when present, are given higher priority than normal Write commands. The VPW commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPW commands that come in to DDRC will time-out right-away and will be considered as expired-VPW. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPW. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPW_EN==1."]
        #[inline(always)]
        pub const fn set_vpw_timeout_range(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Perfvpw1 {
        #[inline(always)]
        fn default() -> Perfvpw1 {
            Perfvpw1(0)
        }
    }
    impl core::fmt::Debug for Perfvpw1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perfvpw1")
                .field("vpw_timeout_range", &self.vpw_timeout_range())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perfvpw1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perfvpw1 {{ vpw_timeout_range: {=u16:?} }}",
                self.vpw_timeout_range()
            )
        }
    }
    #[doc = "Description: Write CAM Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perfwr1(pub u32);
    impl Perfwr1 {
        #[doc = "Description: Number of clocks that the WR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn w_max_starve(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Description: Number of clocks that the WR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always."]
        #[inline(always)]
        pub const fn set_w_max_starve(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Description: Number of transactions that are serviced once the WR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn w_xact_run_length(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Description: Number of transactions that are serviced once the WR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always."]
        #[inline(always)]
        pub const fn set_w_xact_run_length(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Perfwr1 {
        #[inline(always)]
        fn default() -> Perfwr1 {
            Perfwr1(0)
        }
    }
    impl core::fmt::Debug for Perfwr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perfwr1")
                .field("w_max_starve", &self.w_max_starve())
                .field("w_xact_run_length", &self.w_xact_run_length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perfwr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perfwr1 {{ w_max_starve: {=u16:?}, w_xact_run_length: {=u8:?} }}",
                self.w_max_starve(),
                self.w_xact_run_length()
            )
        }
    }
    #[doc = "Description: Port Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pstat(pub u32);
    impl Pstat {
        #[doc = "Description: Indicates if there are outstanding reads for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_busy_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding reads for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1."]
        #[inline(always)]
        pub const fn set_rd_port_busy_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_busy_15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Indicates if there are outstanding writes for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1."]
        #[inline(always)]
        pub const fn set_wr_port_busy_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pstat {
        #[inline(always)]
        fn default() -> Pstat {
            Pstat(0)
        }
    }
    impl core::fmt::Debug for Pstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pstat")
                .field("rd_port_busy_0", &self.rd_port_busy_0())
                .field("rd_port_busy_1", &self.rd_port_busy_1())
                .field("rd_port_busy_2", &self.rd_port_busy_2())
                .field("rd_port_busy_3", &self.rd_port_busy_3())
                .field("rd_port_busy_4", &self.rd_port_busy_4())
                .field("rd_port_busy_5", &self.rd_port_busy_5())
                .field("rd_port_busy_6", &self.rd_port_busy_6())
                .field("rd_port_busy_7", &self.rd_port_busy_7())
                .field("rd_port_busy_8", &self.rd_port_busy_8())
                .field("rd_port_busy_9", &self.rd_port_busy_9())
                .field("rd_port_busy_10", &self.rd_port_busy_10())
                .field("rd_port_busy_11", &self.rd_port_busy_11())
                .field("rd_port_busy_12", &self.rd_port_busy_12())
                .field("rd_port_busy_13", &self.rd_port_busy_13())
                .field("rd_port_busy_14", &self.rd_port_busy_14())
                .field("rd_port_busy_15", &self.rd_port_busy_15())
                .field("wr_port_busy_0", &self.wr_port_busy_0())
                .field("wr_port_busy_1", &self.wr_port_busy_1())
                .field("wr_port_busy_2", &self.wr_port_busy_2())
                .field("wr_port_busy_3", &self.wr_port_busy_3())
                .field("wr_port_busy_4", &self.wr_port_busy_4())
                .field("wr_port_busy_5", &self.wr_port_busy_5())
                .field("wr_port_busy_6", &self.wr_port_busy_6())
                .field("wr_port_busy_7", &self.wr_port_busy_7())
                .field("wr_port_busy_8", &self.wr_port_busy_8())
                .field("wr_port_busy_9", &self.wr_port_busy_9())
                .field("wr_port_busy_10", &self.wr_port_busy_10())
                .field("wr_port_busy_11", &self.wr_port_busy_11())
                .field("wr_port_busy_12", &self.wr_port_busy_12())
                .field("wr_port_busy_13", &self.wr_port_busy_13())
                .field("wr_port_busy_14", &self.wr_port_busy_14())
                .field("wr_port_busy_15", &self.wr_port_busy_15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pstat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pstat {{ rd_port_busy_0: {=bool:?}, rd_port_busy_1: {=bool:?}, rd_port_busy_2: {=bool:?}, rd_port_busy_3: {=bool:?}, rd_port_busy_4: {=bool:?}, rd_port_busy_5: {=bool:?}, rd_port_busy_6: {=bool:?}, rd_port_busy_7: {=bool:?}, rd_port_busy_8: {=bool:?}, rd_port_busy_9: {=bool:?}, rd_port_busy_10: {=bool:?}, rd_port_busy_11: {=bool:?}, rd_port_busy_12: {=bool:?}, rd_port_busy_13: {=bool:?}, rd_port_busy_14: {=bool:?}, rd_port_busy_15: {=bool:?}, wr_port_busy_0: {=bool:?}, wr_port_busy_1: {=bool:?}, wr_port_busy_2: {=bool:?}, wr_port_busy_3: {=bool:?}, wr_port_busy_4: {=bool:?}, wr_port_busy_5: {=bool:?}, wr_port_busy_6: {=bool:?}, wr_port_busy_7: {=bool:?}, wr_port_busy_8: {=bool:?}, wr_port_busy_9: {=bool:?}, wr_port_busy_10: {=bool:?}, wr_port_busy_11: {=bool:?}, wr_port_busy_12: {=bool:?}, wr_port_busy_13: {=bool:?}, wr_port_busy_14: {=bool:?}, wr_port_busy_15: {=bool:?} }}" , self . rd_port_busy_0 () , self . rd_port_busy_1 () , self . rd_port_busy_2 () , self . rd_port_busy_3 () , self . rd_port_busy_4 () , self . rd_port_busy_5 () , self . rd_port_busy_6 () , self . rd_port_busy_7 () , self . rd_port_busy_8 () , self . rd_port_busy_9 () , self . rd_port_busy_10 () , self . rd_port_busy_11 () , self . rd_port_busy_12 () , self . rd_port_busy_13 () , self . rd_port_busy_14 () , self . rd_port_busy_15 () , self . wr_port_busy_0 () , self . wr_port_busy_1 () , self . wr_port_busy_2 () , self . wr_port_busy_3 () , self . wr_port_busy_4 () , self . wr_port_busy_5 () , self . wr_port_busy_6 () , self . wr_port_busy_7 () , self . wr_port_busy_8 () , self . wr_port_busy_9 () , self . wr_port_busy_10 () , self . wr_port_busy_11 () , self . wr_port_busy_12 () , self . wr_port_busy_13 () , self . wr_port_busy_14 () , self . wr_port_busy_15 ())
        }
    }
    #[doc = "Description: Low Power Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrctl(pub u32);
    impl Pwrctl {
        #[doc = "Description: If true then the uMCTL2 puts the SDRAM into Self Refresh after a programmable number of cycles \"maximum idle clocks before Self Refresh (PWRTMG.selfref_to_x32)\". This register bit may be re- programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn selfref_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If true then the uMCTL2 puts the SDRAM into Self Refresh after a programmable number of cycles \"maximum idle clocks before Self Refresh (PWRTMG.selfref_to_x32)\". This register bit may be re- programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_selfref_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: If true then the uMCTL2 goes into power-down after a programmable number of cycles \"maximum idle clocks before power down\" (PWRTMG.powerdown_to_x32). This register bit may be re-programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn powerdown_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If true then the uMCTL2 goes into power-down after a programmable number of cycles \"maximum idle clocks before power down\" (PWRTMG.powerdown_to_x32). This register bit may be re-programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_powerdown_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: Enable the assertion of dfi_dram_clk_disable whenever a clock is not required by the SDRAM. If set to 0, dfi_dram_clk_disable is never asserted. Assertion of dfi_dram_clk_disable is as follows: In DDR2/DDR3, can only be asserted in Self Refresh. In DDR4, can be asserted in following: in Self Refresh. in Maximum Power Saving Mode In mDDR/LPDDR2/LPDDR3, can be asserted in following: in Self Refresh in Power Down in Deep Power Down during Normal operation (Clock Stop) Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn en_dfi_dram_clk_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enable the assertion of dfi_dram_clk_disable whenever a clock is not required by the SDRAM. If set to 0, dfi_dram_clk_disable is never asserted. Assertion of dfi_dram_clk_disable is as follows: In DDR2/DDR3, can only be asserted in Self Refresh. In DDR4, can be asserted in following: in Self Refresh. in Maximum Power Saving Mode In mDDR/LPDDR2/LPDDR3, can be asserted in following: in Self Refresh in Power Down in Deep Power Down during Normal operation (Clock Stop) Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_en_dfi_dram_clk_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Description: A value of 1 to this register causes system to move to Self Refresh state immediately, as long as it is not in INIT or DPD/MPSM operating_mode. This is referred to as Software Entry/Exit to Self Refresh. 1 - Software Entry to Self Refresh 0 - Software Exit from Self Refresh Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn selfref_sw(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Description: A value of 1 to this register causes system to move to Self Refresh state immediately, as long as it is not in INIT or DPD/MPSM operating_mode. This is referred to as Software Entry/Exit to Self Refresh. 1 - Software Entry to Self Refresh 0 - Software Exit from Self Refresh Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_selfref_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Pwrctl {
        #[inline(always)]
        fn default() -> Pwrctl {
            Pwrctl(0)
        }
    }
    impl core::fmt::Debug for Pwrctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrctl")
                .field("selfref_en", &self.selfref_en())
                .field("powerdown_en", &self.powerdown_en())
                .field("en_dfi_dram_clk_disable", &self.en_dfi_dram_clk_disable())
                .field("selfref_sw", &self.selfref_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwrctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pwrctl {{ selfref_en: {=bool:?}, powerdown_en: {=bool:?}, en_dfi_dram_clk_disable: {=bool:?}, selfref_sw: {=bool:?} }}" , self . selfref_en () , self . powerdown_en () , self . en_dfi_dram_clk_disable () , self . selfref_sw ())
        }
    }
    #[doc = "Description: Low Power Timing Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrtmg(pub u32);
    impl Pwrtmg {
        #[doc = "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into power-down. This must be enabled in the PWRCTL.powerdown_en. Unit: Multiples of 32 clocks FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn powerdown_to_x32(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into power-down. This must be enabled in the PWRCTL.powerdown_en. Unit: Multiples of 32 clocks FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_powerdown_to_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into Self Refresh. This must be enabled in the PWRCTL.selfref_en. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x40 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn selfref_to_x32(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into Self Refresh. This must be enabled in the PWRCTL.selfref_en. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x40 Exists: Always."]
        #[inline(always)]
        pub const fn set_selfref_to_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Pwrtmg {
        #[inline(always)]
        fn default() -> Pwrtmg {
            Pwrtmg(0)
        }
    }
    impl core::fmt::Debug for Pwrtmg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrtmg")
                .field("powerdown_to_x32", &self.powerdown_to_x32())
                .field("selfref_to_x32", &self.selfref_to_x32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwrtmg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pwrtmg {{ powerdown_to_x32: {=u8:?}, selfref_to_x32: {=u8:?} }}",
                self.powerdown_to_x32(),
                self.selfref_to_x32()
            )
        }
    }
    #[doc = "Description: Port n Read QoS Configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qos0(pub u32);
    impl Qos0 {
        #[doc = "Description: Separation level1 indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 13(for dual RAQ) or 0 to 14(for single RAQ) which corresponds to arqos. Note that for PA, arqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. All of the map_level* registers must be set to distinct values. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos_map_level1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Separation level1 indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 13(for dual RAQ) or 0 to 14(for single RAQ) which corresponds to arqos. Note that for PA, arqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. All of the map_level* registers must be set to distinct values. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rqos_map_level1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region 0 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos_map_region0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region 0 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rqos_map_region0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region1 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region 1 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos_map_region1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region1 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region 1 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rqos_map_region1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Qos0 {
        #[inline(always)]
        fn default() -> Qos0 {
            Qos0(0)
        }
    }
    impl core::fmt::Debug for Qos0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qos0")
                .field("rqos_map_level1", &self.rqos_map_level1())
                .field("rqos_map_region0", &self.rqos_map_region0())
                .field("rqos_map_region1", &self.rqos_map_region1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qos0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Qos0 {{ rqos_map_level1: {=u8:?}, rqos_map_region0: {=u8:?}, rqos_map_region1: {=u8:?} }}" , self . rqos_map_level1 () , self . rqos_map_region0 () , self . rqos_map_region1 ())
        }
    }
    #[doc = "Description: Port n Read QoS Configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qos1(pub u32);
    impl Qos1 {
        #[doc = "Description: Specifies the timeout value for transactions mapped to the blue address queue. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos_map_timeoutb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Description: Specifies the timeout value for transactions mapped to the blue address queue. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rqos_map_timeoutb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Description: Specifies the timeout value for transactions mapped to the red address queue. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rqos_map_timeoutr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Description: Specifies the timeout value for transactions mapped to the red address queue. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rqos_map_timeoutr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Qos1 {
        #[inline(always)]
        fn default() -> Qos1 {
            Qos1(0)
        }
    }
    impl core::fmt::Debug for Qos1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qos1")
                .field("rqos_map_timeoutb", &self.rqos_map_timeoutb())
                .field("rqos_map_timeoutr", &self.rqos_map_timeoutr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qos1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Qos1 {{ rqos_map_timeoutb: {=u16:?}, rqos_map_timeoutr: {=u16:?} }}",
                self.rqos_map_timeoutb(),
                self.rqos_map_timeoutr()
            )
        }
    }
    #[doc = "Description: Port n Configuration Read Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R(pub u32);
    impl R {
        #[doc = "Description: Determines the initial load value of read aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the read aging counter sets the priority of the read channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level (timeout condition - Priority0). For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (arqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. In this case, external dynamic priority input, arqos (for reads only) can still be used to set the DDRC read priority (2 priority levels: low priority read - LPR, high priority read - HPR) on a command by command basis. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_priority(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Determines the initial load value of read aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the read aging counter sets the priority of the read channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level (timeout condition - Priority0). For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (arqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. In this case, external dynamic priority input, arqos (for reads only) can still be used to set the DDRC read priority (2 priority levels: low priority read - LPR, high priority read - HPR) on a command by command basis. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_port_priority(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: If set to 1, enables aging function for the read channel of the port. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_aging_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables aging function for the read channel of the port. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_port_aging_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Description: If set to 1, enables the AXI urgent sideband signal (arurgent). When enabled and arurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_rd signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that arurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_urgent_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables the AXI urgent sideband signal (arurgent). When enabled and arurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_rd signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that arurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_port_urgent_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: \"(MEMC_DDR4_EN==1) ? 0x0 : 0x1\" Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_port_pagematch_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: \"(MEMC_DDR4_EN==1) ? 0x0 : 0x1\" Exists: Always."]
        #[inline(always)]
        pub const fn set_rd_port_pagematch_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for R {
        #[inline(always)]
        fn default() -> R {
            R(0)
        }
    }
    impl core::fmt::Debug for R {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R")
                .field("rd_port_priority", &self.rd_port_priority())
                .field("rd_port_aging_en", &self.rd_port_aging_en())
                .field("rd_port_urgent_en", &self.rd_port_urgent_en())
                .field("rd_port_pagematch_en", &self.rd_port_pagematch_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "R {{ rd_port_priority: {=u16:?}, rd_port_aging_en: {=bool:?}, rd_port_urgent_en: {=bool:?}, rd_port_pagematch_en: {=bool:?} }}" , self . rd_port_priority () , self . rd_port_aging_en () , self . rd_port_urgent_en () , self . rd_port_pagematch_en ())
        }
    }
    #[doc = "Description: Rank Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rankctl(pub u32);
    impl Rankctl {
        #[doc = "Description: Only present for multi-rank configurations. Background: Reads to the same rank can be performed back-to-back. Reads to different ranks require additional gap dictated by the register RANKCTL.diff_rank_rd_gap. This is to avoid possible data bus contention as well as to give PHY enough time to switch the delay when changing ranks. The uMCTL2 arbitrates for bus access on a cycle-by-cycle basis; therefore after a read is scheduled, there are few clock cycles (determined by the value on diff_rank_rd_gap register) in which only reads from the same rank are eligible to be scheduled. This prevents reads from other ranks from having fair access to the data bus. This parameter represents the maximum number of reads that can be scheduled consecutively to the same rank. After this number is reached, a delay equal to RANKCTL.diff_rank_rd_gap is inserted by the scheduler to allow all ranks a fair opportunity to be scheduled. Higher numbers increase bandwidth utilization, lower numbers increase fairness. This feature can be DISABLED by setting this register to 0. When set to 0, the Controller will stay on the same rank as long as commands are available for it. Minimum programmable value is 0 (feature disabled) and maximum programmable value is 0xF. Feature limitation: max_rank_rd feature works as described only in the mode in which one command at the DDRC input results in one DFI command at the output. An example of this mode is: BL8 hardware configuration (MEMC_BURST_LENGTH=8) and Full bus width mode (MSTR.data_bus_width=2'b00) and BL8 mode of operation (MSTR.burst_rdwr=4'b0100). In modes where single HIF command results in multiple DFI commands (eg: Half Bus Width, BL4 etc.), the same rank commands would be serviced for as long as they are available, which is equivalent to this feature being disabled. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn max_rank_rd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Only present for multi-rank configurations. Background: Reads to the same rank can be performed back-to-back. Reads to different ranks require additional gap dictated by the register RANKCTL.diff_rank_rd_gap. This is to avoid possible data bus contention as well as to give PHY enough time to switch the delay when changing ranks. The uMCTL2 arbitrates for bus access on a cycle-by-cycle basis; therefore after a read is scheduled, there are few clock cycles (determined by the value on diff_rank_rd_gap register) in which only reads from the same rank are eligible to be scheduled. This prevents reads from other ranks from having fair access to the data bus. This parameter represents the maximum number of reads that can be scheduled consecutively to the same rank. After this number is reached, a delay equal to RANKCTL.diff_rank_rd_gap is inserted by the scheduler to allow all ranks a fair opportunity to be scheduled. Higher numbers increase bandwidth utilization, lower numbers increase fairness. This feature can be DISABLED by setting this register to 0. When set to 0, the Controller will stay on the same rank as long as commands are available for it. Minimum programmable value is 0 (feature disabled) and maximum programmable value is 0xF. Feature limitation: max_rank_rd feature works as described only in the mode in which one command at the DDRC input results in one DFI command at the output. An example of this mode is: BL8 hardware configuration (MEMC_BURST_LENGTH=8) and Full bus width mode (MSTR.data_bus_width=2'b00) and BL8 mode of operation (MSTR.burst_rdwr=4'b0100). In modes where single HIF command results in multiple DFI commands (eg: Half Bus Width, BL4 etc.), the same rank commands would be serviced for as long as they are available, which is equivalent to this feature being disabled. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_max_rank_rd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive reads to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during reads. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn diff_rank_rd_gap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive reads to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during reads. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_diff_rank_rd_gap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive writes to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during writes. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn diff_rank_wr_gap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive writes to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during writes. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_diff_rank_wr_gap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Rankctl {
        #[inline(always)]
        fn default() -> Rankctl {
            Rankctl(0)
        }
    }
    impl core::fmt::Debug for Rankctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rankctl")
                .field("max_rank_rd", &self.max_rank_rd())
                .field("diff_rank_rd_gap", &self.diff_rank_rd_gap())
                .field("diff_rank_wr_gap", &self.diff_rank_wr_gap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rankctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rankctl {{ max_rank_rd: {=u8:?}, diff_rank_rd_gap: {=u8:?}, diff_rank_wr_gap: {=u8:?} }}" , self . max_rank_rd () , self . diff_rank_rd_gap () , self . diff_rank_wr_gap ())
        }
    }
    #[doc = "Description: Refresh Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfshctl0(pub u32);
    impl Rfshctl0 {
        #[doc = "Description: The programmed value + 1 is the number of refresh timeouts that is allowed to accumulate before traffic is blocked and the refreshes are forced to execute. Closing pages to perform a refresh is a one-time penalty that must be paid for each group of refreshes. Therefore, performing refreshes in a burst reduces the per-refresh penalty of these page closings. Higher numbers for RFSHCTL.refresh_burst slightly increases utilization; lower numbers decreases the worst-case latency associated with refreshes. 0 - single refresh 1 - burst-of-2 refresh 7 - burst-of-8 refresh For information on burst refresh feature refer to section 3.9 of DDR2 JEDEC specification - JESD79-2F.pdf. For DDR2/3, the refresh is always per-rank and not per- bank. The rank refresh can be accumulated over 8*tREFI cycles using the burst refresh feature. In DDR4 mode, according to Fine Granuarity feature, 8 refreshes can be postponed in 1X mode, 16 refreshes in 2X mode and 32 refreshes in 4X mode. If using PHY-initiated updates, care must be taken in the setting of RFSHCTL0.refresh_burst, to ensure that tRFCmax is not violated due to a PHY-initiated update occurring shortly before a refresh burst was due. In this situation, the refresh burst will be delayed until the PHY- initiated update is complete. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_burst(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: The programmed value + 1 is the number of refresh timeouts that is allowed to accumulate before traffic is blocked and the refreshes are forced to execute. Closing pages to perform a refresh is a one-time penalty that must be paid for each group of refreshes. Therefore, performing refreshes in a burst reduces the per-refresh penalty of these page closings. Higher numbers for RFSHCTL.refresh_burst slightly increases utilization; lower numbers decreases the worst-case latency associated with refreshes. 0 - single refresh 1 - burst-of-2 refresh 7 - burst-of-8 refresh For information on burst refresh feature refer to section 3.9 of DDR2 JEDEC specification - JESD79-2F.pdf. For DDR2/3, the refresh is always per-rank and not per- bank. The rank refresh can be accumulated over 8*tREFI cycles using the burst refresh feature. In DDR4 mode, according to Fine Granuarity feature, 8 refreshes can be postponed in 1X mode, 16 refreshes in 2X mode and 32 refreshes in 4X mode. If using PHY-initiated updates, care must be taken in the setting of RFSHCTL0.refresh_burst, to ensure that tRFCmax is not violated due to a PHY-initiated update occurring shortly before a refresh burst was due. In this situation, the refresh burst will be delayed until the PHY- initiated update is complete. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_refresh_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[doc = "Description: If the refresh timer (tRFCnom, also known as tREFI) has expired at least once, but it has not expired (RFSHCTL0.refresh_burst+1) times yet, then a speculative refresh may be performed. A speculative refresh is a refresh performed at a time when refresh would be useful, but before it is absolutely required. When the SDRAM bus is idle for a period of time determined by this RFSHCTL0.refresh_to_x32 and the refresh timer has expired at least once since the last refresh, then a speculative refresh is performed. Speculative refreshes continues successively until there are no refreshes pending or until new reads or writes are issued to the uMCTL2. FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_to_x32(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "Description: If the refresh timer (tRFCnom, also known as tREFI) has expired at least once, but it has not expired (RFSHCTL0.refresh_burst+1) times yet, then a speculative refresh may be performed. A speculative refresh is a refresh performed at a time when refresh would be useful, but before it is absolutely required. When the SDRAM bus is idle for a period of time determined by this RFSHCTL0.refresh_to_x32 and the refresh timer has expired at least once since the last refresh, then a speculative refresh is performed. Speculative refreshes continues successively until there are no refreshes pending or until new reads or writes are issued to the uMCTL2. FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always."]
        #[inline(always)]
        pub const fn set_refresh_to_x32(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "Description: Threshold value in number of clock cycles before the critical refresh or page timer expires. A critical refresh is to be issued before this threshold is reached. It is recommended that this not be changed from the default value, currently shown as 0x2. It must always be less than internally used t_rfc_nom_x32. Note that, in LPDDR2/LPDDR3, internally used t_rfc_nom_x32 may be equal to RFSHTMG.t_rfc_nom_x32>>2 if derating is enabled (DERATEEN.derate_enable=1). Otherwise, internally used t_rfc_nom_x32 will be equal to RFSHTMG.t_rfc_nom_x32. Unit: Multiples of 32 clocks. Value After Reset: 0x2 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_margin(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Threshold value in number of clock cycles before the critical refresh or page timer expires. A critical refresh is to be issued before this threshold is reached. It is recommended that this not be changed from the default value, currently shown as 0x2. It must always be less than internally used t_rfc_nom_x32. Note that, in LPDDR2/LPDDR3, internally used t_rfc_nom_x32 may be equal to RFSHTMG.t_rfc_nom_x32>>2 if derating is enabled (DERATEEN.derate_enable=1). Otherwise, internally used t_rfc_nom_x32 will be equal to RFSHTMG.t_rfc_nom_x32. Unit: Multiples of 32 clocks. Value After Reset: 0x2 Exists: Always."]
        #[inline(always)]
        pub const fn set_refresh_margin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Rfshctl0 {
        #[inline(always)]
        fn default() -> Rfshctl0 {
            Rfshctl0(0)
        }
    }
    impl core::fmt::Debug for Rfshctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfshctl0")
                .field("refresh_burst", &self.refresh_burst())
                .field("refresh_to_x32", &self.refresh_to_x32())
                .field("refresh_margin", &self.refresh_margin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfshctl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rfshctl0 {{ refresh_burst: {=u8:?}, refresh_to_x32: {=u8:?}, refresh_margin: {=u8:?} }}" , self . refresh_burst () , self . refresh_to_x32 () , self . refresh_margin ())
        }
    }
    #[doc = "Description: Refresh Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfshctl1(pub u32);
    impl Rfshctl1 {
        #[doc = "Description: Refresh timer start for rank 0 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_timer0_start_value_x32(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Refresh timer start for rank 0 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_refresh_timer0_start_value_x32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Description: Refresh timer start for rank 1 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_timer1_start_value_x32(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: Refresh timer start for rank 1 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1."]
        #[inline(always)]
        pub const fn set_refresh_timer1_start_value_x32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Rfshctl1 {
        #[inline(always)]
        fn default() -> Rfshctl1 {
            Rfshctl1(0)
        }
    }
    impl core::fmt::Debug for Rfshctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfshctl1")
                .field(
                    "refresh_timer0_start_value_x32",
                    &self.refresh_timer0_start_value_x32(),
                )
                .field(
                    "refresh_timer1_start_value_x32",
                    &self.refresh_timer1_start_value_x32(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfshctl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rfshctl1 {{ refresh_timer0_start_value_x32: {=u16:?}, refresh_timer1_start_value_x32: {=u16:?} }}" , self . refresh_timer0_start_value_x32 () , self . refresh_timer1_start_value_x32 ())
        }
    }
    #[doc = "Description: Refresh Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfshctl3(pub u32);
    impl Rfshctl3 {
        #[doc = "Description: When '1', disable auto-refresh generated by the uMCTL2. When auto-refresh is disabled, the SoC core must generate refreshes using the registers reg_ddrc_rank0_refresh, reg_ddrc_rank1_refresh, reg_ddrc_rank2_refresh and reg_ddrc_rank3_refresh. When dis_auto_refresh transitions from 0 to 1, any pending refreshes are immediately scheduled by the uMCTL2. If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), disable auto- refresh is not supported, and this bit must be set to '0'. This register field is changeable on the fly. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_auto_refresh(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: When '1', disable auto-refresh generated by the uMCTL2. When auto-refresh is disabled, the SoC core must generate refreshes using the registers reg_ddrc_rank0_refresh, reg_ddrc_rank1_refresh, reg_ddrc_rank2_refresh and reg_ddrc_rank3_refresh. When dis_auto_refresh transitions from 0 to 1, any pending refreshes are immediately scheduled by the uMCTL2. If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), disable auto- refresh is not supported, and this bit must be set to '0'. This register field is changeable on the fly. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_dis_auto_refresh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Toggle this signal (either from 0 to 1 or from 1 to 0) to indicate that the refresh register(s) have been updated. The value is automatically updated when exiting soft reset, so it does not need to be toggled initially. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_update_level(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Toggle this signal (either from 0 to 1 or from 1 to 0) to indicate that the refresh register(s) have been updated. The value is automatically updated when exiting soft reset, so it does not need to be toggled initially. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_refresh_update_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Rfshctl3 {
        #[inline(always)]
        fn default() -> Rfshctl3 {
            Rfshctl3(0)
        }
    }
    impl core::fmt::Debug for Rfshctl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfshctl3")
                .field("dis_auto_refresh", &self.dis_auto_refresh())
                .field("refresh_update_level", &self.refresh_update_level())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfshctl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfshctl3 {{ dis_auto_refresh: {=bool:?}, refresh_update_level: {=bool:?} }}",
                self.dis_auto_refresh(),
                self.refresh_update_level()
            )
        }
    }
    #[doc = "Description: Refresh Timing Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfshtmg(pub u32);
    impl Rfshtmg {
        #[doc = "Description: tRFC (min): Minimum time from refresh to refresh or activate. For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tRFCab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tRFCpb For configurations with MEMC_FREQ_RATIO=2, program this to tRFC(min)/2 and round up to next integer value. In DDR4 mode, tRFC(min) value is different depending on the refresh mode (fixed 1X,2X,4X) and the device density. The user should program the appropriate value from the spec based on the 'refresh_mode' and the device density that is used. Unit: Clocks. Value After Reset: 0x8c Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rfc_min(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Description: tRFC (min): Minimum time from refresh to refresh or activate. For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tRFCab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tRFCpb For configurations with MEMC_FREQ_RATIO=2, program this to tRFC(min)/2 and round up to next integer value. In DDR4 mode, tRFC(min) value is different depending on the refresh mode (fixed 1X,2X,4X) and the device density. The user should program the appropriate value from the spec based on the 'refresh_mode' and the device density that is used. Unit: Clocks. Value After Reset: 0x8c Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rfc_min(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Description: tREFI: Average time interval between refreshes per rank (specification: 7.8us for DDR2, DDR3 and DDR4. See JEDEC specification for mDDR, LPDDR2 and LPDDR3). For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tREFIab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tREFIpb For configurations with MEMC_FREQ_RATIO=2, program this to (tREFI/2), no rounding up. In DDR4 mode, tREFI value is different depending on the refresh mode. The user should program the appropriate value from the spec based on the value programmed in the refresh mode register. Note that RFSHTMG.t_rfc_nom_x32 * 32 must be greater than RFSHTMG.t_rfc_min. Unit: Multiples of 32 clocks. Value After Reset: 0x62 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rfc_nom_x32(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Description: tREFI: Average time interval between refreshes per rank (specification: 7.8us for DDR2, DDR3 and DDR4. See JEDEC specification for mDDR, LPDDR2 and LPDDR3). For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tREFIab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tREFIpb For configurations with MEMC_FREQ_RATIO=2, program this to (tREFI/2), no rounding up. In DDR4 mode, tREFI value is different depending on the refresh mode. The user should program the appropriate value from the spec based on the value programmed in the refresh mode register. Note that RFSHTMG.t_rfc_nom_x32 * 32 must be greater than RFSHTMG.t_rfc_min. Unit: Multiples of 32 clocks. Value After Reset: 0x62 Exists: Always."]
        #[inline(always)]
        pub const fn set_t_rfc_nom_x32(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Rfshtmg {
        #[inline(always)]
        fn default() -> Rfshtmg {
            Rfshtmg(0)
        }
    }
    impl core::fmt::Debug for Rfshtmg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfshtmg")
                .field("t_rfc_min", &self.t_rfc_min())
                .field("t_rfc_nom_x32", &self.t_rfc_nom_x32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfshtmg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfshtmg {{ t_rfc_min: {=u16:?}, t_rfc_nom_x32: {=u16:?} }}",
                self.t_rfc_min(),
                self.t_rfc_nom_x32()
            )
        }
    }
    #[doc = "Description: Scrubber Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbrctl(pub u32);
    impl Sbrctl {
        #[doc = "Description: Enable ECC scrubber. If set to 1, enables the scrubber to generate background read commands after the memories are initialized. If set to 0, disables the scrubber, resets the address generator to 0 and clears the scrubber status. This bitfield must be accessed separately from the other bitfields in this register. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Enable ECC scrubber. If set to 1, enables the scrubber to generate background read commands after the memories are initialized. If set to 0, disables the scrubber, resets the address generator to 0 and clears the scrubber status. This bitfield must be accessed separately from the other bitfields in this register. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Continue scrubbing during low power. If set to 1, burst of scrubs will be issued in HW controlled low power modes. There are two such modes: automatically initiated by idleness or initiated by HW low-power (LP) interface. If set to 0, the scrubber will not attempt to send commands while the DDRC is in HW controlled low power modes. In this case, the scrubber will remember the last address issued and will automatically continue from there when the DDRC exits the LP mode. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_during_lowpower(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Continue scrubbing during low power. If set to 1, burst of scrubs will be issued in HW controlled low power modes. There are two such modes: automatically initiated by idleness or initiated by HW low-power (LP) interface. If set to 0, the scrubber will not attempt to send commands while the DDRC is in HW controlled low power modes. In this case, the scrubber will remember the last address issued and will automatically continue from there when the DDRC exits the LP mode. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_during_lowpower(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: scrub_mode:0 ECC scrubber will perform reads scrub_mode:1 ECC scrubber will perform writes Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Description: scrub_mode:0 ECC scrubber will perform reads scrub_mode:1 ECC scrubber will perform writes Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Description: Scrub burst count. Determines the number of back-to-back scrub read commands that can be issued together when the controller is in one of the HW controlled low power modes. During low power, the period of the scrub burst becomes \\\"scrub_burst*scrub_interval\\\" cycles. During normal operation mode of the controller (ie. not in power-down or self refresh), scrub_burst is ignored and only one scrub command is generated. Valid values are: 1: 1 read, 2: 4 reads, 3: 16 reads, 4: 64 reads, 5: 256 reads, 6: 1024 reads. Value After Reset: 0x1 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_burst(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Description: Scrub burst count. Determines the number of back-to-back scrub read commands that can be issued together when the controller is in one of the HW controlled low power modes. During low power, the period of the scrub burst becomes \\\"scrub_burst*scrub_interval\\\" cycles. During normal operation mode of the controller (ie. not in power-down or self refresh), scrub_burst is ignored and only one scrub command is generated. Valid values are: 1: 1 read, 2: 4 reads, 3: 16 reads, 4: 64 reads, 5: 256 reads, 6: 1024 reads. Value After Reset: 0x1 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_burst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Description: Scrub interval. (512 x scrub_interval) number of clock cycles between two scrub read commands. If set to 0, scrub commands are issued back-to-back. This mode of operation (scrub_interval=0) can typically be used for scrubbing the full range of memory at once before or after SW controlled low power operations. After completing the full range of scrub while scrub_interval=0, scrub_done register is set and sbr_done_intr interrupt signal is asserted. Value After Reset: 0xff Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_interval(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x1fff;
            val as u16
        }
        #[doc = "Description: Scrub interval. (512 x scrub_interval) number of clock cycles between two scrub read commands. If set to 0, scrub commands are issued back-to-back. This mode of operation (scrub_interval=0) can typically be used for scrubbing the full range of memory at once before or after SW controlled low power operations. After completing the full range of scrub while scrub_interval=0, scrub_done register is set and sbr_done_intr interrupt signal is asserted. Value After Reset: 0xff Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_interval(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 8usize)) | (((val as u32) & 0x1fff) << 8usize);
        }
    }
    impl Default for Sbrctl {
        #[inline(always)]
        fn default() -> Sbrctl {
            Sbrctl(0)
        }
    }
    impl core::fmt::Debug for Sbrctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sbrctl")
                .field("scrub_en", &self.scrub_en())
                .field("scrub_during_lowpower", &self.scrub_during_lowpower())
                .field("scrub_mode", &self.scrub_mode())
                .field("scrub_burst", &self.scrub_burst())
                .field("scrub_interval", &self.scrub_interval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sbrctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sbrctl {{ scrub_en: {=bool:?}, scrub_during_lowpower: {=bool:?}, scrub_mode: {=bool:?}, scrub_burst: {=u8:?}, scrub_interval: {=u16:?} }}" , self . scrub_en () , self . scrub_during_lowpower () , self . scrub_mode () , self . scrub_burst () , self . scrub_interval ())
        }
    }
    #[doc = "Description: Scrubber Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbrstat(pub u32);
    impl Sbrstat {
        #[doc = "Description: Scrubber busy. Controller sets this bit to 1 when the scrubber logic has outstanding read commands being executed. Cleared when there are no active outstanding scrub reads in the system. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Scrubber busy. Controller sets this bit to 1 when the scrubber logic has outstanding read commands being executed. Cleared when there are no active outstanding scrub reads in the system. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: Scrubber done. Controller sets this bit to 1, after full range of addresses are scrubbed once while scrub_interval is set to 0. Cleared if scrub_en is set to 0 (scrubber disabled) or scrub_interval is set to a non-zero value for normal scrub operation. The interrupt signal, sbr_done_intr, is equivalent to this status bitfield. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Scrubber done. Controller sets this bit to 1, after full range of addresses are scrubbed once while scrub_interval is set to 0. Cleared if scrub_en is set to 0 (scrubber disabled) or scrub_interval is set to a non-zero value for normal scrub operation. The interrupt signal, sbr_done_intr, is equivalent to this status bitfield. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Sbrstat {
        #[inline(always)]
        fn default() -> Sbrstat {
            Sbrstat(0)
        }
    }
    impl core::fmt::Debug for Sbrstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sbrstat")
                .field("scrub_busy", &self.scrub_busy())
                .field("scrub_done", &self.scrub_done())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sbrstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sbrstat {{ scrub_busy: {=bool:?}, scrub_done: {=bool:?} }}",
                self.scrub_busy(),
                self.scrub_done()
            )
        }
    }
    #[doc = "Description: Scrubber Write Data Pattern0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbrwdata0(pub u32);
    impl Sbrwdata0 {
        #[doc = "Description: ECC Scrubber write data pattern for data bus\\[31:0\\]
Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[must_use]
        #[inline(always)]
        pub const fn scrub_pattern0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Description: ECC Scrubber write data pattern for data bus\\[31:0\\]
Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1."]
        #[inline(always)]
        pub const fn set_scrub_pattern0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sbrwdata0 {
        #[inline(always)]
        fn default() -> Sbrwdata0 {
            Sbrwdata0(0)
        }
    }
    impl core::fmt::Debug for Sbrwdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sbrwdata0")
                .field("scrub_pattern0", &self.scrub_pattern0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sbrwdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sbrwdata0 {{ scrub_pattern0: {=u32:?} }}",
                self.scrub_pattern0()
            )
        }
    }
    #[doc = "Description: Scheduler Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sched(pub u32);
    impl Sched {
        #[doc = "Description: Active low signal. When asserted ('0'), all incoming transactions are forced to low priority. This implies that all High Priority Read (HPR) and Variable Priority Read commands (VPR) will be treated as Low Priority Read (LPR) commands. On the write side, all Variable Priority Write (VPW) commands will be treated as Normal Priority Write (NPW) commands. Forcing the incoming transactions to low priority implicitly turns off Bypass path for read commands. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn force_low_pri_n(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: Active low signal. When asserted ('0'), all incoming transactions are forced to low priority. This implies that all High Priority Read (HPR) and Variable Priority Read commands (VPR) will be treated as Low Priority Read (LPR) commands. On the write side, all Variable Priority Write (VPW) commands will be treated as Normal Priority Write (NPW) commands. Forcing the incoming transactions to low priority implicitly turns off Bypass path for read commands. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_force_low_pri_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Description: If set then the bank selector prefers writes over reads. FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn prefer_write(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set then the bank selector prefers writes over reads. FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_prefer_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Description: If true, bank is kept open only until there are page hit transactions available in the CAM to that bank. The last read or write command in the CAM with a bank and page hit will be executed with auto-precharge if SCHED1.pageclose_timer=0. Even if this register set to 1 and SCHED1.pageclose_timer is set to 0, explicit precharge (and not auto-precharge) may be issued in some cases where there is a mode switch between Write and Read or between LPR and HPR. The Read and Write commands that are executed as part of the ECC scrub requests are also executed without auto-precharge. If false, the bank remains open until there is a need to close it (to open a different page, or for page timeout or refresh timeout) - also known as open page policy. The open page policy can be overridden by setting the per-command-autopre bit on the HIF interface (co_ih_rxcmd_autopre). The pageclose feature provids a midway between Open and Close page policies. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn pageclose(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If true, bank is kept open only until there are page hit transactions available in the CAM to that bank. The last read or write command in the CAM with a bank and page hit will be executed with auto-precharge if SCHED1.pageclose_timer=0. Even if this register set to 1 and SCHED1.pageclose_timer is set to 0, explicit precharge (and not auto-precharge) may be issued in some cases where there is a mode switch between Write and Read or between LPR and HPR. The Read and Write commands that are executed as part of the ECC scrub requests are also executed without auto-precharge. If false, the bank remains open until there is a need to close it (to open a different page, or for page timeout or refresh timeout) - also known as open page policy. The open page policy can be overridden by setting the per-command-autopre bit on the HIF interface (co_ih_rxcmd_autopre). The pageclose feature provids a midway between Open and Close page policies. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_pageclose(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Description: Number of entries in the low priority transaction store is this value + 1. (MEMC_NO_OF_ENTRY - (SCHED.lpr_num_entries + 1)) is the number of entries available for the high priority transaction store. Setting this to maximum value allocates all entries to low priority transaction store. Setting this to 0 allocates 1 entry to low priority transaction store and the rest to high priority transaction store. Note: In ECC configurations, the numbers of write and low priority read credits issued is one less than in the non-ECC case. One entry each is reserved in the write and low- priority read CAMs for storing the RMW requests arising out of single bit error correction RMW operation. Value After Reset: \"MEMC_NO_OF_ENTRY/2\" Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn lpr_num_entries(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Description: Number of entries in the low priority transaction store is this value + 1. (MEMC_NO_OF_ENTRY - (SCHED.lpr_num_entries + 1)) is the number of entries available for the high priority transaction store. Setting this to maximum value allocates all entries to low priority transaction store. Setting this to 0 allocates 1 entry to low priority transaction store and the rest to high priority transaction store. Note: In ECC configurations, the numbers of write and low priority read credits issued is one less than in the non-ECC case. One entry each is reserved in the write and low- priority read CAMs for storing the RMW requests arising out of single bit error correction RMW operation. Value After Reset: \"MEMC_NO_OF_ENTRY/2\" Exists: Always."]
        #[inline(always)]
        pub const fn set_lpr_num_entries(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Description: UNUSED Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn go2critical_hysteresis(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Description: UNUSED Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_go2critical_hysteresis(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Description: When the preferred transaction store is empty for these many clock cycles, switch to the alternate transaction store if it is non-empty. The read transaction store (both high and low priority) is the default preferred transaction store and the write transaction store is the alternative store. When prefer write over read is set this is reversed. 0x0 is a legal value for this register. When set to 0x0, the transaction store switching will happen immediately when the switching conditions become true. FOR PERFORMANCE ONLY Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn rdwr_idle_gap(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Description: When the preferred transaction store is empty for these many clock cycles, switch to the alternate transaction store if it is non-empty. The read transaction store (both high and low priority) is the default preferred transaction store and the write transaction store is the alternative store. When prefer write over read is set this is reversed. 0x0 is a legal value for this register. When set to 0x0, the transaction store switching will happen immediately when the switching conditions become true. FOR PERFORMANCE ONLY Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_rdwr_idle_gap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Sched {
        #[inline(always)]
        fn default() -> Sched {
            Sched(0)
        }
    }
    impl core::fmt::Debug for Sched {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sched")
                .field("force_low_pri_n", &self.force_low_pri_n())
                .field("prefer_write", &self.prefer_write())
                .field("pageclose", &self.pageclose())
                .field("lpr_num_entries", &self.lpr_num_entries())
                .field("go2critical_hysteresis", &self.go2critical_hysteresis())
                .field("rdwr_idle_gap", &self.rdwr_idle_gap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sched {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sched {{ force_low_pri_n: {=bool:?}, prefer_write: {=bool:?}, pageclose: {=bool:?}, lpr_num_entries: {=u8:?}, go2critical_hysteresis: {=u8:?}, rdwr_idle_gap: {=u8:?} }}" , self . force_low_pri_n () , self . prefer_write () , self . pageclose () , self . lpr_num_entries () , self . go2critical_hysteresis () , self . rdwr_idle_gap ())
        }
    }
    #[doc = "Description: Scheduler Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sched1(pub u32);
    impl Sched1 {
        #[doc = "Description: This field works in conjunction with SCHED.pageclose. It only has meaning if SCHED.pageclose==1. If SCHED.pageclose==1 and pageclose_timer==0, then an auto-precharge may be scheduled for last read or write command in the CAM with a bank and page hit. Note, sometimes an explicit precharge is scheduled instead of the auto-precharge. See SCHED.pageclose for details of when this may happen. If SCHED.pageclose==1 and pageclose_timer>0, then an auto-precharge is not scheduled for last read or write command in the CAM with a bank and page hit. Instead, a timer is started, with pageclose_timer as the initial value. There is a timer on a per bank basis. The timer decrements unless the next read or write in the CAM to a bank is a page hit. It gets reset to pageclose_timer value if the next read or write in the CAM to a bank is a page hit. Once the timer has reached zero, an explcit precharge will be attempted to be scheduled. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn pageclose_timer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Description: This field works in conjunction with SCHED.pageclose. It only has meaning if SCHED.pageclose==1. If SCHED.pageclose==1 and pageclose_timer==0, then an auto-precharge may be scheduled for last read or write command in the CAM with a bank and page hit. Note, sometimes an explicit precharge is scheduled instead of the auto-precharge. See SCHED.pageclose for details of when this may happen. If SCHED.pageclose==1 and pageclose_timer>0, then an auto-precharge is not scheduled for last read or write command in the CAM with a bank and page hit. Instead, a timer is started, with pageclose_timer as the initial value. There is a timer on a per bank basis. The timer decrements unless the next read or write in the CAM to a bank is a page hit. It gets reset to pageclose_timer value if the next read or write in the CAM to a bank is a page hit. Once the timer has reached zero, an explcit precharge will be attempted to be scheduled. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_pageclose_timer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Sched1 {
        #[inline(always)]
        fn default() -> Sched1 {
            Sched1(0)
        }
    }
    impl core::fmt::Debug for Sched1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sched1")
                .field("pageclose_timer", &self.pageclose_timer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sched1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sched1 {{ pageclose_timer: {=u8:?} }}",
                self.pageclose_timer()
            )
        }
    }
    #[doc = "Description: SAR Size Register n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Size(pub u32);
    impl Size {
        #[doc = "Description: Number of blocks for address region n. This register determines the total size of the region in multiples of minimum block size as specified by the hardware parameter UMCTL2_SARMINSIZE. The register value is encoded as number of blocks = nblocks + 1. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn nblocks(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Description: Number of blocks for address region n. This register determines the total size of the region in multiples of minimum block size as specified by the hardware parameter UMCTL2_SARMINSIZE. The register value is encoded as number of blocks = nblocks + 1. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_nblocks(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Size {
        #[inline(always)]
        fn default() -> Size {
            Size(0)
        }
    }
    impl core::fmt::Debug for Size {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Size")
                .field("nblocks", &self.nblocks())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Size {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Size {{ nblocks: {=u8:?} }}", self.nblocks())
        }
    }
    #[doc = "Description: Operating Mode Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "Description: Operating mode. This is 3-bits wide in configurations with mDDR/LPDDR2/LPDDR3/DDR4 support and 2-bits in all other configurations. non-mDDR/LPDDR2/LPDDR3 and non-DDR4 designs: 00 - Init 01 - Normal 10 - Power-down 11 - Self refresh mDDR/LPDDR2/LPDDR3 or DDR4 designs: 000 - Init 001 - Normal 010 - Power-down 011 - Self refresh 1XX - Deep power-down / Maximum Power Saving Mode Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn operating_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Description: Operating mode. This is 3-bits wide in configurations with mDDR/LPDDR2/LPDDR3/DDR4 support and 2-bits in all other configurations. non-mDDR/LPDDR2/LPDDR3 and non-DDR4 designs: 00 - Init 01 - Normal 10 - Power-down 11 - Self refresh mDDR/LPDDR2/LPDDR3 or DDR4 designs: 000 - Init 001 - Normal 010 - Power-down 011 - Self refresh 1XX - Deep power-down / Maximum Power Saving Mode Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_operating_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Description: Flags if Self Refresh is entered and if it was under Automatic Self Refresh control only or not. 00 - SDRAM is not in Self Refresh 11 - SDRAM is in Self Refresh and Self Refresh was caused by Automatic Self Refresh only 10 - SDRAM is in Self Refresh and Self Refresh was not caused solely under Automatic Self Refresh control. It could have been caused by Hardware Low Power Interface and/or Software (reg_ddrc_selfref_sw). Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn selfref_type(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Description: Flags if Self Refresh is entered and if it was under Automatic Self Refresh control only or not. 00 - SDRAM is not in Self Refresh 11 - SDRAM is in Self Refresh and Self Refresh was caused by Automatic Self Refresh only 10 - SDRAM is in Self Refresh and Self Refresh was not caused solely under Automatic Self Refresh control. It could have been caused by Hardware Low Power Interface and/or Software (reg_ddrc_selfref_sw). Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_selfref_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat")
                .field("operating_mode", &self.operating_mode())
                .field("selfref_type", &self.selfref_type())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Stat {{ operating_mode: {=u8:?}, selfref_type: {=u8:?} }}",
                self.operating_mode(),
                self.selfref_type()
            )
        }
    }
    #[doc = "Description: Port n Channel m Configuration ID Value Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Valuech(pub u32);
    impl Valuech {
        #[doc = "Description: Determines the value used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn id_value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Description: Determines the value used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_id_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Valuech {
        #[inline(always)]
        fn default() -> Valuech {
            Valuech(0)
        }
    }
    impl core::fmt::Debug for Valuech {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Valuech")
                .field("id_value", &self.id_value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Valuech {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Valuech {{ id_value: {=u32:?} }}", self.id_value())
        }
    }
    #[doc = "Description: Port n Configuration Write Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct W(pub u32);
    impl W {
        #[doc = "Description: Determines the initial load value of write aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the write aging counter sets the initial priority of the write channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level. For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (awqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_priority(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: Determines the initial load value of write aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the write aging counter sets the initial priority of the write channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level. For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (awqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_port_priority(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: If set to 1, enables aging function for the write channel of the port. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_aging_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables aging function for the write channel of the port. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_port_aging_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Description: If set to 1, enables the AXI urgent sideband signal (awurgent). When enabled and awurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_wr signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that awurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_urgent_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables the AXI urgent sideband signal (awurgent). When enabled and awurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_wr signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that awurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_port_urgent_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: 0x1 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_port_pagematch_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: 0x1 Exists: Always."]
        #[inline(always)]
        pub const fn set_wr_port_pagematch_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for W {
        #[inline(always)]
        fn default() -> W {
            W(0)
        }
    }
    impl core::fmt::Debug for W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("W")
                .field("wr_port_priority", &self.wr_port_priority())
                .field("wr_port_aging_en", &self.wr_port_aging_en())
                .field("wr_port_urgent_en", &self.wr_port_urgent_en())
                .field("wr_port_pagematch_en", &self.wr_port_pagematch_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "W {{ wr_port_priority: {=u16:?}, wr_port_aging_en: {=bool:?}, wr_port_urgent_en: {=bool:?}, wr_port_pagematch_en: {=bool:?} }}" , self . wr_port_priority () , self . wr_port_aging_en () , self . wr_port_urgent_en () , self . wr_port_pagematch_en ())
        }
    }
    #[doc = "Description: Port n Write QoS Configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wqos0(pub u32);
    impl Wqos0 {
        #[doc = "Description: Separation level indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 14 which corresponds to awqos. Note that for PA, awqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wqos_map_level(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Description: Separation level indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 14 which corresponds to awqos. Note that for PA, awqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wqos_map_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wqos_map_region0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wqos_map_region0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region 1 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wqos_map_region1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region 1 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wqos_map_region1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Wqos0 {
        #[inline(always)]
        fn default() -> Wqos0 {
            Wqos0(0)
        }
    }
    impl core::fmt::Debug for Wqos0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wqos0")
                .field("wqos_map_level", &self.wqos_map_level())
                .field("wqos_map_region0", &self.wqos_map_region0())
                .field("wqos_map_region1", &self.wqos_map_region1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wqos0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wqos0 {{ wqos_map_level: {=u8:?}, wqos_map_region0: {=u8:?}, wqos_map_region1: {=u8:?} }}" , self . wqos_map_level () , self . wqos_map_region0 () , self . wqos_map_region1 ())
        }
    }
    #[doc = "Description: Port n Write QoS Configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wqos1(pub u32);
    impl Wqos1 {
        #[doc = "Description: Specifies the timeout value for write transactions. Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn wqos_map_timeout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Description: Specifies the timeout value for write transactions. Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_wqos_map_timeout(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Wqos1 {
        #[inline(always)]
        fn default() -> Wqos1 {
            Wqos1(0)
        }
    }
    impl core::fmt::Debug for Wqos1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wqos1")
                .field("wqos_map_timeout", &self.wqos_map_timeout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wqos1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wqos1 {{ wqos_map_timeout: {=u16:?} }}",
                self.wqos_map_timeout()
            )
        }
    }
    #[doc = "Description: ZQ Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zqctl0(pub u32);
    impl Zqctl0 {
        #[doc = "Description: tZQCS: Number of cycles of NOP required after a ZQCS (ZQ calibration short) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2, program this to tZQCS/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x40 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_zq_short_nop(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: tZQCS: Number of cycles of NOP required after a ZQCS (ZQ calibration short) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2, program this to tZQCS/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x40 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_t_zq_short_nop(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Description: tZQoper for DDR3/DDR4, tZQCL for LPDDR2/LPDDR3: Number of cycles of NOP required after a ZQCL (ZQ calibration long) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2: DDR3/DDR4: program this to tZQoper/2 and round it up to the next integer value. LPDDR2/LPDDR3: program this to tZQCL/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x200 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_zq_long_nop(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Description: tZQoper for DDR3/DDR4, tZQCL for LPDDR2/LPDDR3: Number of cycles of NOP required after a ZQCL (ZQ calibration long) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2: DDR3/DDR4: program this to tZQoper/2 and round it up to the next integer value. LPDDR2/LPDDR3: program this to tZQCL/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x200 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_t_zq_long_nop(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Description: 1 - Denotes that ZQ resistor is shared between ranks. Means ZQinit/ZQCL/ZQCS commands are sent to one rank at a time with tZQinit/tZQCL/tZQCS timing met between commands so that commands to different ranks do not overlap. 0 - ZQ resistor is not shared. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn zq_resistor_shared(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Description: 1 - Denotes that ZQ resistor is shared between ranks. Means ZQinit/ZQCL/ZQCS commands are sent to one rank at a time with tZQinit/tZQCL/tZQCS timing met between commands so that commands to different ranks do not overlap. 0 - ZQ resistor is not shared. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_zq_resistor_shared(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Description: 1 - Disable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. 0 - Enable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_srx_zqcl(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Description: 1 - Disable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. 0 - Enable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_dis_srx_zqcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Description: 1 - Disable uMCTL2 generation of ZQCS command. Register reg_ddrc_zq_calib_short can be used instead to control ZQ calibration commands. 0 - Internally generate ZQCS commands based on ZQCTL1.t_zq_short_interval_x1024. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_auto_zq(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Description: 1 - Disable uMCTL2 generation of ZQCS command. Register reg_ddrc_zq_calib_short can be used instead to control ZQ calibration commands. 0 - Internally generate ZQCS commands based on ZQCTL1.t_zq_short_interval_x1024. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_dis_auto_zq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Zqctl0 {
        #[inline(always)]
        fn default() -> Zqctl0 {
            Zqctl0(0)
        }
    }
    impl core::fmt::Debug for Zqctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Zqctl0")
                .field("t_zq_short_nop", &self.t_zq_short_nop())
                .field("t_zq_long_nop", &self.t_zq_long_nop())
                .field("zq_resistor_shared", &self.zq_resistor_shared())
                .field("dis_srx_zqcl", &self.dis_srx_zqcl())
                .field("dis_auto_zq", &self.dis_auto_zq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Zqctl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Zqctl0 {{ t_zq_short_nop: {=u16:?}, t_zq_long_nop: {=u16:?}, zq_resistor_shared: {=bool:?}, dis_srx_zqcl: {=bool:?}, dis_auto_zq: {=bool:?} }}" , self . t_zq_short_nop () , self . t_zq_long_nop () , self . zq_resistor_shared () , self . dis_srx_zqcl () , self . dis_auto_zq ())
        }
    }
    #[doc = "Description: ZQ Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zqctl1(pub u32);
    impl Zqctl1 {
        #[doc = "Description: Average interval to wait between automatically issuing ZQCS (ZQ calibration short) commands to DDR3/DDR4/LPDDR2/LPDDR3 devices. Meaningless, if ZQCTL0.dis_auto_zq=1. Unit: 1024 clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x100 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_zq_short_interval_x1024(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Description: Average interval to wait between automatically issuing ZQCS (ZQ calibration short) commands to DDR3/DDR4/LPDDR2/LPDDR3 devices. Meaningless, if ZQCTL0.dis_auto_zq=1. Unit: 1024 clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x100 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1."]
        #[inline(always)]
        pub const fn set_t_zq_short_interval_x1024(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Zqctl1 {
        #[inline(always)]
        fn default() -> Zqctl1 {
            Zqctl1(0)
        }
    }
    impl core::fmt::Debug for Zqctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Zqctl1")
                .field(
                    "t_zq_short_interval_x1024",
                    &self.t_zq_short_interval_x1024(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Zqctl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Zqctl1 {{ t_zq_short_interval_x1024: {=u32:?} }}",
                self.t_zq_short_interval_x1024()
            )
        }
    }
    #[doc = "Description: ZQ Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zqstat(pub u32);
    impl Zqstat {
        #[doc = "Description: SoC core may initiate a ZQ Reset operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQ Reset request. It goes low when the ZQ Reset command is issued to the SDRAM and the associated NOP period is over. It is recommended not to perform ZQ Reset commands when this signal is high. 0 - Indicates that the SoC core can initiate a ZQ Reset operation 1 - Indicates that ZQ Reset operation is in progress Value After Reset: 0x0 Exists: Always."]
        #[must_use]
        #[inline(always)]
        pub const fn zq_reset_busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Description: SoC core may initiate a ZQ Reset operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQ Reset request. It goes low when the ZQ Reset command is issued to the SDRAM and the associated NOP period is over. It is recommended not to perform ZQ Reset commands when this signal is high. 0 - Indicates that the SoC core can initiate a ZQ Reset operation 1 - Indicates that ZQ Reset operation is in progress Value After Reset: 0x0 Exists: Always."]
        #[inline(always)]
        pub const fn set_zq_reset_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Zqstat {
        #[inline(always)]
        fn default() -> Zqstat {
            Zqstat(0)
        }
    }
    impl core::fmt::Debug for Zqstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Zqstat")
                .field("zq_reset_busy", &self.zq_reset_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Zqstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Zqstat {{ zq_reset_busy: {=bool:?} }}",
                self.zq_reset_busy()
            )
        }
    }
}
