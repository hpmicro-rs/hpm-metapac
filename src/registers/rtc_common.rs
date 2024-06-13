use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "RTC.",
            ),
            items: &[
                BlockItem {
                    name: "second",
                    description: Some(
                        "Second counter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Second",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "subsec",
                    description: Some(
                        "Sub-second counter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Subsec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sec_snap",
                    description: Some(
                        "Second counter snap shot.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecSnap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sub_snap",
                    description: Some(
                        "Sub-second counter snap shot.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SubSnap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm0",
                    description: Some(
                        "RTC alarm0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alarm0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm0_inc",
                    description: Some(
                        "Alarm0 incremental.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alarm0Inc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm1",
                    description: Some(
                        "RTC alarm1.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alarm1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm1_inc",
                    description: Some(
                        "Alarm1 incremental.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alarm1Inc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm_flag",
                    description: Some(
                        "RTC alarm flag.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlarmFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm_en",
                    description: Some(
                        "RTC alarm enable.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlarmEn",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alarm0",
            extends: None,
            description: Some(
                "RTC alarm0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alarm",
                    description: Some(
                        "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC.",
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
            name: "Alarm0Inc",
            extends: None,
            description: Some(
                "Alarm0 incremental.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "increase",
                    description: Some(
                        "adder when ARLAM0 happen, helps to create periodical alarm.",
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
            name: "Alarm1",
            extends: None,
            description: Some(
                "RTC alarm1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alarm",
                    description: Some(
                        "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC.",
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
            name: "Alarm1Inc",
            extends: None,
            description: Some(
                "Alarm1 incremental.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "increase",
                    description: Some(
                        "adder when ARLAM0 happen, helps to create periodical alarm.",
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
            name: "AlarmEn",
            extends: None,
            description: Some(
                "RTC alarm enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable0",
                    description: Some(
                        "alarm0 mask 0: alarm0 disabled 1: alarm0 enabled.",
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
                    name: "enable1",
                    description: Some(
                        "alarm1 mask 0: alarm1 disabled 1: alarm1 enabled.",
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
            name: "AlarmFlag",
            extends: None,
            description: Some(
                "RTC alarm flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alarm0",
                    description: Some(
                        "alarm0 happen.",
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
                    name: "alarm1",
                    description: Some(
                        "alarm1 happen.",
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
            name: "SecSnap",
            extends: None,
            description: Some(
                "Second counter snap shot.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_snap",
                    description: Some(
                        "second snap shot, write to take snap shot.",
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
            name: "Second",
            extends: None,
            description: Some(
                "Second counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "second",
                    description: Some(
                        "second counter.",
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
            name: "SubSnap",
            extends: None,
            description: Some(
                "Sub-second counter snap shot.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sub_snap",
                    description: Some(
                        "sub second snap shot, write to take snap shot.",
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
            name: "Subsec",
            extends: None,
            description: Some(
                "Sub-second counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "subsec",
                    description: Some(
                        "sub second counter.",
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
