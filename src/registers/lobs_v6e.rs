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
                    name: "sigsela1",
                    description: Some(
                        "Signal Select1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
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
                    name: "sigsela2",
                    description: Some(
                        "Signal Select2 Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigsela2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigena",
                    description: Some(
                        "Signal Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigena",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "state",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "State",
                        },
                    ),
                },
                BlockItem {
                    name: "lar",
                    description: Some(
                        "Lock Access Register.",
                    ),
                    array: None,
                    byte_offset: 0xfb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lar",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "State",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "sigsel",
                    description: Some(
                        "Signal Select Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trigctrl",
                    description: Some(
                        "Trigger Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trigctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nextstate",
                    description: Some(
                        "Next State Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nextstate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "action",
                    description: Some(
                        "Action Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Action",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "countcomp",
                    description: Some(
                        "Counter Compare Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Countcomp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extmask",
                    description: Some(
                        "External Mask Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extmask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extcomp",
                    description: Some(
                        "External Compare Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extcomp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigmask",
                    description: Some(
                        "Signal Mask Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigmask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "compen",
                    description: Some(
                        "Compare Enable register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Compen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigcomp0",
                    description: Some(
                        "Signal Compare Register0.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigcomp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigcomp1",
                    description: Some(
                        "Signal Compare Register1.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigcomp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigcomp2",
                    description: Some(
                        "Signal Compare Register2.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigcomp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sigcomp3",
                    description: Some(
                        "Signal Compare Register3.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sigcomp3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Action",
            extends: None,
            description: Some(
                "Action Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trace",
                    description: Some(
                        "Trace active. 0b0 Trace disable. 0b1 Trace enable.",
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
            name: "Compen",
            extends: None,
            description: Some(
                "Compare Enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Select compare signal number0-3.",
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
            name: "Countcomp",
            extends: None,
            description: Some(
                "Counter Compare Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
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
            name: "Extcomp",
            extends: None,
            description: Some(
                "External Compare Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "External Compare.",
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
            name: "Extmask",
            extends: None,
            description: Some(
                "External Mask Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "External Mask.",
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
                    name: "en1",
                    description: Some(
                        "Enable sample group number1.",
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
                    name: "en2",
                    description: Some(
                        "Enable sample group number2.",
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
            name: "Grpsela",
            extends: None,
            description: Some(
                "Group Select Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num1",
                    description: Some(
                        "Select sample group number1.",
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
                    name: "num2",
                    description: Some(
                        "Select sample group number2.",
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
            name: "Lar",
            extends: None,
            description: Some(
                "Lock Access Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Lock Access Value.",
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
            name: "Nextstate",
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
            name: "Sigcomp0",
            extends: None,
            description: Some(
                "Signal Compare Register0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value0",
                    description: Some(
                        "Compare golden value for Signal Group signals[31:0].",
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
            name: "Sigcomp1",
            extends: None,
            description: Some(
                "Signal Compare Register1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value1",
                    description: Some(
                        "Compare golden value for Signal Group signals[63:32].",
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
            name: "Sigcomp2",
            extends: None,
            description: Some(
                "Signal Compare Register2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value2",
                    description: Some(
                        "Compare golden value for Signal Group signals[95:64].",
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
            name: "Sigcomp3",
            extends: None,
            description: Some(
                "Signal Compare Register3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value3",
                    description: Some(
                        "Compare golden value for Signal Group signals[127:96].",
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
            name: "Sigena",
            extends: None,
            description: Some(
                "Signal Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en1",
                    description: Some(
                        "Enable sample signal number1.",
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
                    name: "en2",
                    description: Some(
                        "Enable sample signal number2.",
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
            name: "Sigmask",
            extends: None,
            description: Some(
                "Signal Mask Register.",
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
            name: "Sigsel",
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
            name: "Sigsela1",
            extends: None,
            description: Some(
                "Signal Select1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num1",
                    description: Some(
                        "Select sample signal bit number1 in first group.",
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
                    name: "num2",
                    description: Some(
                        "Select sample signal bit number2 in first group.",
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
                    name: "num3",
                    description: Some(
                        "Select sample signal bit number3 in first group.",
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
                    name: "num4",
                    description: Some(
                        "Select sample signal bit number4 in first group.",
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
            name: "Sigsela2",
            extends: None,
            description: Some(
                "Signal Select2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num1",
                    description: Some(
                        "Select sample signal bit number1 in second group.",
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
                    name: "num2",
                    description: Some(
                        "Select sample signal bit number2 in second group.",
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
                    name: "num3",
                    description: Some(
                        "Select sample signal bit number3 in second group.",
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
                    name: "num4",
                    description: Some(
                        "Select sample signal bit number4 in second group.",
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
                        "Burst Cfg 3b011 Incr4 3b101 Incr8 3b111 Incr16.",
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
                        "Sample Rate 4 take one every 5 5 take one every 6 6 take one every 7.",
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
                    name: "full_clear",
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
            name: "Trigctrl",
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
                    name: "trace",
                    description: Some(
                        "Trace capture control. 0b10 Trace is captured every ELACLK cycle. others Reserved.",
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
            ],
        },
    ],
    enums: &[],
};
