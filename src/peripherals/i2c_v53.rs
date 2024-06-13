#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "I2C0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Address Register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Data Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Setup Register."]
    #[inline(always)]
    pub const fn setup(self) -> crate::common::Reg<regs::Setup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "I2C Timing Paramater Multiplier."]
    #[inline(always)]
    pub const fn tpm(self) -> crate::common::Reg<regs::Tpm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid."]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "FIFO Size: 0: 2 bytes 1: 4 bytes 2: 8 bytes 3: 16 bytes."]
        #[inline(always)]
        pub const fn fifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO Size: 0: 2 bytes 1: 4 bytes 2: 8 bytes 3: 16 bytes."]
        #[inline(always)]
        pub fn set_fifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Write this register with the following values to perform the corresponding actions: 0x0: no action 0x1: issue a data transaction (Master only) 0x2: respond with an ACK to the received byte 0x3: respond with a NACK to the received byte 0x4: clear the FIFO 0x5: reset the I2C controller (abort current transaction, set the SDA and SCL line to the open-drain mode, reset the Status Register and the Interrupt Enable Register, and empty the FIFO) When issuing a data transaction by writing 0x1 to this register, the CMD field stays at 0x1 for the duration of the entire transaction, and it is only cleared to 0x0 after when the transaction has completed or when the controller loses the arbitration. Note: No transaction will be issued by the controller when all phases (Start, Address, Data and Stop) are disabled."]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Write this register with the following values to perform the corresponding actions: 0x0: no action 0x1: issue a data transaction (Master only) 0x2: respond with an ACK to the received byte 0x3: respond with a NACK to the received byte 0x4: clear the FIFO 0x5: reset the I2C controller (abort current transaction, set the SDA and SCL line to the open-drain mode, reset the Status Register and the Interrupt Enable Register, and empty the FIFO) When issuing a data transaction by writing 0x1 to this register, the CMD field stays at 0x1 for the duration of the entire transaction, and it is only cleared to 0x0 after when the transaction has completed or when the controller loses the arbitration. Note: No transaction will be issued by the controller when all phases (Start, Address, Data and Stop) are disabled."]
        #[inline(always)]
        pub fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
        #[inline(always)]
        pub const fn datacnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
        #[inline(always)]
        pub fn set_datacnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
        #[inline(always)]
        pub const fn phase_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
        #[inline(always)]
        pub fn set_phase_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable this bit to send the data after Address phase. Master mode only."]
        #[inline(always)]
        pub const fn phase_data(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this bit to send the data after Address phase. Master mode only."]
        #[inline(always)]
        pub fn set_phase_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable this bit to send the address after START condition. Master mode only."]
        #[inline(always)]
        pub const fn phase_addr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this bit to send the address after START condition. Master mode only."]
        #[inline(always)]
        pub fn set_phase_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
        #[inline(always)]
        pub const fn phase_start(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
        #[inline(always)]
        pub fn set_phase_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software."]
        #[inline(always)]
        pub const fn reset_on(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "set to send reset signals(just toggle clock bus defined by reset_len). this register is clered when reset is end, can't be cleared by software."]
        #[inline(always)]
        pub fn set_reset_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "set to hold input clock to high when reset is active."]
        #[inline(always)]
        pub const fn reset_hold_sckin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "set to hold input clock to high when reset is active."]
        #[inline(always)]
        pub fn set_reset_hold_sckin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle."]
        #[inline(always)]
        pub const fn reset_len(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "reset clock cycles. the clock high/low time is defined by Setup.T_SCLHi, 50% duty cycle."]
        #[inline(always)]
        pub fn set_reset_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
        #[inline(always)]
        pub const fn datacnt_high(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
        #[inline(always)]
        pub fn set_datacnt_high(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO."]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
        #[inline(always)]
        pub const fn fifoempty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
        #[inline(always)]
        pub fn set_fifoempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
        #[inline(always)]
        pub const fn fifofull(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
        #[inline(always)]
        pub fn set_fifofull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
        #[inline(always)]
        pub const fn fifohalf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
        #[inline(always)]
        pub fn set_fifohalf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
        #[inline(always)]
        pub const fn addrhit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
        #[inline(always)]
        pub fn set_addrhit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
        #[inline(always)]
        pub const fn arblose(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
        #[inline(always)]
        pub fn set_arblose(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
        #[inline(always)]
        pub const fn bytetrans(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
        #[inline(always)]
        pub fn set_bytetrans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
        #[inline(always)]
        pub const fn byterecv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
        #[inline(always)]
        pub fn set_byterecv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
        #[inline(always)]
        pub const fn cmpl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
        #[inline(always)]
        pub fn set_cmpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "Setup Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Setup(pub u32);
    impl Setup {
        #[doc = "Enable the I2C controller. 1: Enable 0: Disable."]
        #[inline(always)]
        pub const fn iicen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the I2C controller. 1: Enable 0: Disable."]
        #[inline(always)]
        pub fn set_iicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode."]
        #[inline(always)]
        pub const fn addressing(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode."]
        #[inline(always)]
        pub fn set_addressing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Configure this device as a master or a slave. 1: Master mode 0: Slave mode."]
        #[inline(always)]
        pub const fn master(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Configure this device as a master or a slave. 1: Master mode 0: Slave mode."]
        #[inline(always)]
        pub fn set_master(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable the direct memory access mode data transfer. 1: Enable 0: Disable."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the direct memory access mode data transfer. 1: Enable 0: Disable."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
        #[inline(always)]
        pub const fn t_sclhi(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x01ff;
            val as u16
        }
        #[doc = "The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode."]
        #[inline(always)]
        pub fn set_t_sclhi(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 4usize)) | (((val as u32) & 0x01ff) << 4usize);
        }
        #[doc = "The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
        #[inline(always)]
        pub const fn t_sclradio(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode."]
        #[inline(always)]
        pub fn set_t_sclradio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)."]
        #[inline(always)]
        pub const fn t_hddat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1)."]
        #[inline(always)]
        pub fn set_t_hddat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)."]
        #[inline(always)]
        pub const fn t_sp(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1)."]
        #[inline(always)]
        pub fn set_t_sp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register."]
        #[inline(always)]
        pub const fn t_sudat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register."]
        #[inline(always)]
        pub fn set_t_sudat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Setup {
        #[inline(always)]
        fn default() -> Setup {
            Setup(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Indicates that the FIFO is empty."]
        #[inline(always)]
        pub const fn fifoempty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the FIFO is empty."]
        #[inline(always)]
        pub fn set_fifoempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicates that the FIFO is full."]
        #[inline(always)]
        pub const fn fifofull(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the FIFO is full."]
        #[inline(always)]
        pub fn set_fifofull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter: Indicates that the FIFO is half-empty."]
        #[inline(always)]
        pub const fn fifohalf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter: Indicates that the FIFO is half-empty."]
        #[inline(always)]
        pub fn set_fifohalf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
        #[inline(always)]
        pub const fn addrhit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
        #[inline(always)]
        pub fn set_addrhit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Indicates that the controller has lost the bus arbitration."]
        #[inline(always)]
        pub const fn arblose(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the controller has lost the bus arbitration."]
        #[inline(always)]
        pub fn set_arblose(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Indicates that a STOP Condition has been transmitted/received."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that a STOP Condition has been transmitted/received."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Indicates that a START Condition or a repeated START condition has been transmitted/received."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that a START Condition or a repeated START condition has been transmitted/received."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Indicates that a byte of data has been transmitted."]
        #[inline(always)]
        pub const fn bytetrans(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that a byte of data has been transmitted."]
        #[inline(always)]
        pub fn set_bytetrans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Indicates that a byte of data has been received."]
        #[inline(always)]
        pub const fn byterecv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that a byte of data has been received."]
        #[inline(always)]
        pub fn set_byterecv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
        #[inline(always)]
        pub const fn cmpl(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
        #[inline(always)]
        pub fn set_cmpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK."]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK."]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy."]
        #[inline(always)]
        pub const fn busbusy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy."]
        #[inline(always)]
        pub fn set_busbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call."]
        #[inline(always)]
        pub const fn gencall(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call."]
        #[inline(always)]
        pub fn set_gencall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Indicates the current status of the SCL line on the bus 1: high 0: low."]
        #[inline(always)]
        pub const fn linescl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates the current status of the SCL line on the bus 1: high 0: low."]
        #[inline(always)]
        pub fn set_linescl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Indicates the current status of the SDA line on the bus 1: high 0: low."]
        #[inline(always)]
        pub const fn linesda(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates the current status of the SDA line on the bus 1: high 0: low."]
        #[inline(always)]
        pub fn set_linesda(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "I2C Timing Paramater Multiplier."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tpm(pub u32);
    impl Tpm {
        #[doc = "A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
        #[inline(always)]
        pub const fn tpm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
        #[inline(always)]
        pub fn set_tpm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Tpm {
        #[inline(always)]
        fn default() -> Tpm {
            Tpm(0)
        }
    }
}
