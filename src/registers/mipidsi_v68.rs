use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "MipiDsi",
            extends: None,
            description: Some(
                "MIPI_DSI0.",
            ),
            items: &[
                BlockItem {
                    name: "version",
                    description: Some(
                        "version.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Version",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwr_up",
                    description: Some(
                        "power up.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PwrUp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clkmgr_cfg",
                    description: Some(
                        "divide lanebyteclk for timeout.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClkmgrCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_vcid",
                    description: Some(
                        "virtual channel ID for DPI traffic.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiVcid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_color_coding",
                    description: Some(
                        "dpi color coding.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiColorCoding",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_cfg_pol",
                    description: Some(
                        "the polarity of DPI signals.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiCfgPol",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_lp_cmd_tim",
                    description: Some(
                        "the timing for low-power commands sent while in video mode.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiLpCmdTim",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pckhdl_cfg",
                    description: Some(
                        "configures how EoTp, BTA, CRC and ECC to be used.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PckhdlCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gen_vcid",
                    description: Some(
                        "configures the virtual channel ID of read response to store and return to generic interface.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GenVcid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mode_cfg",
                    description: Some(
                        "configures the mode of operation between video or command mode.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ModeCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_mode_cfg",
                    description: Some(
                        "several aspect of video mode operation.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidModeCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_pkt_size",
                    description: Some(
                        "configures the video packet size.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidPktSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_num_chunks",
                    description: Some(
                        "configures the number of chunks to use.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidNumChunks",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_null_size",
                    description: Some(
                        "configures the size of null packets.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidNullSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hsa_time",
                    description: Some(
                        "configures the video HAS time.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHsaTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hbp_time",
                    description: Some(
                        "configure the video HBP time.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHbpTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hline_time",
                    description: Some(
                        "configures the overall time for each video line.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHlineTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vsa_lines",
                    description: Some(
                        "configures the vsa period.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVsaLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vbp_lines",
                    description: Some(
                        "configures the vbp period.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVbpLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vfp_lines",
                    description: Some(
                        "configures the vfp period.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVfpLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vactive_lines",
                    description: Some(
                        "configures the vertical resolution of video.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVactiveLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_mode_cfg",
                    description: Some(
                        "This register configures several aspect of command mode operation, tearing effect, acknowledge for each packet and the speed mode to transmit each Data Type related to commands.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdModeCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gen_hdr",
                    description: Some(
                        "sets the header for new packets sent using the generic interface.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GenHdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gen_pld_data",
                    description: Some(
                        "sets the payload for packets sent using the generic interface.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GenPldData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_pkt_status",
                    description: Some(
                        "information about the status of FIFOs related to DBI and Generic interface.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdPktStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "to_cnt_cfg",
                    description: Some(
                        "configures the trigger timeout errors.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ToCntCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hs_rd_to_cnt",
                    description: Some(
                        "configures the peripheral response timeout after high speed read operations.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HsRdToCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lp_rd_to_cnt",
                    description: Some(
                        "configures the peripheral response timeout after low-power read operation.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LpRdToCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hs_wr_to_cnt",
                    description: Some(
                        "configures the peripheral response timeout after high speed write operations.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HsWrToCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lp_wr_to_cnt",
                    description: Some(
                        "configures the peripheral response timeout after low power write operations.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LpWrToCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bta_to_cnt",
                    description: Some(
                        "configures the periphera response timeout after bus turnaround.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BtaToCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdf_3d",
                    description: Some(
                        "sotres 3d control information for vss packets in video mode.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdf3d",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpclk_ctrl",
                    description: Some(
                        "configures the possibility for using non continuous clock in the clock lane.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LpclkCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tmr_lpclk_cfg",
                    description: Some(
                        "sets the time that dsi host assumes in calculations for the clock lane to switch between high-speed and low-power.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTmrLpclkCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tmr_cfg",
                    description: Some(
                        "sets the time that dsi host assumes in calculations for data lanes to switch between hs to lp.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTmrCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_rstz",
                    description: Some(
                        "controls resets and the pll of d-phy.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyRstz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_if_cfg",
                    description: Some(
                        "configures the number of active lanes.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyIfCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_ulps_ctrl",
                    description: Some(
                        "configures entering and leaving ulps.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyUlpsCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tx_triggers",
                    description: Some(
                        "configures the pins that activate triggers in the d-phy.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTxTriggers",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_status",
                    description: Some(
                        "contains information about the status of the d-phy.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tst_ctrl0",
                    description: Some(
                        "controls clock and clear pins of the d-phy vendor specific interface.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTstCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tst_ctrl1",
                    description: Some(
                        "controls data and enable pins of the d-phy.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTstCtrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st0",
                    description: Some(
                        "controls the status of interrupt.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntSt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st1",
                    description: Some(
                        "the interrupt source related to timeout etc.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntSt1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk0",
                    description: Some(
                        "configures masks for the sources of interrupt that affec int_st0.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMsk0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk1",
                    description: Some(
                        "configures masks for int_st1.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMsk1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cal",
                    description: Some(
                        "controls the skew calibration of D-phy.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force0",
                    description: Some(
                        "forces that affect the int_st0 register.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForce0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force1",
                    description: Some(
                        "forces interrupts that affect the int_st1 register.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForce1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_tmr_rd",
                    description: Some(
                        "configures times related to PHY to perform some operations in lane byte clock cycle.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyTmrRd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "auto_ulps_min_time",
                    description: Some(
                        "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AutoUlpsMinTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_mode",
                    description: Some(
                        "select phy mode.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_shadow_ctrl",
                    description: Some(
                        "controls dpi shadow feature.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidShadowCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_vcid_act",
                    description: Some(
                        "holds the value that controller is using for DPI_VCID.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiVcidAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_color_coding_act",
                    description: Some(
                        "holds the value that controller is using for DPI_COLOR_CODING.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiColorCodingAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dpi_lp_cmd_tim_act",
                    description: Some(
                        "holds value that controller is using for dpi_lp_cmd_time.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DpiLpCmdTimAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_mode_cfg_act",
                    description: Some(
                        "holds value that controller is using for vid_mode_cfg.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidModeCfgAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_pkt_size_act",
                    description: Some(
                        "holds value that controller is using for vid_pkt_size.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidPktSizeAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_num_chunks_act",
                    description: Some(
                        "holds value that controller is using for vid_num_chunks.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidNumChunksAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_null_size_act",
                    description: Some(
                        "holds the value that controller is using for vid_null_size.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidNullSizeAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hsa_time_act",
                    description: Some(
                        "the value of vid_hsa_time.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHsaTimeAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hbp_time_act",
                    description: Some(
                        "the value that controller is using for vid_hbp_time.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHbpTimeAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_hline_time_act",
                    description: Some(
                        "the value for vid_hline_time.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidHlineTimeAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vsa_lines_act",
                    description: Some(
                        "value for vid_vsa_lines.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVsaLinesAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vbp_lines_act",
                    description: Some(
                        "value for vid_vbp_lines.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVbpLinesAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vfp_lines_act",
                    description: Some(
                        "value for vid_vfp_lines.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVfpLinesAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_vactive_lines_act",
                    description: Some(
                        "value for vid_vactive_lines.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidVactiveLinesAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid_pkt_status",
                    description: Some(
                        "status of fifo related to dpi.",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VidPktStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdf_3d_act",
                    description: Some(
                        "value for sdf_3d.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdf3dAct",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AutoUlpsMinTime",
            extends: None,
            description: Some(
                "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ulps_min_time",
                    description: Some(
                        "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BtaToCnt",
            extends: None,
            description: Some(
                "configures the periphera response timeout after bus turnaround.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bta_to_cnt",
                    description: Some(
                        "sets the period for which dsi host keeps the link still after completing a bus turnaround.",
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
            name: "ClkmgrCfg",
            extends: None,
            description: Some(
                "divide lanebyteclk for timeout.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_esc_clk_division",
                    description: Some(
                        "the division factor for the TX Escape clock source lanebyteclk.",
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
                    name: "to_clk_division",
                    description: Some(
                        "the timeout clock division factor for HS to LP and LP to HS transition error.",
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
            name: "CmdModeCfg",
            extends: None,
            description: Some(
                "This register configures several aspect of command mode operation, tearing effect, acknowledge for each packet and the speed mode to transmit each Data Type related to commands.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tear_fx_en",
                    description: Some(
                        "When set to 1, this bit enables the tearing effect acknowledge request.",
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
                    name: "ack_rqst_en",
                    description: Some(
                        "When set to 1, this bit enables the acknowledge request after each packet transmission.",
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
                    name: "gen_sw_0p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_sw_1p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_sw_2p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_sr_0p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_sr_1p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_sr_2p_tx",
                    description: Some(
                        "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "gen_lw_tx",
                    description: Some(
                        "This bit configures the Generic long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "dcs_sw_0p_tx",
                    description: Some(
                        "This bit configures the DCS short write packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "dcs_sw_1p_tx",
                    description: Some(
                        "This bit configures the DCS short write packet with one parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "dcs_sr_0p_tx",
                    description: Some(
                        "This bit configures the DCS short read packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "dcs_lw_tx",
                    description: Some(
                        "This bit configures the DCS long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
                    name: "max_rd_pkt_size",
                    description: Some(
                        "This bit configures the maximum read packet size command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power.",
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
            name: "CmdPktStatus",
            extends: None,
            description: Some(
                "information about the status of FIFOs related to DBI and Generic interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gen_cmd_empty",
                    description: Some(
                        "indicates the empty status of the generic command FIFO.",
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
                    name: "gen_cmd_full",
                    description: Some(
                        "indicates the full status of the generic command FIFO.",
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
                    name: "gen_pld_w_empty",
                    description: Some(
                        "indicates the empty status of the generic write payload FIFO.",
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
                    name: "gen_pld_w_full",
                    description: Some(
                        "indicates the full status of the generic write payload FIFO.",
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
                    name: "gen_pld_r_empty",
                    description: Some(
                        "indicates the empty status of the generic read payload FIFO.",
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
                    name: "gen_pld_r_full",
                    description: Some(
                        "indicates the full status of the generic read payoad FIFO.",
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
                    name: "gen_rd_cmd_busy",
                    description: Some(
                        "indicates a read command is issued and the entire response is not sotred in the FIFO.",
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
                    name: "gen_buff_cmd_empty",
                    description: Some(
                        "the empty status of the generic command internal buffer.",
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
                    name: "gen_buff_cmd_full",
                    description: Some(
                        "the full status of the generic command internal buffer.",
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
                    name: "gen_buff_pld_empty",
                    description: Some(
                        "the empty status of the generic payload internal buffer.",
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
                    name: "gen_buff_pld_full",
                    description: Some(
                        "the full status of the generic payload internal buffer.",
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
            ],
        },
        FieldSet {
            name: "DpiCfgPol",
            extends: None,
            description: Some(
                "the polarity of DPI signals.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dataen_active_low",
                    description: Some(
                        "configures the data enable pin active low.",
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
                    name: "vsync_active_low",
                    description: Some(
                        "configures the vertical synchronism pin as active low.",
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
                    name: "hsync_active_low",
                    description: Some(
                        "configures the horizontal synchronism pin as active low.",
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
                    name: "shutd_active_low",
                    description: Some(
                        "configures the shutdown pin as active low.",
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
                    name: "colorm_active_low",
                    description: Some(
                        "configures the color mode pin as active low.",
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
            name: "DpiColorCoding",
            extends: None,
            description: Some(
                "dpi color coding.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpi_color_coding",
                    description: Some(
                        "configures the DPI color for video mode.",
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
                    name: "loosely18_en",
                    description: Some(
                        "when set to 1, this bit activates loosely packed variant to 18-bit configurations.",
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
            name: "DpiColorCodingAct",
            extends: None,
            description: Some(
                "holds the value that controller is using for DPI_COLOR_CODING.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dip_color_coding",
                    description: Some(
                        "configures the DPI color for video mode.",
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
                    name: "loosely18_en",
                    description: Some(
                        "avtivates loosely packed variant to 18-bit configuration.",
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
            name: "DpiLpCmdTim",
            extends: None,
            description: Some(
                "the timing for low-power commands sent while in video mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "invact_lpcmd_time",
                    description: Some(
                        "transmission of commands in low-power mode, defines the size in bytes of the largest packet that can fit in a line during the VACT region.",
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
                    name: "outvact_lpcmd_time",
                    description: Some(
                        "transmission of commands in low-power mode, defines the size in bytes of the largest pachet that can fit in a line during the VSA VBP and VFP;.",
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
            name: "DpiLpCmdTimAct",
            extends: None,
            description: Some(
                "holds value that controller is using for dpi_lp_cmd_time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "invact_lpcmd_time",
                    description: Some(
                        "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the vact regions.",
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
                    name: "outvact_lpcmd_time",
                    description: Some(
                        "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the VSA VBP and VFP regions.",
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
            name: "DpiVcid",
            extends: None,
            description: Some(
                "virtual channel ID for DPI traffic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpi_vcid",
                    description: Some(
                        "the DPI virtual channel id to the video mode packets.",
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
            ],
        },
        FieldSet {
            name: "DpiVcidAct",
            extends: None,
            description: Some(
                "holds the value that controller is using for DPI_VCID.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpi_vcid",
                    description: Some(
                        "specifies the DPI virtual channel id that is indexed to the video mode packets.",
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
            ],
        },
        FieldSet {
            name: "GenHdr",
            extends: None,
            description: Some(
                "sets the header for new packets sent using the generic interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gen_dt",
                    description: Some(
                        "configures the packet data type of the header packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gen_vc",
                    description: Some(
                        "configures the virtual channel ID of the header packet.",
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
                    name: "gen_wc_lsbyte",
                    description: Some(
                        "configures the least significant byte of the header packet's word count for long packets or data0 for short packets.",
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
                    name: "gen_wc_msbyte",
                    description: Some(
                        "configures the most significant byte of the header packet's word count for long packets or data 1 for shout packets.",
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
            name: "GenPldData",
            extends: None,
            description: Some(
                "sets the payload for packets sent using the generic interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gen_pld_b1",
                    description: Some(
                        "indicates byte1 of the packet payload.",
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
                    name: "gen_pld_b2",
                    description: Some(
                        "indicates byte2 of the packet payload.",
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
                    name: "gen_pld_b3",
                    description: Some(
                        "indicates byte3 of the packet payload.",
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
                    name: "gen_pld_b4",
                    description: Some(
                        "indicates byte4 of the packet payload.",
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
            name: "GenVcid",
            extends: None,
            description: Some(
                "configures the virtual channel ID of read response to store and return to generic interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gen_vcid_rx",
                    description: Some(
                        "indicates the generic interface read-back virtual channel identication.",
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
                    name: "gen_vcid_tear_auto",
                    description: Some(
                        "indicates the virtual channel identification for tear effect by hardware.",
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
                    name: "gen_vcid_tx_auto",
                    description: Some(
                        "indicates the generic interface virtual channel identification where generic packet is automatically generated and transmitted.",
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
            ],
        },
        FieldSet {
            name: "HsRdToCnt",
            extends: None,
            description: Some(
                "configures the peripheral response timeout after high speed read operations.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hs_rd_to_cnt",
                    description: Some(
                        "sets a period for which DWC_mipi_dsi_host keeps the link still after sending a high speed read operation;.",
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
            name: "HsWrToCnt",
            extends: None,
            description: Some(
                "configures the peripheral response timeout after high speed write operations.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hs_wr_to_cnt",
                    description: Some(
                        "sets the period for which dwc_mipi_dsi_host keeps the link still after sending a high speed write operation.",
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
            name: "IntForce0",
            extends: None,
            description: Some(
                "forces that affect the int_st0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_ack_with_err0",
                    description: Some(
                        "force the SoT serror from the acknowledge error report.",
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
                    name: "force_ack_with_err1",
                    description: Some(
                        "force the SoT sync error from the acknowledge error report.",
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
                    name: "force_ack_with_err2",
                    description: Some(
                        "force the EoT sync error from the acknowledge error report.",
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
                    name: "force_ack_with_err3",
                    description: Some(
                        "force the Escap mode entry command error from the acknowledge error report.",
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
                    name: "force_ack_with_err4",
                    description: Some(
                        "force the LP transmit sync error from the acknowledge error report.",
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
                    name: "force_ack_with_err5",
                    description: Some(
                        "force the peripheral timeout error from the acknowledge error report.",
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
                    name: "force_ack_with_err6",
                    description: Some(
                        "force the false control error fro the acknowledge error report.",
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
                    name: "force_ack_with_err7",
                    description: Some(
                        "force the reserved from the acknowledge error report.",
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
                    name: "force_ack_with_err8",
                    description: Some(
                        "force the ecc error sigle-bit from the acknowledge error report.",
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
                    name: "force_ack_with_err_9",
                    description: Some(
                        "force the ECC error multi-bit from the acknowledge error report.",
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
                    name: "force_ack_with_err_10",
                    description: Some(
                        "force the checksum error from the acknowledge error report.",
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
                    name: "force_ack_with_err_11",
                    description: Some(
                        "force the not recongnized dsi data type from the acknowledge error report.",
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
                    name: "force_ack_with_err_12",
                    description: Some(
                        "force the dsi vc id invalid from the acknowledge error report.",
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
                    name: "force_ack_with_err_13",
                    description: Some(
                        "force the invalid transmission length from the acknowledge error report.",
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
                    name: "force_ack_with_err_14",
                    description: Some(
                        "force the reserved from the acknowledge error report.",
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
                    name: "force_ack_with_err_15",
                    description: Some(
                        "force the DSI protocal violation from the acknowledge error report.",
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
                    name: "force_dphy_errors_0",
                    description: Some(
                        "force ErrEsc escape entry error from lane0.",
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
                    name: "force_dphy_errors_1",
                    description: Some(
                        "force ErrSyncEsc low-power data transmission synchronization error from lane 0.",
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
                    name: "force_dphy_errors_2",
                    description: Some(
                        "force control error ErrControl from lane0.",
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
                    name: "force_dphy_errors_3",
                    description: Some(
                        "force LP0 contention error ErrContentionLP0 from lane0.",
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
                    name: "force_dphy_errors_4",
                    description: Some(
                        "force LP1 contention error ErrContentionLP1 from lane0.",
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
            name: "IntForce1",
            extends: None,
            description: Some(
                "forces interrupts that affect the int_st1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_to_hs_tx",
                    description: Some(
                        "force that the high-speed transmission timeout counter reached the end and contention has been detected.",
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
                    name: "force_to_lp_tx",
                    description: Some(
                        "force that the low-power reception timeout counter reached the end and contention has been detected.",
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
                    name: "force_ecc_sigle_err",
                    description: Some(
                        "force that the ECC single error has been detected and corrected in a reveived packet.",
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
                    name: "force_ecc_multi_err",
                    description: Some(
                        "force that the ECC multiple error has been detected in a revieved packet.",
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
                    name: "force_crc_err",
                    description: Some(
                        "force that the CRC error has been detected in the reveived packet payload.",
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
                    name: "force_pkt_size_err",
                    description: Some(
                        "force that the packet size error has been detected during the packet reception.",
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
                    name: "force_eopt_err",
                    description: Some(
                        "force that the EoTp packet has not been received at the end of the incoming peripheral transmission.",
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
                    name: "force_dpi_bpld_wr_err",
                    description: Some(
                        "force the payload FIFO is full during a DPI pixel line storage.",
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
                    name: "force_gen_cmd_wr_err",
                    description: Some(
                        "force the system tried to write a command and FIFO is full.",
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
                    name: "force_gen_pld_wr_err",
                    description: Some(
                        "force the system tried to write a payload and FIFO is full.",
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
                    name: "force_gen_pld_send_err",
                    description: Some(
                        "force the payload FIFO become empty when packet build.",
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
                    name: "force_gen_pld_rd_err",
                    description: Some(
                        "force that during a DCS read data, the payload FIFO becomes empty.",
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
                    name: "force_gen_pld_recev_err",
                    description: Some(
                        "force that during a generic interface packet read back, the payload FIFO full.",
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
                    name: "force_dpi_buff_pld_under",
                    description: Some(
                        "force an underflow when reading payload to build dsi packet for video mode.",
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
                    name: "force_tear_request_err",
                    description: Some(
                        "force tear_request has occurred but tear effect is not active in dsi host and device.",
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
            name: "IntMsk0",
            extends: None,
            description: Some(
                "configures masks for the sources of interrupt that affec int_st0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_ack_with_err0",
                    description: Some(
                        "disable the SoT serror from the acknowledge error report.",
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
                    name: "mask_ack_with_err1",
                    description: Some(
                        "disable the SoT sync error from the acknowledge error report.",
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
                    name: "mask_ack_with_err2",
                    description: Some(
                        "disable the EoT sync error from the acknowledge error report.",
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
                    name: "mask_ack_with_err3",
                    description: Some(
                        "disable the Escap mode entry command error from the acknowledge error report.",
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
                    name: "mask_ack_with_err4",
                    description: Some(
                        "disable the LP transmit sync error from the acknowledge error report.",
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
                    name: "mask_ack_with_err5",
                    description: Some(
                        "disable the peripheral timeout error from the acknowledge error report.",
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
                    name: "mask_ack_with_err6",
                    description: Some(
                        "disable the false control error fro the acknowledge error report.",
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
                    name: "mask_ack_with_err7",
                    description: Some(
                        "disable the reserved from the acknowledge error report.",
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
                    name: "mask_ack_with_err8",
                    description: Some(
                        "disable the ecc error sigle-bit from the acknowledge error report.",
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
                    name: "mask_ack_with_err_9",
                    description: Some(
                        "disable the ECC error multi-bit from the acknowledge error report.",
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
                    name: "mask_ack_with_err_10",
                    description: Some(
                        "disable the checksum error from the acknowledge error report.",
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
                    name: "mask_ack_with_err_11",
                    description: Some(
                        "disable the not recongnized dsi data type from the acknowledge error report.",
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
                    name: "mask_ack_with_err_12",
                    description: Some(
                        "disable the dsi vc id invalid from the acknowledge error report.",
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
                    name: "mask_ack_with_err_13",
                    description: Some(
                        "disable the invalid transmission length from the acknowledge error report.",
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
                    name: "mask_ack_with_err_14",
                    description: Some(
                        "disable the reserved from the acknowledge error report.",
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
                    name: "mask_ack_with_err_15",
                    description: Some(
                        "disable the DSI protocal violation from the acknowledge error report.",
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
                    name: "mask_dphy_errors_0",
                    description: Some(
                        "disable ErrEsc escape entry error from lane0.",
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
                    name: "mask_dphy_errors_1",
                    description: Some(
                        "disable ErrSyncEsc low-power data transmission synchronization error from lane 0.",
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
                    name: "mask_dphy_errors_2",
                    description: Some(
                        "disable control error ErrControl from lane0.",
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
                    name: "mask_dphy_errors_3",
                    description: Some(
                        "disable LP0 contention error ErrContentionLP0 from lane0.",
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
                    name: "mask_dphy_errors_4",
                    description: Some(
                        "disable LP1 contention error ErrContentionLP1 from lane0.",
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
            name: "IntMsk1",
            extends: None,
            description: Some(
                "configures masks for int_st1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_to_hs_tx",
                    description: Some(
                        "disable that the high-speed transmission timeout counter reached the end and contention has been detected.",
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
                    name: "mask_to_lp_tx",
                    description: Some(
                        "disable that the low-power reception timeout counter reached the end and contention has been detected.",
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
                    name: "mask_ecc_sigle_err",
                    description: Some(
                        "disable that the ECC single error has been detected and corrected in a reveived packet.",
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
                    name: "mask_ecc_multi_err",
                    description: Some(
                        "disable that the ECC multiple error has been detected in a revieved packet.",
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
                    name: "mask_crc_err",
                    description: Some(
                        "disable that the CRC error has been detected in the reveived packet payload.",
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
                    name: "mask_pkt_size_err",
                    description: Some(
                        "disable that the packet size error has been detected during the packet reception.",
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
                    name: "mask_eopt_err",
                    description: Some(
                        "disable that the EoTp packet has not been received at the end of the incoming peripheral transmission.",
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
                    name: "mask_dpi_bpld_wr_err",
                    description: Some(
                        "disable the payload FIFO is full during a DPI pixel line storage.",
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
                    name: "mask_gen_cmd_wr_err",
                    description: Some(
                        "disable the system tried to write a command and FIFO is full.",
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
                    name: "mask_gen_pld_wr_err",
                    description: Some(
                        "disable the system tried to write a payload and FIFO is full.",
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
                    name: "mask_gen_pld_send_err",
                    description: Some(
                        "disable the payload FIFO become empty when packet build.",
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
                    name: "mask_gen_pld_rd_err",
                    description: Some(
                        "disable that during a DCS read data, the payload FIFO becomes empty.",
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
                    name: "mask_gen_pld_recev_err",
                    description: Some(
                        "disable that during a generic interface packet read back, the payload FIFO full.",
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
                    name: "mask_dpi_buff_pld_under",
                    description: Some(
                        "disable an underflow when reading payload to build dsi packet for video mode.",
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
                    name: "mask_tear_request_err",
                    description: Some(
                        "disable tear_request has occurred but tear effect is not active in dsi host and device.",
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
            name: "IntSt0",
            extends: None,
            description: Some(
                "controls the status of interrupt.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ack_with_err0",
                    description: Some(
                        "retrives the SoT serror from the acknowledge error report.",
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
                    name: "ack_with_err1",
                    description: Some(
                        "retrives the SoT sync error from the acknowledge error report.",
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
                    name: "ack_with_err2",
                    description: Some(
                        "retrives the EoT sync error from the acknowledge error report.",
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
                    name: "ack_with_err3",
                    description: Some(
                        "retrives the Escap mode entry command error from the acknowledge error report.",
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
                    name: "ack_with_err4",
                    description: Some(
                        "retrives the LP transmit sync error from the acknowledge error report.",
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
                    name: "ack_with_err5",
                    description: Some(
                        "retrives the peripheral timeout error from the acknowledge error report.",
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
                    name: "ack_with_err6",
                    description: Some(
                        "retrieves the false control error fro the acknowledge error report.",
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
                    name: "ack_with_err7",
                    description: Some(
                        "retrieves the reserved from the acknowledge error report.",
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
                    name: "ack_with_err8",
                    description: Some(
                        "retrives the ecc error sigle-bit from the acknowledge error report.",
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
                    name: "ack_with_err_9",
                    description: Some(
                        "retrives the ECC error multi-bit from the acknowledge error report.",
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
                    name: "ack_with_err_10",
                    description: Some(
                        "retrives the checksum error from the acknowledge error report.",
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
                    name: "ack_with_err_11",
                    description: Some(
                        "retrives the not recongnized dsi data type from the acknowledge error report.",
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
                    name: "ack_with_err_12",
                    description: Some(
                        "retrieves the dsi vc id invalid from the acknowledge error report.",
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
                    name: "ack_with_err_13",
                    description: Some(
                        "retrives the invalid transmission length from the acknowledge error report.",
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
                    name: "ack_with_err_14",
                    description: Some(
                        "retrives the reserved from the acknowledge error report.",
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
                    name: "ack_with_err_15",
                    description: Some(
                        "retrives the DSI protocal violation from the acknowledge error report.",
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
                    name: "dphy_errors_0",
                    description: Some(
                        "indicates ErrEsc escape entry error from lane0.",
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
                    name: "dphy_errors_1",
                    description: Some(
                        "indicates ErrSyncEsc low-power data transmission synchronization error from lane 0.",
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
                    name: "dphy_errors_2",
                    description: Some(
                        "indicates control error ErrControl from lane0.",
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
                    name: "dphy_errors_3",
                    description: Some(
                        "indicates LP0 contention error ErrContentionLP0 from lane0.",
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
                    name: "dphy_errors_4",
                    description: Some(
                        "indicates LP1 contention error ErrContentionLP1 from lane0.",
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
            name: "IntSt1",
            extends: None,
            description: Some(
                "the interrupt source related to timeout etc.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to_hs_tx",
                    description: Some(
                        "indicates that the high-speed transmission timeout counter reached the end and contention has been detected.",
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
                    name: "to_lp_tx",
                    description: Some(
                        "indicates that the low-power reception timeout counter reached the end and contention has been detected.",
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
                    name: "ecc_sigle_err",
                    description: Some(
                        "indicates that the ECC single error has been detected and corrected in a reveived packet.",
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
                    name: "ecc_multi_err",
                    description: Some(
                        "indicates that the ECC multiple error has been detected in a revieved packet.",
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
                    name: "crc_err",
                    description: Some(
                        "indicates that the CRC error has been detected in the reveived packet payload.",
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
                    name: "pkt_size_err",
                    description: Some(
                        "indicates that the packet size error has been detected during the packet reception.",
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
                    name: "eopt_err",
                    description: Some(
                        "indicates that the EoTp packet has not been received at the end of the incoming peripheral transmission.",
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
                    name: "dpi_bpld_wr_err",
                    description: Some(
                        "indicates the payload FIFO is full during a DPI pixel line storage.",
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
                    name: "gen_cmd_wr_err",
                    description: Some(
                        "indicates the system tried to write a command and FIFO is full.",
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
                    name: "gen_pld_wr_err",
                    description: Some(
                        "indicates the system tried to write a payload and FIFO is full.",
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
                    name: "gen_pld_send_err",
                    description: Some(
                        "indicates the payload FIFO become empty when packet build.",
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
                    name: "gen_pld_rd_err",
                    description: Some(
                        "indicates that during a DCS read data, the payload FIFO becomes empty.",
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
                    name: "gen_pld_recev_err",
                    description: Some(
                        "indicates that during a generic interface packet read back, the payload FIFO full.",
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
                    name: "dpi_buff_pld_under",
                    description: Some(
                        "indicates an underflow when reading payload to build dsi packet for video mode.",
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
                    name: "tear_request_err",
                    description: Some(
                        "indicates tear_request has occurred but tear effect is not active in dsi host and device.",
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
            name: "LpRdToCnt",
            extends: None,
            description: Some(
                "configures the peripheral response timeout after low-power read operation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lp_rd_to_cnt",
                    description: Some(
                        "sets a period for which dwc_mipi_dsi_host keeps the link still after sending a low power read operation.",
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
            name: "LpWrToCnt",
            extends: None,
            description: Some(
                "configures the peripheral response timeout after low power write operations.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lp_wr_to_cnt",
                    description: Some(
                        "sets the period for which dsi host keeps the link still after sending a low power write operation.",
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
            name: "LpclkCtrl",
            extends: None,
            description: Some(
                "configures the possibility for using non continuous clock in the clock lane.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_txrequestclkhs",
                    description: Some(
                        "controls the D-PHY PPI txrequestclkhs signal.",
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
                    name: "auto_clklane_ctrl",
                    description: Some(
                        "enables the automatic mechanism to stop providing clock in the clock lane.",
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
            name: "ModeCfg",
            extends: None,
            description: Some(
                "configures the mode of operation between video or command mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd_video_mode",
                    description: Some(
                        "0x0: video mode 0x1: command mode.",
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
            ],
        },
        FieldSet {
            name: "PckhdlCfg",
            extends: None,
            description: Some(
                "configures how EoTp, BTA, CRC and ECC to be used.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eotp_tx_en",
                    description: Some(
                        "enable the EoTp transmission in high-speed.",
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
                    name: "eotp_rx_en",
                    description: Some(
                        "enable the EoTp reception.",
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
                    name: "bta_en",
                    description: Some(
                        "enable the bus turn-around request.",
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
                    name: "ecc_rx_en",
                    description: Some(
                        "enable the ecc reception error correction and reporting.",
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
                    name: "crc_rx_en",
                    description: Some(
                        "enable the crc reception and error reporting.",
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
                    name: "eotp_tx_lp_en",
                    description: Some(
                        "enable the EoTp transmission in low-power.",
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
            name: "PhyCal",
            extends: None,
            description: Some(
                "controls the skew calibration of D-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txskewcalhs",
                    description: Some(
                        "High-speed skew calibration is started when txskewcalhs is set high (assuming that PHY is in Stop state).",
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
            ],
        },
        FieldSet {
            name: "PhyIfCfg",
            extends: None,
            description: Some(
                "configures the number of active lanes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "n_lanes",
                    description: Some(
                        "configures the number of active data lanes.",
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
                    name: "phy_stop_wait_time",
                    description: Some(
                        "configures the minimum time phy needs to stay in stopstate before requesting an highspeed transmission.",
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
            name: "PhyMode",
            extends: None,
            description: Some(
                "select phy mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_mode",
                    description: Some(
                        "sel DPHY or CPHY.",
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
            ],
        },
        FieldSet {
            name: "PhyRstz",
            extends: None,
            description: Some(
                "controls resets and the pll of d-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_shutdownz",
                    description: Some(
                        "places the dphy macro in power down mode when set to 0.",
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
                    name: "phy_rstz",
                    description: Some(
                        "make the dphy in reset state when set to 0.",
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
                    name: "phy_enableclk",
                    description: Some(
                        "enable dphy clock lane.",
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
                    name: "phy_forcepll",
                    description: Some(
                        "when the d-phy is in ulps, enable the d-phy pll.",
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
            name: "PhyStatus",
            extends: None,
            description: Some(
                "contains information about the status of the d-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_lock",
                    description: Some(
                        "This bit indicates the status of phylock D-PHY signal.",
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
                    name: "phy_direction",
                    description: Some(
                        "This bit indicates the status of phydirection D-PHY signal.",
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
                    name: "phy_stopstateclklane",
                    description: Some(
                        "This bit indicates the status of phystopstateclklane D-PHY signal.",
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
                    name: "phy_ulpsactivenotclk",
                    description: Some(
                        "This bit indicates the status of phyulpsactivenotclk D-PHY signal.",
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
                    name: "phy_stopstate0lane",
                    description: Some(
                        "This bit indicates the status of phystopstate0lane D-PHY signal.",
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
                    name: "phy_ulpsactivenot0lane",
                    description: Some(
                        "This bit indicates the status of ulpsactivenot0lane D-PHY signal.",
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
                    name: "phy_rxulpsesc0lane",
                    description: Some(
                        "This bit indicates the status of rxulpsesc0lane D-PHY signa.",
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
                    name: "phy_stopstate1lane",
                    description: Some(
                        "This bit indicates the status of phystopstate1lane D-PHY signal.",
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
                    name: "phy_ulpsactivenot1lane",
                    description: Some(
                        "This bit indicates the status of ulpsactivenot1lane D-PHY signal.",
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
                    name: "phy_stopstate2lane",
                    description: Some(
                        "This bit indicates the status of phystopstate2lane D-PHY signal.",
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
                    name: "phy_ulpsactivenot2lane",
                    description: Some(
                        "This bit indicates the status of ulpsactivenot2lane D-PHY signa.",
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
                    name: "phy_stopstate3lane",
                    description: Some(
                        "This bit indicates the status of phystopstate3lane D-PHY signal.",
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
                    name: "phy_ulpsactivenot3lane",
                    description: Some(
                        "indicates the status of ulpsactivenot3lane d-phy signal.",
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
            ],
        },
        FieldSet {
            name: "PhyTmrCfg",
            extends: None,
            description: Some(
                "sets the time that dsi host assumes in calculations for data lanes to switch between hs to lp.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_lp2hs_time",
                    description: Some(
                        "This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phy_hs2lp_time",
                    description: Some(
                        "This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PhyTmrLpclkCfg",
            extends: None,
            description: Some(
                "sets the time that dsi host assumes in calculations for the clock lane to switch between high-speed and low-power.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_clklp2hs_time",
                    description: Some(
                        "configures the maximum time that the d-phy clock lane takes to go from low-power to high-speed transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phy_clkhs2lp_time",
                    description: Some(
                        "configures the maximum time that the d-phy clock lane takes to go from high-speed to low-power transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PhyTmrRd",
            extends: None,
            description: Some(
                "configures times related to PHY to perform some operations in lane byte clock cycle.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_rd_time",
                    description: Some(
                        "the maximum time required to perform a read command in lane byte clock cycles.",
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
            name: "PhyTstCtrl0",
            extends: None,
            description: Some(
                "controls clock and clear pins of the d-phy vendor specific interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_testclr",
                    description: Some(
                        "reserve.",
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
                    name: "phy_testclk",
                    description: Some(
                        "reserve.",
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
            name: "PhyTstCtrl1",
            extends: None,
            description: Some(
                "controls data and enable pins of the d-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_testdin",
                    description: Some(
                        "reserve.",
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
                    name: "phy_testdout",
                    description: Some(
                        "reserve.",
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
                    name: "phy_testen",
                    description: Some(
                        "reserve.",
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
            ],
        },
        FieldSet {
            name: "PhyTxTriggers",
            extends: None,
            description: Some(
                "configures the pins that activate triggers in the d-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_tx_triggers",
                    description: Some(
                        "controls the trigger transmissions.",
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
            name: "PhyUlpsCtrl",
            extends: None,
            description: Some(
                "configures entering and leaving ulps.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_txrequlpsclk",
                    description: Some(
                        "ulps mode request on clock lane.",
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
                    name: "phy_txexitulpsclk",
                    description: Some(
                        "ulps mode exit on clock lane.",
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
                    name: "phy_txrequlpslan",
                    description: Some(
                        "ulps mode request on all active data lanes.",
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
                    name: "phy_txexitulpslan",
                    description: Some(
                        "ulps mode exit on all active data lanes.",
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
            name: "PwrUp",
            extends: None,
            description: Some(
                "power up.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shutdownz",
                    description: Some(
                        "0x0: reset the core 0x1: power up the core.",
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
            ],
        },
        FieldSet {
            name: "Sdf3d",
            extends: None,
            description: Some(
                "sotres 3d control information for vss packets in video mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode_3d",
                    description: Some(
                        "defines 3D mode on/off.",
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
                    name: "format_3d",
                    description: Some(
                        "defines 3D image format.",
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
                    name: "second_vsync",
                    description: Some(
                        "defines whether there is a second VSYNC pulse.",
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
                    name: "right_first",
                    description: Some(
                        "0x0: left eye is sent first 0x1:right eye is sent first.",
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
                    name: "send_3d_cfg",
                    description: Some(
                        "set the next vss packet to include 3d control payload in every vss packet.",
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
            ],
        },
        FieldSet {
            name: "Sdf3dAct",
            extends: None,
            description: Some(
                "value for sdf_3d.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode_3d",
                    description: Some(
                        "This field specifies 3D Mode On/Off and Display Orientation.",
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
                    name: "format_3d",
                    description: Some(
                        "This field specifies 3D Image Format.",
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
                    name: "second_vsync",
                    description: Some(
                        "This field specifies whether there is a second VSYNC pulse between Left and Right Images, when 3D Image Format is Frame-based.",
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
                    name: "right_first",
                    description: Some(
                        "This bit specifies the left/right order.",
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
                    name: "send_3d_cfg",
                    description: Some(
                        "When set, causes the next VSS packet to include 3D control payload in every VSS packet.",
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
            ],
        },
        FieldSet {
            name: "ToCntCfg",
            extends: None,
            description: Some(
                "configures the trigger timeout errors.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lprx_to_cnt",
                    description: Some(
                        "configures the timeout counter that triggers a low power reception timeout contention detection.",
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
                    name: "hstx_to_cnt",
                    description: Some(
                        "configures the timeout counter that triggers a high speed transmission timeout contention detection.",
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
            name: "Version",
            extends: None,
            description: Some(
                "version.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "version of DSI.",
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
            name: "VidHbpTime",
            extends: None,
            description: Some(
                "configure the video HBP time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hpb_time",
                    description: Some(
                        "configures the Horizontal back porch period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidHbpTimeAct",
            extends: None,
            description: Some(
                "the value that controller is using for vid_hbp_time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hbp_time",
                    description: Some(
                        "the horizontal back porch period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidHlineTime",
            extends: None,
            description: Some(
                "configures the overall time for each video line.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hline_time",
                    description: Some(
                        "configures the size of the total line time in lane byte clock cycles.",
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
            name: "VidHlineTimeAct",
            extends: None,
            description: Some(
                "the value for vid_hline_time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hline_time",
                    description: Some(
                        "the size of total line: hsa+hbp+hact+hfp.",
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
            name: "VidHsaTime",
            extends: None,
            description: Some(
                "configures the video HAS time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hsa_time",
                    description: Some(
                        "configure the Horizontal synchronism active period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidHsaTimeAct",
            extends: None,
            description: Some(
                "the value of vid_hsa_time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_hsa_time",
                    description: Some(
                        "the horizontal synchronism active period in lane byte clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidModeCfg",
            extends: None,
            description: Some(
                "several aspect of video mode operation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_mode_type",
                    description: Some(
                        "indicates the video mode transmission type.",
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
                    name: "lp_vsa_en",
                    description: Some(
                        "enable the return to low-power inside the VSA period when timing allows.",
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
                    name: "lp_vbp_en",
                    description: Some(
                        "enable the return to low-power inside the VBP period when timing allows.",
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
                    name: "lp_vfp_en",
                    description: Some(
                        "enable the return to low-power inside the VFP period when timing allows.",
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
                    name: "lp_vact_en",
                    description: Some(
                        "enable the return to low-power inside the VACT period when timing allows.",
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
                    name: "lp_hbp_en",
                    description: Some(
                        "enable the return to low-power inside the HBP period when timing allows.",
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
                    name: "lp_hfp_en",
                    description: Some(
                        "enable the return to low-power inside the HFP period when timing allows.",
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
                    name: "frame_bta_ack_en",
                    description: Some(
                        "enable the request for an acknowledge response at the end of a frame.",
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
                    name: "lp_cmd_en",
                    description: Some(
                        "enable command transmission only in low-power mode.",
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
                    name: "vpg_en",
                    description: Some(
                        "enable video mode pattern generator.",
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
                    name: "vpg_mode",
                    description: Some(
                        "0x0: colorbar 0x1: berpattern, vertical only.",
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
                    name: "vpg_orientation",
                    description: Some(
                        "indicates the color bar orientation : 0x0: vertical mode 0x1: horizontal mode.",
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
            name: "VidModeCfgAct",
            extends: None,
            description: Some(
                "holds value that controller is using for vid_mode_cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_mode_type",
                    description: Some(
                        "specifies the video mode transmission type.",
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
                    name: "lp_vsa_en",
                    description: Some(
                        "enable the returne to low-power inside the VSA period when timing allows.",
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
                    name: "lp_vbp_en",
                    description: Some(
                        "enable the returne to low-power inside the VBP period when timing allows.",
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
                    name: "lp_vfp_en",
                    description: Some(
                        "enable the returne to low-power inside the VFP period when timing allows.",
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
                    name: "lp_vact_en",
                    description: Some(
                        "enable the returne to low-power inside the VACT period when timing allows.",
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
                    name: "lp_hbp_en",
                    description: Some(
                        "enable the returne to low-power inside the HBP period when timing allows.",
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
                    name: "lp_hfp_en",
                    description: Some(
                        "enable the returne to low-power inside the HFP period when timing allows.",
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
                    name: "frame_bta_ack_en",
                    description: Some(
                        "enable the request for an acknowledge response at the end of a frame.",
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
                    name: "lp_cmd_en",
                    description: Some(
                        "enable the command transmission only in low-power mode.",
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
            name: "VidNullSize",
            extends: None,
            description: Some(
                "configures the size of null packets.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_null_size",
                    description: Some(
                        "configures the number of bytes inside a null packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidNullSizeAct",
            extends: None,
            description: Some(
                "holds the value that controller is using for vid_null_size.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_null_size",
                    description: Some(
                        "the number of bytes in side a null packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidNumChunks",
            extends: None,
            description: Some(
                "configures the number of chunks to use.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_num_chunks",
                    description: Some(
                        "configures the number of chunks to be transmitted a line period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidNumChunksAct",
            extends: None,
            description: Some(
                "holds value that controller is using for vid_num_chunks.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_num_chunks",
                    description: Some(
                        "the number of chunks to be transmitted during a line period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidPktSize",
            extends: None,
            description: Some(
                "configures the video packet size.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_pkt_size",
                    description: Some(
                        "configures the number of pixels in a single video packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidPktSizeAct",
            extends: None,
            description: Some(
                "holds value that controller is using for vid_pkt_size.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_pkt_size",
                    description: Some(
                        "the number of pixels in a single video packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidPktStatus",
            extends: None,
            description: Some(
                "status of fifo related to dpi.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpi_cmd_w_empty",
                    description: Some(
                        "This bit indicates the empty status of write command FIFO for video Mode. This bit is set to 0 for command Mode.",
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
                    name: "dpi_cmd_w_full",
                    description: Some(
                        "This bit indicates the full status of write command FIFO for video Mode. This bit is set to 0 for command Mode.",
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
                    name: "dpi_pld_w_empty",
                    description: Some(
                        "This bit indicates the empty status of write payload FIFO for video Mode. This bit is set to 0 for command Mode.",
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
                    name: "dpi_pld_w_full",
                    description: Some(
                        "This bit indicates the full status of write payload FIFO for video Mode. This bit is set to 0 for command Mode.",
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
                    name: "dpi_buff_pld_empty",
                    description: Some(
                        "This bit indicates the empty status of the payload internal buffer for video Mode. This bit is set to 0 for command Mod.",
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
                    name: "dpi_buff_pld_full",
                    description: Some(
                        "This bit indicates the full status of the payload internal buffer for video Mode. This bit is set to 0 for command Mode.",
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
            name: "VidShadowCtrl",
            extends: None,
            description: Some(
                "controls dpi shadow feature.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vid_shadow_en",
                    description: Some(
                        "when set to 1, DPI receives the active configuration from the auxiliary register.",
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
                    name: "vid_shadow_req",
                    description: Some(
                        "when set to 1, request that the dpi register from regbank are copied to the auxiliary registers.",
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
                    name: "vid_shadow_pin_req",
                    description: Some(
                        "when set to 1, the video request is done by external pin.",
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
            ],
        },
        FieldSet {
            name: "VidVactiveLines",
            extends: None,
            description: Some(
                "configures the vertical resolution of video.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "v_active_lines",
                    description: Some(
                        "configures the vertical active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVactiveLinesAct",
            extends: None,
            description: Some(
                "value for vid_vactive_lines.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "v_active_lines",
                    description: Some(
                        "vertical active period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVbpLines",
            extends: None,
            description: Some(
                "configures the vbp period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp_lines",
                    description: Some(
                        "configures the vertical back porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVbpLinesAct",
            extends: None,
            description: Some(
                "value for vid_vbp_lines.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp_lines",
                    description: Some(
                        "vertical back porch period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVfpLines",
            extends: None,
            description: Some(
                "configures the vfp period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp_linies",
                    description: Some(
                        "configures the vertical front porch period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVfpLinesAct",
            extends: None,
            description: Some(
                "value for vid_vfp_lines.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp_lines",
                    description: Some(
                        "vertical porch period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVsaLines",
            extends: None,
            description: Some(
                "configures the vsa period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa_lines",
                    description: Some(
                        "configures the verical synchronism active period measured in number of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "VidVsaLinesAct",
            extends: None,
            description: Some(
                "value for vid_vsa_lines.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa_lines",
                    description: Some(
                        "vertical synchronism active period.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
