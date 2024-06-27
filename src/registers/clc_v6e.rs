use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Clc",
            extends: None,
            description: Some(
                "CLC0.",
            ),
            items: &[
                BlockItem {
                    name: "vdvq_chan",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "VdvqChan",
                        },
                    ),
                },
                BlockItem {
                    name: "dq_adc_sw_ready",
                    description: Some(
                        "enable d/q chan software inject adc value.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DqAdcSwReady",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Coeff",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "coeff_b0",
                    description: Some(
                        "&index0 zone &index1 b0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffB0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_b1",
                    description: Some(
                        "&index0 zone &index1 b1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffB1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_b2",
                    description: Some(
                        "&index0 zone &index1 b2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffB2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_b3",
                    description: Some(
                        "&index0 zone &index1 b3.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffB3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_a0",
                    description: Some(
                        "&index0 zone &index1 a0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffA0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_a1",
                    description: Some(
                        "&index0 zone &index1 a1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffA1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_a2",
                    description: Some(
                        "&index0 zone &index1 a2.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffA2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff_ks",
                    description: Some(
                        "&index0 zone &index1 kscaling.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CoeffKs",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "VdvqChan",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "mode",
                    description: Some(
                        "&index0 mode ctrl.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "adc_expect",
                    description: Some(
                        "&index0 adc expect.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcExpect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_chan",
                    description: Some(
                        "&index0 adc used channel.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcChan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_offset",
                    description: Some(
                        "&index0 adc used offset.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_lowth",
                    description: Some(
                        "&index0 eadc_lowth value used in error adc cofficient selection.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcLowth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_highth",
                    description: Some(
                        "&index0 eadc_highth value used in error adc cofficient selection.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcHighth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_midlowth",
                    description: Some(
                        "&index0 eadc_midlowth value used in error adc cofficient selection.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcMidlowth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_midhighth",
                    description: Some(
                        "&index0 eadc_midhighth value used in error adc cofficient selection.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcMidhighth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2z2_clamp_lo",
                    description: Some(
                        "&index0 2p2z output clamp low threshold.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2z2ClampLo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2z2_clamp_hi",
                    description: Some(
                        "&index0 2p2z output clamp high threshold.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2z2ClampHi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_clamp_lo",
                    description: Some(
                        "&index0 3p3z output clamp low threshold.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3ClampLo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_clamp_hi",
                    description: Some(
                        "&index0 3p3z output clamp high threshold.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3ClampHi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coeff",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Coeff",
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_period",
                    description: Some(
                        "&index0 pwm_period.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "output_value",
                    description: Some(
                        "&index0 output value.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutputValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timestamp",
                    description: Some(
                        "&index0 adc timestamp used.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timestamp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_curr",
                    description: Some(
                        "&index0 error adc latest value.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcCurr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_pre0",
                    description: Some(
                        "&index0 error adc previous0 value.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcPre0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eadc_pre1",
                    description: Some(
                        "&index0 error adc previous1 value.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EadcPre1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2z2_curr",
                    description: Some(
                        "&index0 2p2z latest value.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2z2Curr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p2z2_pre0",
                    description: Some(
                        "&index0 2p2z previous0 value.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P2z2Pre0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_curr",
                    description: Some(
                        "&index0 3p3z latest value.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3Curr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_forbid_lo",
                    description: Some(
                        "&index0 3p3z output forbid low threshold.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3ForbidLo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_forbid_md",
                    description: Some(
                        "&index0 3p3z output forbid middle threshold.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3ForbidMd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p3z3_forbid_hi",
                    description: Some(
                        "&index0 3p3z output forbid high threshold.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P3z3ForbidHi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_sw",
                    description: Some(
                        "&index0 adc software inject value.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "&index0 irq_status.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AdcChan",
            extends: None,
            description: Some(
                "&index0 adc used channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_chan",
                    description: Some(
                        "adc used chan ID.",
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
            ],
        },
        FieldSet {
            name: "AdcExpect",
            extends: None,
            description: Some(
                "&index0 adc expect.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_expect",
                    description: Some(
                        "adc expect value.",
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
            name: "AdcOffset",
            extends: None,
            description: Some(
                "&index0 adc used offset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_offset",
                    description: Some(
                        "adc used offset.",
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
            name: "AdcSw",
            extends: None,
            description: Some(
                "&index0 adc software inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_sw",
                    description: Some(
                        "adc software inject value.",
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
            name: "CoeffA0",
            extends: None,
            description: Some(
                "&index0 zone &index1 a0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_a0",
                    description: Some(
                        "coefficient a0.",
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
            name: "CoeffA1",
            extends: None,
            description: Some(
                "&index0 zone &index1 a1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_a1",
                    description: Some(
                        "coefficient a1.",
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
            name: "CoeffA2",
            extends: None,
            description: Some(
                "&index0 zone &index1 a2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_a2",
                    description: Some(
                        "coefficient a2.",
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
            name: "CoeffB0",
            extends: None,
            description: Some(
                "&index0 zone &index1 b0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_b0",
                    description: Some(
                        "coefficient b0.",
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
            name: "CoeffB1",
            extends: None,
            description: Some(
                "&index0 zone &index1 b1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_b1",
                    description: Some(
                        "coefficient b1.",
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
            name: "CoeffB2",
            extends: None,
            description: Some(
                "&index0 zone &index1 b2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_b2",
                    description: Some(
                        "coefficient b2.",
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
            name: "CoeffB3",
            extends: None,
            description: Some(
                "&index0 zone &index1 b3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_b3",
                    description: Some(
                        "coefficient b3.",
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
            name: "CoeffKs",
            extends: None,
            description: Some(
                "&index0 zone &index1 kscaling.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coeff_kscaling",
                    description: Some(
                        "coefficient kscaling.",
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
            ],
        },
        FieldSet {
            name: "DqAdcSwReady",
            extends: None,
            description: Some(
                "enable d/q chan software inject adc value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dq_adc_sw_ready",
                    description: Some(
                        "enable d/q chan software inject adc value.",
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
            name: "EadcCurr",
            extends: None,
            description: Some(
                "&index0 error adc latest value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_curr",
                    description: Some(
                        "error adc latest value.",
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
            name: "EadcHighth",
            extends: None,
            description: Some(
                "&index0 eadc_highth value used in error adc cofficient selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_highth",
                    description: Some(
                        "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient.",
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
            name: "EadcLowth",
            extends: None,
            description: Some(
                "&index0 eadc_lowth value used in error adc cofficient selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_lowth",
                    description: Some(
                        "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient.",
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
            name: "EadcMidhighth",
            extends: None,
            description: Some(
                "&index0 eadc_midhighth value used in error adc cofficient selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_midhighth",
                    description: Some(
                        "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient.",
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
            name: "EadcMidlowth",
            extends: None,
            description: Some(
                "&index0 eadc_midlowth value used in error adc cofficient selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_midlowth",
                    description: Some(
                        "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient.",
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
            name: "EadcPre0",
            extends: None,
            description: Some(
                "&index0 error adc previous0 value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_pre0",
                    description: Some(
                        "error adc previous 0 value.",
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
            name: "EadcPre1",
            extends: None,
            description: Some(
                "&index0 error adc previous1 value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eadc_pre1",
                    description: Some(
                        "error adc previous 1 value.",
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
                "&index0 mode ctrl.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable_irq",
                    description: Some(
                        "enable irq: irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0.",
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
                    name: "dq_mode",
                    description: Some(
                        "dq mode.",
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
                    name: "mask_mode",
                    description: Some(
                        "open mode: CLC keep working even if bad irq status ocurred.",
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
                    name: "enable_clc",
                    description: Some(
                        "enable CLC.",
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
            name: "OutputValue",
            extends: None,
            description: Some(
                "&index0 output value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "output_value",
                    description: Some(
                        "output_value.",
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
            name: "P2z2ClampHi",
            extends: None,
            description: Some(
                "&index0 2p2z output clamp high threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_2p2z_clamp_hi",
                    description: Some(
                        "2p2z output clamp high threshold.",
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
            name: "P2z2ClampLo",
            extends: None,
            description: Some(
                "&index0 2p2z output clamp low threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_2p2z_clamp_lo",
                    description: Some(
                        "2p2z output clamp low threshold.",
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
            name: "P2z2Curr",
            extends: None,
            description: Some(
                "&index0 2p2z latest value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_2p2z_curr",
                    description: Some(
                        "2p2z latest value.",
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
            name: "P2z2Pre0",
            extends: None,
            description: Some(
                "&index0 2p2z previous0 value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_2p2z_pre0",
                    description: Some(
                        "2p2z previous 0 value.",
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
            name: "P3z3ClampHi",
            extends: None,
            description: Some(
                "&index0 3p3z output clamp high threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_clamp_hi",
                    description: Some(
                        "3p3z output clamp high threshold.",
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
            name: "P3z3ClampLo",
            extends: None,
            description: Some(
                "&index0 3p3z output clamp low threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_clamp_lo",
                    description: Some(
                        "3p3z output clamp low threshold.",
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
            name: "P3z3Curr",
            extends: None,
            description: Some(
                "&index0 3p3z latest value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_curr",
                    description: Some(
                        "3p3z latest value.",
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
            name: "P3z3ForbidHi",
            extends: None,
            description: Some(
                "&index0 3p3z output forbid high threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_forbid_hi",
                    description: Some(
                        "3p3z output forbid high threshold.",
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
            name: "P3z3ForbidLo",
            extends: None,
            description: Some(
                "&index0 3p3z output forbid low threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_forbid_lo",
                    description: Some(
                        "3p3z output forbid low threshold.",
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
            name: "P3z3ForbidMd",
            extends: None,
            description: Some(
                "&index0 3p3z output forbid middle threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "_3p3z_forbid_md",
                    description: Some(
                        "3p3z output forbid middle threshold.",
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
            name: "PwmPeriod",
            extends: None,
            description: Some(
                "&index0 pwm_period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm_period",
                    description: Some(
                        "pwm_period.",
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
            name: "Status",
            extends: None,
            description: Some(
                "&index0 irq_status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "status, write 1 to clear it. : irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0.",
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
            ],
        },
        FieldSet {
            name: "Timestamp",
            extends: None,
            description: Some(
                "&index0 adc timestamp used.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timestamp",
                    description: Some(
                        "timestamp.",
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
