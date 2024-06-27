use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Affiliate",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "Affiliate of Group.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AffiliateValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Affiliate of Group.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AffiliateSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "Affiliate of Group.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AffiliateClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "Affiliate of Group.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AffiliateToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Cpu",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "lp",
                    description: Some(
                        "CPU0 LP control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lock",
                    description: Some(
                        "CPU0 Lock GPR.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 14,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wakeup_status",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
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
                                "WakeupStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wakeup_enable",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WakeupEnable",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Group0",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group0Value",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group0Set",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group0Clear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group0Toggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Group1",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group1Value",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group1Set",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group1Clear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "Group setting.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Group1Toggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Monitor",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "control",
                    description: Some(
                        "Clock measure and monitor control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "current",
                    description: Some(
                        "Clock measure result.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Current",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "low_limit",
                    description: Some(
                        "Clock lower limit.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LowLimit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "high_limit",
                    description: Some(
                        "Clock upper limit.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HighLimit",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Power",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "status",
                    description: Some(
                        "Power Setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lf_wait",
                    description: Some(
                        "Power Setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LfWait",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "off_wait",
                    description: Some(
                        "Power Setting.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OffWait",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Reset",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "control",
                    description: Some(
                        "Reset Setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ResetControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "config",
                    description: Some(
                        "Reset Setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "counter",
                    description: Some(
                        "Reset Setting.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Counter",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Retention",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "value",
                    description: Some(
                        "Retention Contol.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RetentionValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Retention Contol.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RetentionSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clear",
                    description: Some(
                        "Retention Contol.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RetentionClear",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "toggle",
                    description: Some(
                        "Retention Contol.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RetentionToggle",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Sysctl",
            extends: None,
            description: Some(
                "SYSCTL.",
            ),
            items: &[
                BlockItem {
                    name: "resource",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 228,
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
                                "Resource",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "group0",
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
                    byte_offset: 0x800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Group0",
                        },
                    ),
                },
                BlockItem {
                    name: "group1",
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
                    byte_offset: 0x840,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Group1",
                        },
                    ),
                },
                BlockItem {
                    name: "affiliate",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x900,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Affiliate",
                        },
                    ),
                },
                BlockItem {
                    name: "retention",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x920,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Retention",
                        },
                    ),
                },
                BlockItem {
                    name: "power",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Power",
                        },
                    ),
                },
                BlockItem {
                    name: "reset",
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
                    byte_offset: 0x1400,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Reset",
                        },
                    ),
                },
                BlockItem {
                    name: "clock",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 73,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcclk",
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
                    byte_offset: 0x1c00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adcclk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i2sclk",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1c10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I2sclk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "global00",
                    description: Some(
                        "Clock senario.",
                    ),
                    array: None,
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Global00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x2400,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Monitor",
                        },
                    ),
                },
                BlockItem {
                    name: "cpu",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1024,
                            },
                        ),
                    ),
                    byte_offset: 0x2800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Cpu",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adcclk",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux",
                    description: Some(
                        "current mux 0: ana clock N 1: axi clock.",
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
                    name: "preserve",
                    description: Some(
                        "preserve function against global select 0: select global clock setting 1: not select global clock setting.",
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
                    name: "loc_busy",
                    description: Some(
                        "local busy 0: a change is pending for current node 1: current node is changing status.",
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
                    name: "glb_busy",
                    description: Some(
                        "global busy 0: no changes pending to any clock 1: any of nodes is changing status.",
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
            name: "AffiliateClear",
            extends: None,
            description: Some(
                "Affiliate of Group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0.",
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
            name: "AffiliateSet",
            extends: None,
            description: Some(
                "Affiliate of Group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0.",
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
            name: "AffiliateToggle",
            extends: None,
            description: Some(
                "Affiliate of Group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: toggle the result that whether the group is assigned to CPU0 before.",
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
            name: "AffiliateValue",
            extends: None,
            description: Some(
                "Affiliate of Group.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3.",
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
            name: "Clock",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256.",
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
                    name: "mux",
                    description: Some(
                        "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll1_clk0 4:pll1_clk1 5:pll1_clk2 6:pll2_clk0 7:pll2_clk1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "preserve",
                    description: Some(
                        "preserve function against global select 0: select global clock setting 1: not select global clock setting.",
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
                    name: "loc_busy",
                    description: Some(
                        "local busy 0: a change is pending for current node 1: current node is changing status.",
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
                    name: "glb_busy",
                    description: Some(
                        "global busy 0: no changes pending to any clock 1: any of nodes is changing status.",
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
            name: "Config",
            extends: None,
            description: Some(
                "Reset Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "post_wait",
                    description: Some(
                        "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M.",
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
                    name: "rstclk_num",
                    description: Some(
                        "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M.",
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
                    name: "pre_wait",
                    description: Some(
                        "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M.",
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
            ],
        },
        FieldSet {
            name: "Counter",
            extends: None,
            description: Some(
                "Reset Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M.",
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
            ],
        },
        FieldSet {
            name: "Current",
            extends: None,
            description: Some(
                "Clock measure result.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frequency",
                    description: Some(
                        "self updating measure result.",
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
            name: "Global00",
            extends: None,
            description: Some(
                "Clock senario.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux",
                    description: Some(
                        "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7.",
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
            ],
        },
        FieldSet {
            name: "Gpr",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr",
                    description: Some(
                        "register for software to handle resume, can save resume address or status.",
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
            name: "Group0Clear",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed.",
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
            name: "Group0Set",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed.",
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
            name: "Group0Toggle",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before.",
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
            name: "Group0Value",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed.",
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
            name: "Group1Clear",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed.",
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
            name: "Group1Set",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed.",
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
            name: "Group1Toggle",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before.",
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
            name: "Group1Value",
            extends: None,
            description: Some(
                "Group setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed.",
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
            name: "HighLimit",
            extends: None,
            description: Some(
                "Clock upper limit.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frequency",
                    description: Some(
                        "upper frequency.",
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
            name: "I2sclk",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mux",
                    description: Some(
                        "current mux 0: aud clock 0 1: aud clock 1.",
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
                    name: "preserve",
                    description: Some(
                        "preserve function against global select 0: select global clock setting 1: not select global clock setting.",
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
                    name: "loc_busy",
                    description: Some(
                        "local busy 0: a change is pending for current node 1: current node is changing status.",
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
                    name: "glb_busy",
                    description: Some(
                        "global busy 0: no changes pending to any clock 1: any of nodes is changing status.",
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
            name: "LfWait",
            extends: None,
            description: Some(
                "Power Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wait",
                    description: Some(
                        "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz.",
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
            ],
        },
        FieldSet {
            name: "Lock",
            extends: None,
            description: Some(
                "CPU0 Lock GPR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "Lock bit for CPU_LOCK.",
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
                    name: "gpr",
                    description: Some(
                        "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LowLimit",
            extends: None,
            description: Some(
                "Clock lower limit.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frequency",
                    description: Some(
                        "lower frequency.",
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
            name: "Lp",
            extends: None,
            description: Some(
                "CPU0 LP control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved.",
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
                    name: "reset_flag",
                    description: Some(
                        "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened.",
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
                    name: "sleep_flag",
                    description: Some(
                        "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened.",
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
                    name: "wake_flag",
                    description: Some(
                        "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened.",
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
                    name: "exec",
                    description: Some(
                        "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing.",
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
                    name: "wake",
                    description: Some(
                        "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted.",
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
                    name: "halt",
                    description: Some(
                        "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI.",
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
                    name: "wake_cnt",
                    description: Some(
                        "CPU0 wake up counter, counter satuated at 255, write 0x00 to clear.",
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
            name: "MonitorControl",
            extends: None,
            description: Some(
                "Clock measure and monitor control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "selection",
                    description: Some(
                        "clock measurement selection.",
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
                    name: "reference",
                    description: Some(
                        "refrence clock selection, 0: 32k 1: 24M.",
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
                    name: "accuracy",
                    description: Some(
                        "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz.",
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
                    name: "mode",
                    description: Some(
                        "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register.",
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
                    name: "start",
                    description: Some(
                        "start measurement.",
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
                    name: "low",
                    description: Some(
                        "clock frequency lower than lower limit.",
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
                    name: "high",
                    description: Some(
                        "clock frequency higher than upper limit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "div",
                    description: Some(
                        "output divider.",
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
                    name: "outen",
                    description: Some(
                        "enable clock output.",
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
                    name: "div_busy",
                    description: Some(
                        "divider is applying new setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "valid",
                    description: Some(
                        "result is ready for read 0: not ready 1: result is ready.",
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
            name: "OffWait",
            extends: None,
            description: Some(
                "Power Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wait",
                    description: Some(
                        "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz.",
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
            ],
        },
        FieldSet {
            name: "ResetControl",
            extends: None,
            description: Some(
                "Reset Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly.",
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
                    name: "hold",
                    description: Some(
                        "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold.",
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
                    name: "flag_wake",
                    description: Some(
                        "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit.",
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
                    name: "flag",
                    description: Some(
                        "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit.",
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
            name: "Resource",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved.",
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
                    name: "loc_busy",
                    description: Some(
                        "local busy 0: no change is pending for current node 1: current node is changing status.",
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
                    name: "glb_busy",
                    description: Some(
                        "global busy 0: no changes pending to any nodes 1: any of nodes is changing status.",
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
            name: "RetentionClear",
            extends: None,
            description: Some(
                "Retention Contol.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RetentionSet",
            extends: None,
            description: Some(
                "Retention Contol.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RetentionToggle",
            extends: None,
            description: Some(
                "Retention Contol.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RetentionValue",
            extends: None,
            description: Some(
                "Retention Contol.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link",
                    description: Some(
                        "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: cpu1_mem is kept on while cpu0 stop bit05: cpu1_ctx is kept on while cpu0 stop bit06: otn_mem is kept on while cpu0 stop bit07: otn_ctx is kept on while cpu0 stop bit08: xtal_hold is kept on while cpu0 stop bit09: pll0_hold is kept on while cpu0 stop bit10: pll1_hold is kept on while cpu0 stop bit11: pll2_hold is kept on while cpu0 stop.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some(
                "Power Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lf_ack",
                    description: Some(
                        "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off.",
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
                    name: "lf_disable",
                    description: Some(
                        "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off.",
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
                    name: "flag_wake",
                    description: Some(
                        "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit.",
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
                    name: "flag",
                    description: Some(
                        "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit.",
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
            name: "WakeupEnable",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "IRQ wakeup enable.",
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
            name: "WakeupStatus",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status",
                    description: Some(
                        "IRQ values.",
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