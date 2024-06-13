#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TRGM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgm {
    ptr: *mut u8,
}
unsafe impl Send for Trgm {}
unsafe impl Sync for Trgm {}
impl Trgm {
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
    pub const fn filtcfg(self, n: usize) -> crate::common::Reg<regs::Filtcfg, crate::common::RW> {
        assert!(n < 28usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgocfg(self, n: usize) -> crate::common::Reg<regs::Trgocfg, crate::common::RW> {
        assert!(n < 137usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dmacfg(self, n: usize) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[doc = "General Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "adc matrix select register."]
    #[inline(always)]
    pub const fn adc_matrix_sel(self) -> crate::common::Reg<regs::AdcMatrixSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "dac matrix select register."]
    #[inline(always)]
    pub const fn dac_matrix_sel(self) -> crate::common::Reg<regs::DacMatrixSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "position matrix select register0."]
    #[inline(always)]
    pub const fn pos_matrix_sel0(
        self,
    ) -> crate::common::Reg<regs::PosMatrixSel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "position matrix select register1."]
    #[inline(always)]
    pub const fn pos_matrix_sel1(
        self,
    ) -> crate::common::Reg<regs::PosMatrixSel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgm_in(self, n: usize) -> crate::common::Reg<regs::TrgmIn, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgm_out(self, n: usize) -> crate::common::Reg<regs::TrgmOut, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "adc matrix select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcMatrixSel(pub u32);
    impl AdcMatrixSel {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei0_adc0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei0_adc1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei1_adc0_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei1_adc0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[inline(always)]
        pub const fn qei1_adc1_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[inline(always)]
        pub fn set_qei1_adc1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for AdcMatrixSel {
        #[inline(always)]
        fn default() -> AdcMatrixSel {
            AdcMatrixSel(0)
        }
    }
    #[doc = "dac matrix select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacMatrixSel(pub u32);
    impl DacMatrixSel {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn acmp0_dac_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp0_dac_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn acmp1_dac_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp1_dac_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac0_dac_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac0_dac_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[inline(always)]
        pub const fn dac1_dac_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[inline(always)]
        pub fn set_dac1_dac_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DacMatrixSel {
        #[inline(always)]
        fn default() -> DacMatrixSel {
            DacMatrixSel(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub const fn dmasrcsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub fn set_dmasrcsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dmamux_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dmamux_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtcfg(pub u32);
    impl Filtcfg {
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub const fn filtlen_base(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub fn set_filtlen_base(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn filtlen_shift(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_filtlen_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Filtcfg {
        #[inline(always)]
        fn default() -> Filtcfg {
            Filtcfg(0)
        }
    }
    #[doc = "General Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub const fn trgopen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub fn set_trgopen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "position matrix select register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosMatrixSel0(pub u32);
    impl PosMatrixSel0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sei_posin0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sei_posin1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mmc0_posin_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mmc0_posin_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[inline(always)]
        pub const fn mmc1_posin_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[inline(always)]
        pub fn set_mmc1_posin_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PosMatrixSel0 {
        #[inline(always)]
        fn default() -> PosMatrixSel0 {
            PosMatrixSel0(0)
        }
    }
    #[doc = "position matrix select register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosMatrixSel1(pub u32);
    impl PosMatrixSel1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qeo0_pos_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo0_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qeo1_pos_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo1_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for PosMatrixSel1 {
        #[inline(always)]
        fn default() -> PosMatrixSel1 {
            PosMatrixSel1(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgmIn(pub u32);
    impl TrgmIn {
        #[doc = "mmc1_trig_out\\[1:0\\], mmc0_trig_out\\[1:0\\],sync_pulse\\[3:0\\],moto_gpio_in_sync\\[7:0\\],//31:16 gtmr3_to_motor_sync\\[1:0\\],gtmr2_to_motor_sync\\[1:0\\],gtmr1_to_motor_sync\\[1:0\\],gtmr0_to_motor_sync\\[1:0\\], //15:8 acmp_out_sync\\[1:0\\],can2mot_event_sync\\[1:0\\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0."]
        #[inline(always)]
        pub const fn trgm_in(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "mmc1_trig_out\\[1:0\\], mmc0_trig_out\\[1:0\\],sync_pulse\\[3:0\\],moto_gpio_in_sync\\[7:0\\],//31:16 gtmr3_to_motor_sync\\[1:0\\],gtmr2_to_motor_sync\\[1:0\\],gtmr1_to_motor_sync\\[1:0\\],gtmr0_to_motor_sync\\[1:0\\], //15:8 acmp_out_sync\\[1:0\\],can2mot_event_sync\\[1:0\\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0."]
        #[inline(always)]
        pub fn set_trgm_in(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgmIn {
        #[inline(always)]
        fn default() -> TrgmIn {
            TrgmIn(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgmOut(pub u32);
    impl TrgmOut {
        #[doc = "motor_to_opamp0\\[7:0\\]
= trig_mux_out\\[7:0\\]; motor_to_opamp1\\[7:0\\]
= trig_mux_out\\[15:8\\]; motor_to_gtmr0_capt\\[1:0\\]
= trig_mux_out\\[17:16\\]; motor_to_gtmr0_sync = trig_mux_out\\[18\\]; motor_to_gtmr1_capt\\[1:0\\]
= trig_mux_out\\[20:19\\]; motor_to_gtmr1_sync = trig_mux_out\\[21\\]; motor_to_gtmr2_capt\\[1:0\\]
= trig_mux_out\\[23:22\\]; motor_to_gtmr2_sync = trig_mux_out\\[24\\]; motor_to_gtmr3_capt\\[1:0\\]
= trig_mux_out\\[26:25\\]; motor_to_gtmr3_sync = trig_mux_out\\[27\\]; acmp_window\\[1:0\\]
= trig_mux_out\\[29:28\\]; dac0_buf_trigger = trig_mux_out\\[30\\]; dac1_buf_trigger = trig_mux_out\\[31\\]; dac0_step_trigger\\[3:0\\]
= {trig_mux_out\\[24:22\\],trig_mux_out\\[30\\]};//use same buf_trig, and gtmr2 dac1_step_trigger\\[3:0\\]
= {trig_mux_out\\[27:25\\],trig_mux_out\\[31\\]}; //use same buf_trig, and gtmr3."]
        #[inline(always)]
        pub const fn trgm_out(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "motor_to_opamp0\\[7:0\\]
= trig_mux_out\\[7:0\\]; motor_to_opamp1\\[7:0\\]
= trig_mux_out\\[15:8\\]; motor_to_gtmr0_capt\\[1:0\\]
= trig_mux_out\\[17:16\\]; motor_to_gtmr0_sync = trig_mux_out\\[18\\]; motor_to_gtmr1_capt\\[1:0\\]
= trig_mux_out\\[20:19\\]; motor_to_gtmr1_sync = trig_mux_out\\[21\\]; motor_to_gtmr2_capt\\[1:0\\]
= trig_mux_out\\[23:22\\]; motor_to_gtmr2_sync = trig_mux_out\\[24\\]; motor_to_gtmr3_capt\\[1:0\\]
= trig_mux_out\\[26:25\\]; motor_to_gtmr3_sync = trig_mux_out\\[27\\]; acmp_window\\[1:0\\]
= trig_mux_out\\[29:28\\]; dac0_buf_trigger = trig_mux_out\\[30\\]; dac1_buf_trigger = trig_mux_out\\[31\\]; dac0_step_trigger\\[3:0\\]
= {trig_mux_out\\[24:22\\],trig_mux_out\\[30\\]};//use same buf_trig, and gtmr2 dac1_step_trigger\\[3:0\\]
= {trig_mux_out\\[27:25\\],trig_mux_out\\[31\\]}; //use same buf_trig, and gtmr3."]
        #[inline(always)]
        pub fn set_trgm_out(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgmOut {
        #[inline(always)]
        fn default() -> TrgmOut {
            TrgmOut(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgocfg(pub u32);
    impl Trgocfg {
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub const fn trigosel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub fn set_trigosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn redg2pen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub fn set_redg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn fedg2pen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub fn set_fedg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Trgocfg {
        #[inline(always)]
        fn default() -> Trgocfg {
            Trgocfg(0)
        }
    }
}
