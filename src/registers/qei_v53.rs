use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Count",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "z",
                    description: Some(
                        "Z counter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Z",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ph",
                    description: Some(
                        "Phase counter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ph",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd",
                    description: Some(
                        "Speed counter.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr",
                    description: Some(
                        "Timer counter.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Qei",
            extends: None,
            description: Some(
                "QEI0.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phcfg",
                    description: Some(
                        "Phase configure register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdgcfg",
                    description: Some(
                        "Watchdog configure register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdgcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phidx",
                    description: Some(
                        "Phase index register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phidx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgoen",
                    description: Some(
                        "Tigger output enable register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trgoen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "readen",
                    description: Some(
                        "Read event enable register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Readen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zcmp",
                    description: Some(
                        "Z comparator.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phcmp",
                    description: Some(
                        "Phase comparator.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spdcmp",
                    description: Some(
                        "Speed comparator.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spdcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaen",
                    description: Some(
                        "DMA request enable register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqen",
                    description: Some(
                        "Interrupt request register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "count",
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
                    byte_offset: 0x30,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "count_current",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "count_read",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "count_snap0",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "count_snap1",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "zcmp2",
                    description: Some(
                        "Z comparator.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zcmp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phcmp2",
                    description: Some(
                        "Phase comparator.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phcmp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spdcmp2",
                    description: Some(
                        "Speed comparator.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spdcmp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "match_cfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MatchCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_a",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_b",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_z",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_h",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_h2",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filt_cfg_f",
                    description: Some(
                        "no description available.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FiltCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qei_cfg",
                    description: Some(
                        "qei config register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "QeiCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0_num",
                    description: Some(
                        "pulse0_num.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0Num",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1_num",
                    description: Some(
                        "pulse1_num.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1Num",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle0_cnt",
                    description: Some(
                        "cycle0_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle0Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle0pulse_cnt",
                    description: Some(
                        "cycle0pulse_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle0pulseCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle1_cnt",
                    description: Some(
                        "cycle1_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle1Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle1pulse_cnt",
                    description: Some(
                        "cycle1pulse_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle1pulseCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle0_snap0",
                    description: Some(
                        "cycle0_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle0Snap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle0_snap1",
                    description: Some(
                        "cycle0_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle0Snap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle1_snap0",
                    description: Some(
                        "cycle1_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle1Snap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle1_snap1",
                    description: Some(
                        "cycle1_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle1Snap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle0_num",
                    description: Some(
                        "cycle0_num.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle0Num",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cycle1_num",
                    description: Some(
                        "cycle1_num.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cycle1Num",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0_cnt",
                    description: Some(
                        "pulse0_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0cycle_cnt",
                    description: Some(
                        "pulse0cycle_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0cycleCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1_cnt",
                    description: Some(
                        "pulse1_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1cycle_cnt",
                    description: Some(
                        "pulse1cycle_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1cycleCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0_snap0",
                    description: Some(
                        "pulse0_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0Snap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0cycle_snap0",
                    description: Some(
                        "pulse0cycle_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0cycleSnap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0_snap1",
                    description: Some(
                        "pulse0_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0Snap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse0cycle_snap1",
                    description: Some(
                        "pulse0cycle_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse0cycleSnap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1_snap0",
                    description: Some(
                        "pulse1_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1Snap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1cycle_snap0",
                    description: Some(
                        "pulse1cycle_snap0.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1cycleSnap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1_snap1",
                    description: Some(
                        "pulse1_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1Snap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse1cycle_snap1",
                    description: Some(
                        "pulse1cycle_snap1.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pulse1cycleSnap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcx_cfg0",
                    description: Some(
                        "adcx_cfg0.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcxCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcx_cfg1",
                    description: Some(
                        "adcx_cfg1.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcxCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcx_cfg2",
                    description: Some(
                        "adcx_cfg2.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcxCfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcy_cfg0",
                    description: Some(
                        "adcy_cfg0.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcyCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcy_cfg1",
                    description: Some(
                        "adcy_cfg1.",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcyCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcy_cfg2",
                    description: Some(
                        "adcy_cfg2.",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcyCfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cal_cfg",
                    description: Some(
                        "cal_cfg.",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CalCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phase_param",
                    description: Some(
                        "phase_param.",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhaseParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_threshold",
                    description: Some(
                        "pos_threshold.",
                    ),
                    array: None,
                    byte_offset: 0x238,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosThreshold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uvw_pos",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UvwPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uvw_pos_cfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x258,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UvwPosCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phase_cnt",
                    description: Some(
                        "phase_cnt.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhaseCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phase_update",
                    description: Some(
                        "phase_update.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhaseUpdate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "position",
                    description: Some(
                        "position.",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Position",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "position_update",
                    description: Some(
                        "position_update.",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PositionUpdate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "angle",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Angle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_timeout",
                    description: Some(
                        "pos_timeout.",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosTimeout",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AdcxCfg0",
            extends: None,
            description: Some(
                "adcx_cfg0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x_chan",
                    description: Some(
                        "No description available.",
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
                    name: "x_adc_enable",
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
                    name: "x_adcsel",
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
            name: "AdcxCfg1",
            extends: None,
            description: Some(
                "adcx_cfg1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x_param0",
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
                    name: "x_param1",
                    description: Some(
                        "No description available.",
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
            name: "AdcxCfg2",
            extends: None,
            description: Some(
                "adcx_cfg2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x_offset",
                    description: Some(
                        "No description available.",
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
            name: "AdcyCfg0",
            extends: None,
            description: Some(
                "adcy_cfg0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_chan",
                    description: Some(
                        "No description available.",
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
                    name: "y_adc_enable",
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
                    name: "y_adcsel",
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
            name: "AdcyCfg1",
            extends: None,
            description: Some(
                "adcy_cfg1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_param0",
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
                    name: "y_param1",
                    description: Some(
                        "No description available.",
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
            name: "AdcyCfg2",
            extends: None,
            description: Some(
                "adcy_cfg2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_offset",
                    description: Some(
                        "No description available.",
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
            name: "Angle",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "angle",
                    description: Some(
                        "No description available.",
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
            name: "CalCfg",
            extends: None,
            description: Some(
                "cal_cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xy_delay",
                    description: Some(
                        "valid x/y delay, larger than this delay will be treated as invalid data. Default 1.25us@200MHz; max 80ms;.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "Control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enctyp",
                    description: Some(
                        "000-abz; 001-pd; 010-ud; 011-UVW(hal) 100-single A; 101-single sin; 110: sin&cos.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "WorkMode",
                    ),
                },
                Field {
                    name: "rd_sel",
                    description: Some(
                        "define the width/counter value(affect width_match, width_match2, width_cur, timer_cur, width_read, timer_read, width_snap0,width_snap1, timer_snap0, timer_snap1) 0 : same as hpm1000/500/500s; 1: use width for position; use timer for angle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SpdTmrReadSel",
                    ),
                },
                Field {
                    name: "rstcnt",
                    description: Some(
                        "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx.",
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
                    name: "snapen",
                    description: Some(
                        "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert.",
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
                    name: "faultpos",
                    description: Some(
                        "No description available.",
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
                    name: "hrdir1",
                    description: Some(
                        "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction).",
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
                    name: "hrdir0",
                    description: Some(
                        "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction).",
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
                    name: "hfdir1",
                    description: Some(
                        "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction).",
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
                    name: "hfdir0",
                    description: Some(
                        "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction).",
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
                    name: "pausez",
                    description: Some(
                        "1- pause zcnt when PAUSE assert.",
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
                    name: "pauseph",
                    description: Some(
                        "1- pause phcnt when PAUSE assert.",
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
                    name: "pausespd",
                    description: Some(
                        "1- pause spdcnt when PAUSE assert.",
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
                    name: "pausepos",
                    description: Some(
                        "1- pause position output valid when PAUSE assert.",
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
                    name: "h2rdir1",
                    description: Some(
                        "No description available.",
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
                    name: "h2rdir0",
                    description: Some(
                        "No description available.",
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
                    name: "h2fdir1",
                    description: Some(
                        "No description available.",
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
                    name: "h2fdir0",
                    description: Some(
                        "No description available.",
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
                    name: "z_only_en",
                    description: Some(
                        "1- phcnt will set to phidx when Z input assert(for xy analog signal and digital z, also need set phcaliz).",
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
                    name: "phcaliz",
                    description: Some(
                        "1- phcnt will set to phidx when Z input assert(for abz digital signsl).",
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
                    name: "zcntcfg",
                    description: Some(
                        "Counting mode of Z-phase counter. 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ZCntMode",
                    ),
                },
                Field {
                    name: "read",
                    description: Some(
                        "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0.",
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
            name: "Cycle0Cnt",
            extends: None,
            description: Some(
                "cycle0_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle0_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Cycle0Num",
            extends: None,
            description: Some(
                "cycle0_num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle0_num",
                    description: Some(
                        "No description available.",
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
            name: "Cycle0Snap0",
            extends: None,
            description: Some(
                "cycle0_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle0_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Cycle0Snap1",
            extends: None,
            description: Some(
                "cycle0_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle0_snap1",
                    description: Some(
                        "No description available.",
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
            name: "Cycle0pulseCnt",
            extends: None,
            description: Some(
                "cycle0pulse_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle0pulse_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Cycle1Cnt",
            extends: None,
            description: Some(
                "cycle1_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle1_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Cycle1Num",
            extends: None,
            description: Some(
                "cycle1_num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle1_num",
                    description: Some(
                        "No description available.",
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
            name: "Cycle1Snap0",
            extends: None,
            description: Some(
                "cycle1_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle1_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Cycle1Snap1",
            extends: None,
            description: Some(
                "cycle1_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle1_snap1",
                    description: Some(
                        "No description available.",
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
            name: "Cycle1pulseCnt",
            extends: None,
            description: Some(
                "cycle1pulse_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cycle1pulse_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Dmaen",
            extends: None,
            description: Some(
                "DMA request enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faultfen",
                    description: Some(
                        "No description available.",
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
                    name: "home2fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse1fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse0fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle1fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle0fen",
                    description: Some(
                        "No description available.",
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
                    name: "dirchgfen",
                    description: Some(
                        "No description available.",
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
                    name: "pos2cmpfen",
                    description: Some(
                        "No description available.",
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
                    name: "widthtmfen",
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
                    name: "zmissfen",
                    description: Some(
                        "No description available.",
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
                    name: "zphfen",
                    description: Some(
                        "1- generate dma request when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- generate dma request when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- generate dma request when homef flag set.",
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
                    name: "wdgfen",
                    description: Some(
                        "1- generate dma request when wdg flag set.",
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
            name: "FiltCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "filtlen",
                    description: Some(
                        "This bitfields defines the filter counter length.",
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
                        "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stable low mode; 111-stable high mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "FilterMode",
                    ),
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
            name: "Irqen",
            extends: None,
            description: Some(
                "Interrupt request register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faulte",
                    description: Some(
                        "No description available.",
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
                    name: "home2e",
                    description: Some(
                        "No description available.",
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
                    name: "pulse1e",
                    description: Some(
                        "No description available.",
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
                    name: "pulse0e",
                    description: Some(
                        "No description available.",
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
                    name: "cycle1e",
                    description: Some(
                        "No description available.",
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
                    name: "cycle0e",
                    description: Some(
                        "No description available.",
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
                    name: "dirchge",
                    description: Some(
                        "No description available.",
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
                    name: "pos2cmpe",
                    description: Some(
                        "No description available.",
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
                    name: "widthtme",
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
                    name: "zmisse",
                    description: Some(
                        "No description available.",
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
                    name: "zphie",
                    description: Some(
                        "1- generate interrupt when zphf flag set.",
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
                    name: "poscmpie",
                    description: Some(
                        "1- generate interrupt when poscmpf flag set.",
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
                    name: "homeie",
                    description: Some(
                        "1- generate interrupt when homef flag set.",
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
                    name: "wdgie",
                    description: Some(
                        "1- generate interrupt when wdg flag set.",
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
            name: "MatchCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_match2_opt",
                    description: Some(
                        "No description available.",
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
                    name: "pos_match2_dir",
                    description: Some(
                        "No description available.",
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
                    name: "phase_match_dis2",
                    description: Some(
                        "No description available.",
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
                    name: "spdcmp2dis",
                    description: Some(
                        "No description available.",
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
                    name: "dircmp2",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "dircmp2dis",
                    description: Some(
                        "No description available.",
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
                    name: "zcmp2dis",
                    description: Some(
                        "No description available.",
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
                    name: "pos_match_opt",
                    description: Some(
                        "No description available.",
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
                    name: "pos_match_dir",
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
                    name: "phase_match_dis",
                    description: Some(
                        "No description available.",
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
                    name: "spdcmpdis",
                    description: Some(
                        "No description available.",
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
                    name: "dircmp",
                    description: Some(
                        "0- position compare need positive rotation 1- position compare need negative rotation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "dircmpdis",
                    description: Some(
                        "1- postion compare not include rotation direction.",
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
                    name: "zcmpdis",
                    description: Some(
                        "1- postion compare not include zcnt.",
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
            name: "Ph",
            extends: None,
            description: Some(
                "Phase counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phcnt",
                    description: Some(
                        "phcnt value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bstat",
                    description: Some(
                        "1- b input is high 0- b input is low.",
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
                    name: "astat",
                    description: Some(
                        "1- a input is high 0- a input is low.",
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
                    name: "dir",
                    description: Some(
                        "1- reverse rotation 0- forward rotation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
            ],
        },
        FieldSet {
            name: "PhaseCnt",
            extends: None,
            description: Some(
                "phase_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phase_cnt",
                    description: Some(
                        "No description available.",
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
            name: "PhaseParam",
            extends: None,
            description: Some(
                "phase_param.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phase_param",
                    description: Some(
                        "No description available.",
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
            name: "PhaseUpdate",
            extends: None,
            description: Some(
                "phase_update.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "value to be added or minus from phase_cnt. only valid when inc or dec is set in one 32bit write operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dec",
                    description: Some(
                        "set to minus value from phase_cnt(set inc and dec same time willl act inc).",
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
                    name: "inc",
                    description: Some(
                        "set to add value to phase_cnt.",
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
            name: "Phcfg",
            extends: None,
            description: Some(
                "Phase configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phmax",
                    description: Some(
                        "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax.",
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
            name: "Phcmp",
            extends: None,
            description: Some(
                "Phase comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phcmp",
                    description: Some(
                        "phcnt position compare value.",
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
            name: "Phcmp2",
            extends: None,
            description: Some(
                "Phase comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phcmp2",
                    description: Some(
                        "No description available.",
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
            name: "Phidx",
            extends: None,
            description: Some(
                "Phase index register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phidx",
                    description: Some(
                        "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1.",
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
            name: "PosThreshold",
            extends: None,
            description: Some(
                "pos_threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_threshold",
                    description: Some(
                        "No description available.",
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
            name: "PosTimeout",
            extends: None,
            description: Some(
                "pos_timeout.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timeout",
                    description: Some(
                        "postion timeout value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "enable position timeout feature, if timeout, send valid again.",
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
            name: "Position",
            extends: None,
            description: Some(
                "position.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "position",
                    description: Some(
                        "No description available.",
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
            name: "PositionUpdate",
            extends: None,
            description: Some(
                "position_update.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "value to be added or minus from position. only valid when inc or dec is set in one 32bit write operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dec",
                    description: Some(
                        "set to minus value from position(set inc and dec same time willl act inc).",
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
                    name: "inc",
                    description: Some(
                        "set to add value to position.",
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
            name: "Pulse0Cnt",
            extends: None,
            description: Some(
                "pulse0_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Pulse0Num",
            extends: None,
            description: Some(
                "pulse0_num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0_num",
                    description: Some(
                        "for speed detection, will count the cycle number for configed pulse_num.",
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
            name: "Pulse0Snap0",
            extends: None,
            description: Some(
                "pulse0_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Pulse0Snap1",
            extends: None,
            description: Some(
                "pulse0_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0_snap1",
                    description: Some(
                        "No description available.",
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
            name: "Pulse0cycleCnt",
            extends: None,
            description: Some(
                "pulse0cycle_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0cycle_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Pulse0cycleSnap0",
            extends: None,
            description: Some(
                "pulse0cycle_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0cycle_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Pulse0cycleSnap1",
            extends: None,
            description: Some(
                "pulse0cycle_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse0cycle_snap1",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1Cnt",
            extends: None,
            description: Some(
                "pulse1_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1Num",
            extends: None,
            description: Some(
                "pulse1_num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1_num",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1Snap0",
            extends: None,
            description: Some(
                "pulse1_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1Snap1",
            extends: None,
            description: Some(
                "pulse1_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1_snap1",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1cycleCnt",
            extends: None,
            description: Some(
                "pulse1cycle_cnt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1cycle_cnt",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1cycleSnap0",
            extends: None,
            description: Some(
                "pulse1cycle_snap0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1cycle_snap0",
                    description: Some(
                        "No description available.",
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
            name: "Pulse1cycleSnap1",
            extends: None,
            description: Some(
                "pulse1cycle_snap1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pulse1cycle_snap1",
                    description: Some(
                        "No description available.",
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
            name: "QeiCfg",
            extends: None,
            description: Some(
                "qei config register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "siga_en",
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
                    name: "sigb_en",
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
                    name: "sigz_en",
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
                    name: "posidge_en",
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
                    name: "negedge_en",
                    description: Some(
                        "bit4: negedge enable bit3: posedge enable bit2: W in hal enable bit1: signal b(or V in hal) enable bit0: signal a(or U in hal) enable such as: 01001: use posedge A 11010: use both edge of signal B 11111: use both edge of all HAL siganls.",
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
                    name: "uvw_pos_opt0",
                    description: Some(
                        "set to output next area position for QEO use; clr to output exact point position for MMC use.",
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
                    name: "speed_dir_chg_en",
                    description: Some(
                        "clear counter if detect direction change.",
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
            name: "Readen",
            extends: None,
            description: Some(
                "Read event enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faultfen",
                    description: Some(
                        "No description available.",
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
                    name: "home2fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse1fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse0fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle1fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle0fen",
                    description: Some(
                        "No description available.",
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
                    name: "dirchgfen",
                    description: Some(
                        "No description available.",
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
                    name: "pos2cmpfen",
                    description: Some(
                        "No description available.",
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
                    name: "widthtmfen",
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
                    name: "zmissfen",
                    description: Some(
                        "No description available.",
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
                    name: "zphfen",
                    description: Some(
                        "1- load counters to their read registers when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- load counters to their read registers when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- load counters to their read registers when homef flag set.",
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
                    name: "wdgfen",
                    description: Some(
                        "1- load counters to their read registers when wdg flag set.",
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
            name: "Spd",
            extends: None,
            description: Some(
                "Speed counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdcnt",
                    description: Some(
                        "spdcnt value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bstat",
                    description: Some(
                        "1- b input is high 0- b input is low.",
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
                    name: "astat",
                    description: Some(
                        "1- a input is high 0- a input is low.",
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
                    name: "dir",
                    description: Some(
                        "1- reverse rotation 0- forward rotation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Spdcmp",
            extends: None,
            description: Some(
                "Speed comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdcmp",
                    description: Some(
                        "spdcnt position compare value.",
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
            name: "Spdcmp2",
            extends: None,
            description: Some(
                "Speed comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdcmp2",
                    description: Some(
                        "No description available.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faultf",
                    description: Some(
                        "No description available.",
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
                    name: "home2f",
                    description: Some(
                        "No description available.",
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
                    name: "pulse1f",
                    description: Some(
                        "No description available.",
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
                    name: "pulse0f",
                    description: Some(
                        "No description available.",
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
                    name: "cycle1f",
                    description: Some(
                        "No description available.",
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
                    name: "cycle0f",
                    description: Some(
                        "No description available.",
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
                    name: "dirchgf",
                    description: Some(
                        "No description available.",
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
                    name: "pos2cmpf",
                    description: Some(
                        "No description available.",
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
                    name: "widthtmf",
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
                    name: "zmissf",
                    description: Some(
                        "No description available.",
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
                    name: "zphf",
                    description: Some(
                        "z input flag.",
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
                    name: "poscmpf",
                    description: Some(
                        "postion compare match flag.",
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
                    name: "homef",
                    description: Some(
                        "home flag.",
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
                    name: "wdgf",
                    description: Some(
                        "watchdog flag.",
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
            name: "Tmr",
            extends: None,
            description: Some(
                "Timer counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmrcnt",
                    description: Some(
                        "32 bit free run timer.",
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
            name: "Trgoen",
            extends: None,
            description: Some(
                "Tigger output enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "faultfen",
                    description: Some(
                        "No description available.",
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
                    name: "home2fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse1fen",
                    description: Some(
                        "No description available.",
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
                    name: "pulse0fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle1fen",
                    description: Some(
                        "No description available.",
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
                    name: "cycle0fen",
                    description: Some(
                        "No description available.",
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
                    name: "dirchgfen",
                    description: Some(
                        "No description available.",
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
                    name: "pos2cmpfen",
                    description: Some(
                        "No description available.",
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
                    name: "widthtmfen",
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
                    name: "zmissfen",
                    description: Some(
                        "No description available.",
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
                    name: "zphfen",
                    description: Some(
                        "1- enable trigger output when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- enable trigger output when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- enable trigger output when homef flag set.",
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
                    name: "wdgfen",
                    description: Some(
                        "1- enable trigger output when wdg flag set.",
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
            name: "UvwPos",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uvw_pos0",
                    description: Some(
                        "No description available.",
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
            name: "UvwPosCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "w_pos_sel",
                    description: Some(
                        "No description available.",
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
                    name: "v_pos_sel",
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
                    name: "u_pos_sel",
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
                    name: "pos_en",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "Wdgcfg",
            extends: None,
            description: Some(
                "Watchdog configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdgto",
                    description: Some(
                        "watch dog timeout value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdog_cfg",
                    description: Some(
                        "define as stop if phase_cnt change is less than it if 0, then each change of phase_cnt will clear wdog counter; if 2, then phase_cnt change larger than 2 will clear wdog counter.",
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
                Field {
                    name: "wdgen",
                    description: Some(
                        "1- enable wdog counter.",
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
            name: "Z",
            extends: None,
            description: Some(
                "Z counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcnt",
                    description: Some(
                        "zcnt value.",
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
            name: "Zcmp",
            extends: None,
            description: Some(
                "Z comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcmp",
                    description: Some(
                        "zcnt postion compare value.",
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
            name: "Zcmp2",
            extends: None,
            description: Some(
                "Z comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcmp2",
                    description: Some(
                        "No description available.",
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
            name: "Dir",
            description: Some(
                "Rotation direction.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FORWARD",
                    description: Some(
                        "Forward",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REVERSE",
                    description: Some(
                        "Reverse",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FilterMode",
            description: Some(
                "Filter mode.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "BYPASS",
                    description: Some(
                        "Bypass",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BURR",
                    description: Some(
                        "Rapid change mode",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DELAY",
                    description: Some(
                        "Delay filter mode",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PEAK",
                    description: Some(
                        "Stable low mode",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "VALLEY",
                    description: Some(
                        "Stable high mode",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "SpdTmrReadSel",
            description: Some(
                "Read register select.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SPD_TMR",
                    description: Some(
                        "As speed and timer",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "POS_ANGLE",
                    description: Some(
                        "Speed for postion, timer for angle",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WorkMode",
            description: Some(
                "Decoder work mode.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ABZ",
                    description: Some(
                        "ABZ.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PD",
                    description: Some(
                        "PD.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UD",
                    description: Some(
                        "UD.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "UVW",
                    description: Some(
                        "UVW.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SINGLE",
                    description: Some(
                        "Single A.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SIN",
                    description: Some(
                        "Single sin.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SIN_COS",
                    description: Some(
                        "Sin & Cos.",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "ZCntMode",
            description: Some(
                "Z counter inc mode.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_Z_INPUT",
                    description: Some(
                        "Z counter.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_PHASE_COUNT_MAX",
                    description: Some(
                        "Z counter with phase.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
