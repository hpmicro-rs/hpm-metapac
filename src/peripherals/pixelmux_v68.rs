#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PIXEL_MUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PixelMux {
    ptr: *mut u8,
}
unsafe impl Send for PixelMux {}
unsafe impl Sync for PixelMux {}
impl PixelMux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "pixel path mux register."]
    #[inline(always)]
    pub const fn pixmux(self) -> crate::common::Reg<regs::Pixmux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dsi_setting(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DsiSetting, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "common register."]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "gpr write-read register 0."]
    #[inline(always)]
    pub const fn gpr_wr_d0(self) -> crate::common::Reg<regs::GprWrD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "gpr write-read register 1."]
    #[inline(always)]
    pub const fn gpr_wr_d1(self) -> crate::common::Reg<regs::GprWrD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "gpr write-read register 2."]
    #[inline(always)]
    pub const fn gpr_wr_d2(self) -> crate::common::Reg<regs::GprWrD2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "gpr write-read register 3."]
    #[inline(always)]
    pub const fn gpr_wr_d3(self) -> crate::common::Reg<regs::GprWrD3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "gpr write-read register 4."]
    #[inline(always)]
    pub const fn gpr_wr_d4(self) -> crate::common::Reg<regs::GprWrD4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "gpr write-read register 5."]
    #[inline(always)]
    pub const fn gpr_wr_d5(self) -> crate::common::Reg<regs::GprWrD5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "gpr write-read register 6."]
    #[inline(always)]
    pub const fn gpr_wr_d6(self) -> crate::common::Reg<regs::GprWrD6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "gpr write-read register 7."]
    #[inline(always)]
    pub const fn gpr_wr_d7(self) -> crate::common::Reg<regs::GprWrD7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "gpr write-read register 8."]
    #[inline(always)]
    pub const fn gpr_wr_d8(self) -> crate::common::Reg<regs::GprWrD8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "gpr write-read register 9."]
    #[inline(always)]
    pub const fn gpr_wr_d9(self) -> crate::common::Reg<regs::GprWrD9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "gpr read-only register 0."]
    #[inline(always)]
    pub const fn gpr_ro_d0(self) -> crate::common::Reg<regs::GprRoD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "gpr read-only register 1."]
    #[inline(always)]
    pub const fn gpr_ro_d1(self) -> crate::common::Reg<regs::GprRoD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "gpr read-only register 2."]
    #[inline(always)]
    pub const fn gpr_ro_d2(self) -> crate::common::Reg<regs::GprRoD2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "gpr read-only register 3."]
    #[inline(always)]
    pub const fn gpr_ro_d3(self) -> crate::common::Reg<regs::GprRoD3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "gpr read-only register 4."]
    #[inline(always)]
    pub const fn gpr_ro_d4(self) -> crate::common::Reg<regs::GprRoD4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "gpr read-only register 5."]
    #[inline(always)]
    pub const fn gpr_ro_d5(self) -> crate::common::Reg<regs::GprRoD5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "gpr read-only register 6."]
    #[inline(always)]
    pub const fn gpr_ro_d6(self) -> crate::common::Reg<regs::GprRoD6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "gpr read-only register 7."]
    #[inline(always)]
    pub const fn gpr_ro_d7(self) -> crate::common::Reg<regs::GprRoD7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "gpr read-only register 8."]
    #[inline(always)]
    pub const fn gpr_ro_d8(self) -> crate::common::Reg<regs::GprRoD8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "gpr read-only register 9."]
    #[inline(always)]
    pub const fn gpr_ro_d9(self) -> crate::common::Reg<regs::GprRoD9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "gpr write1 set/no-write clr register."]
    #[inline(always)]
    pub const fn gpr_wr1_clr_d0(self) -> crate::common::Reg<regs::GprWr1ClrD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DsiSetting(pub u32);
    impl DsiSetting {
        #[doc = "DSI input pixel data type: ‘h0: RGB565_CFG1 ‘h1: RGB565_CFG2 ‘h2: RGB565_CFG3 ‘h3: RGB666_CFG1 ‘h4: RGB666_CFG2 ‘h5: RGB888 ‘h6: RGB_10BIT ‘h7: RGB_12BIT, no support ‘h8:YUV422_12BIT,no support ‘h9: YUV422_10BIT, no support ‘ha: YUV422_8BIT, no support ‘hb: YUV420_8BIT,no support ‘hc~’hf: Reserved."]
        #[inline(always)]
        pub const fn dsi_data_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DSI input pixel data type: ‘h0: RGB565_CFG1 ‘h1: RGB565_CFG2 ‘h2: RGB565_CFG3 ‘h3: RGB666_CFG1 ‘h4: RGB666_CFG2 ‘h5: RGB888 ‘h6: RGB_10BIT ‘h7: RGB_12BIT, no support ‘h8:YUV422_12BIT,no support ‘h9: YUV422_10BIT, no support ‘ha: YUV422_8BIT, no support ‘hb: YUV420_8BIT,no support ‘hc~’hf: Reserved."]
        #[inline(always)]
        pub fn set_dsi_data_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "DSI pixel data type enable: Bit0: RGB565_CFG1 Bit1: RGB565_CFG2 Bit2: RGB565_CFG3 Bit3: RGB666_CFG1 Bit4: RGB666_CFG2 Bit5: RGB888 Bit6: RGB_10BIT Bit7: RGB_12BIT, no support Bit8: YUV422_12BIT, no support Bit9: YUV422_10BIT, no support Bit10: YUV422_8BIT, no support Bit11:YUV420_8BIT,no support others: Reserved."]
        #[inline(always)]
        pub const fn dsi_data_enable(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "DSI pixel data type enable: Bit0: RGB565_CFG1 Bit1: RGB565_CFG2 Bit2: RGB565_CFG3 Bit3: RGB666_CFG1 Bit4: RGB666_CFG2 Bit5: RGB888 Bit6: RGB_10BIT Bit7: RGB_12BIT, no support Bit8: YUV422_12BIT, no support Bit9: YUV422_10BIT, no support Bit10: YUV422_8BIT, no support Bit11:YUV420_8BIT,no support others: Reserved."]
        #[inline(always)]
        pub fn set_dsi_data_enable(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DsiSetting {
        #[inline(always)]
        fn default() -> DsiSetting {
            DsiSetting(0)
        }
    }
    #[doc = "gpr read-only register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD0(pub u32);
    impl GprRoD0 {
        #[doc = "{2'b0, tx_phy0_tx3_ctl_o,tx_phy0_tx2_ctl_o, tx_phy0_tx1_ctl_o,tx_phy0_tx0_ctl_o, tx_phy0_txck_ctl_o,tx_phy0_pll_dtest_o}."]
        #[inline(always)]
        pub const fn tx_phy0_ctl_o(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "{2'b0, tx_phy0_tx3_ctl_o,tx_phy0_tx2_ctl_o, tx_phy0_tx1_ctl_o,tx_phy0_tx0_ctl_o, tx_phy0_txck_ctl_o,tx_phy0_pll_dtest_o}."]
        #[inline(always)]
        pub fn set_tx_phy0_ctl_o(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "{2'b0, tx_phy1_tx3_ctl_o,tx_phy1_tx2_ctl_o, tx_phy1_tx1_ctl_o,tx_phy1_tx0_ctl_o, tx_phy1_txck_ctl_o,tx_phy1_pll_dtest_o}."]
        #[inline(always)]
        pub const fn tx_phy1_ctl_o(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "{2'b0, tx_phy1_tx3_ctl_o,tx_phy1_tx2_ctl_o, tx_phy1_tx1_ctl_o,tx_phy1_tx0_ctl_o, tx_phy1_txck_ctl_o,tx_phy1_pll_dtest_o}."]
        #[inline(always)]
        pub fn set_tx_phy1_ctl_o(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for GprRoD0 {
        #[inline(always)]
        fn default() -> GprRoD0 {
            GprRoD0(0)
        }
    }
    #[doc = "gpr read-only register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD1(pub u32);
    impl GprRoD1 {
        #[doc = "csi0 apb parity check interrupt satus."]
        #[inline(always)]
        pub const fn csi0_sta_ap_if_int_sta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "csi0 apb parity check interrupt satus."]
        #[inline(always)]
        pub fn set_csi0_sta_ap_if_int_sta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "csi0 ap diag faults."]
        #[inline(always)]
        pub const fn csi0_cfg_csi_ap_diag_faults(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x0fff;
            val as u16
        }
        #[doc = "csi0 ap diag faults."]
        #[inline(always)]
        pub fn set_csi0_cfg_csi_ap_diag_faults(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
        }
        #[doc = "interrupt of csi0 ap."]
        #[inline(always)]
        pub const fn irq_csi0_ap(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt of csi0 ap."]
        #[inline(always)]
        pub fn set_irq_csi0_ap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for GprRoD1 {
        #[inline(always)]
        fn default() -> GprRoD1 {
            GprRoD1(0)
        }
    }
    #[doc = "gpr read-only register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD2(pub u32);
    impl GprRoD2 {
        #[doc = "csi1 apb parity check interrupt satus."]
        #[inline(always)]
        pub const fn csi1_sta_ap_if_int_sta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "csi1 apb parity check interrupt satus."]
        #[inline(always)]
        pub fn set_csi1_sta_ap_if_int_sta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "csi1 ap diag faults."]
        #[inline(always)]
        pub const fn csi1_cfg_csi_ap_diag_faults(&self) -> u16 {
            let val = (self.0 >> 5usize) & 0x0fff;
            val as u16
        }
        #[doc = "csi1 ap diag faults."]
        #[inline(always)]
        pub fn set_csi1_cfg_csi_ap_diag_faults(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
        }
        #[doc = "interrupt of csi1 ap."]
        #[inline(always)]
        pub const fn irq_csi1_ap(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt of csi1 ap."]
        #[inline(always)]
        pub fn set_irq_csi1_ap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for GprRoD2 {
        #[inline(always)]
        fn default() -> GprRoD2 {
            GprRoD2(0)
        }
    }
    #[doc = "gpr read-only register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD3(pub u32);
    impl GprRoD3 {
        #[doc = "rx phy0 rx0_ctlo."]
        #[inline(always)]
        pub const fn rx_phy0_rx0_ctlo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy0 rx0_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy0_rx0_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "rx phy0 rx1_ctlo."]
        #[inline(always)]
        pub const fn rx_phy0_rx1_ctlo(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy0 rx1_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy0_rx1_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "rx phy0 rxck_ctlo."]
        #[inline(always)]
        pub const fn rx_phy0_rxck_ctlo(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "rx phy0 rxck_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy0_rxck_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for GprRoD3 {
        #[inline(always)]
        fn default() -> GprRoD3 {
            GprRoD3(0)
        }
    }
    #[doc = "gpr read-only register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD4(pub u32);
    impl GprRoD4 {
        #[doc = "rx phy1 rx0_ctlo."]
        #[inline(always)]
        pub const fn rx_phy1_rx0_ctlo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy1 rx0_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy1_rx0_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "rx phy1 rx1_ctlo."]
        #[inline(always)]
        pub const fn rx_phy1_rx1_ctlo(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy1 rx1_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy1_rx1_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "rx phy1 rxck_ctlo."]
        #[inline(always)]
        pub const fn rx_phy1_rxck_ctlo(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "rx phy1 rxck_ctlo."]
        #[inline(always)]
        pub fn set_rx_phy1_rxck_ctlo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for GprRoD4 {
        #[inline(always)]
        fn default() -> GprRoD4 {
            GprRoD4(0)
        }
    }
    #[doc = "gpr read-only register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD5(pub u32);
    impl GprRoD5 {
        #[doc = "tx phy0 tx0_bist_out."]
        #[inline(always)]
        pub const fn tx_phy0_tx0_bist_out(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx0_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy0_tx0_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "tx phy0 tx1_bist_out."]
        #[inline(always)]
        pub const fn tx_phy0_tx1_bist_out(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx1_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy0_tx1_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "tx phy0 tx2_bist_out."]
        #[inline(always)]
        pub const fn tx_phy0_tx2_bist_out(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx2_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy0_tx2_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "tx phy0 tx3_bist_out."]
        #[inline(always)]
        pub const fn tx_phy0_tx3_bist_out(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx3_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy0_tx3_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "tx phy0 txck_bist_out."]
        #[inline(always)]
        pub const fn tx_phy0_txck_bist_out(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 txck_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "tx phy0 tx0_bist_done."]
        #[inline(always)]
        pub const fn tx_phy0_tx0_bist_done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx0_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy0_tx0_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "tx phy0 tx1_bist_done."]
        #[inline(always)]
        pub const fn tx_phy0_tx1_bist_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx1_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy0_tx1_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "tx phy0 tx2_bist_done."]
        #[inline(always)]
        pub const fn tx_phy0_tx2_bist_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx2_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy0_tx2_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "tx phy0 tx3_bist_done."]
        #[inline(always)]
        pub const fn tx_phy0_tx3_bist_done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx3_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy0_tx3_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "tx phy0 txck_bist_done."]
        #[inline(always)]
        pub const fn tx_phy0_txck_bist_done(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 txck_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "tx phy0 txck_ok_pad."]
        #[inline(always)]
        pub const fn tx_phy0_txck_bist_ok_pad(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 txck_ok_pad."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_bist_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "tx phy0 txck_done_pad."]
        #[inline(always)]
        pub const fn tx_phy0_txck_bist_done_pad(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 txck_done_pad."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_bist_done_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "dsi0_prbs_state for debug only."]
        #[inline(always)]
        pub const fn dsi0_prbs_state(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "dsi0_prbs_state for debug only."]
        #[inline(always)]
        pub fn set_dsi0_prbs_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for GprRoD5 {
        #[inline(always)]
        fn default() -> GprRoD5 {
            GprRoD5(0)
        }
    }
    #[doc = "gpr read-only register 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD6(pub u32);
    impl GprRoD6 {
        #[doc = "tx phy1 tx0_bist_out."]
        #[inline(always)]
        pub const fn tx_phy1_tx0_bist_out(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx0_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy1_tx0_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "tx phy1 tx1_bist_out."]
        #[inline(always)]
        pub const fn tx_phy1_tx1_bist_out(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx1_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy1_tx1_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "tx phy1 tx2_bist_out."]
        #[inline(always)]
        pub const fn tx_phy1_tx2_bist_out(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx2_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy1_tx2_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "tx phy1 tx3_bist_out."]
        #[inline(always)]
        pub const fn tx_phy1_tx3_bist_out(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx3_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy1_tx3_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "tx phy1 txck_bist_out."]
        #[inline(always)]
        pub const fn tx_phy1_txck_bist_out(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 txck_bist_out."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "tx phy1 tx0_bist_done."]
        #[inline(always)]
        pub const fn tx_phy1_tx0_bist_done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx0_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy1_tx0_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "tx phy1 tx1_bist_done."]
        #[inline(always)]
        pub const fn tx_phy1_tx1_bist_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx1_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy1_tx1_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "tx phy1 tx2_bist_done."]
        #[inline(always)]
        pub const fn tx_phy1_tx2_bist_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx2_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy1_tx2_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "tx phy1 tx3_bist_done."]
        #[inline(always)]
        pub const fn tx_phy1_tx3_bist_done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx3_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy1_tx3_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "tx phy1 txck_bist_done."]
        #[inline(always)]
        pub const fn tx_phy1_txck_bist_done(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 txck_bist_done."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "tx phy1 txck_ok_pad."]
        #[inline(always)]
        pub const fn tx_phy1_txck_bist_ok_pad(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 txck_ok_pad."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_bist_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "tx phy1 txck_done_pad."]
        #[inline(always)]
        pub const fn tx_phy1_txck_bist_done_pad(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 txck_done_pad."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_bist_done_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "dsi1_prbs_state for debug only."]
        #[inline(always)]
        pub const fn dsi1_prbs_state(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "dsi1_prbs_state for debug only."]
        #[inline(always)]
        pub fn set_dsi1_prbs_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for GprRoD6 {
        #[inline(always)]
        fn default() -> GprRoD6 {
            GprRoD6(0)
        }
    }
    #[doc = "gpr read-only register 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD7(pub u32);
    impl GprRoD7 {
        #[doc = "rx phy0 bist_done_pad."]
        #[inline(always)]
        pub const fn rx_phy0_bist_done_pad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_done_pad."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_done_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "rx phy0 bist_ok_pad."]
        #[inline(always)]
        pub const fn rx_phy0_bist_ok_pad(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_ok_pad."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "rx phy0 rx0_bist_out."]
        #[inline(always)]
        pub const fn rx_phy0_rx0_bist_out(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 rx0_bist_out."]
        #[inline(always)]
        pub fn set_rx_phy0_rx0_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "rx phy0 rx1_bist_out."]
        #[inline(always)]
        pub const fn rx_phy0_rx1_bist_out(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 rx1_bist_out."]
        #[inline(always)]
        pub fn set_rx_phy0_rx1_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "rx phy0 rx0_bist_done."]
        #[inline(always)]
        pub const fn rx_phy0_rx0_bist_done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 rx0_bist_done."]
        #[inline(always)]
        pub fn set_rx_phy0_rx0_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "rx phy0 rx1_bist_done."]
        #[inline(always)]
        pub const fn rx_phy0_rx1_bist_done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 rx1_bist_done."]
        #[inline(always)]
        pub fn set_rx_phy0_rx1_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "rx_phy0_burn_in_ok_pad."]
        #[inline(always)]
        pub const fn rx_phy0_burn_in_ok_pad(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "rx_phy0_burn_in_ok_pad."]
        #[inline(always)]
        pub fn set_rx_phy0_burn_in_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for GprRoD7 {
        #[inline(always)]
        fn default() -> GprRoD7 {
            GprRoD7(0)
        }
    }
    #[doc = "gpr read-only register 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD8(pub u32);
    impl GprRoD8 {
        #[doc = "rx phy1 bist_done_pad."]
        #[inline(always)]
        pub const fn rx_phy1_bist_done_pad(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_done_pad."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_done_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "rx phy1 bist_ok_pad."]
        #[inline(always)]
        pub const fn rx_phy1_bist_ok_pad(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_ok_pad."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "rx phy1 rx0_bist_out."]
        #[inline(always)]
        pub const fn rx_phy1_rx0_bist_out(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 rx0_bist_out."]
        #[inline(always)]
        pub fn set_rx_phy1_rx0_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "rx phy1 rx1_bist_out."]
        #[inline(always)]
        pub const fn rx_phy1_rx1_bist_out(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 rx1_bist_out."]
        #[inline(always)]
        pub fn set_rx_phy1_rx1_bist_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "rx phy1 rx0_bist_done."]
        #[inline(always)]
        pub const fn rx_phy1_rx0_bist_done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 rx0_bist_done."]
        #[inline(always)]
        pub fn set_rx_phy1_rx0_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "rx phy1 rx1_bist_done."]
        #[inline(always)]
        pub const fn rx_phy1_rx1_bist_done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 rx1_bist_done."]
        #[inline(always)]
        pub fn set_rx_phy1_rx1_bist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "rx_phy1_burn_in_ok_pad."]
        #[inline(always)]
        pub const fn rx_phy1_burn_in_ok_pad(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "rx_phy1_burn_in_ok_pad."]
        #[inline(always)]
        pub fn set_rx_phy1_burn_in_ok_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for GprRoD8 {
        #[inline(always)]
        fn default() -> GprRoD8 {
            GprRoD8(0)
        }
    }
    #[doc = "gpr read-only register 9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprRoD9(pub u32);
    impl GprRoD9 {}
    impl Default for GprRoD9 {
        #[inline(always)]
        fn default() -> GprRoD9 {
            GprRoD9(0)
        }
    }
    #[doc = "gpr write1 set/no-write clr register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWr1ClrD0(pub u32);
    impl GprWr1ClrD0 {
        #[doc = "gpr register, write 1 /no-write set/clr matching bit."]
        #[inline(always)]
        pub const fn gpr_wr1_clr_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "gpr register, write 1 /no-write set/clr matching bit."]
        #[inline(always)]
        pub fn set_gpr_wr1_clr_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GprWr1ClrD0 {
        #[inline(always)]
        fn default() -> GprWr1ClrD0 {
            GprWr1ClrD0(0)
        }
    }
    #[doc = "gpr write-read register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD0(pub u32);
    impl GprWrD0 {
        #[doc = "dsi controller 0 reset, active low."]
        #[inline(always)]
        pub const fn dsi0_soft_reset_n(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "dsi controller 0 reset, active low."]
        #[inline(always)]
        pub fn set_dsi0_soft_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "dsi controller 1 reset, active low."]
        #[inline(always)]
        pub const fn dsi1_soft_reset_n(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "dsi controller 1 reset, active low."]
        #[inline(always)]
        pub fn set_dsi1_soft_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "csi controller 0 reset, active low."]
        #[inline(always)]
        pub const fn csi0_soft_reset_n(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "csi controller 0 reset, active low."]
        #[inline(always)]
        pub fn set_csi0_soft_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "csi controller 1 reset, active low."]
        #[inline(always)]
        pub const fn csi1_soft_reset_n(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "csi controller 1 reset, active low."]
        #[inline(always)]
        pub fn set_csi1_soft_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "dsi0 dpi shuntdown control."]
        #[inline(always)]
        pub const fn dsi0_dpishutdn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "dsi0 dpi shuntdown control."]
        #[inline(always)]
        pub fn set_dsi0_dpishutdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "dsi0 dpi cholor mode control."]
        #[inline(always)]
        pub const fn dsi0_dpicolorm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "dsi0 dpi cholor mode control."]
        #[inline(always)]
        pub fn set_dsi0_dpicolorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "dsi0 dpi update configure."]
        #[inline(always)]
        pub const fn dsi0_dpiupdatecfg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "dsi0 dpi update configure."]
        #[inline(always)]
        pub fn set_dsi0_dpiupdatecfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "dsi1 dpi shuntdown control."]
        #[inline(always)]
        pub const fn dsi1_dpishutdn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "dsi1 dpi shuntdown control."]
        #[inline(always)]
        pub fn set_dsi1_dpishutdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "dsi1 dpi cholor mode control."]
        #[inline(always)]
        pub const fn dsi1_dpicolorm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "dsi1 dpi cholor mode control."]
        #[inline(always)]
        pub fn set_dsi1_dpicolorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "dsi1 dpi update configure."]
        #[inline(always)]
        pub const fn dsi1_dpiupdatecfg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "dsi1 dpi update configure."]
        #[inline(always)]
        pub fn set_dsi1_dpiupdatecfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "csi0 apb interface error check enable."]
        #[inline(always)]
        pub const fn csi0_cfg_apb_slverror_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "csi0 apb interface error check enable."]
        #[inline(always)]
        pub fn set_csi0_cfg_apb_slverror_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "csi0 apb interface error interrupt enable."]
        #[inline(always)]
        pub const fn csi0_cfg_ap_if_int_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "csi0 apb interface error interrupt enable."]
        #[inline(always)]
        pub fn set_csi0_cfg_ap_if_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "csi0 apb interface parity check enable."]
        #[inline(always)]
        pub const fn csi0_cfg_ap_if_check_en(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x1f;
            val as u8
        }
        #[doc = "csi0 apb interface parity check enable."]
        #[inline(always)]
        pub fn set_csi0_cfg_ap_if_check_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
        }
        #[doc = "csi1 apb interface error check enable."]
        #[inline(always)]
        pub const fn csi1_cfg_apb_slverror_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "csi1 apb interface error check enable."]
        #[inline(always)]
        pub fn set_csi1_cfg_apb_slverror_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "csi1 apb interface error interrupt enable."]
        #[inline(always)]
        pub const fn csi1_cfg_ap_if_int_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "csi1 apb interface error interrupt enable."]
        #[inline(always)]
        pub fn set_csi1_cfg_ap_if_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "csi1 apb interface parity check enable."]
        #[inline(always)]
        pub const fn csi1_cfg_ap_if_check_en(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x1f;
            val as u8
        }
        #[doc = "csi1 apb interface parity check enable."]
        #[inline(always)]
        pub fn set_csi1_cfg_ap_if_check_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 22usize)) | (((val as u32) & 0x1f) << 22usize);
        }
    }
    impl Default for GprWrD0 {
        #[inline(always)]
        fn default() -> GprWrD0 {
            GprWrD0(0)
        }
    }
    #[doc = "gpr write-read register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD1(pub u32);
    impl GprWrD1 {
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn lcdc0_p0_ctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_lcdc0_p0_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn lcdc0_p1_ctrl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_lcdc0_p1_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn lcdc1_p0_ctrl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_lcdc1_p0_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn lcdc1_p1_ctrl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_lcdc1_p1_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn pdma_p0_ctrl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_pdma_p0_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn pdma_p1_ctrl(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_pdma_p1_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub const fn jpeg_ctrl(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma."]
        #[inline(always)]
        pub fn set_jpeg_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for GprWrD1 {
        #[inline(always)]
        fn default() -> GprWrD1 {
            GprWrD1(0)
        }
    }
    #[doc = "gpr write-read register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD2(pub u32);
    impl GprWrD2 {
        #[doc = "tx phy0 pll_div."]
        #[inline(always)]
        pub const fn tx_phy0_pll_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "tx phy0 pll_div."]
        #[inline(always)]
        pub fn set_tx_phy0_pll_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "tx phy0 byps_ckdet."]
        #[inline(always)]
        pub const fn tx_phy0_byps_ckdet(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 byps_ckdet."]
        #[inline(always)]
        pub fn set_tx_phy0_byps_ckdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "tx phy0 shutdownz, active low."]
        #[inline(always)]
        pub const fn tx_phy0_shutdownz(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 shutdownz, active low."]
        #[inline(always)]
        pub fn set_tx_phy0_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "tx phy0 reset, active low."]
        #[inline(always)]
        pub const fn tx_phy0_reset_n(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 reset, active low."]
        #[inline(always)]
        pub fn set_tx_phy0_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "tx phy0 iddq_en."]
        #[inline(always)]
        pub const fn tx_phy0_iddq_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 iddq_en."]
        #[inline(always)]
        pub fn set_tx_phy0_iddq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "tx phy0 refclk_div."]
        #[inline(always)]
        pub const fn tx_phy0_refclk_div(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "tx phy0 refclk_div."]
        #[inline(always)]
        pub fn set_tx_phy0_refclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "tx phy0 phy_mode."]
        #[inline(always)]
        pub const fn tx_phy0_phy_mode(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 phy_mode."]
        #[inline(always)]
        pub fn set_tx_phy0_phy_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "tx phy0 rate_lvds."]
        #[inline(always)]
        pub const fn tx_phy0_rate_lvds(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 rate_lvds."]
        #[inline(always)]
        pub fn set_tx_phy0_rate_lvds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
        #[doc = "tx phy0 port_pll_rdy_sel."]
        #[inline(always)]
        pub const fn tx_phy0_port_pll_rdy_sel(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 port_pll_rdy_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_port_pll_rdy_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for GprWrD2 {
        #[inline(always)]
        fn default() -> GprWrD2 {
            GprWrD2(0)
        }
    }
    #[doc = "gpr write-read register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD3(pub u32);
    impl GprWrD3 {
        #[doc = "tx phy0 pll_ctrl."]
        #[inline(always)]
        pub const fn tx_phy0_pll_ctrl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "tx phy0 pll_ctrl."]
        #[inline(always)]
        pub fn set_tx_phy0_pll_ctrl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GprWrD3 {
        #[inline(always)]
        fn default() -> GprWrD3 {
            GprWrD3(0)
        }
    }
    #[doc = "gpr write-read register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD4(pub u32);
    impl GprWrD4 {
        #[doc = "tx phy0 ckphy_ctl."]
        #[inline(always)]
        pub const fn tx_phy0_ckphy_ctl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "tx phy0 ckphy_ctl."]
        #[inline(always)]
        pub fn set_tx_phy0_ckphy_ctl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "tx phy0 dsi0_prbs_start."]
        #[inline(always)]
        pub const fn tx_phy0_dsi0_prbs_start(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 dsi0_prbs_start."]
        #[inline(always)]
        pub fn set_tx_phy0_dsi0_prbs_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "tx phy0 dsi0_prbs_disable."]
        #[inline(always)]
        pub const fn tx_phy0_dsi0_prbs_disable(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 dsi0_prbs_disable."]
        #[inline(always)]
        pub fn set_tx_phy0_dsi0_prbs_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "tx phy0 tx0_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy0_tx0_pat_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 tx0_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_tx0_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "tx phy0 tx1_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy0_tx1_pat_sel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 tx1_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_tx1_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "tx phy0 tx2_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy0_tx2_pat_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 tx2_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_tx2_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "tx phy0 tx3_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy0_tx3_pat_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 tx3_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_tx3_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "tx phy0 txck_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy0_txck_pat_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy0 txck_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "tx_phy0 tx0_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx0_lpbk_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy0 tx0_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx0_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "tx_phy0 tx1_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx1_lpbk_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy0 tx1_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx1_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "tx_phy0 tx2_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx2_lpbk_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy0 tx2_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx2_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "tx_phy0 tx3_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx3_lpbk_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy0 tx3_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx3_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "tx_phy0 txck_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy0_txck_lpbk_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy0 txck_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "tx phy0 tx0_bist_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx0_bist_en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx0_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx0_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "tx phy0 tx1_bist_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx1_bist_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx1_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx1_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "tx phy0 tx2_bist_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx2_bist_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx2_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx2_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "tx phy0 tx3_bist_en."]
        #[inline(always)]
        pub const fn tx_phy0_tx3_bist_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 tx3_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy0_tx3_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "tx phy0 txck_bist_en."]
        #[inline(always)]
        pub const fn tx_phy0_txck_bist_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy0 txck_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy0_txck_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GprWrD4 {
        #[inline(always)]
        fn default() -> GprWrD4 {
            GprWrD4(0)
        }
    }
    #[doc = "gpr write-read register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD5(pub u32);
    impl GprWrD5 {
        #[doc = "tx phy1 pll_div."]
        #[inline(always)]
        pub const fn tx_phy1_pll_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "tx phy1 pll_div."]
        #[inline(always)]
        pub fn set_tx_phy1_pll_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "tx phy1 byps_ckdet."]
        #[inline(always)]
        pub const fn tx_phy1_byps_ckdet(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 byps_ckdet."]
        #[inline(always)]
        pub fn set_tx_phy1_byps_ckdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "tx phy1 shutdownz, active low."]
        #[inline(always)]
        pub const fn tx_phy1_shutdownz(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 shutdownz, active low."]
        #[inline(always)]
        pub fn set_tx_phy1_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "tx phy1 reset, active low."]
        #[inline(always)]
        pub const fn tx_phy1_reset_n(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 reset, active low."]
        #[inline(always)]
        pub fn set_tx_phy1_reset_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "tx phy1 iddq_en."]
        #[inline(always)]
        pub const fn tx_phy1_iddq_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 iddq_en."]
        #[inline(always)]
        pub fn set_tx_phy1_iddq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "tx phy1 refclk_div."]
        #[inline(always)]
        pub const fn tx_phy1_refclk_div(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "tx phy1 refclk_div."]
        #[inline(always)]
        pub fn set_tx_phy1_refclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "tx phy1 phy_mode."]
        #[inline(always)]
        pub const fn tx_phy1_phy_mode(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 phy_mode."]
        #[inline(always)]
        pub fn set_tx_phy1_phy_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "tx phy1 rate_lvds."]
        #[inline(always)]
        pub const fn tx_phy1_rate_lvds(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 rate_lvds."]
        #[inline(always)]
        pub fn set_tx_phy1_rate_lvds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
        #[doc = "tx phy1 port_pll_rdy_sel."]
        #[inline(always)]
        pub const fn tx_phy1_port_pll_rdy_sel(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 port_pll_rdy_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_port_pll_rdy_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for GprWrD5 {
        #[inline(always)]
        fn default() -> GprWrD5 {
            GprWrD5(0)
        }
    }
    #[doc = "gpr write-read register 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD6(pub u32);
    impl GprWrD6 {
        #[doc = "tx phy1 pll_ctrl."]
        #[inline(always)]
        pub const fn tx_phy1_pll_ctrl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "tx phy1 pll_ctrl."]
        #[inline(always)]
        pub fn set_tx_phy1_pll_ctrl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GprWrD6 {
        #[inline(always)]
        fn default() -> GprWrD6 {
            GprWrD6(0)
        }
    }
    #[doc = "gpr write-read register 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD7(pub u32);
    impl GprWrD7 {
        #[doc = "tx phy1 ckphy_ctl."]
        #[inline(always)]
        pub const fn tx_phy1_ckphy_ctl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "tx phy1 ckphy_ctl."]
        #[inline(always)]
        pub fn set_tx_phy1_ckphy_ctl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "tx phy1 dsi0_prbs_start."]
        #[inline(always)]
        pub const fn tx_phy1_dsi0_prbs_start(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 dsi0_prbs_start."]
        #[inline(always)]
        pub fn set_tx_phy1_dsi0_prbs_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "tx phy1 dsi0_prbs_disable."]
        #[inline(always)]
        pub const fn tx_phy1_dsi0_prbs_disable(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 dsi0_prbs_disable."]
        #[inline(always)]
        pub fn set_tx_phy1_dsi0_prbs_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "tx phy1 tx0_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy1_tx0_pat_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 tx0_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_tx0_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "tx phy1 tx1_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy1_tx1_pat_sel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 tx1_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_tx1_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "tx phy1 tx2_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy1_tx2_pat_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 tx2_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_tx2_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "tx phy1 tx3_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy1_tx3_pat_sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 tx3_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_tx3_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "tx phy1 txck_pat_sel."]
        #[inline(always)]
        pub const fn tx_phy1_txck_pat_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "tx phy1 txck_pat_sel."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_pat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "tx_phy1 tx0_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx0_lpbk_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy1 tx0_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx0_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "tx_phy1 tx1_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx1_lpbk_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy1 tx1_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx1_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "tx_phy1 tx2_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx2_lpbk_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy1 tx2_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx2_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "tx_phy1 tx3_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx3_lpbk_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy1 tx3_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx3_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "tx_phy1 txck_lpbk_en."]
        #[inline(always)]
        pub const fn tx_phy1_txck_lpbk_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "tx_phy1 txck_lpbk_en."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_lpbk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "tx phy1 tx0_bist_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx0_bist_en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx0_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx0_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "tx phy1 tx1_bist_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx1_bist_en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx1_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx1_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "tx phy1 tx2_bist_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx2_bist_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx2_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx2_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "tx phy1 tx3_bist_en."]
        #[inline(always)]
        pub const fn tx_phy1_tx3_bist_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 tx3_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy1_tx3_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "tx phy1 txck_bist_en."]
        #[inline(always)]
        pub const fn tx_phy1_txck_bist_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "tx phy1 txck_bist_en."]
        #[inline(always)]
        pub fn set_tx_phy1_txck_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GprWrD7 {
        #[inline(always)]
        fn default() -> GprWrD7 {
            GprWrD7(0)
        }
    }
    #[doc = "gpr write-read register 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD8(pub u32);
    impl GprWrD8 {
        #[doc = "rx phy0 phy_mode."]
        #[inline(always)]
        pub const fn rx_phy0_phy_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "rx phy0 phy_mode."]
        #[inline(always)]
        pub fn set_rx_phy0_phy_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "rx phy0 bist_ckin_sel."]
        #[inline(always)]
        pub const fn rx_phy0_bist_ckin_sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_ckin_sel."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_ckin_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "rx phy0 bist_en."]
        #[inline(always)]
        pub const fn rx_phy0_bist_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_en."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "rx phy0 bist_en_pad."]
        #[inline(always)]
        pub const fn rx_phy0_bist_en_pad(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_en_pad."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_en_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "rx phy0 bist_mode."]
        #[inline(always)]
        pub const fn rx_phy0_bist_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 bist_mode."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "rx phy0 rx0_bist_en rx1_bist_en."]
        #[inline(always)]
        pub const fn rx_phy0_rx0_bist_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 rx0_bist_en rx1_bist_en."]
        #[inline(always)]
        pub fn set_rx_phy0_rx0_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "rx phy0 bist_freq_trim."]
        #[inline(always)]
        pub const fn rx_phy0_bist_freq_trim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy0 bist_freq_trim."]
        #[inline(always)]
        pub fn set_rx_phy0_bist_freq_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "rx phy0 lpbk_mode."]
        #[inline(always)]
        pub const fn rx_phy0_lpbk_mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "rx phy0 lpbk_mode."]
        #[inline(always)]
        pub fn set_rx_phy0_lpbk_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "rx phy0 burn_in_en_pad."]
        #[inline(always)]
        pub const fn rx_phy0_burn_in_en_pad(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 burn_in_en_pad."]
        #[inline(always)]
        pub fn set_rx_phy0_burn_in_en_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "rx phy0 burn_in_mode."]
        #[inline(always)]
        pub const fn rx_phy0_brun_in_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy0 burn_in_mode."]
        #[inline(always)]
        pub fn set_rx_phy0_brun_in_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GprWrD8 {
        #[inline(always)]
        fn default() -> GprWrD8 {
            GprWrD8(0)
        }
    }
    #[doc = "gpr write-read register 9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprWrD9(pub u32);
    impl GprWrD9 {
        #[doc = "rx phy1 phy_mode."]
        #[inline(always)]
        pub const fn rx_phy1_phy_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "rx phy1 phy_mode."]
        #[inline(always)]
        pub fn set_rx_phy1_phy_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "rx phy1 bist_ckin_sel."]
        #[inline(always)]
        pub const fn rx_phy1_bist_ckin_sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_ckin_sel."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_ckin_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "rx phy1 bist_en."]
        #[inline(always)]
        pub const fn rx_phy1_bist_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_en."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "rx phy1 bist_en_pad."]
        #[inline(always)]
        pub const fn rx_phy1_bist_en_pad(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_en_pad."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_en_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "rx phy1 bist_mode."]
        #[inline(always)]
        pub const fn rx_phy1_bist_mode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 bist_mode."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "rx phy1 rx0_bist_en rx1_bist_en."]
        #[inline(always)]
        pub const fn rx_phy1_rx0_bist_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 rx0_bist_en rx1_bist_en."]
        #[inline(always)]
        pub fn set_rx_phy1_rx0_bist_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "rx phy1 bist_freq_trim."]
        #[inline(always)]
        pub const fn rx_phy1_bist_freq_trim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "rx phy1 bist_freq_trim."]
        #[inline(always)]
        pub fn set_rx_phy1_bist_freq_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "rx phy1 lpbk_mode."]
        #[inline(always)]
        pub const fn rx_phy1_lpbk_mode(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "rx phy1 lpbk_mode."]
        #[inline(always)]
        pub fn set_rx_phy1_lpbk_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "rx phy1 burn_in_en_pad."]
        #[inline(always)]
        pub const fn rx_phy1_burn_in_en_pad(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 burn_in_en_pad."]
        #[inline(always)]
        pub fn set_rx_phy1_burn_in_en_pad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "rx phy1 burn_in_mode."]
        #[inline(always)]
        pub const fn rx_phy1_brun_in_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "rx phy1 burn_in_mode."]
        #[inline(always)]
        pub fn set_rx_phy1_brun_in_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GprWrD9 {
        #[inline(always)]
        fn default() -> GprWrD9 {
            GprWrD9(0)
        }
    }
    #[doc = "common register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misc(pub u32);
    impl Misc {
        #[doc = "LVB DI0 optional general purpose control which is usually unused by display."]
        #[inline(always)]
        pub const fn lvb_di0_ctl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI0 optional general purpose control which is usually unused by display."]
        #[inline(always)]
        pub fn set_lvb_di0_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LVB DI1 optional general purpose control which is usually unused by display."]
        #[inline(always)]
        pub const fn lvb_di1_ctl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI1 optional general purpose control which is usually unused by display."]
        #[inline(always)]
        pub fn set_lvb_di1_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Misc {
        #[inline(always)]
        fn default() -> Misc {
            Misc(0)
        }
    }
    #[doc = "pixel path mux register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pixmux(pub u32);
    impl Pixmux {
        #[doc = "CAM0 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP."]
        #[inline(always)]
        pub const fn cam0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "CAM0 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP."]
        #[inline(always)]
        pub fn set_cam0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "CAM0 pixel bus enable."]
        #[inline(always)]
        pub const fn cam0_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CAM0 pixel bus enable."]
        #[inline(always)]
        pub fn set_cam0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CAM1 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP."]
        #[inline(always)]
        pub const fn cam1_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "CAM1 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP."]
        #[inline(always)]
        pub fn set_cam1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "CAM1 pixel bus enable."]
        #[inline(always)]
        pub const fn cam1_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CAM1 pixel bus enable."]
        #[inline(always)]
        pub fn set_cam1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DSI1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn dsi0_sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DSI1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_dsi0_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DSI1 pixel bus enable."]
        #[inline(always)]
        pub const fn dsi0_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DSI1 pixel bus enable."]
        #[inline(always)]
        pub fn set_dsi0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DSI0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn dsi1_sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DSI0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_dsi1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "DSI0 pixel bus enable."]
        #[inline(always)]
        pub const fn dsi1_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "DSI0 pixel bus enable."]
        #[inline(always)]
        pub fn set_dsi1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "LVB DI0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn lvb_di0_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_lvb_di0_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "LVB DI0 pixel bus enable."]
        #[inline(always)]
        pub const fn lvb_di0_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI0 pixel bus enable."]
        #[inline(always)]
        pub fn set_lvb_di0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "LVB DI1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn lvb_di1_sel(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_lvb_di1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "LVB DI1 pixel bus enable."]
        #[inline(always)]
        pub const fn lvb_di1_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "LVB DI1 pixel bus enable."]
        #[inline(always)]
        pub fn set_lvb_di1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "GWC0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn gwc0_sel(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GWC0 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_gwc0_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GWC0 pixel bus enable."]
        #[inline(always)]
        pub const fn gwc0_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GWC0 pixel bus enable."]
        #[inline(always)]
        pub fn set_gwc0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "GWC1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn gwc1_sel(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "GWC1 pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_gwc1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "GWC1 pixel bus enable."]
        #[inline(always)]
        pub const fn gwc1_en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "GWC1 pixel bus enable."]
        #[inline(always)]
        pub fn set_gwc1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "RGB pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub const fn rgb_sel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "RGB pixel bus selection 1: LCDC1 0: LCDC0."]
        #[inline(always)]
        pub fn set_rgb_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "RGB pixel bus enable."]
        #[inline(always)]
        pub const fn rgb_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "RGB pixel bus enable."]
        #[inline(always)]
        pub fn set_rgb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Pixmux {
        #[inline(always)]
        fn default() -> Pixmux {
            Pixmux(0)
        }
    }
}
