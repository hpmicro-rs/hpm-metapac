use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cal",
            extends: None,
            description: Some(
                "no description available.",
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
                                "CalCfg0",
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
                                "CalCfg1",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Cmp",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "cfg",
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
                                "Cfg",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Cnt",
            extends: None,
            description: Some(
                "no description available.",
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
                                "CntCfg0",
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
                                "CntCfg1",
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
                    name: "cfg3",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg3",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pwm",
            extends: None,
            description: Some(
                "no description available.",
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
                                "PwmCfg0",
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
                                "PwmCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dead_area",
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
                                "DeadArea",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pwmv2",
            extends: None,
            description: Some(
                "PWM0.",
            ),
            items: &[
                BlockItem {
                    name: "work_ctrl0",
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
                                "WorkCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "unlock",
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
                                "Unlock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shadow_val",
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
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ShadowVal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "force_mode",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ForceMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "work_ctrl1",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WorkCtrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pwm",
                        },
                    ),
                },
                BlockItem {
                    name: "trigger_cfg",
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
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TriggerCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glb_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GlbCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glb_ctrl2",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GlbCtrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_reload_work",
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
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntReloadWork",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp_val_work",
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
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmpValWork",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "force_work",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x27c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ForceWork",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_val",
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
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntVal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac_value_sv",
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
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DacValueSv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "capture_pos",
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
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CapturePos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "capture_neg",
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
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CaptureNeg",
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
                    byte_offset: 0x400,
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
                    byte_offset: 0x404,
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
                    name: "irq_sts_cmp",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts_reload",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsReload",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts_cap_pos",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsCapPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts_cap_neg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsCapNeg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts_fault",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsFault",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_sts_burstend",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x424,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStsBurstend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_cmp",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnCmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_reload",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x434,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnReload",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_cap_pos",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x438,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnCapPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_cap_neg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x43c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnCapNeg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_fault",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x440,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnFault",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_en_burstend",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x444,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnBurstend",
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
                    byte_offset: 0x480,
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
                    name: "cnt",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x500,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Cnt",
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_glbcfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x540,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CntGlbcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cal",
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
                    byte_offset: 0x600,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Cal",
                        },
                    ),
                },
                BlockItem {
                    name: "cmp",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Cmp",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CalCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal_d_param",
                    description: Some(
                        "dac/counter value parameter.",
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
                    name: "cal_t_param",
                    description: Some(
                        "period parameter.",
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
                    name: "cal_ll_param",
                    description: Some(
                        "low limit parameter.",
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
                    name: "cal_lu_param",
                    description: Some(
                        "up limit parameter.",
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
            ],
        },
        FieldSet {
            name: "CalCfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal_in_off",
                    description: Some(
                        "offset for calculation unit, select from one of the shadow_val.",
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
                    name: "cal_lim_lo",
                    description: Some(
                        "low limit offset selection, select from one of the shadow_val.",
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
                    name: "cal_ll_en",
                    description: Some(
                        "set to enable low limit.",
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
                    name: "cal_lim_up",
                    description: Some(
                        "up limit offset selection, select from one of the shadow_val.",
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
                    name: "cal_lu_en",
                    description: Some(
                        "set to enable up limit.",
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
                    name: "cal_in_index",
                    description: Some(
                        "0~3 to select one of the dac input value; 4~7 to select one of the current counter value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cal_t_index",
                    description: Some(
                        "select one of 4 counter reload time.",
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
            ],
        },
        FieldSet {
            name: "CaptureNeg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "capture_neg",
                    description: Some(
                        "counter value captured at input negedge.",
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
            name: "CapturePos",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_index",
                    description: Some(
                        "related counter.",
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
                    name: "capture_selgpio",
                    description: Some(
                        "0: result from CAP[ 7:0], from trgm 1: result from CAP[15:8], from gpio.",
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
                    name: "capture_pos",
                    description: Some(
                        "related counter value captured at input negedge.",
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
            name: "Cfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp_cnt",
                    description: Some(
                        "select one from 4 counters, only for N>=16. for N<16, this field is0, every 4 compare point related to one counter(0123 for counter0, 4567 for counter1….).",
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
                    name: "cmp_in_sel",
                    description: Some(
                        "0x00~0x1B select one of the shadow_val directly 0x20~0x2F select one of the calculation cell output 0x30~0x37 select one of capture_pos value(low 8bit are 0) 0x38+k select T/4 0x3E select 0xFFFFF000 0x3F select 0xFFFFFF00 others select 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: Some(
                        "CmpSource",
                    ),
                },
                Field {
                    name: "cmp_update_time",
                    description: Some(
                        "define when to use the shadow register value for working register(trig_cmp) 000: software set work_ctrl1.shadow_lock bit 001: update immediately(at next cycle) 010: related counter reload time 011: use cmp_update_trigger(from trig_mux, selected by cmp_trig_sel) 100: use the related counter rld_cmp_sel0 to select one compare point 101: use the related counter rld_cmp_sel1, to select one compare point 11x: reserved, no update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "CmpShadowUpdateTrigger",
                    ),
                },
                Field {
                    name: "cmp_trig_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
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
                    name: "cnt_trig0",
                    description: Some(
                        "change counter value to one of the calculation cell output when cnt_update_triger0 issued.",
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
                    name: "cnt_update_en0",
                    description: Some(
                        "set to enable using trig0 to load calculation cell output to counter.",
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
                    name: "cnt_update_trig0",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
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
                Field {
                    name: "cnt_trig1",
                    description: Some(
                        "change counter value to one of the calculation cell output when cnt_update_triger1 issued.",
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
                    name: "cnt_update_en1",
                    description: Some(
                        "set to enable using trig1 to load calculation cell output to counter.",
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
                    name: "cnt_update_trig1",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
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
                    name: "cnt_reload_trig",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt_reload_en",
                    description: Some(
                        "set to use input signal(selected by cnt_reload_trig) to reload timer.",
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
            name: "Cfg3",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_burst",
                    description: Some(
                        "output pwm wave for configured burst(timer period), 0 for one burst; 1 for two burst. set to 0xFFFF for always output pwm wave bit's only used when setting cnt_sw_start or trigger selected by cnt_start_sel.",
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
                    name: "cnt_hw_start_en",
                    description: Some(
                        "enable use trigger to start pwm output(at next reload point), by cnt_start_sel.",
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
                    name: "cnt_start_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
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
            ],
        },
        FieldSet {
            name: "CmpValWork",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "compare point working register.",
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
            name: "CntCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_d_param",
                    description: Some(
                        "input dac data parameter.",
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
                    name: "rld_update_time",
                    description: Some(
                        "define when to use the calculation output value as reload time 00: software set work_ctrl1.shadow_lock bit 01: use compare point selected by rld_cmp_sel0 or rld_cmp_sel1 10: counter reload time 11: use rld_trig_sel to select one of the input trigger NOTE: 00 is not recommended since the update time is not controllable, may cause error in complex application.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ReloadUpdateTrigger",
                    ),
                },
                Field {
                    name: "rld_trig_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rld_cmp_sel0",
                    description: Some(
                        "select one compare point from 24, set to 0x1F to disable current selection.",
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
                    name: "rld_cmp_sel1",
                    description: Some(
                        "select one compare point from 24, set to 0x1F to disable current selection, used for reload value, compare value, force value update.",
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
            ],
        },
        FieldSet {
            name: "CntCfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_in_off",
                    description: Some(
                        "input data offset selection, from one of the shadow_val, default just shadow reload time.",
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
                    name: "cnt_lim_lo",
                    description: Some(
                        "low limit offset selection, from one of the shadow_val.",
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
                    name: "cnt_ll_en",
                    description: Some(
                        "set to enable low limit.",
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
                    name: "cnt_lim_up",
                    description: Some(
                        "up limit offset selection, from one of the shadow_val.",
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
                    name: "cnt_lu_en",
                    description: Some(
                        "set to enable up limit, use cnt_lu_off to select one of the shadow register value as limitation.",
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
                    name: "cnt_dac_index",
                    description: Some(
                        "select one of the dac value.",
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
            ],
        },
        FieldSet {
            name: "CntGlbcfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer_enable",
                    description: Some(
                        "1 to enable the main cycle counter; 0 to stop the counter; NOTE: when counter stopped, the related trigger_out will be cleared to 0, the related pwm output will keep value not changed.",
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
                    name: "timer_reset",
                    description: Some(
                        "set to clear current timer. Auto clear.",
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
                    name: "cnt_sw_start",
                    description: Some(
                        "set to start pwm output(at next reload point), write only, Auto clear. User can disable pwm output before burst end by start again with cnt_burst=0.",
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
            ],
        },
        FieldSet {
            name: "CntReloadWork",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "counter0 reload working register.",
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
            name: "CntVal",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "main counter value.",
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
            name: "DacValueSv",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "save dac0_value when dac0_valid if dac_sw_mode is 0; software write dac_value directly if dac_sw_mode is 1.",
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
            name: "DeadArea",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dead_area",
                    description: Some(
                        "16bit cycle delay plus 8bit hr_delay min value is 2 cycles, less than 0x200 will be treated as no dead area; NOTE: dead insertion must be configured with pair, that is, for pwm 01/23/45/67. otherwise the result maybe UNKNOWN!!!.",
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
            name: "DmaEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma0_sel",
                    description: Some(
                        "selelct one of compare point(0~23) or one reload point(24~27) as dma0.",
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
                    name: "dma0_en",
                    description: Some(
                        "enable dma0.",
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
                    name: "dma1_sel",
                    description: Some(
                        "selelct one of compare point(0~23) or one reload point(24~27) as dma0.",
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
                    name: "dma1_en",
                    description: Some(
                        "enable dma1.",
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
                    name: "dma2_sel",
                    description: Some(
                        "selelct one of compare point(0~23) or one reload point(24~27) as dma0.",
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
                    name: "dma2_en",
                    description: Some(
                        "enable dma2.",
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
                    name: "dma3_sel",
                    description: Some(
                        "selelct one of compare point(0~23) or one reload point(24~27) as dma0.",
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
                    name: "dma3_en",
                    description: Some(
                        "enable dma3.",
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
            name: "ForceMode",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_mode",
                    description: Some(
                        "2bit for each PWM channel(0~7); 00: force output 0 01: force output 1 10: output highz(pad_oe_*=0) 11: no force this field may be changed by software as shadow register , the update time should be defined by chan_cfg.load, only for PWM channels.",
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
                        "ForceMode",
                    ),
                },
                Field {
                    name: "polarity",
                    description: Some(
                        "one bit for one pwm channel, it's used as shadow register when pwm_cfg0.polarity_opt0 is set. output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ForceWork",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_mode",
                    description: Some(
                        "force_mode work register.",
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
                        "ForceMode",
                    ),
                },
                Field {
                    name: "out_polarity",
                    description: Some(
                        "force working register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GlbCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frac_disable",
                    description: Some(
                        "set to disable bit[7:0] in DAC value when Calculation Unit use it.",
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
                    name: "hr_pwm_en",
                    description: Some(
                        "set to enable hr pwm, clear to bypass delay chain.",
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
                    name: "output_delay",
                    description: Some(
                        "add delay after dead_area insertiong logic, for hr_pwm.",
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
                    name: "sw_force",
                    description: Some(
                        "software write 1 to start software force, if the pwm_cfg[n].sw_force_en is set, force will take effort.",
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
            name: "GlbCtrl2",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow_lock_en",
                    description: Some(
                        "enable shadow_lock feature, if cleared, shadow_lock will be always 0.",
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
                    name: "fault_clear",
                    description: Some(
                        "software write 1 to clear fault event if pwm_cfg.fault_rec_time is 2'b11. software need to clear it after the fault signal is de-assert and before next fault one bit for one pwm channel.",
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
                    name: "debug_in_en",
                    description: Some(
                        "set to enable debug_in signal as fault signal, generally disable pwm output.",
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
                    name: "dac_sw_mode",
                    description: Some(
                        "set for software DAC mode, software can write dac_value*_sv directly, and dac_valid from moto system is ignored.",
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
            name: "IrqEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_overflow",
                    description: Some(
                        "enable interrupt when calculation unit overflow.",
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
            name: "IrqEnBurstend",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_burstend",
                    description: Some(
                        "interrupt enable field for output burst done event , and each bit means one main counter.",
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
            ],
        },
        FieldSet {
            name: "IrqEnCapNeg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_cap_neg",
                    description: Some(
                        "interrupt enable field for negedge capture event , and each bit means one capture channel.",
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
            name: "IrqEnCapPos",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_cap_pos",
                    description: Some(
                        "interrupt enable field for posedge capture event , and each bit means one capture channel.",
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
            name: "IrqEnCmp",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_cmp",
                    description: Some(
                        "interrupt enable field for compare point match event, and each bit means one compare point.",
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
            name: "IrqEnFault",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_fault",
                    description: Some(
                        "interrupt enable field for external fault event , and each bit means one external fault channel.",
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
            name: "IrqEnReload",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en_reload",
                    description: Some(
                        "interrupt enable field for reload event , and each bit means one main counter.",
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
                    name: "irq_cmp",
                    description: Some(
                        "for 24 channel, compare event.",
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
                    name: "irq_reload",
                    description: Some(
                        "when clock counter reach the reload time.",
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
                    name: "irq_capture_pos",
                    description: Some(
                        "capture posedge status.",
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
                    name: "irq_capture_neg",
                    description: Some(
                        "capture negedge status.",
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
                    name: "irq_fault",
                    description: Some(
                        "for external fault event.",
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
                    name: "irq_burstend",
                    description: Some(
                        "end of output burst.",
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
                    name: "irq_cal_overflow",
                    description: Some(
                        "end of output burst.",
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
            name: "IrqStsBurstend",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_burstend",
                    description: Some(
                        "interrupt flag for output burst done event , and each bit means one main counter.",
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
            ],
        },
        FieldSet {
            name: "IrqStsCapNeg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_cap_neg",
                    description: Some(
                        "interrupt flag for negedge capture event , and each bit means one capture channel.",
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
            name: "IrqStsCapPos",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_cap_pos",
                    description: Some(
                        "interrupt flag for posedge capture event , and each bit means one capture channel.",
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
            name: "IrqStsCmp",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_cmp",
                    description: Some(
                        "interrupt flag for compare point match event, and each bit means one compare point.",
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
            name: "IrqStsFault",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_fault",
                    description: Some(
                        "interrupt flag for external fault event , and each bit means one external fault channel.",
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
            name: "IrqStsReload",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_sts_reload",
                    description: Some(
                        "interrupt flag for reload event , and each bit means one main counter.",
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
            ],
        },
        FieldSet {
            name: "PwmCfg0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "polarity_opt0",
                    description: Some(
                        "set to use shadow polarity.",
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
                    name: "out_polarity",
                    description: Some(
                        "output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault) when polarity_opt0 is set, this bit is controlled by shadow register, can't be writable; read as working register use compare channel settings(in cmp_cfg) as shadow register update.",
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
                    name: "pol_update_sel",
                    description: Some(
                        "used when polarity_opt0 is set, define when to update polarity working register. 0: software set work_ctrl1.shadow_lock bit 1: update at reload point;.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ShadowOutputPolarity",
                    ),
                },
                Field {
                    name: "fault_en_sync",
                    description: Some(
                        "set to enable the input faults from trig_mux(trigger_in[0] for channel0/1, 1 for 23, 2 for 45, 3 for 67).",
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
                    name: "fault_en_async",
                    description: Some(
                        "set to enable the input async faults from pad directly.",
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
                    name: "fault_pol_async",
                    description: Some(
                        "fault polarity for input fault from pad, 1-active low; 0-active high;.",
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
                    name: "fault_sel_async",
                    description: Some(
                        "select from 16bit async fault from pad.",
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
                    name: "trig_sel4",
                    description: Some(
                        "for N=0/2/4/6, clear to select 2 compare point(N*2~N*2+1); set to select 4 compare point(N*2~N*2+3); or use 2 compare point(N*2+2~N*2+3); for N=1/3/5/7, this bit is no means, it can work on pair mode, or use 2 compare point (N*2+2~N*2+3); assume select ab or abcd, abcd can between 0 and 2T. output will be 1 when counter value between a and b; if b<=a then output all 0; if b>=(T+a), then output all 1;.",
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
            name: "PwmCfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fault_rec_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux, used for fault recovery if fault_rec_time is set to 2'b10.",
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
                    name: "pwm_force_sel",
                    description: Some(
                        "select one trigger from 8 as force signal, should be level signal, 1 for force active, 0 for no force.",
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
                    name: "force_act_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux, will load hw/sw force at this time.",
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
                Field {
                    name: "force_trig_sel",
                    description: Some(
                        "select one trigger from 8, should set to pulse in trig_mux, will load shadow register(force)mode) to force_mode_work at this time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "force_time",
                    description: Some(
                        "00: force immediately 01: force at main counter reload time 10: force at trig signal selected by force_act_sel 11: no force the force assert/deassert will happen at the force_time; qeo force and value also latched at this time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ForceTrigger",
                    ),
                },
                Field {
                    name: "pwm_logic",
                    description: Some(
                        "valid only for pwm0/2/4/6 when trig_sel4 is set 00: ab OR cd; 01: ab AND cd; 10: ab XOR cd; 11: cd.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PwmLogic",
                    ),
                },
                Field {
                    name: "pair_mode",
                    description: Some(
                        "if set to 1, PWM work at pair mode, pwm_cfg for channel 2m is used for channel 2m+1(m=0,1,2,3), except the dead area, which is separate for each channel even in pair mode software need set this bit for both channel of one pair, otherwise result unknown.",
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
                    name: "sw_force_en",
                    description: Some(
                        "0 for hardware force, from trig_mux selected by pwm_force_sel 1 for software force, from glb_ctrl.sw_force.",
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
                    name: "fault_rec_time",
                    description: Some(
                        "00: immediately 01: after main counter reload time 10: use fault_rec_sel to select one of the input trigger 11: software write fault_clear in glb_ctrl2, no effort if pwm_fault is still assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "FaultRecoveryTrigger",
                    ),
                },
                Field {
                    name: "fault_mode",
                    description: Some(
                        "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "FaultMode",
                    ),
                },
                Field {
                    name: "force_update_time",
                    description: Some(
                        "define when to use the shadow register value for working register(force_mode) 00: software set work_ctrl1.shadow_lock bit 01: use the related counter rld_cmp_sel0 and rld_cmp_sel1, to select one compare point 10: related counter reload time(selected by pwm_cnt) 11: use force_trig_sel to select one of the input trigger NOTE: 00/01 are not recommended since the update time is not controllable, may cause error in complex application. 00 is used for initialization or debug, not suggest for real time update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ForceShadowUpdateTrigger",
                    ),
                },
                Field {
                    name: "highz_en_n",
                    description: Some(
                        "0 to highz pwm outputs(pad_oe*=0), software need set this bit to 1 to enable pwm output.",
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
            name: "ShadowVal",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frac",
                    description: Some(
                        "Fractional part of the shadow value.",
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
                    name: "value",
                    description: Some(
                        "shadow registers, if used as reload or compare point, shall be 24bit clock cycles plus 1bit half cycle and 7bit high-resolution delay.",
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
                Field {
                    name: "int",
                    description: Some(
                        "Integer part of the shadow value.",
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
            name: "TriggerCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigger_out_sel",
                    description: Some(
                        "select one from 24 compare result as trigger out, set at compare point, clear at reload point.",
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
            name: "Unlock",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "unlock_bit",
                    description: Some(
                        "bit2 to bit 29 for value_shadow, bit30 for force_mode the shadow registers can be updated only when related unlock_bit is set; this register can only be updated after unlock.",
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
            name: "WorkCtrl0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow_unlock",
                    description: Some(
                        "write 0x… first to unlock, then set related bits in unlock_sel to unlock following shadow registers(from 0x04 to 0x78), otherwise the shadow registers can not be written. The shadow registers will be loaded to work registers only when shadow_lock is 1 or lock is not enabled This bit can be cleared by set shadow_lock bit in work_ctrl1.",
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
            name: "WorkCtrl1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow_lock",
                    description: Some(
                        "one to lock, sofware can't write any shadow registers Software have to write 0x…. to work_ctrl0 to clear this bit.",
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
    ],
    enums: &[
        Enum {
            name: "CmpShadowUpdateTrigger",
            description: Some(
                "define when to use the shadow register value for working register(trig_cmp)",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ON_SHLK",
                    description: Some(
                        "software set work_ctrl1.shadow_lock bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_MODIFY",
                    description: Some(
                        "update immediately(at next cycle)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "related counter reload time",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ON_TRIGMUX",
                    description: Some(
                        "use cmp_update_trigger(from trig_mux, selected by cmp_trig_sel)",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ON_RLD_CMP_SEL0",
                    description: Some(
                        "use the related counter rld_cmp_sel0 to select one compare point",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "ON_RLD_CMP_SEL1",
                    description: Some(
                        "use the related counter rld_cmp_sel1, to select one compare point",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "reserved, no update",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "CmpSource",
            description: Some(
                "select one of the calculation cell output",
            ),
            bit_size: 6,
            variants: &[
                EnumVariant {
                    name: "SHADOW_VAL",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "CALCULATE",
                    description: None,
                    value: 32,
                },
                EnumVariant {
                    name: "CAPTURE_POS",
                    description: None,
                    value: 48,
                },
                EnumVariant {
                    name: "COUNTERS",
                    description: Some(
                        "select T/4",
                    ),
                    value: 56,
                },
                EnumVariant {
                    name: "_0XFFFFF000",
                    description: Some(
                        "select 0xFFFFF000",
                    ),
                    value: 62,
                },
                EnumVariant {
                    name: "_0XFFFFFF00",
                    description: Some(
                        "select 0xFFFFFF00",
                    ),
                    value: 63,
                },
            ],
        },
        Enum {
            name: "FaultMode",
            description: Some(
                "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_0",
                    description: Some(
                        "force output 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_1",
                    description: Some(
                        "force output 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH_Z",
                    description: Some(
                        "output highz(pad_oe_*=0)",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "FaultRecoveryTrigger",
            description: Some(
                "define when to use the shadow register value for working register(force_mode)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "IMMEDIATELY",
                    description: Some(
                        "immediately",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "after main counter reload time",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_HW_EVENT",
                    description: Some(
                        "after hardware event assert",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ON_FAULT_CLEAR",
                    description: Some(
                        "after software write faultclr bit in GCR register",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ForceMode",
            description: Some(
                "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_0",
                    description: Some(
                        "force output 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_1",
                    description: Some(
                        "force output 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HIGH_Z",
                    description: Some(
                        "output highz(pad_oe_*=0)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NO_FORCE",
                    description: Some(
                        "no force",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ForceShadowUpdateTrigger",
            description: Some(
                "define when to use the shadow register value for working register(force_mode)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "IMMEDIATELY",
                    description: Some(
                        "software set work_ctrl1.shadow_lock bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_CMP_POINT",
                    description: Some(
                        "related counter reload time(selected by pwm_cnt)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "use the related counter rld_cmp_sel0 and rld_cmp_sel1, to select one compare point",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "after SHSYNCI assert",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ForceTrigger",
            description: Some(
                "define when to use the shadow register value for working register(force_mode)",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "IMMEDIATELY",
                    description: Some(
                        "force immediately",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "force at main counter reload time",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_TRIGMUX",
                    description: Some(
                        "force at trig signal selected by force_act_sel",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NNONE",
                    description: Some(
                        "no force the force assert/deassert will happen at the force_time; qeo force and value also latched at this time",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "PwmLogic",
            description: Some(
                "valid only for pwm0/2/4/6 when trig_sel4 is set",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "AB_OR_CD",
                    description: Some(
                        "ab OR cd",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "AB_AND_CD",
                    description: Some(
                        "ab AND cd",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "AB_XOR_CD",
                    description: Some(
                        "ab XOR cd",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CD",
                    description: Some(
                        "cd",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ReloadUpdateTrigger",
            description: Some(
                "define when to use the calculation output value as reload time",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ON_SHLK",
                    description: Some(
                        "software set work_ctrl1.shadow_lock bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_COMPARE_POINT",
                    description: Some(
                        "use compare point selected by rld_cmp_sel0 or rld_cmp_sel1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "counter reload time",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ON_TRIGGER",
                    description: Some(
                        "use rld_trig_sel to select one of the input trigger",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "ShadowOutputPolarity",
            description: Some(
                "used when polarity_opt0 is set",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_SHLK",
                    description: Some(
                        "software set work_ctrl1.shadow_lock bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_RELOAD",
                    description: Some(
                        "update at reload point",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
