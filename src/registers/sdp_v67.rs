use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Sdp",
            extends: None,
            description: Some(
                "SDP.",
            ),
            items: &[
                BlockItem {
                    name: "sdpcr",
                    description: Some(
                        "SDP control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "modctrl",
                    description: Some(
                        "Mod control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Modctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pktcnt",
                    description: Some(
                        "packet counter registers.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pktcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sta",
                    description: Some(
                        "Status Registers.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keyaddr",
                    description: Some(
                        "Key Address.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Keyaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "keydat",
                    description: Some(
                        "Key Data.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Keydat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ciphiv",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ciphiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "haswrd",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Haswrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmdptr",
                    description: Some(
                        "Command Pointer.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmdptr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "npktptr",
                    description: Some(
                        "Next Packet Address Pointer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Npktptr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pktctl",
                    description: Some(
                        "Packet Control Registers.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pktctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pktsrc",
                    description: Some(
                        "Packet Memory Source Address.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pktsrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pktdst",
                    description: Some(
                        "Packet Memory Destination Address.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pktdst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pktbuf",
                    description: Some(
                        "Packet buffer size.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pktbuf",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ciphiv",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ciphiv",
                    description: Some(
                        "cipher initialization vector.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cmdptr",
            extends: None,
            description: Some(
                "Command Pointer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdptr",
                    description: Some(
                        "current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Haswrd",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "haswrd",
                    description: Some(
                        "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result[31:0] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Keyaddr",
            extends: None,
            description: Some(
                "Key Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subwrd",
                    description: Some(
                        "Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "index",
                    description: Some(
                        "To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-[number_keys]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Keydat",
            extends: None,
            description: Some(
                "Key Data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "keydat",
                    description: Some(
                        "This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Modctrl",
            extends: None,
            description: Some(
                "Mod control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "keyswp",
                    description: Some(
                        "Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "doutswp",
                    description: Some(
                        "Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dinswp",
                    description: Some(
                        "Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasout",
                    description: Some(
                        "When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "haschk",
                    description: Some(
                        "HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasalg",
                    description: Some(
                        "HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesdir",
                    description: Some(
                        "AES direction 1x1, AES Decryption 1x0, AES Encryption.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesks",
                    description: Some(
                        "AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0[127:0] from the key manager for AES128; AES256 will use kman_sk0[255:0] as AES key. 0x21: kman_sk0[255:128] from the key manager for AES128; not valid for AES256. 0x22: kman_sk1[127:0] from the key manager for AES128; AES256 will use kman_sk1[255:0] as AES key. 0x23: kman_sk1[255:128] from the key manager for AES128; not valid for AES256. 0x24: kman_sk2[127:0] from the key manager for AES128; AES256 will use kman_sk2[255:0] as AES key. 0x25: kman_sk2[255:128] from the key manager for AES128; not valid for AES256. 0x26: kman_sk3[127:0] from the key manager for AES128; AES256 will use kman_sk3[255:0] as AES key. 0x27: kman_sk3[255:128] from the key manager for AES128; not valid for AES256. 0x30: exip0_key[127:0] from OTP for AES128; AES256 will use exip0_key[255:0] as AES key. 0x31: exip0_key[255:128] from OTP for AES128; not valid for AES256. 0x32: exip1_key[127:0] from OTP for AES128; AES256 will use exip1_key[255:0] as AES key. 0x33: exip1_key[255:128] from OTP for AES128; not valid for AES256. Other values, reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesmod",
                    description: Some(
                        "AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesalg",
                    description: Some(
                        "AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; 0x8 = SM4； Others, reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Npktptr",
            extends: None,
            description: Some(
                "Next Packet Address Pointer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "npktptr",
                    description: Some(
                        "Next Packet Address Pointer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pktbuf",
            extends: None,
            description: Some(
                "Packet buffer size.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pktbuf",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pktcnt",
            extends: None,
            description: Some(
                "packet counter registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntincr",
                    description: Some(
                        "The value written to this field is added to the spacket count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cntval",
                    description: Some(
                        "This read-only field shows the current (instantaneous) value of the packet counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pktctl",
            extends: None,
            description: Some(
                "Packet Control Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pktint",
                    description: Some(
                        "Reflects whether the channel must issue an interrupt upon the completion of the packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrsema",
                    description: Some(
                        "whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chain",
                    description: Some(
                        "whether the next command pointer register must be loaded into the channel's current descriptor pointer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasini",
                    description: Some(
                        "Hash Initialization packat.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasfnl",
                    description: Some(
                        "Hash Termination packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ciphiv",
                    description: Some(
                        "Load Initial Vector for the AES in this packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pkttag",
                    description: Some(
                        "packet tag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pktdst",
            extends: None,
            description: Some(
                "Packet Memory Destination Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pktdst",
                    description: Some(
                        "Packet Memory Destination Address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pktsrc",
            extends: None,
            description: Some(
                "Packet Memory Source Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pktsrc",
                    description: Some(
                        "Packet Memory Source Address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sdpcr",
            extends: None,
            description: Some(
                "SDP control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tstpkt0irq",
                    description: Some(
                        "Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrpdi",
                    description: Some(
                        "Decryption Disable bit, Write to 1 to disable the decryption.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "confen",
                    description: Some(
                        "Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcpen",
                    description: Some(
                        "Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashen",
                    description: Some(
                        "HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ciphen",
                    description: Some(
                        "Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasdis",
                    description: Some(
                        "HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cipdis",
                    description: Some(
                        "Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clkgat",
                    description: Some(
                        "Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrst",
                    description: Some(
                        "soft reset. Write 1 then 0, to reset the SDP block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sta",
            extends: None,
            description: Some(
                "Status Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errchain",
                    description: Some(
                        "buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errhas",
                    description: Some(
                        "Hashing Check Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errdst",
                    description: Some(
                        "Destination Buffer Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errsrc",
                    description: Some(
                        "Source Buffer Access Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errpkt",
                    description: Some(
                        "Packet head access error, or status update error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errset",
                    description: Some(
                        "Working mode setup error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktdon",
                    description: Some(
                        "Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pktcnt0",
                    description: Some(
                        "Packet Counter registers reachs to ZERO now.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hasbsy",
                    description: Some(
                        "Hashing Busy.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesbsy",
                    description: Some(
                        "AES Busy.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chn1pkt0",
                    description: Some(
                        "the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irq",
                    description: Some(
                        "interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tag",
                    description: Some(
                        "packet tag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
