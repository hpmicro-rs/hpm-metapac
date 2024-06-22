#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "UART0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
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
    #[doc = "Over Sample Control Register."]
    #[inline(always)]
    pub const fn oscr(self) -> crate::common::Reg<regs::Oscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Divisor Latch LSB (when DLAB = 1)."]
    #[inline(always)]
    pub const fn dll(self) -> crate::common::Reg<regs::Dll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receiver Buffer Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Transmitter Holding Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Divisor Latch MSB (when DLAB = 1)."]
    #[inline(always)]
    pub const fn dlm(self) -> crate::common::Reg<regs::Dlm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register (when DLAB = 0)."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FIFO Control Register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt Identification Register."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Line Control Register."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Modem Control Register (."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Line Status Register."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Modem Status Register."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "GPR Register."]
    #[inline(always)]
    pub const fn gpr(self) -> crate::common::Reg<regs::Gpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO."]
        #[inline(always)]
        pub const fn fifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO."]
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
    #[doc = "Divisor Latch LSB (when DLAB = 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dll(pub u32);
    impl Dll {
        #[doc = "Least significant byte of the Divisor Latch."]
        #[inline(always)]
        pub const fn dll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Least significant byte of the Divisor Latch."]
        #[inline(always)]
        pub fn set_dll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dll {
        #[inline(always)]
        fn default() -> Dll {
            Dll(0)
        }
    }
    #[doc = "Divisor Latch MSB (when DLAB = 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlm(pub u32);
    impl Dlm {
        #[doc = "Most significant byte of the Divisor Latch."]
        #[inline(always)]
        pub const fn dlm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Most significant byte of the Divisor Latch."]
        #[inline(always)]
        pub fn set_dlm(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dlm {
        #[inline(always)]
        fn default() -> Dlm {
            Dlm(0)
        }
    }
    #[doc = "FIFO Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[inline(always)]
        pub const fn fifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
        #[inline(always)]
        pub fn set_fifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn rfiforst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub fn set_rfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub const fn tfiforst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
        #[inline(always)]
        pub fn set_tfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn dmae(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable 0: Disable 1: Enable."]
        #[inline(always)]
        pub fn set_dmae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[inline(always)]
        pub const fn tfifot(&self) -> super::vals::TxFifoTrigger {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::TxFifoTrigger::from_bits(val as u8)
        }
        #[doc = "Transmitter FIFO trigger level."]
        #[inline(always)]
        pub fn set_tfifot(&mut self, val: super::vals::TxFifoTrigger) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Receiver FIFO trigger level."]
        #[inline(always)]
        pub const fn rfifot(&self) -> super::vals::RxFifoTrigger {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::RxFifoTrigger::from_bits(val as u8)
        }
        #[doc = "Receiver FIFO trigger level."]
        #[inline(always)]
        pub fn set_rfifot(&mut self, val: super::vals::RxFifoTrigger) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    #[doc = "GPR Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr(pub u32);
    impl Gpr {
        #[doc = "A one-byte storage register."]
        #[inline(always)]
        pub const fn data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "A one-byte storage register."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Gpr {
        #[inline(always)]
        fn default() -> Gpr {
            Gpr(0)
        }
    }
    #[doc = "Interrupt Enable Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn erbi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable."]
        #[inline(always)]
        pub fn set_erbi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable transmitter holding register interrupt."]
        #[inline(always)]
        pub const fn ethei(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable transmitter holding register interrupt."]
        #[inline(always)]
        pub fn set_ethei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable receiver line status interrupt."]
        #[inline(always)]
        pub const fn elsi(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable receiver line status interrupt."]
        #[inline(always)]
        pub fn set_elsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
        #[inline(always)]
        pub const fn emsi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
        #[inline(always)]
        pub fn set_emsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "Interrupt Identification Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir(pub u32);
    impl Iir {
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[inline(always)]
        pub const fn intrid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID, see IIR2 for detail decoding."]
        #[inline(always)]
        pub fn set_intrid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[inline(always)]
        pub const fn fifoed(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
        #[inline(always)]
        pub fn set_fifoed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Iir {
        #[inline(always)]
        fn default() -> Iir {
            Iir(0)
        }
    }
    #[doc = "Line Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u32);
    impl Lcr {
        #[doc = "Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits."]
        #[inline(always)]
        pub const fn wls(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits."]
        #[inline(always)]
        pub fn set_wls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits."]
        #[inline(always)]
        pub const fn stb(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits."]
        #[inline(always)]
        pub fn set_stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
        #[inline(always)]
        pub const fn pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
        #[inline(always)]
        pub fn set_pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
        #[inline(always)]
        pub const fn eps(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
        #[inline(always)]
        pub fn set_eps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
        #[inline(always)]
        pub const fn sps(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
        #[inline(always)]
        pub fn set_sps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break control."]
        #[inline(always)]
        pub const fn bc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Break control."]
        #[inline(always)]
        pub fn set_bc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Divisor latch access bit."]
        #[inline(always)]
        pub const fn dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Divisor latch access bit."]
        #[inline(always)]
        pub fn set_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    #[doc = "Line Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u32);
    impl Lsr {
        #[doc = "Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read."]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read."]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun."]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun."]
        #[inline(always)]
        pub fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error This bit is set when the received parity does not match with the parity selected in the LCR\\[5:4\\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error This bit is set when the received parity does not match with the parity selected in the LCR\\[5:4\\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub const fn lbreak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO."]
        #[inline(always)]
        pub fn set_lbreak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1."]
        #[inline(always)]
        pub const fn thre(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1."]
        #[inline(always)]
        pub fn set_thre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero."]
        #[inline(always)]
        pub const fn temt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero."]
        #[inline(always)]
        pub fn set_temt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lsr {
        #[inline(always)]
        fn default() -> Lsr {
            Lsr(0)
        }
    }
    #[doc = "Modem Control Register (."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW."]
        #[inline(always)]
        pub const fn rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW."]
        #[inline(always)]
        pub fn set_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable loopback mode 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn loop_(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable loopback mode 0: Disable 1: Enable."]
        #[inline(always)]
        pub fn set_loop_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS."]
        #[inline(always)]
        pub const fn afe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS."]
        #[inline(always)]
        pub fn set_afe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "Modem Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u32);
    impl Msr {
        #[doc = "Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
        #[inline(always)]
        pub const fn dcts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
        #[inline(always)]
        pub fn set_dcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    #[doc = "Over Sample Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oscr(pub u32);
    impl Oscr {
        #[doc = "Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC."]
        #[inline(always)]
        pub const fn osc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: reserved OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC."]
        #[inline(always)]
        pub fn set_osc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Oscr {
        #[inline(always)]
        fn default() -> Oscr {
            Oscr(0)
        }
    }
    #[doc = "Receiver Buffer Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u32);
    impl Rbr {
        #[doc = "Receive data read port."]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive data read port."]
        #[inline(always)]
        pub fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rbr {
        #[inline(always)]
        fn default() -> Rbr {
            Rbr(0)
        }
    }
    #[doc = "Transmitter Holding Register (when DLAB = 0)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Thr(pub u32);
    impl Thr {
        #[doc = "Transmit data write port."]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit data write port."]
        #[inline(always)]
        pub fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Thr {
        #[inline(always)]
        fn default() -> Thr {
            Thr(0)
        }
    }
}
pub mod vals {
    #[doc = "Transmitter FIFO trigger level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RxFifoTrigger {
        #[doc = "1 byte"]
        NOT_EMPTY = 0x0,
        #[doc = "1/4 full"]
        GT_ONE_QUARTER = 0x01,
        #[doc = "1/2 full"]
        GT_HALF = 0x02,
        #[doc = "3/4 full"]
        GT_THREE_QUARTER = 0x03,
    }
    impl RxFifoTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RxFifoTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RxFifoTrigger {
        #[inline(always)]
        fn from(val: u8) -> RxFifoTrigger {
            RxFifoTrigger::from_bits(val)
        }
    }
    impl From<RxFifoTrigger> for u8 {
        #[inline(always)]
        fn from(val: RxFifoTrigger) -> u8 {
            RxFifoTrigger::to_bits(val)
        }
    }
    #[doc = "Receiver FIFO trigger level."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TxFifoTrigger {
        #[doc = "1 byte or more empty space in fifo"]
        NOT_FULL = 0x0,
        #[doc = "3/4 full"]
        LT_THREE_QUARTER = 0x01,
        #[doc = "1/2 full"]
        LT_HALF = 0x02,
        #[doc = "1/4 full"]
        LT_ONE_QUARTER = 0x03,
    }
    impl TxFifoTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TxFifoTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TxFifoTrigger {
        #[inline(always)]
        fn from(val: u8) -> TxFifoTrigger {
            TxFifoTrigger::from_bits(val)
        }
    }
    impl From<TxFifoTrigger> for u8 {
        #[inline(always)]
        fn from(val: TxFifoTrigger) -> u8 {
            TxFifoTrigger::to_bits(val)
        }
    }
}
