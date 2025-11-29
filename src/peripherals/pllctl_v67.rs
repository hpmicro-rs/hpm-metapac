#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll {
    ptr: *mut u8,
}
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PLLx config0."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PLLx config1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PLLx config2."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PLLx frac mode frequency adjust."]
    #[inline(always)]
    pub const fn freq(self) -> crate::common::Reg<regs::Freq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PLLx lock control."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PLLx status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PLLx divider0 control."]
    #[inline(always)]
    pub const fn div0(self) -> crate::common::Reg<regs::Div0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PLLx divider1 control."]
    #[inline(always)]
    pub const fn div1(self) -> crate::common::Reg<regs::Div1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
#[doc = "PLLCTL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllctl {
    ptr: *mut u8,
}
unsafe impl Send for Pllctl {}
unsafe impl Sync for Pllctl {}
impl Pllctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crystal control and status."]
    #[inline(always)]
    pub const fn xtal(self) -> crate::common::Reg<regs::Xtal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pll(self, n: usize) -> Pll {
        assert!(n < 5usize);
        unsafe { Pll::from_ptr(self.ptr.wrapping_add(0x80usize + n * 128usize) as _) }
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
    #[doc = "PLLx config0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "1: int mode; 0: frac mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dsmpd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: int mode; 0: frac mode."]
        #[inline(always)]
        pub const fn set_dsmpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_disable_sscg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ss_disable_sscg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_reset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ss_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_downspread(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread."]
        #[inline(always)]
        pub const fn set_ss_downspread(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_divval(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn set_ss_divval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_spread(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x1f;
            val as u8
        }
        #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn set_ss_spread(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
        }
        #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd."]
        #[must_use]
        #[inline(always)]
        pub const fn postdiv1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn set_postdiv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd."]
        #[must_use]
        #[inline(always)]
        pub const fn refdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn set_refdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb."]
        #[must_use]
        #[inline(always)]
        pub const fn ss_rstptr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb."]
        #[inline(always)]
        pub const fn set_ss_rstptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    impl core::fmt::Debug for Cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg0")
                .field("dsmpd", &self.dsmpd())
                .field("ss_disable_sscg", &self.ss_disable_sscg())
                .field("ss_reset", &self.ss_reset())
                .field("ss_downspread", &self.ss_downspread())
                .field("ss_divval", &self.ss_divval())
                .field("ss_spread", &self.ss_spread())
                .field("postdiv1", &self.postdiv1())
                .field("refdiv", &self.refdiv())
                .field("ss_rstptr", &self.ss_rstptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg0 {{ dsmpd: {=bool:?}, ss_disable_sscg: {=bool:?}, ss_reset: {=bool:?}, ss_downspread: {=bool:?}, ss_divval: {=u8:?}, ss_spread: {=u8:?}, postdiv1: {=u8:?}, refdiv: {=u8:?}, ss_rstptr: {=bool:?} }}" , self . dsmpd () , self . ss_disable_sscg () , self . ss_reset () , self . ss_downspread () , self . ss_divval () , self . ss_spread () , self . postdiv1 () , self . refdiv () , self . ss_rstptr ())
        }
    }
    #[doc = "PLLx config1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_cnt_cfg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3."]
        #[inline(always)]
        pub const fn set_lock_cnt_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn pllpd_sw(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence."]
        #[inline(always)]
        pub const fn set_pllpd_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;."]
        #[must_use]
        #[inline(always)]
        pub const fn clken_sw(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;."]
        #[inline(always)]
        pub const fn set_clken_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings."]
        #[must_use]
        #[inline(always)]
        pub const fn pllctrl_hw_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings."]
        #[inline(always)]
        pub const fn set_pllctrl_hw_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("lock_cnt_cfg", &self.lock_cnt_cfg())
                .field("pllpd_sw", &self.pllpd_sw())
                .field("clken_sw", &self.clken_sw())
                .field("pllctrl_hw_en", &self.pllctrl_hw_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg1 {{ lock_cnt_cfg: {=bool:?}, pllpd_sw: {=bool:?}, clken_sw: {=bool:?}, pllctrl_hw_en: {=bool:?} }}" , self . lock_cnt_cfg () , self . pllpd_sw () , self . clken_sw () , self . pllctrl_hw_en ())
        }
    }
    #[doc = "PLLx config2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "fbdiv used in int mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fbdiv_int(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "fbdiv used in int mode."]
        #[inline(always)]
        pub const fn set_fbdiv_int(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    impl core::fmt::Debug for Cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg2")
                .field("fbdiv_int", &self.fbdiv_int())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfg2 {{ fbdiv_int: {=u16:?} }}", self.fbdiv_int())
        }
    }
    #[doc = "PLLx divider0 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div0(pub u32);
    impl Div0 {
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[must_use]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Div0 {
        #[inline(always)]
        fn default() -> Div0 {
            Div0(0)
        }
    }
    impl core::fmt::Debug for Div0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Div0")
                .field("div", &self.div())
                .field("enable", &self.enable())
                .field("response", &self.response())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Div0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Div0 {{ div: {=u8:?}, enable: {=bool:?}, response: {=bool:?}, busy: {=bool:?} }}",
                self.div(),
                self.enable(),
                self.response(),
                self.busy()
            )
        }
    }
    #[doc = "PLLx divider1 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div1(pub u32);
    impl Div1 {
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[must_use]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Div1 {
        #[inline(always)]
        fn default() -> Div1 {
            Div1(0)
        }
    }
    impl core::fmt::Debug for Div1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Div1")
                .field("div", &self.div())
                .field("enable", &self.enable())
                .field("response", &self.response())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Div1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Div1 {{ div: {=u8:?}, enable: {=bool:?}, response: {=bool:?}, busy: {=bool:?} }}",
                self.div(),
                self.enable(),
                self.response(),
                self.busy()
            )
        }
    }
    #[doc = "PLLx frac mode frequency adjust."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Freq(pub u32);
    impl Freq {
        #[doc = "fbdiv used in frac mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fbdiv_frac(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "fbdiv used in frac mode."]
        #[inline(always)]
        pub const fn set_fbdiv_frac(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)."]
        #[must_use]
        #[inline(always)]
        pub const fn frac(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)."]
        #[inline(always)]
        pub const fn set_frac(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Freq {
        #[inline(always)]
        fn default() -> Freq {
            Freq(0)
        }
    }
    impl core::fmt::Debug for Freq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Freq")
                .field("fbdiv_frac", &self.fbdiv_frac())
                .field("frac", &self.frac())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Freq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Freq {{ fbdiv_frac: {=u8:?}, frac: {=u32:?} }}",
                self.fbdiv_frac(),
                self.frac()
            )
        }
    }
    #[doc = "PLLx lock control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_ss_divval(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn set_lock_ss_divval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_ss_spead(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn set_lock_ss_spead(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_postdiv1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn set_lock_postdiv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_refdiv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn set_lock_refdiv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_ss_rstptr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn set_lock_ss_rstptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    impl core::fmt::Debug for Lock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lock")
                .field("lock_ss_divval", &self.lock_ss_divval())
                .field("lock_ss_spead", &self.lock_ss_spead())
                .field("lock_postdiv1", &self.lock_postdiv1())
                .field("lock_refdiv", &self.lock_refdiv())
                .field("lock_ss_rstptr", &self.lock_ss_rstptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lock {{ lock_ss_divval: {=bool:?}, lock_ss_spead: {=bool:?}, lock_postdiv1: {=bool:?}, lock_refdiv: {=bool:?}, lock_ss_rstptr: {=bool:?} }}" , self . lock_ss_divval () , self . lock_ss_spead () , self . lock_postdiv1 () , self . lock_refdiv () , self . lock_ss_rstptr ())
        }
    }
    #[doc = "PLLx status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_lock_sync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pll_lock_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_lock_comb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pll_lock_comb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
        #[must_use]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
        #[inline(always)]
        pub const fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable from SYSCTL block."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "enable from SYSCTL block."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("pll_lock_sync", &self.pll_lock_sync())
                .field("pll_lock_comb", &self.pll_lock_comb())
                .field("response", &self.response())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ pll_lock_sync: {=bool:?}, pll_lock_comb: {=bool:?}, response: {=bool:?}, enable: {=bool:?} }}" , self . pll_lock_sync () , self . pll_lock_comb () , self . response () , self . enable ())
        }
    }
    #[doc = "Crystal control and status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal(pub u32);
    impl Xtal {
        #[doc = "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ramp_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[inline(always)]
        pub const fn set_ramp_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[must_use]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Xtal {
        #[inline(always)]
        fn default() -> Xtal {
            Xtal(0)
        }
    }
    impl core::fmt::Debug for Xtal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtal")
                .field("ramp_time", &self.ramp_time())
                .field("enable", &self.enable())
                .field("response", &self.response())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xtal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xtal {{ ramp_time: {=u32:?}, enable: {=bool:?}, response: {=bool:?} }}",
                self.ramp_time(),
                self.enable(),
                self.response()
            )
        }
    }
}
