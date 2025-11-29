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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn softpkey(self, n: usize) -> crate::common::Reg<regs::Softpkey, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "secure key generation."]
    #[inline(always)]
    pub const fn sec_key_ctl(self) -> crate::common::Reg<regs::SecKeyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "non-secure key generation."]
    #[inline(always)]
    pub const fn nsc_key_ctl(self) -> crate::common::Reg<regs::NscKeyCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Random number interface behavior."]
    #[inline(always)]
    pub const fn rng(self) -> crate::common::Reg<regs::Rng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "key read out control."]
    #[inline(always)]
    pub const fn read_control(self) -> crate::common::Reg<regs::ReadControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
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
    #[doc = "non-secure key generation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NscKeyCtl(pub u32);
    impl NscKeyCtl {
        #[doc = "non-secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[must_use]
        #[inline(always)]
        pub const fn key_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "non-secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub const fn set_key_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn fmk_sel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key."]
        #[inline(always)]
        pub const fn set_fmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn zmk_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn set_zmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn smk_sel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn set_smk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn sk_val(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub const fn set_sk_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "block non-secure state key setting being changed."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_nsc_ctl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "block non-secure state key setting being changed."]
        #[inline(always)]
        pub const fn set_lock_nsc_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for NscKeyCtl {
        #[inline(always)]
        fn default() -> NscKeyCtl {
            NscKeyCtl(0)
        }
    }
    impl core::fmt::Debug for NscKeyCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NscKeyCtl")
                .field("key_sel", &self.key_sel())
                .field("fmk_sel", &self.fmk_sel())
                .field("zmk_sel", &self.zmk_sel())
                .field("smk_sel", &self.smk_sel())
                .field("sk_val", &self.sk_val())
                .field("lock_nsc_ctl", &self.lock_nsc_ctl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NscKeyCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "NscKeyCtl {{ key_sel: {=u8:?}, fmk_sel: {=bool:?}, zmk_sel: {=bool:?}, smk_sel: {=bool:?}, sk_val: {=bool:?}, lock_nsc_ctl: {=bool:?} }}" , self . key_sel () , self . fmk_sel () , self . zmk_sel () , self . smk_sel () , self . sk_val () , self . lock_nsc_ctl ())
        }
    }
    #[doc = "key read out control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ReadControl(pub u32);
    impl ReadControl {
        #[doc = "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[must_use]
        #[inline(always)]
        pub const fn block_smk_read(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub const fn set_block_smk_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[must_use]
        #[inline(always)]
        pub const fn block_pk_read(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out."]
        #[inline(always)]
        pub const fn set_block_pk_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for ReadControl {
        #[inline(always)]
        fn default() -> ReadControl {
            ReadControl(0)
        }
    }
    impl core::fmt::Debug for ReadControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ReadControl")
                .field("block_smk_read", &self.block_smk_read())
                .field("block_pk_read", &self.block_pk_read())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ReadControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ReadControl {{ block_smk_read: {=bool:?}, block_pk_read: {=bool:?} }}",
                self.block_smk_read(),
                self.block_pk_read()
            )
        }
    }
    #[doc = "Random number interface behavior."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rng(pub u32);
    impl Rng {
        #[doc = "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG."]
        #[must_use]
        #[inline(always)]
        pub const fn rng_xor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG."]
        #[inline(always)]
        pub const fn set_rng_xor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software."]
        #[must_use]
        #[inline(always)]
        pub const fn block_rng_xor(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software."]
        #[inline(always)]
        pub const fn set_block_rng_xor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rng {
        #[inline(always)]
        fn default() -> Rng {
            Rng(0)
        }
    }
    impl core::fmt::Debug for Rng {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rng")
                .field("rng_xor", &self.rng_xor())
                .field("block_rng_xor", &self.block_rng_xor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rng {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rng {{ rng_xor: {=bool:?}, block_rng_xor: {=bool:?} }}",
                self.rng_xor(),
                self.block_rng_xor()
            )
        }
    }
    #[doc = "secure key generation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecKeyCtl(pub u32);
    impl SecKeyCtl {
        #[doc = "secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[must_use]
        #[inline(always)]
        pub const fn key_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected."]
        #[inline(always)]
        pub const fn set_key_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn fmk_sel(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key."]
        #[inline(always)]
        pub const fn set_fmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn zmk_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key."]
        #[inline(always)]
        pub const fn set_zmk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key."]
        #[must_use]
        #[inline(always)]
        pub const fn smk_sel(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key."]
        #[inline(always)]
        pub const fn set_smk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn sk_val(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid."]
        #[inline(always)]
        pub const fn set_sk_val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "block secure state key setting being changed."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_sec_ctl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "block secure state key setting being changed."]
        #[inline(always)]
        pub const fn set_lock_sec_ctl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SecKeyCtl {
        #[inline(always)]
        fn default() -> SecKeyCtl {
            SecKeyCtl(0)
        }
    }
    impl core::fmt::Debug for SecKeyCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecKeyCtl")
                .field("key_sel", &self.key_sel())
                .field("fmk_sel", &self.fmk_sel())
                .field("zmk_sel", &self.zmk_sel())
                .field("smk_sel", &self.smk_sel())
                .field("sk_val", &self.sk_val())
                .field("lock_sec_ctl", &self.lock_sec_ctl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecKeyCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SecKeyCtl {{ key_sel: {=u8:?}, fmk_sel: {=bool:?}, zmk_sel: {=bool:?}, smk_sel: {=bool:?}, sk_val: {=bool:?}, lock_sec_ctl: {=bool:?} }}" , self . key_sel () , self . fmk_sel () , self . zmk_sel () , self . smk_sel () , self . sk_val () , self . lock_sec_ctl ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Softmkey(pub u32);
    impl Softmkey {
        #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Softmkey {
        #[inline(always)]
        fn default() -> Softmkey {
            Softmkey(0)
        }
    }
    impl core::fmt::Debug for Softmkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Softmkey")
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Softmkey {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Softmkey {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Softpkey(pub u32);
    impl Softpkey {
        #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Softpkey {
        #[inline(always)]
        fn default() -> Softpkey {
            Softpkey(0)
        }
    }
    impl core::fmt::Debug for Softpkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Softpkey")
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Softpkey {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Softpkey {{ key: {=u32:?} }}", self.key())
        }
    }
}
