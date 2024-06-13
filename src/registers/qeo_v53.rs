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
                    name: "wave_vd_vq_inject",
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
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WaveVdVqInject",
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
                    byte_offset: 0x20,
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
                    byte_offset: 0x24,
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
                    byte_offset: 0x30,
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
                    name: "wave_limit",
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
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "WaveLimit",
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
                    byte_offset: 0x54,
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
                    name: "abz_mode",
                    description: Some(
                        "wave_a/b/z output mode.",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                    byte_offset: 0x64,
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
                    byte_offset: 0x68,
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
                    byte_offset: 0x74,
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
                    byte_offset: 0x78,
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
                    byte_offset: 0x7c,
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
                    name: "pwm_mode",
                    description: Some(
                        "pwm mode.",
                    ),
                    array: None,
                    byte_offset: 0x80,
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
                    byte_offset: 0x84,
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
                    byte_offset: 0x88,
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
                    byte_offset: 0x98,
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
                    byte_offset: 0xf8,
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
                    byte_offset: 0xfc,
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
                    byte_offset: 0x100,
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
                    byte_offset: 0x104,
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
                    byte_offset: 0x108,
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
                    byte_offset: 0x10c,
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
                    byte_offset: 0x110,
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
            ],
        },
        Block {
            name: "WaveLimit",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "min",
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
                                "Min",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max",
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
                                "Max",
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
                    enumm: None,
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
                    enumm: None,
                },
                Field {
                    name: "z_type",
                    description: Some(
                        "wave_z type: 0: zero pulse and output high at both wave_a and wave_b are high. mantain about 25% period. 1: zero pulse output high about 75% period. start from 0 to 75% period. 2: zero pulse output high about 100% period. 3: wave_z output as tree-phase wave same as wave_a/wave_b.",
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
                    enumm: None,
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
                    enumm: None,
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
                    enumm: None,
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
                        "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period.",
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
            name: "Max",
            extends: None,
            description: Some(
                "wave0 high area limit value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit0",
                    description: Some(
                        "high area limit level0.",
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
                    name: "limit1",
                    description: Some(
                        "high area limit level1.",
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
            name: "Min",
            extends: None,
            description: Some(
                "wave0 low area limit value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit0",
                    description: Some(
                        "low area limit level0.",
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
                    name: "limit1",
                    description: Some(
                        "low area limit level1.",
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
            name: "PwmDebug0",
            extends: None,
            description: Some(
                "qeo debug 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wave0",
                    description: Some(
                        "wave0 observe.",
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
                    name: "wave1",
                    description: Some(
                        "wave1 observe.",
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
            name: "PwmDebug1",
            extends: None,
            description: Some(
                "qeo debug 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wave2",
                    description: Some(
                        "wave2 observe.",
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
                    name: "wave_a",
                    description: Some(
                        "wave_a observe.",
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
                    name: "wave_b",
                    description: Some(
                        "wave_b observe.",
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
                    name: "wave_z",
                    description: Some(
                        "wave_z observe.",
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
                    name: "pwm0_safety",
                    description: Some(
                        "PWM safety mode phase table.",
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
                    name: "pwm1_safety",
                    description: Some(
                        "PWM safety mode phase table.",
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
                Field {
                    name: "pwm2_safety",
                    description: Some(
                        "PWM safety mode phase table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm3_safety",
                    description: Some(
                        "PWM safety mode phase table.",
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
                    name: "pwm4_safety",
                    description: Some(
                        "PWM safety mode phase table.",
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
                    name: "pwm5_safety",
                    description: Some(
                        "PWM safety mode phase table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm6_safety",
                    description: Some(
                        "PWM safety mode phase table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm7_safety",
                    description: Some(
                        "PWM safety mode phase table.",
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
                        "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period.",
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
            name: "PwmPhaseTable",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm0",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm1",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm2",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm3",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm4",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm5",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm6",
                    description: Some(
                        "pwm phase table value.",
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
                    name: "pwm7",
                    description: Some(
                        "pwm phase table value.",
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
                    bit_size: 16,
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
                    enumm: None,
                },
                Field {
                    name: "en_wave0_vd_vq_inject",
                    description: Some(
                        "wave0 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject.",
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
                    name: "en_wave1_vd_vq_inject",
                    description: Some(
                        "wave1 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject.",
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
                    name: "en_wave2_vd_vq_inject",
                    description: Some(
                        "wave2 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject.",
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
                    enumm: None,
                },
                Field {
                    name: "wave0_below_min_limit",
                    description: Some(
                        "wave0 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit0.level1_min_limit.",
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
                        "wave0 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit.",
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
                        "wave0 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit.",
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
                        "wave0 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit.",
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
                        "wave1 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit1.level1_min_limit.",
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
                        "wave1 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit.",
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
                        "wave1 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit.",
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
                        "wave1 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit.",
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
                        "wave2 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit2.level1_min_limit.",
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
                        "wave2 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit.",
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
                        "wave2 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit.",
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
                        "wave2 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit.",
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
                        "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period.",
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
            name: "WaveVdVqInject",
            extends: None,
            description: Some(
                "no description available.",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vq_val",
                    description: Some(
                        "Vq inject value.",
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
    ],
    enums: &[],
};
