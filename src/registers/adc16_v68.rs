use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "ADC0.",
            ),
            items: &[
                BlockItem {
                    name: "config",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
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
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_dma_addr",
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
                                "TrgDmaAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_sw_sta",
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
                                "TrgSwSta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bus_result",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
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
                                "BusResult",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_cfg0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_cfg0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_dma_addr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqDmaAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_wr_addr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqWrAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_dma_cfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x80c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqDmaCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_que",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x810,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqQue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seq_high_cfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x850,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SeqHighCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prd_cfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0xc00,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "PrdCfg",
                        },
                    ),
                },
                BlockItem {
                    name: "sample_cfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
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
                                "SampleCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "conv_cfg1",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ConvCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_cfg0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ana_ctrl0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AnaCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ana_status",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AnaStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc16_params",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 34,
                                stride: 2,
                            },
                        ),
                    ),
                    byte_offset: 0x1400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Adc16Params",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc16_config0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1444,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc16Config0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc16_config1",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1460,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc16Config1",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "PrdCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "prd_cfg",
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
                                "PrdCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prd_thshd_cfg",
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
                                "PrdThshdCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "prd_result",
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
                                "PrdResult",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adc16Config0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "conv_param",
                    description: Some(
                        "conversion parameter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "preempt_en",
                    description: Some(
                        "set to enable preemption feature.",
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
                    name: "cal_avg_cfg",
                    description: Some(
                        "for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bandgap_en",
                    description: Some(
                        "set to enable bandgap. user should set reg_en and bandgap_en before use adc16.",
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
                    name: "reg_en",
                    description: Some(
                        "set to enable regulator.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Adc16Config1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cov_end_cnt",
                    description: Some(
                        "used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number+1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Adc16Params",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "param_val",
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
            ],
        },
        FieldSet {
            name: "AdcCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port3_realtime",
                    description: Some(
                        "set to enable trg queue stop other queues.",
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
                    name: "adc_ahb_en",
                    description: Some(
                        "set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sel_sync_ahb",
                    description: Some(
                        "set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode.",
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
            name: "AnaCtrl0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "startcal",
                    description: Some(
                        "set to start the offset calibration cycle (Active H). user need to clear it after setting it.",
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
                    name: "adc_clk_on",
                    description: Some(
                        "set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access.",
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
            ],
        },
        FieldSet {
            name: "AnaStatus",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "calon",
                    description: Some(
                        "Indicates if the ADC is in calibration mode (Active H).",
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
            name: "BufCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wait_dis",
                    description: Some(
                        "set to disable read waiting, get result immediately but maybe not current conversion result.",
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
            name: "BusResult",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chan_result",
                    description: Some(
                        "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long.",
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
                    name: "valid",
                    description: Some(
                        "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again.",
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
            name: "Config",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chan0",
                    description: Some(
                        "channel number for 1st conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inten0",
                    description: Some(
                        "interrupt enable for 1st conversion.",
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
                    name: "queue_en",
                    description: Some(
                        "preemption queue enable control.",
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
                    name: "chan1",
                    description: Some(
                        "channel number for 2nd conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inten1",
                    description: Some(
                        "interrupt enable for 2nd conversion.",
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
                    name: "chan2",
                    description: Some(
                        "channel number for 3rd conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inten2",
                    description: Some(
                        "interrupt enable for 3rd conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chan3",
                    description: Some(
                        "channel number for 4th conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inten3",
                    description: Some(
                        "interrupt enable for 4th conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trig_len",
                    description: Some(
                        "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ConvCfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clock_divider",
                    description: Some(
                        "clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3, ... 15 for 1:16 Note: set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk.",
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
                    name: "convert_clock_number",
                    description: Some(
                        "convert clock numbers, set to 21 (0x15) for 16bit mode, which means convert need 21 adc clock cycles(based on clock after divider); user can use small value to get faster conversion, but less accuracy, need to config cov_end_cnt at adc16_config1 also. Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 21 for 16bit mode, clock_divder to 3, then each ADC conversion(plus sample) need 25 cycles(50MHz).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdog",
                    description: Some(
                        "set if one chanel watch dog event triggered.",
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
                    name: "ahb_err",
                    description: Some(
                        "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma_fifo_full",
                    description: Some(
                        "DMA fifo full interrupt, user need to check clock frequency if it's set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_cvc",
                    description: Some(
                        "one conversion complete in seq_queue if related seq_int_en is set.",
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
                    name: "seq_cmpt",
                    description: Some(
                        "the whole sequence complete interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_dmaabt",
                    description: Some(
                        "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_hw_cflct",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_sw_cflct",
                    description: Some(
                        "sequence queue conflict interrupt, set if HW or SW trigger received during conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "read_cflct",
                    description: Some(
                        "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trig_hw_cflct",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trig_sw_cflct",
                    description: Some(
                        "No description available.",
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
                    name: "trig_cmpt",
                    description: Some(
                        "interrupt for one trigger conversion complete if enabled.",
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
            name: "IntSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdog",
                    description: Some(
                        "set if one chanel watch dog event triggered.",
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
                    name: "ahb_err",
                    description: Some(
                        "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma_fifo_full",
                    description: Some(
                        "DMA fifo full interrupt, user need to check clock frequency if it's set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_cvc",
                    description: Some(
                        "one conversion complete in seq_queue if related seq_int_en is set.",
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
                    name: "seq_cmpt",
                    description: Some(
                        "the whole sequence complete interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_dmaabt",
                    description: Some(
                        "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_hw_cflct",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_sw_cflct",
                    description: Some(
                        "sequence queue conflict interrupt, set if HW or SW trigger received during conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "read_cflct",
                    description: Some(
                        "read conflict interrupt, set if wait_dis is set, one conversion is in progress, SW read another channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trig_hw_cflct",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trig_sw_cflct",
                    description: Some(
                        "No description available.",
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
                    name: "trig_cmpt",
                    description: Some(
                        "interrupt for one trigger conversion complete if enabled.",
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
            name: "PrdCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prd",
                    description: Some(
                        "conver period, with prescale. Set to 0 means disable current channel.",
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
                    name: "prescale",
                    description: Some(
                        "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PrdResult",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chan_result",
                    description: Some(
                        "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel.",
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
            name: "PrdThshdCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thshdl",
                    description: Some(
                        "threshold low.",
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
                    name: "thshdh",
                    description: Some(
                        "threshold high, assert interrupt(if enabled) if result exceed high or low.",
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
            name: "SampleCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sample_clock_number",
                    description: Some(
                        "sample clock number, base on clock_period, default one period.",
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
                    name: "sample_clock_number_shift",
                    description: Some(
                        "shift for sample clock number.",
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
            ],
        },
        FieldSet {
            name: "SeqCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hw_trig_en",
                    description: Some(
                        "set to enable external HW trigger, only trigger on posedge.",
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
                    name: "sw_trig_en",
                    description: Some(
                        "set to enable SW trigger.",
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
                    name: "sw_trig",
                    description: Some(
                        "SW trigger, pulse signal, cleared by HW one cycle later.",
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
                    name: "cont_en",
                    description: Some(
                        "if set, HW will continue process the queue till end(seq_len) after trigger once.",
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
                    name: "restart_en",
                    description: Some(
                        "if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used.",
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
                    name: "seq_len",
                    description: Some(
                        "sequence queue length, 0 for one, 0xF for 16.",
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
                    name: "cycle",
                    description: Some(
                        "current dma write cycle bit.",
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
            name: "SeqDmaAddr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tar_addr",
                    description: Some(
                        "dma target address, should be 4-byte aligned.",
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
            name: "SeqDmaCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf_len",
                    description: Some(
                        "dma buffer length, after write to (tar_addr[31:2]+buf_len)*4, the next dma address will be tar_addr[31:2]*4 0 for 4byte; 0xFFF for 16kbyte.",
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
                    name: "stop_en",
                    description: Some(
                        "set to stop dma if reach the stop_pos.",
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
                    name: "dma_rst",
                    description: Some(
                        "set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst.",
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
                    name: "stop_pos",
                    description: Some(
                        "if stop_en is set, SW is responsible to update this field to the next read point, HW should not write data to this point since it's not read out by SW yet.",
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
            name: "SeqHighCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "buf_len_high",
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
                    name: "stop_pos_high",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SeqQue",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chan_num_4_0",
                    description: Some(
                        "channel number for current conversion.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seq_int_en",
                    description: Some(
                        "interrupt enable for current conversion.",
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
            ],
        },
        FieldSet {
            name: "SeqWrAddr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seq_wr_pointer",
                    description: Some(
                        "HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TrgDmaAddr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg_dma_addr",
                    description: Some(
                        "buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion).",
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
            name: "TrgSwSta",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trig_sw_index",
                    description: Some(
                        "which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c.",
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
                    name: "trg_sw_sta",
                    description: Some(
                        "SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it.",
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
    ],
    enums: &[],
};
