#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CONCTL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conctl {
    ptr: *mut u8,
}
unsafe impl Send for Conctl {}
unsafe impl Sync for Conctl {}
impl Conctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl4(self) -> crate::common::Reg<regs::Ctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl5(self) -> crate::common::Reg<regs::Ctrl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet0_txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet0_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet0_rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet0_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl2(pub u32);
    impl Ctrl2 {
        #[doc = "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;."]
        #[inline(always)]
        pub const fn enet0_rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;."]
        #[inline(always)]
        pub fn set_enet0_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet0_flowctrl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet0_flowctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "000:Reserved 001:RGMII 100:RMII 111:Reserved."]
        #[inline(always)]
        pub const fn enet0_phy_intf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "000:Reserved 001:RGMII 100:RMII 111:Reserved."]
        #[inline(always)]
        pub fn set_enet0_phy_intf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet0_refclk_oe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet0_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ENET0 LPI IRQ Enable."]
        #[inline(always)]
        pub const fn enet0_lpi_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENET0 LPI IRQ Enable."]
        #[inline(always)]
        pub fn set_enet0_lpi_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl2 {
        #[inline(always)]
        fn default() -> Ctrl2 {
            Ctrl2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl3(pub u32);
    impl Ctrl3 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_flowctrl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_flowctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_phy_intf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_phy_intf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn enet1_refclk_oe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_enet1_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ENET1 LPI Interrupt Enable."]
        #[inline(always)]
        pub const fn enet1_lpi_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENET1 LPI Interrupt Enable."]
        #[inline(always)]
        pub fn set_enet1_lpi_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl3 {
        #[inline(always)]
        fn default() -> Ctrl3 {
            Ctrl3(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl4(pub u32);
    impl Ctrl4 {
        #[doc = "force use sw DLL config."]
        #[inline(always)]
        pub const fn sdxc0_gpr_cclk_rx_dly_sw_force(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "force use sw DLL config."]
        #[inline(always)]
        pub fn set_sdxc0_gpr_cclk_rx_dly_sw_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc0_gpr_cclk_rx_dly_sw_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc0_gpr_cclk_rx_dly_sw_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "enable strobe clock, maybe used when update strobe DLL."]
        #[inline(always)]
        pub const fn sdxc0_gpr_strobe_in_enable(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "enable strobe clock, maybe used when update strobe DLL."]
        #[inline(always)]
        pub fn set_sdxc0_gpr_strobe_in_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "for strobe DLL, default 7taps(1ns)."]
        #[inline(always)]
        pub const fn sdxc0_gpr_tuning_strobe_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "for strobe DLL, default 7taps(1ns)."]
        #[inline(always)]
        pub fn set_sdxc0_gpr_tuning_strobe_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "for card clock DLL, default 0."]
        #[inline(always)]
        pub const fn sdxc0_gpr_tuning_card_clk_sel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x1f;
            val as u8
        }
        #[doc = "for card clock DLL, default 0."]
        #[inline(always)]
        pub fn set_sdxc0_gpr_tuning_card_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub const fn sdxc0_cardclk_inv_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub fn set_sdxc0_cardclk_inv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub const fn sdxc0_wkp_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub fn set_sdxc0_wkp_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub const fn sdxc0_sys_irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub fn set_sdxc0_sys_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl4 {
        #[inline(always)]
        fn default() -> Ctrl4 {
            Ctrl4(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl5(pub u32);
    impl Ctrl5 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc1_gpr_cclk_rx_dly_sw_force(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc1_gpr_cclk_rx_dly_sw_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc1_gpr_cclk_rx_dly_sw_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc1_gpr_cclk_rx_dly_sw_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc1_gpr_strobe_in_enable(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc1_gpr_strobe_in_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc1_gpr_tuning_strobe_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc1_gpr_tuning_strobe_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sdxc1_gpr_tuning_card_clk_sel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sdxc1_gpr_tuning_card_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub const fn sdxc1_cardclk_inv_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub fn set_sdxc1_cardclk_inv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub const fn sdxc1_wkp_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub fn set_sdxc1_wkp_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub const fn sdxc1_sys_irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub fn set_sdxc1_sys_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl5 {
        #[inline(always)]
        fn default() -> Ctrl5 {
            Ctrl5(0)
        }
    }
}
