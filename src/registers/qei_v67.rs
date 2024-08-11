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
                    name: "z",
                    description: Some(
                        "Z counter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Z",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ph",
                    description: Some(
                        "Phase counter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ph",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd",
                    description: Some(
                        "Speed counter.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spd",
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
            name: "Qei",
            extends: None,
            description: Some(
                "QEI0.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Control register.",
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
                    name: "phidx",
                    description: Some(
                        "Phase index register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phidx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trgoen",
                    description: Some(
                        "Tigger output enable register.",
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
                    name: "zcmp",
                    description: Some(
                        "Z comparator.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phcmp",
                    description: Some(
                        "Phase comparator.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Phcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spdcmp",
                    description: Some(
                        "Speed comparator.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spdcmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaen",
                    description: Some(
                        "DMA request enable register.",
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
                        "Interrupt request register.",
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
                    name: "spdhis",
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
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spdhis",
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
                "Control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enctyp",
                    description: Some(
                        "00-abz; 01-pd; 10-ud; 11-reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "WorkMode",
                    ),
                },
                Field {
                    name: "rstcnt",
                    description: Some(
                        "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx.",
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
                        "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert.",
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
                    name: "hfdir0",
                    description: Some(
                        "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction).",
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
                    name: "hfdir1",
                    description: Some(
                        "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction).",
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
                Field {
                    name: "hrdir0",
                    description: Some(
                        "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hrdir1",
                    description: Some(
                        "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction).",
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
                    name: "pausez",
                    description: Some(
                        "1- pause zcnt when PAUSE assert.",
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
                    name: "pauseph",
                    description: Some(
                        "1- pause phcnt when PAUSE assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pausespd",
                    description: Some(
                        "1- pause spdcnt when PAUSE assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hrstz",
                    description: Some(
                        "1- reset zcnt when H assert.",
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
                    name: "hrstph",
                    description: Some(
                        "1- reset phcnt when H assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hrstspd",
                    description: Some(
                        "1- reset spdcnt when H assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "read",
                    description: Some(
                        "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0.",
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
                "DMA request enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zphfen",
                    description: Some(
                        "1- generate dma request when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- generate dma request when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- generate dma request when homef flag set.",
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
                    name: "wdgfen",
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
            name: "Irqen",
            extends: None,
            description: Some(
                "Interrupt request register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zphie",
                    description: Some(
                        "1- generate interrupt when zphf flag set.",
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
                    name: "poscmpie",
                    description: Some(
                        "1- generate interrupt when poscmpf flag set.",
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
                    name: "homeie",
                    description: Some(
                        "1- generate interrupt when homef flag set.",
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
                        "1- generate interrupt when wdg flag set.",
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
            name: "Ph",
            extends: None,
            description: Some(
                "Phase counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phcnt",
                    description: Some(
                        "phcnt value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bstat",
                    description: Some(
                        "1- b input is high 0- b input is low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "astat",
                    description: Some(
                        "1- a input is high 0- a input is low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
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
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
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
                    name: "phmax",
                    description: Some(
                        "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phcaliz",
                    description: Some(
                        "1- phcnt will set to phidx when Z input assert.",
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
                    name: "zcntcfg",
                    description: Some(
                        "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "ZCntMode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Phcmp",
            extends: None,
            description: Some(
                "Phase comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phcmp",
                    description: Some(
                        "phcnt position compare value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dircmp",
                    description: Some(
                        "0- position compare need positive rotation 1- position compare need negative rotation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "dircmpdis",
                    description: Some(
                        "1- postion compare not include rotation direction.",
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
                    name: "zcmpdis",
                    description: Some(
                        "1- postion compare not include zcnt.",
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
            name: "Phidx",
            extends: None,
            description: Some(
                "Phase index register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phidx",
                    description: Some(
                        "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
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
                    name: "zphfen",
                    description: Some(
                        "1- load counters to their read registers when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- load counters to their read registers when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- load counters to their read registers when homef flag set.",
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
                    name: "wdgfen",
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
            name: "Spd",
            extends: None,
            description: Some(
                "Speed counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdcnt",
                    description: Some(
                        "spdcnt value.",
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
                    name: "bstat",
                    description: Some(
                        "1- b input is high 0- b input is low.",
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
                    name: "astat",
                    description: Some(
                        "1- a input is high 0- a input is low.",
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
                    enumm: Some(
                        "Dir",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Spdcmp",
            extends: None,
            description: Some(
                "Speed comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdcmp",
                    description: Some(
                        "spdcnt position compare value.",
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
            name: "Spdhis",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spdhis0",
                    description: Some(
                        "copy of spdcnt, load from spdcnt after any transition from a = low, b = low.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zphf",
                    description: Some(
                        "z input flag.",
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
                    name: "poscmpf",
                    description: Some(
                        "postion compare match flag.",
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
                    name: "homef",
                    description: Some(
                        "home flag.",
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
                        "watchdog flag.",
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
                    name: "tmrcnt",
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
                "Tigger output enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zphfen",
                    description: Some(
                        "1- enable trigger output when zphf flag set.",
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
                    name: "poscmpfen",
                    description: Some(
                        "1- enable trigger output when poscmpf flag set.",
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
                    name: "homefen",
                    description: Some(
                        "1- enable trigger output when homef flag set.",
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
                    name: "wdgfen",
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
        FieldSet {
            name: "Z",
            extends: None,
            description: Some(
                "Z counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcnt",
                    description: Some(
                        "zcnt value.",
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
            name: "Zcmp",
            extends: None,
            description: Some(
                "Z comparator.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zcmp",
                    description: Some(
                        "zcnt postion compare value.",
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
    enums: &[
        Enum {
            name: "Dir",
            description: Some(
                "Rotation direction.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FORWARD",
                    description: Some(
                        "Forward",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REVERSE",
                    description: Some(
                        "Reverse",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "WorkMode",
            description: Some(
                "Decoder work mode.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ABZ",
                    description: Some(
                        "ABZ.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PD",
                    description: Some(
                        "PD mode, Pluse + Direction.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "UD",
                    description: Some(
                        "UD mode, Up pluse + Down pluse.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "ZCntMode",
            description: Some(
                "Z counter inc mode.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ON_Z_INPUT",
                    description: Some(
                        "Z counter.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_PHASE_COUNT_MAX",
                    description: Some(
                        "Z counter with phase.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
