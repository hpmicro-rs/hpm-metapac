use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Assign",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "pin",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
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
                                "Pin",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Gpiom",
            extends: None,
            description: Some(
                "GPIOM.",
            ),
            items: &[
                BlockItem {
                    name: "assign",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Assign",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Pin",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "select",
                    description: Some(
                        "select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "PinSelect",
                    ),
                },
                Field {
                    name: "hide",
                    description: Some(
                        "pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio.",
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
                    name: "lock",
                    description: Some(
                        "lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable.",
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
            name: "PinSelect",
            description: Some(
                "select which gpio controls chip pin",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "GPIO0",
                    description: Some(
                        "soc gpio0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "GPIO1",
                    description: Some(
                        "soc gpio1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CPU0_FGPIO",
                    description: Some(
                        "cpu0 fastgpio",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CPU1_FGPIO",
                    description: Some(
                        "cpu1 fast gpio",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
