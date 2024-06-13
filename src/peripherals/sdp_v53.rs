#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SDP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdp {
    ptr: *mut u8,
}
unsafe impl Send for Sdp {}
unsafe impl Sync for Sdp {}
impl Sdp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDP control register."]
    #[inline(always)]
    pub const fn sdpcr(self) -> crate::common::Reg<regs::Sdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Mod control register."]
    #[inline(always)]
    pub const fn modctrl(self) -> crate::common::Reg<regs::Modctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "packet counter registers."]
    #[inline(always)]
    pub const fn pktcnt(self) -> crate::common::Reg<regs::Pktcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Status Registers."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Key Address."]
    #[inline(always)]
    pub const fn keyaddr(self) -> crate::common::Reg<regs::Keyaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Key Data."]
    #[inline(always)]
    pub const fn keydat(self) -> crate::common::Reg<regs::Keydat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ciphiv(self, n: usize) -> crate::common::Reg<regs::Ciphiv, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn haswrd(self, n: usize) -> crate::common::Reg<regs::Haswrd, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize + n * 4usize) as _) }
    }
    #[doc = "Command Pointer."]
    #[inline(always)]
    pub const fn cmdptr(self) -> crate::common::Reg<regs::Cmdptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Next Packet Address Pointer."]
    #[inline(always)]
    pub const fn npktptr(self) -> crate::common::Reg<regs::Npktptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Packet Control Registers."]
    #[inline(always)]
    pub const fn pktctl(self) -> crate::common::Reg<regs::Pktctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Packet Memory Source Address."]
    #[inline(always)]
    pub const fn pktsrc(self) -> crate::common::Reg<regs::Pktsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Packet Memory Destination Address."]
    #[inline(always)]
    pub const fn pktdst(self) -> crate::common::Reg<regs::Pktdst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Packet buffer size."]
    #[inline(always)]
    pub const fn pktbuf(self) -> crate::common::Reg<regs::Pktbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ciphiv(pub u32);
    impl Ciphiv {
        #[doc = "cipher initialization vector."]
        #[inline(always)]
        pub const fn ciphiv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cipher initialization vector."]
        #[inline(always)]
        pub fn set_ciphiv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ciphiv {
        #[inline(always)]
        fn default() -> Ciphiv {
            Ciphiv(0)
        }
    }
    #[doc = "Command Pointer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdptr(pub u32);
    impl Cmdptr {
        #[doc = "current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)."]
        #[inline(always)]
        pub const fn cmdptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)."]
        #[inline(always)]
        pub fn set_cmdptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmdptr {
        #[inline(always)]
        fn default() -> Cmdptr {
            Cmdptr(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Haswrd(pub u32);
    impl Haswrd {
        #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
        #[inline(always)]
        pub const fn haswrd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
        #[inline(always)]
        pub fn set_haswrd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Haswrd {
        #[inline(always)]
        fn default() -> Haswrd {
            Haswrd(0)
        }
    }
    #[doc = "Key Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyaddr(pub u32);
    impl Keyaddr {
        #[doc = "Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
        #[inline(always)]
        pub const fn subwrd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
        #[inline(always)]
        pub fn set_subwrd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
        #[inline(always)]
        pub fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Keyaddr {
        #[inline(always)]
        fn default() -> Keyaddr {
            Keyaddr(0)
        }
    }
    #[doc = "Key Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keydat(pub u32);
    impl Keydat {
        #[doc = "This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key."]
        #[inline(always)]
        pub const fn keydat(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key."]
        #[inline(always)]
        pub fn set_keydat(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keydat {
        #[inline(always)]
        fn default() -> Keydat {
            Keydat(0)
        }
    }
    #[doc = "Mod control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modctrl(pub u32);
    impl Modctrl {
        #[doc = "Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub const fn keyswp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub fn set_keyswp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub const fn doutswp(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub fn set_doutswp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub const fn dinswp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format."]
        #[inline(always)]
        pub fn set_dinswp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH."]
        #[inline(always)]
        pub const fn hasout(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH."]
        #[inline(always)]
        pub fn set_hasout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
        #[inline(always)]
        pub const fn haschk(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
        #[inline(always)]
        pub fn set_haschk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —."]
        #[inline(always)]
        pub const fn hasalg(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —."]
        #[inline(always)]
        pub fn set_hasalg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "AES direction 1x1, AES Decryption 1x0, AES Encryption."]
        #[inline(always)]
        pub const fn aesdir(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES direction 1x1, AES Decryption 1x0, AES Encryption."]
        #[inline(always)]
        pub fn set_aesdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
        #[inline(always)]
        pub const fn aesks(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\]
as AES key. 0x21: kman_sk0\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\]
as AES key. 0x23: kman_sk1\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\]
as AES key. 0x25: kman_sk2\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\]
from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\]
as AES key. 0x27: kman_sk3\\[255:128\\]
from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\]
from OTP for AES128; AES256 will use exip0_key\\[255:0\\]
as AES key. 0x31: exip0_key\\[255:128\\]
from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\]
from OTP for AES128; AES256 will use exip1_key\\[255:0\\]
as AES key. 0x33: exip1_key\\[255:128\\]
from OTP for AES128; not valid for AES256. Other values, reserved."]
        #[inline(always)]
        pub fn set_aesks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
        #[inline(always)]
        pub const fn aesmod(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
        #[inline(always)]
        pub fn set_aesmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; 0x8 = SM4； Others, reserved."]
        #[inline(always)]
        pub const fn aesalg(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; 0x8 = SM4； Others, reserved."]
        #[inline(always)]
        pub fn set_aesalg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Modctrl {
        #[inline(always)]
        fn default() -> Modctrl {
            Modctrl(0)
        }
    }
    #[doc = "Next Packet Address Pointer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Npktptr(pub u32);
    impl Npktptr {
        #[doc = "Next Packet Address Pointer."]
        #[inline(always)]
        pub const fn npktptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Next Packet Address Pointer."]
        #[inline(always)]
        pub fn set_npktptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Npktptr {
        #[inline(always)]
        fn default() -> Npktptr {
            Npktptr(0)
        }
    }
    #[doc = "Packet buffer size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pktbuf(pub u32);
    impl Pktbuf {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pktbuf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pktbuf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pktbuf {
        #[inline(always)]
        fn default() -> Pktbuf {
            Pktbuf(0)
        }
    }
    #[doc = "packet counter registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pktcnt(pub u32);
    impl Pktcnt {
        #[doc = "The value written to this field is added to the spacket count."]
        #[inline(always)]
        pub const fn cntincr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The value written to this field is added to the spacket count."]
        #[inline(always)]
        pub fn set_cntincr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "This read-only field shows the current (instantaneous) value of the packet counter."]
        #[inline(always)]
        pub const fn cntval(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "This read-only field shows the current (instantaneous) value of the packet counter."]
        #[inline(always)]
        pub fn set_cntval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Pktcnt {
        #[inline(always)]
        fn default() -> Pktcnt {
            Pktcnt(0)
        }
    }
    #[doc = "Packet Control Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pktctl(pub u32);
    impl Pktctl {
        #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet."]
        #[inline(always)]
        pub const fn pktint(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet."]
        #[inline(always)]
        pub fn set_pktint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
        #[inline(always)]
        pub const fn dcrsema(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
        #[inline(always)]
        pub fn set_dcrsema(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
        #[inline(always)]
        pub const fn chain(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
        #[inline(always)]
        pub fn set_chain(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Hash Initialization packat."]
        #[inline(always)]
        pub const fn hasini(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Initialization packat."]
        #[inline(always)]
        pub fn set_hasini(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash Termination packet."]
        #[inline(always)]
        pub const fn hasfnl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash Termination packet."]
        #[inline(always)]
        pub fn set_hasfnl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Load Initial Vector for the AES in this packet."]
        #[inline(always)]
        pub const fn ciphiv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Load Initial Vector for the AES in this packet."]
        #[inline(always)]
        pub fn set_ciphiv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "packet tag."]
        #[inline(always)]
        pub const fn pkttag(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "packet tag."]
        #[inline(always)]
        pub fn set_pkttag(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pktctl {
        #[inline(always)]
        fn default() -> Pktctl {
            Pktctl(0)
        }
    }
    #[doc = "Packet Memory Destination Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pktdst(pub u32);
    impl Pktdst {
        #[doc = "Packet Memory Destination Address."]
        #[inline(always)]
        pub const fn pktdst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Packet Memory Destination Address."]
        #[inline(always)]
        pub fn set_pktdst(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pktdst {
        #[inline(always)]
        fn default() -> Pktdst {
            Pktdst(0)
        }
    }
    #[doc = "Packet Memory Source Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pktsrc(pub u32);
    impl Pktsrc {
        #[doc = "Packet Memory Source Address."]
        #[inline(always)]
        pub const fn pktsrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Packet Memory Source Address."]
        #[inline(always)]
        pub fn set_pktsrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pktsrc {
        #[inline(always)]
        fn default() -> Pktsrc {
            Pktsrc(0)
        }
    }
    #[doc = "SDP control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdpcr(pub u32);
    impl Sdpcr {
        #[doc = "Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
        #[inline(always)]
        pub const fn inten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)."]
        #[inline(always)]
        pub const fn rdscen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)."]
        #[inline(always)]
        pub fn set_rdscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
        #[inline(always)]
        pub const fn tstpkt0irq(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
        #[inline(always)]
        pub fn set_tstpkt0irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Decryption Disable bit, Write to 1 to disable the decryption."]
        #[inline(always)]
        pub const fn dcrpdi(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Decryption Disable bit, Write to 1 to disable the decryption."]
        #[inline(always)]
        pub fn set_dcrpdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
        #[inline(always)]
        pub const fn confen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
        #[inline(always)]
        pub fn set_confen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
        #[inline(always)]
        pub const fn mcpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
        #[inline(always)]
        pub fn set_mcpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
        #[inline(always)]
        pub const fn ciphen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
        #[inline(always)]
        pub fn set_ciphen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip."]
        #[inline(always)]
        pub const fn hasdis(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip."]
        #[inline(always)]
        pub fn set_hasdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip."]
        #[inline(always)]
        pub const fn cipdis(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip."]
        #[inline(always)]
        pub fn set_cipdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
        #[inline(always)]
        pub const fn clkgat(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
        #[inline(always)]
        pub fn set_clkgat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "soft reset. Write 1 then 0, to reset the SDP block."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "soft reset. Write 1 then 0, to reset the SDP block."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sdpcr {
        #[inline(always)]
        fn default() -> Sdpcr {
            Sdpcr(0)
        }
    }
    #[doc = "Status Registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
        #[inline(always)]
        pub const fn errchain(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
        #[inline(always)]
        pub fn set_errchain(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hashing Check Error."]
        #[inline(always)]
        pub const fn errhas(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hashing Check Error."]
        #[inline(always)]
        pub fn set_errhas(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Destination Buffer Error."]
        #[inline(always)]
        pub const fn errdst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Buffer Error."]
        #[inline(always)]
        pub fn set_errdst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Source Buffer Access Error."]
        #[inline(always)]
        pub const fn errsrc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Source Buffer Access Error."]
        #[inline(always)]
        pub fn set_errsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Packet head access error, or status update error."]
        #[inline(always)]
        pub const fn errpkt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Packet head access error, or status update error."]
        #[inline(always)]
        pub fn set_errpkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Working mode setup error."]
        #[inline(always)]
        pub const fn errset(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Working mode setup error."]
        #[inline(always)]
        pub fn set_errset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
        #[inline(always)]
        pub const fn pktdon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
        #[inline(always)]
        pub fn set_pktdon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Packet Counter registers reachs to ZERO now."]
        #[inline(always)]
        pub const fn pktcnt0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Packet Counter registers reachs to ZERO now."]
        #[inline(always)]
        pub fn set_pktcnt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hashing Busy."]
        #[inline(always)]
        pub const fn hasbsy(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Hashing Busy."]
        #[inline(always)]
        pub fn set_hasbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "AES Busy."]
        #[inline(always)]
        pub const fn aesbsy(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AES Busy."]
        #[inline(always)]
        pub fn set_aesbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
        #[inline(always)]
        pub const fn chn1pkt0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
        #[inline(always)]
        pub fn set_chn1pkt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
        #[inline(always)]
        pub fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "packet tag."]
        #[inline(always)]
        pub const fn tag(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "packet tag."]
        #[inline(always)]
        pub fn set_tag(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
}
