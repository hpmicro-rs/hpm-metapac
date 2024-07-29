#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DAC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
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
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn step_cfg(self, n: usize) -> crate::common::Reg<regs::StepCfg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn buf_addr(self, n: usize) -> crate::common::Reg<regs::BufAddr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn buf_length(self) -> crate::common::Reg<regs::BufLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts(self) -> crate::common::Reg<regs::IrqSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en(self) -> crate::common::Reg<regs::IrqEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn dma_en(self) -> crate::common::Reg<regs::DmaEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ana_cfg0(self) -> crate::common::Reg<regs::AnaCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg0_bak(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn status0(self) -> crate::common::Reg<regs::Status0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AnaCfg0(pub u32);
    impl AnaCfg0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac12bit_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac12bit_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn bypass_cali_gm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_bypass_cali_gm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cali_delta_v_cfg(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cali_delta_v_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac_config(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac_config(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac12bit_lp_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac12bit_lp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for AnaCfg0 {
        #[inline(always)]
        fn default() -> AnaCfg0 {
            AnaCfg0(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufAddr(pub u32);
    impl BufAddr {
        #[doc = "set to stop read point at end of bufffer0."]
        #[inline(always)]
        pub const fn buf_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to stop read point at end of bufffer0."]
        #[inline(always)]
        pub fn set_buf_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
        #[inline(always)]
        pub const fn buf_start_addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue."]
        #[inline(always)]
        pub fn set_buf_start_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for BufAddr {
        #[inline(always)]
        fn default() -> BufAddr {
            BufAddr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufLength(pub u32);
    impl BufLength {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf0_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf0_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer."]
        #[inline(always)]
        pub const fn buf1_len(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer."]
        #[inline(always)]
        pub fn set_buf1_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BufLength {
        #[inline(always)]
        fn default() -> BufLength {
            BufLength(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved."]
        #[inline(always)]
        pub const fn hburst_cfg(&self) -> super::vals::HburstCfg {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::HburstCfg::from_bits(val as u8)
        }
        #[doc = "DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved."]
        #[inline(always)]
        pub fn set_hburst_cfg(&mut self, val: super::vals::HburstCfg) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first."]
        #[inline(always)]
        pub const fn buf_data_mode(&self) -> super::vals::BufDataMode {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::BufDataMode::from_bits(val as u8)
        }
        #[doc = "data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first."]
        #[inline(always)]
        pub fn set_buf_data_mode(&mut self, val: super::vals::BufDataMode) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO; 11: trigger mode, DAC output from external trigger signals Note: Trigger mode is not supported in hpm63xx and hpm62xx families."]
        #[inline(always)]
        pub const fn dac_mode(&self) -> super::vals::DacMode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::DacMode::from_bits(val as u8)
        }
        #[doc = "00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO; 11: trigger mode, DAC output from external trigger signals Note: Trigger mode is not supported in hpm63xx and hpm62xx families."]
        #[inline(always)]
        pub fn set_dac_mode(&mut self, val: super::vals::DacMode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode."]
        #[inline(always)]
        pub const fn hw_trig_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode."]
        #[inline(always)]
        pub fn set_hw_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
        #[inline(always)]
        pub const fn trig_mode(&self) -> super::vals::TrigMode {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::TrigMode::from_bits(val as u8)
        }
        #[doc = "0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data."]
        #[inline(always)]
        pub fn set_trig_mode(&mut self, val: super::vals::TrigMode) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)."]
        #[inline(always)]
        pub const fn sync_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock)."]
        #[inline(always)]
        pub fn set_sync_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
        #[inline(always)]
        pub const fn dma_ahb_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode."]
        #[inline(always)]
        pub fn set_dma_ahb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "dac data used in direct mode(dac_mode==2'b10)."]
        #[inline(always)]
        pub const fn sw_dac_data(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "dac data used in direct mode(dac_mode==2'b10)."]
        #[inline(always)]
        pub fn set_sw_dac_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "step mode and buffer mode: defines how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Direct mode and trigger mode: defines how many clk_dac cycles to accpet the input data, dac will not accept new written data or trigger data before the clock cycles passed. should configured to less than 1MHz. Note: For direct mode and trigger mode, this config is not supported in hpm63xx and hpm62xx families."]
        #[inline(always)]
        pub const fn div_cfg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "step mode and buffer mode: defines how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Direct mode and trigger mode: defines how many clk_dac cycles to accpet the input data, dac will not accept new written data or trigger data before the clock cycles passed. should configured to less than 1MHz. Note: For direct mode and trigger mode, this config is not supported in hpm63xx and hpm62xx families."]
        #[inline(always)]
        pub fn set_div_cfg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8."]
        #[inline(always)]
        pub const fn ana_div_cfg(&self) -> super::vals::AnaDiv {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::AnaDiv::from_bits(val as u8)
        }
        #[doc = "clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8."]
        #[inline(always)]
        pub fn set_ana_div_cfg(&mut self, val: super::vals::AnaDiv) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "set to enable analog clock(divided by ana_div_cfg) need to be set in direct mode and trigger mode."]
        #[inline(always)]
        pub const fn ana_clk_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable analog clock(divided by ana_div_cfg) need to be set in direct mode and trigger mode."]
        #[inline(always)]
        pub fn set_ana_clk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "software trigger0 for step mode, W1C in single mode. RW in continual mode."]
        #[inline(always)]
        pub const fn step_sw_trig(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "software trigger0 for step mode, W1C in single mode. RW in continual mode."]
        #[inline(always)]
        pub fn set_step_sw_trig(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "software trigger for buffer mode, W1C in single mode. RW in continual mode."]
        #[inline(always)]
        pub const fn buf_sw_trig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "software trigger for buffer mode, W1C in single mode. RW in continual mode."]
        #[inline(always)]
        pub fn set_buf_sw_trig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to clear FIFO content(set both read/write pointer to 0)."]
        #[inline(always)]
        pub const fn fifo_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to clear FIFO content(set both read/write pointer to 0)."]
        #[inline(always)]
        pub fn set_fifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "set to reset dma read pointer to buf0_start_addr."]
        #[inline(always)]
        pub const fn dma_rst0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "set to reset dma read pointer to buf0_start_addr."]
        #[inline(always)]
        pub fn set_dma_rst0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "set to reset dma read pointer to buf1_start_addr; if set both dma_rst0&dma_rst1, will set to buf0_start_addr user can set fifo_clr bit when use dma_rst*."]
        #[inline(always)]
        pub const fn dma_rst1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set to reset dma read pointer to buf1_start_addr; if set both dma_rst0&dma_rst1, will set to buf0_start_addr user can set fifo_clr bit when use dma_rst*."]
        #[inline(always)]
        pub fn set_dma_rst1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEn(pub u32);
    impl DmaEn {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf0_cmpt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf0_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf1_cmpt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf1_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DmaEn {
        #[inline(always)]
        fn default() -> DmaEn {
            DmaEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEn(pub u32);
    impl IrqEn {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf0_cmpt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf0_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf1_cmpt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf1_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn fifo_empty(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_fifo_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ahb_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ahb_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn step_cmpt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_step_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for IrqEn {
        #[inline(always)]
        fn default() -> IrqEn {
            IrqEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSts(pub u32);
    impl IrqSts {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf0_cmpt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf0_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf1_cmpt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf1_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn fifo_empty(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_fifo_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set if hresp==2'b01(ERROR)."]
        #[inline(always)]
        pub const fn ahb_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set if hresp==2'b01(ERROR)."]
        #[inline(always)]
        pub fn set_ahb_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for IrqSts {
        #[inline(always)]
        fn default() -> IrqSts {
            IrqSts(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status0(pub u32);
    impl Status0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cur_buf_index(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cur_buf_index(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cur_buf_offset(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cur_buf_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
        }
    }
    impl Default for Status0 {
        #[inline(always)]
        fn default() -> Status0 {
            Status0(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StepCfg(pub u32);
    impl StepCfg {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn start_point(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_start_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point."]
        #[inline(always)]
        pub const fn step_num(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point."]
        #[inline(always)]
        pub fn set_step_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn end_point(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_end_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "0 for up, 1 for down."]
        #[inline(always)]
        pub const fn up_down(&self) -> super::vals::StepDir {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::StepDir::from_bits(val as u8)
        }
        #[doc = "0 for up, 1 for down."]
        #[inline(always)]
        pub fn set_up_down(&mut self, val: super::vals::StepDir) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "0: stop at end point; 1: reload start point, step again."]
        #[inline(always)]
        pub const fn round_mode(&self) -> super::vals::RoundMode {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::RoundMode::from_bits(val as u8)
        }
        #[doc = "0: stop at end point; 1: reload start point, step again."]
        #[inline(always)]
        pub fn set_round_mode(&mut self, val: super::vals::RoundMode) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
    }
    impl Default for StepCfg {
        #[inline(always)]
        fn default() -> StepCfg {
            StepCfg(0)
        }
    }
}
pub mod vals {
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum AnaDiv {
        DIV2 = 0x0,
        DIV4 = 0x01,
        DIV6 = 0x02,
        DIV8 = 0x03,
    }
    impl AnaDiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AnaDiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AnaDiv {
        #[inline(always)]
        fn from(val: u8) -> AnaDiv {
            AnaDiv::from_bits(val)
        }
    }
    impl From<AnaDiv> for u8 {
        #[inline(always)]
        fn from(val: AnaDiv) -> u8 {
            AnaDiv::to_bits(val)
        }
    }
    #[doc = "Format of buffer data."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BufDataMode {
        TWO_POINTS = 0x0,
        ONE_POINT = 0x01,
    }
    impl BufDataMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BufDataMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BufDataMode {
        #[inline(always)]
        fn from(val: u8) -> BufDataMode {
            BufDataMode::from_bits(val)
        }
    }
    impl From<BufDataMode> for u8 {
        #[inline(always)]
        fn from(val: BufDataMode) -> u8 {
            BufDataMode::to_bits(val)
        }
    }
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum DacMode {
        DIRECT = 0x0,
        STEP = 0x01,
        BUFFER = 0x02,
        TRIGGER = 0x03,
    }
    impl DacMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DacMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DacMode {
        #[inline(always)]
        fn from(val: u8) -> DacMode {
            DacMode::from_bits(val)
        }
    }
    impl From<DacMode> for u8 {
        #[inline(always)]
        fn from(val: DacMode) -> u8 {
            DacMode::to_bits(val)
        }
    }
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HburstCfg {
        SINGLE = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        INCR4 = 0x03,
        _RESERVED_4 = 0x04,
        INCR8 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl HburstCfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HburstCfg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HburstCfg {
        #[inline(always)]
        fn from(val: u8) -> HburstCfg {
            HburstCfg::from_bits(val)
        }
    }
    impl From<HburstCfg> for u8 {
        #[inline(always)]
        fn from(val: HburstCfg) -> u8 {
            HburstCfg::to_bits(val)
        }
    }
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RoundMode {
        STOP = 0x0,
        RELOAD = 0x01,
    }
    impl RoundMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RoundMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RoundMode {
        #[inline(always)]
        fn from(val: u8) -> RoundMode {
            RoundMode::from_bits(val)
        }
    }
    impl From<RoundMode> for u8 {
        #[inline(always)]
        fn from(val: RoundMode) -> u8 {
            RoundMode::to_bits(val)
        }
    }
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum StepDir {
        UP = 0x0,
        DOWN = 0x01,
    }
    impl StepDir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> StepDir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for StepDir {
        #[inline(always)]
        fn from(val: u8) -> StepDir {
            StepDir::from_bits(val)
        }
    }
    impl From<StepDir> for u8 {
        #[inline(always)]
        fn from(val: StepDir) -> u8 {
            StepDir::to_bits(val)
        }
    }
    #[doc = "No description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TrigMode {
        SINGLE = 0x0,
        CONTINUAL = 0x01,
    }
    impl TrigMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TrigMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TrigMode {
        #[inline(always)]
        fn from(val: u8) -> TrigMode {
            TrigMode::from_bits(val)
        }
    }
    impl From<TrigMode> for u8 {
        #[inline(always)]
        fn from(val: TrigMode) -> u8 {
            TrigMode::to_bits(val)
        }
    }
}
