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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgocfg(self, n: usize) -> crate::common::Reg<regs::Trgocfg, crate::common::RW> {
        assert!(n < 137usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dmacfg(self, n: usize) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "General Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "adc matrix select register."]
    #[inline(always)]
    pub const fn adc_matrix_sel(self) -> crate::common::Reg<regs::AdcMatrixSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "dac matrix select register."]
    #[inline(always)]
    pub const fn dac_matrix_sel(self) -> crate::common::Reg<regs::DacMatrixSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "position matrix select register0."]
    #[inline(always)]
    pub const fn pos_matrix_sel0(
        self,
    ) -> crate::common::Reg<regs::PosMatrixSel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "position matrix select register1."]
    #[inline(always)]
    pub const fn pos_matrix_sel1(
        self,
    ) -> crate::common::Reg<regs::PosMatrixSel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgm_in(self, n: usize) -> crate::common::Reg<regs::TrgmIn, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgm_out(self, n: usize) -> crate::common::Reg<regs::TrgmOut, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 4usize) as _)
        }
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
    #[doc = "adc matrix select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcMatrixSel(pub u32);
    impl AdcMatrixSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei0_adc0_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei0_adc0_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei0_adc0_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei0_adc0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei0_adc1_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei0_adc1_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei0_adc1_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei0_adc1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei1_adc0_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei1_adc0_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei1_adc0_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei1_adc0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn qei1_adc1_sel(&self) -> super::vals::AdcSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::AdcSel::from_bits(val as u8)
        }
        #[doc = "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
        #[inline(always)]
        pub const fn set_qei1_adc1_sel(&mut self, val: super::vals::AdcSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qei1_adc1_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qei1_adc1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AdcMatrixSel {
        #[inline(always)]
        fn default() -> AdcMatrixSel {
            AdcMatrixSel(0)
        }
    }
    impl core::fmt::Debug for AdcMatrixSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcMatrixSel")
                .field("qei0_adc0_sel", &self.qei0_adc0_sel())
                .field("qei0_adc0_invert", &self.qei0_adc0_invert())
                .field("qei0_adc1_sel", &self.qei0_adc1_sel())
                .field("qei0_adc1_invert", &self.qei0_adc1_invert())
                .field("qei1_adc0_sel", &self.qei1_adc0_sel())
                .field("qei1_adc0_invert", &self.qei1_adc0_invert())
                .field("qei1_adc1_sel", &self.qei1_adc1_sel())
                .field("qei1_adc1_invert", &self.qei1_adc1_invert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcMatrixSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AdcMatrixSel {{ qei0_adc0_sel: {:?}, qei0_adc0_invert: {=bool:?}, qei0_adc1_sel: {:?}, qei0_adc1_invert: {=bool:?}, qei1_adc0_sel: {:?}, qei1_adc0_invert: {=bool:?}, qei1_adc1_sel: {:?}, qei1_adc1_invert: {=bool:?} }}" , self . qei0_adc0_sel () , self . qei0_adc0_invert () , self . qei0_adc1_sel () , self . qei0_adc1_invert () , self . qei1_adc0_sel () , self . qei1_adc0_invert () , self . qei1_adc1_sel () , self . qei1_adc1_invert ())
        }
    }
    #[doc = "dac matrix select register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacMatrixSel(pub u32);
    impl DacMatrixSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acmp0_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0_dac_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acmp0_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acmp1_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1_dac_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acmp1_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dac0_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dac0_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dac0_dac_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dac0_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1_dac_sel(&self) -> super::vals::DacSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::DacSel::from_bits(val as u8)
        }
        #[doc = "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved."]
        #[inline(always)]
        pub const fn set_dac1_dac_sel(&mut self, val: super::vals::DacSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1_dac_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dac1_dac_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DacMatrixSel {
        #[inline(always)]
        fn default() -> DacMatrixSel {
            DacMatrixSel(0)
        }
    }
    impl core::fmt::Debug for DacMatrixSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DacMatrixSel")
                .field("acmp0_dac_sel", &self.acmp0_dac_sel())
                .field("acmp0_dac_invert", &self.acmp0_dac_invert())
                .field("acmp1_dac_sel", &self.acmp1_dac_sel())
                .field("acmp1_dac_invert", &self.acmp1_dac_invert())
                .field("dac0_dac_sel", &self.dac0_dac_sel())
                .field("dac0_dac_invert", &self.dac0_dac_invert())
                .field("dac1_dac_sel", &self.dac1_dac_sel())
                .field("dac1_dac_invert", &self.dac1_dac_invert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacMatrixSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DacMatrixSel {{ acmp0_dac_sel: {:?}, acmp0_dac_invert: {=bool:?}, acmp1_dac_sel: {:?}, acmp1_dac_invert: {=bool:?}, dac0_dac_sel: {:?}, dac0_dac_invert: {=bool:?}, dac1_dac_sel: {:?}, dac1_dac_invert: {=bool:?} }}" , self . acmp0_dac_sel () , self . acmp0_dac_invert () , self . acmp1_dac_sel () , self . acmp1_dac_invert () , self . dac0_dac_sel () , self . dac0_dac_invert () , self . dac1_dac_sel () , self . dac1_dac_invert ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[must_use]
        #[inline(always)]
        pub const fn dmasrcsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub const fn set_dmasrcsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dmamux_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dmamux_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    impl core::fmt::Debug for Dmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacfg")
                .field("dmasrcsel", &self.dmasrcsel())
                .field("dmamux_en", &self.dmamux_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmacfg {{ dmasrcsel: {=u8:?}, dmamux_en: {=bool:?} }}",
                self.dmasrcsel(),
                self.dmamux_en()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtcfg(pub u32);
    impl Filtcfg {
        #[doc = "This bitfields defines the filter counter length."]
        #[must_use]
        #[inline(always)]
        pub const fn filtlen_base(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub const fn set_filtlen_base(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn filtlen_shift(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_filtlen_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub const fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::FilterMode {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::FilterMode::from_bits(val as u8)
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::FilterMode) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[must_use]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub const fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Filtcfg {
        #[inline(always)]
        fn default() -> Filtcfg {
            Filtcfg(0)
        }
    }
    impl core::fmt::Debug for Filtcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Filtcfg")
                .field("filtlen_base", &self.filtlen_base())
                .field("filtlen_shift", &self.filtlen_shift())
                .field("syncen", &self.syncen())
                .field("mode", &self.mode())
                .field("outinv", &self.outinv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filtcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Filtcfg {{ filtlen_base: {=u16:?}, filtlen_shift: {=u8:?}, syncen: {=bool:?}, mode: {:?}, outinv: {=bool:?} }}" , self . filtlen_base () , self . filtlen_shift () , self . syncen () , self . mode () , self . outinv ())
        }
    }
    #[doc = "General Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "The bitfield enable the TRGM outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn trgopen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub const fn set_trgopen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("trgopen", &self.trgopen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gcr {{ trgopen: {=u8:?} }}", self.trgopen())
        }
    }
    #[doc = "position matrix select register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosMatrixSel0(pub u32);
    impl PosMatrixSel0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sei_posin0_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sei_posin0_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sei_posin0_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sei_posin0_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sei_posin1_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sei_posin1_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sei_posin1_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sei_posin1_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mmc0_posin_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mmc0_posin_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mmc0_posin_invert(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mmc0_posin_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn mmc1_posin_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved."]
        #[inline(always)]
        pub const fn set_mmc1_posin_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mmc1_posin_invert(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mmc1_posin_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosMatrixSel0 {
        #[inline(always)]
        fn default() -> PosMatrixSel0 {
            PosMatrixSel0(0)
        }
    }
    impl core::fmt::Debug for PosMatrixSel0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosMatrixSel0")
                .field("sei_posin0_sel", &self.sei_posin0_sel())
                .field("sei_posin0_invert", &self.sei_posin0_invert())
                .field("sei_posin1_sel", &self.sei_posin1_sel())
                .field("sei_posin1_invert", &self.sei_posin1_invert())
                .field("mmc0_posin_sel", &self.mmc0_posin_sel())
                .field("mmc0_posin_invert", &self.mmc0_posin_invert())
                .field("mmc1_posin_sel", &self.mmc1_posin_sel())
                .field("mmc1_posin_invert", &self.mmc1_posin_invert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosMatrixSel0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PosMatrixSel0 {{ sei_posin0_sel: {:?}, sei_posin0_invert: {=bool:?}, sei_posin1_sel: {:?}, sei_posin1_invert: {=bool:?}, mmc0_posin_sel: {:?}, mmc0_posin_invert: {=bool:?}, mmc1_posin_sel: {:?}, mmc1_posin_invert: {=bool:?} }}" , self . sei_posin0_sel () , self . sei_posin0_invert () , self . sei_posin1_sel () , self . sei_posin1_invert () , self . mmc0_posin_sel () , self . mmc0_posin_invert () , self . mmc1_posin_sel () , self . mmc1_posin_invert ())
        }
    }
    #[doc = "position matrix select register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosMatrixSel1(pub u32);
    impl PosMatrixSel1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qeo0_pos_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 0usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qeo0_pos_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qeo0_pos_invert(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qeo0_pos_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qeo1_pos_sel(&self) -> super::vals::PosSel {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::PosSel::from_bits(val as u8)
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qeo1_pos_sel(&mut self, val: super::vals::PosSel) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn qeo1_pos_invert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_qeo1_pos_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for PosMatrixSel1 {
        #[inline(always)]
        fn default() -> PosMatrixSel1 {
            PosMatrixSel1(0)
        }
    }
    impl core::fmt::Debug for PosMatrixSel1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosMatrixSel1")
                .field("qeo0_pos_sel", &self.qeo0_pos_sel())
                .field("qeo0_pos_invert", &self.qeo0_pos_invert())
                .field("qeo1_pos_sel", &self.qeo1_pos_sel())
                .field("qeo1_pos_invert", &self.qeo1_pos_invert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosMatrixSel1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PosMatrixSel1 {{ qeo0_pos_sel: {:?}, qeo0_pos_invert: {=bool:?}, qeo1_pos_sel: {:?}, qeo1_pos_invert: {=bool:?} }}" , self . qeo0_pos_sel () , self . qeo0_pos_invert () , self . qeo1_pos_sel () , self . qeo1_pos_invert ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgmIn(pub u32);
    impl TrgmIn {
        #[doc = "mmc1_trig_out\\[1:0\\], mmc0_trig_out\\[1:0\\],sync_pulse\\[3:0\\],moto_gpio_in_sync\\[7:0\\],//31:16 gtmr3_to_motor_sync\\[1:0\\],gtmr2_to_motor_sync\\[1:0\\],gtmr1_to_motor_sync\\[1:0\\],gtmr0_to_motor_sync\\[1:0\\], //15:8 acmp_out_sync\\[1:0\\],can2mot_event_sync\\[1:0\\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0."]
        #[must_use]
        #[inline(always)]
        pub const fn trgm_in(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "mmc1_trig_out\\[1:0\\], mmc0_trig_out\\[1:0\\],sync_pulse\\[3:0\\],moto_gpio_in_sync\\[7:0\\],//31:16 gtmr3_to_motor_sync\\[1:0\\],gtmr2_to_motor_sync\\[1:0\\],gtmr1_to_motor_sync\\[1:0\\],gtmr0_to_motor_sync\\[1:0\\], //15:8 acmp_out_sync\\[1:0\\],can2mot_event_sync\\[1:0\\],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0."]
        #[inline(always)]
        pub const fn set_trgm_in(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgmIn {
        #[inline(always)]
        fn default() -> TrgmIn {
            TrgmIn(0)
        }
    }
    impl core::fmt::Debug for TrgmIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgmIn")
                .field("trgm_in", &self.trgm_in())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgmIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TrgmIn {{ trgm_in: {=u32:?} }}", self.trgm_in())
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
        #[must_use]
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
        pub const fn set_trgm_out(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgmOut {
        #[inline(always)]
        fn default() -> TrgmOut {
            TrgmOut(0)
        }
    }
    impl core::fmt::Debug for TrgmOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgmOut")
                .field("trgm_out", &self.trgm_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgmOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TrgmOut {{ trgm_out: {=u32:?} }}", self.trgm_out())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgocfg(pub u32);
    impl Trgocfg {
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[must_use]
        #[inline(always)]
        pub const fn trigosel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub const fn set_trigosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[must_use]
        #[inline(always)]
        pub const fn redg2pen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn set_redg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[must_use]
        #[inline(always)]
        pub const fn fedg2pen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn set_fedg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- Invert the output."]
        #[must_use]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub const fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Trgocfg {
        #[inline(always)]
        fn default() -> Trgocfg {
            Trgocfg(0)
        }
    }
    impl core::fmt::Debug for Trgocfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trgocfg")
                .field("trigosel", &self.trigosel())
                .field("redg2pen", &self.redg2pen())
                .field("fedg2pen", &self.fedg2pen())
                .field("outinv", &self.outinv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trgocfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Trgocfg {{ trigosel: {=u8:?}, redg2pen: {=bool:?}, fedg2pen: {=bool:?}, outinv: {=bool:?} }}" , self . trigosel () , self . redg2pen () , self . fedg2pen () , self . outinv ())
        }
    }
}
pub mod vals {
    #[doc = "ADC selections, 0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct AdcSel(u8);
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
    impl core::fmt::Debug for AdcSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("ADC0"),
                0x01 => f.write_str("ADC1"),
                0x02 => f.write_str("RDC_ADC0"),
                0x03 => f.write_str("RDC_ADC1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcSel {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "ADC0"),
                0x01 => defmt::write!(f, "ADC1"),
                0x02 => defmt::write!(f, "RDC_ADC0"),
                0x03 => defmt::write!(f, "RDC_ADC1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DacSel(u8);
    impl DacSel {
        pub const QEO0_DAC0: Self = Self(0x0);
        pub const QEO0_DAC1: Self = Self(0x01);
        pub const QEO0_DAC2: Self = Self(0x02);
        pub const QEO1_DAC0: Self = Self(0x03);
        pub const QEO1_DAC1: Self = Self(0x04);
        pub const QEO1_DAC2: Self = Self(0x05);
        pub const RDC_DAC0: Self = Self(0x06);
        pub const RDC_DAC1: Self = Self(0x07);
    }
    impl DacSel {
        pub const fn from_bits(val: u8) -> DacSel {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for DacSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("QEO0_DAC0"),
                0x01 => f.write_str("QEO0_DAC1"),
                0x02 => f.write_str("QEO0_DAC2"),
                0x03 => f.write_str("QEO1_DAC0"),
                0x04 => f.write_str("QEO1_DAC1"),
                0x05 => f.write_str("QEO1_DAC2"),
                0x06 => f.write_str("RDC_DAC0"),
                0x07 => f.write_str("RDC_DAC1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DacSel {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "QEO0_DAC0"),
                0x01 => defmt::write!(f, "QEO0_DAC1"),
                0x02 => defmt::write!(f, "QEO0_DAC2"),
                0x03 => defmt::write!(f, "QEO1_DAC0"),
                0x04 => defmt::write!(f, "QEO1_DAC1"),
                0x05 => defmt::write!(f, "QEO1_DAC2"),
                0x06 => defmt::write!(f, "RDC_DAC0"),
                0x07 => defmt::write!(f, "RDC_DAC1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct PosSel(u8);
    impl PosSel {
        pub const SEI_POS_OUT0: Self = Self(0x0);
        pub const SEI_POS_OUT1: Self = Self(0x01);
        pub const QEI0_POS: Self = Self(0x02);
        pub const QEI1_POS: Self = Self(0x03);
        pub const MMC0_POS_OUT0: Self = Self(0x04);
        pub const MMC0_POS_OUT1: Self = Self(0x05);
        pub const MMC1_POS_OUT0: Self = Self(0x06);
        pub const MMC1_POS_OUT1: Self = Self(0x07);
    }
    impl PosSel {
        pub const fn from_bits(val: u8) -> PosSel {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for PosSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("SEI_POS_OUT0"),
                0x01 => f.write_str("SEI_POS_OUT1"),
                0x02 => f.write_str("QEI0_POS"),
                0x03 => f.write_str("QEI1_POS"),
                0x04 => f.write_str("MMC0_POS_OUT0"),
                0x05 => f.write_str("MMC0_POS_OUT1"),
                0x06 => f.write_str("MMC1_POS_OUT0"),
                0x07 => f.write_str("MMC1_POS_OUT1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSel {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "SEI_POS_OUT0"),
                0x01 => defmt::write!(f, "SEI_POS_OUT1"),
                0x02 => defmt::write!(f, "QEI0_POS"),
                0x03 => defmt::write!(f, "QEI1_POS"),
                0x04 => defmt::write!(f, "MMC0_POS_OUT0"),
                0x05 => defmt::write!(f, "MMC0_POS_OUT1"),
                0x06 => defmt::write!(f, "MMC1_POS_OUT0"),
                0x07 => defmt::write!(f, "MMC1_POS_OUT1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
