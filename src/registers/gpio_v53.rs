use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "As",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO interrupt asynchronous value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AsValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO interrupt asynchronous set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AsSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO interrupt asynchronous clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AsClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO interrupt asynchronous toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AsToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Di",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO input value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DiValue",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Do",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO output value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DoValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO output set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DoSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO output clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DoClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO output toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DoToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Gpio",
            extends: None,
            description: Some(
                "FGPIO, GPIO0, PGPIO",
            ),
            items: &[
                BlockItem {
                    name: "di",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Di",
                        },
                    ),
                },
                BlockItem {
                    name: "do_",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Do",
                        },
                    ),
                },
                BlockItem {
                    name: "oe",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Oe",
                        },
                    ),
                },
                BlockItem {
                    name: "if_",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x300,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "If",
                        },
                    ),
                },
                BlockItem {
                    name: "ie",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x400,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ie",
                        },
                    ),
                },
                BlockItem {
                    name: "pl",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x500,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pl",
                        },
                    ),
                },
                BlockItem {
                    name: "tp",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x600,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Tp",
                        },
                    ),
                },
                BlockItem {
                    name: "as_",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x700,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "As",
                        },
                    ),
                },
                BlockItem {
                    name: "pd",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 15,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pd",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Ie",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO interrupt enable value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IeValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO interrupt enable set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IeSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO interrupt enable clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IeClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO interrupt enable toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IeToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "If",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO interrupt flag value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IfValue",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Oe",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO direction value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OeValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO direction set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OeSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO direction clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OeClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO direction toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OeToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pd",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO dual edge interrupt enable value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO dual edge interrupt enable set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO dual edge interrupt enable clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO dual edge interrupt enable toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO interrupt polarity value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PlValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO interrupt polarity set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PlSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO interrupt polarity clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PlClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO interrupt polarity toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PlToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tp",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "GPIO interrupt type value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "GPIO interrupt type set.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "GPIO interrupt type clear.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "GPIO interrupt type toggle.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TpToggle",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AsClear",
            extends: None,
            description: Some(
                "GPIO interrupt asynchronous clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_async",
                    description: Some(
                        "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise.",
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
            name: "AsSet",
            extends: None,
            description: Some(
                "GPIO interrupt asynchronous set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_async",
                    description: Some(
                        "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise.",
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
            name: "AsToggle",
            extends: None,
            description: Some(
                "GPIO interrupt asynchronous toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_async",
                    description: Some(
                        "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise.",
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
            name: "AsValue",
            extends: None,
            description: Some(
                "GPIO interrupt asynchronous value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_async",
                    description: Some(
                        "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise.",
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
            name: "DiValue",
            extends: None,
            description: Some(
                "GPIO input value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "input",
                    description: Some(
                        "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin.",
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
            name: "DoClear",
            extends: None,
            description: Some(
                "GPIO output clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "output",
                    description: Some(
                        "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output.",
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
            name: "DoSet",
            extends: None,
            description: Some(
                "GPIO output set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "output",
                    description: Some(
                        "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output.",
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
            name: "DoToggle",
            extends: None,
            description: Some(
                "GPIO output toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "output",
                    description: Some(
                        "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output.",
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
            name: "DoValue",
            extends: None,
            description: Some(
                "GPIO output value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "output",
                    description: Some(
                        "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output.",
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
            name: "IeClear",
            extends: None,
            description: Some(
                "GPIO interrupt enable clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en",
                    description: Some(
                        "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable.",
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
            name: "IeSet",
            extends: None,
            description: Some(
                "GPIO interrupt enable set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en",
                    description: Some(
                        "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable.",
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
            name: "IeToggle",
            extends: None,
            description: Some(
                "GPIO interrupt enable toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en",
                    description: Some(
                        "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable.",
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
            name: "IeValue",
            extends: None,
            description: Some(
                "GPIO interrupt enable value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_en",
                    description: Some(
                        "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable.",
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
            name: "IfValue",
            extends: None,
            description: Some(
                "GPIO interrupt flag value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_flag",
                    description: Some(
                        "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending.",
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
            name: "OeClear",
            extends: None,
            description: Some(
                "GPIO direction clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "direction",
                    description: Some(
                        "GPIO direction, each bit represents a bus bit 0: input 1: output.",
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
            name: "OeSet",
            extends: None,
            description: Some(
                "GPIO direction set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "direction",
                    description: Some(
                        "GPIO direction, each bit represents a bus bit 0: input 1: output.",
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
            name: "OeToggle",
            extends: None,
            description: Some(
                "GPIO direction toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "direction",
                    description: Some(
                        "GPIO direction, each bit represents a bus bit 0: input 1: output.",
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
            name: "OeValue",
            extends: None,
            description: Some(
                "GPIO direction value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "direction",
                    description: Some(
                        "GPIO direction, each bit represents a bus bit 0: input 1: output.",
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
            name: "PdClear",
            extends: None,
            description: Some(
                "GPIO dual edge interrupt enable clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_dual",
                    description: Some(
                        "GPIO dual edge interrupt enable clear 0: keep original edge interrupt type 1: single edge interrupt enable.",
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
            name: "PdSet",
            extends: None,
            description: Some(
                "GPIO dual edge interrupt enable set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_dual",
                    description: Some(
                        "GPIO dual edge interrupt enable set 0: keep original edge interrupt type 1: dual edge interrupt enable.",
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
            name: "PdToggle",
            extends: None,
            description: Some(
                "GPIO dual edge interrupt enable toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_dual",
                    description: Some(
                        "GPIO dual edge interrupt enable toggle 0: keep original edge interrupt type 1: change original edge interrupt type to another one.",
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
            name: "PdValue",
            extends: None,
            description: Some(
                "GPIO dual edge interrupt enable value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_dual",
                    description: Some(
                        "GPIO dual edge interrupt enable 0: single edge interrupt 1: dual edge interrupt enable.",
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
            name: "PlClear",
            extends: None,
            description: Some(
                "GPIO interrupt polarity clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_pol",
                    description: Some(
                        "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge.",
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
            name: "PlSet",
            extends: None,
            description: Some(
                "GPIO interrupt polarity set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_pol",
                    description: Some(
                        "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge.",
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
            name: "PlToggle",
            extends: None,
            description: Some(
                "GPIO interrupt polarity toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_pol",
                    description: Some(
                        "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge.",
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
            name: "PlValue",
            extends: None,
            description: Some(
                "GPIO interrupt polarity value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_pol",
                    description: Some(
                        "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge.",
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
            name: "TpClear",
            extends: None,
            description: Some(
                "GPIO interrupt type clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_type",
                    description: Some(
                        "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge.",
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
            name: "TpSet",
            extends: None,
            description: Some(
                "GPIO interrupt type set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_type",
                    description: Some(
                        "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge.",
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
            name: "TpToggle",
            extends: None,
            description: Some(
                "GPIO interrupt type toggle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_type",
                    description: Some(
                        "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge.",
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
            name: "TpValue",
            extends: None,
            description: Some(
                "GPIO interrupt type value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_type",
                    description: Some(
                        "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge.",
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
