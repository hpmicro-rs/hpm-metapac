use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Bmon",
            extends: None,
            description: Some(
                "BMON.",
            ),
            items: &[
                BlockItem {
                    name: "monitor",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Monitor",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Monitor",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "control",
                    description: Some(
                        "Glitch and clock monitor control.",
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
                    name: "status",
                    description: Some(
                        "Glitch and clock monitor status.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
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
                "Glitch and clock monitor control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "enable glitch detector 0: detector disabled 1: detector enabled.",
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
                        "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain.",
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
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "Glitch and clock monitor status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flag",
                    description: Some(
                        "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected.",
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
    ],
    enums: &[],
};
