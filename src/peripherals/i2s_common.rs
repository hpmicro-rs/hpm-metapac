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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Rx FIFO Filling Level."]
    #[inline(always)]
    pub const fn rfifo_fillings(
        self,
    ) -> crate::common::Reg<regs::RfifoFillings, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Tx FIFO Filling Level."]
    #[inline(always)]
    pub const fn tfifo_fillings(
        self,
    ) -> crate::common::Reg<regs::TfifoFillings, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TX/RX FIFO Threshold setting."]
    #[inline(always)]
    pub const fn fifo_thresh(self) -> crate::common::Reg<regs::FifoThresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status Registers."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxd(self, n: usize) -> crate::common::Reg<regs::Rxd, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txd(self, n: usize) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Configruation Regsiters."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Misc configuration Registers."]
    #[inline(always)]
    pub const fn misc_cfgr(self) -> crate::common::Reg<regs::MiscCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxdslot(self, n: usize) -> crate::common::Reg<regs::Rxdslot, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txdslot(self, n: usize) -> crate::common::Reg<regs::Txdslot, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 4usize) as _) }
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
    #[doc = "Configruation Regsiters."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATSIZ = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn chsiz(&self) -> super::vals::ChannelSize {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::ChannelSize::from_bits(val as u8)
        }
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATSIZ = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn set_chsiz(&mut self, val: super::vals::ChannelSize) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn datsiz(&self) -> super::vals::DataSize {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::DataSize::from_bits(val as u8)
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn set_datsiz(&mut self, val: super::vals::DataSize) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn std(&self) -> super::vals::Std {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Std::from_bits(val as u8)
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn set_std(&mut self, val: super::vals::Std) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tdm_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[inline(always)]
        pub const fn set_tdm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CH_MAX\\[4:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 5'h2: 2 channels 5'h4: 4 channels ... 5‘h10: 16 channels (max)."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_max(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "CH_MAX\\[4:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 5'h2: 2 channels 5'h4: 4 channels ... 5‘h10: 16 channels (max)."]
        #[inline(always)]
        pub const fn set_ch_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_edge(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[inline(always)]
        pub const fn set_frame_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "asserted to use external clk source."]
        #[must_use]
        #[inline(always)]
        pub const fn mck_sel_op(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn set_mck_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "asserted to use external clk source."]
        #[must_use]
        #[inline(always)]
        pub const fn fclk_sel_op(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn set_fclk_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "asserted to use external clk source."]
        #[must_use]
        #[inline(always)]
        pub const fn bclk_sel_op(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use external clk source."]
        #[inline(always)]
        pub const fn set_bclk_sel_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_mclk_in(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode."]
        #[inline(always)]
        pub const fn set_inv_mclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Invert the MCLK before sending it out to pad. Only valid in MCLK master mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_mclk_out(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the MCLK before sending it out to pad. Only valid in MCLK master mode."]
        #[inline(always)]
        pub const fn set_inv_mclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_fclk_in(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode."]
        #[inline(always)]
        pub const fn set_inv_fclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Invert the FCLK before sending it out to pad. Only valid in FCLK master mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_fclk_out(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the FCLK before sending it out to pad. Only valid in FCLK master mode."]
        #[inline(always)]
        pub const fn set_inv_fclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_bclk_in(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode."]
        #[inline(always)]
        pub const fn set_inv_bclk_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Invert the BCLK before sending it out to pad. Only valid in BCLK master mode."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_bclk_out(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Invert the BCLK before sending it out to pad. Only valid in BCLK master mode."]
        #[inline(always)]
        pub const fn set_inv_bclk_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[must_use]
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
        pub const fn set_bclk_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 21usize)) | (((val as u32) & 0x01ff) << 21usize);
        }
        #[doc = "Gate off the bclk. Asserted to gate-off the BCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn bclk_gateoff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Gate off the bclk. Asserted to gate-off the BCLK."]
        #[inline(always)]
        pub const fn set_bclk_gateoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("chsiz", &self.chsiz())
                .field("datsiz", &self.datsiz())
                .field("std", &self.std())
                .field("tdm_en", &self.tdm_en())
                .field("ch_max", &self.ch_max())
                .field("frame_edge", &self.frame_edge())
                .field("mck_sel_op", &self.mck_sel_op())
                .field("fclk_sel_op", &self.fclk_sel_op())
                .field("bclk_sel_op", &self.bclk_sel_op())
                .field("inv_mclk_in", &self.inv_mclk_in())
                .field("inv_mclk_out", &self.inv_mclk_out())
                .field("inv_fclk_in", &self.inv_fclk_in())
                .field("inv_fclk_out", &self.inv_fclk_out())
                .field("inv_bclk_in", &self.inv_bclk_in())
                .field("inv_bclk_out", &self.inv_bclk_out())
                .field("bclk_div", &self.bclk_div())
                .field("bclk_gateoff", &self.bclk_gateoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfgr {{ chsiz: {:?}, datsiz: {:?}, std: {:?}, tdm_en: {=bool:?}, ch_max: {=u8:?}, frame_edge: {=bool:?}, mck_sel_op: {=bool:?}, fclk_sel_op: {=bool:?}, bclk_sel_op: {=bool:?}, inv_mclk_in: {=bool:?}, inv_mclk_out: {=bool:?}, inv_fclk_in: {=bool:?}, inv_fclk_out: {=bool:?}, inv_bclk_in: {=bool:?}, inv_bclk_out: {=bool:?}, bclk_div: {=u16:?}, bclk_gateoff: {=bool:?} }}" , self . chsiz () , self . datsiz () , self . std () , self . tdm_en () , self . ch_max () , self . frame_edge () , self . mck_sel_op () , self . fclk_sel_op () , self . bclk_sel_op () , self . inv_mclk_in () , self . inv_mclk_out () , self . inv_fclk_in () , self . inv_fclk_out () , self . inv_bclk_in () , self . inv_bclk_out () , self . bclk_div () , self . bclk_gateoff ())
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "enable for the module."]
        #[must_use]
        #[inline(always)]
        pub const fn i2s_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable for the module."]
        #[inline(always)]
        pub const fn set_i2s_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enable for each RX data pad."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_en(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "enable for each RX data pad."]
        #[inline(always)]
        pub const fn set_rx_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "enable for each TX data pad."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_en(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "enable for each TX data pad."]
        #[inline(always)]
        pub const fn set_tx_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoclr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn set_rxfifoclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoclr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn set_txfifoclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_dma_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub const fn set_rx_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dma_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use DMA, else to use interrupt."]
        #[inline(always)]
        pub const fn set_tx_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdaie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
        #[inline(always)]
        pub const fn set_rxdaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn txdnie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
        #[inline(always)]
        pub const fn set_txdnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst_clkgen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst_clkgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "software reset the TX module if asserted to be 1'b1. Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst_tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the TX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "software reset the RX module if asserted to be 1'b1. Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst_rx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the RX module if asserted to be 1'b1. Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("i2s_en", &self.i2s_en())
                .field("rx_en", &self.rx_en())
                .field("tx_en", &self.tx_en())
                .field("rxfifoclr", &self.rxfifoclr())
                .field("txfifoclr", &self.txfifoclr())
                .field("rx_dma_en", &self.rx_dma_en())
                .field("tx_dma_en", &self.tx_dma_en())
                .field("errie", &self.errie())
                .field("rxdaie", &self.rxdaie())
                .field("txdnie", &self.txdnie())
                .field("sftrst_clkgen", &self.sftrst_clkgen())
                .field("sftrst_tx", &self.sftrst_tx())
                .field("sftrst_rx", &self.sftrst_rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ i2s_en: {=bool:?}, rx_en: {=u8:?}, tx_en: {=u8:?}, rxfifoclr: {=bool:?}, txfifoclr: {=bool:?}, rx_dma_en: {=bool:?}, tx_dma_en: {=bool:?}, errie: {=bool:?}, rxdaie: {=bool:?}, txdnie: {=bool:?}, sftrst_clkgen: {=bool:?}, sftrst_tx: {=bool:?}, sftrst_rx: {=bool:?} }}" , self . i2s_en () , self . rx_en () , self . tx_en () , self . rxfifoclr () , self . txfifoclr () , self . rx_dma_en () , self . tx_dma_en () , self . errie () , self . rxdaie () , self . txdnie () , self . sftrst_clkgen () , self . sftrst_tx () , self . sftrst_rx ())
        }
    }
    #[doc = "TX/RX FIFO Threshold setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoThresh(pub u32);
    impl FifoThresh {
        #[doc = "RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
        #[inline(always)]
        pub const fn set_rx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
        #[inline(always)]
        pub const fn set_tx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for FifoThresh {
        #[inline(always)]
        fn default() -> FifoThresh {
            FifoThresh(0)
        }
    }
    impl core::fmt::Debug for FifoThresh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FifoThresh")
                .field("rx", &self.rx())
                .field("tx", &self.tx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FifoThresh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FifoThresh {{ rx: {=u8:?}, tx: {=u8:?} }}",
                self.rx(),
                self.tx()
            )
        }
    }
    #[doc = "Misc configuration Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscCfgr(pub u32);
    impl MiscCfgr {
        #[doc = "Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mclkoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
        #[inline(always)]
        pub const fn set_mclkoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
        #[must_use]
        #[inline(always)]
        pub const fn mclk_gateoff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
        #[inline(always)]
        pub const fn set_mclk_gateoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for MiscCfgr {
        #[inline(always)]
        fn default() -> MiscCfgr {
            MiscCfgr(0)
        }
    }
    impl core::fmt::Debug for MiscCfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiscCfgr")
                .field("mclkoe", &self.mclkoe())
                .field("mclk_gateoff", &self.mclk_gateoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiscCfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MiscCfgr {{ mclkoe: {=bool:?}, mclk_gateoff: {=bool:?} }}",
                self.mclkoe(),
                self.mclk_gateoff()
            )
        }
    }
    #[doc = "Rx FIFO Filling Level."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RfifoFillings(pub u32);
    impl RfifoFillings {
        #[doc = "RX0 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn rx0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RX0 fifo fillings."]
        #[inline(always)]
        pub const fn set_rx0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RX1 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn rx1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RX1 fifo fillings."]
        #[inline(always)]
        pub const fn set_rx1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RX2 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn rx2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RX2 fifo fillings."]
        #[inline(always)]
        pub const fn set_rx2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "RX3 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn rx3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RX3 fifo fillings."]
        #[inline(always)]
        pub const fn set_rx3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RfifoFillings {
        #[inline(always)]
        fn default() -> RfifoFillings {
            RfifoFillings(0)
        }
    }
    impl core::fmt::Debug for RfifoFillings {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RfifoFillings")
                .field("rx0", &self.rx0())
                .field("rx1", &self.rx1())
                .field("rx2", &self.rx2())
                .field("rx3", &self.rx3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RfifoFillings {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RfifoFillings {{ rx0: {=u8:?}, rx1: {=u8:?}, rx2: {=u8:?}, rx3: {=u8:?} }}",
                self.rx0(),
                self.rx1(),
                self.rx2(),
                self.rx3()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxd(pub u32);
    impl Rxd {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxd {
        #[inline(always)]
        fn default() -> Rxd {
            Rxd(0)
        }
    }
    impl core::fmt::Debug for Rxd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxd").field("d", &self.d()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxd {{ d: {=u32:?} }}", self.d())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdslot(pub u32);
    impl Rxdslot {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rxdslot {
        #[inline(always)]
        fn default() -> Rxdslot {
            Rxdslot(0)
        }
    }
    impl core::fmt::Debug for Rxdslot {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdslot").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdslot {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdslot {{ en: {=u16:?} }}", self.en())
        }
    }
    #[doc = "Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Asserted when rx fifo data are available."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when rx fifo data are available."]
        #[inline(always)]
        pub const fn set_rx_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "Asserted when tx fifo data are needed."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dn(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when tx fifo data are needed."]
        #[inline(always)]
        pub const fn set_tx_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_ov(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
        #[inline(always)]
        pub const fn set_rx_ov(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ud(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
        #[inline(always)]
        pub const fn set_tx_ud(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    impl core::fmt::Debug for Sta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sta")
                .field("rx_da", &self.rx_da())
                .field("tx_dn", &self.tx_dn())
                .field("rx_ov", &self.rx_ov())
                .field("tx_ud", &self.tx_ud())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sta {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sta {{ rx_da: {=u8:?}, tx_dn: {=u8:?}, rx_ov: {=u8:?}, tx_ud: {=u8:?} }}",
                self.rx_da(),
                self.tx_dn(),
                self.rx_ov(),
                self.tx_ud()
            )
        }
    }
    #[doc = "Tx FIFO Filling Level."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TfifoFillings(pub u32);
    impl TfifoFillings {
        #[doc = "TX0 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn tx0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TX0 fifo fillings."]
        #[inline(always)]
        pub const fn set_tx0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TX1 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn tx1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "TX1 fifo fillings."]
        #[inline(always)]
        pub const fn set_tx1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "TX2 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn tx2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "TX2 fifo fillings."]
        #[inline(always)]
        pub const fn set_tx2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "TX3 fifo fillings."]
        #[must_use]
        #[inline(always)]
        pub const fn tx3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "TX3 fifo fillings."]
        #[inline(always)]
        pub const fn set_tx3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TfifoFillings {
        #[inline(always)]
        fn default() -> TfifoFillings {
            TfifoFillings(0)
        }
    }
    impl core::fmt::Debug for TfifoFillings {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TfifoFillings")
                .field("tx0", &self.tx0())
                .field("tx1", &self.tx1())
                .field("tx2", &self.tx2())
                .field("tx3", &self.tx3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TfifoFillings {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TfifoFillings {{ tx0: {=u8:?}, tx1: {=u8:?}, tx2: {=u8:?}, tx3: {=u8:?} }}",
                self.tx0(),
                self.tx1(),
                self.tx2(),
                self.tx3()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txd(pub u32);
    impl Txd {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txd {
        #[inline(always)]
        fn default() -> Txd {
            Txd(0)
        }
    }
    impl core::fmt::Debug for Txd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txd").field("d", &self.d()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txd {{ d: {=u32:?} }}", self.d())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdslot(pub u32);
    impl Txdslot {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Txdslot {
        #[inline(always)]
        fn default() -> Txdslot {
            Txdslot(0)
        }
    }
    impl core::fmt::Debug for Txdslot {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdslot").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdslot {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txdslot {{ en: {=u16:?} }}", self.en())
        }
    }
}
pub mod vals {
    #[doc = "Channel length."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ChannelSize {
        #[doc = "16-bit wide"]
        _16BIT = 0x0,
        #[doc = "32-bit wide"]
        _32BIT = 0x01,
    }
    impl ChannelSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ChannelSize {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ChannelSize {
        #[inline(always)]
        fn from(val: u8) -> ChannelSize {
            ChannelSize::from_bits(val)
        }
    }
    impl From<ChannelSize> for u8 {
        #[inline(always)]
        fn from(val: ChannelSize) -> u8 {
            ChannelSize::to_bits(val)
        }
    }
    #[doc = "Data length to be transferred."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DataSize {
        #[doc = "16-bit data length"]
        _16BIT = 0x0,
        #[doc = "24-bit data length"]
        _24BIT = 0x01,
        #[doc = "32-bit data length"]
        _32BIT = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl DataSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataSize {
        #[inline(always)]
        fn from(val: u8) -> DataSize {
            DataSize::from_bits(val)
        }
    }
    impl From<DataSize> for u8 {
        #[inline(always)]
        fn from(val: DataSize) -> u8 {
            DataSize::to_bits(val)
        }
    }
    #[doc = "I2S standard selection."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Std {
        #[doc = "I2S Philips"]
        PHILIPS = 0x0,
        #[doc = "MSB fist"]
        MSB = 0x01,
        #[doc = "LSB first"]
        LSB = 0x02,
        #[doc = "PCM"]
        PCM = 0x03,
    }
    impl Std {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Std {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Std {
        #[inline(always)]
        fn from(val: u8) -> Std {
            Std::from_bits(val)
        }
    }
    impl From<Std> for u8 {
        #[inline(always)]
        fn from(val: Std) -> u8 {
            Std::to_bits(val)
        }
    }
}
