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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl4(self) -> crate::common::Reg<regs::Ctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl5(self) -> crate::common::Reg<regs::Ctrl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
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
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    impl core::fmt::Debug for Ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl0")
                .field("enet0_txclk_dly_sel", &self.enet0_txclk_dly_sel())
                .field("enet0_rxclk_dly_sel", &self.enet0_rxclk_dly_sel())
                .field("enet1_txclk_dly_sel", &self.enet1_txclk_dly_sel())
                .field("enet1_rxclk_dly_sel", &self.enet1_rxclk_dly_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl0 {{ enet0_txclk_dly_sel: {=u8:?}, enet0_rxclk_dly_sel: {=u8:?}, enet1_txclk_dly_sel: {=u8:?}, enet1_rxclk_dly_sel: {=u8:?} }}" , self . enet0_txclk_dly_sel () , self . enet0_rxclk_dly_sel () , self . enet1_txclk_dly_sel () , self . enet1_rxclk_dly_sel ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl2(pub u32);
    impl Ctrl2 {
        #[doc = "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;."]
        #[inline(always)]
        pub const fn set_enet0_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_flowctrl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_flowctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "000:Reserved 001:RGMII 100:RMII 111:Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_phy_intf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "000:Reserved 001:RGMII 100:RMII 111:Reserved."]
        #[inline(always)]
        pub const fn set_enet0_phy_intf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_refclk_oe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet0_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ENET0 LPI IRQ Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enet0_lpi_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENET0 LPI IRQ Enable."]
        #[inline(always)]
        pub const fn set_enet0_lpi_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl2 {
        #[inline(always)]
        fn default() -> Ctrl2 {
            Ctrl2(0)
        }
    }
    impl core::fmt::Debug for Ctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl2")
                .field("enet0_rmii_txclk_sel", &self.enet0_rmii_txclk_sel())
                .field("enet0_flowctrl", &self.enet0_flowctrl())
                .field("enet0_phy_intf_sel", &self.enet0_phy_intf_sel())
                .field("enet0_refclk_oe", &self.enet0_refclk_oe())
                .field("enet0_lpi_irq_en", &self.enet0_lpi_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl2 {{ enet0_rmii_txclk_sel: {=bool:?}, enet0_flowctrl: {=bool:?}, enet0_phy_intf_sel: {=u8:?}, enet0_refclk_oe: {=bool:?}, enet0_lpi_irq_en: {=bool:?} }}" , self . enet0_rmii_txclk_sel () , self . enet0_flowctrl () , self . enet0_phy_intf_sel () , self . enet0_refclk_oe () , self . enet0_lpi_irq_en ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl3(pub u32);
    impl Ctrl3 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_flowctrl(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_flowctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_phy_intf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_phy_intf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_refclk_oe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enet1_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ENET1 LPI Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enet1_lpi_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENET1 LPI Interrupt Enable."]
        #[inline(always)]
        pub const fn set_enet1_lpi_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl3 {
        #[inline(always)]
        fn default() -> Ctrl3 {
            Ctrl3(0)
        }
    }
    impl core::fmt::Debug for Ctrl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl3")
                .field("enet1_rmii_txclk_sel", &self.enet1_rmii_txclk_sel())
                .field("enet1_flowctrl", &self.enet1_flowctrl())
                .field("enet1_phy_intf_sel", &self.enet1_phy_intf_sel())
                .field("enet1_refclk_oe", &self.enet1_refclk_oe())
                .field("enet1_lpi_irq_en", &self.enet1_lpi_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl3 {{ enet1_rmii_txclk_sel: {=bool:?}, enet1_flowctrl: {=bool:?}, enet1_phy_intf_sel: {=u8:?}, enet1_refclk_oe: {=bool:?}, enet1_lpi_irq_en: {=bool:?} }}" , self . enet1_rmii_txclk_sel () , self . enet1_flowctrl () , self . enet1_phy_intf_sel () , self . enet1_refclk_oe () , self . enet1_lpi_irq_en ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl4(pub u32);
    impl Ctrl4 {
        #[doc = "force use sw DLL config."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_gpr_cclk_rx_dly_sw_force(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "force use sw DLL config."]
        #[inline(always)]
        pub const fn set_sdxc0_gpr_cclk_rx_dly_sw_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_gpr_cclk_rx_dly_sw_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc0_gpr_cclk_rx_dly_sw_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "enable strobe clock, maybe used when update strobe DLL."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_gpr_strobe_in_enable(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "enable strobe clock, maybe used when update strobe DLL."]
        #[inline(always)]
        pub const fn set_sdxc0_gpr_strobe_in_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "for strobe DLL, default 7taps(1ns)."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_gpr_tuning_strobe_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "for strobe DLL, default 7taps(1ns)."]
        #[inline(always)]
        pub const fn set_sdxc0_gpr_tuning_strobe_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "for card clock DLL, default 0."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_gpr_tuning_card_clk_sel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x1f;
            val as u8
        }
        #[doc = "for card clock DLL, default 0."]
        #[inline(always)]
        pub const fn set_sdxc0_gpr_tuning_card_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
        }
        #[doc = "card clock inverter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_cardclk_inv_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub const fn set_sdxc0_cardclk_inv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wakeup irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_wkp_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub const fn set_sdxc0_wkp_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "system irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc0_sys_irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub const fn set_sdxc0_sys_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl4 {
        #[inline(always)]
        fn default() -> Ctrl4 {
            Ctrl4(0)
        }
    }
    impl core::fmt::Debug for Ctrl4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl4")
                .field(
                    "sdxc0_gpr_cclk_rx_dly_sw_force",
                    &self.sdxc0_gpr_cclk_rx_dly_sw_force(),
                )
                .field(
                    "sdxc0_gpr_cclk_rx_dly_sw_sel",
                    &self.sdxc0_gpr_cclk_rx_dly_sw_sel(),
                )
                .field(
                    "sdxc0_gpr_strobe_in_enable",
                    &self.sdxc0_gpr_strobe_in_enable(),
                )
                .field(
                    "sdxc0_gpr_tuning_strobe_sel",
                    &self.sdxc0_gpr_tuning_strobe_sel(),
                )
                .field(
                    "sdxc0_gpr_tuning_card_clk_sel",
                    &self.sdxc0_gpr_tuning_card_clk_sel(),
                )
                .field("sdxc0_cardclk_inv_en", &self.sdxc0_cardclk_inv_en())
                .field("sdxc0_wkp_irq_en", &self.sdxc0_wkp_irq_en())
                .field("sdxc0_sys_irq_en", &self.sdxc0_sys_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl4 {{ sdxc0_gpr_cclk_rx_dly_sw_force: {=bool:?}, sdxc0_gpr_cclk_rx_dly_sw_sel: {=u8:?}, sdxc0_gpr_strobe_in_enable: {=bool:?}, sdxc0_gpr_tuning_strobe_sel: {=u8:?}, sdxc0_gpr_tuning_card_clk_sel: {=u8:?}, sdxc0_cardclk_inv_en: {=bool:?}, sdxc0_wkp_irq_en: {=bool:?}, sdxc0_sys_irq_en: {=bool:?} }}" , self . sdxc0_gpr_cclk_rx_dly_sw_force () , self . sdxc0_gpr_cclk_rx_dly_sw_sel () , self . sdxc0_gpr_strobe_in_enable () , self . sdxc0_gpr_tuning_strobe_sel () , self . sdxc0_gpr_tuning_card_clk_sel () , self . sdxc0_cardclk_inv_en () , self . sdxc0_wkp_irq_en () , self . sdxc0_sys_irq_en ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl5(pub u32);
    impl Ctrl5 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_gpr_cclk_rx_dly_sw_force(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc1_gpr_cclk_rx_dly_sw_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_gpr_cclk_rx_dly_sw_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc1_gpr_cclk_rx_dly_sw_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_gpr_strobe_in_enable(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc1_gpr_strobe_in_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_gpr_tuning_strobe_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc1_gpr_tuning_strobe_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_gpr_tuning_card_clk_sel(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sdxc1_gpr_tuning_card_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
        }
        #[doc = "card clock inverter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_cardclk_inv_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "card clock inverter enable."]
        #[inline(always)]
        pub const fn set_sdxc1_cardclk_inv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wakeup irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_wkp_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup irq enable."]
        #[inline(always)]
        pub const fn set_sdxc1_wkp_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "system irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdxc1_sys_irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "system irq enable."]
        #[inline(always)]
        pub const fn set_sdxc1_sys_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl5 {
        #[inline(always)]
        fn default() -> Ctrl5 {
            Ctrl5(0)
        }
    }
    impl core::fmt::Debug for Ctrl5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl5")
                .field(
                    "sdxc1_gpr_cclk_rx_dly_sw_force",
                    &self.sdxc1_gpr_cclk_rx_dly_sw_force(),
                )
                .field(
                    "sdxc1_gpr_cclk_rx_dly_sw_sel",
                    &self.sdxc1_gpr_cclk_rx_dly_sw_sel(),
                )
                .field(
                    "sdxc1_gpr_strobe_in_enable",
                    &self.sdxc1_gpr_strobe_in_enable(),
                )
                .field(
                    "sdxc1_gpr_tuning_strobe_sel",
                    &self.sdxc1_gpr_tuning_strobe_sel(),
                )
                .field(
                    "sdxc1_gpr_tuning_card_clk_sel",
                    &self.sdxc1_gpr_tuning_card_clk_sel(),
                )
                .field("sdxc1_cardclk_inv_en", &self.sdxc1_cardclk_inv_en())
                .field("sdxc1_wkp_irq_en", &self.sdxc1_wkp_irq_en())
                .field("sdxc1_sys_irq_en", &self.sdxc1_sys_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl5 {{ sdxc1_gpr_cclk_rx_dly_sw_force: {=bool:?}, sdxc1_gpr_cclk_rx_dly_sw_sel: {=u8:?}, sdxc1_gpr_strobe_in_enable: {=bool:?}, sdxc1_gpr_tuning_strobe_sel: {=u8:?}, sdxc1_gpr_tuning_card_clk_sel: {=u8:?}, sdxc1_cardclk_inv_en: {=bool:?}, sdxc1_wkp_irq_en: {=bool:?}, sdxc1_sys_irq_en: {=bool:?} }}" , self . sdxc1_gpr_cclk_rx_dly_sw_force () , self . sdxc1_gpr_cclk_rx_dly_sw_sel () , self . sdxc1_gpr_strobe_in_enable () , self . sdxc1_gpr_tuning_strobe_sel () , self . sdxc1_gpr_tuning_card_clk_sel () , self . sdxc1_cardclk_inv_en () , self . sdxc1_wkp_irq_en () , self . sdxc1_sys_irq_en ())
        }
    }
}
