use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Trgm",
            extends: None,
            description: Some(
                "TRGM0.",
            ),
            items: &[
                BlockItem {
                    name: "filtcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgocfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 68,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trgocfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfg",
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
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "General Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dmacfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmasrcsel",
                    description: Some(
                        "This field selects one of the DMA requests as the DMA request output.",
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
            name: "Filtcfg",
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
                        "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode.",
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
            name: "Gcr",
            extends: None,
            description: Some(
                "General Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trgopen",
                    description: Some(
                        "The bitfield enable the TRGM outputs.",
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
            ],
        },
        FieldSet {
            name: "Trgocfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigosel",
                    description: Some(
                        "This bitfield selects one of the TRGM inputs as output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "redg2pen",
                    description: Some(
                        "1- The selected input signal rising edge will be convert to an pulse on output.",
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
                    name: "fedg2pen",
                    description: Some(
                        "1- The selected input signal falling edge will be convert to an pulse on output.",
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
                    name: "outinv",
                    description: Some(
                        "1- Invert the output.",
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
            ],
        },
    ],
    enums: &[
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
                    name: "RAPID_CHANGE",
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
                    name: "STABLE_LOW",
                    description: Some(
                        "Stable low mode",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "STABLE_HIGH",
                    description: Some(
                        "Stable high mode",
                    ),
                    value: 7,
                },
            ],
        },
    ],
};
