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
                    name: "ch_cfg",
                    description: Some(
                        "Channel Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChCfg",
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
                    name: "ctrl_inbuf",
                    description: Some(
                        "In Buf Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CtrlInbuf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_filt0",
                    description: Some(
                        "Filter 0 Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CtrlFilt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_filt1",
                    description: Some(
                        "Filter 1 Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CtrlFilt1",
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
                BlockItem {
                    name: "memaddr",
                    description: Some(
                        "Memory Access Address.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Memaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "memdata",
                    description: Some(
                        "Memory Access Data.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Memdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpf_ma",
                    description: Some(
                        "HPF A Coef Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpfMa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpf_b",
                    description: Some(
                        "HPF B Coef Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpfB",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChCfg",
            extends: None,
            description: Some(
                "Channel Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch0_type",
                    description: Some(
                        "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch1_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch2_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch3_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch4_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch5_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch6_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch7_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch8_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch9_type",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
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
                    name: "hpf_en",
                    description: Some(
                        "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data.",
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
                    name: "dec_aft_cic",
                    description: Some(
                        "decimation rate after CIC. Now it is forced to be 3.",
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
                    name: "filt_crx_err_ie",
                    description: Some(
                        "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "use_coef_ram",
                    description: Some(
                        "Asserted to use Coef RAM instead of Coef ROM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
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
            name: "CtrlFilt0",
            extends: None,
            description: Some(
                "Filter 0 Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef_start_addr",
                    description: Some(
                        "Starting address of Coef of filter type 2'b00 in coef memory.",
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
                    name: "coef_len_m0",
                    description: Some(
                        "Coef length of filter type 2'b00 in coef memory.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CtrlFilt1",
            extends: None,
            description: Some(
                "Filter 1 Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef_start_addr",
                    description: Some(
                        "Starting address of Coef of filter type 2'b01 in coef memory.",
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
                    name: "coef_len_m1",
                    description: Some(
                        "Coef length of filter type 2'b01 in coef memory.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CtrlInbuf",
            extends: None,
            description: Some(
                "In Buf Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start_addr",
                    description: Some(
                        "The starting address of channel 0 in filter data buffer.",
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
                    name: "pitch",
                    description: Some(
                        "The spacing between starting address of adjacent channels.",
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
                Field {
                    name: "max_ptr",
                    description: Some(
                        "The buf size-1 for each channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpfB",
            extends: None,
            description: Some(
                "HPF B Coef Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef",
                    description: Some(
                        "coef B of the Order-1 HPF.",
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
            name: "HpfMa",
            extends: None,
            description: Some(
                "HPF A Coef Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef",
                    description: Some(
                        "Composite value of coef A of the Order-1 HPF.",
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
            name: "Memaddr",
            extends: None,
            description: Some(
                "Memory Access Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM.",
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
            name: "Memdata",
            extends: None,
            description: Some(
                "Memory Access Data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "The data write-to/read-from buffer.",
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
                Field {
                    name: "filt_crx_err",
                    description: Some(
                        "data accessed out of boundary error.",
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
                    name: "_7",
                    description: Some(
                        "7.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_6",
                    description: Some(
                        "6.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_5",
                    description: Some(
                        "5.",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
