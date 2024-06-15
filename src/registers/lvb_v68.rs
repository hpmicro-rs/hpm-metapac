use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lvb",
            extends: None,
            description: Some(
                "LVB.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "control register.",
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
                    name: "phy_stat",
                    description: Some(
                        "LVDS TX PHY Status register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_pow_ctrl",
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
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyPowCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_phy",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "TxPhy",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "TxPhy",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "TX PHY Setting.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "TX_PHY Setting.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "TX PHY Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_deemp",
                    description: Some(
                        "output de-emphasis level trimming(Unit: dB) 00: 0 01: 2.5 10: 6.0 11: 6.0.",
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
                    name: "tx_sr",
                    description: Some(
                        "output slew-rate trimming 00: slowest slew-rate; 11: fastest slew-rate.",
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
                    name: "tx_amp",
                    description: Some(
                        "Output voltage Adjustment(Unit: mV). 0000 : 50 0001: 100 0010: 150 0011: 200 0100: 250 0101: 300 0110: 350 0111: 400 1000: 450 1001: 500 1010: 550 1011~1111: 600.",
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
                    name: "tx_vcom",
                    description: Some(
                        "output Common Mode Voltage adjustment(Unit: V). 0000: 0.7 0001: 0.8 0010: 0.9 0011: 1.0 0100: 1.1 0101: 1.2 0110: 1.3 0111: 1.4 1000~1111: 1.5.",
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
                    name: "tx_phase_sel",
                    description: Some(
                        "data/clock lane output phase adjustment: 0000: 0 0001: data lane is 1/32, clock lane is 1/16 0010: data lane is 2/32, clock lane is 2/16 0011: data lane is 3/32, clock lane is 3/16 0100: data lane is 4/32, clock lane is 4/16 0101: data lane is 5/32, clock lane is 5/16 0110: data lane is 6/32, clock lane is 6/16 0111: data lane is 7/32, clock lane is 7/16 1000: data lane is 8/32, clock lane is 8/16 1001: data lane is 9/32, clock lane is 9/16 1010: data lane is 10/32, clock lane is 10/16 1011: data lane is 11/32, clock lane is 11/16 1100: data lane is 12/32, clock lane is 12/16 1101: data lane is 13/32, clock lane is 13/16 1110: data lane is 14/32, clock lane is 14/16 1111: data lane is 15/32, clock lane is 15/16.",
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
                    name: "tx_bus_width",
                    description: Some(
                        "Parallel data bus width select： 000: 4-bit mode, txN_data[3:0] are valid, txN_data[11:4] can be arbitrary state. 001: 6-bit mode, txN_data[5:0] are valid, txN_data[11:6] can be arbitrary state. 010: 7-bit mode. txN_data[6:0] are valid, txN_data[11:7] can be arbitrary state. 011: 8-bit mode. txN_data[7:0] are valid, txN_data[11:8] can be arbitrary state. 100: 9-bit mode. txN_data[8:0] are valid, txN_data[11:9] can be arbitrary state. 101: 10-bit mode. txN_data[9:0] are valid, txN_data[11:10] can be arbitrary state. 110: 11-bit mode. txN_data[10:0] are valid, txN_data[11] can be arbitrary state. 111: 12-bit mode. txN_data[11:0] are valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_rterm_en",
                    description: Some(
                        "Inner Terminal Resistance enable 0: Disable rterm 2000ohm 1: Enable rterm 100ohm.",
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
                    name: "tx_idle",
                    description: Some(
                        "Force the high-speed differential signal to common mode. This signal can be set during IP power up stage to prevent unexpected leakage current in TXP/TXN 0: Normal operation 1: Force TXPN /TXMN to common mode.",
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
            ],
        },
        FieldSet {
            name: "Ctl1",
            extends: None,
            description: Some(
                "TX_PHY Setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_ctl",
                    description: Some(
                        "No description available.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch0_en",
                    description: Some(
                        "Channel 0 enable: 1: enable 0: disable.",
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
                    name: "ch0_sel",
                    description: Some(
                        "Channel 0 select: 1: select DI 1 0: select DI 0.",
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
                    name: "ch1_en",
                    description: Some(
                        "Channel 1 enable: 1: enable 0: disable.",
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
                    name: "ch1_sel",
                    description: Some(
                        "Channel 1 select: 1: select DI 1 0: select DI 0.",
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
                    name: "ch0_bit_mapping",
                    description: Some(
                        "Channel 0 data protocol: 1: JEIDA standard 0: SPWG standard.",
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
                    name: "ch1_bit_mapping",
                    description: Some(
                        "Channel 1 data protocol: 1: JEIDA standard 0: SPWG standard.",
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
                    name: "lvds_txclk_shift",
                    description: Some(
                        "Shift the LVDS TX PHY clock in relation to the data. 000: txck is 7'b1100011 001: txck is 7‘b1110001 010: txck is 7‘b1111000 011: txck is 7‘b1000111 100: txck is 7‘b0001111 101: txck is 7‘b0011110 110: txck is 7‘b0111100 111: txck is 7‘b1100011.",
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
                    name: "di0_vsync_polarity",
                    description: Some(
                        "DI 0 vsync polarity: 1: active low 0: active high.",
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
                    name: "di1_vsync_polarity",
                    description: Some(
                        "DI 1 vsync polarity: 1: active low 0: active high.",
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
                    name: "split_mode_en",
                    description: Some(
                        "Split mode enable: 1: enable 0: disable Note: when using split mode, ch0/1 should be enabled, and should select same DI.",
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
                    name: "split_hswhbp_width",
                    description: Some(
                        "Just for split mode, the sum of HSW and HBP width is even 1: yes 0: no.",
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
                    name: "split_ch_mode",
                    description: Some(
                        "Just for split mode 1: two channel pixel data are not aligned 0: two channel pixel data are aligned.",
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
                    name: "split_ch_reverse",
                    description: Some(
                        "Just for split mode, reverse two channel data.",
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
            ],
        },
        FieldSet {
            name: "PhyPowCtrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx0_pd",
                    description: Some(
                        "Power down control signal of channel tx0 0: Normal operation 1: Power down channel.",
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
                    name: "tx1_pd",
                    description: Some(
                        "Power down control signal of channel tx1 0: Normal operation 1: Power down channel.",
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
                    name: "tx2_pd",
                    description: Some(
                        "Power down control signal of channel tx2 0: Normal operation 1: Power down channel.",
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
                    name: "tx3_pd",
                    description: Some(
                        "Power down control signal of channel tx3 0: Normal operation 1: Power down channel.",
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
                    name: "txck_pd",
                    description: Some(
                        "Power down control signal of channel txck 0: Normal operation 1: Power down channel.",
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
                    name: "pwon_pll",
                    description: Some(
                        "pll power on.",
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
            ],
        },
        FieldSet {
            name: "PhyStat",
            extends: None,
            description: Some(
                "LVDS TX PHY Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvds0_tx_phy_pll_lock",
                    description: Some(
                        "LVDS0 TX PHY PLL Lock indication Signal, 1 means pll already locked.",
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
                    name: "lvds1_tx_phy_pll_lock",
                    description: Some(
                        "LVDS1 TX PHY PLL Lock indication Signal, 1 means pll already locked.",
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
    ],
    enums: &[],
};
