use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ioc",
            extends: None,
            description: Some(
                "IOC.",
            ),
            items: &[
                BlockItem {
                    name: "pad",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 492,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pad",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pad",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "func_ctl",
                    description: Some(
                        "ALT SELECT.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FuncCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pad_ctl",
                    description: Some(
                        "PAD SETTINGS.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PadCtl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "FuncCtl",
            extends: None,
            description: Some(
                "ALT SELECT.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alt_select",
                    description: Some(
                        "alt select 0: ALT0 1: ALT1 … 31:ALT31.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "analog",
                    description: Some(
                        "select analog pin in pad 0: disable 1: enable.",
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
                    name: "loop_back",
                    description: Some(
                        "force input on 0: disable 1: enable.",
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
            name: "PadCtl",
            extends: None,
            description: Some(
                "PAD SETTINGS.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ds",
                    description: Some(
                        "drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA.",
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
                    name: "pe",
                    description: Some(
                        "pull enable 0: pull disable 1: pull enable.",
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
                    name: "ps",
                    description: Some(
                        "pull select 0: pull down 1: pull up.",
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
                    name: "smt",
                    description: Some(
                        "schmitt trigger enable, only available in high-speed IO 0: disable 1: enable.",
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
                    name: "od",
                    description: Some(
                        "open drain 0: open drain disable 1: open drain enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ms",
                    description: Some(
                        "pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
