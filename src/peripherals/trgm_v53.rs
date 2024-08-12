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
        pub const fn qei0_adc0_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc0_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei0_adc0_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei0_adc1_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc1_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei0_adc1_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei0_adc1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei1_adc0_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei1_adc0_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei1_adc0_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei1_adc0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[inline(always)]
        pub const fn qei1_adc1_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[inline(always)]
        pub fn set_qei1_adc1_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qei1_adc1_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qei1_adc1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        pub const fn acmp0_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp0_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn acmp0_dac_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp0_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn acmp1_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp1_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn acmp1_dac_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_acmp1_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac0_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac0_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac0_dac_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac0_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[inline(always)]
        pub const fn dac1_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[inline(always)]
        pub fn set_dac1_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dac1_dac_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dac1_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        pub const fn mode(&self) -> super::vals::FilterMode {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::FilterMode::from_bits(val as u8)
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::FilterMode) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
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
        pub const fn sei_posin0_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin0_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sei_posin0_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sei_posin1_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin1_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sei_posin1_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sei_posin1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mmc0_posin_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mmc0_posin_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mmc0_posin_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mmc0_posin_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[inline(always)]
        pub const fn mmc1_posin_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[inline(always)]
        pub fn set_mmc1_posin_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn mmc1_posin_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_mmc1_posin_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
        pub const fn qeo0_pos_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo0_pos_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qeo0_pos_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo0_pos_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qeo1_pos_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo1_pos_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn qeo1_pos_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_qeo1_pos_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
