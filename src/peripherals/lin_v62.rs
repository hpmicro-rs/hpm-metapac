#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LIN0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lin {
    ptr: *mut u8,
}
unsafe impl Send for Lin {}
unsafe impl Sync for Lin {}
impl Lin {
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
    pub const fn databyte(self, n: usize) -> crate::common::Reg<regs::Databyte, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "state register."]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "error register."]
    #[inline(always)]
    pub const fn error(self) -> crate::common::Reg<regs::Error, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "data lenth register."]
    #[inline(always)]
    pub const fn data_len(self) -> crate::common::Reg<regs::DataLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "baudrate control low register."]
    #[inline(always)]
    pub const fn baudrate_ctl_low(
        self,
    ) -> crate::common::Reg<regs::BaudrateCtlLow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "baudrate control high register."]
    #[inline(always)]
    pub const fn bardrate_ctl_high(
        self,
    ) -> crate::common::Reg<regs::BardrateCtlHigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "id register."]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "timeout control register."]
    #[inline(always)]
    pub const fn tv(self) -> crate::common::Reg<regs::Tv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
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
    #[doc = "baudrate control high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BardrateCtlHigh(pub u32);
    impl BardrateCtlHigh {
        #[doc = "bit div register 8."]
        #[must_use]
        #[inline(always)]
        pub const fn bt_div_high(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bit div register 8."]
        #[inline(always)]
        pub const fn set_bt_div_high(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "bt_mul register."]
        #[must_use]
        #[inline(always)]
        pub const fn bt_mul(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "bt_mul register."]
        #[inline(always)]
        pub const fn set_bt_mul(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "prescl register."]
        #[must_use]
        #[inline(always)]
        pub const fn prescl(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "prescl register."]
        #[inline(always)]
        pub const fn set_prescl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for BardrateCtlHigh {
        #[inline(always)]
        fn default() -> BardrateCtlHigh {
            BardrateCtlHigh(0)
        }
    }
    impl core::fmt::Debug for BardrateCtlHigh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BardrateCtlHigh")
                .field("bt_div_high", &self.bt_div_high())
                .field("bt_mul", &self.bt_mul())
                .field("prescl", &self.prescl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BardrateCtlHigh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BardrateCtlHigh {{ bt_div_high: {=bool:?}, bt_mul: {=u8:?}, prescl: {=u8:?} }}",
                self.bt_div_high(),
                self.bt_mul(),
                self.prescl()
            )
        }
    }
    #[doc = "baudrate control low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BaudrateCtlLow(pub u32);
    impl BaudrateCtlLow {
        #[doc = "bit div register 7:0."]
        #[must_use]
        #[inline(always)]
        pub const fn bt_div_low(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "bit div register 7:0."]
        #[inline(always)]
        pub const fn set_bt_div_low(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for BaudrateCtlLow {
        #[inline(always)]
        fn default() -> BaudrateCtlLow {
            BaudrateCtlLow(0)
        }
    }
    impl core::fmt::Debug for BaudrateCtlLow {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BaudrateCtlLow")
                .field("bt_div_low", &self.bt_div_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BaudrateCtlLow {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BaudrateCtlLow {{ bt_div_low: {=u8:?} }}",
                self.bt_div_low()
            )
        }
    }
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u32);
    impl Control {
        #[doc = "master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn start_req(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred."]
        #[inline(always)]
        pub const fn set_start_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core."]
        #[inline(always)]
        pub const fn set_wakeup_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0."]
        #[inline(always)]
        pub const fn set_reset_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "write 1 to reset the int bit in the status register and the interrupt request output of LIN."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_int(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "write 1 to reset the int bit in the status register and the interrupt request output of LIN."]
        #[inline(always)]
        pub const fn set_reset_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "slave only. Write 1 after handling a data request interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn data_ack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Write 1 after handling a data request interrupt."]
        #[inline(always)]
        pub const fn set_data_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1: transmit operation 0: receive operation."]
        #[must_use]
        #[inline(always)]
        pub const fn transmit(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1: transmit operation 0: receive operation."]
        #[inline(always)]
        pub const fn set_transmit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
        #[must_use]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
        #[inline(always)]
        pub const fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    impl core::fmt::Debug for Control {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Control")
                .field("start_req", &self.start_req())
                .field("wakeup_req", &self.wakeup_req())
                .field("reset_error", &self.reset_error())
                .field("reset_int", &self.reset_int())
                .field("data_ack", &self.data_ack())
                .field("transmit", &self.transmit())
                .field("sleep", &self.sleep())
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Control {{ start_req: {=bool:?}, wakeup_req: {=bool:?}, reset_error: {=bool:?}, reset_int: {=bool:?}, data_ack: {=bool:?}, transmit: {=bool:?}, sleep: {=bool:?}, stop: {=bool:?} }}" , self . start_req () , self . wakeup_req () , self . reset_error () , self . reset_int () , self . data_ack () , self . transmit () , self . sleep () , self . stop ())
        }
    }
    #[doc = "data lenth register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataLen(pub u32);
    impl DataLen {
        #[doc = "data length."]
        #[must_use]
        #[inline(always)]
        pub const fn data_length(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "data length."]
        #[inline(always)]
        pub const fn set_data_length(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "1:enhence check mode."]
        #[must_use]
        #[inline(always)]
        pub const fn enh_check(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1:enhence check mode."]
        #[inline(always)]
        pub const fn set_enh_check(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for DataLen {
        #[inline(always)]
        fn default() -> DataLen {
            DataLen(0)
        }
    }
    impl core::fmt::Debug for DataLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DataLen")
                .field("data_length", &self.data_length())
                .field("enh_check", &self.enh_check())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DataLen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DataLen {{ data_length: {=u8:?}, enh_check: {=bool:?} }}",
                self.data_length(),
                self.enh_check()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Databyte(pub u32);
    impl Databyte {
        #[doc = "data byte."]
        #[must_use]
        #[inline(always)]
        pub const fn data_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "data byte."]
        #[inline(always)]
        pub const fn set_data_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Databyte {
        #[inline(always)]
        fn default() -> Databyte {
            Databyte(0)
        }
    }
    impl core::fmt::Debug for Databyte {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Databyte")
                .field("data_byte", &self.data_byte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Databyte {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Databyte {{ data_byte: {=u8:?} }}", self.data_byte())
        }
    }
    #[doc = "error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Error(pub u32);
    impl Error {
        #[doc = "bit error."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bit error."]
        #[inline(always)]
        pub const fn set_bit_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "checksum error."]
        #[must_use]
        #[inline(always)]
        pub const fn chk_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "checksum error."]
        #[inline(always)]
        pub const fn set_chk_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "slave only. identifier parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. identifier parity error."]
        #[inline(always)]
        pub const fn set_parity_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Error {
        #[inline(always)]
        fn default() -> Error {
            Error(0)
        }
    }
    impl core::fmt::Debug for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Error")
                .field("bit_error", &self.bit_error())
                .field("chk_error", &self.chk_error())
                .field("timeout", &self.timeout())
                .field("parity_error", &self.parity_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Error {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Error {{ bit_error: {=bool:?}, chk_error: {=bool:?}, timeout: {=bool:?}, parity_error: {=bool:?} }}" , self . bit_error () , self . chk_error () , self . timeout () , self . parity_error ())
        }
    }
    #[doc = "id register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Id(pub u32);
    impl Id {
        #[doc = "id register."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "id register."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Id {
        #[inline(always)]
        fn default() -> Id {
            Id(0)
        }
    }
    impl core::fmt::Debug for Id {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Id").field("id", &self.id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Id {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Id {{ id: {=u8:?} }}", self.id())
        }
    }
    #[doc = "state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State(pub u32);
    impl State {
        #[doc = "set after a transmission has been successful finished and it will reset at the start of a transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn complete(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set after a transmission has been successful finished and it will reset at the start of a transmission."]
        #[inline(always)]
        pub const fn set_complete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1."]
        #[inline(always)]
        pub const fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set when detecte an error, clear by reset_error."]
        #[must_use]
        #[inline(always)]
        pub const fn error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set when detecte an error, clear by reset_error."]
        #[inline(always)]
        pub const fn set_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set when request an interrupt. Reset by reset_int."]
        #[must_use]
        #[inline(always)]
        pub const fn int(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set when request an interrupt. Reset by reset_int."]
        #[inline(always)]
        pub const fn set_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
        #[must_use]
        #[inline(always)]
        pub const fn data_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
        #[inline(always)]
        pub const fn set_data_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
        #[must_use]
        #[inline(always)]
        pub const fn aborted(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
        #[inline(always)]
        pub const fn set_aborted(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_idle_tv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s."]
        #[inline(always)]
        pub const fn set_bus_idle_tv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The bit indicates whether the LIN bus is active or not."]
        #[must_use]
        #[inline(always)]
        pub const fn lin_active(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The bit indicates whether the LIN bus is active or not."]
        #[inline(always)]
        pub const fn set_lin_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for State {
        #[inline(always)]
        fn default() -> State {
            State(0)
        }
    }
    impl core::fmt::Debug for State {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("State")
                .field("complete", &self.complete())
                .field("wakeup", &self.wakeup())
                .field("error", &self.error())
                .field("int", &self.int())
                .field("data_req", &self.data_req())
                .field("aborted", &self.aborted())
                .field("bus_idle_tv", &self.bus_idle_tv())
                .field("lin_active", &self.lin_active())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for State {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "State {{ complete: {=bool:?}, wakeup: {=bool:?}, error: {=bool:?}, int: {=bool:?}, data_req: {=bool:?}, aborted: {=bool:?}, bus_idle_tv: {=bool:?}, lin_active: {=bool:?} }}" , self . complete () , self . wakeup () , self . error () , self . int () , self . data_req () , self . aborted () , self . bus_idle_tv () , self . lin_active ())
        }
    }
    #[doc = "timeout control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tv(pub u32);
    impl Tv {
        #[doc = "slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms."]
        #[must_use]
        #[inline(always)]
        pub const fn wup_repeat_time(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms."]
        #[inline(always)]
        pub const fn set_wup_repeat_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_inactivity_time(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s."]
        #[inline(always)]
        pub const fn set_bus_inactivity_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "master_mode."]
        #[must_use]
        #[inline(always)]
        pub const fn master_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "master_mode."]
        #[inline(always)]
        pub const fn set_master_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "initial_mode."]
        #[must_use]
        #[inline(always)]
        pub const fn initial_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "initial_mode."]
        #[inline(always)]
        pub const fn set_initial_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Tv {
        #[inline(always)]
        fn default() -> Tv {
            Tv(0)
        }
    }
    impl core::fmt::Debug for Tv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tv")
                .field("wup_repeat_time", &self.wup_repeat_time())
                .field("bus_inactivity_time", &self.bus_inactivity_time())
                .field("master_mode", &self.master_mode())
                .field("initial_mode", &self.initial_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tv {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tv {{ wup_repeat_time: {=u8:?}, bus_inactivity_time: {=u8:?}, master_mode: {=bool:?}, initial_mode: {=bool:?} }}" , self . wup_repeat_time () , self . bus_inactivity_time () , self . master_mode () , self . initial_mode ())
        }
    }
}
