use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Count",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "w",
                    description: Some(
                        "W counter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "W",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "v",
                    description: Some(
                        "V counter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "V",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "u",
                    description: Some(
                        "U counter.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "U",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmr",
                    description: Some(
                        "Timer counter.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Hall",
            extends: None,
            description: Some(
                "HALL0.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
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
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phcfg",
                    description: Some(
                        "Phase configure register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdgcfg",
                    description: Some(
                        "Watchdog configure register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdgcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uvwcfg",
                    description: Some(
                        "U,V,W configure register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uvwcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgoen",
                    description: Some(
                        "Trigger output enable register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trgoen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "readen",
                    description: Some(
                        "Read event enable register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Readen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaen",
                    description: Some(
                        "DMA enable register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqen",
                    description: Some(
                        "Interrupt request enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "count",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Count",
                        },
                    ),
                },
                BlockItem {
                    name: "his",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x70,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "His",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "His",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "his0",
                    description: Some(
                        "history register 0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "His0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "his1",
                    description: Some(
                        "history register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "His1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rstcnt",
                    description: Some(
                        "set to reset all counter and related snapshots.",
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
                    name: "snapen",
                    description: Some(
                        "1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "read",
                    description: Some(
                        "1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0.",
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
            name: "Dmaen",
            extends: None,
            description: Some(
                "DMA enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wfen",
                    description: Some(
                        "1- generate dma request when w flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vfen",
                    description: Some(
                        "1- generate dma request when v flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufen",
                    description: Some(
                        "1- generate dma request when u flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phdlyen",
                    description: Some(
                        "1- generate dma request when phdly flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phpreen",
                    description: Some(
                        "1- generate dma request when phpre flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phupten",
                    description: Some(
                        "1- generate dma request when phupt flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgen",
                    description: Some(
                        "1- generate dma request when wdg flag set.",
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
            name: "His0",
            extends: None,
            description: Some(
                "history register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uhis0",
                    description: Some(
                        "copy of ucnt when u signal transition from 0 to 1.",
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
            name: "His1",
            extends: None,
            description: Some(
                "history register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uhis1",
                    description: Some(
                        "copy of ucnt when u signal transition from 1 to 0.",
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
            name: "Irqen",
            extends: None,
            description: Some(
                "Interrupt request enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wfie",
                    description: Some(
                        "1- generate interrupt request when w flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vfie",
                    description: Some(
                        "1- generate interrupt request when v flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufie",
                    description: Some(
                        "1- generate interrupt request when u flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phdlyie",
                    description: Some(
                        "1- generate interrupt request when phdly flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phpreie",
                    description: Some(
                        "1- generate interrupt request when phpre flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phuptie",
                    description: Some(
                        "1- generate interrupt request when phupt flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgie",
                    description: Some(
                        "1- generate interrupt request when wdg flag set.",
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
            name: "Phcfg",
            extends: None,
            description: Some(
                "Phase configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlycnt",
                    description: Some(
                        "delay clock cycles number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlysel",
                    description: Some(
                        "This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle.",
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
            name: "Readen",
            extends: None,
            description: Some(
                "Read event enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wfen",
                    description: Some(
                        "1- load counters to their read registers when w flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vfen",
                    description: Some(
                        "1- load counters to their read registers when v flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufen",
                    description: Some(
                        "1- load counters to their read registers when u flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phdlyen",
                    description: Some(
                        "1- load counters to their read registers when phdly flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phpreen",
                    description: Some(
                        "1- load counters to their read registers when phpre flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phupten",
                    description: Some(
                        "1- load counters to their read registers when phupt flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgen",
                    description: Some(
                        "1- load counters to their read registers when wdg flag set.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wf",
                    description: Some(
                        "w flag, will set when w signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vf",
                    description: Some(
                        "v flag, will set when v signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uf",
                    description: Some(
                        "u flag, will set when u signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phdlyf",
                    description: Some(
                        "phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phpref",
                    description: Some(
                        "phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phuptf",
                    description: Some(
                        "phase update flag, will set when any of u, v, w signal toggle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgf",
                    description: Some(
                        "watchdog count timeout flag.",
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
            name: "Tmr",
            extends: None,
            description: Some(
                "Timer counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer",
                    description: Some(
                        "32 bit free run timer.",
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
            name: "Trgoen",
            extends: None,
            description: Some(
                "Trigger output enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wfen",
                    description: Some(
                        "1- enable trigger output when w flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vfen",
                    description: Some(
                        "1- enable trigger output when v flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufen",
                    description: Some(
                        "1- enable trigger output when u flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phdlyen",
                    description: Some(
                        "1- enable trigger output when phdly flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phpreen",
                    description: Some(
                        "1- enable trigger output when phpre flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phupten",
                    description: Some(
                        "1- enable trigger output when phupt flag set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgen",
                    description: Some(
                        "1- enable trigger output when wdg flag set.",
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
            name: "U",
            extends: None,
            description: Some(
                "U counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucnt",
                    description: Some(
                        "ucnt counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wstat",
                    description: Some(
                        "this bit indicate W state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vstat",
                    description: Some(
                        "this bit indicate V state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ustat",
                    description: Some(
                        "this bit indicate U state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dir",
                    description: Some(
                        "1- reverse rotation 0- forward rotation.",
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
            name: "Uvwcfg",
            extends: None,
            description: Some(
                "U,V,W configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "precnt",
                    description: Some(
                        "the clock cycle number which the pre flag will set before the next uvw transition.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "V",
            extends: None,
            description: Some(
                "V counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcnt",
                    description: Some(
                        "vcnt counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "W",
            extends: None,
            description: Some(
                "W counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wcnt",
                    description: Some(
                        "wcnt counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdgcfg",
            extends: None,
            description: Some(
                "Watchdog configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdgto",
                    description: Some(
                        "watch dog timeout value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgen",
                    description: Some(
                        "1- enable wdog counter.",
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
    enums: &[],
};
