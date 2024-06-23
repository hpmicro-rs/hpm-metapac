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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "state register."]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "error register."]
    #[inline(always)]
    pub const fn error(self) -> crate::common::Reg<regs::Error, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "data lenth register."]
    #[inline(always)]
    pub const fn data_len(self) -> crate::common::Reg<regs::DataLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "baudrate control low register."]
    #[inline(always)]
    pub const fn baudrate_ctl_low(
        self,
    ) -> crate::common::Reg<regs::BaudrateCtlLow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "baudrate control high register."]
    #[inline(always)]
    pub const fn bardrate_ctl_high(
        self,
    ) -> crate::common::Reg<regs::BardrateCtlHigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "id register."]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "timeout control register."]
    #[inline(always)]
    pub const fn tv(self) -> crate::common::Reg<regs::Tv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "baudrate control high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BardrateCtlHigh(pub u32);
    impl BardrateCtlHigh {
        #[doc = "bit div register 8."]
        #[inline(always)]
        pub const fn bt_div_high(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bit div register 8."]
        #[inline(always)]
        pub fn set_bt_div_high(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "bt_mul register."]
        #[inline(always)]
        pub const fn bt_mul(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x1f;
            val as u8
        }
        #[doc = "bt_mul register."]
        #[inline(always)]
        pub fn set_bt_mul(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
        }
        #[doc = "prescl register."]
        #[inline(always)]
        pub const fn prescl(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "prescl register."]
        #[inline(always)]
        pub fn set_prescl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for BardrateCtlHigh {
        #[inline(always)]
        fn default() -> BardrateCtlHigh {
            BardrateCtlHigh(0)
        }
    }
    #[doc = "baudrate control low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BaudrateCtlLow(pub u32);
    impl BaudrateCtlLow {
        #[doc = "bit div register 7:0."]
        #[inline(always)]
        pub const fn bt_div_low(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "bit div register 7:0."]
        #[inline(always)]
        pub fn set_bt_div_low(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for BaudrateCtlLow {
        #[inline(always)]
        fn default() -> BaudrateCtlLow {
            BaudrateCtlLow(0)
        }
    }
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u32);
    impl Control {
        #[doc = "master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred."]
        #[inline(always)]
        pub const fn start_req(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred."]
        #[inline(always)]
        pub fn set_start_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core."]
        #[inline(always)]
        pub const fn wakeup_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core."]
        #[inline(always)]
        pub fn set_wakeup_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0."]
        #[inline(always)]
        pub const fn reset_error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0."]
        #[inline(always)]
        pub fn set_reset_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "write 1 to reset the int bit in the status register and the interrupt request output of LIN."]
        #[inline(always)]
        pub const fn reset_int(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "write 1 to reset the int bit in the status register and the interrupt request output of LIN."]
        #[inline(always)]
        pub fn set_reset_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "slave only. Write 1 after handling a data request interrupt."]
        #[inline(always)]
        pub const fn data_ack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Write 1 after handling a data request interrupt."]
        #[inline(always)]
        pub fn set_data_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1: transmit operation 0: receive operation."]
        #[inline(always)]
        pub const fn transmit(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1: transmit operation 0: receive operation."]
        #[inline(always)]
        pub fn set_transmit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
        #[inline(always)]
        pub fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    #[doc = "data lenth register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataLen(pub u32);
    impl DataLen {
        #[doc = "data length."]
        #[inline(always)]
        pub const fn data_length(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "data length."]
        #[inline(always)]
        pub fn set_data_length(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "1:enhence check mode."]
        #[inline(always)]
        pub const fn enh_check(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1:enhence check mode."]
        #[inline(always)]
        pub fn set_enh_check(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for DataLen {
        #[inline(always)]
        fn default() -> DataLen {
            DataLen(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Databyte(pub u32);
    impl Databyte {
        #[doc = "data byte."]
        #[inline(always)]
        pub const fn data_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "data byte."]
        #[inline(always)]
        pub fn set_data_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Databyte {
        #[inline(always)]
        fn default() -> Databyte {
            Databyte(0)
        }
    }
    #[doc = "error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Error(pub u32);
    impl Error {
        #[doc = "bit error."]
        #[inline(always)]
        pub const fn bit_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bit error."]
        #[inline(always)]
        pub fn set_bit_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "checksum error."]
        #[inline(always)]
        pub const fn chk_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "checksum error."]
        #[inline(always)]
        pub fn set_chk_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms."]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms."]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "slave only. identifier parity error."]
        #[inline(always)]
        pub const fn parity_error(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. identifier parity error."]
        #[inline(always)]
        pub fn set_parity_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Error {
        #[inline(always)]
        fn default() -> Error {
            Error(0)
        }
    }
    #[doc = "id register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Id(pub u32);
    impl Id {
        #[doc = "id register."]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "id register."]
        #[inline(always)]
        pub fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Id {
        #[inline(always)]
        fn default() -> Id {
            Id(0)
        }
    }
    #[doc = "state register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State(pub u32);
    impl State {
        #[doc = "set after a transmission has been successful finished and it will reset at the start of a transmission."]
        #[inline(always)]
        pub const fn complete(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set after a transmission has been successful finished and it will reset at the start of a transmission."]
        #[inline(always)]
        pub fn set_complete(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1."]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1."]
        #[inline(always)]
        pub fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set when detecte an error, clear by reset_error."]
        #[inline(always)]
        pub const fn error(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set when detecte an error, clear by reset_error."]
        #[inline(always)]
        pub fn set_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set when request an interrupt. Reset by reset_int."]
        #[inline(always)]
        pub const fn int(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set when request an interrupt. Reset by reset_int."]
        #[inline(always)]
        pub fn set_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
        #[inline(always)]
        pub const fn data_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. Sets after receiving the identifier and requests an interrupt to the host controller."]
        #[inline(always)]
        pub fn set_data_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
        #[inline(always)]
        pub const fn aborted(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error."]
        #[inline(always)]
        pub fn set_aborted(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s."]
        #[inline(always)]
        pub const fn bus_idle_tv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s."]
        #[inline(always)]
        pub fn set_bus_idle_tv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The bit indicates whether the LIN bus is active or not."]
        #[inline(always)]
        pub const fn lin_active(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The bit indicates whether the LIN bus is active or not."]
        #[inline(always)]
        pub fn set_lin_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for State {
        #[inline(always)]
        fn default() -> State {
            State(0)
        }
    }
    #[doc = "timeout control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tv(pub u32);
    impl Tv {
        #[doc = "slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms."]
        #[inline(always)]
        pub const fn wup_repeat_time(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms."]
        #[inline(always)]
        pub fn set_wup_repeat_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s."]
        #[inline(always)]
        pub const fn bus_inactivity_time(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s."]
        #[inline(always)]
        pub fn set_bus_inactivity_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "master_mode."]
        #[inline(always)]
        pub const fn master_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "master_mode."]
        #[inline(always)]
        pub fn set_master_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "initial_mode."]
        #[inline(always)]
        pub const fn initial_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "initial_mode."]
        #[inline(always)]
        pub fn set_initial_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Tv {
        #[inline(always)]
        fn default() -> Tv {
            Tv(0)
        }
    }
}
