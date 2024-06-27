#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn trg_dma_addr(self) -> crate::common::Reg<regs::TrgDmaAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn trg_sw_sta(self) -> crate::common::Reg<regs::TrgSwSta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn bus_result(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::BusResult, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn buf_cfg0(self) -> crate::common::Reg<regs::BufCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn seq_cfg0(self) -> crate::common::Reg<regs::SeqCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn seq_dma_addr(self) -> crate::common::Reg<regs::SeqDmaAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn seq_wr_addr(self) -> crate::common::Reg<regs::SeqWrAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn seq_dma_cfg(self) -> crate::common::Reg<regs::SeqDmaCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn seq_que(self, n: usize) -> crate::common::Reg<regs::SeqQue, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0810usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn seq_high_cfg(self) -> crate::common::Reg<regs::SeqHighCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0850usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn prd_cfg(self, n: usize) -> PrdCfg {
        assert!(n < 16usize);
        unsafe { PrdCfg::from_ptr(self.ptr.add(0x0c00usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn sample_cfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SampleCfg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn conv_cfg1(self) -> crate::common::Reg<regs::ConvCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1104usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn adc_cfg0(self) -> crate::common::Reg<regs::AdcCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1108usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_sts(self) -> crate::common::Reg<regs::IntSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1110usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1114usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn trigmux_en(self) -> crate::common::Reg<regs::TrigmuxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1118usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn trg_cmpt_flag(self) -> crate::common::Reg<regs::TrgCmptFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x111cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ana_ctrl0(self) -> crate::common::Reg<regs::AnaCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1200usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ana_status(self) -> crate::common::Reg<regs::AnaStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1210usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn adc16_params(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Adc16Params, crate::common::RW> {
        assert!(n < 34usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1400usize + n * 2usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn adc16_config0(self) -> crate::common::Reg<regs::Adc16Config0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1444usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn adc16_config1(self) -> crate::common::Reg<regs::Adc16Config1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1460usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrdCfg {
    ptr: *mut u8,
}
unsafe impl Send for PrdCfg {}
unsafe impl Sync for PrdCfg {}
impl PrdCfg {
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
    pub const fn prd_cfg(self) -> crate::common::Reg<regs::PrdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn prd_thshd_cfg(self) -> crate::common::Reg<regs::PrdThshdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn prd_result(self) -> crate::common::Reg<regs::PrdResult, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc16Config0(pub u32);
    impl Adc16Config0 {
        #[doc = "conversion parameter."]
        #[inline(always)]
        pub const fn conv_param(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "conversion parameter."]
        #[inline(always)]
        pub fn set_conv_param(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "set to enable preemption feature."]
        #[inline(always)]
        pub const fn preempt_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable preemption feature."]
        #[inline(always)]
        pub fn set_preempt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved."]
        #[inline(always)]
        pub const fn cal_avg_cfg(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved."]
        #[inline(always)]
        pub fn set_cal_avg_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
        #[inline(always)]
        pub const fn bandgap_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
        #[inline(always)]
        pub fn set_bandgap_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "set to enable regulator."]
        #[inline(always)]
        pub const fn reg_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable regulator."]
        #[inline(always)]
        pub fn set_reg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Adc16Config0 {
        #[inline(always)]
        fn default() -> Adc16Config0 {
            Adc16Config0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc16Config1(pub u32);
    impl Adc16Config1 {
        #[doc = "used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
        #[inline(always)]
        pub const fn cov_end_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1)."]
        #[inline(always)]
        pub fn set_cov_end_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Adc16Config1 {
        #[inline(always)]
        fn default() -> Adc16Config1 {
            Adc16Config1(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adc16Params(pub u16);
    impl Adc16Params {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn param_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_param_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Adc16Params {
        #[inline(always)]
        fn default() -> Adc16Params {
            Adc16Params(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcCfg0(pub u32);
    impl AdcCfg0 {
        #[doc = "set to enable trg queue stop other queues."]
        #[inline(always)]
        pub const fn port3_realtime(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable trg queue stop other queues."]
        #[inline(always)]
        pub fn set_port3_realtime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;."]
        #[inline(always)]
        pub const fn adc_ahb_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;."]
        #[inline(always)]
        pub fn set_adc_ahb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode."]
        #[inline(always)]
        pub const fn sel_sync_ahb(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode."]
        #[inline(always)]
        pub fn set_sel_sync_ahb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AdcCfg0 {
        #[inline(always)]
        fn default() -> AdcCfg0 {
            AdcCfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AnaCtrl0(pub u32);
    impl AnaCtrl0 {
        #[doc = "set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
        #[inline(always)]
        pub const fn startcal(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
        #[inline(always)]
        pub fn set_startcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access."]
        #[inline(always)]
        pub const fn adc_clk_on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access."]
        #[inline(always)]
        pub fn set_adc_clk_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "\"set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output\"."]
        #[inline(always)]
        pub const fn moto_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "\"set to enable moto_soc and moto_valid. Should use AHB clock for adc, this bit can be used avoid async output\"."]
        #[inline(always)]
        pub fn set_moto_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AnaCtrl0 {
        #[inline(always)]
        fn default() -> AnaCtrl0 {
            AnaCtrl0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AnaStatus(pub u32);
    impl AnaStatus {
        #[doc = "Indicates if the ADC is in calibration mode (Active H)."]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if the ADC is in calibration mode (Active H)."]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for AnaStatus {
        #[inline(always)]
        fn default() -> AnaStatus {
            AnaStatus(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufCfg0(pub u32);
    impl BufCfg0 {
        #[doc = "set to disable read waiting, get result immediately but maybe not current conversion result."]
        #[inline(always)]
        pub const fn wait_dis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to disable read waiting, get result immediately but maybe not current conversion result."]
        #[inline(always)]
        pub fn set_wait_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "bus mode enable."]
        #[inline(always)]
        pub const fn bus_mode_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "bus mode enable."]
        #[inline(always)]
        pub fn set_bus_mode_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BufCfg0 {
        #[inline(always)]
        fn default() -> BufCfg0 {
            BufCfg0(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusResult(pub u32);
    impl BusResult {
        #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long."]
        #[inline(always)]
        pub const fn chan_result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long."]
        #[inline(always)]
        pub fn set_chan_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for BusResult {
        #[inline(always)]
        fn default() -> BusResult {
            BusResult(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "channel number for 1st conversion."]
        #[inline(always)]
        pub const fn chan0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "channel number for 1st conversion."]
        #[inline(always)]
        pub fn set_chan0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "interrupt enable for 1st conversion."]
        #[inline(always)]
        pub const fn inten0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for 1st conversion."]
        #[inline(always)]
        pub fn set_inten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "preemption queue enable control."]
        #[inline(always)]
        pub const fn queue_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "preemption queue enable control."]
        #[inline(always)]
        pub fn set_queue_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "channel number for 2nd conversion."]
        #[inline(always)]
        pub const fn chan1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "channel number for 2nd conversion."]
        #[inline(always)]
        pub fn set_chan1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "interrupt enable for 2nd conversion."]
        #[inline(always)]
        pub const fn inten1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for 2nd conversion."]
        #[inline(always)]
        pub fn set_inten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "channel number for 3rd conversion."]
        #[inline(always)]
        pub const fn chan2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "channel number for 3rd conversion."]
        #[inline(always)]
        pub fn set_chan2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "interrupt enable for 3rd conversion."]
        #[inline(always)]
        pub const fn inten2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for 3rd conversion."]
        #[inline(always)]
        pub fn set_inten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "channel number for 4th conversion."]
        #[inline(always)]
        pub const fn chan3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "channel number for 4th conversion."]
        #[inline(always)]
        pub fn set_chan3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "interrupt enable for 4th conversion."]
        #[inline(always)]
        pub const fn inten3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for 4th conversion."]
        #[inline(always)]
        pub fn set_inten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3."]
        #[inline(always)]
        pub const fn trig_len(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3."]
        #[inline(always)]
        pub fn set_trig_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConvCfg1(pub u32);
    impl ConvCfg1 {
        #[doc = "clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk."]
        #[inline(always)]
        pub const fn clock_divider(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk."]
        #[inline(always)]
        pub fn set_clock_divider(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz)."]
        #[inline(always)]
        pub const fn convert_clock_number(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz)."]
        #[inline(always)]
        pub fn set_convert_clock_number(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
    }
    impl Default for ConvCfg1 {
        #[inline(always)]
        fn default() -> ConvCfg1 {
            ConvCfg1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "set if one chanel watch dog event triggered."]
        #[inline(always)]
        pub const fn wdog(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "set if one chanel watch dog event triggered."]
        #[inline(always)]
        pub fn set_wdog(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn stop_pos(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_stop_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr."]
        #[inline(always)]
        pub const fn ahb_err(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr."]
        #[inline(always)]
        pub fn set_ahb_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA fifo full interrupt, user need to check clock frequency if it's set."]
        #[inline(always)]
        pub const fn dma_fifo_full(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA fifo full interrupt, user need to check clock frequency if it's set."]
        #[inline(always)]
        pub fn set_dma_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "one conversion complete in seq_queue if related seq_int_en is set."]
        #[inline(always)]
        pub const fn seq_cvc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "one conversion complete in seq_queue if related seq_int_en is set."]
        #[inline(always)]
        pub fn set_seq_cvc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "the whole sequence complete interrupt."]
        #[inline(always)]
        pub const fn seq_cmpt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "the whole sequence complete interrupt."]
        #[inline(always)]
        pub fn set_seq_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set."]
        #[inline(always)]
        pub const fn seq_dmaabt(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set."]
        #[inline(always)]
        pub fn set_seq_dmaabt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn seq_hw_cflct(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_seq_hw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "sequence queue conflict interrupt, set if HW or SW trigger received during conversion."]
        #[inline(always)]
        pub const fn seq_sw_cflct(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "sequence queue conflict interrupt, set if HW or SW trigger received during conversion."]
        #[inline(always)]
        pub fn set_seq_sw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel."]
        #[inline(always)]
        pub const fn read_cflct(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel."]
        #[inline(always)]
        pub fn set_read_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn trig_hw_cflct(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_trig_hw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn trig_sw_cflct(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_trig_sw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "interrupt for one trigger conversion complete if enabled."]
        #[inline(always)]
        pub const fn trig_cmpt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt for one trigger conversion complete if enabled."]
        #[inline(always)]
        pub fn set_trig_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSts(pub u32);
    impl IntSts {
        #[doc = "set if one chanel watch dog event triggered."]
        #[inline(always)]
        pub const fn wdog(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "set if one chanel watch dog event triggered."]
        #[inline(always)]
        pub fn set_wdog(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn stop_pos(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_stop_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr."]
        #[inline(always)]
        pub const fn ahb_err(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr."]
        #[inline(always)]
        pub fn set_ahb_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA fifo full interrupt, user need to check clock frequency if it's set."]
        #[inline(always)]
        pub const fn dma_fifo_full(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA fifo full interrupt, user need to check clock frequency if it's set."]
        #[inline(always)]
        pub fn set_dma_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "one conversion complete in seq_queue if related seq_int_en is set."]
        #[inline(always)]
        pub const fn seq_cvc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "one conversion complete in seq_queue if related seq_int_en is set."]
        #[inline(always)]
        pub fn set_seq_cvc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "the whole sequence complete interrupt."]
        #[inline(always)]
        pub const fn seq_cmpt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "the whole sequence complete interrupt."]
        #[inline(always)]
        pub fn set_seq_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set."]
        #[inline(always)]
        pub const fn seq_dmaabt(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set."]
        #[inline(always)]
        pub fn set_seq_dmaabt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn seq_hw_cflct(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_seq_hw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "sequence queue conflict interrupt, set if HW or SW trigger received during conversion."]
        #[inline(always)]
        pub const fn seq_sw_cflct(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "sequence queue conflict interrupt, set if HW or SW trigger received during conversion."]
        #[inline(always)]
        pub fn set_seq_sw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel."]
        #[inline(always)]
        pub const fn read_cflct(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel."]
        #[inline(always)]
        pub fn set_read_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn trig_hw_cflct(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_trig_hw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn trig_sw_cflct(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_trig_sw_cflct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "interrupt for one trigger conversion complete if enabled."]
        #[inline(always)]
        pub const fn trig_cmpt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt for one trigger conversion complete if enabled."]
        #[inline(always)]
        pub fn set_trig_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IntSts {
        #[inline(always)]
        fn default() -> IntSts {
            IntSts(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrdCfg(pub u32);
    impl PrdCfg {
        #[doc = "conver period, with prescale. Set to 0 means disable current channel."]
        #[inline(always)]
        pub const fn prd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "conver period, with prescale. Set to 0 means disable current channel."]
        #[inline(always)]
        pub fn set_prd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx."]
        #[inline(always)]
        pub const fn prescale(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx."]
        #[inline(always)]
        pub fn set_prescale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for PrdCfg {
        #[inline(always)]
        fn default() -> PrdCfg {
            PrdCfg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrdResult(pub u32);
    impl PrdResult {
        #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel."]
        #[inline(always)]
        pub const fn chan_result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel."]
        #[inline(always)]
        pub fn set_chan_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PrdResult {
        #[inline(always)]
        fn default() -> PrdResult {
            PrdResult(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrdThshdCfg(pub u32);
    impl PrdThshdCfg {
        #[doc = "threshold low."]
        #[inline(always)]
        pub const fn thshdl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "threshold low."]
        #[inline(always)]
        pub fn set_thshdl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
        #[inline(always)]
        pub const fn thshdh(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
        #[inline(always)]
        pub fn set_thshdh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PrdThshdCfg {
        #[inline(always)]
        fn default() -> PrdThshdCfg {
            PrdThshdCfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleCfg(pub u32);
    impl SampleCfg {
        #[doc = "sample clock number, base on clock_period, default one period."]
        #[inline(always)]
        pub const fn sample_clock_number(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "sample clock number, base on clock_period, default one period."]
        #[inline(always)]
        pub fn set_sample_clock_number(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "shift for sample clock number."]
        #[inline(always)]
        pub const fn sample_clock_number_shift(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "shift for sample clock number."]
        #[inline(always)]
        pub fn set_sample_clock_number_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
    }
    impl Default for SampleCfg {
        #[inline(always)]
        fn default() -> SampleCfg {
            SampleCfg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqCfg0(pub u32);
    impl SeqCfg0 {
        #[doc = "set to enable external HW trigger, only trigger on posedge."]
        #[inline(always)]
        pub const fn hw_trig_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable external HW trigger, only trigger on posedge."]
        #[inline(always)]
        pub fn set_hw_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "set to enable SW trigger."]
        #[inline(always)]
        pub const fn sw_trig_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable SW trigger."]
        #[inline(always)]
        pub fn set_sw_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SW trigger, pulse signal, cleared by HW one cycle later."]
        #[inline(always)]
        pub const fn sw_trig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SW trigger, pulse signal, cleared by HW one cycle later."]
        #[inline(always)]
        pub fn set_sw_trig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "if set, HW will continue process the queue till end(seq_len) after trigger once."]
        #[inline(always)]
        pub const fn cont_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "if set, HW will continue process the queue till end(seq_len) after trigger once."]
        #[inline(always)]
        pub fn set_cont_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used."]
        #[inline(always)]
        pub const fn restart_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used."]
        #[inline(always)]
        pub fn set_restart_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "sequence queue length, 0 for one, 0xF for 16."]
        #[inline(always)]
        pub const fn seq_len(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "sequence queue length, 0 for one, 0xF for 16."]
        #[inline(always)]
        pub fn set_seq_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "current dma write cycle bit."]
        #[inline(always)]
        pub const fn cycle(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "current dma write cycle bit."]
        #[inline(always)]
        pub fn set_cycle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SeqCfg0 {
        #[inline(always)]
        fn default() -> SeqCfg0 {
            SeqCfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqDmaAddr(pub u32);
    impl SeqDmaAddr {
        #[doc = "dma target address, should be 4-byte aligned."]
        #[inline(always)]
        pub const fn tar_addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "dma target address, should be 4-byte aligned."]
        #[inline(always)]
        pub fn set_tar_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for SeqDmaAddr {
        #[inline(always)]
        fn default() -> SeqDmaAddr {
            SeqDmaAddr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqDmaCfg(pub u32);
    impl SeqDmaCfg {
        #[doc = "dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
        #[inline(always)]
        pub const fn buf_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
        #[inline(always)]
        pub fn set_buf_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "set to stop dma if reach the stop_pos."]
        #[inline(always)]
        pub const fn stop_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to stop dma if reach the stop_pos."]
        #[inline(always)]
        pub fn set_stop_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst."]
        #[inline(always)]
        pub const fn dma_rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst."]
        #[inline(always)]
        pub fn set_dma_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet."]
        #[inline(always)]
        pub const fn stop_pos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet."]
        #[inline(always)]
        pub fn set_stop_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for SeqDmaCfg {
        #[inline(always)]
        fn default() -> SeqDmaCfg {
            SeqDmaCfg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqHighCfg(pub u32);
    impl SeqHighCfg {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn buf_len_high(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_buf_len_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn stop_pos_high(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_stop_pos_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
        }
    }
    impl Default for SeqHighCfg {
        #[inline(always)]
        fn default() -> SeqHighCfg {
            SeqHighCfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqQue(pub u32);
    impl SeqQue {
        #[doc = "channel number for current conversion."]
        #[inline(always)]
        pub const fn chan_num_4_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "channel number for current conversion."]
        #[inline(always)]
        pub fn set_chan_num_4_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "interrupt enable for current conversion."]
        #[inline(always)]
        pub const fn seq_int_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for current conversion."]
        #[inline(always)]
        pub fn set_seq_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for SeqQue {
        #[inline(always)]
        fn default() -> SeqQue {
            SeqQue(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SeqWrAddr(pub u32);
    impl SeqWrAddr {
        #[doc = "HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4."]
        #[inline(always)]
        pub const fn seq_wr_pointer(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4."]
        #[inline(always)]
        pub fn set_seq_wr_pointer(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for SeqWrAddr {
        #[inline(always)]
        fn default() -> SeqWrAddr {
            SeqWrAddr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgCmptFlag(pub u32);
    impl TrgCmptFlag {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn trg_cmpt_flag(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_trg_cmpt_flag(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for TrgCmptFlag {
        #[inline(always)]
        fn default() -> TrgCmptFlag {
            TrgCmptFlag(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgDmaAddr(pub u32);
    impl TrgDmaAddr {
        #[doc = "buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)."]
        #[inline(always)]
        pub const fn trg_dma_addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)."]
        #[inline(always)]
        pub fn set_trg_dma_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for TrgDmaAddr {
        #[inline(always)]
        fn default() -> TrgDmaAddr {
            TrgDmaAddr(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgSwSta(pub u32);
    impl TrgSwSta {
        #[doc = "which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c."]
        #[inline(always)]
        pub const fn trig_sw_index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c."]
        #[inline(always)]
        pub fn set_trig_sw_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
        #[inline(always)]
        pub const fn trg_sw_sta(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
        #[inline(always)]
        pub fn set_trg_sw_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for TrgSwSta {
        #[inline(always)]
        fn default() -> TrgSwSta {
            TrgSwSta(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigmuxEn(pub u32);
    impl TrigmuxEn {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn wdog(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_wdog(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn stop_pos(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_stop_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn seq_cvc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_seq_cvc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn seq_cmpt(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_seq_cmpt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for TrigmuxEn {
        #[inline(always)]
        fn default() -> TrigmuxEn {
            TrigmuxEn(0)
        }
    }
}
