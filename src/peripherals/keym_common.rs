#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "KEYM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keym {
    ptr: *mut u8,
}
unsafe impl Send for Keym {}
unsafe impl Sync for Keym {}
impl Keym {
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
    pub const fn softmkey(self, n: usize) -> crate::common::Reg<regs::Softmkey, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn softpkey(self, n: usize) -> crate::common::Reg<regs::Softpkey, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "secure key generation."]
    #[inline(always)]
    pub const fn sec_key_ctl(self) -> crate::common::Reg<regs::SecKeyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "non-secure key generation."]
    #[inline(always)]
    pub const fn nsc_key_ctl(self) -> crate::common::Reg<regs::NscKeyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Random number interface behavior."]
    #[inline(always)]
    pub const fn rng(self) -> crate::common::Reg<regs::Rng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "key read out control."]
    #[inline(always)]
    pub const fn read_control(self) -> crate::common::Reg<regs::ReadControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "non-secure key generation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NscKeyCtl(pub u32);
    impl NscKeyCtl {
        #[doc = "non-secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub const fn key_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "non-secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub fn set_key_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key."]
        #[inline(always)]
        pub const fn fmk_sel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key."]
        #[inline(always)]
        pub fn set_fmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn zmk_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub fn set_zmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn smk_sel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub fn set_smk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub const fn sk_val(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub fn set_sk_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "block non-secure state key setting being changed."]
        #[inline(always)]
        pub const fn lock_nsc_ctl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "block non-secure state key setting being changed."]
        #[inline(always)]
        pub fn set_lock_nsc_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for NscKeyCtl {
        #[inline(always)]
        fn default() -> NscKeyCtl {
            NscKeyCtl(0)
        }
    }
    #[doc = "key read out control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ReadControl(pub u32);
    impl ReadControl {
        #[doc = "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub const fn block_smk_read(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub fn set_block_smk_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub const fn block_pk_read(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub fn set_block_pk_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for ReadControl {
        #[inline(always)]
        fn default() -> ReadControl {
            ReadControl(0)
        }
    }
    #[doc = "Random number interface behavior."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rng(pub u32);
    impl Rng {
        #[doc = "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG."]
        #[inline(always)]
        pub const fn rng_xor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG."]
        #[inline(always)]
        pub fn set_rng_xor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software."]
        #[inline(always)]
        pub const fn block_rng_xor(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software."]
        #[inline(always)]
        pub fn set_block_rng_xor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rng {
        #[inline(always)]
        fn default() -> Rng {
            Rng(0)
        }
    }
    #[doc = "secure key generation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecKeyCtl(pub u32);
    impl SecKeyCtl {
        #[doc = "secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub const fn key_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub fn set_key_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key."]
        #[inline(always)]
        pub const fn fmk_sel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key."]
        #[inline(always)]
        pub fn set_fmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn zmk_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub fn set_zmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key."]
        #[inline(always)]
        pub const fn smk_sel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key."]
        #[inline(always)]
        pub fn set_smk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub const fn sk_val(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub fn set_sk_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "block secure state key setting being changed."]
        #[inline(always)]
        pub const fn lock_sec_ctl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "block secure state key setting being changed."]
        #[inline(always)]
        pub fn set_lock_sec_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SecKeyCtl {
        #[inline(always)]
        fn default() -> SecKeyCtl {
            SecKeyCtl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Softmkey(pub u32);
    impl Softmkey {
        #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0."]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Softmkey {
        #[inline(always)]
        fn default() -> Softmkey {
            Softmkey(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Softpkey(pub u32);
    impl Softpkey {
        #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0."]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Softpkey {
        #[inline(always)]
        fn default() -> Softpkey {
            Softpkey(0)
        }
    }
}
