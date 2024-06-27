#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data FIFO Path Control Register."]
    #[inline(always)]
    pub const fn sdfifoctrl(self) -> crate::common::Reg<regs::Sdfifoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data Path Control Primary Register."]
    #[inline(always)]
    pub const fn sdctrlp(self) -> crate::common::Reg<regs::Sdctrlp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Data Path Control Extra Register."]
    #[inline(always)]
    pub const fn sdctrle(self) -> crate::common::Reg<regs::Sdctrle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Data Path Status."]
    #[inline(always)]
    pub const fn sdst(self) -> crate::common::Reg<regs::Sdst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn sdata(self) -> crate::common::Reg<regs::Sdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FIFO Data."]
    #[inline(always)]
    pub const fn sdfifo(self) -> crate::common::Reg<regs::Sdfifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "instant Amplitude Results."]
    #[inline(always)]
    pub const fn scamp(self) -> crate::common::Reg<regs::Scamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Amplitude Threshold for High Limit."]
    #[inline(always)]
    pub const fn schtl(self) -> crate::common::Reg<regs::Schtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Amplitude Threshold for zero crossing."]
    #[inline(always)]
    pub const fn schtlz(self) -> crate::common::Reg<regs::Schtlz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Amplitude Threshold for low limit."]
    #[inline(always)]
    pub const fn scllt(self) -> crate::common::Reg<regs::Scllt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Amplitude Path Control."]
    #[inline(always)]
    pub const fn scctrl(self) -> crate::common::Reg<regs::Scctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Amplitude Path Status."]
    #[inline(always)]
    pub const fn scst(self) -> crate::common::Reg<regs::Scst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "SDM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdm {
    ptr: *mut u8,
}
unsafe impl Send for Sdm {}
unsafe impl Sync for Sdm {}
impl Sdm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDM control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status Registers."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 4usize);
        unsafe { Ch::from_ptr(self.ptr.add(0x10usize + n * 64usize) as _) }
    }
}
pub mod regs {
    #[doc = "SDM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Interrupt Enable."]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable."]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel Enable."]
        #[inline(always)]
        pub const fn ch_en(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel Enable."]
        #[inline(always)]
        pub fn set_ch_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Asserted to double sync the mdat input pin before its usage inside the module."]
        #[inline(always)]
        pub const fn sync_mdat(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted to double sync the mdat input pin before its usage inside the module."]
        #[inline(always)]
        pub fn set_sync_mdat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "Asserted to double sync the mclk input pin before its usage inside the module."]
        #[inline(always)]
        pub const fn sync_mclk(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "Asserted to double sync the mclk input pin before its usage inside the module."]
        #[inline(always)]
        pub fn set_sync_mclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined."]
        #[inline(always)]
        pub const fn chmd(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x0fff;
            val as u16
        }
        #[doc = "Channel Rcv mode Bits\\[2:0\\]
for Ch0. Bits\\[5:3\\]
for Ch1 Bits\\[8:6\\]
for Ch2 Bits\\[11:9\\]
for Ch3 3'b000: Capture at posedge of MCLK 3'b001: Capture at both posedge and negedge of MCLK 3'b010: Manchestor Mode 3'b011: Capture at negedge of MCLK 3'b100: Capture at every other posedge of MCLK 3'b101: Capture at every other negedge of MCLK Others: Undefined."]
        #[inline(always)]
        pub fn set_chmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 14usize)) | (((val as u32) & 0x0fff) << 14usize);
        }
        #[doc = "software reset the module if asserted to be1’b1."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the module if asserted to be1’b1."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Ch0 Error interrupt enable."]
        #[inline(always)]
        pub const fn ch0err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ch0 Error interrupt enable."]
        #[inline(always)]
        pub fn set_ch0err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Ch1 Error interrupt enable."]
        #[inline(always)]
        pub const fn ch1err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Ch1 Error interrupt enable."]
        #[inline(always)]
        pub fn set_ch1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Ch2 Error interrupt enable."]
        #[inline(always)]
        pub const fn ch2err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Ch2 Error interrupt enable."]
        #[inline(always)]
        pub fn set_ch2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Ch3 Error interrupt enable."]
        #[inline(always)]
        pub const fn ch3err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Ch3 Error interrupt enable."]
        #[inline(always)]
        pub fn set_ch3err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Ch0 Data Ready interrupt enable."]
        #[inline(always)]
        pub const fn ch0dry(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Ch0 Data Ready interrupt enable."]
        #[inline(always)]
        pub fn set_ch0dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Ch1 Data Ready interrupt enable."]
        #[inline(always)]
        pub const fn ch1dry(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Ch1 Data Ready interrupt enable."]
        #[inline(always)]
        pub fn set_ch1dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ch2 Data Ready interrupt enable."]
        #[inline(always)]
        pub const fn ch2dry(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ch2 Data Ready interrupt enable."]
        #[inline(always)]
        pub fn set_ch2dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Ch3 Data Ready interrupt enable."]
        #[inline(always)]
        pub const fn ch3dry(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ch3 Data Ready interrupt enable."]
        #[inline(always)]
        pub fn set_ch3dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "instant Amplitude Results."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scamp(pub u32);
    impl Scamp {
        #[doc = "instant Amplitude Results."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "instant Amplitude Results."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Scamp {
        #[inline(always)]
        fn default() -> Scamp {
            Scamp(0)
        }
    }
    #[doc = "Amplitude Path Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scctrl(pub u32);
    impl Scctrl {
        #[doc = "Amplitude Path Enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Amplitude Path Enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NotZero: Ignore the first samples that are not accurate Zero: Use all samples."]
        #[inline(always)]
        pub const fn ign_ini_samples(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "NotZero: Ignore the first samples that are not accurate Zero: Use all samples."]
        #[inline(always)]
        pub fn set_ign_ini_samples(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "CIC decimation ratio. 0 means div-by-32."]
        #[inline(always)]
        pub const fn cic_dec_ratio(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "CIC decimation ratio. 0 means div-by-32."]
        #[inline(always)]
        pub fn set_cic_dec_ratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[doc = "CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC."]
        #[inline(always)]
        pub const fn sgd_ordr(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC."]
        #[inline(always)]
        pub fn set_sgd_ordr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "LLT interrupt Enable."]
        #[inline(always)]
        pub const fn ll_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LLT interrupt Enable."]
        #[inline(always)]
        pub fn set_ll_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HLT Interrupt Enable."]
        #[inline(always)]
        pub const fn hl_ie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "HLT Interrupt Enable."]
        #[inline(always)]
        pub fn set_hl_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Module failure Interrupt enable."]
        #[inline(always)]
        pub const fn mf_ie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Module failure Interrupt enable."]
        #[inline(always)]
        pub fn set_mf_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Zero Crossing Enable."]
        #[inline(always)]
        pub const fn hz_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Zero Crossing Enable."]
        #[inline(always)]
        pub fn set_hz_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Scctrl {
        #[inline(always)]
        fn default() -> Scctrl {
            Scctrl(0)
        }
    }
    #[doc = "Amplitude Threshold for High Limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Schtl(pub u32);
    impl Schtl {
        #[doc = "Amplitude Threshold for High Limit."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Amplitude Threshold for High Limit."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Schtl {
        #[inline(always)]
        fn default() -> Schtl {
            Schtl(0)
        }
    }
    #[doc = "Amplitude Threshold for zero crossing."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Schtlz(pub u32);
    impl Schtlz {
        #[doc = "Amplitude Threshold for zero crossing."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Amplitude Threshold for zero crossing."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Schtlz {
        #[inline(always)]
        fn default() -> Schtlz {
            Schtlz(0)
        }
    }
    #[doc = "Amplitude Threshold for low limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scllt(pub u32);
    impl Scllt {
        #[doc = "Amplitude Threshold for low limit."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Amplitude Threshold for low limit."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Scllt {
        #[inline(always)]
        fn default() -> Scllt {
            Scllt(0)
        }
    }
    #[doc = "Amplitude Path Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scst(pub u32);
    impl Scst {
        #[doc = "LLT out of range. Error flag."]
        #[inline(always)]
        pub const fn cmpl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LLT out of range. Error flag."]
        #[inline(always)]
        pub fn set_cmpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HLT out of range. Error flag."]
        #[inline(always)]
        pub const fn cmph(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HLT out of range. Error flag."]
        #[inline(always)]
        pub fn set_cmph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "power modulator Failure found. MCLK not found. Error flag."]
        #[inline(always)]
        pub const fn mf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "power modulator Failure found. MCLK not found. Error flag."]
        #[inline(always)]
        pub fn set_mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Amplitude rising above HZ event found."]
        #[inline(always)]
        pub const fn hz(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Amplitude rising above HZ event found."]
        #[inline(always)]
        pub fn set_hz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Scst {
        #[inline(always)]
        fn default() -> Scst {
            Scst(0)
        }
    }
    #[doc = "Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdata(pub u32);
    impl Sdata {
        #[doc = "Data."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sdata {
        #[inline(always)]
        fn default() -> Sdata {
            Sdata(0)
        }
    }
    #[doc = "Data Path Control Extra Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdctrle(pub u32);
    impl Sdctrle {
        #[doc = "NotZero: Don't store the first samples that are not accurate Zero: Store all samples."]
        #[inline(always)]
        pub const fn ign_ini_samples(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "NotZero: Don't store the first samples that are not accurate Zero: Store all samples."]
        #[inline(always)]
        pub fn set_ign_ini_samples(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "CIC decimation ratio. 0 means div-by-256."]
        #[inline(always)]
        pub const fn cic_dec_ratio(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "CIC decimation ratio. 0 means div-by-256."]
        #[inline(always)]
        pub fn set_cic_dec_ratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "CIC shift control."]
        #[inline(always)]
        pub const fn cic_scl(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "CIC shift control."]
        #[inline(always)]
        pub fn set_cic_scl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "Asserted to double sync the PWM trigger signal."]
        #[inline(always)]
        pub const fn pwmsync(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to double sync the PWM trigger signal."]
        #[inline(always)]
        pub fn set_pwmsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC."]
        #[inline(always)]
        pub const fn sgd_ordr(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "CIC order 0: SYNC1 1: SYNC2 2: SYNC3 3: FAST_SYNC."]
        #[inline(always)]
        pub fn set_sgd_ordr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "\"1: the read output of SData is data and timestamp interleaved. First is data. 0: the read output of SData is data only\"."]
        #[inline(always)]
        pub const fn data_s_t(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "\"1: the read output of SData is data and timestamp interleaved. First is data. 0: the read output of SData is data only\"."]
        #[inline(always)]
        pub fn set_data_s_t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "1: the output of SDFIFO is data and timestamp interleaved. First is data. 0: the output of SDFIFO is data only."]
        #[inline(always)]
        pub const fn dfifo_s_t(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1: the output of SDFIFO is data and timestamp interleaved. First is data. 0: the output of SDFIFO is data only."]
        #[inline(always)]
        pub fn set_dfifo_s_t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1. Use the time (when the data is calculated out) - delta_time_of_filter_span as the timestamp. 0: Use the time when the data is calculated out."]
        #[inline(always)]
        pub const fn timestamp_type(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1. Use the time (when the data is calculated out) - delta_time_of_filter_span as the timestamp. 0: Use the time when the data is calculated out."]
        #[inline(always)]
        pub fn set_timestamp_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1: the CIC stage can be paused by the mask input. 0: the CIC stage won't be paused by the mask input."]
        #[inline(always)]
        pub const fn cic_gate_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "1: the CIC stage can be paused by the mask input. 0: the CIC stage won't be paused by the mask input."]
        #[inline(always)]
        pub fn set_cic_gate_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Select the mask signal for CIC gate signal."]
        #[inline(always)]
        pub const fn cic_gate_sel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x0f;
            val as u8
        }
        #[doc = "Select the mask signal for CIC gate signal."]
        #[inline(always)]
        pub fn set_cic_gate_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
        }
        #[doc = "1: When mask signal is 1, pause the CIC stage at he rising edge of mask signal. 0: When mask signal is 0, pause the CIC stage at he falling edge of mask signal."]
        #[inline(always)]
        pub const fn cic_gate_pol(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1: When mask signal is 1, pause the CIC stage at he rising edge of mask signal. 0: When mask signal is 0, pause the CIC stage at he falling edge of mask signal."]
        #[inline(always)]
        pub fn set_cic_gate_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1: the gate cycle is determined by SDFIFOCTRLn\\[GATE_SAMPLES\\]. 0: the gate cycle is determined by the CIC decimation counter, and the minimal gated off PDM bits are determined by SDFIFOCTRLn\\[GATE_SAMPLES\\], and at the same time, to keep alignment with normal PCM sampling time."]
        #[inline(always)]
        pub const fn cic_gate_type(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1: the gate cycle is determined by SDFIFOCTRLn\\[GATE_SAMPLES\\]. 0: the gate cycle is determined by the CIC decimation counter, and the minimal gated off PDM bits are determined by SDFIFOCTRLn\\[GATE_SAMPLES\\], and at the same time, to keep alignment with normal PCM sampling time."]
        #[inline(always)]
        pub fn set_cic_gate_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sdctrle {
        #[inline(always)]
        fn default() -> Sdctrle {
            Sdctrle(0)
        }
    }
    #[doc = "Data Path Control Primary Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdctrlp(pub u32);
    impl Sdctrlp {
        #[doc = "Data Path Enable."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Path Enable."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready."]
        #[inline(always)]
        pub const fn dr_opt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready."]
        #[inline(always)]
        pub fn set_dr_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1:32 bit data 0:16 bit data."]
        #[inline(always)]
        pub const fn d32(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1:32 bit data 0:16 bit data."]
        #[inline(always)]
        pub fn set_d32(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled."]
        #[inline(always)]
        pub const fn wtsyncen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled."]
        #[inline(always)]
        pub fn set_wtsyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1: Manually clear WTSYNFLG. Auto-clear."]
        #[inline(always)]
        pub const fn wtsynmclr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1: Manually clear WTSYNFLG. Auto-clear."]
        #[inline(always)]
        pub fn set_wtsynmclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR."]
        #[inline(always)]
        pub const fn wtsynaclr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR."]
        #[inline(always)]
        pub fn set_wtsynaclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1."]
        #[inline(always)]
        pub const fn ffsyncclren(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1."]
        #[inline(always)]
        pub fn set_ffsyncclren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Select the PWM SYNC Source."]
        #[inline(always)]
        pub const fn syncsel(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x3f;
            val as u8
        }
        #[doc = "Select the PWM SYNC Source."]
        #[inline(always)]
        pub fn set_syncsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
        }
        #[doc = "Ch Data Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn drie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Ch Data Ready Interrupt Enable."]
        #[inline(always)]
        pub fn set_drie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Ch CIC Data Saturation Interrupt Enable."]
        #[inline(always)]
        pub const fn dsatie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ch CIC Data Saturation Interrupt Enable."]
        #[inline(always)]
        pub fn set_dsatie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ch Data FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn dffovie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ch Data FIFO overflow interrupt enable."]
        #[inline(always)]
        pub fn set_dffovie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Acknowledge feedback interrupt enable."]
        #[inline(always)]
        pub const fn af_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge feedback interrupt enable."]
        #[inline(always)]
        pub fn set_af_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Watch dog threshold for channel failure of CLK halting."]
        #[inline(always)]
        pub const fn wdog_thr(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0xff;
            val as u8
        }
        #[doc = "Watch dog threshold for channel failure of CLK halting."]
        #[inline(always)]
        pub fn set_wdog_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 17usize)) | (((val as u32) & 0xff) << 17usize);
        }
        #[doc = "Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]."]
        #[inline(always)]
        pub const fn manch_thr(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[doc = "Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]."]
        #[inline(always)]
        pub fn set_manch_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for Sdctrlp {
        #[inline(always)]
        fn default() -> Sdctrlp {
            Sdctrlp(0)
        }
    }
    #[doc = "FIFO Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdfifo(pub u32);
    impl Sdfifo {
        #[doc = "FIFO Data."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO Data."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sdfifo {
        #[inline(always)]
        fn default() -> Sdfifo {
            Sdfifo(0)
        }
    }
    #[doc = "Data FIFO Path Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdfifoctrl(pub u32);
    impl Sdfifoctrl {
        #[doc = "FIFO data ready interrupt enable."]
        #[inline(always)]
        pub const fn d_rdy_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO data ready interrupt enable."]
        #[inline(always)]
        pub fn set_d_rdy_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO threshold (0,..,16) (fillings > threshold, then gen int)."]
        #[inline(always)]
        pub const fn thrsh(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "FIFO threshold (0,..,16) (fillings > threshold, then gen int)."]
        #[inline(always)]
        pub fn set_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[doc = "The number-1-3 of input PDM bit samples to be gated when CIC_GATE_EN=1. Max 255. So the minimum gated samples is 4 samples when GATE_SAMPLES=0."]
        #[inline(always)]
        pub const fn gate_samples(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The number-1-3 of input PDM bit samples to be gated when CIC_GATE_EN=1. Max 255. So the minimum gated samples is 4 samples when GATE_SAMPLES=0."]
        #[inline(always)]
        pub fn set_gate_samples(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Sdfifoctrl {
        #[inline(always)]
        fn default() -> Sdfifoctrl {
            Sdfifoctrl(0)
        }
    }
    #[doc = "Data Path Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdst(pub u32);
    impl Sdst {
        #[doc = "Data FIFO Fillings."]
        #[inline(always)]
        pub const fn fill(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Data FIFO Fillings."]
        #[inline(always)]
        pub fn set_fill(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Wait-for-sync event found."]
        #[inline(always)]
        pub const fn wtsynflg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Wait-for-sync event found."]
        #[inline(always)]
        pub fn set_wtsynflg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CIC out Data saturation err. Error flag."]
        #[inline(always)]
        pub const fn dsat_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CIC out Data saturation err. Error flag."]
        #[inline(always)]
        pub fn set_dsat_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data FIFO Overflow Error. Error flag."]
        #[inline(always)]
        pub const fn dov_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data FIFO Overflow Error. Error flag."]
        #[inline(always)]
        pub fn set_dov_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Achnowledge flag."]
        #[inline(always)]
        pub const fn af(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Achnowledge flag."]
        #[inline(always)]
        pub fn set_af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO data ready."]
        #[inline(always)]
        pub const fn fifo_dr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO data ready."]
        #[inline(always)]
        pub fn set_fifo_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1: next readout is timestamp 0: next readout is data."]
        #[inline(always)]
        pub const fn sdfifo_d0_t1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1: next readout is timestamp 0: next readout is data."]
        #[inline(always)]
        pub fn set_sdfifo_d0_t1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1: next readout is timestamp 0: next readout is data."]
        #[inline(always)]
        pub const fn sdata_d0_t1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1: next readout is timestamp 0: next readout is data."]
        #[inline(always)]
        pub fn set_sdata_d0_t1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "maxim of mclk spacing in cycles, using edges of mclk signal. In manchester coding mode, it is just the period of MCLK. In other modes, it is almost the half period."]
        #[inline(always)]
        pub const fn period_mclk(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0xff;
            val as u8
        }
        #[doc = "maxim of mclk spacing in cycles, using edges of mclk signal. In manchester coding mode, it is just the period of MCLK. In other modes, it is almost the half period."]
        #[inline(always)]
        pub fn set_period_mclk(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 23usize)) | (((val as u32) & 0xff) << 23usize);
        }
    }
    impl Default for Sdst {
        #[inline(always)]
        fn default() -> Sdst {
            Sdst(0)
        }
    }
    #[doc = "Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Ch0 Error."]
        #[inline(always)]
        pub const fn ch0err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ch0 Error."]
        #[inline(always)]
        pub fn set_ch0err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Ch1 Error."]
        #[inline(always)]
        pub const fn ch1err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Ch1 Error."]
        #[inline(always)]
        pub fn set_ch1err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Ch2 Error."]
        #[inline(always)]
        pub const fn ch2err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Ch2 Error."]
        #[inline(always)]
        pub fn set_ch2err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Ch3 Error. ORed together by channel related error signals and corresponding error interrupt enable signals. De-assert this bit by write-1-clear the corresponding error status bits in the channel status registers."]
        #[inline(always)]
        pub const fn ch3err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Ch3 Error. ORed together by channel related error signals and corresponding error interrupt enable signals. De-assert this bit by write-1-clear the corresponding error status bits in the channel status registers."]
        #[inline(always)]
        pub fn set_ch3err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Ch0 Data Ready."]
        #[inline(always)]
        pub const fn ch0dry(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Ch0 Data Ready."]
        #[inline(always)]
        pub fn set_ch0dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Ch1 Data Ready."]
        #[inline(always)]
        pub const fn ch1dry(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Ch1 Data Ready."]
        #[inline(always)]
        pub fn set_ch1dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ch2 Data Ready."]
        #[inline(always)]
        pub const fn ch2dry(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ch2 Data Ready."]
        #[inline(always)]
        pub fn set_ch2dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Ch3 Data Ready. De-assert this bit by reading the data (or data fifo) registers."]
        #[inline(always)]
        pub const fn ch3dry(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ch3 Data Ready. De-assert this bit by reading the data (or data fifo) registers."]
        #[inline(always)]
        pub fn set_ch3dry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
}
