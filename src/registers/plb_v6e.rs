use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Plb",
            extends: None,
            description: Some(
                "PLB.",
            ),
            items: &[
                BlockItem {
                    name: "type_a",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "TypeA",
                        },
                    ),
                },
                BlockItem {
                    name: "type_b",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x400,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "TypeB",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TypeA",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "lookup_table",
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
                                "LookupTable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_inject",
                    description: Some(
                        "TYPE A CHN&index0 software inject.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TypeASwInject",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TypeB",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "lut",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
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
                                "Lut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp",
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
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mode",
                    description: Some(
                        "TYPE B CHN&index0 mode ctrl.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_inject",
                    description: Some(
                        "TYPE B CHN&index0 software inject.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TypeBSwInject",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmp",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_value",
                    description: Some(
                        "cmp value, using as data unit operation.",
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
            name: "LookupTable",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lookup_table",
                    description: Some(
                        "using 4 bit trig_in as lookup index. software can program this register as trig_in's true table.",
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
            ],
        },
        FieldSet {
            name: "Lut",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lookup_table",
                    description: Some(
                        "lut0 and lut1 union as 64bit, consider each 4bit as one slice. then, total 16 slice. slice0 as bit3:0, slice1 as bit7:4...etc. using 4bit trig in as index of slice. the operate sel in data unit of type B channle is decided by which slice value choosed by trig_in.",
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
            name: "Mode",
            extends: None,
            description: Some(
                "TYPE B CHN&index0 mode ctrl.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_sel",
                    description: Some(
                        "trig out 0 output type in current channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "out1_sel",
                    description: Some(
                        "trig out 1 output type in current channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "out2_sel",
                    description: Some(
                        "trig out 2 output type in current channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "out3_sel",
                    description: Some(
                        "trig out 3 output type in current channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opt_sel",
                    description: Some(
                        "operation selection in data unit.",
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
            name: "TypeASwInject",
            extends: None,
            description: Some(
                "TYPE A CHN&index0 software inject.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw_inject",
                    description: Some(
                        "software can inject value to TYPEA's output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TypeBSwInject",
            extends: None,
            description: Some(
                "TYPE B CHN&index0 software inject.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "software_inject",
                    description: Some(
                        "data unit value can be changed if program this register.",
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
