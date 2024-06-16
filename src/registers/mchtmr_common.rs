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
                    fieldset: None,
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
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
