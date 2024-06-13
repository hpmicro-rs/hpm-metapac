use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rng",
            extends: None,
            description: Some(
                "RNG.",
            ),
            items: &[
                BlockItem {
                    name: "cmd",
                    description: Some(
                        "Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "sta",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "err",
                    description: Some(
                        "Error Registers.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Err",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fo2b",
                    description: Some(
                        "FIFO out to bus/cpu.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fo2b",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "r2sk",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R2sk",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmd",
            extends: None,
            description: Some(
                "Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "slfchk",
                    description: Some(
                        "Self Test, when both ST and GS triggered, ST first and GS next.",
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
                    name: "gensd",
                    description: Some(
                        "Generate Seed, when both ST and GS triggered, ST first and GS next.",
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
                Field {
                    name: "clrint",
                    description: Some(
                        "Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt.",
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
                    name: "clrerr",
                    description: Some(
                        "Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt.",
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
                    name: "sftrst",
                    description: Some(
                        "Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
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
                    name: "fufmod",
                    description: Some(
                        "FIFO underflow response mode 00 Return all zeros and set the ESR[FUFE]. 01 Return all zeros and set the ESR[FUFE]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL[MASKERR]).",
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
                Field {
                    name: "autrsd",
                    description: Some(
                        "Auto Reseed.",
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
                    name: "mirqdn",
                    description: Some(
                        "Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA[SDN, STDN]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD[GS,ST]) being set, indicating that the operation is still taking place.",
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
                    name: "mirqerr",
                    description: Some(
                        "Mask Interrupt Request for Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Err",
            extends: None,
            description: Some(
                "Error Registers.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sckerr",
                    description: Some(
                        "Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD[CE].",
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
                    name: "fufe",
                    description: Some(
                        "FIFO access error(underflow).",
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
            ],
        },
        FieldSet {
            name: "Fo2b",
            extends: None,
            description: Some(
                "FIFO out to bus/cpu.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fo2b",
                    description: Some(
                        "SW read the FIFO output.",
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
            name: "R2sk",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fo2s0",
                    description: Some(
                        "FIFO out to KMAN, will be SDP engine key.",
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
            name: "Sta",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "when 1, means the RNG engine is busy for seeding or random number generation, self test and so on.",
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
                Field {
                    name: "idle",
                    description: Some(
                        "Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsdreq",
                    description: Some(
                        "Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD[GS], or automatically if the CTRL[ARS] is set.",
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
                    name: "scdn",
                    description: Some(
                        "Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD[ST]. 0 Self test not completed 1 Completed a self test since the last reset.",
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
                    name: "fsddn",
                    description: Some(
                        "1st Seed done When \"1\", Indicates that the RNG generated the first seed.",
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
                    name: "nsddn",
                    description: Some(
                        "New seed done.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "frnnu",
                    description: Some(
                        "Fifo Level, Indicates the number of random words currently in the output FIFO.",
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
                    name: "fsize",
                    description: Some(
                        "Fifo Size, it is 5 in this design.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "funcerr",
                    description: Some(
                        "Error was detected, check ESR register for details.",
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
                Field {
                    name: "scpf",
                    description: Some(
                        "Self Check Pass Fail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
