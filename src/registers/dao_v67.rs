use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dao",
            extends: None,
            description: Some(
                "DAO.",
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
                    name: "cmd",
                    description: Some(
                        "Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "rx_cfgr",
                    description: Some(
                        "Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxCfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxslt",
                    description: Some(
                        "RX Slot Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxslt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpf_ma",
                    description: Some(
                        "HPF A Coef Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpfMa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpf_b",
                    description: Some(
                        "HPF B Coef Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpfB",
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
                    name: "run",
                    description: Some(
                        "Enable this module to run.",
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
                    name: "sftrst",
                    description: Some(
                        "Self-clear.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "false_run",
                    description: Some(
                        "the module continues to consume data, but all the pads are constant, thus no audio out.",
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
                    name: "false_level",
                    description: Some(
                        "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "invert",
                    description: Some(
                        "all the outputs are inverted before sending to pad.",
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
                    name: "remap",
                    description: Some(
                        "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version.",
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
                    name: "left_en",
                    description: Some(
                        "Asserted to enable the left channel.",
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
                    name: "right_en",
                    description: Some(
                        "Asserted to enable the right channel.",
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
                    name: "mono",
                    description: Some(
                        "Asserted to let the left and right channel output the same value.",
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
                    name: "hpf_en",
                    description: Some(
                        "Whether HPF is enabled. This HPF is used to filter out the DC part.",
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
            ],
        },
        FieldSet {
            name: "HpfB",
            extends: None,
            description: Some(
                "HPF B Coef Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef",
                    description: Some(
                        "coef B of the Order-1 HPF.",
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
            name: "HpfMa",
            extends: None,
            description: Some(
                "HPF A Coef Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "coef",
                    description: Some(
                        "Composite value of coef A of the Order-1 HPF.",
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
            name: "RxCfgr",
            extends: None,
            description: Some(
                "Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch_max",
                    description: Some(
                        "CH_MAX[3:0] is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX[0] is always 0. 4'h2: 2 channels 4'h4: 4 channels etc.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxslt",
            extends: None,
            description: Some(
                "RX Slot Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Slot enable for the channels.",
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
