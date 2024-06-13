#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MBX0A."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbx {
    ptr: *mut u8,
}
unsafe impl Send for Mbx {}
unsafe impl Sync for Mbx {}
impl Mbx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command Registers."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status Registers."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Transmit word message to other core."]
    #[inline(always)]
    pub const fn txreg(self) -> crate::common::Reg<regs::Txreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Receive word message from other core."]
    #[inline(always)]
    pub const fn rxreg(self) -> crate::common::Reg<regs::Rxreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txwrd(self, n: usize) -> crate::common::Reg<regs::Txwrd, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxwrd(self, n: usize) -> crate::common::Reg<regs::Rxwrd, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Command Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
        #[inline(always)]
        pub const fn rwmvie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
        #[inline(always)]
        pub fn set_rwmvie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
        #[inline(always)]
        pub const fn twmeie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
        #[inline(always)]
        pub fn set_twmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
        #[inline(always)]
        pub const fn rfmfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
        #[inline(always)]
        pub fn set_rfmfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
        #[inline(always)]
        pub const fn rfmaie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
        #[inline(always)]
        pub fn set_rfmaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
        #[inline(always)]
        pub const fn tfmeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
        #[inline(always)]
        pub fn set_tfmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
        #[inline(always)]
        pub const fn tfmaie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
        #[inline(always)]
        pub fn set_tfmaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
        #[inline(always)]
        pub const fn beie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
        #[inline(always)]
        pub fn set_beie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
        #[inline(always)]
        pub const fn barctl(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Bus Access Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
        #[inline(always)]
        pub fn set_barctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Reset TX Fifo and word."]
        #[inline(always)]
        pub const fn txreset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reset TX Fifo and word."]
        #[inline(always)]
        pub fn set_txreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Receive word message from other core."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxreg(pub u32);
    impl Rxreg {
        #[doc = "Receive word message from other core."]
        #[inline(always)]
        pub const fn rxreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive word message from other core."]
        #[inline(always)]
        pub fn set_rxreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxreg {
        #[inline(always)]
        fn default() -> Rxreg {
            Rxreg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxwrd(pub u32);
    impl Rxwrd {
        #[doc = "RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO."]
        #[inline(always)]
        pub const fn rxfifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO."]
        #[inline(always)]
        pub fn set_rxfifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxwrd {
        #[inline(always)]
        fn default() -> Rxwrd {
            Rxwrd(0)
        }
    }
    #[doc = "Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
        #[inline(always)]
        pub const fn rwmv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
        #[inline(always)]
        pub fn set_rwmv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
        #[inline(always)]
        pub const fn twme(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
        #[inline(always)]
        pub fn set_twme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
        #[inline(always)]
        pub const fn rfmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
        #[inline(always)]
        pub fn set_rfmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
        #[inline(always)]
        pub const fn rfma(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
        #[inline(always)]
        pub fn set_rfma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
        #[inline(always)]
        pub const fn tfme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
        #[inline(always)]
        pub fn set_tfme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)."]
        #[inline(always)]
        pub const fn tfma(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)."]
        #[inline(always)]
        pub fn set_tfma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn ew2ro(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_ew2ro(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn eaiva(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_eaiva(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn ewtff(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_ewtff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn errfe(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_errfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn ewtrf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_ewtrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub const fn errre(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
        #[inline(always)]
        pub fn set_errre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TX FIFO empty message word count."]
        #[inline(always)]
        pub const fn tfec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX FIFO empty message word count."]
        #[inline(always)]
        pub fn set_tfec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RX FIFO valid message count."]
        #[inline(always)]
        pub const fn rfvc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "RX FIFO valid message count."]
        #[inline(always)]
        pub fn set_rfvc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Transmit word message to other core."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txreg(pub u32);
    impl Txreg {
        #[doc = "Transmit word message to other core."]
        #[inline(always)]
        pub const fn txreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit word message to other core."]
        #[inline(always)]
        pub fn set_txreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txreg {
        #[inline(always)]
        fn default() -> Txreg {
            Txreg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txwrd(pub u32);
    impl Txwrd {
        #[doc = "TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
        #[inline(always)]
        pub const fn txfifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
        #[inline(always)]
        pub fn set_txfifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txwrd {
        #[inline(always)]
        fn default() -> Txwrd {
            Txwrd(0)
        }
    }
}
