#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LCB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcb {
    ptr: *mut u8,
}
unsafe impl Send for Lcb {}
unsafe impl Sync for Lcb {}
impl Lcb {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "LVDS RX PHY Status register."]
    #[inline(always)]
    pub const fn phy_stat(self) -> crate::common::Reg<regs::PhyStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_pow_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyPowCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_d_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyDCtrl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_ck_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyCkCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_adj_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhyAdjCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_su_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PhySuCtrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "mode selection： 00: lvds display(4 line), two LVDS RX PHY must be LVDS display mode 01: cam link(4 line), two LVDS RX PHY must be LVDS display mode 10: sync code(2 line), LVDS RX PHY must be LVDS cameral mode 11: sync code(1line), LVDS RX PHY must be LVDS cameral mode."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "mode selection： 00: lvds display(4 line), two LVDS RX PHY must be LVDS display mode 01: cam link(4 line), two LVDS RX PHY must be LVDS display mode 10: sync code(2 line), LVDS RX PHY must be LVDS cameral mode 11: sync code(1line), LVDS RX PHY must be LVDS cameral mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "just for LVDS Display mode, data width: 1: 24bit 0: 18bit(3line)."]
        #[inline(always)]
        pub const fn data_width(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "just for LVDS Display mode, data width: 1: 24bit 0: 18bit(3line)."]
        #[inline(always)]
        pub fn set_data_width(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "just for LVDS Display mode, data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[inline(always)]
        pub const fn bit_mapping(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "just for LVDS Display mode, data protocol: 1: JEIDA standard 0: SPWG standard."]
        #[inline(always)]
        pub fn set_bit_mapping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "just for CAM LINK mode, data width: 00: 24bit 01: 30bit 10: 36bit 11: reserved."]
        #[inline(always)]
        pub const fn cam_link_width(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "just for CAM LINK mode, data width: 00: 24bit 01: 30bit 10: 36bit 11: reserved."]
        #[inline(always)]
        pub fn set_cam_link_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "just for LVDS Display mode and CAM LINK mode, clock selection: 1: LVDS1 RXCK 0: LVDS0 RXCK."]
        #[inline(always)]
        pub const fn lvds_rxck_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "just for LVDS Display mode and CAM LINK mode, clock selection: 1: LVDS1 RXCK 0: LVDS0 RXCK."]
        #[inline(always)]
        pub fn set_lvds_rxck_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyAdjCtrl(pub u32);
    impl PhyAdjCtrl {
        #[doc = "LVDS RX PHY RXCK line: DLL loop delay coarse adjustment initial signal 00000000: min ; 11111111: max."]
        #[inline(always)]
        pub const fn lvds_dll_tuning_int(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "LVDS RX PHY RXCK line: DLL loop delay coarse adjustment initial signal 00000000: min ; 11111111: max."]
        #[inline(always)]
        pub fn set_lvds_dll_tuning_int(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "LVDS RX PHY RX1 line: bit \\[7:0\\]
: Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved."]
        #[inline(always)]
        pub const fn lvds_rx1_dline_adj(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "LVDS RX PHY RX1 line: bit \\[7:0\\]
: Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved."]
        #[inline(always)]
        pub fn set_lvds_rx1_dline_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "LVDS RX PHY RX0 line: bit \\[7:0\\]
: Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved."]
        #[inline(always)]
        pub const fn lvds_rx0_dline_adj(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "LVDS RX PHY RX0 line: bit \\[7:0\\]
: Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved."]
        #[inline(always)]
        pub fn set_lvds_rx0_dline_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for PhyAdjCtrl {
        #[inline(always)]
        fn default() -> PhyAdjCtrl {
            PhyAdjCtrl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCkCtrl(pub u32);
    impl PhyCkCtrl {
        #[doc = "bit 0 : DLL loop delay adjustment minimum control signal 0: used for RCKP/RCKN’s frequency is 40Mhz~70Mhz 1:used for RCKP/RCKN’s frequency is 70Mhz~110Mhz bit \\[2:1\\]
: DLL loop delay adjustment current regulation control signal. 00: min; 11: max bit 3 : Reserved bit 4 : Clock Lane Skew adjust enable in LVDS Camera Mode. bit \\[7:5\\]
: Bus width selection in LVDS Camera Mode 000: 4bit; 001:6bit; 010:7bit; 011:8bit; 100:9bit; 101:10bit; 110:11bit; 111:12bit. bit \\[10:8\\]
: DDR Clock duty cycle adjust in LVDS Camera Mode. bit \\[15:11\\]
: Reserved."]
        #[inline(always)]
        pub const fn rx_ctl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "bit 0 : DLL loop delay adjustment minimum control signal 0: used for RCKP/RCKN’s frequency is 40Mhz~70Mhz 1:used for RCKP/RCKN’s frequency is 70Mhz~110Mhz bit \\[2:1\\]
: DLL loop delay adjustment current regulation control signal. 00: min; 11: max bit 3 : Reserved bit 4 : Clock Lane Skew adjust enable in LVDS Camera Mode. bit \\[7:5\\]
: Bus width selection in LVDS Camera Mode 000: 4bit; 001:6bit; 010:7bit; 011:8bit; 100:9bit; 101:10bit; 110:11bit; 111:12bit. bit \\[10:8\\]
: DDR Clock duty cycle adjust in LVDS Camera Mode. bit \\[15:11\\]
: Reserved."]
        #[inline(always)]
        pub fn set_rx_ctl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm."]
        #[inline(always)]
        pub const fn rx_rterm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm."]
        #[inline(always)]
        pub fn set_rx_rterm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground."]
        #[inline(always)]
        pub const fn rx_vcom(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground."]
        #[inline(always)]
        pub fn set_rx_vcom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for PhyCkCtrl {
        #[inline(always)]
        fn default() -> PhyCkCtrl {
            PhyCkCtrl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyDCtrl(pub u32);
    impl PhyDCtrl {
        #[doc = "bit 0 : Lane N Data MSB first enable signal. 0: LSB ; 1: MSB bit 1 : Lane N Data Polarity signal. 0: Not inverting; 1: Inverting bit \\[4:2\\]
: Phase difference between the output first bit data (rxN\\[6:0\\]) and the input clock (RCKP/N) in LVDS Display Mode. bit 5 : Reserved bit 6 : Output data sampling clock control signal 0: Sampling using the rising edge of the clock pck. 1: Sampling using the falling edge of the clock pck. bit 7 : Reserved bit 8 : Data Lane N Skew adjust enable in LVDS Camera Mode. bit \\[12:9\\]
: Data Lane N Skew adjust; 0000: min; 0111: default; 1111: max. bit \\[15:13\\]
: Reserved."]
        #[inline(always)]
        pub const fn rx_ctl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "bit 0 : Lane N Data MSB first enable signal. 0: LSB ; 1: MSB bit 1 : Lane N Data Polarity signal. 0: Not inverting; 1: Inverting bit \\[4:2\\]
: Phase difference between the output first bit data (rxN\\[6:0\\]) and the input clock (RCKP/N) in LVDS Display Mode. bit 5 : Reserved bit 6 : Output data sampling clock control signal 0: Sampling using the rising edge of the clock pck. 1: Sampling using the falling edge of the clock pck. bit 7 : Reserved bit 8 : Data Lane N Skew adjust enable in LVDS Camera Mode. bit \\[12:9\\]
: Data Lane N Skew adjust; 0000: min; 0111: default; 1111: max. bit \\[15:13\\]
: Reserved."]
        #[inline(always)]
        pub fn set_rx_ctl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm."]
        #[inline(always)]
        pub const fn rx_rterm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm."]
        #[inline(always)]
        pub fn set_rx_rterm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground."]
        #[inline(always)]
        pub const fn rx_vcom(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground."]
        #[inline(always)]
        pub fn set_rx_vcom(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for PhyDCtrl {
        #[inline(always)]
        fn default() -> PhyDCtrl {
            PhyDCtrl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyPowCtrl(pub u32);
    impl PhyPowCtrl {
        #[doc = "Power down control signal of channel rx0 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn rx0_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel rx0 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub fn set_rx0_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power down control signal of channel rx1 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn rx1_pd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel rx1 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub fn set_rx1_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Power down control signal of channel rxck 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn rxck_pd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel rxck 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub fn set_rxck_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Power down control signal of channel rxck/rx1/rx0 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub const fn iddq_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Power down control signal of channel rxck/rx1/rx0 0: Normal operation 1: Power down channel."]
        #[inline(always)]
        pub fn set_iddq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PhyPowCtrl {
        #[inline(always)]
        fn default() -> PhyPowCtrl {
            PhyPowCtrl(0)
        }
    }
    #[doc = "LVDS RX PHY Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStat(pub u32);
    impl PhyStat {
        #[doc = "LVDS0 RX PHY DLL Lock indication Signal, 1 means dll already locked."]
        #[inline(always)]
        pub const fn lvds0_rx_phy_dll_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LVDS0 RX PHY DLL Lock indication Signal, 1 means dll already locked."]
        #[inline(always)]
        pub fn set_lvds0_rx_phy_dll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LVDS1 RX PHY DLL Lock indication Signal, 1 means dll already locked."]
        #[inline(always)]
        pub const fn lvds1_rx_phy_dll_lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LVDS1 RX PHY DLL Lock indication Signal, 1 means dll already locked."]
        #[inline(always)]
        pub fn set_lvds1_rx_phy_dll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PhyStat {
        #[inline(always)]
        fn default() -> PhyStat {
            PhyStat(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhySuCtrl(pub u32);
    impl PhySuCtrl {
        #[doc = "bit \\[2:0\\]
: Reference voltage/current adjustment control signal. 000: min; 111: max bit \\[3\\]
: Internal bias circuit selection signal. 0: from Bandgap Mode; 1: from self-bias mode bit \\[7:4\\]
: Reserved."]
        #[inline(always)]
        pub const fn su_ctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "bit \\[2:0\\]
: Reference voltage/current adjustment control signal. 000: min; 111: max bit \\[3\\]
: Internal bias circuit selection signal. 0: from Bandgap Mode; 1: from self-bias mode bit \\[7:4\\]
: Reserved."]
        #[inline(always)]
        pub fn set_su_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for PhySuCtrl {
        #[inline(always)]
        fn default() -> PhySuCtrl {
            PhySuCtrl(0)
        }
    }
}
