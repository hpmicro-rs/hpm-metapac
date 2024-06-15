use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lcb",
            extends: None,
            description: Some(
                "LCB.",
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
                        "LVDS RX PHY Status register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                    byte_offset: 0x68,
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
                    name: "phy_d_ctrl",
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
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyDCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_ck_ctrl",
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
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCkCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_adj_ctrl",
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
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyAdjCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_su_ctrl",
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
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhySuCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "mode selection： 00: lvds display(4 line), two LVDS RX PHY must be LVDS display mode 01: cam link(4 line), two LVDS RX PHY must be LVDS display mode 10: sync code(2 line), LVDS RX PHY must be LVDS cameral mode 11: sync code(1line), LVDS RX PHY must be LVDS cameral mode.",
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
                    name: "data_width",
                    description: Some(
                        "just for LVDS Display mode, data width: 1: 24bit 0: 18bit(3line).",
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
                    name: "bit_mapping",
                    description: Some(
                        "just for LVDS Display mode, data protocol: 1: JEIDA standard 0: SPWG standard.",
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
                    name: "cam_link_width",
                    description: Some(
                        "just for CAM LINK mode, data width: 00: 24bit 01: 30bit 10: 36bit 11: reserved.",
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
                    name: "lvds_rxck_sel",
                    description: Some(
                        "just for LVDS Display mode and CAM LINK mode, clock selection: 1: LVDS1 RXCK 0: LVDS0 RXCK.",
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
            name: "PhyAdjCtrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvds_dll_tuning_int",
                    description: Some(
                        "LVDS RX PHY RXCK line: DLL loop delay coarse adjustment initial signal 00000000: min ; 11111111: max.",
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
                    name: "lvds_rx1_dline_adj",
                    description: Some(
                        "LVDS RX PHY RX1 line: bit [7:0] : Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved.",
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
                    name: "lvds_rx0_dline_adj",
                    description: Some(
                        "LVDS RX PHY RX0 line: bit [7:0] : Lane N skew adjustment control signal between data and clock 0000000: max; 1111111: min bit 8 : Reserved.",
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
            name: "PhyCkCtrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_ctl",
                    description: Some(
                        "bit 0 : DLL loop delay adjustment minimum control signal 0: used for RCKP/RCKN’s frequency is 40Mhz~70Mhz 1:used for RCKP/RCKN’s frequency is 70Mhz~110Mhz bit [2:1] : DLL loop delay adjustment current regulation control signal. 00: min; 11: max bit 3 : Reserved bit 4 : Clock Lane Skew adjust enable in LVDS Camera Mode. bit [7:5] : Bus width selection in LVDS Camera Mode 000: 4bit; 001:6bit; 010:7bit; 011:8bit; 100:9bit; 101:10bit; 110:11bit; 111:12bit. bit [10:8] : DDR Clock duty cycle adjust in LVDS Camera Mode. bit [15:11] : Reserved.",
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
                Field {
                    name: "rx_rterm",
                    description: Some(
                        "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm.",
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
                    name: "rx_vcom",
                    description: Some(
                        "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground.",
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
            ],
        },
        FieldSet {
            name: "PhyDCtrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_ctl",
                    description: Some(
                        "bit 0 : Lane N Data MSB first enable signal. 0: LSB ; 1: MSB bit 1 : Lane N Data Polarity signal. 0: Not inverting; 1: Inverting bit [4:2] : Phase difference between the output first bit data (rxN[6:0]) and the input clock (RCKP/N) in LVDS Display Mode. bit 5 : Reserved bit 6 : Output data sampling clock control signal 0: Sampling using the rising edge of the clock pck. 1: Sampling using the falling edge of the clock pck. bit 7 : Reserved bit 8 : Data Lane N Skew adjust enable in LVDS Camera Mode. bit [12:9] : Data Lane N Skew adjust; 0000: min; 0111: default; 1111: max. bit [15:13] : Reserved.",
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
                Field {
                    name: "rx_rterm",
                    description: Some(
                        "Terminal impedance regulation control signal 0000: hi-z; 0001: 150ohm; 1000:100ohm; 1111:75ohm.",
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
                    name: "rx_vcom",
                    description: Some(
                        "bit 1: Receiver hysteresis enable signal. 0: enable; 1: disable bit 0: Terminal impedance common mode selection control signal. 0: floating; 1: Ground.",
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
                    name: "rx0_pd",
                    description: Some(
                        "Power down control signal of channel rx0 0: Normal operation 1: Power down channel.",
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
                    name: "rx1_pd",
                    description: Some(
                        "Power down control signal of channel rx1 0: Normal operation 1: Power down channel.",
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
                    name: "rxck_pd",
                    description: Some(
                        "Power down control signal of channel rxck 0: Normal operation 1: Power down channel.",
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
                    name: "iddq_en",
                    description: Some(
                        "Power down control signal of channel rxck/rx1/rx0 0: Normal operation 1: Power down channel.",
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
            name: "PhyStat",
            extends: None,
            description: Some(
                "LVDS RX PHY Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvds0_rx_phy_dll_lock",
                    description: Some(
                        "LVDS0 RX PHY DLL Lock indication Signal, 1 means dll already locked.",
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
                    name: "lvds1_rx_phy_dll_lock",
                    description: Some(
                        "LVDS1 RX PHY DLL Lock indication Signal, 1 means dll already locked.",
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
            name: "PhySuCtrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su_ctrl",
                    description: Some(
                        "bit [2:0] : Reference voltage/current adjustment control signal. 000: min; 111: max bit [3] : Internal bias circuit selection signal. 0: from Bandgap Mode; 1: from self-bias mode bit [7:4] : Reserved.",
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
    ],
    enums: &[],
};
