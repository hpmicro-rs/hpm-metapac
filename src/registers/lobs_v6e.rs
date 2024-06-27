use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lobs",
            extends: None,
            description: Some(
                "LOBS.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "streamctrl",
                    description: Some(
                        "Stream Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Streamctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptaction",
                    description: Some(
                        "Pre-trigger Action Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptaction",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "startaddr",
                    description: Some(
                        "Start Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Startaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "endaddr",
                    description: Some(
                        "End Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Endaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctsr",
                    description: Some(
                        "Current Trigger State Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccvr",
                    description: Some(
                        "Current Counter Value Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cavr",
                    description: Some(
                        "Current Action Value Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cavr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifostate",
                    description: Some(
                        "Fifo State Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifostate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "finaladdr",
                    description: Some(
                        "Final Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Finaladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grpsela",
                    description: Some(
                        "Group Select Register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grpsela",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grpena",
                    description: Some(
                        "Group Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grpena",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigsela0",
                    description: Some(
                        "Signal Select0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigsela0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigsela1",
                    description: Some(
                        "Signal Select1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigsela1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigena0",
                    description: Some(
                        "Signal Enable0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigena0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigena1",
                    description: Some(
                        "Signal Enable1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigena1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigsel0",
                    description: Some(
                        "Signal Select Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trigctrl0",
                    description: Some(
                        "Trigger Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trigctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nextstate0",
                    description: Some(
                        "Next State Register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nextstate0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "action0",
                    description: Some(
                        "Action Register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Action0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "countcomp0",
                    description: Some(
                        "Signal Mask Register.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Countcomp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigmask0",
                    description: Some(
                        "Counter Compare Register.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigmask0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "compen0",
                    description: Some(
                        "Compare Enable register.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Compen0",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Action0",
            extends: None,
            description: Some(
                "Action Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trace",
                    description: Some(
                        "Trace active. 0b0 Trace is not active. 0b1 Trace is active.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cavr",
            extends: None,
            description: Some(
                "Current Action Value Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trace",
                    description: Some(
                        "Trace active. 0b0 Trace is not active. 0b1 Trace is active.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccvr",
            extends: None,
            description: Some(
                "Current Counter Value Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ccvr",
                    description: Some(
                        "Returns the counter value when the CTSR was last read. If the CTSR has never been read, then the value in the CCVR is undefined.",
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
            name: "Compen0",
            extends: None,
            description: Some(
                "Compare Enable register.",
            ),
            bit_size: 32,
            fields: &[],
        },
        FieldSet {
            name: "Countcomp0",
            extends: None,
            description: Some(
                "Signal Mask Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trace",
                    description: Some(
                        "A value that, when reached in the associated up-counter for this Trigger State, causes a Trigger Counter Comparison match to occur.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "run",
                    description: Some(
                        "Run control. 0 LOBS disabled. Register programming permitted. 1 LOBS enabled.",
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
            name: "Ctsr",
            extends: None,
            description: Some(
                "Current Trigger State Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctsr",
                    description: Some(
                        "Reads current Trigger State. This is a one-hot encoded field. When CTRL.RUN: 0 RAZ. 1 Returns current Trigger State. If FINALSTATE is 1, then the CTSR field gives the Trigger State when FINALSTATE became 1.",
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
                    name: "finalstate",
                    description: Some(
                        "0 LOBS is still tracing. 1 Indicates that the LOBS has stopped advancing Trigger States and stopped trace. FINALSTATE can be set by TRIGCTRL.COUNTBRK reaching the final loop count, or by programming NEXTSTATEto zero.",
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
            name: "Endaddr",
            extends: None,
            description: Some(
                "End Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "End address.",
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
            name: "Fifostate",
            extends: None,
            description: Some(
                "Fifo State Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FIFO empty.",
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
                    name: "full",
                    description: Some(
                        "FIFO full.",
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
            name: "Finaladdr",
            extends: None,
            description: Some(
                "Final Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Final address.",
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
            name: "Grpena",
            extends: None,
            description: Some(
                "Group Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable trace group number0-1.",
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
            ],
        },
        FieldSet {
            name: "Grpsela",
            extends: None,
            description: Some(
                "Group Select Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num0",
                    description: Some(
                        "Select trace group number0.",
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
                    name: "num1",
                    description: Some(
                        "Select trace group number1.",
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
            ],
        },
        FieldSet {
            name: "Nextstate0",
            extends: None,
            description: Some(
                "Next State Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nextstate",
                    description: Some(
                        "Selects the next state to move to after the Trigger Condition has been met in the current state. 0x0 Do not change state. This is the final Trigger State. 0x1 Selects Trigger State 0. 0x2 Selects Trigger State 1. 0x4 Selects Trigger State 2. 0x8 Selects Trigger State 3. 0x10 Selects Trigger State 4.",
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
            name: "Ptaction",
            extends: None,
            description: Some(
                "Pre-trigger Action Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trace",
                    description: Some(
                        "Enables trace.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sigena0",
            extends: None,
            description: Some(
                "Signal Enable0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable trace signal number0-3.",
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
            ],
        },
        FieldSet {
            name: "Sigena1",
            extends: None,
            description: Some(
                "Signal Enable1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable trace signal number0-3.",
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
            ],
        },
        FieldSet {
            name: "Sigmask0",
            extends: None,
            description: Some(
                "Counter Compare Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num0",
                    description: Some(
                        "Select compare signal number0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num1",
                    description: Some(
                        "Select compare signal number1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num2",
                    description: Some(
                        "Select compare signal number2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num3",
                    description: Some(
                        "Select compare signal number3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sigsel0",
            extends: None,
            description: Some(
                "Signal Select Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Selects Signal Group. 0x1 Selects Signal Group 0. 0x2 Selects Signal Group 1. 0x4 Selects Signal Group 2. 0x8 Selects Signal Group 3. 0x10 Selects Signal Group 4. 0x20 Selects Signal Group 5. 0x40 Selects Signal Group 6. 0x80 Selects Signal Group 7. 0x100 Selects Signal Group 8. 0x200 Selects Signal Group 9. 0x400 Selects Signal Group 10. 0x800 Selects Signal Group 11.",
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
            name: "Sigsela0",
            extends: None,
            description: Some(
                "Signal Select0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num0",
                    description: Some(
                        "Select trace signal number0 in group0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num1",
                    description: Some(
                        "Select trace signal number1 in group0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num2",
                    description: Some(
                        "Select trace signal number2 in group0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num3",
                    description: Some(
                        "Select trace signal number3 in group0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sigsela1",
            extends: None,
            description: Some(
                "Signal Select1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num0",
                    description: Some(
                        "Select trace signal number0 in group1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num1",
                    description: Some(
                        "Select trace signal number1 in group1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num2",
                    description: Some(
                        "Select trace signal number2 in group1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "num3",
                    description: Some(
                        "Select trace signal number3 in group1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Startaddr",
            extends: None,
            description: Some(
                "Start Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Start address.",
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
            name: "Streamctrl",
            extends: None,
            description: Some(
                "Stream Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "burst",
                    description: Some(
                        "Burst Cfg 0b011 Incr4 0b101 Incr8 0b111 Incr16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sample",
                    description: Some(
                        "Sample Rate 0b110 Incr4 take one every four 0b101 Incr8 take one every five 0b100 Incr16 take one every six.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clear",
                    description: Some(
                        "FIFO Overflow Clear.",
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
                    name: "sel",
                    description: Some(
                        "Signal Group Select 0 128bit from one group 1 from 2 groups, 4bit in each group.",
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
            ],
        },
        FieldSet {
            name: "Trigctrl0",
            extends: None,
            description: Some(
                "Trigger Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "comp",
                    description: Some(
                        "Trigger Signal Comparison type select. 0b000 Trigger Signal Comparisons disabled. The enabled counters count clocks immediately after the Trigger State has been entered and generate a programmable Output Action and transition to the next Trigger State when the Counter Compare Register count is reached, that is when a Trigger Counter Comparison match occurs. 0b001 Compare type is equal (==). 0b010 Compare type is greater than (>). 0b011 Compare type is greater than or equal (>=). 0b101 Compare type is not equal (!=). 0b110 Compare type is less than (<). 0b111 Compare type is less than or equal (<=).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compsel",
                    description: Some(
                        "Comparison mode. Acts as both a counter enable and a select for the comparison mode. 0b0 Disable counters and select Trigger Signal Comparison mode. 0b1 Enable counters and select Trigger Counter Comparison mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "watchrst",
                    description: Some(
                        "Counter reset. 0b0 Do not reset the counter after a Trigger Signal Comparison match. 0b1 Reset the counter after a Trigger Signal Comparison match The counter acts like an activity watchdog timer, only allowing advancement to the next Trigger State when the Trigger Counter Comparison is reached. The counter is reset by a signal comparison.",
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
                    name: "countsrc",
                    description: Some(
                        "Counter source select. 0b0 Counter is incremented every ELACLK cycle. 0b1 Counter is incremented when Trigger Signal Comparison matches.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trace",
                    description: Some(
                        "Trace capture control. 0b00 Trace is captured when Trigger Signal Comparison succeeds. 0b01 Trace is captured when Trigger Counter Comparison succeeds. 0b10 Trace is captured every ELACLK cycle. 0b11 Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "countclr",
                    description: Some(
                        "Counter clear. 0b0 Do not clear the counter value when moving to a different NEXTSTATE. 0b1 Clear the counter value when moving to a different NEXTSTATE. Note TRIGCTRL.WATCHRST must be 0b0 when using this feature.",
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
                    name: "counterbrk",
                    description: Some(
                        "Loop counter break. The loop counter break uses the Trigger State counter to break loops between Trigger States after a Trigger Counter Comparison. When the counter comparison matches, the Trigger State goes into a final state,which stops trace writes and leaves the output actions at the previous Trigger State ACTION value. 0b0 Normal operation. 0b1 Break Trigger State loop: A counter comparison match causes a transition to the final state, otherwise go to the NEXTSTATE Trigger State as the counter increments.",
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
    enums: &[],
};
