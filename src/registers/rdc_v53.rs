use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rdc",
            extends: None,
            description: Some(
                "RDC.",
            ),
            items: &[
                BlockItem {
                    name: "rdc_ctl",
                    description: Some(
                        "rdc control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RdcCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acc_i",
                    description: Some(
                        "accumulate result of i_channel.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AccI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acc_q",
                    description: Some(
                        "accumulate result of q_channel.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AccQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_ctl",
                    description: Some(
                        "input channel selection.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_ctl",
                    description: Some(
                        "output channel selection.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exc_timming",
                    description: Some(
                        "excitation signal timming setting.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExcTimming",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exc_scaling",
                    description: Some(
                        "amplitude scaling for excitation.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExcScaling",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exc_offset",
                    description: Some(
                        "amplitude offset setting.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExcOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_scaling",
                    description: Some(
                        "amplitude scaling for excitation.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmScaling",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_offset",
                    description: Some(
                        "amplitude offset setting.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trig_out0_cfg",
                    description: Some(
                        "Configuration for trigger out 0 in clock cycle.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrigOut0Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trig_out1_cfg",
                    description: Some(
                        "Configuration for trigger out 1 in clock cycle.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrigOut1Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm_dz",
                    description: Some(
                        "pwm dead zone control in clock cycle.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwmDz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync_out_ctrl",
                    description: Some(
                        "synchronize output signal control.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SyncOutCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exc_sync_dly",
                    description: Some(
                        "trigger in delay timming in soc bus cycle.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExcSyncDly",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_i",
                    description: Some(
                        "max value of i_channel.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MaxI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "min_i",
                    description: Some(
                        "min value of i_channel.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MinI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max_q",
                    description: Some(
                        "max value of q_channel.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MaxQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "min_q",
                    description: Some(
                        "min value of q_channel.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MinQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "thrs_i",
                    description: Some(
                        "the offset setting for edge detection of the i_channel.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ThrsI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "thrs_q",
                    description: Some(
                        "the offset setting for edge detection of the q_channel.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ThrsQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edg_det_ctl",
                    description: Some(
                        "the control for edge detection.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdgDetCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acc_scaling",
                    description: Some(
                        "scaling for accumulation result.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AccScaling",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exc_period",
                    description: Some(
                        "period of excitation.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExcPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync_delay_i",
                    description: Some(
                        "delay setting in clock cycle for synchronous signal.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SyncDelayI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rise_delay_i",
                    description: Some(
                        "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RiseDelayI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fall_delay_i",
                    description: Some(
                        "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FallDelayI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sample_rise_i",
                    description: Some(
                        "sample value on rising edge of rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SampleRiseI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sample_fall_i",
                    description: Some(
                        "sample value on falling edge of rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SampleFallI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acc_cnt_i",
                    description: Some(
                        "number of accumulation.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AccCntI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sign_cnt_i",
                    description: Some(
                        "sample counter of opposite sign with rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SignCntI",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync_delay_q",
                    description: Some(
                        "delay setting in clock cycle for synchronous signal.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SyncDelayQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rise_delay_q",
                    description: Some(
                        "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RiseDelayQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fall_delay_q",
                    description: Some(
                        "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FallDelayQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sample_rise_q",
                    description: Some(
                        "sample value on rising edge of rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SampleRiseQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sample_fall_q",
                    description: Some(
                        "sample value on falling edge of rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SampleFallQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acc_cnt_q",
                    description: Some(
                        "number of accumulation.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AccCntQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sign_cnt_q",
                    description: Some(
                        "sample counter of opposite sign with rectify signal.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SignCntQ",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "amp_max",
                    description: Some(
                        "the maximum of acc amplitude.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AmpMax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "amp_min",
                    description: Some(
                        "the minimum of acc amplitude.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AmpMin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "the interrupt mask control.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
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
                    name: "adc_int_state",
                    description: Some(
                        "the interrupt state.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcIntState",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AccCntI",
            extends: None,
            description: Some(
                "number of accumulation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_pos",
                    description: Some(
                        "sample number during the positive of rectify signal 1: 1 2: 2 ….",
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
                    name: "cnt_neg",
                    description: Some(
                        "sample number during the negtive of rectify signal 1: 1 2: 2 ….",
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
            name: "AccCntQ",
            extends: None,
            description: Some(
                "number of accumulation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_pos",
                    description: Some(
                        "sample number during the positive of rectify signal 1: 1 2: 2 ….",
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
                    name: "cnt_neg",
                    description: Some(
                        "sample number during the negtive of rectify signal 1: 1 2: 2 ….",
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
            name: "AccI",
            extends: None,
            description: Some(
                "accumulate result of i_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "accumulate result of i_channel, this is a signed number.",
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
            name: "AccQ",
            extends: None,
            description: Some(
                "accumulate result of q_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "accumulate result of q_channel, this is a signed number.",
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
            name: "AccScaling",
            extends: None,
            description: Some(
                "scaling for accumulation result.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc_shift",
                    description: Some(
                        "Accumulation value shift control, this is a sign number. 0: {acc[39],acc[38:8]} 1: {acc[39],acc[37:7]} 2: {acc[39],acc[36:6]} … 7: {acc[39],acc[31:1]} 8: {acc[39],acc[30:0]} 9: acc/2^9 10: acc/2^10 … 15:acc/2^15.",
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
                    name: "toxic_lk",
                    description: Some(
                        "Toxic accumulation data be removed control 1: enable 0: disable.",
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
            name: "AdcIntState",
            extends: None,
            description: Some(
                "the interrupt state.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc_amp_ovl_sta",
                    description: Some(
                        "accumulate ample underflow interrupt status.",
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
                    name: "acc_amp_ovh_sta",
                    description: Some(
                        "accumulate ample overflow interrupt status.",
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
                    name: "acc_vld_q_ovl_sta",
                    description: Some(
                        "q_channel accumulate underflow interrupt status.",
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
                    name: "acc_vld_i_ovl_sta",
                    description: Some(
                        "i_channel accumulate underflow interrupt status.",
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
                    name: "acc_vld_q_ovh_sta",
                    description: Some(
                        "q_channel accumulate overflow interrupt status.",
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
                    name: "acc_vld_i_ovh_sta",
                    description: Some(
                        "i_channel accumulate overflow interrupt status.",
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
                    name: "sample_falling_q_sta",
                    description: Some(
                        "q_channel falling edge interrupt status.",
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
                    name: "sample_rising_q_sta",
                    description: Some(
                        "q_channel rising edge interrupt status.",
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
                    name: "sample_falling_i_sta",
                    description: Some(
                        "i_channel falling edge interrupt status.",
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
                    name: "sample_rising_i_sta",
                    description: Some(
                        "i_channel rising edge interrupt status.",
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
                    name: "falling_delay_q_sta",
                    description: Some(
                        "q_channel delayed rectify signal falling edge interrupt status.",
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
                    name: "rising_delay_q_sta",
                    description: Some(
                        "q_channel delayed rectify signal rising edge interrupt status.",
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
                    name: "falling_delay_i_sta",
                    description: Some(
                        "i_channel delayed rectify signal falling edge interrupt status.",
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
                    name: "rising_delay_i_sta",
                    description: Some(
                        "i_channel delayed rectify signal rising edge interrupt status.",
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
                    name: "acc_vld_q_sta",
                    description: Some(
                        "q_channel accumulate valid interrupt status for i_channel.",
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
                    name: "acc_vld_i_sta",
                    description: Some(
                        "i_channel accumulate valid interrupt status for i_channel.",
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
            ],
        },
        FieldSet {
            name: "AmpMax",
            extends: None,
            description: Some(
                "the maximum of acc amplitude.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max",
                    description: Some(
                        "the maximum of acc amplitude.",
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
            name: "AmpMin",
            extends: None,
            description: Some(
                "the minimum of acc amplitude.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "min",
                    description: Some(
                        "the minimum of acc amplitude.",
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
            name: "EdgDetCtl",
            extends: None,
            description: Some(
                "the control for edge detection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "filter",
                    description: Some(
                        "The continuous positive or negative number for edge detection 0: 1 1: 2 … 7: 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hold",
                    description: Some(
                        "The minimum edge distance in sample 0:1 sample 1:2 sample 2:3 samples … 63:64 samples.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ExcOffset",
            extends: None,
            description: Some(
                "amplitude offset setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_offset",
                    description: Some(
                        "Offset for excitation.",
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
            name: "ExcPeriod",
            extends: None,
            description: Some(
                "period of excitation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exc_period",
                    description: Some(
                        "The num in clock cycle for period of excitation 0: invalid value 1:1 cycle 2:2 cycles ….",
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
            name: "ExcScaling",
            extends: None,
            description: Some(
                "amplitude scaling for excitation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_man",
                    description: Some(
                        "Amplitude scaling for excitation, amplitude = [table value] x man / 2^exp.",
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
                    name: "amp_exp",
                    description: Some(
                        "Amplitude scaling for excitation, amplitude = [table value] x man / 2^exp.",
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
            ],
        },
        FieldSet {
            name: "ExcSyncDly",
            extends: None,
            description: Some(
                "trigger in delay timming in soc bus cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delay",
                    description: Some(
                        "Trigger in delay timming in bus cycle from rising edge of trigger signal 0: 1 cycle 1: 2 cycle … 0xffffff: 2^24 cycle.",
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
                Field {
                    name: "disable",
                    description: Some(
                        "Disable hardware trigger input 0: enable 1: disable.",
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
            name: "ExcTimming",
            extends: None,
            description: Some(
                "excitation signal timming setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smp_rate",
                    description: Some(
                        "The period for excitation sample in clock cycle， 0: not allowed 1: 1 cycle 2: 2 cycles … 65535 : 65535 cycles.",
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
                    name: "smp_num",
                    description: Some(
                        "Number of sample every excitation period 0: 4 point 1: 8 point … 8: 1024 point.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm_prd",
                    description: Some(
                        "Pwm period in samples， 0：1 sample period 1: 2 sample period ... 15: 16 sample period.",
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
                    name: "swap",
                    description: Some(
                        "Swap output of PWM and DAC 0: disable swap 1: swap output.",
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
            name: "FallDelayI",
            extends: None,
            description: Some(
                "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fall_delay",
                    description: Some(
                        "Delay value on falling edge of i_channel data 0: 1 cycle 1: 2 cycles ….",
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
            name: "FallDelayQ",
            extends: None,
            description: Some(
                "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fall_delay",
                    description: Some(
                        "Delay value on falling edge of q_channel data 0: 1 cycle 1: 2 cycles ….",
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
            name: "InCtl",
            extends: None,
            description: Some(
                "input channel selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch_i_sel",
                    description: Some(
                        "Input channel selection for i_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected.",
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
                    name: "port_i_sel",
                    description: Some(
                        "Input port selection for i_channel, 0:sel port0 1:sel port1.",
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
                    name: "ch_q_sel",
                    description: Some(
                        "Input channel selection for q_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "port_q_sel",
                    description: Some(
                        "Input port selection for q_channel, 0:sel port0 1:sel port1.",
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
            ],
        },
        FieldSet {
            name: "IntEn",
            extends: None,
            description: Some(
                "the interrupt mask control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc_amp_ovl_en",
                    description: Some(
                        "accumulate ample underflow interrupt enable.",
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
                    name: "acc_amp_ovh_en",
                    description: Some(
                        "accumulate ample overflow interrupt enable.",
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
                    name: "acc_vld_q_ovl_en",
                    description: Some(
                        "q_channel accumulate underflow interrupt enable.",
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
                    name: "acc_vld_i_ovl_en",
                    description: Some(
                        "i_channel accumulate underflow interrupt enable.",
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
                    name: "acc_vld_q_ovh_en",
                    description: Some(
                        "q_channel accumulate overflow interrupt enable.",
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
                    name: "acc_vld_i_ovh_en",
                    description: Some(
                        "i_channel accumulate overflow interrupt enable.",
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
                    name: "sample_falling_q_en",
                    description: Some(
                        "q_channel falling edge interrupt enable.",
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
                    name: "sample_rising_q_en",
                    description: Some(
                        "q_channel rising edge interrupt enable.",
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
                    name: "sample_falling_i_en",
                    description: Some(
                        "i_channel falling edge interrupt enable.",
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
                    name: "sample_rising_i_en",
                    description: Some(
                        "i_channel rising edge interrupt enable.",
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
                    name: "falling_delay_q_en",
                    description: Some(
                        "q_channel delayed rectify signal falling edge interrupt enable.",
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
                    name: "rising_delay_q_en",
                    description: Some(
                        "q_channel delayed rectify signal rising edge interrupt enable.",
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
                    name: "falling_delay_i_en",
                    description: Some(
                        "i_channel delayed rectify signal falling edge interrupt enable.",
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
                    name: "rising_delay_i_en",
                    description: Some(
                        "i_channel delayed rectify signal rising edge interrupt enable.",
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
                    name: "acc_vld_q_en",
                    description: Some(
                        "q_channel accumulate valid interrupt enable for i_channel.",
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
                    name: "acc_vld_i_en",
                    description: Some(
                        "i_channel accumulate valid interrupt enable for i_channel.",
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
                    name: "int_en",
                    description: Some(
                        "enable interrupt output.",
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
            name: "MaxI",
            extends: None,
            description: Some(
                "max value of i_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "valid",
                    description: Some(
                        "Max value valid, write clear 0: max value is not valid 1: max value is valid.",
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
                    name: "max",
                    description: Some(
                        "Max value of i_channel, write clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MaxQ",
            extends: None,
            description: Some(
                "max value of q_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "valid",
                    description: Some(
                        "Max value valid, write clear 0: max value is not valid 1: max value is valid.",
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
                    name: "max",
                    description: Some(
                        "Max value of q_channel, write clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MinI",
            extends: None,
            description: Some(
                "min value of i_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "valid",
                    description: Some(
                        "Min value valid, write clear 0: min value is not valid 1: min value is valid.",
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
                    name: "min",
                    description: Some(
                        "Min value of i_channel, write clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MinQ",
            extends: None,
            description: Some(
                "min value of q_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "valid",
                    description: Some(
                        "Min value valid, write clear 0: min value is not valid 1: min value is valid.",
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
                    name: "min",
                    description: Some(
                        "Min value of q_channel, write clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OutCtl",
            extends: None,
            description: Some(
                "output channel selection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch_i_sel",
                    description: Some(
                        "Output channel selection for i_channel.",
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
                    name: "ch_q_sel",
                    description: Some(
                        "Output channel selection for q_channel.",
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
            name: "PwmDz",
            extends: None,
            description: Some(
                "pwm dead zone control in clock cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dz_p",
                    description: Some(
                        "Exc_p dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone ….",
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
                    name: "dz_n",
                    description: Some(
                        "Exc_n dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone ….",
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
            name: "PwmOffset",
            extends: None,
            description: Some(
                "amplitude offset setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_offset",
                    description: Some(
                        "Offset for excitation.",
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
            name: "PwmScaling",
            extends: None,
            description: Some(
                "amplitude scaling for excitation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp_man",
                    description: Some(
                        "Amplitude scaling for excitation, amplitude = [table value] x man / 2^exp.",
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
                    name: "amp_exp",
                    description: Some(
                        "Amplitude scaling for excitation, amplitude = [table value] x man / 2^exp.",
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
                    name: "dither",
                    description: Some(
                        "Enable dither of pwm 0: disable 1: enable.",
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
                    name: "p_pol",
                    description: Some(
                        "Polarity of exc_p signal 0: high active 1: low active.",
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
                    name: "n_pol",
                    description: Some(
                        "Polarity of exc_n signal 0: high active 1: low active.",
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
            ],
        },
        FieldSet {
            name: "RdcCtl",
            extends: None,
            description: Some(
                "rdc control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exc_en",
                    description: Some(
                        "Enable rdc excite signal 0: rdc disable 1: rdc enable.",
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
                    name: "exc_start",
                    description: Some(
                        "Write 1 start excite signal, always read 0 0: no effect 1: start excite signal.",
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
                    name: "acc_en",
                    description: Some(
                        "Enable rdc accumulate 0: rdc disable 1: rdc enable.",
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
                    name: "rectify_sel",
                    description: Some(
                        "Select reference point of rectify signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal 4: use value on external pin 5: use invert value on external pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acc_len",
                    description: Some(
                        "Accumulate time, support on the fly change 0：1 cycle 1：2 cycles … 255: 256 cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ts_sel",
                    description: Some(
                        "Time stamp selection for accumulation 0: end of accumulation 1: start of accumulation 2: center of accumulation.",
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
            ],
        },
        FieldSet {
            name: "RiseDelayI",
            extends: None,
            description: Some(
                "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rise_delay",
                    description: Some(
                        "Delay value on rising edge of i_channel data 0: 1 cycle 1: 2 cycles ….",
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
            name: "RiseDelayQ",
            extends: None,
            description: Some(
                "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rise_delay",
                    description: Some(
                        "Delay value on rising edge of q_channel data 0: 1 cycle 1: 2 cycles ….",
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
            name: "SampleFallI",
            extends: None,
            description: Some(
                "sample value on falling edge of rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "sample value on falling edge of rectify signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SampleFallQ",
            extends: None,
            description: Some(
                "sample value on falling edge of rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "sample value on falling edge of rectify signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SampleRiseI",
            extends: None,
            description: Some(
                "sample value on rising edge of rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "sample value on rising edge of rectify signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SampleRiseQ",
            extends: None,
            description: Some(
                "sample value on rising edge of rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "sample value on rising edge of rectify signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SignCntI",
            extends: None,
            description: Some(
                "sample counter of opposite sign with rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_pos",
                    description: Some(
                        "Negative sample counter during positive rectify signal.",
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
                    name: "cnt_neg",
                    description: Some(
                        "Positive sample counter during negative rectify signal.",
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
            name: "SignCntQ",
            extends: None,
            description: Some(
                "sample counter of opposite sign with rectify signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_pos",
                    description: Some(
                        "Negative sample counter during positive rectify signal.",
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
                    name: "cnt_neg",
                    description: Some(
                        "Positive sample counter during negative rectify signal.",
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
            name: "SyncDelayI",
            extends: None,
            description: Some(
                "delay setting in clock cycle for synchronous signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delay",
                    description: Some(
                        "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ...",
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
            name: "SyncDelayQ",
            extends: None,
            description: Some(
                "delay setting in clock cycle for synchronous signal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delay",
                    description: Some(
                        "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ...",
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
            name: "SyncOutCtrl",
            extends: None,
            description: Some(
                "synchronize output signal control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync_out_sel",
                    description: Some(
                        "Select output synchornize signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal.",
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
                    name: "max2trig_en",
                    description: Some(
                        "Enable trigger out from the max point of exciting signal 1: enable 0: disable.",
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
                    name: "min2trig_en",
                    description: Some(
                        "Enable trigger out from the min point of exciting signal 1: enable 0: disable.",
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
                    name: "pwm_out_dly",
                    description: Some(
                        "Delay bettween the delyed trigger and the first pwm pulse in clock cycle 1: 1 cycle 2: 2 cycle ….",
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
            name: "ThrsI",
            extends: None,
            description: Some(
                "the offset setting for edge detection of the i_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thrs",
                    description: Some(
                        "The offset setting for edge detection of the i_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ThrsQ",
            extends: None,
            description: Some(
                "the offset setting for edge detection of the q_channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thrs",
                    description: Some(
                        "The offset setting for edge detection of the q_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TrigOut0Cfg",
            extends: None,
            description: Some(
                "Configuration for trigger out 0 in clock cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lead_tim",
                    description: Some(
                        "Lead time for trigger out0 from center of low level , this is a signed value … 2: 2 cycle befor center of low level 1: 1 cycle before center of low level 0: center of low level -1: 1cycle after center of low level -2: 2cycle after center of low level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Enable trigger out0 0: disable 1: enable.",
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
            ],
        },
        FieldSet {
            name: "TrigOut1Cfg",
            extends: None,
            description: Some(
                "Configuration for trigger out 1 in clock cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lead_tim",
                    description: Some(
                        "Lead time for trigger out0 from center of hight level , this is a signed value … 2: 2 cycle befor center of hight level 1: 1 cycle before center of hight level 0: center of hight level -1: 1cycle after center of hight level -2: 2cycle after center of hight level.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Enable trigger out1 0: disable 1: enable.",
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
            ],
        },
    ],
    enums: &[],
};
