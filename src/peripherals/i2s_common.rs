#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "I2S0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s {
    ptr: *mut u8,
}
unsafe impl Send for I2s {}
unsafe impl Sync for I2s {}
impl I2s {
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
    #[doc = "Rx FIFO Filling Level."]
    #[inline(always)]
    pub const fn rfifo_fillings(
        self,
    ) -> crate::common::Reg<regs::RfifoFillings, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Tx FIFO Filling Level."]
    #[inline(always)]
    pub const fn tfifo_fillings(
        self,
    ) -> crate::common::Reg<regs::TfifoFillings, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TX/RX FIFO Threshold setting."]
    #[inline(always)]
    pub const fn fifo_thresh(self) -> crate::common::Reg<regs::FifoThresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Status Registers."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxd(self, n: usize) -> crate::common::Reg<regs::Rxd, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txd(self, n: usize) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Configruation Regsiters."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Misc configuration Registers."]
    #[inline(always)]
    pub const fn misc_cfgr(self) -> crate::common::Reg<regs::MiscCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxdslot(self, n: usize) -> crate::common::Reg<regs::Rxdslot, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txdslot(self, n: usize) -> crate::common::Reg<regs::Txdslot, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configruation Regsiters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATSIZ = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn chsiz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATSIZ = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_chsiz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn datsiz(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_datsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn std(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_std(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[inline(always)]
        pub const fn tdm_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[inline(always)]
        pub fn set_tdm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CH_MAX\\[4:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 5'h2: 2 channels 5'h4: 4 channels ... 5‘h10: 16 channels (max)."]
        #[inline(always)]
        pub const fn ch_max(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "CH_MAX\\[4:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 5'h2: 2 channels 5'h4: 4 channels ... 5‘h10: 16 channels (max)."]
        #[inline(always)]
        pub fn set_ch_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[inline(always)]
        pub const fn frame_edge(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[inline(always)]
        pub fn set_frame_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn mck_sel_op(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub fn set_mck_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn fclk_sel_op(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub fn set_fclk_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn bclk_sel_op(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub fn set_bclk_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode."]
        #[inline(always)]
        pub const fn inv_mclk_in(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode."]
        #[inline(always)]
        pub fn set_inv_mclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Invert the MCLK before sending it out to pad. Only valid in MCLK master mode."]
        #[inline(always)]
        pub const fn inv_mclk_out(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the MCLK before sending it out to pad. Only valid in MCLK master mode."]
        #[inline(always)]
        pub fn set_inv_mclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode."]
        #[inline(always)]
        pub const fn inv_fclk_in(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode."]
        #[inline(always)]
        pub fn set_inv_fclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Invert the FCLK before sending it out to pad. Only valid in FCLK master mode."]
        #[inline(always)]
        pub const fn inv_fclk_out(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the FCLK before sending it out to pad. Only valid in FCLK master mode."]
        #[inline(always)]
        pub fn set_inv_fclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode."]
        #[inline(always)]
        pub const fn inv_bclk_in(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode."]
        #[inline(always)]
        pub fn set_inv_bclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Invert the BCLK before sending it out to pad. Only valid in BCLK master mode."]
        #[inline(always)]
        pub const fn inv_bclk_out(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the BCLK before sending it out to pad. Only valid in BCLK master mode."]
        #[inline(always)]
        pub fn set_inv_bclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[inline(always)]
        pub const fn bclk_div(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x01ff;
            val as u16
        }
        #[doc = "Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[inline(always)]
        pub fn set_bclk_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 21usize)) | (((val as u32) & 0x01ff) << 21usize);
        }
        #[doc = "Gate off the bclk. Asserted to gate-off the BCLK."]
        #[inline(always)]
        pub const fn bclk_gateoff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Gate off the bclk. Asserted to gate-off the BCLK."]
        #[inline(always)]
        pub fn set_bclk_gateoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "enable for the module."]
        #[inline(always)]
        pub const fn i2s_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable for the module."]
        #[inline(always)]
        pub fn set_i2s_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enable for each RX data pad."]
        #[inline(always)]
        pub const fn rx_en(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "enable for each RX data pad."]
        #[inline(always)]
        pub fn set_rx_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "enable for each TX data pad."]
        #[inline(always)]
        pub const fn tx_en(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "enable for each TX data pad."]
        #[inline(always)]
        pub fn set_tx_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn rxfifoclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub fn set_rxfifoclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn txfifoclr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub fn set_txfifoclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub const fn rx_dma_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub fn set_rx_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub const fn tx_dma_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub fn set_tx_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
        #[inline(always)]
        pub const fn rxdaie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
        #[inline(always)]
        pub fn set_rxdaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
        #[inline(always)]
        pub const fn txdnie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
        #[inline(always)]
        pub fn set_txdnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn sftrst_clkgen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub fn set_sftrst_clkgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "software reset the TX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn sftrst_tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the TX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub fn set_sftrst_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "software reset the RX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn sftrst_rx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the RX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub fn set_sftrst_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "TX/RX FIFO Threshold setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoThresh(pub u32);
    impl FifoThresh {
        #[doc = "RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
        #[inline(always)]
        pub const fn rx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
        #[inline(always)]
        pub fn set_rx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
        #[inline(always)]
        pub fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for FifoThresh {
        #[inline(always)]
        fn default() -> FifoThresh {
            FifoThresh(0)
        }
    }
    #[doc = "Misc configuration Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscCfgr(pub u32);
    impl MiscCfgr {
        #[doc = "Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[inline(always)]
        pub const fn mclkoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[inline(always)]
        pub fn set_mclkoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
        #[inline(always)]
        pub const fn mclk_gateoff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
        #[inline(always)]
        pub fn set_mclk_gateoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MiscCfgr {
        #[inline(always)]
        fn default() -> MiscCfgr {
            MiscCfgr(0)
        }
    }
    #[doc = "Rx FIFO Filling Level."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RfifoFillings(pub u32);
    impl RfifoFillings {
        #[doc = "RX0 fifo fillings."]
        #[inline(always)]
        pub const fn rx0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RX0 fifo fillings."]
        #[inline(always)]
        pub fn set_rx0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RX1 fifo fillings."]
        #[inline(always)]
        pub const fn rx1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RX1 fifo fillings."]
        #[inline(always)]
        pub fn set_rx1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RX2 fifo fillings."]
        #[inline(always)]
        pub const fn rx2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RX2 fifo fillings."]
        #[inline(always)]
        pub fn set_rx2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "RX3 fifo fillings."]
        #[inline(always)]
        pub const fn rx3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RX3 fifo fillings."]
        #[inline(always)]
        pub fn set_rx3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RfifoFillings {
        #[inline(always)]
        fn default() -> RfifoFillings {
            RfifoFillings(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd(pub u32);
    impl Rxd {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxd {
        #[inline(always)]
        fn default() -> Rxd {
            Rxd(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdslot(pub u32);
    impl Rxdslot {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rxdslot {
        #[inline(always)]
        fn default() -> Rxdslot {
            Rxdslot(0)
        }
    }
    #[doc = "Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Asserted when rx fifo data are available."]
        #[inline(always)]
        pub const fn rx_da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when rx fifo data are available."]
        #[inline(always)]
        pub fn set_rx_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "Asserted when tx fifo data are needed."]
        #[inline(always)]
        pub const fn tx_dn(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when tx fifo data are needed."]
        #[inline(always)]
        pub fn set_tx_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
        #[inline(always)]
        pub const fn rx_ov(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
        #[inline(always)]
        pub fn set_rx_ov(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
        #[inline(always)]
        pub const fn tx_ud(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
        #[inline(always)]
        pub fn set_tx_ud(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    #[doc = "Tx FIFO Filling Level."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TfifoFillings(pub u32);
    impl TfifoFillings {
        #[doc = "TX0 fifo fillings."]
        #[inline(always)]
        pub const fn tx0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TX0 fifo fillings."]
        #[inline(always)]
        pub fn set_tx0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TX1 fifo fillings."]
        #[inline(always)]
        pub const fn tx1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "TX1 fifo fillings."]
        #[inline(always)]
        pub fn set_tx1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "TX2 fifo fillings."]
        #[inline(always)]
        pub const fn tx2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "TX2 fifo fillings."]
        #[inline(always)]
        pub fn set_tx2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "TX3 fifo fillings."]
        #[inline(always)]
        pub const fn tx3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "TX3 fifo fillings."]
        #[inline(always)]
        pub fn set_tx3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TfifoFillings {
        #[inline(always)]
        fn default() -> TfifoFillings {
            TfifoFillings(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd(pub u32);
    impl Txd {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txd {
        #[inline(always)]
        fn default() -> Txd {
            Txd(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdslot(pub u32);
    impl Txdslot {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Txdslot {
        #[inline(always)]
        fn default() -> Txdslot {
            Txdslot(0)
        }
    }
}
