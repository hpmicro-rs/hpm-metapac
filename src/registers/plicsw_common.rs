use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Plicsw",
            extends: None,
            description: Some(
                "PLICSW.",
            ),
            items: &[
                BlockItem {
                    name: "pending",
                    description: Some(
                        "Pending status.",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pending",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "Interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "claim",
                    description: Some(
                        "Claim and complete.",
                    ),
                    array: None,
                    byte_offset: 0x200004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Claim",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Claim",
            extends: None,
            description: Some(
                "Claim and complete.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt_id",
                    description: Some(
                        "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed).",
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
            name: "Inten",
            extends: None,
            description: Some(
                "Interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt",
                    description: Some(
                        "enable software interrupt.",
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
            name: "Pending",
            extends: None,
            description: Some(
                "Pending status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt",
                    description: Some(
                        "writing 1 to trigger software interrupt.",
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
            ],
        },
    ],
    enums: &[],
};
