use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Synt",
        extends: None,
        description: Some("SYNT."),
        items: &[
            BlockItem {
                name: "gcr",
                description: Some("Global control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Gcr"),
                }),
            },
            BlockItem {
                name: "rld",
                description: Some("Counter reload register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rld"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("Counter."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cnt"),
                }),
            },
            BlockItem {
                name: "cmp",
                description: Some("no description available."),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 0x20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cmp"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cmp",
            extends: None,
            description: Some("no description available."),
            bit_size: 32,
            fields: &[Field {
                name: "cmp",
                description: Some(
                    "comparator value, the output will assert when counter count to this value.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some("Counter."),
            bit_size: 32,
            fields: &[Field {
                name: "cnt",
                description: Some("counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some("Global control register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cen",
                    description: Some("1- Enable counter."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crst",
                    description: Some("1- Reset counter."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rld",
            extends: None,
            description: Some("Counter reload register."),
            bit_size: 32,
            fields: &[Field {
                name: "rld",
                description: Some("counter reload value."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
