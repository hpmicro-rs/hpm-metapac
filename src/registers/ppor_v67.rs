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
                    name: "reset_hot",
                    description: Some(
                        "reset type triggered by reset.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetHot",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_cold",
                    description: Some(
                        "reset type attribute.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetCold",
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
            name: "ResetCold",
            extends: None,
            description: Some(
                "reset type attribute.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flag",
                    description: Some(
                        "perform cold or warm reset of chip, 0 for warm reset, fuse value and debug connection preserved; 1 for cold reset, fuse value reloaded and debug connection corrupted. This bit is ignored when hot reset selected 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
                        "enable of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
                        "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
                    name: "status",
                    description: Some(
                        "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
            name: "ResetHot",
            extends: None,
            description: Some(
                "reset type triggered by reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "type_",
                    description: Some(
                        "reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
                        "current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software.",
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
