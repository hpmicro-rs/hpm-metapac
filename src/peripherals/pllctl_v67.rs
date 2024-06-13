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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PLLx config1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PLLx config2."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PLLx frac mode frequency adjust."]
    #[inline(always)]
    pub const fn freq(self) -> crate::common::Reg<regs::Freq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PLLx lock control."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PLLx status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PLLx divider0 control."]
    #[inline(always)]
    pub const fn div0(self) -> crate::common::Reg<regs::Div0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "PLLx divider1 control."]
    #[inline(always)]
    pub const fn div1(self) -> crate::common::Reg<regs::Div1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pll(self, n: usize) -> Pll {
        assert!(n < 5usize);
        unsafe { Pll::from_ptr(self.ptr.add(0x80usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "PLLx config0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "1: int mode; 0: frac mode."]
        #[inline(always)]
        pub const fn dsmpd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: int mode; 0: frac mode."]
        #[inline(always)]
        pub fn set_dsmpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ss_disable_sscg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ss_disable_sscg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ss_reset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ss_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread."]
        #[inline(always)]
        pub const fn ss_downspread(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread."]
        #[inline(always)]
        pub fn set_ss_downspread(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn ss_divval(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd."]
        #[inline(always)]
        pub fn set_ss_divval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn ss_spread(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x1f;
            val as u8
        }
        #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd."]
        #[inline(always)]
        pub fn set_ss_spread(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
        }
        #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn postdiv1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd."]
        #[inline(always)]
        pub fn set_postdiv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd."]
        #[inline(always)]
        pub const fn refdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd."]
        #[inline(always)]
        pub fn set_refdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb."]
        #[inline(always)]
        pub const fn ss_rstptr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb."]
        #[inline(always)]
        pub fn set_ss_rstptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    #[doc = "PLLx config1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3."]
        #[inline(always)]
        pub const fn lock_cnt_cfg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3."]
        #[inline(always)]
        pub fn set_lock_cnt_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence."]
        #[inline(always)]
        pub const fn pllpd_sw(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence."]
        #[inline(always)]
        pub fn set_pllpd_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;."]
        #[inline(always)]
        pub const fn clken_sw(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;."]
        #[inline(always)]
        pub fn set_clken_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings."]
        #[inline(always)]
        pub const fn pllctrl_hw_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings."]
        #[inline(always)]
        pub fn set_pllctrl_hw_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    #[doc = "PLLx config2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "fbdiv used in int mode."]
        #[inline(always)]
        pub const fn fbdiv_int(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "fbdiv used in int mode."]
        #[inline(always)]
        pub fn set_fbdiv_int(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    #[doc = "PLLx divider0 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div0(pub u32);
    impl Div0 {
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Div0 {
        #[inline(always)]
        fn default() -> Div0 {
            Div0(0)
        }
    }
    #[doc = "PLLx divider1 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div1(pub u32);
    impl Div1 {
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag 0: divider is working 1: divider is changing status."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Div1 {
        #[inline(always)]
        fn default() -> Div1 {
            Div1(0)
        }
    }
    #[doc = "PLLx frac mode frequency adjust."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Freq(pub u32);
    impl Freq {
        #[doc = "fbdiv used in frac mode."]
        #[inline(always)]
        pub const fn fbdiv_frac(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "fbdiv used in frac mode."]
        #[inline(always)]
        pub fn set_fbdiv_frac(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)."]
        #[inline(always)]
        pub const fn frac(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)."]
        #[inline(always)]
        pub fn set_frac(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Freq {
        #[inline(always)]
        fn default() -> Freq {
            Freq(0)
        }
    }
    #[doc = "PLLx lock control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn lock_ss_divval(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub fn set_lock_ss_divval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn lock_ss_spead(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub fn set_lock_ss_spead(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn lock_postdiv1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub fn set_lock_postdiv1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn lock_refdiv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub fn set_lock_refdiv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub const fn lock_ss_rstptr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable."]
        #[inline(always)]
        pub fn set_lock_ss_rstptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    #[doc = "PLLx status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pll_lock_sync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pll_lock_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pll_lock_comb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pll_lock_comb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable from SYSCTL block."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "enable from SYSCTL block."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "Crystal control and status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal(pub u32);
    impl Xtal {
        #[doc = "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[inline(always)]
        pub const fn ramp_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles."]
        #[inline(always)]
        pub fn set_ramp_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub const fn response(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use."]
        #[inline(always)]
        pub fn set_response(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Xtal {
        #[inline(always)]
        fn default() -> Xtal {
            Xtal(0)
        }
    }
}
