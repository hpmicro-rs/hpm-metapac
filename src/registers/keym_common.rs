use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Keym",
            extends: None,
            description: Some(
                "KEYM.",
            ),
            items: &[
                BlockItem {
                    name: "softmkey",
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
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Softmkey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "softpkey",
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
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Softpkey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sec_key_ctl",
                    description: Some(
                        "secure key generation.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecKeyCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nsc_key_ctl",
                    description: Some(
                        "non-secure key generation.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "NscKeyCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rng",
                    description: Some(
                        "Random number interface behavior.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rng",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "read_control",
                    description: Some(
                        "key read out control.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ReadControl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "NscKeyCtl",
            extends: None,
            description: Some(
                "non-secure key generation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key_sel",
                    description: Some(
                        "non-secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmk_sel",
                    description: Some(
                        "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key.",
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
                    name: "zmk_sel",
                    description: Some(
                        "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smk_sel",
                    description: Some(
                        "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sk_val",
                    description: Some(
                        "session key valid 0: session key is all 0's and not usable 1: session key is valid.",
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
                    name: "lock_nsc_ctl",
                    description: Some(
                        "block non-secure state key setting being changed.",
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
            name: "ReadControl",
            extends: None,
            description: Some(
                "key read out control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "block_smk_read",
                    description: Some(
                        "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out.",
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
                    name: "block_pk_read",
                    description: Some(
                        "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out.",
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
            ],
        },
        FieldSet {
            name: "Rng",
            extends: None,
            description: Some(
                "Random number interface behavior.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rng_xor",
                    description: Some(
                        "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG.",
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
                    name: "block_rng_xor",
                    description: Some(
                        "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software.",
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
            ],
        },
        FieldSet {
            name: "SecKeyCtl",
            extends: None,
            description: Some(
                "secure key generation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key_sel",
                    description: Some(
                        "secure symmtric key synthesize setting, key is a XOR of following bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmk_sel",
                    description: Some(
                        "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key.",
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
                    name: "zmk_sel",
                    description: Some(
                        "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smk_sel",
                    description: Some(
                        "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sk_val",
                    description: Some(
                        "session key valid 0: session key is all 0's and not usable 1: session key is valid.",
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
                    name: "lock_sec_ctl",
                    description: Some(
                        "block secure state key setting being changed.",
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
            name: "Softmkey",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0.",
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
            name: "Softpkey",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0.",
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
    ],
    enums: &[],
};
