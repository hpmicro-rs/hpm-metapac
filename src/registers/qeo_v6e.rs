use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Qeo",
            extends: None,
            description: Some(
                "QEO0.",
            ),
            items: &[
                BlockItem {
                    name: "wave_mode",
                    description: Some(
                        "analog waves mode.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_resolution",
                    description: Some(
                        "resolution of wave0/1/2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveResolution",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_phase_shift",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
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
                                "WavePhaseShift",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_vd_inject",
                    description: Some(
                        "wave vd inject value.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveVdInject",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_vq_inject",
                    description: Some(
                        "wave vq inject value.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveVqInject",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_vd_vq_load",
                    description: Some(
                        "load wave0/1/2 vd vq value.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveVdVqLoad",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_amplitude",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveAmplitude",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_mid_point",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveMidPoint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_limit0",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x48,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "WaveLimit0",
                        },
                    ),
                },
                BlockItem {
                    name: "wave_limit1",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x60,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "WaveLimit1",
                        },
                    ),
                },
                BlockItem {
                    name: "wave_deadzone_shift",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveDeadzoneShift",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wave_pwm_cycle",
                    description: Some(
                        "pwm_cycle.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WavePwmCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_mode",
                    description: Some(
                        "wave_a/b/z output mode.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_resolution",
                    description: Some(
                        "resolution of wave_a/b/z.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzResolution",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_phase_shift",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzPhaseShift",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_line_width",
                    description: Some(
                        "Two-phase orthogonality wave 1/4 period.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzLineWidth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_wdog_width",
                    description: Some(
                        "wdog width of qeo.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzWdogWidth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_postion_sync",
                    description: Some(
                        "sync abz owned postion.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzPostionSync",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_overall_offset",
                    description: Some(
                        "abz overall position offset.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzOverallOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_z_start",
                    description: Some(
                        "zero phase start line num.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzZStart",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_z_end",
                    description: Some(
                        "zero phase end line num.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzZEnd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_z_offset",
                    description: Some(
                        "zero phase start and end 1/4 line num.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzZOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "abz_z_pulse_width",
                    description: Some(
                        "zero pulse witdth.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbzZPulseWidth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_mode",
                    description: Some(
                        "pwm mode.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_resolution",
                    description: Some(
                        "resolution of pwm.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmResolution",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_phase_shift",
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
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmPhaseShift",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_phase_table",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmPhaseTable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_postion_software",
                    description: Some(
                        "softwave inject postion.",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmPostionSoftware",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_postion_sel",
                    description: Some(
                        "select softwave inject postion.",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmPostionSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_status",
                    description: Some(
                        "qeo status.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug0",
                    description: Some(
                        "qeo debug 0.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug1",
                    description: Some(
                        "qeo debug 1.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug2",
                    description: Some(
                        "qeo debug 2.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug3",
                    description: Some(
                        "qeo debug 3.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug4",
                    description: Some(
                        "qeo debug 4.",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_debug5",
                    description: Some(
                        "qeo debug 5.",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDebug5",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "WaveLimit0",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "min_level0",
                    description: Some(
                        "wave0 low area limit value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MinLevel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_level0",
                    description: Some(
                        "wave0 high area limit value.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MaxLevel0",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "WaveLimit1",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "min_level1",
                    description: Some(
                        "wave0 low area limit value level1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MinLevel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_level1",
                    description: Some(
                        "wave0 high area limit value level1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MaxLevel1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AbzLineWidth",
            extends: None,
            description: Some(
                "Two-phase orthogonality wave 1/4 period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "line",
                    description: Some(
                        "the num of system clk by 1/4 period when using as Two-phase orthogonality.",
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
            name: "AbzMode",
            extends: None,
            description: Some(
                "wave_a/b/z output mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "a_type",
                    description: Some(
                        "wave_a type: 0: Two-phase orthogonality wave_a. 1: pulse wave of pulse/reverse type. 2: up wave of up/down type. 3: Three-phase orthogonality wave_a.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "AWaveType",
                    ),
                },
                Field {
                    name: "b_type",
                    description: Some(
                        "wave_b type: 0: Two-phase orthogonality wave_b. 1: reverse wave of pulse/reverse type. 2: down wave of up/down type. 3: Three-phase orthogonality wave_b.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "BWaveType",
                    ),
                },
                Field {
                    name: "z_type",
                    description: Some(
                        "wave_z type: 0: zero pulse type, start and end line number decided by z_start、z_end and z_offset. 1: zero pulse type, z output start to high when position= z_start, and mantain numbers of 1/4 line cfg in z_pulse_width register 2: reserved 3: wave_z output as tree-phase wave same as wave_a/wave_b.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ZWaveType",
                    ),
                },
                Field {
                    name: "a_polarity",
                    description: Some(
                        "wave_a polarity. 0: normal output. 1: invert normal output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WavePolarity",
                    ),
                },
                Field {
                    name: "b_polarity",
                    description: Some(
                        "wave_b polarity. 0: normal output. 1: invert normal output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WavePolarity",
                    ),
                },
                Field {
                    name: "z_polarity",
                    description: Some(
                        "wave_z polarity. 0: normal output. 1: invert normal output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "WavePolarity",
                    ),
                },
                Field {
                    name: "en_wdog",
                    description: Some(
                        "enable abz wdog: 0: disable abz wdog. 1: enable abz wdog.",
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
                    name: "position_sync_mode",
                    description: Some(
                        "position sync mode: 0: only sync integer line part into qeo own position. 1: sync integer and fraction part into qeo own position.",
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
                    name: "reverse_edge_type",
                    description: Some(
                        "pulse reverse wave，reverse edge point: 0: between pulse's posedge and negedge, min period dedicated by the num line_width 1: edge change point flow pulse's negedge.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ReverseEdgeType",
                    ),
                },
                Field {
                    name: "abz_output_enable",
                    description: Some(
                        "abz output enable： 0：abz output disable, all keep 0 1：abz output enable.",
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
            name: "AbzOverallOffset",
            extends: None,
            description: Some(
                "abz overall position offset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "abz position overall offset, it affects abz position before resolution convert.",
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
            name: "AbzPhaseShift",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period.",
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
            name: "AbzPostionSync",
            extends: None,
            description: Some(
                "sync abz owned postion.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "postion",
                    description: Some(
                        "load next valid postion into abz owned postion. always read 0 0: sync abz owned postion with next valid postion. 1: not sync.",
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
            name: "AbzResolution",
            extends: None,
            description: Some(
                "resolution of wave_a/b/z.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lines",
                    description: Some(
                        "wave_a/b/z resolution.",
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
            name: "AbzWdogWidth",
            extends: None,
            description: Some(
                "wdog width of qeo.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "width",
                    description: Some(
                        "wave will step 1/4 line to reminder user QEO still in controlled if QEO has no any toggle after the num of wdog_width sys clk.",
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
            name: "AbzZEnd",
            extends: None,
            description: Some(
                "zero phase end line num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "z_end",
                    description: Some(
                        "number of Z end line.",
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
            name: "AbzZOffset",
            extends: None,
            description: Some(
                "zero phase start and end 1/4 line num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "z_start_offset",
                    description: Some(
                        "number of Z start 1/4 line.",
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
                    name: "z_end_offset",
                    description: Some(
                        "number of Z end 1/4 line.",
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
            ],
        },
        FieldSet {
            name: "AbzZPulseWidth",
            extends: None,
            description: Some(
                "zero pulse witdth.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "number of z_pulse_width.",
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
            name: "AbzZStart",
            extends: None,
            description: Some(
                "zero phase start line num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "z_start",
                    description: Some(
                        "number of Z start line.",
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
            name: "MaxLevel0",
            extends: None,
            description: Some(
                "wave0 high area limit value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit_level0",
                    description: Some(
                        "high area limit level0.",
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
            name: "MaxLevel1",
            extends: None,
            description: Some(
                "wave0 high area limit value level1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit_level1",
                    description: Some(
                        "high area limit level1.",
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
            name: "MinLevel0",
            extends: None,
            description: Some(
                "wave0 low area limit value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit_level0",
                    description: Some(
                        "low area limit level0.",
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
            name: "MinLevel1",
            extends: None,
            description: Some(
                "wave0 low area limit value level1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit_level1",
                    description: Some(
                        "low area limit level1.",
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
            name: "PwmDebug0",
            extends: None,
            description: Some(
                "qeo debug 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_dac0",
                    description: Some(
                        "wave0.",
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
            name: "PwmDebug1",
            extends: None,
            description: Some(
                "qeo debug 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pad_a",
                    description: Some(
                        "pad_a observe.",
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
                    name: "pad_b",
                    description: Some(
                        "pad_b observe.",
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
                    name: "pad_z",
                    description: Some(
                        "pad_z observe.",
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
                    name: "qeo_finish",
                    description: Some(
                        "qeo finish observe.",
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
            ],
        },
        FieldSet {
            name: "PwmDebug2",
            extends: None,
            description: Some(
                "qeo debug 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "abz_own_postion",
                    description: Some(
                        "abz_own_postion observe.",
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
            name: "PwmDebug3",
            extends: None,
            description: Some(
                "qeo debug 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "abz_own_postion",
                    description: Some(
                        "abz_own_postion observe.",
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
            name: "PwmDebug4",
            extends: None,
            description: Some(
                "qeo debug 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_dac1",
                    description: Some(
                        "wave1.",
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
            name: "PwmDebug5",
            extends: None,
            description: Some(
                "qeo debug 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_dac2",
                    description: Some(
                        "wave2.",
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
            name: "PwmMode",
            extends: None,
            description: Some(
                "pwm mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phase_num",
                    description: Some(
                        "pwm force phase number.",
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
                    name: "revise_up_dn",
                    description: Some(
                        "exchange PWM pairs’ output 0: not exchange. 1: exchange.",
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
                    name: "pwm_safety_bypass",
                    description: Some(
                        "PWM safety mode bypass 0: not bypass 1: bypass.",
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
                    name: "pwm_enter_safety_mode",
                    description: Some(
                        "PWM enter safety mode 0: not enter 1: enter.",
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
                    name: "enable_pwm",
                    description: Some(
                        "enable PWM force output 0: disable 1: enable.",
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
                    name: "pwm_safety",
                    description: Some(
                        "PWM safety mode phase table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "PwmMode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "PwmPhaseShift",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period.",
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
            name: "PwmPhaseTable",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm",
                    description: Some(
                        "pwm phase table value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "PwmMode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "PwmPostionSel",
            extends: None,
            description: Some(
                "select softwave inject postion.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "postion_sel",
                    description: Some(
                        "enable softwave inject postion. 0: disable. 1: enable.",
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
            name: "PwmPostionSoftware",
            extends: None,
            description: Some(
                "softwave inject postion.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "postion_softwave",
                    description: Some(
                        "softwave inject postion.",
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
            name: "PwmResolution",
            extends: None,
            description: Some(
                "resolution of pwm.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lines",
                    description: Some(
                        "pwm resolution.",
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
            name: "PwmStatus",
            extends: None,
            description: Some(
                "qeo status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm_safety",
                    description: Some(
                        "pwm_fault status.",
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
                    name: "pwm_fource",
                    description: Some(
                        "qeo_pwm_force observe.",
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
            name: "WaveAmplitude",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_val",
                    description: Some(
                        "amplitude scaling value. bit15-12 are integer part value. bit11-0 are fraction value.",
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
                    name: "en_scal",
                    description: Some(
                        "enable wave amplitude scaling. 0: disable; 1: enable.",
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
            name: "WaveDeadzoneShift",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "wave0 deadzone shifter value.",
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
            name: "WaveMidPoint",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "wave0 output middle point, use this value as 32 bit signed value. bit 31 is signed bit. bit30-27 is integer part value. bit26-0 is fraction value.",
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
            name: "WaveMode",
            extends: None,
            description: Some(
                "analog waves mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "waves_output_type",
                    description: Some(
                        "wave0/1/2 output mode. 0: cosine wave. 1: saddle wave. 2. abs cosine wave. 3. saw wave.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WavesOutputType",
                    ),
                },
                Field {
                    name: "vd_vq_sel",
                    description: Some(
                        "vd_vq sel ctrl: 0: from CLC. 1: from software.",
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
                    name: "en_wave_vd_vq_inject",
                    description: Some(
                        "wave VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject.",
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
                    name: "enable_pos_valid",
                    description: Some(
                        "enable position valid to trigger analog wave calcuation 0: disable. 1: enable.",
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
                    name: "enable_dq_valid",
                    description: Some(
                        "enable vd or vq valid to trigger analog wave calcuation 0: disable. 1: enable.",
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
                    name: "saddle_type",
                    description: Some(
                        "saddle type seclect; 0:standard saddle. 1: triple-cos saddle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SaddleType",
                    ),
                },
                Field {
                    name: "wave0_below_min_limit",
                    description: Some(
                        "wave0 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit0.level1_min_limit.",
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
                    name: "wave0_low_area0_limit",
                    description: Some(
                        "wave0 low area0 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit.",
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
                    name: "wave0_low_area1_limit",
                    description: Some(
                        "wave0 low area1 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit.",
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
                    name: "wave0_high_area0_limit",
                    description: Some(
                        "wave0 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit.",
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
                    name: "wave0_high_area1_limit",
                    description: Some(
                        "wave0 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit.",
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
                    name: "wave0_above_max_limit",
                    description: Some(
                        "wave0 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit.",
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
                    name: "wave1_below_min_limit",
                    description: Some(
                        "wave1 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit1.level1_min_limit.",
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
                    name: "wave1_low_area0_limit",
                    description: Some(
                        "wave1 low area0 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit.",
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
                    name: "wave1_low_area1_limit",
                    description: Some(
                        "wave1 low area1 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit.",
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
                    name: "wave1_high_area0_limit",
                    description: Some(
                        "wave1 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit.",
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
                    name: "wave1_high_area1_limit",
                    description: Some(
                        "wave1 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit.",
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
                    name: "wave1_above_max_limit",
                    description: Some(
                        "wave1 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wave2_below_min_limit",
                    description: Some(
                        "wave2 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit2.level1_min_limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wave2_low_area0_limit",
                    description: Some(
                        "wave2 low area0 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit.",
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
                    name: "wave2_low_area1_limit",
                    description: Some(
                        "wave2 low area1 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit.",
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
                    name: "wave2_high_area0_limit",
                    description: Some(
                        "wave2 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit.",
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
                    name: "wave2_high_area1_limit",
                    description: Some(
                        "wave2 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit.",
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
                    name: "wave2_above_max_limit",
                    description: Some(
                        "wave2 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit.",
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
            name: "WavePhaseShift",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period.",
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
            name: "WavePwmCycle",
            extends: None,
            description: Some(
                "pwm_cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "pwm_cycle.",
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
            name: "WaveResolution",
            extends: None,
            description: Some(
                "resolution of wave0/1/2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lines",
                    description: Some(
                        "wave0/1/2 resolution.",
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
            name: "WaveVdInject",
            extends: None,
            description: Some(
                "wave vd inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vd_val",
                    description: Some(
                        "Vd inject value.",
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
            name: "WaveVdVqLoad",
            extends: None,
            description: Some(
                "load wave0/1/2 vd vq value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "load",
                    description: Some(
                        "load wave0/1/2 vd vq value. always read 0 0: vd vq keep previous value. 1: load wave0/1/2 vd vq value at sametime.",
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
            name: "WaveVqInject",
            extends: None,
            description: Some(
                "wave vq inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vq_val",
                    description: Some(
                        "Vq inject value.",
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
    enums: &[
        Enum {
            name: "AWaveType",
            description: Some(
                "wave_a type.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TWO_PHASE",
                    description: Some(
                        "Two-phase orthogonality wave_a",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PULSE",
                    description: Some(
                        "pulse wave of pulse/reverse type",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UP",
                    description: Some(
                        "up wave of up/down type",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "THREE_PHASE",
                    description: Some(
                        "Three-phase orthogonality wave_a",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "BWaveType",
            description: Some(
                "wave_b type.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TWO_PHASE",
                    description: Some(
                        "Two-phase orthogonality wave_b",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REVERSE",
                    description: Some(
                        "reverse wave of pulse/reverse type",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "down wave of up/down type",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "THREE_PHASE",
                    description: Some(
                        "Three-phase orthogonality wave_b",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PwmMode",
            description: Some(
                "PWM safety mode phase table.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "normal output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FORCE_0",
                    description: Some(
                        "force output 0",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "FORCE_1",
                    description: Some(
                        "force output 1",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ReverseEdgeType",
            description: Some(
                "pulse reverse wave，reverse edge point.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BETWEEN_POS_NEG",
                    description: Some(
                        "between pulse's posedge and negedge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "EDGE_CHANGE_POINT",
                    description: Some(
                        "edge change point flow pulse's negedge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "SaddleType",
            description: Some(
                "saddle type seclect.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "STANDARD",
                    description: Some(
                        "standard saddle",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "TRIPLE_COS",
                    description: Some(
                        "triple-cos saddle",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WavePolarity",
            description: Some(
                "wave polarity.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "normal output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INVERT",
                    description: Some(
                        "invert normal output",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WavesOutputType",
            description: Some(
                "wave0/1/2 output mode.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "COSINE",
                    description: Some(
                        "cosine wave",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SADDLE",
                    description: Some(
                        "saddle wave",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ABS_COSINE",
                    description: Some(
                        "abs cosine wave",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SAW",
                    description: Some(
                        "saw wave",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ZWaveType",
            description: Some(
                "wave_z type.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ZERO_PULSE_HIGH_25",
                    description: Some(
                        "zero pulse and output high at both wave_a and wave_b are high. mantain about 25% period",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ZERO_PULSE_HIGH_75",
                    description: Some(
                        "zero pulse output high about 75% period. start from 0 to 75% period",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ZERO_PULSE_HIGH_100",
                    description: Some(
                        "zero pulse output high about 100% period",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "THREE_PHASE",
                    description: Some(
                        "wave_z output as tree-phase wave same as wave_a/wave_b",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
