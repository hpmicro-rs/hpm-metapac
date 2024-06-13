use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tamp",
            extends: None,
            description: Some(
                "TAMP.",
            ),
            items: &[
                BlockItem {
                    name: "tamp",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "TampTamp",
                        },
                    ),
                },
                BlockItem {
                    name: "tamp_flag",
                    description: Some(
                        "Tamper flag.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TampFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en",
                    description: Some(
                        "Tamper interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEn",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TampTamp",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "control",
                    description: Some(
                        "Tamper n control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Control",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "poly",
                    description: Some(
                        "Tamper n Polynomial of LFSR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Poly",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lfsr",
                    description: Some(
                        "Tamper n LFSR shift register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lfsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Control",
            extends: None,
            description: Some(
                "Tamper n control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "enable tamper 0: tamper disableed 1: tamper enabled.",
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
                    name: "active",
                    description: Some(
                        "select active or passive tamper 0: passive tamper 1: active tamper.",
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
                    name: "recover",
                    description: Some(
                        "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover.",
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
                    name: "speed",
                    description: Some(
                        "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "value",
                    description: Some(
                        "pin value for passive tamper.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filter",
                    description: Some(
                        "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bypass",
                    description: Some(
                        "bypass tamper violation filter 0: filter applied 1: filter not used.",
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
                    name: "lock",
                    description: Some(
                        "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle.",
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
            name: "IrqEn",
            extends: None,
            description: Some(
                "Tamper interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en",
                    description: Some(
                        "interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock",
                    description: Some(
                        "lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle.",
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
            name: "Lfsr",
            extends: None,
            description: Some(
                "Tamper n LFSR shift register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lfsr",
                    description: Some(
                        "LFSR for active tamper, write only register, always read 0.",
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
            name: "Poly",
            extends: None,
            description: Some(
                "Tamper n Polynomial of LFSR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poly",
                    description: Some(
                        "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\".",
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
            name: "TampFlag",
            extends: None,
            description: Some(
                "Tamper flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flag",
                    description: Some(
                        "tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disappeared.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