pub mod vals {
    #[doc = "ADC selections, 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct AdcSel(pub u8);
    impl AdcSel {
        pub const ADC0: Self = Self(0x0);
        pub const ADC1: Self = Self(0x01);
        pub const RDC_ADC0: Self = Self(0x02);
        pub const RDC_ADC1: Self = Self(0x03);
    }
    impl AdcSel {
        pub const fn from_bits(val: u8) -> AdcSel {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for AdcSel {
        #[inline(always)]
        fn from(val: u8) -> AdcSel {
            AdcSel::from_bits(val)
        }
    }
    impl From<AdcSel> for u8 {
        #[inline(always)]
        fn from(val: AdcSel) -> u8 {
            AdcSel::to_bits(val)
        }
    }
    #[doc = "DAC selections, 0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum DacSel {
        QEO0_DAC0 = 0x0,
        QEO0_DAC1 = 0x01,
        QEO0_DAC2 = 0x02,
        QEO1_DAC0 = 0x03,
        QEO1_DAC1 = 0x04,
        QEO1_DAC2 = 0x05,
        RDC_DAC0 = 0x06,
        RDC_DAC1 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
        _RESERVED_40 = 0x40,
        _RESERVED_41 = 0x41,
        _RESERVED_42 = 0x42,
        _RESERVED_43 = 0x43,
        _RESERVED_44 = 0x44,
        _RESERVED_45 = 0x45,
        _RESERVED_46 = 0x46,
        _RESERVED_47 = 0x47,
        _RESERVED_48 = 0x48,
        _RESERVED_49 = 0x49,
        _RESERVED_4a = 0x4a,
        _RESERVED_4b = 0x4b,
        _RESERVED_4c = 0x4c,
        _RESERVED_4d = 0x4d,
        _RESERVED_4e = 0x4e,
        _RESERVED_4f = 0x4f,
        _RESERVED_50 = 0x50,
        _RESERVED_51 = 0x51,
        _RESERVED_52 = 0x52,
        _RESERVED_53 = 0x53,
        _RESERVED_54 = 0x54,
        _RESERVED_55 = 0x55,
        _RESERVED_56 = 0x56,
        _RESERVED_57 = 0x57,
        _RESERVED_58 = 0x58,
        _RESERVED_59 = 0x59,
        _RESERVED_5a = 0x5a,
        _RESERVED_5b = 0x5b,
        _RESERVED_5c = 0x5c,
        _RESERVED_5d = 0x5d,
        _RESERVED_5e = 0x5e,
        _RESERVED_5f = 0x5f,
        _RESERVED_60 = 0x60,
        _RESERVED_61 = 0x61,
        _RESERVED_62 = 0x62,
        _RESERVED_63 = 0x63,
        _RESERVED_64 = 0x64,
        _RESERVED_65 = 0x65,
        _RESERVED_66 = 0x66,
        _RESERVED_67 = 0x67,
        _RESERVED_68 = 0x68,
        _RESERVED_69 = 0x69,
        _RESERVED_6a = 0x6a,
        _RESERVED_6b = 0x6b,
        _RESERVED_6c = 0x6c,
        _RESERVED_6d = 0x6d,
        _RESERVED_6e = 0x6e,
        _RESERVED_6f = 0x6f,
        _RESERVED_70 = 0x70,
        _RESERVED_71 = 0x71,
        _RESERVED_72 = 0x72,
        _RESERVED_73 = 0x73,
        _RESERVED_74 = 0x74,
        _RESERVED_75 = 0x75,
        _RESERVED_76 = 0x76,
        _RESERVED_77 = 0x77,
        _RESERVED_78 = 0x78,
        _RESERVED_79 = 0x79,
        _RESERVED_7a = 0x7a,
        _RESERVED_7b = 0x7b,
        _RESERVED_7c = 0x7c,
        _RESERVED_7d = 0x7d,
        _RESERVED_7e = 0x7e,
        _RESERVED_7f = 0x7f,
    }
    impl DacSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DacSel {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DacSel {
        #[inline(always)]
        fn from(val: u8) -> DacSel {
            DacSel::from_bits(val)
        }
    }
    impl From<DacSel> for u8 {
        #[inline(always)]
        fn from(val: DacSel) -> u8 {
            DacSel::to_bits(val)
        }
    }
    #[doc = "Filter mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FilterMode {
        #[doc = "Bypass"]
        BYPASS = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Rapid change mode"]
        RAPID_CHANGE = 0x04,
        #[doc = "Delay filter mode"]
        DELAY = 0x05,
        #[doc = "Stable low mode"]
        STABLE_LOW = 0x06,
        #[doc = "Stable high mode"]
        STABLE_HIGH = 0x07,
    }
    impl FilterMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterMode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterMode {
        #[inline(always)]
        fn from(val: u8) -> FilterMode {
            FilterMode::from_bits(val)
        }
    }
    impl From<FilterMode> for u8 {
        #[inline(always)]
        fn from(val: FilterMode) -> u8 {
            FilterMode::to_bits(val)
        }
    }
    #[doc = "Position selections, 0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PosSel {
        SEI_POS_OUT0 = 0x0,
        SEI_POS_OUT1 = 0x01,
        QEI0_POS = 0x02,
        QEI1_POS = 0x03,
        MMC0_POS_OUT0 = 0x04,
        MMC0_POS_OUT1 = 0x05,
        MMC1_POS_OUT0 = 0x06,
        MMC1_POS_OUT1 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
        _RESERVED_40 = 0x40,
        _RESERVED_41 = 0x41,
        _RESERVED_42 = 0x42,
        _RESERVED_43 = 0x43,
        _RESERVED_44 = 0x44,
        _RESERVED_45 = 0x45,
        _RESERVED_46 = 0x46,
        _RESERVED_47 = 0x47,
        _RESERVED_48 = 0x48,
        _RESERVED_49 = 0x49,
        _RESERVED_4a = 0x4a,
        _RESERVED_4b = 0x4b,
        _RESERVED_4c = 0x4c,
        _RESERVED_4d = 0x4d,
        _RESERVED_4e = 0x4e,
        _RESERVED_4f = 0x4f,
        _RESERVED_50 = 0x50,
        _RESERVED_51 = 0x51,
        _RESERVED_52 = 0x52,
        _RESERVED_53 = 0x53,
        _RESERVED_54 = 0x54,
        _RESERVED_55 = 0x55,
        _RESERVED_56 = 0x56,
        _RESERVED_57 = 0x57,
        _RESERVED_58 = 0x58,
        _RESERVED_59 = 0x59,
        _RESERVED_5a = 0x5a,
        _RESERVED_5b = 0x5b,
        _RESERVED_5c = 0x5c,
        _RESERVED_5d = 0x5d,
        _RESERVED_5e = 0x5e,
        _RESERVED_5f = 0x5f,
        _RESERVED_60 = 0x60,
        _RESERVED_61 = 0x61,
        _RESERVED_62 = 0x62,
        _RESERVED_63 = 0x63,
        _RESERVED_64 = 0x64,
        _RESERVED_65 = 0x65,
        _RESERVED_66 = 0x66,
        _RESERVED_67 = 0x67,
        _RESERVED_68 = 0x68,
        _RESERVED_69 = 0x69,
        _RESERVED_6a = 0x6a,
        _RESERVED_6b = 0x6b,
        _RESERVED_6c = 0x6c,
        _RESERVED_6d = 0x6d,
        _RESERVED_6e = 0x6e,
        _RESERVED_6f = 0x6f,
        _RESERVED_70 = 0x70,
        _RESERVED_71 = 0x71,
        _RESERVED_72 = 0x72,
        _RESERVED_73 = 0x73,
        _RESERVED_74 = 0x74,
        _RESERVED_75 = 0x75,
        _RESERVED_76 = 0x76,
        _RESERVED_77 = 0x77,
        _RESERVED_78 = 0x78,
        _RESERVED_79 = 0x79,
        _RESERVED_7a = 0x7a,
        _RESERVED_7b = 0x7b,
        _RESERVED_7c = 0x7c,
        _RESERVED_7d = 0x7d,
        _RESERVED_7e = 0x7e,
        _RESERVED_7f = 0x7f,
    }
    impl PosSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PosSel {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PosSel {
        #[inline(always)]
        fn from(val: u8) -> PosSel {
            PosSel::from_bits(val)
        }
    }
    impl From<PosSel> for u8 {
        #[inline(always)]
        fn from(val: PosSel) -> u8 {
            PosSel::to_bits(val)
        }
    }
}
