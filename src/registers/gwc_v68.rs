use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Channel",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "cfg0",
                    description: Some(
                        "config reg 0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg1",
                    description: Some(
                        "config reg 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "refcrc",
                    description: Some(
                        "reference CRC.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Refcrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calcrc",
                    description: Some(
                        "calculated CRC.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calcrc",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Gwc",
            extends: None,
            description: Some(
                "GWC0.",
            ),
            items: &[
                BlockItem {
                    name: "glb_ctrl",
                    description: Some(
                        "control reg.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GlbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_mask",
                    description: Some(
                        "interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqMask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts",
                    description: Some(
                        "interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "channel",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 240,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Channel",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Calcrc",
            extends: None,
            description: Some(
                "calculated CRC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal_crc",
                    description: Some(
                        "calculated CRC for last frame.",
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
            name: "Cfg0",
            extends: None,
            description: Some(
                "config reg 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start_col",
                    description: Some(
                        "define the window start column number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "start_row",
                    description: Some(
                        "define the window start row number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "freeze",
                    description: Some(
                        "freeze config. set to freeze all other config registers for current channel. can only be cleared by system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "channel enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfg1",
            extends: None,
            description: Some(
                "config reg 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "end_col",
                    description: Some(
                        "define the window end column number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "end_row",
                    description: Some(
                        "define the window end row number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GlbCtrl",
            extends: None,
            description: Some(
                "control reg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gwc_en",
                    description: Some(
                        "graphic window check enable. set to enable the whole block.",
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
                    name: "clk_pol",
                    description: Some(
                        "graphic clock polarity. set to invert input graphic clock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IrqMask",
            extends: None,
            description: Some(
                "interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_mask",
                    description: Some(
                        "error interrupt mask.",
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
                    name: "func_mask",
                    description: Some(
                        "function interrupt mask.",
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
                Field {
                    name: "mask_rreez",
                    description: Some(
                        "freeze mask, set to disable changing ERR_MASK and FUNC_MASK. can only be cleared by system reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IrqSts",
            extends: None,
            description: Some(
                "interrupt status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gwc_fail_sts",
                    description: Some(
                        "graphic window check fail interrupt status. will be set if the calculated CRC not equal reference CRC. one bit for each channel. software write 1 to clear.",
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
                    name: "err_sts",
                    description: Some(
                        "error status, it's OR of GWC_FAIL_STS[15:0].",
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
                    name: "func_sts",
                    description: Some(
                        "function interrupt status. it's set when detect two VSYNC signals after the block is enabled(GWC_EN is set) software write 1 to clear.",
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
            ],
        },
        FieldSet {
            name: "Refcrc",
            extends: None,
            description: Some(
                "reference CRC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ref_crc",
                    description: Some(
                        "reference CRC polynomial function: 0x104C11DB7.",
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
