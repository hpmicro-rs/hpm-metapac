use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Bcfg",
            extends: None,
            description: Some(
                "BCFG.",
            ),
            items: &[
                BlockItem {
                    name: "vbg_cfg",
                    description: Some(
                        "Bandgap config.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VbgCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irc32k_cfg",
                    description: Some(
                        "On-chip 32k oscillator config.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irc32kCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xtal32k_cfg",
                    description: Some(
                        "XTAL 32K config.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xtal32kCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clk_cfg",
                    description: Some(
                        "Clock config.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClkCfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ClkCfg",
            extends: None,
            description: Some(
                "Clock config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_xtal",
                    description: Some(
                        "force switch to crystal.",
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
                    name: "keep_irc",
                    description: Some(
                        "force irc32k run.",
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
                    name: "xtal_sel",
                    description: Some(
                        "crystal selected.",
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
            name: "Irc32kCfg",
            extends: None,
            description: Some(
                "On-chip 32k oscillator config.",
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
            name: "VbgCfg",
            extends: None,
            description: Some(
                "Bandgap config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbg_p50",
                    description: Some(
                        "Bandgap 0.50V output trim.",
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
                    name: "vbg_p65",
                    description: Some(
                        "Bandgap 0.65V output trim.",
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
                    name: "vbg_1p0",
                    description: Some(
                        "Bandgap 1.0V output trim.",
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
                    name: "power_save",
                    description: Some(
                        "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode.",
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
                    name: "vbg_trimmed",
                    description: Some(
                        "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed.",
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
            name: "Xtal32kCfg",
            extends: None,
            description: Some(
                "XTAL 32K config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amp",
                    description: Some(
                        "crystal 32k amplifier.",
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
                    name: "cfg",
                    description: Some(
                        "crystal 32k config.",
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
                    name: "gmsel",
                    description: Some(
                        "crystal 32k gm selection.",
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
                    name: "hyst_en",
                    description: Some(
                        "crystal 32k hysteres enable.",
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
    ],
    enums: &[],
};
