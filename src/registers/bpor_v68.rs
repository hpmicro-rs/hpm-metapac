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
                    name: "por_config",
                    description: Some(
                        "Power on reset config.",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
            ],
        },
    ],
    fieldsets: &[
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
    ],
    enums: &[],
};
