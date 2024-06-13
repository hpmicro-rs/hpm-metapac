use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Trgm",
            extends: None,
            description: Some(
                "TRGM0.",
            ),
            items: &[
                BlockItem {
                    name: "filtcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 28,
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
                                "Filtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgocfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 137,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trgocfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "General Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_matrix_sel",
                    description: Some(
                        "adc matrix select register.",
                    ),
                    array: None,
                    byte_offset: 0x510,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcMatrixSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac_matrix_sel",
                    description: Some(
                        "dac matrix select register.",
                    ),
                    array: None,
                    byte_offset: 0x514,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DacMatrixSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_matrix_sel0",
                    description: Some(
                        "position matrix select register0.",
                    ),
                    array: None,
                    byte_offset: 0x518,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosMatrixSel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_matrix_sel1",
                    description: Some(
                        "position matrix select register1.",
                    ),
                    array: None,
                    byte_offset: 0x51c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosMatrixSel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgm_in",
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
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgmIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgm_out",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x620,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgmOut",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AdcMatrixSel",
            extends: None,
            description: Some(
                "adc matrix select register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qei0_adc0_sel",
                    description: Some(
                        "No description available.",
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
                    name: "qei0_adc1_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "qei1_adc0_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "qei1_adc1_sel",
                    description: Some(
                        "0-adc0; 1-adc1; 2-rdc_adc0; 3-rdc_adc1; bit7 is used to invert adc_value; others reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DacMatrixSel",
            extends: None,
            description: Some(
                "dac matrix select register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acmp0_dac_sel",
                    description: Some(
                        "No description available.",
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
                    name: "acmp1_dac_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "dac0_dac_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "dac1_dac_sel",
                    description: Some(
                        "0-qeo0_dac0; 1-qeo0_dac1; 2-qeo0_dac2; 3-qeo1_dac0; 4-qeo1_dac1; 5-qeo1_dac2; 6-rdc_dac0; 7-rdc_dac1; bit7 is used to invert dac_value; others reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmacfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmasrcsel",
                    description: Some(
                        "This field selects one of the DMA requests as the DMA request output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmamux_en",
                    description: Some(
                        "No description available.",
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
            name: "Filtcfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "filtlen_base",
                    description: Some(
                        "This bitfields defines the filter counter length.",
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
                    name: "filtlen_shift",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncen",
                    description: Some(
                        "set to enable sychronization input signal with TRGM clock.",
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
                    name: "mode",
                    description: Some(
                        "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "outinv",
                    description: Some(
                        "1- Filter will invert the output 0- Filter will not invert the output.",
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
            name: "Gcr",
            extends: None,
            description: Some(
                "General Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgopen",
                    description: Some(
                        "The bitfield enable the TRGM outputs.",
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
            ],
        },
        FieldSet {
            name: "PosMatrixSel0",
            extends: None,
            description: Some(
                "position matrix select register0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sei_posin0_sel",
                    description: Some(
                        "No description available.",
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
                    name: "sei_posin1_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "mmc0_posin_sel",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "mmc1_posin_sel",
                    description: Some(
                        "0-sei_pos_out0; 1-sei_pos_out1; 2-qei0_pos; 3-qei1_pos; 4-mmc0_pos_out0; 5-mmc0_pos_out1; 6-mmc1_pos_out0; 7-mmc1_pos_out1; bit7 is used to invert position value; others reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PosMatrixSel1",
            extends: None,
            description: Some(
                "position matrix select register1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qeo0_pos_sel",
                    description: Some(
                        "No description available.",
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
                    name: "qeo1_pos_sel",
                    description: Some(
                        "No description available.",
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
            name: "TrgmIn",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgm_in",
                    description: Some(
                        "mmc1_trig_out[1:0], mmc0_trig_out[1:0],sync_pulse[3:0],moto_gpio_in_sync[7:0],//31:16 gtmr3_to_motor_sync[1:0],gtmr2_to_motor_sync[1:0],gtmr1_to_motor_sync[1:0],gtmr0_to_motor_sync[1:0], //15:8 acmp_out_sync[1:0],can2mot_event_sync[1:0],usb0_sof_tog_sync,pwm_debug,1'b1,1'b0 //7:0.",
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
            name: "TrgmOut",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgm_out",
                    description: Some(
                        "motor_to_opamp0[7:0] = trig_mux_out[7:0]; motor_to_opamp1[7:0] = trig_mux_out[15:8]; motor_to_gtmr0_capt[1:0] = trig_mux_out[17:16]; motor_to_gtmr0_sync = trig_mux_out[18]; motor_to_gtmr1_capt[1:0] = trig_mux_out[20:19]; motor_to_gtmr1_sync = trig_mux_out[21]; motor_to_gtmr2_capt[1:0] = trig_mux_out[23:22]; motor_to_gtmr2_sync = trig_mux_out[24]; motor_to_gtmr3_capt[1:0] = trig_mux_out[26:25]; motor_to_gtmr3_sync = trig_mux_out[27]; acmp_window[1:0] = trig_mux_out[29:28]; dac0_buf_trigger = trig_mux_out[30]; dac1_buf_trigger = trig_mux_out[31]; dac0_step_trigger[3:0] = {trig_mux_out[24:22],trig_mux_out[30]};//use same buf_trig, and gtmr2 dac1_step_trigger[3:0] = {trig_mux_out[27:25],trig_mux_out[31]}; //use same buf_trig, and gtmr3.",
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
            name: "Trgocfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigosel",
                    description: Some(
                        "This bitfield selects one of the TRGM inputs as output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "redg2pen",
                    description: Some(
                        "1- The selected input signal rising edge will be convert to an pulse on output.",
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
                    name: "fedg2pen",
                    description: Some(
                        "1- The selected input signal falling edge will be convert to an pulse on output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "outinv",
                    description: Some(
                        "1- Invert the output.",
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
            ],
        },
    ],
    enums: &[],
};
