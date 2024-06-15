use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Xpi",
        extends: None,
        description: Some("Placeholder for XPI device."),
        items: &[],
    }],
    fieldsets: &[],
    enums: &[],
};
