use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "MipiDsiPhy",
            extends: None,
            description: Some(
                "MIPI_DSI_PHY0.",
            ),
            items: &[
                BlockItem {
                    name: "clane_para0",
                    description: Some(
                        "timer counter about clock lane parameter.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClanePara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clane_para1",
                    description: Some(
                        "timer counter about clock lane parameter.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClanePara1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clane_para2",
                    description: Some(
                        "timer counter about clock lane parameter.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClanePara2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clane_para3",
                    description: Some(
                        "timer counter about clock lane parameter.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClanePara3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane0_para0",
                    description: Some(
                        "timer counter about datalane0 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane0Para0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane0_para1",
                    description: Some(
                        "timer counter about datalane0 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane0Para1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane0_para2",
                    description: Some(
                        "timer counter about datalane0 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane0Para2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane0_para3",
                    description: Some(
                        "timer counter about datalane0 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane0Para3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane0_para4",
                    description: Some(
                        "timer counter about datalane0 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane0Para4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane1_para0",
                    description: Some(
                        "timer counter about datalane1 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane1Para0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane1_para1",
                    description: Some(
                        "timer counter about datalane1 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane1Para1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane1_para2",
                    description: Some(
                        "timer counter about datalane1 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane1Para2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane1_para3",
                    description: Some(
                        "timer counter about datalane1 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane1Para3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane2_para0",
                    description: Some(
                        "timer counter about datalane2 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane2Para0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane2_para1",
                    description: Some(
                        "timer counter about datalane2 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane2Para1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane2_para2",
                    description: Some(
                        "timer counter about datalane2 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane2Para2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane2_para3",
                    description: Some(
                        "timer counter about datalane2 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane2Para3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane3_para0",
                    description: Some(
                        "timer counter about datalane3 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane3Para0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane3_para1",
                    description: Some(
                        "timer counter about datalane3 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane3Para1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane3_para2",
                    description: Some(
                        "timer counter about datalane3 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane3Para2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlane3_para3",
                    description: Some(
                        "timer counter about datalane3 parameter.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlane3Para3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "common_para0",
                    description: Some(
                        "timing parameter for all lanes.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CommonPara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_para0",
                    description: Some(
                        "dphy control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CtrlPara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll_ctrl_para0",
                    description: Some(
                        "dphy pll control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PllCtrlPara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcal_ctrl",
                    description: Some(
                        "dphy calibration control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RcalCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trim_para",
                    description: Some(
                        "dphy trimming parameter.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrimPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test_para0",
                    description: Some(
                        "dphy test control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TestPara0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test_para1",
                    description: Some(
                        "dphy bist test control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TestPara1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "misc_para",
                    description: Some(
                        "dphy control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MiscPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clane_para4",
                    description: Some(
                        "dphy clock lane control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClanePara4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "interface_para",
                    description: Some(
                        "dphy clock lane control parameter.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InterfacePara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcs_reserved_pin_para",
                    description: Some(
                        "reserved the pins for pcs.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PcsReservedPinPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clane_data_para",
                    description: Some(
                        "parallel data about clock lane parameter.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClaneDataPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pma_lane_sel_para",
                    description: Some(
                        "pma about clock lane select parameter.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PmaLaneSelPara",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ClaneDataPara",
            extends: None,
            description: Some(
                "parallel data about clock lane parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clane_data",
                    description: Some(
                        "the parallel data about clock lane.",
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
                    name: "clane_data_sel",
                    description: Some(
                        "select the data about clock lane.",
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
            name: "ClanePara0",
            extends: None,
            description: Some(
                "timer counter about clock lane parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rst2enlptx_c",
                    description: Some(
                        "the soft reset of clk_cfg domain.",
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
            name: "ClanePara1",
            extends: None,
            description: Some(
                "timer counter about clock lane parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_inittime_c",
                    description: Some(
                        "the number of byteclk cycles that clklane drive LP-11 during initialization period.",
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
            name: "ClanePara2",
            extends: None,
            description: Some(
                "timer counter about clock lane parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_clkpre_c",
                    description: Some(
                        "the number of byteclk cycles that hs clock shall be driven prior to data lane beginning the transition from lp to hs mode.",
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
                    name: "t_clkzero_c",
                    description: Some(
                        "the number of byteclk cycles that clock lane clkp/n lines are at the hs-zero state hs-0 during a hs clock transmission.",
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
                    name: "t_clkprepare_c",
                    description: Some(
                        "the number of byteclk cycles that clock lane clkp/n lines are at the hs prepare state lp-00 during a hs clock transmission.",
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
            name: "ClanePara3",
            extends: None,
            description: Some(
                "timer counter about clock lane parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_hsexit_c",
                    description: Some(
                        "the number of byteclk cycles that the clock lane clkp/n lines are at hs-exit state after a hs clock transmission.",
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
                    name: "t_clktrial_c",
                    description: Some(
                        "the number of byteclk cycles that the clock lane clkp/n lines are at state hs-tail sate hs-0 during a hs clock transmission.",
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
                    name: "t_clkpost_c",
                    description: Some(
                        "the number of byteclk cycles that the clock lane should keep sending the hs-clock after the last associated data lane has transitioned to LP mode.",
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
            name: "ClanePara4",
            extends: None,
            description: Some(
                "dphy clock lane control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wakeup_c",
                    description: Some(
                        "the number of byteclk cycles from exiting ultra low power state to enabling the low-power driver.",
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
            name: "CommonPara0",
            extends: None,
            description: Some(
                "timing parameter for all lanes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_lpx",
                    description: Some(
                        "the number of byteclk cycles of transmitted length of any low-power state period.",
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
            name: "CtrlPara0",
            extends: None,
            description: Some(
                "dphy control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "su_iddq_en",
                    description: Some(
                        "power down all modules inside su includes ivref, r-calibration and pll, high effective.",
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
                    name: "pwon_dsi",
                    description: Some(
                        "power on all dsi lane.",
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
                    name: "pwon_pll",
                    description: Some(
                        "power on pll high active.",
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
                    name: "pwon_sel",
                    description: Some(
                        "select the cource of PMA power on control signals.",
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
                    name: "en_lpcd_d0",
                    description: Some(
                        "lp-cd enable for lane0.",
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
                    name: "en_lprx_d0",
                    description: Some(
                        "lp-rx enable for lane0.",
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
                    name: "en_ulprx_d0",
                    description: Some(
                        "ulp-rx enable for lane0.",
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
                    name: "vbg_rdy",
                    description: Some(
                        "the indicator signal of reference generator is ready.",
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
            ],
        },
        FieldSet {
            name: "Dlane0Para0",
            extends: None,
            description: Some(
                "timer counter about datalane0 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rst2enlptx_d0",
                    description: Some(
                        "the number of byteclk cycles that datalane0 wait to enable lptx_en after reset release.",
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
            name: "Dlane0Para1",
            extends: None,
            description: Some(
                "timer counter about datalane0 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_inittime_d0",
                    description: Some(
                        "the number of byteclk cycles that datalane0 drive lp-11 during initiaalization period.",
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
            name: "Dlane0Para2",
            extends: None,
            description: Some(
                "timer counter about datalane0 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_hsexit_d0",
                    description: Some(
                        "the number of byteclk cycles that the datalane0 stay at state hs-exit sate after a hs clock transmission.",
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
                    name: "t_hstrail_d0",
                    description: Some(
                        "the number of byteclk cycles that the datalane0 stay at hs-trail state during a hs clock transmission.",
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
                    name: "t_hszero_d0",
                    description: Some(
                        "the number of byteclk cycles that the datalane0 stay at hs-zero sate during a hs transmission.",
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
                    name: "t_hsprepare_d0",
                    description: Some(
                        "the number of byteclk cycles that the datalane0 stay at hs prepare state lp-00 during a hs transmission.",
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
            name: "Dlane0Para3",
            extends: None,
            description: Some(
                "timer counter about datalane0 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wakeup_d0",
                    description: Some(
                        "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver.",
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
            name: "Dlane0Para4",
            extends: None,
            description: Some(
                "timer counter about datalane0 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_taget_d0",
                    description: Some(
                        "the number of byteclk cycles that the new transmitter drivers the bridge state after accepting control during bta.",
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
                    name: "t_tasure_d0",
                    description: Some(
                        "the number of byteclk cycles that the rx waits after a bridge state has been detected during a turnaround procedure.",
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
                    name: "t_tago_d0",
                    description: Some(
                        "the number of byteclk cycles that the tx drives the bridge state during a turnaroud procedure.",
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
            name: "Dlane1Para0",
            extends: None,
            description: Some(
                "timer counter about datalane1 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rst2enlptx_d1",
                    description: Some(
                        "the number of byteclk cycles that datalane1 wait to enable lptx_en after reset release.",
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
            name: "Dlane1Para1",
            extends: None,
            description: Some(
                "timer counter about datalane1 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_inittime_d1",
                    description: Some(
                        "the number of byteclk cycles that datalane1 drive lp-11 during initiaalization period.",
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
            name: "Dlane1Para2",
            extends: None,
            description: Some(
                "timer counter about datalane1 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_hsexit_d1",
                    description: Some(
                        "the number of byteclk cycles that the datalane1 stay at state hs-exit sate after a hs clock transmission.",
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
                    name: "t_hstrail_d1",
                    description: Some(
                        "the number of byteclk cycles that the datalane1 stay at hs-trail state during a hs clock transmission.",
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
                    name: "t_hszero_d1",
                    description: Some(
                        "the number of byteclk cycles that the datalane1 stay at hs-zero sate during a hs transmission.",
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
                    name: "t_hsprepare_d1",
                    description: Some(
                        "the number of byteclk cycles that the datalane1 stay at hs prepare state lp-00 during a hs transmission.",
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
            name: "Dlane1Para3",
            extends: None,
            description: Some(
                "timer counter about datalane1 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wakeup_d1",
                    description: Some(
                        "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver.",
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
            name: "Dlane2Para0",
            extends: None,
            description: Some(
                "timer counter about datalane2 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rst2enlptx_d2",
                    description: Some(
                        "the number of byteclk cycles that datalane2 wait to enable lptx_en after reset release.",
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
            name: "Dlane2Para1",
            extends: None,
            description: Some(
                "timer counter about datalane2 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_inittime_d2",
                    description: Some(
                        "the number of byteclk cycles that datalane2 drive lp-11 during initiaalization period.",
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
            name: "Dlane2Para2",
            extends: None,
            description: Some(
                "timer counter about datalane2 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_hsexit_d2",
                    description: Some(
                        "the number of byteclk cycles that the datalane2 stay at state hs-exit sate after a hs clock transmission.",
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
                    name: "t_hstrail_d2",
                    description: Some(
                        "the number of byteclk cycles that the datalane2 stay at hs-trail state during a hs clock transmission.",
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
                    name: "t_hszero_d2",
                    description: Some(
                        "the number of byteclk cycles that the datalane2 stay at hs-zero sate during a hs transmission.",
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
                    name: "t_hsprepare_d2",
                    description: Some(
                        "the number of byteclk cycles that the datalane2 stay at hs prepare state lp-00 during a hs transmission.",
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
            name: "Dlane2Para3",
            extends: None,
            description: Some(
                "timer counter about datalane2 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wakeup_d2",
                    description: Some(
                        "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver.",
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
            name: "Dlane3Para0",
            extends: None,
            description: Some(
                "timer counter about datalane3 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rst2enlptx_d3",
                    description: Some(
                        "the number of byteclk cycles that datalane3 wait to enable lptx_en after reset release.",
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
            name: "Dlane3Para1",
            extends: None,
            description: Some(
                "timer counter about datalane3 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_inittime_d3",
                    description: Some(
                        "the number of byteclk cycles that datalane3 drive lp-11 during initiaalization period.",
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
            name: "Dlane3Para2",
            extends: None,
            description: Some(
                "timer counter about datalane3 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_hsexit_d3",
                    description: Some(
                        "the number of byteclk cycles that the datalane3 stay at state hs-exit sate after a hs clock transmission.",
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
                    name: "t_hstrail_d3",
                    description: Some(
                        "the number of byteclk cycles that the datalane3 stay at hs-trail state during a hs clock transmission.",
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
                    name: "t_hszero_d3",
                    description: Some(
                        "the number of byteclk cycles that the datalane3 stay at hs-zero sate during a hs transmission.",
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
                    name: "t_hsprepare_d3",
                    description: Some(
                        "the number of byteclk cycles that the datalane3 stay at hs prepare state lp-00 during a hs transmission.",
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
            name: "Dlane3Para3",
            extends: None,
            description: Some(
                "timer counter about datalane3 parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_wakeup_d3",
                    description: Some(
                        "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver.",
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
            name: "InterfacePara",
            extends: None,
            description: Some(
                "dphy clock lane control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxvalidesc_extend_vld",
                    description: Some(
                        "the extend length of rxvalidesc.",
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
                    name: "txreadyesc_extend_vld",
                    description: Some(
                        "the extend length of txreadyesc.",
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
            name: "MiscPara",
            extends: None,
            description: Some(
                "dphy control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phyerr_mask",
                    description: Some(
                        "mask the phy error.",
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
                    name: "lane_num",
                    description: Some(
                        "the number of active data lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dll_sel",
                    description: Some(
                        "the phase select of clk_rxesc.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PcsReservedPinPara",
            extends: None,
            description: Some(
                "reserved the pins for pcs.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inv_dsi_rclk",
                    description: Some(
                        "pma clock dsi_rclk_i inverter signal.",
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
                    name: "inv_pclk",
                    description: Some(
                        "pclk inverter signal.",
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
                    name: "inv_clk_txesc",
                    description: Some(
                        "clk_txesc inverter signal.",
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
                    name: "inv_clk_txhs",
                    description: Some(
                        "clk_txhs inverter signal.",
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
                    name: "clk_txhs_sel_inner",
                    description: Some(
                        "select the clock source of clk_txhs in pcs.",
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
            ],
        },
        FieldSet {
            name: "PllCtrlPara0",
            extends: None,
            description: Some(
                "dphy pll control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsi_pixelclk_div",
                    description: Some(
                        "pixell clock divided from pll output.",
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
                    name: "pll_div",
                    description: Some(
                        "pll loop divider ratio control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refclk_div",
                    description: Some(
                        "input reference clock divider ratio control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rate",
                    description: Some(
                        "data reate control signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pll_lock",
                    description: Some(
                        "pll lock indication.",
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
            name: "PmaLaneSelPara",
            extends: None,
            description: Some(
                "pma about clock lane select parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pma_dlane1_sel",
                    description: Some(
                        "select the channel 1 as the data lane.",
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
                    name: "pma_dlane2_sel",
                    description: Some(
                        "select the channel 2 as the data lane.",
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
                    name: "pma_dlane3_sel",
                    description: Some(
                        "select the channel 3 as the data lane.",
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
                    name: "pma_dlane4_sel",
                    description: Some(
                        "select the channel 4 as the data lane.",
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
            name: "RcalCtrl",
            extends: None,
            description: Some(
                "dphy calibration control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcal_done",
                    description: Some(
                        "hs-tx output impedance trimming done indicator signal.",
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
                    name: "rcal_ctrl",
                    description: Some(
                        "resistor calibration control, reserved for test.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcal_trim",
                    description: Some(
                        "default value of hs-tx output resistance configure.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcal_en",
                    description: Some(
                        "enable hs-tx output impedance trimming.",
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
            ],
        },
        FieldSet {
            name: "TestPara0",
            extends: None,
            description: Some(
                "dphy test control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ft_sel",
                    description: Some(
                        "pt/ft test mode select.",
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
                    name: "fset_en",
                    description: Some(
                        "enable fast transmission between lp-tx and hs-tx.",
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
                    name: "atest_sel",
                    description: Some(
                        "analog test signal select.",
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
                    name: "atest_en",
                    description: Some(
                        "analog test signal enable.",
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
                    name: "bist_n_ok",
                    description: Some(
                        "indicate prbs7 bist test is ok.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bist_n_done",
                    description: Some(
                        "indicate prbs7 bist test is done.",
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
                    name: "error_num",
                    description: Some(
                        "the byte num of mismatch data of lane in bist mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TestPara1",
            extends: None,
            description: Some(
                "dphy bist test control parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prbs_sel",
                    description: Some(
                        "prbs generator and checker pattern select signal.",
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
                    name: "bist_sel",
                    description: Some(
                        "bist mode select.",
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
                    name: "bist_en",
                    description: Some(
                        "bist enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bist_bit_error",
                    description: Some(
                        "enable insert error in bist test pattern.",
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
                    name: "err_threshold",
                    description: Some(
                        "the threshold of prbs bit error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "check_num",
                    description: Some(
                        "the byte num of prbs bist check num.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TrimPara",
            extends: None,
            description: Some(
                "dphy trimming parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpcd_vref_trim",
                    description: Some(
                        "lp-cd input threshold voltage trimming for lane0.",
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
                    name: "lprx_vref_trim",
                    description: Some(
                        "lp-rx input threshold voltage trimming for lane0.",
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
                    name: "lptx_sr_trim",
                    description: Some(
                        "lp-tx output slew-rate trimming for lane0~4.",
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
                    name: "hstx_amp_trim",
                    description: Some(
                        "hs-tx output vod trimming for lane-0~4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
