#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LVB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvb {
    ptr: *mut u8,
}
unsafe impl Send for Lvb {}
unsafe impl Sync for Lvb {}
impl Lvb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LVDS TX PHY Status register."]
    #[inline(always)]
    pub const fn phy_stat(self) -> crate::common::Reg<regs::PhyStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_pow_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyPowCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tx_phy(self, n: usize) -> TxPhy {
        assert!(n < 10usize);
        unsafe { TxPhy::from_ptr(self.ptr.wrapping_add(0x1cusize + n * 8usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPhy {
    ptr: *mut u8,
}
unsafe impl Send for TxPhy {}
unsafe impl Sync for TxPhy {}
impl TxPhy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TX PHY Setting."]
    #[inline(always)]
    pub const fn ctl0(self) -> crate::common::Reg<regs::Ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TX_PHY Setting."]
    #[inline(always)]
    pub const fn ctl1(self) -> crate::common::Reg<regs::Ctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
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
    #[doc = "TX PHY Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl0(pub u32);
    impl Ctl0 {
        #[doc = "output de-emphasis level trimming(Unit: dB) 00: 0 01: 2.5 10: 6.0 11: 6.0."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_deemp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "output de-emphasis level trimming(Unit: dB) 00: 0 01: 2.5 10: 6.0 11: 6.0."]
        #[inline(always)]
        pub const fn set_tx_deemp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "output slew-rate trimming 00: slowest slew-rate; 11: fastest slew-rate."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_sr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "output slew-rate trimming 00: slowest slew-rate; 11: fastest slew-rate."]
        #[inline(always)]
        pub const fn set_tx_sr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Output voltage Adjustment(Unit: mV). 0000 : 50 0001: 100 0010: 150 0011: 200 0100: 250 0101: 300 0110: 350 0111: 400 1000: 450 1001: 500 1010: 550 1011~1111: 600."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_amp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Output voltage Adjustment(Unit: mV). 0000 : 50 0001: 100 0010: 150 0011: 200 0100: 250 0101: 300 0110: 350 0111: 400 1000: 450 1001: 500 1010: 550 1011~1111: 600."]
        #[inline(always)]
        pub const fn set_tx_amp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "output Common Mode Voltage adjustment(Unit: V). 0000: 0.7 0001: 0.8 0010: 0.9 0011: 1.0 0100: 1.1 0101: 1.2 0110: 1.3 0111: 1.4 1000~1111: 1.5."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_vcom(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "output Common Mode Voltage adjustment(Unit: V). 0000: 0.7 0001: 0.8 0010: 0.9 0011: 1.0 0100: 1.1 0101: 1.2 0110: 1.3 0111: 1.4 1000~1111: 1.5."]
        #[inline(always)]
        pub const fn set_tx_vcom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "data/clock lane output phase adjustment: 0000: 0 0001: data lane is 1/32, clock lane is 1/16 0010: data lane is 2/32, clock lane is 2/16 0011: data lane is 3/32, clock lane is 3/16 0100: data lane is 4/32, clock lane is 4/16 0101: data lane is 5/32, clock lane is 5/16 0110: data lane is 6/32, clock lane is 6/16 0111: data lane is 7/32, clock lane is 7/16 1000: data lane is 8/32, clock lane is 8/16 1001: data lane is 9/32, clock lane is 9/16 1010: data lane is 10/32, clock lane is 10/16 1011: data lane is 11/32, clock lane is 11/16 1100: data lane is 12/32, clock lane is 12/16 1101: data lane is 13/32, clock lane is 13/16 1110: data lane is 14/32, clock lane is 14/16 1111: data lane is 15/32, clock lane is 15/16."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_phase_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "data/clock lane output phase adjustment: 0000: 0 0001: data lane is 1/32, clock lane is 1/16 0010: data lane is 2/32, clock lane is 2/16 0011: data lane is 3/32, clock lane is 3/16 0100: data lane is 4/32, clock lane is 4/16 0101: data lane is 5/32, clock lane is 5/16 0110: data lane is 6/32, clock lane is 6/16 0111: data lane is 7/32, clock lane is 7/16 1000: data lane is 8/32, clock lane is 8/16 1001: data lane is 9/32, clock lane is 9/16 1010: data lane is 10/32, clock lane is 10/16 1011: data lane is 11/32, clock lane is 11/16 1100: data lane is 12/32, clock lane is 12/16 1101: data lane is 13/32, clock lane is 13/16 1110: data lane is 14/32, clock lane is 14/16 1111: data lane is 15/32, clock lane is 15/16."]
        #[inline(always)]
        pub const fn set_tx_phase_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Parallel data bus width select： 000: 4-bit mode, txN_data\\[3:0\\]
are valid, txN_data\\[11:4\\]
can be arbitrary state. 001: 6-bit mode, txN_data\\[5:0\\]
are valid, txN_data\\[11:6\\]
can be arbitrary state. 010: 7-bit mode. txN_data\\[6:0\\]
are valid, txN_data\\[11:7\\]
can be arbitrary state. 011: 8-bit mode. txN_data\\[7:0\\]
are valid, txN_data\\[11:8\\]
can be arbitrary state. 100: 9-bit mode. txN_data\\[8:0\\]
are valid, txN_data\\[11:9\\]
can be arbitrary state. 101: 10-bit mode. txN_data\\[9:0\\]
are valid, txN_data\\[11:10\\]
can be arbitrary state. 110: 11-bit mode. txN_data\\[10:0\\]
are valid, txN_data\\[11\\]
can be arbitrary state. 111: 12-bit mode. txN_data\\[11:0\\]
are valid."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_bus_width(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Parallel data bus width select： 000: 4-bit mode, txN_data\\[3:0\\]
are valid, txN_data\\[11:4\\]
can be arbitrary state. 001: 6-bit mode, txN_data\\[5:0\\]
are valid, txN_data\\[11:6\\]
can be arbitrary state. 010: 7-bit mode. txN_data\\[6:0\\]
are valid, txN_data\\[11:7\\]
can be arbitrary state. 011: 8-bit mode. txN_data\\[7:0\\]
are valid, txN_data\\[11:8\\]
can be arbitrary state. 100: 9-bit mode. txN_data\\[8:0\\]
are valid, txN_data\\[11:9\\]
can be arbitrary state. 101: 10-bit mode. txN_data\\[9:0\\]
are valid, txN_data\\[11:10\\]
can be arbitrary state. 110: 11-bit mode. txN_data\\[10:0\\]
are valid, txN_data\\[11\\]
can be arbitrary state. 111: 12-bit mode. txN_data\\[11:0\\]
are valid."]
        #[inline(always)]
        pub const fn set_tx_bus_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Inner Terminal Resistance enable 0: Disable rterm 2000ohm 1: Enable rterm 100ohm."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_rterm_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Inner Terminal Resistance enable 0: Disable rterm 2000ohm 1: Enable rterm 100ohm."]
        #[inline(always)]
        pub const fn set_tx_rterm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Force the high-speed differential signal to common mode. This signal can be set during IP power up stage to prevent unexpected leakage current in TXP/TXN 0: Normal operation 1: Force TXPN /TXMN to common mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_idle(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Force the high-speed differential signal to common mode. This signal can be set during IP power up stage to prevent unexpected leakage current in TXP/TXN 0: Normal operation 1: Force TXPN /TXMN to common mode."]
        #[inline(always)]
        pub const fn set_tx_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ctl0 {
        #[inline(always)]
        fn default() -> Ctl0 {
            Ctl0(0)
        }
    }
    impl core::fmt::Debug for Ctl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl0")
                .field("tx_deemp", &self.tx_deemp())
                .field("tx_sr", &self.tx_sr())
                .field("tx_amp", &self.tx_amp())
                .field("tx_vcom", &self.tx_vcom())
                .field("tx_phase_sel", &self.tx_phase_sel())
                .field("tx_bus_width", &self.tx_bus_width())
                .field("tx_rterm_en", &self.tx_rterm_en())
                .field("tx_idle", &self.tx_idle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctl0 {{ tx_deemp: {=u8:?}, tx_sr: {=u8:?}, tx_amp: {=u8:?}, tx_vcom: {=u8:?}, tx_phase_sel: {=u8:?}, tx_bus_width: {=u8:?}, tx_rterm_en: {=bool:?}, tx_idle: {=bool:?} }}" , self . tx_deemp () , self . tx_sr () , self . tx_amp () , self . tx_vcom () , self . tx_phase_sel () , self . tx_bus_width () , self . tx_rterm_en () , self . tx_idle ())
        }
    }
    #[doc = "TX_PHY Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl1(pub u32);
    impl Ctl1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ctl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tx_ctl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ctl1 {
        #[inline(always)]
        fn default() -> Ctl1 {
            Ctl1(0)
        }
    }
    impl core::fmt::Debug for Ctl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctl1")
                .field("tx_ctl", &self.tx_ctl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctl1 {{ tx_ctl: {=u32:?} }}", self.tx_ctl())
        }
    }
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Channel 0 enable: 1: enable 0: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 enable: 1: enable 0: disable."]
        #[inline(always)]
        pub const fn set_ch0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 0 select: 1: select DI 1 0: select DI 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 select: 1: select DI 1 0: select DI 0."]
        #[inline(always)]
        pub const fn set_ch0_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel 1 enable: 1: enable 0: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 enable: 1: enable 0: disable."]
        #[inline(always)]
        pub const fn set_ch1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel 1 select: 1: select DI 1 0: select DI 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 select: 1: select DI 1 0: select DI 0."]
        #[inline(always)]
        pub const fn set_ch1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel 0 data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0_bit_mapping(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[inline(always)]
        pub const fn set_ch0_bit_mapping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 1 data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1_bit_mapping(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[inline(always)]
        pub const fn set_ch1_bit_mapping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Shift the LVDS TX PHY clock in relation to the data. 000: txck is 7'b1100011 001: txck is 7‘b1110001 010: txck is 7‘b1111000 011: txck is 7‘b1000111 100: txck is 7‘b0001111 101: txck is 7‘b0011110 110: txck is 7‘b0111100 111: txck is 7‘b1100011."]
        #[must_use]
        #[inline(always)]
        pub const fn lvds_txclk_shift(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Shift the LVDS TX PHY clock in relation to the data. 000: txck is 7'b1100011 001: txck is 7‘b1110001 010: txck is 7‘b1111000 011: txck is 7‘b1000111 100: txck is 7‘b0001111 101: txck is 7‘b0011110 110: txck is 7‘b0111100 111: txck is 7‘b1100011."]
        #[inline(always)]
        pub const fn set_lvds_txclk_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "DI 0 vsync polarity: 1: active low 0: active high."]
        #[must_use]
        #[inline(always)]
        pub const fn di0_vsync_polarity(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DI 0 vsync polarity: 1: active low 0: active high."]
        #[inline(always)]
        pub const fn set_di0_vsync_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DI 1 vsync polarity: 1: active low 0: active high."]
        #[must_use]
        #[inline(always)]
        pub const fn di1_vsync_polarity(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DI 1 vsync polarity: 1: active low 0: active high."]
        #[inline(always)]
        pub const fn set_di1_vsync_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Split mode enable: 1: enable 0: disable Note: when using split mode, ch0/1 should be enabled, and should select same DI."]
        #[must_use]
        #[inline(always)]
        pub const fn split_mode_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Split mode enable: 1: enable 0: disable Note: when using split mode, ch0/1 should be enabled, and should select same DI."]
        #[inline(always)]
        pub const fn set_split_mode_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Just for split mode, the sum of HSW and HBP width is even 1: yes 0: no."]
        #[must_use]
        #[inline(always)]
        pub const fn split_hswhbp_width(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Just for split mode, the sum of HSW and HBP width is even 1: yes 0: no."]
        #[inline(always)]
        pub const fn set_split_hswhbp_width(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Just for split mode 1: two channel pixel data are not aligned 0: two channel pixel data are aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn split_ch_mode(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Just for split mode 1: two channel pixel data are not aligned 0: two channel pixel data are aligned."]
        #[inline(always)]
        pub const fn set_split_ch_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Just for split mode, reverse two channel data."]
        #[must_use]
        #[inline(always)]
        pub const fn split_ch_reverse(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Just for split mode, reverse two channel data."]
        #[inline(always)]
        pub const fn set_split_ch_reverse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("ch0_en", &self.ch0_en())
                .field("ch0_sel", &self.ch0_sel())
                .field("ch1_en", &self.ch1_en())
                .field("ch1_sel", &self.ch1_sel())
                .field("ch0_bit_mapping", &self.ch0_bit_mapping())
                .field("ch1_bit_mapping", &self.ch1_bit_mapping())
                .field("lvds_txclk_shift", &self.lvds_txclk_shift())
                .field("di0_vsync_polarity", &self.di0_vsync_polarity())
                .field("di1_vsync_polarity", &self.di1_vsync_polarity())
                .field("split_mode_en", &self.split_mode_en())
                .field("split_hswhbp_width", &self.split_hswhbp_width())
                .field("split_ch_mode", &self.split_ch_mode())
                .field("split_ch_reverse", &self.split_ch_reverse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ ch0_en: {=bool:?}, ch0_sel: {=bool:?}, ch1_en: {=bool:?}, ch1_sel: {=bool:?}, ch0_bit_mapping: {=bool:?}, ch1_bit_mapping: {=bool:?}, lvds_txclk_shift: {=u8:?}, di0_vsync_polarity: {=bool:?}, di1_vsync_polarity: {=bool:?}, split_mode_en: {=bool:?}, split_hswhbp_width: {=bool:?}, split_ch_mode: {=bool:?}, split_ch_reverse: {=bool:?} }}" , self . ch0_en () , self . ch0_sel () , self . ch1_en () , self . ch1_sel () , self . ch0_bit_mapping () , self . ch1_bit_mapping () , self . lvds_txclk_shift () , self . di0_vsync_polarity () , self . di1_vsync_polarity () , self . split_mode_en () , self . split_hswhbp_width () , self . split_ch_mode () , self . split_ch_reverse ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyPowCtrl(pub u32);
    impl PhyPowCtrl {
        #[doc = "Power down control signal of channel tx0 0: Normal operation 1: Power down channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tx0_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel tx0 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn set_tx0_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power down control signal of channel tx1 0: Normal operation 1: Power down channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tx1_pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel tx1 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn set_tx1_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Power down control signal of channel tx2 0: Normal operation 1: Power down channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tx2_pd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel tx2 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn set_tx2_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Power down control signal of channel tx3 0: Normal operation 1: Power down channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tx3_pd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel tx3 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn set_tx3_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Power down control signal of channel txck 0: Normal operation 1: Power down channel."]
        #[must_use]
        #[inline(always)]
        pub const fn txck_pd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel txck 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn set_txck_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "pll power on."]
        #[must_use]
        #[inline(always)]
        pub const fn pwon_pll(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "pll power on."]
        #[inline(always)]
        pub const fn set_pwon_pll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for PhyPowCtrl {
        #[inline(always)]
        fn default() -> PhyPowCtrl {
            PhyPowCtrl(0)
        }
    }
    impl core::fmt::Debug for PhyPowCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyPowCtrl")
                .field("tx0_pd", &self.tx0_pd())
                .field("tx1_pd", &self.tx1_pd())
                .field("tx2_pd", &self.tx2_pd())
                .field("tx3_pd", &self.tx3_pd())
                .field("txck_pd", &self.txck_pd())
                .field("pwon_pll", &self.pwon_pll())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyPowCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyPowCtrl {{ tx0_pd: {=bool:?}, tx1_pd: {=bool:?}, tx2_pd: {=bool:?}, tx3_pd: {=bool:?}, txck_pd: {=bool:?}, pwon_pll: {=bool:?} }}" , self . tx0_pd () , self . tx1_pd () , self . tx2_pd () , self . tx3_pd () , self . txck_pd () , self . pwon_pll ())
        }
    }
    #[doc = "LVDS TX PHY Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStat(pub u32);
    impl PhyStat {
        #[doc = "LVDS0 TX PHY PLL Lock indication Signal, 1 means pll already locked."]
        #[must_use]
        #[inline(always)]
        pub const fn lvds0_tx_phy_pll_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LVDS0 TX PHY PLL Lock indication Signal, 1 means pll already locked."]
        #[inline(always)]
        pub const fn set_lvds0_tx_phy_pll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LVDS1 TX PHY PLL Lock indication Signal, 1 means pll already locked."]
        #[must_use]
        #[inline(always)]
        pub const fn lvds1_tx_phy_pll_lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LVDS1 TX PHY PLL Lock indication Signal, 1 means pll already locked."]
        #[inline(always)]
        pub const fn set_lvds1_tx_phy_pll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PhyStat {
        #[inline(always)]
        fn default() -> PhyStat {
            PhyStat(0)
        }
    }
    impl core::fmt::Debug for PhyStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyStat")
                .field("lvds0_tx_phy_pll_lock", &self.lvds0_tx_phy_pll_lock())
                .field("lvds1_tx_phy_pll_lock", &self.lvds1_tx_phy_pll_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PhyStat {{ lvds0_tx_phy_pll_lock: {=bool:?}, lvds1_tx_phy_pll_lock: {=bool:?} }}",
                self.lvds0_tx_phy_pll_lock(),
                self.lvds1_tx_phy_pll_lock()
            )
        }
    }
}
