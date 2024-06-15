use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Butn",
            extends: None,
            description: Some(
                "BUTN.",
            ),
            items: &[
                BlockItem {
                    name: "btn_status",
                    description: Some(
                        "Button status.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BtnStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btn_irq_mask",
                    description: Some(
                        "Button interrupt mask.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BtnIrqMask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "led_intense",
                    description: Some(
                        "Debounce setting.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LedIntense",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BtnIrqMask",
            extends: None,
            description: Some(
                "Button interrupt mask.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pbtn",
                    description: Some(
                        "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
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
                    name: "wbtn",
                    description: Some(
                        "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbtn",
                    description: Some(
                        "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
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
                    name: "pclick",
                    description: Some(
                        "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xpclick",
                    description: Some(
                        "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
                    name: "wclick",
                    description: Some(
                        "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
                    name: "xwclick",
                    description: Some(
                        "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
            name: "BtnStatus",
            extends: None,
            description: Some(
                "Button status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pbtn",
                    description: Some(
                        "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
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
                    name: "wbtn",
                    description: Some(
                        "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbtn",
                    description: Some(
                        "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed.",
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
                    name: "pclick",
                    description: Some(
                        "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xpclick",
                    description: Some(
                        "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
                    name: "wclick",
                    description: Some(
                        "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
                    name: "xwclick",
                    description: Some(
                        "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked.",
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
            name: "LedIntense",
            extends: None,
            description: Some(
                "Debounce setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pled",
                    description: Some(
                        "Pbutton brightness 0.",
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
                    name: "rled",
                    description: Some(
                        "Rbutton brightness 0.",
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
    ],
    enums: &[],
};
