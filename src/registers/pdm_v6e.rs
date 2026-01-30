use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pdm",
            extends: None,
            description: Some(
                "PDM.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch_ctrl",
                    description: Some(
                        "Channel Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "St",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cic_cfg",
                    description: Some(
                        "CIC configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CicCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "run",
                    description: Some(
                        "Run Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Run",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChCtrl",
            extends: None,
            description: Some(
                "Channel Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch_en",
                    description: Some(
                        "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics.",
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
                Field {
                    name: "ch_pol",
                    description: Some(
                        "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CicCfg",
            extends: None,
            description: Some(
                "CIC configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cic_dec_ratio",
                    description: Some(
                        "CIC decimation factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgd",
                    description: Some(
                        "Sigma_delta_order[1:0] 2'b00: 7 2'b01: 6 2'b10: 5 Others: unused.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "SigmaDeltaOrder",
                    ),
                },
                Field {
                    name: "post_scale",
                    description: Some(
                        "the shift value after CIC results.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdm_clk_oe",
                    description: Some(
                        "pdm_clk_output_en.",
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
                    name: "pdm_clk_div_bypass",
                    description: Some(
                        "asserted to bypass the pdm clock divider.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdm_clk_hfdiv",
                    description: Some(
                        "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "capt_dly",
                    description: Some(
                        "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cic_sat_err_ie",
                    description: Some(
                        "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled.",
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
                    name: "cic_ovld_err_ie",
                    description: Some(
                        "CIC overload error interrupt enable.",
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
                Field {
                    name: "ofifo_ovfl_err_ie",
                    description: Some(
                        "output fifo overflow error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sof_fedge",
                    description: Some(
                        "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrst",
                    description: Some(
                        "software reset the module. Self-clear.",
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
            name: "Run",
            extends: None,
            description: Some(
                "Run Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdm_en",
                    description: Some(
                        "Asserted to enable the module.",
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
            name: "St",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cic_sat_err",
                    description: Some(
                        "CIC saturation. Write 1 clear.",
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
                    name: "cic_ovld_err",
                    description: Some(
                        "CIC overload error. write 1 clear.",
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
                    name: "ofifo_ovfl_err",
                    description: Some(
                        "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "SigmaDeltaOrder",
            description: Some(
                "Sigma delta order.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_5",
                    description: Some(
                        "5.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "_6",
                    description: Some(
                        "6.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_7",
                    description: Some(
                        "7.",
                    ),
                    value: 0,
                },
            ],
        },
    ],
};
