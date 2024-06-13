use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pdgo",
            extends: None,
            description: Some(
                "PDGO.",
            ),
            items: &[
                BlockItem {
                    name: "dgo_turnoff",
                    description: Some(
                        "trunoff control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoTurnoff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_rc32k_cfg",
                    description: Some(
                        "RC32K CLOCK.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoRc32kCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_gpr00",
                    description: Some(
                        "Generic control 0.",
                    ),
                    array: None,
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoGpr00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_gpr01",
                    description: Some(
                        "Generic control 1.",
                    ),
                    array: None,
                    byte_offset: 0x604,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoGpr01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_gpr02",
                    description: Some(
                        "Generic control 2.",
                    ),
                    array: None,
                    byte_offset: 0x608,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoGpr02",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_gpr03",
                    description: Some(
                        "Generic control 3.",
                    ),
                    array: None,
                    byte_offset: 0x60c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoGpr03",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_ctr0",
                    description: Some(
                        "control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoCtr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_ctr1",
                    description: Some(
                        "control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoCtr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_ctr2",
                    description: Some(
                        "control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoCtr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_ctr3",
                    description: Some(
                        "control register 3.",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoCtr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dgo_ctr4",
                    description: Some(
                        "control register 4.",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DgoCtr4",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "DgoCtr0",
            extends: None,
            description: Some(
                "control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "retention",
                    description: Some(
                        "dgo register status retenion.",
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
            name: "DgoCtr1",
            extends: None,
            description: Some(
                "control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pin_wakeup_status",
                    description: Some(
                        "wakeup pin status.",
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
                    name: "wakeup_en",
                    description: Some(
                        "permit wakeup pin or software wakeup.",
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
                    name: "aoto_sys_wakeup",
                    description: Some(
                        "software wakeup： 0 : wakeup once； 1：auto wakeup Continuously.",
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
            name: "DgoCtr2",
            extends: None,
            description: Some(
                "control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wakeup_pulldn_disable",
                    description: Some(
                        "wakeup pin pull down disable.",
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
                    name: "resetn_pullup_disable",
                    description: Some(
                        "resetn pin pull up disable.",
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
            name: "DgoCtr3",
            extends: None,
            description: Some(
                "control register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wakeup_counter",
                    description: Some(
                        "software wakeup counter.",
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
            name: "DgoCtr4",
            extends: None,
            description: Some(
                "control register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bandgap_lp_mode",
                    description: Some(
                        "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode.",
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
                    name: "bandgap_less_power",
                    description: Some(
                        "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode.",
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
            ],
        },
        FieldSet {
            name: "DgoGpr00",
            extends: None,
            description: Some(
                "Generic control 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr",
                    description: Some(
                        "Generic control.",
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
            name: "DgoGpr01",
            extends: None,
            description: Some(
                "Generic control 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr",
                    description: Some(
                        "Generic control.",
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
            name: "DgoGpr02",
            extends: None,
            description: Some(
                "Generic control 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr",
                    description: Some(
                        "Generic control.",
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
            name: "DgoGpr03",
            extends: None,
            description: Some(
                "Generic control 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr",
                    description: Some(
                        "Generic control.",
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
            name: "DgoRc32kCfg",
            extends: None,
            description: Some(
                "RC32K CLOCK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cap_trim",
                    description: Some(
                        "capacitor trim bits.",
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
                    name: "capex6_trim",
                    description: Some(
                        "IRC32K bit 6.",
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
                    name: "capex7_trim",
                    description: Some(
                        "IRC32K bit 7.",
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
                    name: "irc_trimmed",
                    description: Some(
                        "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed.",
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
            name: "DgoTurnoff",
            extends: None,
            description: Some(
                "trunoff control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1.",
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
