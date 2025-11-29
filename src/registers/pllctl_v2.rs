use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pll",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "mfi",
                    description: Some(
                        "PLL0 multiple register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mfi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mfn",
                    description: Some(
                        "PLL0 fraction numerator register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mfn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mfd",
                    description: Some(
                        "PLL0 fraction demoninator register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mfd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ss_step",
                    description: Some(
                        "PLL0 spread spectrum step register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SsStep",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ss_stop",
                    description: Some(
                        "PLL0 spread spectrum stop register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SsStop",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "config",
                    description: Some(
                        "PLL0 confguration register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "locktime",
                    description: Some(
                        "PLL0 lock time register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Locktime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "steptime",
                    description: Some(
                        "PLL0 step time register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Steptime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "advanced",
                    description: Some(
                        "PLL0 advance configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Advanced",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "div",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Div",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pllctlv2",
            extends: None,
            description: Some(
                "PLLCTLV2.",
            ),
            items: &[
                BlockItem {
                    name: "xtal",
                    description: Some(
                        "OSC configuration.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xtal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pll",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Advanced",
            extends: None,
            description: Some(
                "PLL0 advance configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dither",
                    description: Some(
                        "Enable dither function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slow",
                    description: Some(
                        "Use slow lock flow, PLL lock expendite is disabled. This mode might be stabler. And software need config LOCKTIME field accordingly. 0: fast lock enabled, lock time is 100us 1: fast lock disabled, lock time is 400us.",
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
            ],
        },
        FieldSet {
            name: "Config",
            extends: None,
            description: Some(
                "PLL0 confguration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refsel",
                    description: Some(
                        "Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating. 0: XTAL24M 1: IRC24M.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Refsel",
                    ),
                },
                Field {
                    name: "spread",
                    description: Some(
                        "Enable spread spectrum function. This field supports changing during PLL running.",
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
            name: "Div",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Divider factor, divider factor is DIV/5 + 1 0: divide by 1 1: divide by 1.2 2: divide by 1.4 . . . 63: divide by 13.6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Divider enable status 0: Divider is off 1: Divider is on.",
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
                    name: "response",
                    description: Some(
                        "Divider response status 0: Divider is not stable 1: Divider is stable for use.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag 0: divider is working 1: divider is changing status.",
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
            name: "Locktime",
            extends: None,
            description: Some(
                "PLL0 lock time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locktime",
                    description: Some(
                        "Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mfd",
            extends: None,
            description: Some(
                "PLL0 fraction demoninator register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfd",
                    description: Some(
                        "Demoninator of fraction part,f=fref*(mfi + mfn/mfd). This field should not be changed during PLL enabled. If changed, change will take efftect when PLL re-enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mfi",
            extends: None,
            description: Some(
                "PLL0 multiple register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfi",
                    description: Some(
                        "loop back divider of PLL, support from 13 to 42, f=fref*(mfi + mfn/mfd) 0-15: invalid 16: divide by 16 17: divide by17 . . . 42: divide by 42 43~:invalid.",
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
                    name: "enable",
                    description: Some(
                        "PLL enable status 0: PLL is off 1: PLL is on.",
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
                    name: "response",
                    description: Some(
                        "PLL status 0: PLL is not stable 1: PLL is stable for use.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag 0: PLL is stable or shutdown 1: PLL is changing status.",
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
            name: "Mfn",
            extends: None,
            description: Some(
                "PLL0 fraction numerator register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfn",
                    description: Some(
                        "Numeratorof fractional part,f=fref*(mfi + mfn/mfd). This field supports changing while running.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SsStep",
            extends: None,
            description: Some(
                "PLL0 spread spectrum step register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "step",
                    description: Some(
                        "Step of spread spectrum modulator. This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SsStop",
            extends: None,
            description: Some(
                "PLL0 spread spectrum stop register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stop",
                    description: Some(
                        "Stop point of spread spectrum modulator This register should not be changed during PLL and spread spectrum enabled. If changed, new value will take effect when PLL disabled or spread spectrum disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Steptime",
            extends: None,
            description: Some(
                "PLL0 step time register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "steptime",
                    description: Some(
                        "Step time for MFI on-the-fly change in 24M clock cycles, typical value is 2500.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Xtal",
            extends: None,
            description: Some(
                "OSC configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ramp_time",
                    description: Some(
                        "Rampup time of XTAL oscillator in cycles of RC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on.",
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
                    name: "response",
                    description: Some(
                        "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag 0: Oscillator is working or shutdown 1: Oscillator is changing status.",
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
            name: "Refsel",
            description: Some(
                "Select reference clock, This filed support changing while running, but application must take frequency error and jitter into consideration. And if MFN changed before reference switch, application need make sure time is enough for MFN updating.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IRC24M",
                    description: Some(
                        "IRC24M",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "XTAL24M",
                    description: Some(
                        "XTAL24M",
                    ),
                    value: 0,
                },
            ],
        },
    ],
};
