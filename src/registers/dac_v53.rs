use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "DAC0.",
            ),
            items: &[
                BlockItem {
                    name: "cfg0",
                    description: Some(
                        "No description available.",
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
                        "No description available.",
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
                    name: "cfg2",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "step_cfg",
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
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StepCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_addr",
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
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_length",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufLength",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                    name: "irq_en",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_en",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ana_cfg0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AnaCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg0_bak",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                    name: "status0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status0",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AnaCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac12bit_en",
                    description: Some(
                        "No description available.",
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
                    name: "bypass_cali_gm",
                    description: Some(
                        "No description available.",
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
                    name: "cali_delta_v_cfg",
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
                    name: "dac_config",
                    description: Some(
                        "No description available.",
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
                    name: "dac12bit_lp_mode",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BufAddr",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf_stop",
                    description: Some(
                        "set to stop read point at end of bufffer0.",
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
                    name: "buf_start_addr",
                    description: Some(
                        "buffer start address, should be 4-byte aligned AHB burst can't cross 1K-byte boundary, user should config the address/length/burst to avoid such issue.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BufLength",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf0_len",
                    description: Some(
                        "No description available.",
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
                    name: "buf1_len",
                    description: Some(
                        "buffer length, 1 indicate one 32bit date, 256K-byte max for one buffer.",
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
            name: "Cfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hburst_cfg",
                    description: Some(
                        "DAC support following fixed burst only 000-SINGLE; 011-INCR4; 101: INCR8 others are reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "HburstCfg",
                    ),
                },
                Field {
                    name: "buf_data_mode",
                    description: Some(
                        "data structure for buffer mode, 0: each 32-bit data contains 2 points, b11:0 for first, b27:16 for second. 1: each 32-bit data contains 1 point, b11:0 for first.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "BufDataMode",
                    ),
                },
                Field {
                    name: "dac_mode",
                    description: Some(
                        "00: direct mode, DAC output the fixed configured data(from sw_dac_data) 01: step mode, DAC output from start_point to end point, with configured step, can step up or step down 10: buffer mode, read data from buffer, then output to analog, internal DMA will load next burst if enough space in local FIFO; 11: trigger mode, DAC output from external trigger signals Note: Trigger mode is not supported in hpm63xx and hpm62xx families.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "DacMode",
                    ),
                },
                Field {
                    name: "hw_trig_en",
                    description: Some(
                        "set to use trigger signal from trigger_mux, user should config it to pulse in single mode, and level in continual mode.",
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
                    name: "trig_mode",
                    description: Some(
                        "0: single mode, one trigger pulse will send one 12bit data to DAC analog; 1: continual mode, if trigger signal(either or HW) is set, DAC will send data if FIFO is not empty, if trigger signal is clear, DAC will stop send data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "TrigMode",
                    ),
                },
                Field {
                    name: "sync_mode",
                    description: Some(
                        "1: sync dac clock and ahb clock. all HW trigger signals are pulse in sync mode, can get faster response; 0: async dac clock and ahb_clock all HW trigger signals should be level and should be more than one dac clock cycle, used to get accurate output frequency(which may not be divided from AHB clock).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma_ahb_en",
                    description: Some(
                        "set to enable internal DMA, it will read one burst if enough space in FIFO. Should only be used in buffer mode.",
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
                    name: "sw_dac_data",
                    description: Some(
                        "dac data used in direct mode(dac_mode==2'b10).",
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
            name: "Cfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div_cfg",
                    description: Some(
                        "step mode and buffer mode: defines how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Direct mode and trigger mode: defines how many clk_dac cycles to accpet the input data, dac will not accept new written data or trigger data before the clock cycles passed. should configured to less than 1MHz. Note: For direct mode and trigger mode, this config is not supported in hpm63xx and hpm62xx families.",
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
                    name: "ana_div_cfg",
                    description: Some(
                        "clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "AnaDiv",
                    ),
                },
                Field {
                    name: "ana_clk_en",
                    description: Some(
                        "set to enable analog clock(divided by ana_div_cfg) need to be set in direct mode and trigger mode.",
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
            ],
        },
        FieldSet {
            name: "Cfg2",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "step_sw_trig",
                    description: Some(
                        "software trigger0 for step mode, W1C in single mode. RW in continual mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "buf_sw_trig",
                    description: Some(
                        "software trigger for buffer mode, W1C in single mode. RW in continual mode.",
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
                    name: "fifo_clr",
                    description: Some(
                        "set to clear FIFO content(set both read/write pointer to 0).",
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
                    name: "dma_rst0",
                    description: Some(
                        "set to reset dma read pointer to buf0_start_addr.",
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
                    name: "dma_rst1",
                    description: Some(
                        "set to reset dma read pointer to buf1_start_addr; if set both dma_rst0&dma_rst1, will set to buf0_start_addr user can set fifo_clr bit when use dma_rst*.",
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
            name: "DmaEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf0_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "buf1_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "step_cmpt",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "IrqEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf0_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "buf1_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "fifo_empty",
                    description: Some(
                        "No description available.",
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
                    name: "ahb_error",
                    description: Some(
                        "No description available.",
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
                    name: "step_cmpt",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "IrqSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf0_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "buf1_cmpt",
                    description: Some(
                        "No description available.",
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
                    name: "fifo_empty",
                    description: Some(
                        "No description available.",
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
                    name: "ahb_error",
                    description: Some(
                        "set if hresp==2'b01(ERROR).",
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
                    name: "step_cmpt",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "Status0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cur_buf_index",
                    description: Some(
                        "No description available.",
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
                Field {
                    name: "cur_buf_offset",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "StepCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start_point",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "step_num",
                    description: Some(
                        "output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point.",
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
                    name: "end_point",
                    description: Some(
                        "No description available.",
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
                    name: "up_down",
                    description: Some(
                        "0 for up, 1 for down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "StepDir",
                    ),
                },
                Field {
                    name: "round_mode",
                    description: Some(
                        "0: stop at end point; 1: reload start point, step again.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "RoundMode",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "AnaDiv",
            description: Some(
                "No description available.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "BufDataMode",
            description: Some(
                "Format of buffer data.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "TWO_POINTS",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "ONE_POINT",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "DacMode",
            description: Some(
                "No description available.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIRECT",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "STEP",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "BUFFER",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "TRIGGER",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "HburstCfg",
            description: Some(
                "No description available.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "INCR4",
                    description: None,
                    value: 3,
                },
                EnumVariant {
                    name: "INCR8",
                    description: None,
                    value: 5,
                },
            ],
        },
        Enum {
            name: "RoundMode",
            description: Some(
                "No description available.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STOP",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "StepDir",
            description: Some(
                "No description available.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UP",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "TrigMode",
            description: Some(
                "No description available.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SINGLE",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "CONTINUAL",
                    description: None,
                    value: 1,
                },
            ],
        },
    ],
};
