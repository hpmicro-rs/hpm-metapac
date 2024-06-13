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
                                len: 456,
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
                        "alt select 0: ALT0 1: ALT1 ... 31:ALT31.",
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
                        "drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm.",
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
                    name: "spd",
                    description: Some(
                        "additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz).",
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
                    name: "sr",
                    description: Some(
                        "slew rate 0: Slow slew rate 1: Fast slew rate.",
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
                    name: "od",
                    description: Some(
                        "open drain 0: open drain disable 1: open drain enable.",
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
                    name: "ke",
                    description: Some(
                        "keeper capability enable 0: keeper disable 1: keeper enable.",
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
                    name: "pe",
                    description: Some(
                        "pull enable 0: pull disable 1: pull enable.",
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
                    name: "ps",
                    description: Some(
                        "pull select 0: pull down 1: pull up.",
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
                    name: "prs",
                    description: Some(
                        "select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hys",
                    description: Some(
                        "schmitt trigger enable 0: disable 1: enable.",
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
    ],
    enums: &[],
};
