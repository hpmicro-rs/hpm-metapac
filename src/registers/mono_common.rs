use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mono",
            extends: None,
            description: Some(
                "MONO.",
            ),
            items: &[
                BlockItem {
                    name: "monol",
                    description: Some(
                        "Low part of monotonic counter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Monol",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monoh",
                    description: Some(
                        "High part of monotonic counter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Monoh",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Monoh",
            extends: None,
            description: Some(
                "High part of monotonic counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epoch",
                    description: Some(
                        "Fuse value for high part of monotonica.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Monol",
            extends: None,
            description: Some(
                "Low part of monotonic counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "low part of monotonica counter, write to this counter will cause counter increase by 1.",
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
