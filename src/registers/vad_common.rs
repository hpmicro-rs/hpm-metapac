use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Vad",
            extends: None,
            description: Some(
                "VAD.",
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
                    name: "filtctrl",
                    description: Some(
                        "Filter Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filtctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dec_ctrl0",
                    description: Some(
                        "Decision Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DecCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dec_ctrl1",
                    description: Some(
                        "Decision Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DecCtrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dec_ctrl2",
                    description: Some(
                        "Decision Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DecCtrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "Status.",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "ofifo",
                    description: Some(
                        "Out FIFO.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "run",
                    description: Some(
                        "Run Command Register.",
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
                BlockItem {
                    name: "ofifo_ctrl",
                    description: Some(
                        "Out FIFO Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OfifoCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cic_cfg",
                    description: Some(
                        "CIC Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                    name: "coef",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 1,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Coef",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CicCfg",
            extends: None,
            description: Some(
                "CIC Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
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
            name: "Coef",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "The current detected short time energy.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chnum",
                    description: Some(
                        "the number of channels to be stored in buffer. Asserted to enable 2 channels.",
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
                    name: "ch_pol",
                    description: Some(
                        "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdm_clk_oe",
                    description: Some(
                        "pdm_clk_output_en.",
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
                Field {
                    name: "pdm_clk_div_bypass",
                    description: Some(
                        "asserted to bypass the pdm clock divider.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fifo_thrsh",
                    description: Some(
                        "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "membuf_disable",
                    description: Some(
                        "asserted to disable membuf.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cic_sat_err_ie",
                    description: Some(
                        "CIC saturation Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cic_ovld_err_ie",
                    description: Some(
                        "CIC overload Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iir_ovfl_err_ie",
                    description: Some(
                        "IIR overflow error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iir_ovld_err_ie",
                    description: Some(
                        "IIR overload error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ofifo_ovfl_err_ie",
                    description: Some(
                        "OFIFO overflow error interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "membuf_empty_ie",
                    description: Some(
                        "Buf empty interrupt enable.",
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
                    name: "ofifo_av_ie",
                    description: Some(
                        "OFIFO data available interrupt enable.",
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
                    name: "vad_ie",
                    description: Some(
                        "VAD event interrupt enable.",
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
                    name: "pdm_clk_hfdiv",
                    description: Some(
                        "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
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
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DecCtrl0",
            extends: None,
            description: Some(
                "Decision Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subblk_len",
                    description: Some(
                        "length of sub-block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blk_cfg",
                    description: Some(
                        "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "noise_tol",
                    description: Some(
                        "the value of amplitude for noise determination when calculationg ZCR.",
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
            name: "DecCtrl1",
            extends: None,
            description: Some(
                "Decision Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcr_low",
                    description: Some(
                        "ZCR low limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcr_high",
                    description: Some(
                        "ZCR high limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DecCtrl2",
            extends: None,
            description: Some(
                "Decision Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_low",
                    description: Some(
                        "amplitude low limit.",
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
                    name: "amp_high",
                    description: Some(
                        "amplitude high limit.",
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
            name: "Filtctrl",
            extends: None,
            description: Some(
                "Filter Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iir_slot_en",
                    description: Some(
                        "IIR slot enable.",
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
                    name: "decratio",
                    description: Some(
                        "the decimation ratio of iir after CIC -1 2: means dec-by-3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofifo",
            extends: None,
            description: Some(
                "Out FIFO.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "d",
                    description: Some(
                        "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit [31:16]: the samples earlier in time ([T-1]). Bit [15:0]: the samples later in time ([T]). When there is two channels, the samples in the 32-bits are: bit [31:16]: the samples belong to Ch 1 (when ch_pol[1:0]==2, the data is captured at the positive part of the pdm clk). bit [15:0]: the samples belong to Ch 0 (when ch_pol[1:0]==2, the data is captured at the negtive part of the pdm clk).",
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
            name: "OfifoCtrl",
            extends: None,
            description: Some(
                "Out FIFO Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Asserted to enable OFIFO.",
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
            name: "Run",
            extends: None,
            description: Some(
                "Run Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vad_en",
                    description: Some(
                        "module enable.",
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
                    name: "sftrst",
                    description: Some(
                        "software reset. Self-clear.",
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
            name: "St",
            extends: None,
            description: Some(
                "Status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cic_sat_err",
                    description: Some(
                        "CIC saturation.",
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
                        "CIC overload.",
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
                    name: "iir_ovfl",
                    description: Some(
                        "IIR oberflow.",
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
                    name: "iir_ovld",
                    description: Some(
                        "IIR overloading.",
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
                Field {
                    name: "ofifo_ovfl",
                    description: Some(
                        "OFIFO overflow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "membuf_empty",
                    description: Some(
                        "Buf empty.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ofifo_av",
                    description: Some(
                        "OFIFO data available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vad",
                    description: Some(
                        "VAD event found.",
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
    ],
    enums: &[],
};
