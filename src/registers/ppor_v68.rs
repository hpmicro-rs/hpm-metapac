use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ppor",
            extends: None,
            description: Some(
                "PPOR.",
            ),
            items: &[
                BlockItem {
                    name: "reset_flag",
                    description: Some(
                        "flag indicate reset source.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_status",
                    description: Some(
                        "reset source status.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_hold",
                    description: Some(
                        "reset hold attribute.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetHold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_enable",
                    description: Some(
                        "reset source enable.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetEnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "software_reset",
                    description: Some(
                        "Software reset counter.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SoftwareReset",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ResetEnable",
            extends: None,
            description: Some(
                "reset source enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "enable of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software.",
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
            name: "ResetFlag",
            extends: None,
            description: Some(
                "flag indicate reset source.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flag",
                    description: Some(
                        "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software.",
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
            name: "ResetHold",
            extends: None,
            description: Some(
                "reset hold attribute.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hold",
                    description: Some(
                        "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software.",
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
            name: "ResetStatus",
            extends: None,
            description: Some(
                "reset source status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "current status of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software.",
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
            name: "SoftwareReset",
            extends: None,
            description: Some(
                "Software reset counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset.",
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
