#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SPI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Transfer Format Register."]
    #[inline(always)]
    pub const fn trans_fmt(self) -> crate::common::Reg<regs::TransFmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Transfer Control Register."]
    #[inline(always)]
    pub const fn trans_ctrl(self) -> crate::common::Reg<regs::TransCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Address Register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Data Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn intr_en(self) -> crate::common::Reg<regs::IntrEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn intr_st(self) -> crate::common::Reg<regs::IntrSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interface Timing Register."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Slave Status Register."]
    #[inline(always)]
    pub const fn slv_st(self) -> crate::common::Reg<regs::SlvSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Slave Data Count Register."]
    #[inline(always)]
    pub const fn slv_data_cnt(self) -> crate::common::Reg<regs::SlvDataCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
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
    #[doc = "Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addr(pub u32);
    impl Addr {
        #[doc = "SPI Address (Master mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SPI Address (Master mode only)."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addr {
        #[inline(always)]
        fn default() -> Addr {
            Addr(0)
        }
    }
    impl core::fmt::Debug for Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "SPI Command."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI Command."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd").field("cmd", &self.cmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmd {{ cmd: {=u8:?} }}", self.cmd())
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifosize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words."]
        #[inline(always)]
        pub const fn set_rxfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifosize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words."]
        #[inline(always)]
        pub const fn set_txfifosize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Support for Dual I/O SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn dualspi(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Support for Dual I/O SPI."]
        #[inline(always)]
        pub const fn set_dualspi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Support for Quad I/O SPI."]
        #[must_use]
        #[inline(always)]
        pub const fn quadspi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Support for Quad I/O SPI."]
        #[inline(always)]
        pub const fn set_quadspi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Support for SPI Slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn slave(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Support for SPI Slave mode."]
        #[inline(always)]
        pub const fn set_slave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("rxfifosize", &self.rxfifosize())
                .field("txfifosize", &self.txfifosize())
                .field("dualspi", &self.dualspi())
                .field("quadspi", &self.quadspi())
                .field("slave", &self.slave())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Config {{ rxfifosize: {=u8:?}, txfifosize: {=u8:?}, dualspi: {=bool:?}, quadspi: {=bool:?}, slave: {=bool:?} }}" , self . rxfifosize () , self . txfifosize () , self . dualspi () , self . quadspi () , self . slave ())
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[must_use]
        #[inline(always)]
        pub const fn spirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[inline(always)]
        pub const fn set_spirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfiforst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[inline(always)]
        pub const fn set_rxfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[must_use]
        #[inline(always)]
        pub const fn txfiforst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
        #[inline(always)]
        pub const fn set_txfiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RX DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RX DMA enable."]
        #[inline(always)]
        pub const fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX DMA enable."]
        #[inline(always)]
        pub const fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn rxthres(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
        #[inline(always)]
        pub const fn set_rxthres(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn txthres(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
        #[inline(always)]
        pub const fn set_txthres(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("spirst", &self.spirst())
                .field("rxfiforst", &self.rxfiforst())
                .field("txfiforst", &self.txfiforst())
                .field("rxdmaen", &self.rxdmaen())
                .field("txdmaen", &self.txdmaen())
                .field("rxthres", &self.rxthres())
                .field("txthres", &self.txthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ spirst: {=bool:?}, rxfiforst: {=bool:?}, txfiforst: {=bool:?}, rxdmaen: {=bool:?}, txdmaen: {=bool:?}, rxthres: {=u8:?}, txthres: {=u8:?} }}" , self . spirst () , self . rxfiforst () , self . txfiforst () , self . rxdmaen () , self . txdmaen () , self . rxthres () , self . txthres ())
        }
    }
    #[doc = "Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "Data to transmit or the received data For writes, data is enqueued to the TX FIFO. The least significant byte is always transmitted first. If the TX FIFO is full and the SPIActive bit of the status register is 1, the ready signal hready/pready will be deasserted to insert wait states to the transfer. For reads, data is read and dequeued from the RX FIFO. The least significant byte is the first received byte. If the RX FIFO is empty and the SPIActive bit of the status register is 1, the ready signal hready/pready will be deasserted to insert wait states to the transfer. The FIFOs decouple the speed of the SPI transfers and the software鈥檚 generation/consumption of data. When the TX FIFO is empty, SPI transfers will hold until more data is written to the TX FIFO; when the RX FIFO is full, SPI transfers will hold until there is more room in the RX FIFO. If more data is written to the TX FIFO than the write transfer count (WrTranCnt), the remaining data will stay in the TX FIFO for the next transfer or until the TX FIFO is reset."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data to transmit or the received data For writes, data is enqueued to the TX FIFO. The least significant byte is always transmitted first. If the TX FIFO is full and the SPIActive bit of the status register is 1, the ready signal hready/pready will be deasserted to insert wait states to the transfer. For reads, data is read and dequeued from the RX FIFO. The least significant byte is the first received byte. If the RX FIFO is empty and the SPIActive bit of the status register is 1, the ready signal hready/pready will be deasserted to insert wait states to the transfer. The FIFOs decouple the speed of the SPI transfers and the software鈥檚 generation/consumption of data. When the TX FIFO is empty, SPI transfers will hold until more data is written to the TX FIFO; when the RX FIFO is full, SPI transfers will hold until there is more room in the RX FIFO. If more data is written to the TX FIFO than the write transfer count (WrTranCnt), the remaining data will stay in the TX FIFO for the next transfer or until the TX FIFO is reset."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntrEn(pub u32);
    impl IntrEn {
        #[doc = "Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoorinten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_rxfifoorinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifourinten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_txfifourinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifointen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
        #[inline(always)]
        pub const fn set_rxfifointen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifointen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
        #[inline(always)]
        pub const fn set_txfifointen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)."]
        #[must_use]
        #[inline(always)]
        pub const fn endinten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)."]
        #[inline(always)]
        pub const fn set_endinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn slvcmden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_slvcmden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntrEn {
        #[inline(always)]
        fn default() -> IntrEn {
            IntrEn(0)
        }
    }
    impl core::fmt::Debug for IntrEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntrEn")
                .field("rxfifoorinten", &self.rxfifoorinten())
                .field("txfifourinten", &self.txfifourinten())
                .field("rxfifointen", &self.rxfifointen())
                .field("txfifointen", &self.txfifointen())
                .field("endinten", &self.endinten())
                .field("slvcmden", &self.slvcmden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntrEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntrEn {{ rxfifoorinten: {=bool:?}, txfifourinten: {=bool:?}, rxfifointen: {=bool:?}, txfifointen: {=bool:?}, endinten: {=bool:?}, slvcmden: {=bool:?} }}" , self . rxfifoorinten () , self . txfifourinten () , self . rxfifointen () , self . txfifointen () , self . endinten () , self . slvcmden ())
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntrSt(pub u32);
    impl IntrSt {
        #[doc = "RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoorint(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_rxfifoorint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifourint(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_txfifourint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoint(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
        #[inline(always)]
        pub const fn set_rxfifoint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoint(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
        #[inline(always)]
        pub const fn set_txfifoint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
        #[must_use]
        #[inline(always)]
        pub const fn endint(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
        #[inline(always)]
        pub const fn set_endint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)."]
        #[must_use]
        #[inline(always)]
        pub const fn slvcmdint(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)."]
        #[inline(always)]
        pub const fn set_slvcmdint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for IntrSt {
        #[inline(always)]
        fn default() -> IntrSt {
            IntrSt(0)
        }
    }
    impl core::fmt::Debug for IntrSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntrSt")
                .field("rxfifoorint", &self.rxfifoorint())
                .field("txfifourint", &self.txfifourint())
                .field("rxfifoint", &self.rxfifoint())
                .field("txfifoint", &self.txfifoint())
                .field("endint", &self.endint())
                .field("slvcmdint", &self.slvcmdint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntrSt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntrSt {{ rxfifoorint: {=bool:?}, txfifourint: {=bool:?}, rxfifoint: {=bool:?}, txfifoint: {=bool:?}, endint: {=bool:?}, slvcmdint: {=bool:?} }}" , self . rxfifoorint () , self . txfifourint () , self . rxfifoint () , self . txfifoint () , self . endint () , self . slvcmdint ())
        }
    }
    #[doc = "Slave Data Count Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlvDataCnt(pub u32);
    impl SlvDataCnt {
        #[doc = "Slave received data count."]
        #[must_use]
        #[inline(always)]
        pub const fn rcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Slave received data count."]
        #[inline(always)]
        pub const fn set_rcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Slave transmitted data count."]
        #[must_use]
        #[inline(always)]
        pub const fn wcnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Slave transmitted data count."]
        #[inline(always)]
        pub const fn set_wcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for SlvDataCnt {
        #[inline(always)]
        fn default() -> SlvDataCnt {
            SlvDataCnt(0)
        }
    }
    impl core::fmt::Debug for SlvDataCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlvDataCnt")
                .field("rcnt", &self.rcnt())
                .field("wcnt", &self.wcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SlvDataCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SlvDataCnt {{ rcnt: {=u16:?}, wcnt: {=u16:?} }}",
                self.rcnt(),
                self.wcnt()
            )
        }
    }
    #[doc = "Slave Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlvSt(pub u32);
    impl SlvSt {
        #[doc = "User defined status flags."]
        #[must_use]
        #[inline(always)]
        pub const fn usr_status(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "User defined status flags."]
        #[inline(always)]
        pub const fn set_usr_status(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Data overrun occurs in the last transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn overrun(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Data overrun occurs in the last transaction."]
        #[inline(always)]
        pub const fn set_overrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Data underrun occurs in the last transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn underrun(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Data underrun occurs in the last transaction."]
        #[inline(always)]
        pub const fn set_underrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for SlvSt {
        #[inline(always)]
        fn default() -> SlvSt {
            SlvSt(0)
        }
    }
    impl core::fmt::Debug for SlvSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlvSt")
                .field("usr_status", &self.usr_status())
                .field("ready", &self.ready())
                .field("overrun", &self.overrun())
                .field("underrun", &self.underrun())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SlvSt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SlvSt {{ usr_status: {=u16:?}, ready: {=bool:?}, overrun: {=bool:?}, underrun: {=bool:?} }}" , self . usr_status () , self . ready () , self . overrun () , self . underrun ())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
        #[must_use]
        #[inline(always)]
        pub const fn spiactive(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
        #[inline(always)]
        pub const fn set_spiactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of valid entries in the Receive FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn rxnum_5_0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of valid entries in the Receive FIFO."]
        #[inline(always)]
        pub const fn set_rxnum_5_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Receive FIFO Empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxempty(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Empty flag."]
        #[inline(always)]
        pub const fn set_rxempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive FIFO Full flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfull(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Full flag."]
        #[inline(always)]
        pub const fn set_rxfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Number of valid entries in the Transmit FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn txnum_5_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of valid entries in the Transmit FIFO."]
        #[inline(always)]
        pub const fn set_txnum_5_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Transmit FIFO Empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txempty(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO Empty flag."]
        #[inline(always)]
        pub const fn set_txempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Transmit FIFO Full flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txfull(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO Full flag."]
        #[inline(always)]
        pub const fn set_txfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Number of valid entries in the Receive FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn rxnum_7_6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Number of valid entries in the Receive FIFO."]
        #[inline(always)]
        pub const fn set_rxnum_7_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Number of valid entries in the Transmit FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn txnum_7_6(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Number of valid entries in the Transmit FIFO."]
        #[inline(always)]
        pub const fn set_txnum_7_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("spiactive", &self.spiactive())
                .field("rxnum_5_0", &self.rxnum_5_0())
                .field("rxempty", &self.rxempty())
                .field("rxfull", &self.rxfull())
                .field("txnum_5_0", &self.txnum_5_0())
                .field("txempty", &self.txempty())
                .field("txfull", &self.txfull())
                .field("rxnum_7_6", &self.rxnum_7_6())
                .field("txnum_7_6", &self.txnum_7_6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ spiactive: {=bool:?}, rxnum_5_0: {=u8:?}, rxempty: {=bool:?}, rxfull: {=bool:?}, txnum_5_0: {=u8:?}, txempty: {=bool:?}, txfull: {=bool:?}, rxnum_7_6: {=u8:?}, txnum_7_6: {=u8:?} }}" , self . spiactive () , self . rxnum_5_0 () , self . rxempty () , self . rxfull () , self . txnum_5_0 () , self . txempty () , self . txfull () , self . rxnum_7_6 () , self . txnum_7_6 ())
        }
    }
    #[doc = "Interface Timing Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timing(pub u32);
    impl Timing {
        #[doc = "The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn sclk_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
        #[inline(always)]
        pub const fn set_sclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2."]
        #[must_use]
        #[inline(always)]
        pub const fn csht(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2."]
        #[inline(always)]
        pub const fn set_csht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2."]
        #[must_use]
        #[inline(always)]
        pub const fn cs2sclk(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2."]
        #[inline(always)]
        pub const fn set_cs2sclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Timing {
        #[inline(always)]
        fn default() -> Timing {
            Timing(0)
        }
    }
    impl core::fmt::Debug for Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timing")
                .field("sclk_div", &self.sclk_div())
                .field("csht", &self.csht())
                .field("cs2sclk", &self.cs2sclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timing {{ sclk_div: {=u8:?}, csht: {=u8:?}, cs2sclk: {=u8:?} }}",
                self.sclk_div(),
                self.csht(),
                self.cs2sclk()
            )
        }
    }
    #[doc = "Transfer Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TransCtrl(pub u32);
    impl TransCtrl {
        #[doc = "Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
        #[must_use]
        #[inline(always)]
        pub const fn rdtrancnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
        #[inline(always)]
        pub const fn set_rdtrancnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
        #[must_use]
        #[inline(always)]
        pub const fn dummycnt(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
        #[inline(always)]
        pub const fn set_dummycnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69."]
        #[must_use]
        #[inline(always)]
        pub const fn tokenvalue(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69."]
        #[inline(always)]
        pub const fn set_tokenvalue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
        #[must_use]
        #[inline(always)]
        pub const fn wrtrancnt(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0x01ff;
            val as u16
        }
        #[doc = "Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
        #[inline(always)]
        pub const fn set_wrtrancnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
        }
        #[doc = "Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token."]
        #[must_use]
        #[inline(always)]
        pub const fn tokenen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Token transfer enable (Master mode only) Append a one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token."]
        #[inline(always)]
        pub const fn set_tokenen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn dualquad(&self) -> super::vals::DataPhaseFormat {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::DataPhaseFormat::from_bits(val as u8)
        }
        #[doc = "SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved."]
        #[inline(always)]
        pub const fn set_dualquad(&mut self, val: super::vals::DataPhaseFormat) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn transmode(&self) -> super::vals::TransMode {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::TransMode::from_bits(val as u8)
        }
        #[doc = "Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved."]
        #[inline(always)]
        pub const fn set_transmode(&mut self, val: super::vals::TransMode) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
        #[must_use]
        #[inline(always)]
        pub const fn addrfmt(&self) -> super::vals::AddrPhaseFormat {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::AddrPhaseFormat::from_bits(val as u8)
        }
        #[doc = "SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
        #[inline(always)]
        pub const fn set_addrfmt(&mut self, val: super::vals::AddrPhaseFormat) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase."]
        #[must_use]
        #[inline(always)]
        pub const fn addren(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase."]
        #[inline(always)]
        pub const fn set_addren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase."]
        #[must_use]
        #[inline(always)]
        pub const fn cmden(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase."]
        #[inline(always)]
        pub const fn set_cmden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn slvdataonly(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
        #[inline(always)]
        pub const fn set_slvdataonly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TransCtrl {
        #[inline(always)]
        fn default() -> TransCtrl {
            TransCtrl(0)
        }
    }
    impl core::fmt::Debug for TransCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TransCtrl")
                .field("rdtrancnt", &self.rdtrancnt())
                .field("dummycnt", &self.dummycnt())
                .field("tokenvalue", &self.tokenvalue())
                .field("wrtrancnt", &self.wrtrancnt())
                .field("tokenen", &self.tokenen())
                .field("dualquad", &self.dualquad())
                .field("transmode", &self.transmode())
                .field("addrfmt", &self.addrfmt())
                .field("addren", &self.addren())
                .field("cmden", &self.cmden())
                .field("slvdataonly", &self.slvdataonly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TransCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TransCtrl {{ rdtrancnt: {=u16:?}, dummycnt: {=u8:?}, tokenvalue: {=bool:?}, wrtrancnt: {=u16:?}, tokenen: {=bool:?}, dualquad: {:?}, transmode: {:?}, addrfmt: {:?}, addren: {=bool:?}, cmden: {=bool:?}, slvdataonly: {=bool:?} }}" , self . rdtrancnt () , self . dummycnt () , self . tokenvalue () , self . wrtrancnt () , self . tokenen () , self . dualquad () , self . transmode () , self . addrfmt () , self . addren () , self . cmden () , self . slvdataonly ())
        }
    }
    #[doc = "Transfer Format Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TransFmt(pub u32);
    impl TransFmt {
        #[doc = "SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges."]
        #[must_use]
        #[inline(always)]
        pub const fn cpha(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges."]
        #[inline(always)]
        pub const fn set_cpha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states."]
        #[must_use]
        #[inline(always)]
        pub const fn cpol(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states."]
        #[inline(always)]
        pub const fn set_cpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn slvmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode."]
        #[inline(always)]
        pub const fn set_slvmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first."]
        #[must_use]
        #[inline(always)]
        pub const fn lsb(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first."]
        #[inline(always)]
        pub const fn set_lsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two."]
        #[must_use]
        #[inline(always)]
        pub const fn mosibidir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two."]
        #[inline(always)]
        pub const fn set_mosibidir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
        #[must_use]
        #[inline(always)]
        pub const fn datamerge(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
        #[inline(always)]
        pub const fn set_datamerge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn datalen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)."]
        #[inline(always)]
        pub const fn set_datalen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlen(&self) -> super::vals::AddrLen {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::AddrLen::from_bits(val as u8)
        }
        #[doc = "Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes."]
        #[inline(always)]
        pub const fn set_addrlen(&mut self, val: super::vals::AddrLen) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
    }
    impl Default for TransFmt {
        #[inline(always)]
        fn default() -> TransFmt {
            TransFmt(0)
        }
    }
    impl core::fmt::Debug for TransFmt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TransFmt")
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("slvmode", &self.slvmode())
                .field("lsb", &self.lsb())
                .field("mosibidir", &self.mosibidir())
                .field("datamerge", &self.datamerge())
                .field("datalen", &self.datalen())
                .field("addrlen", &self.addrlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TransFmt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TransFmt {{ cpha: {=bool:?}, cpol: {=bool:?}, slvmode: {=bool:?}, lsb: {=bool:?}, mosibidir: {=bool:?}, datamerge: {=bool:?}, datalen: {=u8:?}, addrlen: {:?} }}" , self . cpha () , self . cpol () , self . slvmode () , self . lsb () , self . mosibidir () , self . datamerge () , self . datalen () , self . addrlen ())
        }
    }
}
pub mod vals {
    #[doc = "spi address length"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AddrLen {
        #[doc = "1 byte"]
        _8BIT = 0x0,
        #[doc = "2 bytes"]
        _16BIT = 0x01,
        #[doc = "3 bytes"]
        _24BIT = 0x02,
        #[doc = "4 bytes"]
        _32BIT = 0x03,
    }
    impl AddrLen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AddrLen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AddrLen {
        #[inline(always)]
        fn from(val: u8) -> AddrLen {
            AddrLen::from_bits(val)
        }
    }
    impl From<AddrLen> for u8 {
        #[inline(always)]
        fn from(val: AddrLen) -> u8 {
            AddrLen::to_bits(val)
        }
    }
    #[doc = "spi address phase format"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AddrPhaseFormat {
        #[doc = "Address phase is the regular (single) mode"]
        SINGLE_IO = 0x0,
        #[doc = "The format of the address phase is the same as the data phase (DualQuad)"]
        DUAL_QUAD_IO = 0x01,
    }
    impl AddrPhaseFormat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AddrPhaseFormat {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AddrPhaseFormat {
        #[inline(always)]
        fn from(val: u8) -> AddrPhaseFormat {
            AddrPhaseFormat::from_bits(val)
        }
    }
    impl From<AddrPhaseFormat> for u8 {
        #[inline(always)]
        fn from(val: AddrPhaseFormat) -> u8 {
            AddrPhaseFormat::to_bits(val)
        }
    }
    #[doc = "spi data phase format"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DataPhaseFormat {
        #[doc = "Regular (Single) mode"]
        SINGLE_IO = 0x0,
        #[doc = "Dual I/O mode"]
        DUAL_IO = 0x01,
        #[doc = "Quad I/O mode"]
        QUAD_IO = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl DataPhaseFormat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataPhaseFormat {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataPhaseFormat {
        #[inline(always)]
        fn from(val: u8) -> DataPhaseFormat {
            DataPhaseFormat::from_bits(val)
        }
    }
    impl From<DataPhaseFormat> for u8 {
        #[inline(always)]
        fn from(val: DataPhaseFormat) -> u8 {
            DataPhaseFormat::to_bits(val)
        }
    }
    #[doc = "spi transfer mode"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TransMode {
        #[doc = "Write and read at the same time"]
        WRITE_READ_TOGETHER = 0x0,
        #[doc = "Write only"]
        WRITE_ONLY = 0x01,
        #[doc = "Read only"]
        READ_ONLY = 0x02,
        #[doc = "Write, Read"]
        WRITE_READ = 0x03,
        #[doc = "Read, Write"]
        READ_WRITE = 0x04,
        #[doc = "Write, Dummy, Read"]
        WRITE_DUMMY_READ = 0x05,
        #[doc = "Read, Dummy, Write"]
        READ_DUMMY_WRITE = 0x06,
        #[doc = "None Data (must enable CmdEn or AddrEn in master mode)"]
        NO_DATA = 0x07,
        #[doc = "Dummy, Write"]
        DUMMY_WRITE = 0x08,
        #[doc = "Dummy, Read"]
        DUMMY_READ = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl TransMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TransMode {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TransMode {
        #[inline(always)]
        fn from(val: u8) -> TransMode {
            TransMode::from_bits(val)
        }
    }
    impl From<TransMode> for u8 {
        #[inline(always)]
        fn from(val: TransMode) -> u8 {
            TransMode::to_bits(val)
        }
    }
}
