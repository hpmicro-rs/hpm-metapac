use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Mchtmr",
        extends: None,
        description: Some("MCHTMR."),
        items: &[
            BlockItem {
                name: "mtime",
                description: Some("Machine Time."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 64,
                    fieldset: Some("Mtime"),
                }),
            },
            BlockItem {
                name: "mtimecmp",
                description: Some("Machine Time Compare."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 64,
                    fieldset: Some("Mtimecmp"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Mtime",
            extends: None,
            description: Some("Machine Time."),
            bit_size: 64,
            fields: &[Field {
                name: "mtime",
                description: Some("Machine time."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 64,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Mtimecmp",
            extends: None,
            description: Some("Machine Time Compare."),
            bit_size: 64,
            fields: &[Field {
                name: "mtimecmp",
                description: Some("Machine time compare."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 64,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
