use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Chn",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "aoi_16to8",
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
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aoi16to8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aoi_8to7_00_01",
                    description: Some(
                        "CHN&index0 AOI_16to8_00_01 OR logic cfg.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aoi8to70001",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aoi_8to7_02_03",
                    description: Some(
                        "CHN&index0 AOI_16to8_02_03 OR logic cfg.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aoi8to70203",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aoi_8to7_04_05",
                    description: Some(
                        "CHN&index0 AOI_16to8_04_05 OR logic cfg.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aoi8to70405",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aoi_8to7_06",
                    description: Some(
                        "CHN&index0 AOI_16to8_06 OR logic cfg.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aoi8to706",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filter_2nd",
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
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filter2nd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filter_3rd",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filter3rd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg_ff",
                    description: Some(
                        "CHN&index0 cfg ff.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CfgFf",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pla",
            extends: None,
            description: Some(
                "PLA0.",
            ),
            items: &[
                BlockItem {
                    name: "chn",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 112,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Chn",
                        },
                    ),
                },
                BlockItem {
                    name: "filter_1st_pla_in",
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
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filter1stPlaIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "filter_1st_pla_out",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Filter1stPlaOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chn_cfg_active",
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
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChnCfgActive",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Aoi16to8",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aoi_16to8_0",
                    description: Some(
                        "select value for AOI_16to8_0. 0: 0. 1: 1st_filter_out[0]. 2: ~1st_filter_out[0]. 3: 1.",
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
                    name: "aoi_16to8_1",
                    description: Some(
                        "select value for AOI_16to8_1. 0: 0. 1: 1st_filter_out[1]. 2: ~1st_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_2",
                    description: Some(
                        "select value for AOI_16to8_2. 0: 0. 1: 1st_filter_out[2]. 2: ~1st_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_3",
                    description: Some(
                        "select value for AOI_16to8_3. 0: 0. 1: 1st_filter_out[3]. 2: ~1st_filter_out[3]. 3: 1.",
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
                    name: "aoi_16to8_4",
                    description: Some(
                        "select value for AOI_16to8_4. 0: 0. 1: 1st_filter_out[4]. 2: ~1st_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_5",
                    description: Some(
                        "select value for AOI_16to8_5. 0: 0. 1: 1st_filter_out[5]. 2: ~1st_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_6",
                    description: Some(
                        "select value for AOI_16to8_6. 0: 0. 1: 1st_filter_out[6]. 2: ~1st_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_7",
                    description: Some(
                        "select value for AOI_16to8_7. 0: 0. 1: 1st_filter_out[7]. 2: ~1st_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_8",
                    description: Some(
                        "select value for AOI_16to8_8. 0: 0. 1: 1st_filter_out[8]. 2: ~1st_filter_out[8]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_9",
                    description: Some(
                        "select value for AOI_16to8_9. 0: 0. 1: 1st_filter_out[9]. 2: ~1st_filter_out[9]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_10",
                    description: Some(
                        "select value for AOI_16to8_10. 0: 0. 1: 1st_filter_out[10]. 2: ~1st_filter_out[10]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_11",
                    description: Some(
                        "select value for AOI_16to8_11. 0: 0. 1: 1st_filter_out[11]. 2: ~1st_filter_out[11]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_12",
                    description: Some(
                        "select value for AOI_16to8_12. 0: 0. 1: 1st_filter_out[12]. 2: ~1st_filter_out[12]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_13",
                    description: Some(
                        "select value for AOI_16to8_13. 0: 0. 1: 1st_filter_out[13]. 2: ~1st_filter_out[13]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_14",
                    description: Some(
                        "select value for AOI_16to8_14. 0: 0. 1: 1st_filter_out[14]. 2: ~1st_filter_out[14]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_16to8_15",
                    description: Some(
                        "select value for AOI_16to8_15. 0: 0. 1: 1st_filter_out[15]. 2: ~1st_filter_out[15]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Aoi8to70001",
            extends: None,
            description: Some(
                "CHN&index0 AOI_16to8_00_01 OR logic cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aoi_8to7_00_0",
                    description: Some(
                        "select value for AOI_8to7_00_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
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
                    name: "aoi_8to7_00_1",
                    description: Some(
                        "select value for AOI_8to7_00_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_00_2",
                    description: Some(
                        "select value for AOI_8to7_00_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_00_3",
                    description: Some(
                        "select value for AOI_8to7_00_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
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
                    name: "aoi_8to7_00_4",
                    description: Some(
                        "select value for AOI_8to7_00_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_00_5",
                    description: Some(
                        "select value for AOI_8to7_00_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_00_6",
                    description: Some(
                        "select value for AOI_8to7_00_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_00_7",
                    description: Some(
                        "select value for AOI_8to7_00_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_0",
                    description: Some(
                        "select value for AOI_8to7_01_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_1",
                    description: Some(
                        "select value for AOI_8to7_01_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_2",
                    description: Some(
                        "select value for AOI_8to7_01_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_3",
                    description: Some(
                        "select value for AOI_8to7_01_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_4",
                    description: Some(
                        "select value for AOI_8to7_01_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_5",
                    description: Some(
                        "select value for AOI_8to7_01_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_6",
                    description: Some(
                        "select value for AOI_8to7_01_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_01_7",
                    description: Some(
                        "select value for AOI_8to7_01_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Aoi8to70203",
            extends: None,
            description: Some(
                "CHN&index0 AOI_16to8_02_03 OR logic cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aoi_8to7_02_0",
                    description: Some(
                        "select value for AOI_8to7_02_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
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
                    name: "aoi_8to7_02_1",
                    description: Some(
                        "select value for AOI_8to7_02_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_02_2",
                    description: Some(
                        "select value for AOI_8to7_02_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_02_3",
                    description: Some(
                        "select value for AOI_8to7_02_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
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
                    name: "aoi_8to7_02_4",
                    description: Some(
                        "select value for AOI_8to7_02_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_02_5",
                    description: Some(
                        "select value for AOI_8to7_02_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_02_6",
                    description: Some(
                        "select value for AOI_8to7_02_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_02_7",
                    description: Some(
                        "select value for AOI_8to7_02_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_0",
                    description: Some(
                        "select value for AOI_8to7_03_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_1",
                    description: Some(
                        "select value for AOI_8to7_03_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_2",
                    description: Some(
                        "select value for AOI_8to7_03_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_3",
                    description: Some(
                        "select value for AOI_8to7_03_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_4",
                    description: Some(
                        "select value for AOI_8to7_03_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_5",
                    description: Some(
                        "select value for AOI_8to7_03_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_6",
                    description: Some(
                        "select value for AOI_8to7_03_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_03_7",
                    description: Some(
                        "select value for AOI_8to7_03_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Aoi8to70405",
            extends: None,
            description: Some(
                "CHN&index0 AOI_16to8_04_05 OR logic cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aoi_8to7_04_0",
                    description: Some(
                        "select value for AOI_8to7_04_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
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
                    name: "aoi_8to7_04_1",
                    description: Some(
                        "select value for AOI_8to7_04_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_04_2",
                    description: Some(
                        "select value for AOI_8to7_04_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_04_3",
                    description: Some(
                        "select value for AOI_8to7_04_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
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
                    name: "aoi_8to7_04_4",
                    description: Some(
                        "select value for AOI_8to7_04_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_04_5",
                    description: Some(
                        "select value for AOI_8to7_04_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_04_6",
                    description: Some(
                        "select value for AOI_8to7_04_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_04_7",
                    description: Some(
                        "select value for AOI_8to7_04_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_0",
                    description: Some(
                        "select value for AOI_8to7_05_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_1",
                    description: Some(
                        "select value for AOI_8to7_05_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_2",
                    description: Some(
                        "select value for AOI_8to7_05_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_3",
                    description: Some(
                        "select value for AOI_8to7_05_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_4",
                    description: Some(
                        "select value for AOI_8to7_05_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_5",
                    description: Some(
                        "select value for AOI_8to7_05_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_6",
                    description: Some(
                        "select value for AOI_8to7_05_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_05_7",
                    description: Some(
                        "select value for AOI_8to7_05_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Aoi8to706",
            extends: None,
            description: Some(
                "CHN&index0 AOI_16to8_06 OR logic cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aoi_8to7_06_0",
                    description: Some(
                        "select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out[0]. 2: ~2nd_filter_out[0]. 3: 1.",
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
                    name: "aoi_8to7_06_1",
                    description: Some(
                        "select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out[1]. 2: ~2nd_filter_out[1]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_06_2",
                    description: Some(
                        "select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out[2]. 2: ~2nd_filter_out[2]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_06_3",
                    description: Some(
                        "select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out[3]. 2: ~2nd_filter_out[3]. 3: 1.",
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
                    name: "aoi_8to7_06_4",
                    description: Some(
                        "select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out[4]. 2: ~2nd_filter_out[4]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_06_5",
                    description: Some(
                        "select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out[5]. 2: ~2nd_filter_out[5]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_06_6",
                    description: Some(
                        "select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out[6]. 2: ~2nd_filter_out[6]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aoi_8to7_06_7",
                    description: Some(
                        "select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out[7]. 2: ~2nd_filter_out[7]. 3: 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CfgFf",
            extends: None,
            description: Some(
                "CHN&index0 cfg ff.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sel_cfg_ff_type",
                    description: Some(
                        "cfg_ff type. 0: DFF. 1: 3rd_filter_0. 2: dual-edge DFF. 3: Trigger FF. 4: JK FF. 5. latch. 6: full adder/minus.",
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
                    name: "sel_clk_source",
                    description: Some(
                        "cfg_ff clock source. 0: system clock. 1: use 3rd_filter_2 as clock.",
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
                    name: "sel_adder_minus",
                    description: Some(
                        "0: select adder when cfg_adder_minus active. 1: select minus when cfg_adder_minus active.",
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
                    name: "dis_osc_loop_clamp",
                    description: Some(
                        "disable osc loop clamp. 0: enable osc loop clamp when osc ring active. 1: disable or clean current osc loop clamp.",
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
                    name: "osc_loop_clamp_value",
                    description: Some(
                        "osc loop clamp value when osc ring active. 0: clamp 0. 1: clamp 1.",
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
            name: "ChnCfgActive",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfg_active",
                    description: Some(
                        "write 0xF00D to enable all setting. Otherwire, all setting inactive.",
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
            name: "Filter1stPlaIn",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync_edge_filter_enable",
                    description: Some(
                        "sync and edge detector filter. 0: disable. 1: enable.",
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
                    name: "software_inject",
                    description: Some(
                        "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level.",
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
                    name: "filter_reverse",
                    description: Some(
                        "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse.",
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
                    name: "edge_dect_enable",
                    description: Some(
                        "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active.",
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
                    name: "nege_edge_dect_enable",
                    description: Some(
                        "nege edge detector enable. 0: disable. 1: enable.",
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
                    name: "pose_edge_dect_enable",
                    description: Some(
                        "pose edge detector enable. 0: disable. 1: enable.",
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
                    name: "filter_sync_level",
                    description: Some(
                        "synchroniser level. 0: 2 level sync. 1: 3 level sync.",
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
                    name: "filter_ext_enable",
                    description: Some(
                        "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active.",
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
                    name: "filter_ext_type",
                    description: Some(
                        "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filter_ext_counter",
                    description: Some(
                        "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Filter1stPlaOut",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync_edge_filter_enable",
                    description: Some(
                        "sync and edge detector filter. 0: disable. 1: enable.",
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
                    name: "software_inject",
                    description: Some(
                        "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level.",
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
                    name: "filter_reverse",
                    description: Some(
                        "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse.",
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
                    name: "edge_dect_enable",
                    description: Some(
                        "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active.",
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
                    name: "nege_edge_dect_enable",
                    description: Some(
                        "nege edge detector enable. 0: disable. 1: enable.",
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
                    name: "pose_edge_dect_enable",
                    description: Some(
                        "pose edge detector enable. 0: disable. 1: enable.",
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
                    name: "filter_sync_level",
                    description: Some(
                        "synchroniser level. 0: 2 level sync. 1: 3 level sync.",
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
                    name: "filter_ext_enable",
                    description: Some(
                        "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active.",
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
                    name: "filter_ext_type",
                    description: Some(
                        "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filter_ext_counter",
                    description: Some(
                        "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Filter2nd",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync_edge_filter_enable",
                    description: Some(
                        "sync and edge detector filter. 0: disable. 1: enable.",
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
                    name: "software_inject",
                    description: Some(
                        "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level.",
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
                    name: "filter_reverse",
                    description: Some(
                        "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse.",
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
                    name: "edge_dect_enable",
                    description: Some(
                        "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active.",
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
                    name: "nege_edge_dect_enable",
                    description: Some(
                        "nege edge detector enable. 0: disable. 1: enable.",
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
                    name: "pose_edge_dect_enable",
                    description: Some(
                        "pose edge detector enable. 0: disable. 1: enable.",
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
                    name: "filter_sync_level",
                    description: Some(
                        "synchroniser level. 0: 2 level sync. 1: 3 level sync.",
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
                    name: "filter_ext_enable",
                    description: Some(
                        "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active.",
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
                    name: "filter_ext_type",
                    description: Some(
                        "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filter_ext_counter",
                    description: Some(
                        "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Filter3rd",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync_edge_filter_enable",
                    description: Some(
                        "sync and edge detector filter. 0: disable. 1: enable.",
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
                    name: "software_inject",
                    description: Some(
                        "software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level.",
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
                    name: "filter_reverse",
                    description: Some(
                        "reverse sync and edge detector filter's output. 0: not reverse. 1: reverse.",
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
                    name: "edge_dect_enable",
                    description: Some(
                        "edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active.",
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
                    name: "nege_edge_dect_enable",
                    description: Some(
                        "nege edge detector enable. 0: disable. 1: enable.",
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
                    name: "pose_edge_dect_enable",
                    description: Some(
                        "pose edge detector enable. 0: disable. 1: enable.",
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
                    name: "filter_sync_level",
                    description: Some(
                        "synchroniser level. 0: 2 level sync. 1: 3 level sync.",
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
                    name: "filter_ext_enable",
                    description: Some(
                        "filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active.",
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
                    name: "filter_ext_type",
                    description: Some(
                        "filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "filter_ext_counter",
                    description: Some(
                        "filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
