#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TSNS."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsns {
    ptr: *mut u8,
}
unsafe impl Send for Tsns {}
unsafe impl Sync for Tsns {}
impl Tsns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Temperature."]
    #[inline(always)]
    pub const fn t(self) -> crate::common::Reg<regs::T, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Maximum Temperature."]
    #[inline(always)]
    pub const fn tmax(self) -> crate::common::Reg<regs::Tmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Minimum Temperature."]
    #[inline(always)]
    pub const fn tmin(self) -> crate::common::Reg<regs::Tmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Sample age."]
    #[inline(always)]
    pub const fn age(self) -> crate::common::Reg<regs::Age, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Sample validity."]
    #[inline(always)]
    pub const fn validity(self) -> crate::common::Reg<regs::Validity, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Temperature flag."]
    #[inline(always)]
    pub const fn flag(self) -> crate::common::Reg<regs::Flag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Maximum temperature to interrupt."]
    #[inline(always)]
    pub const fn upper_lim_irq(self) -> crate::common::Reg<regs::UpperLimIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Minimum temperature to interrupt."]
    #[inline(always)]
    pub const fn lower_lim_irq(self) -> crate::common::Reg<regs::LowerLimIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Maximum temperature to reset."]
    #[inline(always)]
    pub const fn upper_lim_rst(self) -> crate::common::Reg<regs::UpperLimRst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Minimum temperature to reset."]
    #[inline(always)]
    pub const fn lower_lim_rst(self) -> crate::common::Reg<regs::LowerLimRst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Configuration in asynchronous mode."]
    #[inline(always)]
    pub const fn async_(self) -> crate::common::Reg<regs::Async, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Advance configuration."]
    #[inline(always)]
    pub const fn advan(self) -> crate::common::Reg<regs::Advan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
}
pub mod regs {
    #[doc = "Advance configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Advan(pub u32);
    impl Advan {
        #[doc = "use positive compare polarity only."]
        #[inline(always)]
        pub const fn pos_only(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "use positive compare polarity only."]
        #[inline(always)]
        pub fn set_pos_only(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "use negative compare polarity only."]
        #[inline(always)]
        pub const fn neg_only(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "use negative compare polarity only."]
        #[inline(always)]
        pub fn set_neg_only(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "temperature sampling is working."]
        #[inline(always)]
        pub const fn sampling(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sampling is working."]
        #[inline(always)]
        pub fn set_sampling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "interrupt status of active mode."]
        #[inline(always)]
        pub const fn active_irq(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt status of active mode."]
        #[inline(always)]
        pub fn set_active_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "interrupt status of asynchronous mode."]
        #[inline(always)]
        pub const fn async_irq(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt status of asynchronous mode."]
        #[inline(always)]
        pub fn set_async_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Advan {
        #[inline(always)]
        fn default() -> Advan {
            Advan(0)
        }
    }
    #[doc = "Sample age."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Age(pub u32);
    impl Age {
        #[doc = "age of T register in 24MHz clock cycles."]
        #[inline(always)]
        pub const fn age(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "age of T register in 24MHz clock cycles."]
        #[inline(always)]
        pub fn set_age(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Age {
        #[inline(always)]
        fn default() -> Age {
            Age(0)
        }
    }
    #[doc = "Configuration in asynchronous mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Async(pub u32);
    impl Async {
        #[doc = "Value of async mode to compare."]
        #[inline(always)]
        pub const fn value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Value of async mode to compare."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Polarity of internal comparator."]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of internal comparator."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than."]
        #[inline(always)]
        pub const fn async_type(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than."]
        #[inline(always)]
        pub fn set_async_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Async {
        #[inline(always)]
        fn default() -> Async {
            Async(0)
        }
    }
    #[doc = "Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode."]
        #[inline(always)]
        pub const fn async_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode."]
        #[inline(always)]
        pub fn set_async_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode."]
        #[inline(always)]
        pub const fn continuous(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode."]
        #[inline(always)]
        pub fn set_continuous(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average."]
        #[inline(always)]
        pub const fn average(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average."]
        #[inline(always)]
        pub fn set_average(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step."]
        #[inline(always)]
        pub const fn speed(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step."]
        #[inline(always)]
        pub fn set_speed(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Enable compare for maximum temperature."]
        #[inline(always)]
        pub const fn compare_max_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable compare for maximum temperature."]
        #[inline(always)]
        pub fn set_compare_max_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Enable compare for minimum temperature."]
        #[inline(always)]
        pub const fn compare_min_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Enable compare for minimum temperature."]
        #[inline(always)]
        pub fn set_compare_min_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Enable reset."]
        #[inline(always)]
        pub const fn rst_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable reset."]
        #[inline(always)]
        pub fn set_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable interrupt."]
        #[inline(always)]
        pub const fn irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    #[doc = "Temperature flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flag(pub u32);
    impl Flag {
        #[doc = "IRQ flag, write 1 to clear."]
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ flag, write 1 to clear."]
        #[inline(always)]
        pub fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear over temperature status, write 1 to clear."]
        #[inline(always)]
        pub const fn over_temp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear over temperature status, write 1 to clear."]
        #[inline(always)]
        pub fn set_over_temp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear under temperature status, write 1 to clear."]
        #[inline(always)]
        pub const fn under_temp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear under temperature status, write 1 to clear."]
        #[inline(always)]
        pub fn set_under_temp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear maximum recorder of temerature, write 1 to clear."]
        #[inline(always)]
        pub const fn record_max_clr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear maximum recorder of temerature, write 1 to clear."]
        #[inline(always)]
        pub fn set_record_max_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear minimum recorder of temerature, write 1 to clear."]
        #[inline(always)]
        pub const fn record_min_clr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear minimum recorder of temerature, write 1 to clear."]
        #[inline(always)]
        pub fn set_record_min_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Flag {
        #[inline(always)]
        fn default() -> Flag {
            Flag(0)
        }
    }
    #[doc = "Minimum temperature to interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LowerLimIrq(pub u32);
    impl LowerLimIrq {
        #[doc = "Minimum temperature for compare."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Minimum temperature for compare."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LowerLimIrq {
        #[inline(always)]
        fn default() -> LowerLimIrq {
            LowerLimIrq(0)
        }
    }
    #[doc = "Minimum temperature to reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LowerLimRst(pub u32);
    impl LowerLimRst {
        #[doc = "Minimum temperature for compare."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Minimum temperature for compare."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LowerLimRst {
        #[inline(always)]
        fn default() -> LowerLimRst {
            LowerLimRst(0)
        }
    }
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Software trigger for sensing in trigger mode, trigger will be ignored if in sensing or other mode."]
        #[inline(always)]
        pub const fn trigger(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software trigger for sensing in trigger mode, trigger will be ignored if in sensing or other mode."]
        #[inline(always)]
        pub fn set_trigger(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "indicate value in T is valid or not 0: not valid 1:valid."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "indicate value in T is valid or not 0: not valid 1:valid."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "Temperature."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct T(pub u32);
    impl T {
        #[doc = "Signed number of temperature in 256 x celsius degree."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Signed number of temperature in 256 x celsius degree."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for T {
        #[inline(always)]
        fn default() -> T {
            T(0)
        }
    }
    #[doc = "Maximum Temperature."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmax(pub u32);
    impl Tmax {
        #[doc = "maximum temperature ever found."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "maximum temperature ever found."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tmax {
        #[inline(always)]
        fn default() -> Tmax {
            Tmax(0)
        }
    }
    #[doc = "Minimum Temperature."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmin(pub u32);
    impl Tmin {
        #[doc = "minimum temperature ever found."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "minimum temperature ever found."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tmin {
        #[inline(always)]
        fn default() -> Tmin {
            Tmin(0)
        }
    }
    #[doc = "Maximum temperature to interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UpperLimIrq(pub u32);
    impl UpperLimIrq {
        #[doc = "Maximum temperature for compare."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Maximum temperature for compare."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UpperLimIrq {
        #[inline(always)]
        fn default() -> UpperLimIrq {
            UpperLimIrq(0)
        }
    }
    #[doc = "Maximum temperature to reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UpperLimRst(pub u32);
    impl UpperLimRst {
        #[doc = "Maximum temperature for compare."]
        #[inline(always)]
        pub const fn t(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Maximum temperature for compare."]
        #[inline(always)]
        pub fn set_t(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UpperLimRst {
        #[inline(always)]
        fn default() -> UpperLimRst {
            UpperLimRst(0)
        }
    }
    #[doc = "Sample validity."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Validity(pub u32);
    impl Validity {
        #[doc = "time for temperature values to expire in 24M clock cycles."]
        #[inline(always)]
        pub const fn validity(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "time for temperature values to expire in 24M clock cycles."]
        #[inline(always)]
        pub fn set_validity(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Validity {
        #[inline(always)]
        fn default() -> Validity {
            Validity(0)
        }
    }
}
