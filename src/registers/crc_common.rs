use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Chn",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "pre_set",
                    description: Some(
                        "&index0 pre set for crc setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PreSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clr",
                    description: Some(
                        "chn&index0 clear crc result and setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "poly",
                    description: Some(
                        "chn&index0 poly.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "init_data",
                    description: Some(
                        "chn&index0 init_data.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InitData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xorout",
                    description: Some(
                        "chn&index0 xorout.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xorout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misc_setting",
                    description: Some(
                        "chn&index0 misc_setting.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MiscSetting",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "chn&index0 data.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "result",
                    description: Some(
                        "chn&index0 result.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Result",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Crc",
            extends: None,
            description: Some(
                "CRC.",
            ),
            items: &[
                BlockItem {
                    name: "chn",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Chn",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Clr",
            extends: None,
            description: Some(
                "chn&index0 clear crc result and setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clr",
                    description: Some(
                        "write 1 to clr crc setting and result for its channel. always read 0.",
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
            ],
        },
        FieldSet {
            name: "Data",
            extends: None,
            description: Some(
                "chn&index0 data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "data for crc.",
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
            name: "InitData",
            extends: None,
            description: Some(
                "chn&index0 init_data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "init_data",
                    description: Some(
                        "initial data of CRC.",
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
            name: "MiscSetting",
            extends: None,
            description: Some(
                "chn&index0 misc_setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poly_width",
                    description: Some(
                        "crc data length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rev_in",
                    description: Some(
                        "0: no wrap input bit order 1: wrap input bit order.",
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
                    name: "rev_out",
                    description: Some(
                        "0: no wrap output bit order 1: wrap output bit order.",
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
                    name: "byte_rev",
                    description: Some(
                        "0: no wrap input byte order 1: wrap input byte order.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Poly",
            extends: None,
            description: Some(
                "chn&index0 poly.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "poly",
                    description: Some(
                        "poly setting.",
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
            name: "PreSet",
            extends: None,
            description: Some(
                "&index0 pre set for crc setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pre_set",
                    description: Some(
                        "0: no pre set 1: CRC32 2: CRC32-AUTOSAR 3: CRC16-CCITT 4: CRC16-XMODEM 5: CRC16-MODBUS 1: CRC32 2: CRC32-autosar 3: CRC16-ccitt 4: CRC16-xmodem 5: CRC16-modbus 6: crc16_dnp 7: crc16_x25 8: crc16_usb 9: crc16_maxim 10: crc16_ibm 11: crc8_maxim 12: crc8_rohc 13: crc8_itu 14: crc8 15: crc5_usb.",
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
            ],
        },
        FieldSet {
            name: "Result",
            extends: None,
            description: Some(
                "chn&index0 result.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "result",
                    description: Some(
                        "crc result.",
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
            name: "Xorout",
            extends: None,
            description: Some(
                "chn&index0 xorout.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xorout",
                    description: Some(
                        "XOR for CRC result.",
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
