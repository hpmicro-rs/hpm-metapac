#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CAN0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
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
    pub const fn rbuf(self, n: usize) -> crate::common::Reg<regs::Rbuf, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tbuf(self, n: usize) -> crate::common::Reg<regs::Tbuf, crate::common::RW> {
        assert!(n < 18usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tts(self, n: usize) -> crate::common::Reg<regs::Tts, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize + n * 4usize) as _) }
    }
    #[doc = "config, status, command and control bits."]
    #[inline(always)]
    pub const fn cmd_sta_cmd_ctrl(
        self,
    ) -> crate::common::Reg<regs::CmdStaCmdCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Receive and Transmit Interrupt Enable Register RTIE."]
    #[inline(always)]
    pub const fn rtie(self) -> crate::common::Reg<regs::Rtie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)."]
    #[inline(always)]
    pub const fn rtif(self) -> crate::common::Reg<regs::Rtif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa5usize) as _) }
    }
    #[doc = "ERRor INTerrupt Enable and Flag Register ERRINT."]
    #[inline(always)]
    pub const fn errint(self) -> crate::common::Reg<regs::Errint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa6usize) as _) }
    }
    #[doc = "Warning Limits Register LIMIT."]
    #[inline(always)]
    pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa7usize) as _) }
    }
    #[doc = "Bit Timing Register(Slow Speed)."]
    #[inline(always)]
    pub const fn s_presc(self) -> crate::common::Reg<regs::SPresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "Bit Timing Register(Fast Speed)."]
    #[inline(always)]
    pub const fn f_presc(self) -> crate::common::Reg<regs::FPresc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Error and Arbitration Lost Capture Register EALCAP."]
    #[inline(always)]
    pub const fn ealcap(self) -> crate::common::Reg<regs::Ealcap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "Transmitter Delay Compensation Register TDC."]
    #[inline(always)]
    pub const fn tdc(self) -> crate::common::Reg<regs::Tdc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb1usize) as _) }
    }
    #[doc = "Error Counter Registers RECNT."]
    #[inline(always)]
    pub const fn recnt(self) -> crate::common::Reg<regs::Recnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb2usize) as _) }
    }
    #[doc = "Error Counter Registers TECNT."]
    #[inline(always)]
    pub const fn tecnt(self) -> crate::common::Reg<regs::Tecnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb3usize) as _) }
    }
    #[doc = "Acceptance Filter Control Register ACFCTRL."]
    #[inline(always)]
    pub const fn acfctrl(self) -> crate::common::Reg<regs::Acfctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "CiA 603 Time-Stamping TIMECFG."]
    #[inline(always)]
    pub const fn timecfg(self) -> crate::common::Reg<regs::Timecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb5usize) as _) }
    }
    #[doc = "Acceptance Filter Enable ACF_EN."]
    #[inline(always)]
    pub const fn acf_en(self) -> crate::common::Reg<regs::AcfEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb6usize) as _) }
    }
    #[doc = "Acceptance CODE ACODE or ACMASK."]
    #[inline(always)]
    pub const fn acf(self) -> crate::common::Reg<regs::Acf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Version Information VER."]
    #[inline(always)]
    pub const fn ver(self) -> crate::common::Reg<regs::Ver, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "TTCAN: TB Slot Pointer TBSLOT."]
    #[inline(always)]
    pub const fn tbslot(self) -> crate::common::Reg<regs::Tbslot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbeusize) as _) }
    }
    #[doc = "TTCAN: Time Trigger Configuration TTCFG."]
    #[inline(always)]
    pub const fn ttcfg(self) -> crate::common::Reg<regs::Ttcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbfusize) as _) }
    }
    #[doc = "TTCAN: Reference Message REF_MSG."]
    #[inline(always)]
    pub const fn ref_msg(self) -> crate::common::Reg<regs::RefMsg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "TTCAN: Trigger Configuration TRIG_CFG."]
    #[inline(always)]
    pub const fn trig_cfg(self) -> crate::common::Reg<regs::TrigCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "TTCAN: Trigger Time TT_TRIG."]
    #[inline(always)]
    pub const fn tt_trig(self) -> crate::common::Reg<regs::TtTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc6usize) as _) }
    }
    #[doc = "TTCAN: Watch Trigger Time TT_WTRIG."]
    #[inline(always)]
    pub const fn tt_wtrig(self) -> crate::common::Reg<regs::TtWtrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Acceptance CODE ACODE or ACMASK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acf(pub u32);
    impl Acf {
        #[doc = "Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
        #[inline(always)]
        pub const fn code_mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
        #[inline(always)]
        pub fn set_code_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
        #[inline(always)]
        pub const fn aide(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
        #[inline(always)]
        pub fn set_aide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
        #[inline(always)]
        pub const fn aidee(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
        #[inline(always)]
        pub fn set_aidee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Acf {
        #[inline(always)]
        fn default() -> Acf {
            Acf(0)
        }
    }
    #[doc = "Acceptance Filter Enable ACF_EN."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AcfEn(pub u16);
    impl AcfEn {
        #[doc = "Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
        #[inline(always)]
        pub const fn acf_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
        #[inline(always)]
        pub fn set_acf_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for AcfEn {
        #[inline(always)]
        fn default() -> AcfEn {
            AcfEn(0)
        }
    }
    #[doc = "Acceptance Filter Control Register ACFCTRL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acfctrl(pub u8);
    impl Acfctrl {
        #[doc = "acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
        #[inline(always)]
        pub const fn acfadr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
        #[inline(always)]
        pub fn set_acfadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
        #[inline(always)]
        pub const fn selmask(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
        #[inline(always)]
        pub fn set_selmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Acfctrl {
        #[inline(always)]
        fn default() -> Acfctrl {
            Acfctrl(0)
        }
    }
    #[doc = "config, status, command and control bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdStaCmdCtrl(pub u32);
    impl CmdStaCmdCtrl {
        #[doc = "Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
        #[inline(always)]
        pub const fn busoff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
        #[inline(always)]
        pub fn set_busoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity."]
        #[inline(always)]
        pub const fn tactive(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity."]
        #[inline(always)]
        pub fn set_tactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity."]
        #[inline(always)]
        pub const fn ractive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity."]
        #[inline(always)]
        pub fn set_ractive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled."]
        #[inline(always)]
        pub const fn tsss(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled."]
        #[inline(always)]
        pub fn set_tsss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled."]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled."]
        #[inline(always)]
        pub fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
        #[inline(always)]
        pub const fn lbmi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
        #[inline(always)]
        pub fn set_lbmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active."]
        #[inline(always)]
        pub const fn lbme(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active."]
        #[inline(always)]
        pub fn set_lbme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
        #[inline(always)]
        pub const fn tsa(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
        #[inline(always)]
        pub fn set_tsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
        #[inline(always)]
        pub const fn tsall(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
        #[inline(always)]
        pub fn set_tsall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub const fn tsone(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub fn set_tsone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
        #[inline(always)]
        pub const fn tpa(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
        #[inline(always)]
        pub fn set_tpa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub const fn tpe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub fn set_tpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
        #[inline(always)]
        pub const fn stby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
        #[inline(always)]
        pub fn set_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
        #[inline(always)]
        pub const fn lom(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
        #[inline(always)]
        pub fn set_lom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub const fn tbsel(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)."]
        #[inline(always)]
        pub fn set_tbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full."]
        #[inline(always)]
        pub const fn tsstat(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full."]
        #[inline(always)]
        pub fn set_tsstat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
        #[inline(always)]
        pub const fn tttbm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
        #[inline(always)]
        pub fn set_tttbm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty."]
        #[inline(always)]
        pub const fn tsmode(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty."]
        #[inline(always)]
        pub fn set_tsmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
        #[inline(always)]
        pub const fn tsnext(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
        #[inline(always)]
        pub fn set_tsnext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
        #[inline(always)]
        pub const fn fd_iso(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
        #[inline(always)]
        pub fn set_fd_iso(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV)."]
        #[inline(always)]
        pub const fn rstat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV)."]
        #[inline(always)]
        pub fn set_rstat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error."]
        #[inline(always)]
        pub const fn rball(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error."]
        #[inline(always)]
        pub fn set_rball(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release."]
        #[inline(always)]
        pub const fn rrel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release."]
        #[inline(always)]
        pub fn set_rrel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1."]
        #[inline(always)]
        pub const fn rov(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1."]
        #[inline(always)]
        pub fn set_rov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
        #[inline(always)]
        pub const fn rom(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
        #[inline(always)]
        pub fn set_rom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1."]
        #[inline(always)]
        pub const fn sack(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1."]
        #[inline(always)]
        pub fn set_sack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CmdStaCmdCtrl {
        #[inline(always)]
        fn default() -> CmdStaCmdCtrl {
            CmdStaCmdCtrl(0)
        }
    }
    #[doc = "Error and Arbitration Lost Capture Register EALCAP."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ealcap(pub u8);
    impl Ealcap {
        #[doc = "Arbitration Lost Capture (bit position in the frame where the arbitration has been lost)."]
        #[inline(always)]
        pub const fn alc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Arbitration Lost Capture (bit position in the frame where the arbitration has been lost)."]
        #[inline(always)]
        pub fn set_alc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received."]
        #[inline(always)]
        pub const fn koer(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received."]
        #[inline(always)]
        pub fn set_koer(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
        }
    }
    impl Default for Ealcap {
        #[inline(always)]
        fn default() -> Ealcap {
            Ealcap(0)
        }
    }
    #[doc = "ERRor INTerrupt Enable and Flag Register ERRINT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errint(pub u8);
    impl Errint {
        #[doc = "Bus Error Interrupt Flag."]
        #[inline(always)]
        pub const fn beif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error Interrupt Flag."]
        #[inline(always)]
        pub fn set_beif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Bus Error Interrupt Enable."]
        #[inline(always)]
        pub const fn beie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error Interrupt Enable."]
        #[inline(always)]
        pub fn set_beie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Arbitration Lost Interrupt Flag."]
        #[inline(always)]
        pub const fn alif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration Lost Interrupt Flag."]
        #[inline(always)]
        pub fn set_alif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Arbitration Lost Interrupt Enable."]
        #[inline(always)]
        pub const fn alie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Arbitration Lost Interrupt Enable."]
        #[inline(always)]
        pub fn set_alie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
        #[inline(always)]
        pub const fn epif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
        #[inline(always)]
        pub fn set_epif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Error Passive Interrupt Enable."]
        #[inline(always)]
        pub const fn epie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive Interrupt Enable."]
        #[inline(always)]
        pub fn set_epie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive)."]
        #[inline(always)]
        pub const fn epass(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive)."]
        #[inline(always)]
        pub fn set_epass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL."]
        #[inline(always)]
        pub const fn ewarn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL."]
        #[inline(always)]
        pub fn set_ewarn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Errint {
        #[inline(always)]
        fn default() -> Errint {
            Errint(0)
        }
    }
    #[doc = "Bit Timing Register(Fast Speed)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FPresc(pub u32);
    impl FPresc {
        #[doc = "Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
        #[inline(always)]
        pub const fn f_seg_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
        #[inline(always)]
        pub fn set_f_seg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Bit Timing Segment 2 (fast speed) Time after the sample point."]
        #[inline(always)]
        pub const fn f_seg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Bit Timing Segment 2 (fast speed) Time after the sample point."]
        #[inline(always)]
        pub fn set_f_seg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
        #[inline(always)]
        pub const fn f_sjw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
        #[inline(always)]
        pub fn set_f_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
        #[inline(always)]
        pub const fn f_presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
        #[inline(always)]
        pub fn set_f_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for FPresc {
        #[inline(always)]
        fn default() -> FPresc {
            FPresc(0)
        }
    }
    #[doc = "Warning Limits Register LIMIT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Limit(pub u8);
    impl Limit {
        #[doc = "Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
        #[inline(always)]
        pub const fn ewl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
        #[inline(always)]
        pub fn set_ewl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
        #[inline(always)]
        pub const fn afwl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
        #[inline(always)]
        pub fn set_afwl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Limit {
        #[inline(always)]
        fn default() -> Limit {
            Limit(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbuf(pub u32);
    impl Rbuf {
        #[doc = "receive buffer."]
        #[inline(always)]
        pub const fn rbuf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "receive buffer."]
        #[inline(always)]
        pub fn set_rbuf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rbuf {
        #[inline(always)]
        fn default() -> Rbuf {
            Rbuf(0)
        }
    }
    #[doc = "Error Counter Registers RECNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Recnt(pub u8);
    impl Recnt {
        #[doc = "Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen."]
        #[inline(always)]
        pub const fn recnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen."]
        #[inline(always)]
        pub fn set_recnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Recnt {
        #[inline(always)]
        fn default() -> Recnt {
            Recnt(0)
        }
    }
    #[doc = "TTCAN: Reference Message REF_MSG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefMsg(pub u32);
    impl RefMsg {
        #[doc = "REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
        #[inline(always)]
        pub const fn ref_msg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
        #[inline(always)]
        pub fn set_ref_msg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
        #[doc = "REFerence message IDE bit."]
        #[inline(always)]
        pub const fn ref_ide(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "REFerence message IDE bit."]
        #[inline(always)]
        pub fn set_ref_ide(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for RefMsg {
        #[inline(always)]
        fn default() -> RefMsg {
            RefMsg(0)
        }
    }
    #[doc = "Receive and Transmit Interrupt Enable Register RTIE."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtie(pub u8);
    impl Rtie {
        #[doc = "If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty."]
        #[inline(always)]
        pub const fn tsff(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty."]
        #[inline(always)]
        pub fn set_tsff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Error Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Error Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn tpie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_tpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn rafie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_rafie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RB Full Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn rfie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RB Full Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_rfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Receive Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt Enable 0 – Disabled, 1 – Enabled."]
        #[inline(always)]
        pub fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rtie {
        #[inline(always)]
        fn default() -> Rtie {
            Rtie(0)
        }
    }
    #[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtif(pub u8);
    impl Rtif {
        #[doc = "Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
        #[inline(always)]
        pub const fn aif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
        #[inline(always)]
        pub fn set_aif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
        #[inline(always)]
        pub const fn eif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
        #[inline(always)]
        pub fn set_eif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
        #[inline(always)]
        pub const fn tsif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
        #[inline(always)]
        pub fn set_tsif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
        #[inline(always)]
        pub const fn tpif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
        #[inline(always)]
        pub fn set_tpif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i."]
        #[inline(always)]
        pub const fn rafif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i."]
        #[inline(always)]
        pub fn set_rafif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
        #[inline(always)]
        pub const fn rfif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
        #[inline(always)]
        pub fn set_rfif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
        #[inline(always)]
        pub const fn roif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
        #[inline(always)]
        pub fn set_roif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
        #[inline(always)]
        pub const fn rif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
        #[inline(always)]
        pub fn set_rif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Rtif {
        #[inline(always)]
        fn default() -> Rtif {
            Rtif(0)
        }
    }
    #[doc = "Bit Timing Register(Slow Speed)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SPresc(pub u32);
    impl SPresc {
        #[doc = "Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
        #[inline(always)]
        pub const fn s_seg_1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
        #[inline(always)]
        pub fn set_s_seg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Bit Timing Segment 2 (slow speed) Time after the sample point."]
        #[inline(always)]
        pub const fn s_seg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Bit Timing Segment 2 (slow speed) Time after the sample point."]
        #[inline(always)]
        pub fn set_s_seg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
        #[inline(always)]
        pub const fn s_sjw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
        #[inline(always)]
        pub fn set_s_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
        #[inline(always)]
        pub const fn s_presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
        #[inline(always)]
        pub fn set_s_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SPresc {
        #[inline(always)]
        fn default() -> SPresc {
            SPresc(0)
        }
    }
    #[doc = "TTCAN: TB Slot Pointer TBSLOT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tbslot(pub u8);
    impl Tbslot {
        #[doc = "Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
        #[inline(always)]
        pub const fn tbptr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
        #[inline(always)]
        pub fn set_tbptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
        }
        #[doc = "set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
        #[inline(always)]
        pub const fn tbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
        #[inline(always)]
        pub fn set_tbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins."]
        #[inline(always)]
        pub const fn tbe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins."]
        #[inline(always)]
        pub fn set_tbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tbslot {
        #[inline(always)]
        fn default() -> Tbslot {
            Tbslot(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tbuf(pub u32);
    impl Tbuf {
        #[doc = "transmit buffer."]
        #[inline(always)]
        pub const fn tbuf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "transmit buffer."]
        #[inline(always)]
        pub fn set_tbuf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tbuf {
        #[inline(always)]
        fn default() -> Tbuf {
            Tbuf(0)
        }
    }
    #[doc = "Transmitter Delay Compensation Register TDC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdc(pub u8);
    impl Tdc {
        #[doc = "Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
        #[inline(always)]
        pub const fn sspoff(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
        #[inline(always)]
        pub fn set_sspoff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
        #[doc = "Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
        #[inline(always)]
        pub const fn tdcen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
        #[inline(always)]
        pub fn set_tdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Tdc {
        #[inline(always)]
        fn default() -> Tdc {
            Tdc(0)
        }
    }
    #[doc = "Error Counter Registers TECNT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tecnt(pub u8);
    impl Tecnt {
        #[doc = "Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen."]
        #[inline(always)]
        pub const fn tecnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen."]
        #[inline(always)]
        pub fn set_tecnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Tecnt {
        #[inline(always)]
        fn default() -> Tecnt {
            Tecnt(0)
        }
    }
    #[doc = "CiA 603 Time-Stamping TIMECFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timecfg(pub u8);
    impl Timecfg {
        #[doc = "TIME-stamping ENable 0 – disabled 1 – enabled."]
        #[inline(always)]
        pub const fn timeen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIME-stamping ENable 0 – disabled 1 – enabled."]
        #[inline(always)]
        pub fn set_timeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
        #[inline(always)]
        pub const fn timepos(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
        #[inline(always)]
        pub fn set_timepos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for Timecfg {
        #[inline(always)]
        fn default() -> Timecfg {
            Timecfg(0)
        }
    }
    #[doc = "TTCAN: Trigger Configuration TRIG_CFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigCfg(pub u16);
    impl TrigCfg {
        #[doc = "Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
        #[inline(always)]
        pub const fn ttptr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
        #[inline(always)]
        pub fn set_ttptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
        #[inline(always)]
        pub const fn ttype(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
        #[inline(always)]
        pub fn set_ttype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
        #[doc = "Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick."]
        #[inline(always)]
        pub const fn tew(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick."]
        #[inline(always)]
        pub fn set_tew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
        }
    }
    impl Default for TrigCfg {
        #[inline(always)]
        fn default() -> TrigCfg {
            TrigCfg(0)
        }
    }
    #[doc = "TTCAN: Trigger Time TT_TRIG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TtTrig(pub u16);
    impl TtTrig {
        #[doc = "Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
        #[inline(always)]
        pub const fn tt_trig(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
        #[inline(always)]
        pub fn set_tt_trig(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for TtTrig {
        #[inline(always)]
        fn default() -> TtTrig {
            TtTrig(0)
        }
    }
    #[doc = "TTCAN: Watch Trigger Time TT_WTRIG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TtWtrig(pub u16);
    impl TtWtrig {
        #[doc = "Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
        #[inline(always)]
        pub const fn tt_wtrig(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
        #[inline(always)]
        pub fn set_tt_wtrig(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for TtWtrig {
        #[inline(always)]
        fn default() -> TtWtrig {
            TtWtrig(0)
        }
    }
    #[doc = "TTCAN: Time Trigger Configuration TTCFG."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ttcfg(pub u8);
    impl Ttcfg {
        #[doc = "Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled."]
        #[inline(always)]
        pub const fn tten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled."]
        #[inline(always)]
        pub fn set_tten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
        #[inline(always)]
        pub const fn t_presc(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
        #[inline(always)]
        pub fn set_t_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[doc = "Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing a one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
        #[inline(always)]
        pub const fn ttif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing a one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
        #[inline(always)]
        pub fn set_ttif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
        #[inline(always)]
        pub const fn ttie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
        #[inline(always)]
        pub fn set_ttie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF."]
        #[inline(always)]
        pub const fn teif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF."]
        #[inline(always)]
        pub fn set_teif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
        #[inline(always)]
        pub const fn wtif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
        #[inline(always)]
        pub fn set_wtif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Watch Trigger Interrupt Enable."]
        #[inline(always)]
        pub const fn wtie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Watch Trigger Interrupt Enable."]
        #[inline(always)]
        pub fn set_wtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ttcfg {
        #[inline(always)]
        fn default() -> Ttcfg {
            Ttcfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tts(pub u32);
    impl Tts {
        #[doc = "transmission time stamp, word 0, LSB 32bit."]
        #[inline(always)]
        pub const fn tts_wrd0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "transmission time stamp, word 0, LSB 32bit."]
        #[inline(always)]
        pub fn set_tts_wrd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tts {
        #[inline(always)]
        fn default() -> Tts {
            Tts(0)
        }
    }
    #[doc = "Version Information VER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ver(pub u16);
    impl Ver {
        #[doc = "Version of CAN-CTRL, given as decimal value. VER_1 holds the major version and VER_0 the minor version.Example: version 5x16N00S00 is represented by VER_1=5 and VER_0=16."]
        #[inline(always)]
        pub const fn version(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Version of CAN-CTRL, given as decimal value. VER_1 holds the major version and VER_0 the minor version.Example: version 5x16N00S00 is represented by VER_1=5 and VER_0=16."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Ver {
        #[inline(always)]
        fn default() -> Ver {
            Ver(0)
        }
    }
}
