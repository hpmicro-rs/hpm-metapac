use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Conctl",
            extends: None,
            description: Some(
                "CONCTL.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl0",
                    description: Some(
                        "No description available.",
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
                    name: "ctrl2",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl3",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl4",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl5",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl5",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enet0_txclk_dly_sel",
                    description: Some(
                        "No description available.",
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
                    name: "enet0_rxclk_dly_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet1_txclk_dly_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet1_rxclk_dly_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctrl2",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enet0_rmii_txclk_sel",
                    description: Some(
                        "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;.",
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
                    name: "enet0_flowctrl",
                    description: Some(
                        "No description available.",
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
                    name: "enet0_phy_intf_sel",
                    description: Some(
                        "000:Reserved 001:RGMII 100:RMII 111:Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet0_refclk_oe",
                    description: Some(
                        "No description available.",
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
                    name: "enet0_lpi_irq_en",
                    description: Some(
                        "ENET0 LPI IRQ Enable.",
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
            name: "Ctrl3",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enet1_rmii_txclk_sel",
                    description: Some(
                        "No description available.",
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
                    name: "enet1_flowctrl",
                    description: Some(
                        "No description available.",
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
                    name: "enet1_phy_intf_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet1_refclk_oe",
                    description: Some(
                        "No description available.",
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
                    name: "enet1_lpi_irq_en",
                    description: Some(
                        "ENET1 LPI Interrupt Enable.",
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
            name: "Ctrl4",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdxc0_gpr_cclk_rx_dly_sw_force",
                    description: Some(
                        "force use sw DLL config.",
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
                    name: "sdxc0_gpr_cclk_rx_dly_sw_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc0_gpr_strobe_in_enable",
                    description: Some(
                        "enable strobe clock, maybe used when update strobe DLL.",
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
                    name: "sdxc0_gpr_tuning_strobe_sel",
                    description: Some(
                        "for strobe DLL, default 7taps(1ns).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc0_gpr_tuning_card_clk_sel",
                    description: Some(
                        "for card clock DLL, default 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc0_cardclk_inv_en",
                    description: Some(
                        "card clock inverter enable.",
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
                    name: "sdxc0_wkp_irq_en",
                    description: Some(
                        "wakeup irq enable.",
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
                    name: "sdxc0_sys_irq_en",
                    description: Some(
                        "system irq enable.",
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
            name: "Ctrl5",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdxc1_gpr_cclk_rx_dly_sw_force",
                    description: Some(
                        "No description available.",
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
                    name: "sdxc1_gpr_cclk_rx_dly_sw_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc1_gpr_strobe_in_enable",
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
                    name: "sdxc1_gpr_tuning_strobe_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc1_gpr_tuning_card_clk_sel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdxc1_cardclk_inv_en",
                    description: Some(
                        "card clock inverter enable.",
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
                    name: "sdxc1_wkp_irq_en",
                    description: Some(
                        "wakeup irq enable.",
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
                    name: "sdxc1_sys_irq_en",
                    description: Some(
                        "system irq enable.",
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
