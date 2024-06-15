use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "PixelMux",
            extends: None,
            description: Some(
                "PIXEL_MUX.",
            ),
            items: &[
                BlockItem {
                    name: "pixmux",
                    description: Some(
                        "pixel path mux register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pixmux",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dsi_setting",
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
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DsiSetting",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misc",
                    description: Some(
                        "common register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Misc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d0",
                    description: Some(
                        "gpr write-read register 0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d1",
                    description: Some(
                        "gpr write-read register 1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d2",
                    description: Some(
                        "gpr write-read register 2.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d3",
                    description: Some(
                        "gpr write-read register 3.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d4",
                    description: Some(
                        "gpr write-read register 4.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d5",
                    description: Some(
                        "gpr write-read register 5.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d6",
                    description: Some(
                        "gpr write-read register 6.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d7",
                    description: Some(
                        "gpr write-read register 7.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d8",
                    description: Some(
                        "gpr write-read register 8.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr_d9",
                    description: Some(
                        "gpr write-read register 9.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWrD9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d0",
                    description: Some(
                        "gpr read-only register 0.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d1",
                    description: Some(
                        "gpr read-only register 1.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d2",
                    description: Some(
                        "gpr read-only register 2.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d3",
                    description: Some(
                        "gpr read-only register 3.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d4",
                    description: Some(
                        "gpr read-only register 4.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d5",
                    description: Some(
                        "gpr read-only register 5.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d6",
                    description: Some(
                        "gpr read-only register 6.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d7",
                    description: Some(
                        "gpr read-only register 7.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d8",
                    description: Some(
                        "gpr read-only register 8.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ro_d9",
                    description: Some(
                        "gpr read-only register 9.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprRoD9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_wr1_clr_d0",
                    description: Some(
                        "gpr write1 set/no-write clr register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprWr1ClrD0",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "DsiSetting",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsi_data_type",
                    description: Some(
                        "DSI input pixel data type: ‘h0: RGB565_CFG1 ‘h1: RGB565_CFG2 ‘h2: RGB565_CFG3 ‘h3: RGB666_CFG1 ‘h4: RGB666_CFG2 ‘h5: RGB888 ‘h6: RGB_10BIT ‘h7: RGB_12BIT, no support ‘h8:YUV422_12BIT,no support ‘h9: YUV422_10BIT, no support ‘ha: YUV422_8BIT, no support ‘hb: YUV420_8BIT,no support ‘hc~’hf: Reserved.",
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
                Field {
                    name: "dsi_data_enable",
                    description: Some(
                        "DSI pixel data type enable: Bit0: RGB565_CFG1 Bit1: RGB565_CFG2 Bit2: RGB565_CFG3 Bit3: RGB666_CFG1 Bit4: RGB666_CFG2 Bit5: RGB888 Bit6: RGB_10BIT Bit7: RGB_12BIT, no support Bit8: YUV422_12BIT, no support Bit9: YUV422_10BIT, no support Bit10: YUV422_8BIT, no support Bit11:YUV420_8BIT,no support others: Reserved.",
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
            name: "GprRoD0",
            extends: None,
            description: Some(
                "gpr read-only register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy0_ctl_o",
                    description: Some(
                        "{2'b0, tx_phy0_tx3_ctl_o,tx_phy0_tx2_ctl_o, tx_phy0_tx1_ctl_o,tx_phy0_tx0_ctl_o, tx_phy0_txck_ctl_o,tx_phy0_pll_dtest_o}.",
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
                    name: "tx_phy1_ctl_o",
                    description: Some(
                        "{2'b0, tx_phy1_tx3_ctl_o,tx_phy1_tx2_ctl_o, tx_phy1_tx1_ctl_o,tx_phy1_tx0_ctl_o, tx_phy1_txck_ctl_o,tx_phy1_pll_dtest_o}.",
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
            ],
        },
        FieldSet {
            name: "GprRoD1",
            extends: None,
            description: Some(
                "gpr read-only register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csi0_sta_ap_if_int_sta",
                    description: Some(
                        "csi0 apb parity check interrupt satus.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csi0_cfg_csi_ap_diag_faults",
                    description: Some(
                        "csi0 ap diag faults.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irq_csi0_ap",
                    description: Some(
                        "interrupt of csi0 ap.",
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
            name: "GprRoD2",
            extends: None,
            description: Some(
                "gpr read-only register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csi1_sta_ap_if_int_sta",
                    description: Some(
                        "csi1 apb parity check interrupt satus.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csi1_cfg_csi_ap_diag_faults",
                    description: Some(
                        "csi1 ap diag faults.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irq_csi1_ap",
                    description: Some(
                        "interrupt of csi1 ap.",
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
            name: "GprRoD3",
            extends: None,
            description: Some(
                "gpr read-only register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy0_rx0_ctlo",
                    description: Some(
                        "rx phy0 rx0_ctlo.",
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
                Field {
                    name: "rx_phy0_rx1_ctlo",
                    description: Some(
                        "rx phy0 rx1_ctlo.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy0_rxck_ctlo",
                    description: Some(
                        "rx phy0 rxck_ctlo.",
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
            ],
        },
        FieldSet {
            name: "GprRoD4",
            extends: None,
            description: Some(
                "gpr read-only register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy1_rx0_ctlo",
                    description: Some(
                        "rx phy1 rx0_ctlo.",
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
                Field {
                    name: "rx_phy1_rx1_ctlo",
                    description: Some(
                        "rx phy1 rx1_ctlo.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy1_rxck_ctlo",
                    description: Some(
                        "rx phy1 rxck_ctlo.",
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
            ],
        },
        FieldSet {
            name: "GprRoD5",
            extends: None,
            description: Some(
                "gpr read-only register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy0_tx0_bist_out",
                    description: Some(
                        "tx phy0 tx0_bist_out.",
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
                    name: "tx_phy0_tx1_bist_out",
                    description: Some(
                        "tx phy0 tx1_bist_out.",
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
                    name: "tx_phy0_tx2_bist_out",
                    description: Some(
                        "tx phy0 tx2_bist_out.",
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
                    name: "tx_phy0_tx3_bist_out",
                    description: Some(
                        "tx phy0 tx3_bist_out.",
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
                    name: "tx_phy0_txck_bist_out",
                    description: Some(
                        "tx phy0 txck_bist_out.",
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
                    name: "tx_phy0_tx0_bist_done",
                    description: Some(
                        "tx phy0 tx0_bist_done.",
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
                    name: "tx_phy0_tx1_bist_done",
                    description: Some(
                        "tx phy0 tx1_bist_done.",
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
                    name: "tx_phy0_tx2_bist_done",
                    description: Some(
                        "tx phy0 tx2_bist_done.",
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
                    name: "tx_phy0_tx3_bist_done",
                    description: Some(
                        "tx phy0 tx3_bist_done.",
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
                    name: "tx_phy0_txck_bist_done",
                    description: Some(
                        "tx phy0 txck_bist_done.",
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
                    name: "tx_phy0_txck_bist_ok_pad",
                    description: Some(
                        "tx phy0 txck_ok_pad.",
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
                    name: "tx_phy0_txck_bist_done_pad",
                    description: Some(
                        "tx phy0 txck_done_pad.",
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
                    name: "dsi0_prbs_state",
                    description: Some(
                        "dsi0_prbs_state for debug only.",
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
            ],
        },
        FieldSet {
            name: "GprRoD6",
            extends: None,
            description: Some(
                "gpr read-only register 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy1_tx0_bist_out",
                    description: Some(
                        "tx phy1 tx0_bist_out.",
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
                    name: "tx_phy1_tx1_bist_out",
                    description: Some(
                        "tx phy1 tx1_bist_out.",
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
                    name: "tx_phy1_tx2_bist_out",
                    description: Some(
                        "tx phy1 tx2_bist_out.",
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
                    name: "tx_phy1_tx3_bist_out",
                    description: Some(
                        "tx phy1 tx3_bist_out.",
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
                    name: "tx_phy1_txck_bist_out",
                    description: Some(
                        "tx phy1 txck_bist_out.",
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
                    name: "tx_phy1_tx0_bist_done",
                    description: Some(
                        "tx phy1 tx0_bist_done.",
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
                    name: "tx_phy1_tx1_bist_done",
                    description: Some(
                        "tx phy1 tx1_bist_done.",
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
                    name: "tx_phy1_tx2_bist_done",
                    description: Some(
                        "tx phy1 tx2_bist_done.",
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
                    name: "tx_phy1_tx3_bist_done",
                    description: Some(
                        "tx phy1 tx3_bist_done.",
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
                    name: "tx_phy1_txck_bist_done",
                    description: Some(
                        "tx phy1 txck_bist_done.",
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
                    name: "tx_phy1_txck_bist_ok_pad",
                    description: Some(
                        "tx phy1 txck_ok_pad.",
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
                    name: "tx_phy1_txck_bist_done_pad",
                    description: Some(
                        "tx phy1 txck_done_pad.",
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
                    name: "dsi1_prbs_state",
                    description: Some(
                        "dsi1_prbs_state for debug only.",
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
            ],
        },
        FieldSet {
            name: "GprRoD7",
            extends: None,
            description: Some(
                "gpr read-only register 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy0_bist_done_pad",
                    description: Some(
                        "rx phy0 bist_done_pad.",
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
                    name: "rx_phy0_bist_ok_pad",
                    description: Some(
                        "rx phy0 bist_ok_pad.",
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
                    name: "rx_phy0_rx0_bist_out",
                    description: Some(
                        "rx phy0 rx0_bist_out.",
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
                    name: "rx_phy0_rx1_bist_out",
                    description: Some(
                        "rx phy0 rx1_bist_out.",
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
                    name: "rx_phy0_rx0_bist_done",
                    description: Some(
                        "rx phy0 rx0_bist_done.",
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
                    name: "rx_phy0_rx1_bist_done",
                    description: Some(
                        "rx phy0 rx1_bist_done.",
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
                    name: "rx_phy0_burn_in_ok_pad",
                    description: Some(
                        "rx_phy0_burn_in_ok_pad.",
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
            name: "GprRoD8",
            extends: None,
            description: Some(
                "gpr read-only register 8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy1_bist_done_pad",
                    description: Some(
                        "rx phy1 bist_done_pad.",
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
                    name: "rx_phy1_bist_ok_pad",
                    description: Some(
                        "rx phy1 bist_ok_pad.",
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
                    name: "rx_phy1_rx0_bist_out",
                    description: Some(
                        "rx phy1 rx0_bist_out.",
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
                    name: "rx_phy1_rx1_bist_out",
                    description: Some(
                        "rx phy1 rx1_bist_out.",
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
                    name: "rx_phy1_rx0_bist_done",
                    description: Some(
                        "rx phy1 rx0_bist_done.",
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
                    name: "rx_phy1_rx1_bist_done",
                    description: Some(
                        "rx phy1 rx1_bist_done.",
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
                    name: "rx_phy1_burn_in_ok_pad",
                    description: Some(
                        "rx_phy1_burn_in_ok_pad.",
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
            name: "GprRoD9",
            extends: None,
            description: Some(
                "gpr read-only register 9.",
            ),
            bit_size: 32,
            fields: &[],
        },
        FieldSet {
            name: "GprWr1ClrD0",
            extends: None,
            description: Some(
                "gpr write1 set/no-write clr register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr_wr1_clr_data",
                    description: Some(
                        "gpr register, write 1 /no-write set/clr matching bit.",
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
            name: "GprWrD0",
            extends: None,
            description: Some(
                "gpr write-read register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsi0_soft_reset_n",
                    description: Some(
                        "dsi controller 0 reset, active low.",
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
                    name: "dsi1_soft_reset_n",
                    description: Some(
                        "dsi controller 1 reset, active low.",
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
                    name: "csi0_soft_reset_n",
                    description: Some(
                        "csi controller 0 reset, active low.",
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
                    name: "csi1_soft_reset_n",
                    description: Some(
                        "csi controller 1 reset, active low.",
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
                    name: "dsi0_dpishutdn",
                    description: Some(
                        "dsi0 dpi shuntdown control.",
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
                    name: "dsi0_dpicolorm",
                    description: Some(
                        "dsi0 dpi cholor mode control.",
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
                    name: "dsi0_dpiupdatecfg",
                    description: Some(
                        "dsi0 dpi update configure.",
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
                    name: "dsi1_dpishutdn",
                    description: Some(
                        "dsi1 dpi shuntdown control.",
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
                    name: "dsi1_dpicolorm",
                    description: Some(
                        "dsi1 dpi cholor mode control.",
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
                    name: "dsi1_dpiupdatecfg",
                    description: Some(
                        "dsi1 dpi update configure.",
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
                    name: "csi0_cfg_apb_slverror_en",
                    description: Some(
                        "csi0 apb interface error check enable.",
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
                    name: "csi0_cfg_ap_if_int_en",
                    description: Some(
                        "csi0 apb interface error interrupt enable.",
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
                    name: "csi0_cfg_ap_if_check_en",
                    description: Some(
                        "csi0 apb interface parity check enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csi1_cfg_apb_slverror_en",
                    description: Some(
                        "csi1 apb interface error check enable.",
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
                    name: "csi1_cfg_ap_if_int_en",
                    description: Some(
                        "csi1 apb interface error interrupt enable.",
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
                    name: "csi1_cfg_ap_if_check_en",
                    description: Some(
                        "csi1 apb interface parity check enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GprWrD1",
            extends: None,
            description: Some(
                "gpr write-read register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lcdc0_p0_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
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
                Field {
                    name: "lcdc0_p1_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lcdc1_p0_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
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
                    name: "lcdc1_p1_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
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
                    name: "pdma_p0_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdma_p1_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpeg_ctrl",
                    description: Some(
                        "bit0: select cam0; bit1: select cam1; bit2: select jpeg; bit3: select pdma.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GprWrD2",
            extends: None,
            description: Some(
                "gpr write-read register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy0_pll_div",
                    description: Some(
                        "tx phy0 pll_div.",
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
                Field {
                    name: "tx_phy0_byps_ckdet",
                    description: Some(
                        "tx phy0 byps_ckdet.",
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
                    name: "tx_phy0_shutdownz",
                    description: Some(
                        "tx phy0 shutdownz, active low.",
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
                    name: "tx_phy0_reset_n",
                    description: Some(
                        "tx phy0 reset, active low.",
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
                    name: "tx_phy0_iddq_en",
                    description: Some(
                        "tx phy0 iddq_en.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy0_refclk_div",
                    description: Some(
                        "tx phy0 refclk_div.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy0_phy_mode",
                    description: Some(
                        "tx phy0 phy_mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy0_rate_lvds",
                    description: Some(
                        "tx phy0 rate_lvds.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy0_port_pll_rdy_sel",
                    description: Some(
                        "tx phy0 port_pll_rdy_sel.",
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
            ],
        },
        FieldSet {
            name: "GprWrD3",
            extends: None,
            description: Some(
                "gpr write-read register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy0_pll_ctrl",
                    description: Some(
                        "tx phy0 pll_ctrl.",
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
            name: "GprWrD4",
            extends: None,
            description: Some(
                "gpr write-read register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy0_ckphy_ctl",
                    description: Some(
                        "tx phy0 ckphy_ctl.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy0_dsi0_prbs_start",
                    description: Some(
                        "tx phy0 dsi0_prbs_start.",
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
                    name: "tx_phy0_dsi0_prbs_disable",
                    description: Some(
                        "tx phy0 dsi0_prbs_disable.",
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
                    name: "tx_phy0_tx0_pat_sel",
                    description: Some(
                        "tx phy0 tx0_pat_sel.",
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
                    name: "tx_phy0_tx1_pat_sel",
                    description: Some(
                        "tx phy0 tx1_pat_sel.",
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
                    name: "tx_phy0_tx2_pat_sel",
                    description: Some(
                        "tx phy0 tx2_pat_sel.",
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
                    name: "tx_phy0_tx3_pat_sel",
                    description: Some(
                        "tx phy0 tx3_pat_sel.",
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
                    name: "tx_phy0_txck_pat_sel",
                    description: Some(
                        "tx phy0 txck_pat_sel.",
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
                    name: "tx_phy0_tx0_lpbk_en",
                    description: Some(
                        "tx_phy0 tx0_lpbk_en.",
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
                    name: "tx_phy0_tx1_lpbk_en",
                    description: Some(
                        "tx_phy0 tx1_lpbk_en.",
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
                    name: "tx_phy0_tx2_lpbk_en",
                    description: Some(
                        "tx_phy0 tx2_lpbk_en.",
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
                    name: "tx_phy0_tx3_lpbk_en",
                    description: Some(
                        "tx_phy0 tx3_lpbk_en.",
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
                    name: "tx_phy0_txck_lpbk_en",
                    description: Some(
                        "tx_phy0 txck_lpbk_en.",
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
                    name: "tx_phy0_tx0_bist_en",
                    description: Some(
                        "tx phy0 tx0_bist_en.",
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
                    name: "tx_phy0_tx1_bist_en",
                    description: Some(
                        "tx phy0 tx1_bist_en.",
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
                    name: "tx_phy0_tx2_bist_en",
                    description: Some(
                        "tx phy0 tx2_bist_en.",
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
                    name: "tx_phy0_tx3_bist_en",
                    description: Some(
                        "tx phy0 tx3_bist_en.",
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
                    name: "tx_phy0_txck_bist_en",
                    description: Some(
                        "tx phy0 txck_bist_en.",
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
            name: "GprWrD5",
            extends: None,
            description: Some(
                "gpr write-read register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy1_pll_div",
                    description: Some(
                        "tx phy1 pll_div.",
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
                Field {
                    name: "tx_phy1_byps_ckdet",
                    description: Some(
                        "tx phy1 byps_ckdet.",
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
                    name: "tx_phy1_shutdownz",
                    description: Some(
                        "tx phy1 shutdownz, active low.",
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
                    name: "tx_phy1_reset_n",
                    description: Some(
                        "tx phy1 reset, active low.",
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
                    name: "tx_phy1_iddq_en",
                    description: Some(
                        "tx phy1 iddq_en.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy1_refclk_div",
                    description: Some(
                        "tx phy1 refclk_div.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy1_phy_mode",
                    description: Some(
                        "tx phy1 phy_mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy1_rate_lvds",
                    description: Some(
                        "tx phy1 rate_lvds.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy1_port_pll_rdy_sel",
                    description: Some(
                        "tx phy1 port_pll_rdy_sel.",
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
            ],
        },
        FieldSet {
            name: "GprWrD6",
            extends: None,
            description: Some(
                "gpr write-read register 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy1_pll_ctrl",
                    description: Some(
                        "tx phy1 pll_ctrl.",
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
            name: "GprWrD7",
            extends: None,
            description: Some(
                "gpr write-read register 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_phy1_ckphy_ctl",
                    description: Some(
                        "tx phy1 ckphy_ctl.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_phy1_dsi0_prbs_start",
                    description: Some(
                        "tx phy1 dsi0_prbs_start.",
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
                    name: "tx_phy1_dsi0_prbs_disable",
                    description: Some(
                        "tx phy1 dsi0_prbs_disable.",
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
                    name: "tx_phy1_tx0_pat_sel",
                    description: Some(
                        "tx phy1 tx0_pat_sel.",
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
                    name: "tx_phy1_tx1_pat_sel",
                    description: Some(
                        "tx phy1 tx1_pat_sel.",
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
                    name: "tx_phy1_tx2_pat_sel",
                    description: Some(
                        "tx phy1 tx2_pat_sel.",
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
                    name: "tx_phy1_tx3_pat_sel",
                    description: Some(
                        "tx phy1 tx3_pat_sel.",
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
                    name: "tx_phy1_txck_pat_sel",
                    description: Some(
                        "tx phy1 txck_pat_sel.",
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
                    name: "tx_phy1_tx0_lpbk_en",
                    description: Some(
                        "tx_phy1 tx0_lpbk_en.",
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
                    name: "tx_phy1_tx1_lpbk_en",
                    description: Some(
                        "tx_phy1 tx1_lpbk_en.",
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
                    name: "tx_phy1_tx2_lpbk_en",
                    description: Some(
                        "tx_phy1 tx2_lpbk_en.",
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
                    name: "tx_phy1_tx3_lpbk_en",
                    description: Some(
                        "tx_phy1 tx3_lpbk_en.",
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
                    name: "tx_phy1_txck_lpbk_en",
                    description: Some(
                        "tx_phy1 txck_lpbk_en.",
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
                    name: "tx_phy1_tx0_bist_en",
                    description: Some(
                        "tx phy1 tx0_bist_en.",
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
                    name: "tx_phy1_tx1_bist_en",
                    description: Some(
                        "tx phy1 tx1_bist_en.",
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
                    name: "tx_phy1_tx2_bist_en",
                    description: Some(
                        "tx phy1 tx2_bist_en.",
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
                    name: "tx_phy1_tx3_bist_en",
                    description: Some(
                        "tx phy1 tx3_bist_en.",
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
                    name: "tx_phy1_txck_bist_en",
                    description: Some(
                        "tx phy1 txck_bist_en.",
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
            name: "GprWrD8",
            extends: None,
            description: Some(
                "gpr write-read register 8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy0_phy_mode",
                    description: Some(
                        "rx phy0 phy_mode.",
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
                    name: "rx_phy0_bist_ckin_sel",
                    description: Some(
                        "rx phy0 bist_ckin_sel.",
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
                    name: "rx_phy0_bist_en",
                    description: Some(
                        "rx phy0 bist_en.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy0_bist_en_pad",
                    description: Some(
                        "rx phy0 bist_en_pad.",
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
                    name: "rx_phy0_bist_mode",
                    description: Some(
                        "rx phy0 bist_mode.",
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
                    name: "rx_phy0_rx0_bist_en",
                    description: Some(
                        "rx phy0 rx0_bist_en rx1_bist_en.",
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
                    name: "rx_phy0_bist_freq_trim",
                    description: Some(
                        "rx phy0 bist_freq_trim.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy0_lpbk_mode",
                    description: Some(
                        "rx phy0 lpbk_mode.",
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
                    name: "rx_phy0_burn_in_en_pad",
                    description: Some(
                        "rx phy0 burn_in_en_pad.",
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
                    name: "rx_phy0_brun_in_mode",
                    description: Some(
                        "rx phy0 burn_in_mode.",
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
            name: "GprWrD9",
            extends: None,
            description: Some(
                "gpr write-read register 9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_phy1_phy_mode",
                    description: Some(
                        "rx phy1 phy_mode.",
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
                    name: "rx_phy1_bist_ckin_sel",
                    description: Some(
                        "rx phy1 bist_ckin_sel.",
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
                    name: "rx_phy1_bist_en",
                    description: Some(
                        "rx phy1 bist_en.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy1_bist_en_pad",
                    description: Some(
                        "rx phy1 bist_en_pad.",
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
                    name: "rx_phy1_bist_mode",
                    description: Some(
                        "rx phy1 bist_mode.",
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
                    name: "rx_phy1_rx0_bist_en",
                    description: Some(
                        "rx phy1 rx0_bist_en rx1_bist_en.",
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
                    name: "rx_phy1_bist_freq_trim",
                    description: Some(
                        "rx phy1 bist_freq_trim.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx_phy1_lpbk_mode",
                    description: Some(
                        "rx phy1 lpbk_mode.",
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
                    name: "rx_phy1_burn_in_en_pad",
                    description: Some(
                        "rx phy1 burn_in_en_pad.",
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
                    name: "rx_phy1_brun_in_mode",
                    description: Some(
                        "rx phy1 burn_in_mode.",
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
            name: "Misc",
            extends: None,
            description: Some(
                "common register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvb_di0_ctl",
                    description: Some(
                        "LVB DI0 optional general purpose control which is usually unused by display.",
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
                    name: "lvb_di1_ctl",
                    description: Some(
                        "LVB DI1 optional general purpose control which is usually unused by display.",
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
            name: "Pixmux",
            extends: None,
            description: Some(
                "pixel path mux register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cam0_sel",
                    description: Some(
                        "CAM0 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP.",
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
                    name: "cam0_en",
                    description: Some(
                        "CAM0 pixel bus enable.",
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
                    name: "cam1_sel",
                    description: Some(
                        "CAM1 pixel bus selection 111: Reserved 110: LCB1 101: LCB0 100: LCDC1 011: LCDC0 010: CSI1 001: CSI0 000: DVP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cam1_en",
                    description: Some(
                        "CAM1 pixel bus enable.",
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
                    name: "dsi0_sel",
                    description: Some(
                        "DSI1 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "dsi0_en",
                    description: Some(
                        "DSI1 pixel bus enable.",
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
                    name: "dsi1_sel",
                    description: Some(
                        "DSI0 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "dsi1_en",
                    description: Some(
                        "DSI0 pixel bus enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lvb_di0_sel",
                    description: Some(
                        "LVB DI0 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "lvb_di0_en",
                    description: Some(
                        "LVB DI0 pixel bus enable.",
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
                    name: "lvb_di1_sel",
                    description: Some(
                        "LVB DI1 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "lvb_di1_en",
                    description: Some(
                        "LVB DI1 pixel bus enable.",
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
                    name: "gwc0_sel",
                    description: Some(
                        "GWC0 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "gwc0_en",
                    description: Some(
                        "GWC0 pixel bus enable.",
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
                    name: "gwc1_sel",
                    description: Some(
                        "GWC1 pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "gwc1_en",
                    description: Some(
                        "GWC1 pixel bus enable.",
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
                    name: "rgb_sel",
                    description: Some(
                        "RGB pixel bus selection 1: LCDC1 0: LCDC0.",
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
                    name: "rgb_en",
                    description: Some(
                        "RGB pixel bus enable.",
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
            ],
        },
    ],
    enums: &[],
};
