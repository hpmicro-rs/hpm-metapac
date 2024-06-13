use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tsns",
            extends: None,
            description: Some(
                "TSNS.",
            ),
            items: &[
                BlockItem {
                    name: "t",
                    description: Some(
                        "Temperature.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "T",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmax",
                    description: Some(
                        "Maximum Temperature.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmin",
                    description: Some(
                        "Minimum Temperature.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "age",
                    description: Some(
                        "Sample age.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Age",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "Status.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "config",
                    description: Some(
                        "Configuration.",
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
                    name: "validity",
                    description: Some(
                        "Sample validity.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Validity",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flag",
                    description: Some(
                        "Temperature flag.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "upper_lim_irq",
                    description: Some(
                        "Maximum temperature to interrupt.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UpperLimIrq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lower_lim_irq",
                    description: Some(
                        "Minimum temperature to interrupt.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LowerLimIrq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "upper_lim_rst",
                    description: Some(
                        "Maximum temperature to reset.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UpperLimRst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lower_lim_rst",
                    description: Some(
                        "Minimum temperature to reset.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LowerLimRst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "async_",
                    description: Some(
                        "Configuration in asynchronous mode.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Async",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "advan",
                    description: Some(
                        "Advance configuration.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Advan",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Advan",
            extends: None,
            description: Some(
                "Advance configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_only",
                    description: Some(
                        "use positive compare polarity only.",
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
                    name: "neg_only",
                    description: Some(
                        "use negative compare polarity only.",
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
                    name: "sampling",
                    description: Some(
                        "temperature sampling is working.",
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
                    name: "active_irq",
                    description: Some(
                        "interrupt status of active mode.",
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
                    name: "async_irq",
                    description: Some(
                        "interrupt status of asynchronous mode.",
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
            ],
        },
        FieldSet {
            name: "Age",
            extends: None,
            description: Some(
                "Sample age.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "age",
                    description: Some(
                        "age of T register in 24MHz clock cycles.",
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
            name: "Async",
            extends: None,
            description: Some(
                "Configuration in asynchronous mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Value of async mode to compare.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "polarity",
                    description: Some(
                        "Polarity of internal comparator.",
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
                    name: "async_type",
                    description: Some(
                        "Compare hotter than or colder than in asynchoronous mode 0: hotter than 1: colder than.",
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
            ],
        },
        FieldSet {
            name: "Config",
            extends: None,
            description: Some(
                "Configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Enable temperature 0: disable, temperature sensor is shut down 1: enable. Temperature sensor enabled.",
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
                    name: "async_",
                    description: Some(
                        "Acynchronous mode, this mode can work without clock, only available function ios compare to certain ADC value 0: active mode 1: Async mode.",
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
                    name: "continuous",
                    description: Some(
                        "continuous mode that keep sampling temperature peridically 0: trigger mode 1: continuous mode.",
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
                    name: "average",
                    description: Some(
                        "Average time, default in 3 0: measure and return 1: twice and average 2: 4 times and average . . . 7: 128 times and average.",
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
                    name: "speed",
                    description: Some(
                        "cycles of a progressive step in 24M clock, valid from 24-255, default 96 24: 24 cycle for a step 25: 25 cycle for a step 26: 26 cycle for a step ... 255: 255 cycle for a step.",
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
                    name: "compare_max_en",
                    description: Some(
                        "Enable compare for maximum temperature.",
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
                    name: "compare_min_en",
                    description: Some(
                        "Enable compare for minimum temperature.",
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
                    name: "rst_en",
                    description: Some(
                        "Enable reset.",
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
                    name: "irq_en",
                    description: Some(
                        "Enable interrupt.",
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
            name: "Flag",
            extends: None,
            description: Some(
                "Temperature flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq",
                    description: Some(
                        "IRQ flag, write 1 to clear.",
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
                    name: "over_temp",
                    description: Some(
                        "Clear over temperature status, write 1 to clear.",
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
                    name: "under_temp",
                    description: Some(
                        "Clear under temperature status, write 1 to clear.",
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
                    name: "record_max_clr",
                    description: Some(
                        "Clear maximum recorder of temerature, write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "record_min_clr",
                    description: Some(
                        "Clear minimum recorder of temerature, write 1 to clear.",
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
            ],
        },
        FieldSet {
            name: "LowerLimIrq",
            extends: None,
            description: Some(
                "Minimum temperature to interrupt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "Minimum temperature for compare.",
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
            name: "LowerLimRst",
            extends: None,
            description: Some(
                "Minimum temperature to reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "Minimum temperature for compare.",
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
            name: "Status",
            extends: None,
            description: Some(
                "Status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigger",
                    description: Some(
                        "Software trigger for sensing in trigger mode, trigger will be ignored if in sensing or other mode.",
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
                    name: "valid",
                    description: Some(
                        "indicate value in T is valid or not 0: not valid 1:valid.",
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
            name: "T",
            extends: None,
            description: Some(
                "Temperature.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "Signed number of temperature in 256 x celsius degree.",
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
            name: "Tmax",
            extends: None,
            description: Some(
                "Maximum Temperature.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "maximum temperature ever found.",
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
            name: "Tmin",
            extends: None,
            description: Some(
                "Minimum Temperature.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "minimum temperature ever found.",
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
            name: "UpperLimIrq",
            extends: None,
            description: Some(
                "Maximum temperature to interrupt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "Maximum temperature for compare.",
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
            name: "UpperLimRst",
            extends: None,
            description: Some(
                "Maximum temperature to reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t",
                    description: Some(
                        "Maximum temperature for compare.",
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
            name: "Validity",
            extends: None,
            description: Some(
                "Sample validity.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "validity",
                    description: Some(
                        "time for temperature values to expire in 24M clock cycles.",
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
