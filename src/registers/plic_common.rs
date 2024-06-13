use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Plic",
            extends: None,
            description: Some(
                "PLIC.",
            ),
            items: &[
                BlockItem {
                    name: "feature",
                    description: Some(
                        "Feature enable register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Feature",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "priority",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 127,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Priority",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pending",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
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
                    name: "trigger",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trigger",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "number",
                    description: Some(
                        "Number of supported interrupt sources and targets.",
                    ),
                    array: None,
                    byte_offset: 0x1100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Number",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "info",
                    description: Some(
                        "Version and the maximum priority.",
                    ),
                    array: None,
                    byte_offset: 0x1104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Info",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "targetint",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Targetint",
                        },
                    ),
                },
                BlockItem {
                    name: "targetconfig",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4096,
                            },
                        ),
                    ),
                    byte_offset: 0x200000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Targetconfig",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Targetconfig",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "threshold",
                    description: Some(
                        "Target0 priority threshold.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Threshold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "claim",
                    description: Some(
                        "Target claim and complete.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                BlockItem {
                    name: "pps",
                    description: Some(
                        "Preempted priority stack.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pps",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Targetint",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "inten",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Claim",
            extends: None,
            description: Some(
                "Target claim and complete.",
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
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Feature",
            extends: None,
            description: Some(
                "Feature enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preempt",
                    description: Some(
                        "Preemptive priority interrupt enable 0: Disabled 1: Enabled.",
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
                    name: "vectored",
                    description: Some(
                        "Vector mode enable 0: Disabled 1: Enabled.",
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
        FieldSet {
            name: "Info",
            extends: None,
            description: Some(
                "Version and the maximum priority.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "The version of the PLIC design.",
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
                    name: "max_priority",
                    description: Some(
                        "The maximum priority supported.",
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
            name: "Inten",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt",
                    description: Some(
                        "The interrupt enable bit for interrupt. Every interrupt source occupies 1 bit.",
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
            name: "Number",
            extends: None,
            description: Some(
                "Number of supported interrupt sources and targets.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num_interrupt",
                    description: Some(
                        "The number of supported interrupt sources.",
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
                    name: "num_target",
                    description: Some(
                        "The number of supported targets.",
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
            name: "Pending",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt",
                    description: Some(
                        "The interrupt pending status of inpterrupt sources. Every interrupt source occupies 1 bit.",
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
            name: "Pps",
            extends: None,
            description: Some(
                "Preempted priority stack.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priority_preempted",
                    description: Some(
                        "Each bit indicates if the corresponding priority level has been preempted by a higher-priority interrupt.",
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
            name: "Priority",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "priority",
                    description: Some(
                        "Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority.",
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
            name: "Threshold",
            extends: None,
            description: Some(
                "Target0 priority threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "threshold",
                    description: Some(
                        "Interrupt priority threshold.",
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
            name: "Trigger",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "interrupt",
                    description: Some(
                        "The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt.",
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
