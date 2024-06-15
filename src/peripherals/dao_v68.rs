#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DAO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dao {
    ptr: *mut u8,
}
unsafe impl Send for Dao {}
unsafe impl Sync for Dao {}
impl Dao {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn rx_cfgr(self) -> crate::common::Reg<regs::RxCfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RX Slot Control Register."]
    #[inline(always)]
    pub const fn rxslt(self) -> crate::common::Reg<regs::Rxslt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "HPF A Coef Register."]
    #[inline(always)]
    pub const fn hpf_ma(self) -> crate::common::Reg<regs::HpfMa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "HPF B Coef Register."]
    #[inline(always)]
    pub const fn hpf_b(self) -> crate::common::Reg<regs::HpfB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Enable this module to run."]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable this module to run."]
        #[inline(always)]
        pub fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Self-clear."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "the module continues to consume data, but all the pads are constant, thus no audio out."]
        #[inline(always)]
        pub const fn false_run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the module continues to consume data, but all the pads are constant, thus no audio out."]
        #[inline(always)]
        pub fn set_false_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled."]
        #[inline(always)]
        pub const fn false_level(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled."]
        #[inline(always)]
        pub fn set_false_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "all the outputs are inverted before sending to pad."]
        #[inline(always)]
        pub const fn invert(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "all the outputs are inverted before sending to pad."]
        #[inline(always)]
        pub fn set_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version."]
        #[inline(always)]
        pub const fn remap(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version."]
        #[inline(always)]
        pub fn set_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asserted to enable the left channel."]
        #[inline(always)]
        pub const fn left_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the left channel."]
        #[inline(always)]
        pub fn set_left_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Asserted to enable the right channel."]
        #[inline(always)]
        pub const fn right_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the right channel."]
        #[inline(always)]
        pub fn set_right_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Asserted to let the left and right channel output the same value."]
        #[inline(always)]
        pub const fn mono(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to let the left and right channel output the same value."]
        #[inline(always)]
        pub fn set_mono(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Whether HPF is enabled. This HPF is used to filter out the DC part."]
        #[inline(always)]
        pub const fn hpf_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Whether HPF is enabled. This HPF is used to filter out the DC part."]
        #[inline(always)]
        pub fn set_hpf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "HPF B Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfB(pub u32);
    impl HpfB {
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfB {
        #[inline(always)]
        fn default() -> HpfB {
            HpfB(0)
        }
    }
    #[doc = "HPF A Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfMa(pub u32);
    impl HpfMa {
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfMa {
        #[inline(always)]
        fn default() -> HpfMa {
            HpfMa(0)
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxCfgr(pub u32);
    impl RxCfgr {
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn chsiz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_chsiz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn datsiz(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_datsiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard For more details on I2S standards. Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub const fn std(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard For more details on I2S standards. Note: For correct operation, these bits should be configured when the I2S is disabled."]
        #[inline(always)]
        pub fn set_std(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[inline(always)]
        pub const fn tdm_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TDM mode 0: not TDM mode 1: TDM mode."]
        #[inline(always)]
        pub fn set_tdm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc."]
        #[inline(always)]
        pub const fn ch_max(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc."]
        #[inline(always)]
        pub fn set_ch_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[inline(always)]
        pub const fn frame_edge(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame."]
        #[inline(always)]
        pub fn set_frame_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for RxCfgr {
        #[inline(always)]
        fn default() -> RxCfgr {
            RxCfgr(0)
        }
    }
    #[doc = "RX Slot Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxslt(pub u32);
    impl Rxslt {
        #[doc = "Slot enable for the channels."]
        #[inline(always)]
        pub const fn en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Slot enable for the channels."]
        #[inline(always)]
        pub fn set_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxslt {
        #[inline(always)]
        fn default() -> Rxslt {
            Rxslt(0)
        }
    }
}
