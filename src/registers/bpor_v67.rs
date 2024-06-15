use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Bpor",
            extends: None,
            description: Some(
                "BPOR.",
            ),
            items: &[
                BlockItem {
                    name: "por_cause",
                    description: Some(
                        "Power on cause.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PorCause",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "por_select",
                    description: Some(
                        "Power on select.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PorSelect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "por_config",
                    description: Some(
                        "Power on reset config.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PorConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "por_control",
                    description: Some(
                        "Power down control.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PorControl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "PorCause",
            extends: None,
            description: Some(
                "Power on cause.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cause",
                    description: Some(
                        "Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO.",
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
            name: "PorConfig",
            extends: None,
            description: Some(
                "Power on reset config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "retention",
                    description: Some(
                        "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen.",
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
            name: "PorControl",
            extends: None,
            description: Some(
                "Power down control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1.",
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
            name: "PorSelect",
            extends: None,
            description: Some(
                "Power on select.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "select",
                    description: Some(
                        "Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO.",
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
    ],
    enums: &[],
};
