use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ptpc",
            extends: None,
            description: Some(
                "PTPC.",
            ),
            items: &[
                BlockItem {
                    name: "ptpc",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4096,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "PtpcPtpc",
                        },
                    ),
                },
                BlockItem {
                    name: "time_sel",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TimeSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptpc_can_ts_sel",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpcCanTsSel",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "PtpcPtpc",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl0",
                    description: Some(
                        "Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl1",
                    description: Some(
                        "Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timeh",
                    description: Some(
                        "timestamp high.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timeh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timel",
                    description: Some(
                        "timestamp low.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ts_updth",
                    description: Some(
                        "timestamp update high.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsUpdth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ts_updtl",
                    description: Some(
                        "timestamp update low.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsUpdtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addend",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tarh",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tarh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tarl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tarl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pps_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PpsCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "capt_snaph",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CaptSnaph",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "capt_snapl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CaptSnapl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addend",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addend",
                    description: Some(
                        "used in fine update mode only.",
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
            name: "CaptSnaph",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "capt_snap_high",
                    description: Some(
                        "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8.",
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
            name: "CaptSnapl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "capt_snap_low",
                    description: Some(
                        "No description available.",
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
            name: "Ctrl0",
            extends: None,
            description: Some(
                "Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer_enable",
                    description: Some(
                        "No description available.",
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
                    name: "fine_coarse_sel",
                    description: Some(
                        "0: coarse update, ns counter add ss_incr[7:0] each clk 1: fine update, ns counter add ss_incr[7:0] each time addend counter overflow.",
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
                    name: "init_timer",
                    description: Some(
                        "initial timer with ts_updt, pulse, clear after set.",
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
                    name: "update_timer",
                    description: Some(
                        "update timer with +/- ts_updt, pulse, clear after set.",
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
                    name: "comp_en",
                    description: Some(
                        "set to enable compare, will be cleared by HW when compare event triggered.",
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
                    name: "capt_snap_neg_en",
                    description: Some(
                        "No description available.",
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
                    name: "capt_snap_pos_en",
                    description: Some(
                        "set will use posege of input capture signal to latch timestamp value.",
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
                    name: "capt_snap_keep",
                    description: Some(
                        "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event.",
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
                    name: "subsec_digital_rollover",
                    description: Some(
                        "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF.",
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
            ],
        },
        FieldSet {
            name: "Ctrl1",
            extends: None,
            description: Some(
                "Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ss_incr",
                    description: Some(
                        "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "capture_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "comp_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "pps_int_sts1",
                    description: Some(
                        "No description available.",
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
                    name: "capture_int_sts1",
                    description: Some(
                        "No description available.",
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
                    name: "comp_int_sts1",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "IntSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "capture_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "comp_int_sts0",
                    description: Some(
                        "No description available.",
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
                    name: "pps_int_sts1",
                    description: Some(
                        "No description available.",
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
                    name: "capture_int_sts1",
                    description: Some(
                        "No description available.",
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
                    name: "comp_int_sts1",
                    description: Some(
                        "No description available.",
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
            ],
        },
        FieldSet {
            name: "PpsCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_ctrl",
                    description: Some(
                        "No description available.",
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
            name: "PtpcCanTsSel",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsu_tbin0_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsu_tbin1_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsu_tbin2_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsu_tbin3_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tarh",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "target_time_high",
                    description: Some(
                        "used for generate compare signal if enabled.",
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
            name: "Tarl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "target_time_low",
                    description: Some(
                        "No description available.",
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
            name: "TimeSel",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "can0_time_sel",
                    description: Some(
                        "set to use ptpc1 for canx clr to use ptpc0 for canx.",
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
                    name: "can1_time_sel",
                    description: Some(
                        "No description available.",
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
                    name: "can2_time_sel",
                    description: Some(
                        "No description available.",
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
                    name: "can3_time_sel",
                    description: Some(
                        "No description available.",
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
            name: "Timeh",
            extends: None,
            description: Some(
                "timestamp high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timestamp_high",
                    description: Some(
                        "No description available.",
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
            name: "Timel",
            extends: None,
            description: Some(
                "timestamp low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timestamp_low",
                    description: Some(
                        "No description available.",
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
            name: "TsUpdth",
            extends: None,
            description: Some(
                "timestamp update high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_update",
                    description: Some(
                        "together with ts_updtl, used to initial or update timestamp.",
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
            name: "TsUpdtl",
            extends: None,
            description: Some(
                "timestamp update low.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ns_update",
                    description: Some(
                        "No description available.",
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
                    name: "add_sub",
                    description: Some(
                        "1 for sub; 0 for add, used only at update.",
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
