use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Esc",
            extends: None,
            description: Some(
                "ESC.",
            ),
            items: &[
                BlockItem {
                    name: "type_",
                    description: Some(
                        "Type of EtherCAT controller.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "revision",
                    description: Some(
                        "Revision of EtherCAT controller.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Revision",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "build",
                    description: Some(
                        "Build of EtherCAT controller.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Build",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fmmu_num",
                    description: Some(
                        "FMMU supported.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FmmuNum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "syncm_num",
                    description: Some(
                        "SyncManagers supported.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SyncmNum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ram_size",
                    description: Some(
                        "RAM Size.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RamSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_desc",
                    description: Some(
                        "Port Descriptor.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PortDesc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "feature",
                    description: Some(
                        "ESC Feature supported.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Feature",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "station_addr",
                    description: Some(
                        "Configured Station Address.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "StationAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "station_als",
                    description: Some(
                        "Configured Station Alias.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "StationAls",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_wen",
                    description: Some(
                        "Register Write Enable.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RegWen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_wp",
                    description: Some(
                        "Register Write Protection.",
                    ),
                    array: None,
                    byte_offset: 0x21,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RegWp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_wen",
                    description: Some(
                        "ESC Write Enable.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscWen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_wp",
                    description: Some(
                        "ESC Write Protection.",
                    ),
                    array: None,
                    byte_offset: 0x31,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscWp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_rst_ecat",
                    description: Some(
                        "ESC Reset ECAT.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscRstEcat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_rst_pdi",
                    description: Some(
                        "ESC Reset PDI.",
                    ),
                    array: None,
                    byte_offset: 0x41,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscRstPdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_dl_ctrl",
                    description: Some(
                        "ESC DL Control.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EscDlCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "physical_rw_offset",
                    description: Some(
                        "Physical Read/Write Offset.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PhysicalRwOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_dl_stat",
                    description: Some(
                        "ESC DL Status.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EscDlStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "al_ctrl",
                    description: Some(
                        "AL Control.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "AlCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "al_stat",
                    description: Some(
                        "AL Status.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "AlStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "al_stat_code",
                    description: Some(
                        "AL Status Code.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "AlStatCode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "run_led_ovrd",
                    description: Some(
                        "RUN LED Override.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RunLedOvrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "err_led_ovrd",
                    description: Some(
                        "ERR LED Override.",
                    ),
                    array: None,
                    byte_offset: 0x139,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ErrLedOvrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_ctrl",
                    description: Some(
                        "PDI Control.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscPdiCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "esc_cfg",
                    description: Some(
                        "ESC Configuration.",
                    ),
                    array: None,
                    byte_offset: 0x141,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EscCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_info",
                    description: Some(
                        "PDI Information.",
                    ),
                    array: None,
                    byte_offset: 0x14e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PdiInfo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_cfg",
                    description: Some(
                        "PDI Configuration.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PdiCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_sl_cfg",
                    description: Some(
                        "PDI Sync/Latch[1:0] Configuration.",
                    ),
                    array: None,
                    byte_offset: 0x151,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PdiSlCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_ext_cfg",
                    description: Some(
                        "PDI Extended Configuration.",
                    ),
                    array: None,
                    byte_offset: 0x152,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PdiExtCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecat_evt_msk",
                    description: Some(
                        "ECAT Event Mask.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EcatEvtMsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_al_evt_msk",
                    description: Some(
                        "PDI AL Event Mask.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdiAlEvtMsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecat_evt_req",
                    description: Some(
                        "ECAT Event Request.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EcatEvtReq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "al_evt_req",
                    description: Some(
                        "AL Event Request.",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlEvtReq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_err_cnt",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 2,
                            },
                        ),
                    ),
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "RxErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fwd_rx_err_cnt",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FwdRxErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecat_pu_err_cnt",
                    description: Some(
                        "ECAT Processing Unit Error Counter.",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EcatPuErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_err_cnt",
                    description: Some(
                        "PDI Error Counter.",
                    ),
                    array: None,
                    byte_offset: 0x30d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PdiErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lost_link_cnt",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LostLinkCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_div",
                    description: Some(
                        "Watchdog Divider.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "WdgDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_time_pdi",
                    description: Some(
                        "Watchdog Time PDI.",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "WdgTimePdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_time_pdat",
                    description: Some(
                        "Watchdog Time Process Data.",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "WdgTimePdat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_stat_pdat",
                    description: Some(
                        "Watchdog Status Process Data.",
                    ),
                    array: None,
                    byte_offset: 0x440,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "WdgStatPdat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_cnt_pdat",
                    description: Some(
                        "Watchdog Counter Process Data.",
                    ),
                    array: None,
                    byte_offset: 0x442,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "WdgCntPdat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdg_cnt_pdi",
                    description: Some(
                        "Watchdog Counter PDI.",
                    ),
                    array: None,
                    byte_offset: 0x443,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "WdgCntPdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eeprom_cfg",
                    description: Some(
                        "EEPROM Configuration.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EepromCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eeprom_pdi_acc_stat",
                    description: Some(
                        "EEPROM PDI Access State.",
                    ),
                    array: None,
                    byte_offset: 0x501,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EepromPdiAccStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eeprom_ctrl_stat",
                    description: Some(
                        "EEPROM Control/Status.",
                    ),
                    array: None,
                    byte_offset: 0x502,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "EepromCtrlStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eeprom_addr",
                    description: Some(
                        "EEPROM Address.",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EepromAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eeprom_data",
                    description: Some(
                        "EEPROM Data.",
                    ),
                    array: None,
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "EepromData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mii_mng_cs",
                    description: Some(
                        "MII Management Control/Status.",
                    ),
                    array: None,
                    byte_offset: 0x510,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "MiiMngCs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_addr",
                    description: Some(
                        "PHY Address.",
                    ),
                    array: None,
                    byte_offset: 0x512,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PhyAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_reg_addr",
                    description: Some(
                        "PHY Register Address.",
                    ),
                    array: None,
                    byte_offset: 0x513,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PhyRegAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_data",
                    description: Some(
                        "PHY Data.",
                    ),
                    array: None,
                    byte_offset: 0x514,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PhyData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "miim_ecat_acc_stat",
                    description: Some(
                        "MII Management ECAT Access State.",
                    ),
                    array: None,
                    byte_offset: 0x516,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "MiimEcatAccStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "miim_pdi_acc_stat",
                    description: Some(
                        "MII Management PDI Access State.",
                    ),
                    array: None,
                    byte_offset: 0x517,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "MiimPdiAccStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_stat",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 1,
                            },
                        ),
                    ),
                    byte_offset: 0x518,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PhyStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fmmu",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x600,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Fmmu",
                        },
                    ),
                },
                BlockItem {
                    name: "syncm",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Syncm",
                        },
                    ),
                },
                BlockItem {
                    name: "rcv_time",
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
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RcvTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sys_time",
                    description: Some(
                        "System Time.",
                    ),
                    array: None,
                    byte_offset: 0x910,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "SysTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcvt_ecat_pu",
                    description: Some(
                        "Receive Time ECAT Processing Unit.",
                    ),
                    array: None,
                    byte_offset: 0x918,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "RcvtEcatPu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sys_time_offset",
                    description: Some(
                        "System Time Offset.",
                    ),
                    array: None,
                    byte_offset: 0x920,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "SysTimeOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sys_time_delay",
                    description: Some(
                        "System Time Delay.",
                    ),
                    array: None,
                    byte_offset: 0x928,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SysTimeDelay",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sys_time_diff",
                    description: Some(
                        "System Time Difference.",
                    ),
                    array: None,
                    byte_offset: 0x92c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SysTimeDiff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd_cnt_start",
                    description: Some(
                        "Speed Counter Start.",
                    ),
                    array: None,
                    byte_offset: 0x930,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SpdCntStart",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd_cnt_diff",
                    description: Some(
                        "Speed Counter Diff.",
                    ),
                    array: None,
                    byte_offset: 0x932,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SpdCntDiff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sys_time_diff_fd",
                    description: Some(
                        "System Time Difference Filter Depth.",
                    ),
                    array: None,
                    byte_offset: 0x934,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SysTimeDiffFd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd_cnt_fd",
                    description: Some(
                        "Speed Counter Filter Depth.",
                    ),
                    array: None,
                    byte_offset: 0x935,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SpdCntFd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcv_time_lm",
                    description: Some(
                        "Receive Time Latch Mode.",
                    ),
                    array: None,
                    byte_offset: 0x936,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RcvTimeLm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cyc_unit_ctrl",
                    description: Some(
                        "Cyclic Unit Control.",
                    ),
                    array: None,
                    byte_offset: 0x980,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CycUnitCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "synco_act",
                    description: Some(
                        "SYNC Out Unit Activation.",
                    ),
                    array: None,
                    byte_offset: 0x981,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SyncoAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pulse_len",
                    description: Some(
                        "Pulse Length of SyncSignals.",
                    ),
                    array: None,
                    byte_offset: 0x982,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PulseLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "act_stat",
                    description: Some(
                        "Activation Status.",
                    ),
                    array: None,
                    byte_offset: 0x984,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ActStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync0_stat",
                    description: Some(
                        "SYNC0 Status.",
                    ),
                    array: None,
                    byte_offset: 0x98e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Sync0Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync1_stat",
                    description: Some(
                        "SYNC1 Status.",
                    ),
                    array: None,
                    byte_offset: 0x98f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Sync1Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "start_time_co",
                    description: Some(
                        "Start Time Cyclic Operation.",
                    ),
                    array: None,
                    byte_offset: 0x990,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "StartTimeCo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nxt_sync1_pulse",
                    description: Some(
                        "Next SYNC1 Pulse.",
                    ),
                    array: None,
                    byte_offset: 0x998,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "NxtSync1Pulse",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync0_cyc_time",
                    description: Some(
                        "SYNC0 Cycle Time.",
                    ),
                    array: None,
                    byte_offset: 0x9a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sync0CycTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sync1_cyc_time",
                    description: Some(
                        "SYNC1 Cycle Time.",
                    ),
                    array: None,
                    byte_offset: 0x9a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sync1CycTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch0_ctrl",
                    description: Some(
                        "Latch0 Control.",
                    ),
                    array: None,
                    byte_offset: 0x9a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Latch0Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch1_ctrl",
                    description: Some(
                        "Latch1 Control.",
                    ),
                    array: None,
                    byte_offset: 0x9a9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Latch1Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch0_stat",
                    description: Some(
                        "Latch0 Status.",
                    ),
                    array: None,
                    byte_offset: 0x9ae,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Latch0Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch1_stat",
                    description: Some(
                        "Latch1 Status.",
                    ),
                    array: None,
                    byte_offset: 0x9af,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Latch1Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch0_time_pe",
                    description: Some(
                        "Latch0 Time Positive Edge.",
                    ),
                    array: None,
                    byte_offset: 0x9b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Latch0TimePe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch0_time_ne",
                    description: Some(
                        "Latch0 Time Negative Edge.",
                    ),
                    array: None,
                    byte_offset: 0x9b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Latch0TimeNe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch1_time_pe",
                    description: Some(
                        "Latch1 Time Positive Edge.",
                    ),
                    array: None,
                    byte_offset: 0x9c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Latch1TimePe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "latch1_time_ne",
                    description: Some(
                        "Latch1 Time Negative Edge.",
                    ),
                    array: None,
                    byte_offset: 0x9c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Latch1TimeNe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecat_buf_cet",
                    description: Some(
                        "EtherCAT Buffer Change Event Time.",
                    ),
                    array: None,
                    byte_offset: 0x9f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EcatBufCet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_buf_set",
                    description: Some(
                        "PDI Buffer Start Event Time.",
                    ),
                    array: None,
                    byte_offset: 0x9f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdiBufSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_buf_cet",
                    description: Some(
                        "PDI Buffer Change Event Time.",
                    ),
                    array: None,
                    byte_offset: 0x9fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdiBufCet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pid",
                    description: Some(
                        "Product ID.",
                    ),
                    array: None,
                    byte_offset: 0xe00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Pid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vid",
                    description: Some(
                        "Vendor ID.",
                    ),
                    array: None,
                    byte_offset: 0xe08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Vid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dio_out_data",
                    description: Some(
                        "Digital I/O Output Data.",
                    ),
                    array: None,
                    byte_offset: 0xf00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DioOutData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpo",
                    description: Some(
                        "General Purpose Outputs.",
                    ),
                    array: None,
                    byte_offset: 0xf10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Gpo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpi",
                    description: Some(
                        "General Purpose Inputs.",
                    ),
                    array: None,
                    byte_offset: 0xf18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 64,
                            fieldset: Some(
                                "Gpi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte0",
                    description: Some(
                        "User Ram Byte 0.",
                    ),
                    array: None,
                    byte_offset: 0xf80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte1",
                    description: Some(
                        "User Ram Byte 1.",
                    ),
                    array: None,
                    byte_offset: 0xf81,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte2",
                    description: Some(
                        "User Ram Byte 2.",
                    ),
                    array: None,
                    byte_offset: 0xf82,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte3",
                    description: Some(
                        "User Ram Byte 3.",
                    ),
                    array: None,
                    byte_offset: 0xf83,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte4",
                    description: Some(
                        "User Ram Byte 4.",
                    ),
                    array: None,
                    byte_offset: 0xf84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte5",
                    description: Some(
                        "User Ram Byte 5.",
                    ),
                    array: None,
                    byte_offset: 0xf85,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte6",
                    description: Some(
                        "User Ram Byte 6.",
                    ),
                    array: None,
                    byte_offset: 0xf86,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte7",
                    description: Some(
                        "User Ram Byte 7.",
                    ),
                    array: None,
                    byte_offset: 0xf87,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte8",
                    description: Some(
                        "User Ram Byte 8.",
                    ),
                    array: None,
                    byte_offset: 0xf88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte9",
                    description: Some(
                        "User Ram Byte 9.",
                    ),
                    array: None,
                    byte_offset: 0xf89,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte10",
                    description: Some(
                        "User Ram Byte 10.",
                    ),
                    array: None,
                    byte_offset: 0xf8a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte11",
                    description: Some(
                        "User Ram Byte 11.",
                    ),
                    array: None,
                    byte_offset: 0xf8b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte14",
                    description: Some(
                        "User Ram Byte 14.",
                    ),
                    array: None,
                    byte_offset: 0xf8e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte15",
                    description: Some(
                        "User Ram Byte 15.",
                    ),
                    array: None,
                    byte_offset: 0xf8f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ram_byte19",
                    description: Some(
                        "User Ram Byte 19.",
                    ),
                    array: None,
                    byte_offset: 0xf93,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UserRamByte19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdram",
                    description: Some(
                        "Process Data Ram.",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pdram",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdram_als",
                    description: Some(
                        "Process Data Ram Alias.",
                    ),
                    array: None,
                    byte_offset: 0x10000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdramAls",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_cfg0",
                    description: Some(
                        "General Purpose Configure 0.",
                    ),
                    array: None,
                    byte_offset: 0x1f000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_cfg1",
                    description: Some(
                        "General Purpose Configure 1.",
                    ),
                    array: None,
                    byte_offset: 0x1f004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_cfg2",
                    description: Some(
                        "General Purpose Configure 2.",
                    ),
                    array: None,
                    byte_offset: 0x1f008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprCfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg0",
                    description: Some(
                        "PHY Configure 0.",
                    ),
                    array: None,
                    byte_offset: 0x1f010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cfg1",
                    description: Some(
                        "PHY Configure 1.",
                    ),
                    array: None,
                    byte_offset: 0x1f014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpio_ctrl",
                    description: Some(
                        "GPIO Output Enable.",
                    ),
                    array: None,
                    byte_offset: 0x1f020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpioCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpi_override0",
                    description: Some(
                        "GPI low word Override value.",
                    ),
                    array: None,
                    byte_offset: 0x1f030,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpiOverride0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpi_override1",
                    description: Some(
                        "GPI high word Override value.",
                    ),
                    array: None,
                    byte_offset: 0x1f034,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpiOverride1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpo_reg0",
                    description: Some(
                        "GPO low word read value.",
                    ),
                    array: None,
                    byte_offset: 0x1f038,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpoReg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpo_reg1",
                    description: Some(
                        "GPO high word read value.",
                    ),
                    array: None,
                    byte_offset: 0x1f03c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpoReg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpi_reg0",
                    description: Some(
                        "GPI low word read value.",
                    ),
                    array: None,
                    byte_offset: 0x1f040,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpiReg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpi_reg1",
                    description: Some(
                        "GPI high word read value.",
                    ),
                    array: None,
                    byte_offset: 0x1f044,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GpiReg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_status",
                    description: Some(
                        "global status register.",
                    ),
                    array: None,
                    byte_offset: 0x1f060,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "io_cfg",
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
                    byte_offset: 0x1f080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IoCfg",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Fmmu",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "logic_start_addr",
                    description: Some(
                        "Logical Start Address.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LogicStartAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "length",
                    description: Some(
                        "Length.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "FmmuLength",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "logic_start_bit",
                    description: Some(
                        "Logical Start Bit.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LogicStartBit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "logic_stop_bit",
                    description: Some(
                        "Logical Stop Bit.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LogicStopBit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "physical_start_addr",
                    description: Some(
                        "Physical Start Address.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "FmmuPhysicalStartAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "physical_start_bit",
                    description: Some(
                        "Physical Start Bit.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PhysicalStartBit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "type_",
                    description: Some(
                        "Type.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FmmuType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "activate",
                    description: Some(
                        "Activate.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FmmuActivate",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Syncm",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "physical_start_addr",
                    description: Some(
                        "Physical Start Address.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SyncmPhysicalStartAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "length",
                    description: Some(
                        "Length.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SyncmLength",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "control",
                    description: Some(
                        "Control.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Control",
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
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "activate",
                    description: Some(
                        "Activate.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SyncmActivate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdi_ctrl",
                    description: Some(
                        "PDI Control.",
                    ),
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SyncmPdiCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ActStat",
            extends: None,
            description: Some(
                "Activation Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sync0",
                    description: Some(
                        "SYNC0 activation state: 0:First SYNC0 pulse is not pending 1:First SYNC0 pulse is pending.",
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
                    name: "sync1",
                    description: Some(
                        "SYNC1 activation state: 0:First SYNC1 pulse is not pending 1:First SYNC1 pulse is pending.",
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
                    name: "chk_rslt",
                    description: Some(
                        "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated: 0:Start Time was within near future 1:Start Time was out of near future (0x0981[6]).",
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
            ],
        },
        FieldSet {
            name: "AlCtrl",
            extends: None,
            description: Some(
                "AL Control.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ist",
                    description: Some(
                        "Initiate State Transition of the Device State Machine: 1:Request Init State 3:Request Bootstrap State 2:Request Pre-Operational State 4:Request Safe-Operational State 8:Request Operational State.",
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
                    name: "eia",
                    description: Some(
                        "Error Ind Ack: 0:No Ack of Error Ind in AL status register 1:Ack of Error Ind in AL status register.",
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
                    name: "di",
                    description: Some(
                        "Device Identification: 0:No request 1:Device Identification request.",
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
            name: "AlEvtReq",
            extends: None,
            description: Some(
                "AL Event Request.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alc_evt",
                    description: Some(
                        "AL Control event: 0:No AL Control Register change 1:AL Control Register has been written3 (Bit is cleared by reading AL Control register 0x0120:0x0121 from PDI).",
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
                    name: "dcl_evt",
                    description: Some(
                        "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from PDI, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event. Available if Latch Unit is PDI-controlled).",
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
                    name: "st_dc_sync0",
                    description: Some(
                        "State of DC SYNC0 (if register 0x0151[3]=1): (Bit is cleared by reading SYNC0 status 0x098E from PDI, use only in Acknowledge mode).",
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
                    name: "st_dc_sync1",
                    description: Some(
                        "State of DC SYNC1 (if register 0x0151[7]=1): (Bit is cleared by reading of SYNC1 status 0x098F from PDI, use only in Acknowledge mode).",
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
                    name: "sm_act",
                    description: Some(
                        "SyncManager activation register (SyncManager register offset 0x6) changed: 0:No change in any SyncManager 1:At least one SyncManager changed (Bit is cleared by reading SyncManager Activation registers 0x0806 etc. from PDI).",
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
                    name: "ee_emu",
                    description: Some(
                        "EEPROM Emulation: 0:No command pending 1:EEPROM command pending (Bit is cleared by acknowledging the command in EEPROM Control/Status register 0x0502:0x0503[10:8] from PDI).",
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
                    name: "wdg_pd",
                    description: Some(
                        "Watchdog Process Data: 0:Has not expired 1:Has expired (Bit is cleared by reading Watchdog Status Process Data 0x0440 from PDI).",
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
                    name: "sm_int",
                    description: Some(
                        "SyncManager interrupts (SyncManager register offset 0x5, bit [0] or [1]): 0:No SyncManager 0 interrupt 1:SyncManager 0 interrupt pending 0:No SyncManager 1 interrupt 1:SyncManager 1 interrupt pending … 0:No SyncManager 15 interrupt 1:SyncManager 15 interrupt pending.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AlStat",
            extends: None,
            description: Some(
                "AL Status.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "as_",
                    description: Some(
                        "Actual State of the Device State Machine: 1:Init State 3:Bootstrap State 2:Pre-Operational State 4:Safe-Operational State 8:Operational State.",
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
                    name: "ei",
                    description: Some(
                        "Error Ind: 0:Device is in State as requested or Flag cleared by command 1:Device has not entered requested State or changed State as result of a local action.",
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
                    name: "di",
                    description: Some(
                        "Device Identification: 0:Device Identification not valid 1:Device Identification loaded.",
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
            name: "AlStatCode",
            extends: None,
            description: Some(
                "AL Status Code.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "code",
                    description: Some(
                        "AL Status Code.",
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
            name: "Build",
            extends: None,
            description: Some(
                "Build of EtherCAT controller.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "z",
                    description: Some(
                        "maintenance version Z.",
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
                    name: "y",
                    description: Some(
                        "minor version Y.",
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
                    name: "build",
                    description: Some(
                        "No description available.",
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
            name: "Control",
            extends: None,
            description: Some(
                "Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "op_mode",
                    description: Some(
                        "Operation Mode: 00:Buffered (3 buffer mode) 01:Reserved 10:Mailbox (Single buffer mode) 11:Reserved.",
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
                    name: "dir",
                    description: Some(
                        "Direction: 00:Read:ECAT read access, PDI write access. 01:Write:ECAT write access, PDI read access. 10:Reserved 11:Reserved.",
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
                    name: "int_ecat",
                    description: Some(
                        "Interrupt in ECAT Event Request Register: 0:Disabled 1:Enabled.",
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
                    name: "int_al",
                    description: Some(
                        "Interrupt in AL Event Request Register: 0:Disabled 1:Enabled.",
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
                    name: "wdg_trg_en",
                    description: Some(
                        "Watchdog Trigger Enable: 0:Disabled 1:Enabled.",
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
            name: "CycUnitCtrl",
            extends: None,
            description: Some(
                "Cyclic Unit Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "synco",
                    description: Some(
                        "Cyclic Unit and SYNC0 out unit control: 0:ECAT-controlled 1:PDI-controlled.",
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
                    name: "latchi0",
                    description: Some(
                        "Latch In unit 0: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting. Always 1 (PDI-controlled) if System Time is PDI controlled.",
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
                    name: "latchi1",
                    description: Some(
                        "Latch In unit 1: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting.",
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
            name: "DioOutData",
            extends: None,
            description: Some(
                "Digital I/O Output Data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "od",
                    description: Some(
                        "Output Data.",
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
            name: "EcatBufCet",
            extends: None,
            description: Some(
                "EtherCAT Buffer Change Event Time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Local time at the beginning of the frame which causes at least one SyncManager to assert an ECAT event.",
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
            name: "EcatEvtMsk",
            extends: None,
            description: Some(
                "ECAT Event Mask.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "mask",
                    description: Some(
                        "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames: 0:Corresponding ECAT Event Request register bit is not mapped 1:Corresponding ECAT Event Request register bit is mapped.",
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
            name: "EcatEvtReq",
            extends: None,
            description: Some(
                "ECAT Event Request.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dcl_evt",
                    description: Some(
                        "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from ECAT for ECAT-controlled Latch Units, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event).",
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
                    name: "dls_evt",
                    description: Some(
                        "DL Status event: 0:No change in DL Status 1:DL Status change (Bit is cleared by reading out DL Status 0x0110:0x0111 from ECAT).",
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
                    name: "als_evt",
                    description: Some(
                        "AL Status event: 0:No change in AL Status 1:AL Status change (Bit is cleared by reading out AL Status 0x0130:0x0131 from ECAT).",
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
                    name: "mv",
                    description: Some(
                        "Mirrors values of each SyncManager Status: 0:No Sync Channel 0 event 1:Sync Channel 0 event pending 0:No Sync Channel 1 event 1:Sync Channel 1 event pending … 0:No Sync Channel 7 event 1:Sync Channel 7 event pending.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EcatPuErrCnt",
            extends: None,
            description: Some(
                "ECAT Processing Unit Error Counter.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "ECAT Processing Unit error counter (counting is stopped when 0xFF is reached). Counts errors of frames passing the Processing Unit.",
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
            name: "EepromAddr",
            extends: None,
            description: Some(
                "EEPROM Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "EEPROM Address 0:First word (= 16 bit) 1:Second word … Actually used EEPROM Address bits: &[9-0] : EEPROM size up to 16 Kbit [17-0] : EEPROM size 32 Kbit – 4 Mbit [31-0] : EEPROM Emulation.",
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
            name: "EepromCfg",
            extends: None,
            description: Some(
                "EEPROM Configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pdi",
                    description: Some(
                        "EEPROM control is offered to PDI: 0:no 1:yes (PDI has EEPROM control).",
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
                    name: "force_ecat",
                    description: Some(
                        "Force ECAT access: 0:Do not change Bit 0x0501[0] 1:Reset Bit 0x0501[0] to 0.",
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
            name: "EepromCtrlStat",
            extends: None,
            description: Some(
                "EEPROM Control/Status.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ecat_wen",
                    description: Some(
                        "ECAT write enable*2 : 0:Write requests are disabled 1:Write requests are enabled This bit is always 1 if PDI has EEPROM control.",
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
                    name: "ee_emu",
                    description: Some(
                        "EPROM emulation: 0:Normal operation (I²C interface used) 1:PDI emulates EEPROM (I²C not used).",
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
                    name: "num_rd_byte",
                    description: Some(
                        "Supported number of EEPROM read bytes: 0:4 Bytes 1:8 Bytes.",
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
                    name: "ee_algm",
                    description: Some(
                        "Selected EEPROM Algorithm: 0:1 address byte (1Kbit – 16Kbit EEPROMs) 1:2 address bytes (32Kbit – 4 Mbit EEPROMs).",
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
                    name: "cmd",
                    description: Some(
                        "Command register*2: Write:Initiate command. Read:Currently executed command Commands: 000:No command/EEPROM idle (clear error bits) 001:Read 010:Write 100:Reload Others:Reserved/invalid commands (do not issue) EEPROM emulation only:after execution, PDI writes command value to indicate operation is ready.",
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
                    name: "cksm_err",
                    description: Some(
                        "Checksum Error in ESC Configuration Area: 0:Checksum ok 1:Checksum error EEPROM emulation for IP Core only:PDI writes 1 if a CRC failure has occurred for a reload command.",
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
                    name: "ee_lds",
                    description: Some(
                        "EEPROM loading status: 0:EEPROM loaded, device information ok 1:EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure).",
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
                    name: "err_ack_cmd",
                    description: Some(
                        "Error Acknowledge/Command*3 : 0:No error 1:Missing EEPROM acknowledge or invalid command EEPROM emulation only:PDI writes 1 if a temporary failure has occurred.",
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
                    name: "err_wen",
                    description: Some(
                        "Error Write Enable*3 : 0:No error 1:Write Command without Write enable.",
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
                    name: "busy",
                    description: Some(
                        "Busy: 0:EEPROM Interface is idle 1:EEPROM Interface is busy.",
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
            ],
        },
        FieldSet {
            name: "EepromData",
            extends: None,
            description: Some(
                "EEPROM Data.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "lo",
                    description: Some(
                        "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM, lower bytes).",
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
                    name: "hi",
                    description: Some(
                        "EEPROM Read data (data read from EEPROM, higher bytes).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 48,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EepromPdiAccStat",
            extends: None,
            description: Some(
                "EEPROM PDI Access State.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "access",
                    description: Some(
                        "Access to EEPROM: 0:PDI releases EEPROM access 1:PDI takes EEPROM access (PDI has EEPROM control).",
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
            name: "ErrLedOvrd",
            extends: None,
            description: Some(
                "ERR LED Override.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "led_code",
                    description: Some(
                        "LED code: 0x0:Off 0x1-0xC:Flash 1x – 12x 0xD:Blinking 0xE:Flickering 0xF:On.",
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
                    name: "en_ovrd",
                    description: Some(
                        "Enable Override: 0:Override disabled 1:Override enabled.",
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
            name: "EscCfg",
            extends: None,
            description: Some(
                "ESC Configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dev_emu",
                    description: Some(
                        "Device emulation (control of AL status): 0:AL status register has to be set by PDI 1:AL status register will be set to value written to AL control register.",
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
                    name: "eldap",
                    description: Some(
                        "Enhanced Link detection all ports: 0:disabled (if bits [7:4]=0) 1:enabled at all ports (overrides bits [7:4]).",
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
                    name: "dcsou",
                    description: Some(
                        "Distributed Clocks SYNC Out Unit: 0:disabled (power saving) 1:enabled.",
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
                    name: "cdliu",
                    description: Some(
                        "Distributed Clocks Latch In Unit: 0:disabled (power saving) 1:enabled.",
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
                    name: "elp0",
                    description: Some(
                        "Enhanced Link port 0: 0:disabled (if bit 1=0) 1:enabled.",
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
                    name: "elp1",
                    description: Some(
                        "Enhanced Link port 1: 0:disabled (if bit 1=0) 1:enabled.",
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
                    name: "elp2",
                    description: Some(
                        "Enhanced Link port 2: 0:disabled (if bit 1=0) 1:enabled.",
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
                    name: "elp3",
                    description: Some(
                        "Enhanced Link port 3: 0:disabled (if bit 1=0) 1:enabled.",
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
            name: "EscDlCtrl",
            extends: None,
            description: Some(
                "ESC DL Control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fr",
                    description: Some(
                        "Forwarding rule: 0:Forward non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification. The source MAC address is not changed for any frame. 1:Destroy non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are destroyed. The source MAC address is changed by the Processing Unit for every frame (SOURCE_MAC[1] is set.",
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
                    name: "tu",
                    description: Some(
                        "Temporary use of settings in 0x0100:0x0103[8:15]: 0:permanent use 1:use for about 1 second, then revert to previous settings.",
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
                    name: "lp0",
                    description: Some(
                        "Loop Port 0: 00:Auto 01:Auto Close 10:Open 11:Closed NOTE: Loop open means sending/receiving over this port is enabled, loop closed means sending/receiving is disabled and frames are forwarded to the next open port internally. Auto:loop closed at link down, opened at link up Auto Close:loop closed at link down, opened with writing 01 again after link up (or receiving a valid Ethernet frame at the closed port) Open:loop open regardless of link state Closed:loop closed regardless of link state.",
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
                    name: "lp1",
                    description: Some(
                        "Loop Port 1: 00:Auto 01:Auto Close 10:Open 11:Closed.",
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
                    name: "lp2",
                    description: Some(
                        "Loop Port 2: 00:Auto 01:Auto Close 10:Open 11:Closed.",
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
                    name: "lp3",
                    description: Some(
                        "Loop Port 3: 00:Auto 01:Auto Close 10:Open 11:Closed.",
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
                    name: "rfs",
                    description: Some(
                        "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full). RX FIFO Size/RX delay reduction** : Value:EBUS:MII: 0:-50 ns -40 ns (-80 ns***) 1:-40 ns -40 ns (-80 ns***) 2:-30 ns -40 ns 3:-20 ns -40 ns 4:-10 ns no change 5:no change no change 6:no change no change 7:default default NOTE:EEPROM value is only taken over at first EEPROM load after power-on or reset.",
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
                    name: "sa",
                    description: Some(
                        "Station alias: 0:Ignore Station Alias 1:Alias can be used for all configured address comm.",
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
            name: "EscDlStat",
            extends: None,
            description: Some(
                "ESC DL Status.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "eplc",
                    description: Some(
                        "PDI operational/EEPROM loaded correctly: 0:EEPROM not loaded, PDI not operational (no access to Process Data RAM) 1:EEPROM loaded correctly, PDI operational (access to Process Data RAM).",
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
                    name: "wds",
                    description: Some(
                        "PDI Watchdog Status: 0:Watchdog expired 1:Watchdog reloaded.",
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
                    name: "eld",
                    description: Some(
                        "Enhanced Link detection: 0:Deactivated for all ports 1:Activated for at least one port NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset.",
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
                    name: "plp0",
                    description: Some(
                        "Physical link on Port 0: 0:No link 1:Link detected.",
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
                    name: "plp1",
                    description: Some(
                        "Physical link on Port 1: 0:No link 1:Link detected.",
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
                    name: "plp2",
                    description: Some(
                        "Physical link on Port 2: 0:No link 1:Link detected.",
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
                    name: "plp3",
                    description: Some(
                        "Physical link on Port 3: 0:No link 1:Link detected.",
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
                    name: "lp0",
                    description: Some(
                        "Loop Port 0: 0:Open 1:Closed.",
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
                    name: "cp0",
                    description: Some(
                        "Communication on Port 0: 0:No stable communication 1:Communication established.",
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
                    name: "lp1",
                    description: Some(
                        "Loop Port 1: 0:Open 1:Closed.",
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
                    name: "cp1",
                    description: Some(
                        "Communication on Port 1: 0:No stable communication 1:Communication established.",
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
                    name: "lp2",
                    description: Some(
                        "Loop Port 2: 0:Open 1:Closed.",
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
                    name: "cp2",
                    description: Some(
                        "Communication on Port 2: 0:No stable communication 1:Communication established.",
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
                    name: "lp3",
                    description: Some(
                        "Loop Port 3: 0:Open 1:Closed.",
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
                    name: "cp3",
                    description: Some(
                        "Communication on Port 3: 0:No stable communication 1:Communication established.",
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
            ],
        },
        FieldSet {
            name: "EscPdiCtrl",
            extends: None,
            description: Some(
                "PDI Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pdi",
                    description: Some(
                        "Process data interface: 0x00:Interface deactivated (no PDI) 0x01:4 Digital Input 0x02:4 Digital Output 0x03:2 Digital Input and 2 Digital Output 0x04:Digital I/O 0x05:SPI Slave 0x06:Oversampling I/O 0x07:EtherCAT Bridge (port 3) 0x08:16 Bit asynchronous Microcontroller interface 0x09:8 Bit asynchronous Microcontroller interface 0x0A:16 Bit synchronous Microcontroller interface 0x0B:8 Bit synchronous Microcontroller interface 0x10:32 Digital Input and 0 Digital Output 0x11:24 Digital Input and 8 Digital Output 0x12:16 Digital Input and 16 Digital Output 0x13:8 Digital Input and 24 Digital Output 0x14:0 Digital Input and 32 Digital Output 0x80:On-chip bus Others:Reserved.",
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
            name: "EscRstEcat",
            extends: None,
            description: Some(
                "ESC Reset ECAT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pr",
                    description: Some(
                        "Progress of the reset procedure: 00:initial/reset state 01:after writing 0x52 ('R'), when previous state was 00 10:after writing 0x45 ('E'), when previous state was 01 11:after writing 0x53 ('S'), when previous state was 10. This value must not be observed because the ESC enters reset when this state is reached, resulting in state 00.",
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
            name: "EscRstPdi",
            extends: None,
            description: Some(
                "ESC Reset PDI.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rst",
                    description: Some(
                        "A reset is asserted after writing the reset sequence 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive commands. Any other command which does not continue the sequence by writing the next expected value will cancel the reset procedure.",
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
            name: "EscType",
            extends: None,
            description: Some(
                "Type of EtherCAT controller.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "type_",
                    description: Some(
                        "Controller type.",
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
            name: "EscWen",
            extends: None,
            description: Some(
                "ESC Write Enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "If ESC write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if ESC Write Protection is disabled.",
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
            name: "EscWp",
            extends: None,
            description: Some(
                "ESC Write Protection.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Write protect: 0:Protection disabled 1:Protection enabled All areas are write-protected, except for 0x0030.",
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
            name: "Feature",
            extends: None,
            description: Some(
                "ESC Feature supported.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "fmmu",
                    description: Some(
                        "FMMU Operation: 0:Bit oriented 1:Byte oriented.",
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
                    name: "dc",
                    description: Some(
                        "Distributed Clocks: 0:Not available 1:Available.",
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
                    name: "dcw",
                    description: Some(
                        "Distributed Clocks width: 0:32 bit 1:64 bit.",
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
                    name: "eldm",
                    description: Some(
                        "Enhanced Link Detection MII: 0:Not available 1:Available.",
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
                    name: "shfe",
                    description: Some(
                        "Seperate Handling of FCS Errors: 0:Not supported 1:Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter.",
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
                    name: "edsa",
                    description: Some(
                        "Enhanced DC SYNC Activation: 0:Not available 1:Available Note:This feature refers to registers 0x981[7:3] and 0x0984.",
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
                    name: "lrw",
                    description: Some(
                        "EtherCAT LRW command support: 0:Supported 1:Not supported.",
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
                    name: "rwc",
                    description: Some(
                        "EtherCAT read/write command support(BRW,APRW,FPRW): 0:Supported 1:Not supported.",
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
                    name: "ffsc",
                    description: Some(
                        "Fixed FMMU/SyncManager configuration: 0:Variable configuration 1:Fixed configuration (refer to documentation of supporting ESCs).",
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
            ],
        },
        FieldSet {
            name: "FmmuActivate",
            extends: None,
            description: Some(
                "Activate.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "act",
                    description: Some(
                        "0:FMMU deactivated 1:FMMU activated. FMMU checks logically addressed blocks to be mapped according to configured mapping.",
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
            name: "FmmuLength",
            extends: None,
            description: Some(
                "Length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "Offset from the first logical FMMU byte to the last FMMU byte + 1 (e.g., if two bytes are used, then this parameter shall contain 2).",
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
            name: "FmmuNum",
            extends: None,
            description: Some(
                "FMMU supported.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "num",
                    description: Some(
                        "Number of supported FMMU channels (or entities).",
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
            name: "FmmuPhysicalStartAddr",
            extends: None,
            description: Some(
                "Physical Start Address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Physical Start Address (mapped to logical Start address).",
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
            name: "FmmuType",
            extends: None,
            description: Some(
                "Type.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "map_rd",
                    description: Some(
                        "0:Ignore mapping for read accesses 1:Use mapping for read accesses.",
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
                    name: "map_wr",
                    description: Some(
                        "0:Ignore mapping for write accesses 1:Use mapping for write accesses.",
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
            name: "FwdRxErrCnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "err_cnt",
                    description: Some(
                        "Forwarded error counter of Port y (counting is stopped when 0xFF is reached).",
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
            name: "Gpi",
            extends: None,
            description: Some(
                "General Purpose Inputs.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "gpid",
                    description: Some(
                        "General Purpose Input Data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GpiOverride0",
            extends: None,
            description: Some(
                "GPI low word Override value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr_override_low",
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
            name: "GpiOverride1",
            extends: None,
            description: Some(
                "GPI high word Override value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr_override_high",
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
            name: "GpiReg0",
            extends: None,
            description: Some(
                "GPI low word read value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
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
            name: "GpiReg1",
            extends: None,
            description: Some(
                "GPI high word read value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
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
            name: "GpioCtrl",
            extends: None,
            description: Some(
                "GPIO Output Enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpo_trig_sel",
                    description: Some(
                        "select the trigger signal to latch GPO. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl[30]; others no trigger.",
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
                    name: "gpo_trig_en",
                    description: Some(
                        "use gpo_trig_sel can select the trigger event to latch GPO signal(from core) set to use triggered signal; clr to use GPO signals direclty(from reg or pad).",
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
                    name: "gpi_trig_sel",
                    description: Some(
                        "select the trigger signal to latch GPI. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl[31]; others no trigger.",
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
                    name: "gpi_trig_en",
                    description: Some(
                        "use gpi_trig_sel can select the trigger event to latch GPI signal(from reg or pad) set to use triggered signal; clr to use signals direclty(from reg or pad) assign pdi_gpi = gpi_trig_en ? gpi_reg : (gpi_override_en ? gpi_override :pad_di_ecat_gpi);.",
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
                    name: "gpi_override_en",
                    description: Some(
                        "set this bit will use GPI from the software register gpi_override0/1 clr to use GPI from pad directly.",
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
                    name: "sw_latch_gpo",
                    description: Some(
                        "if gpo_trig_sel is set to 4'b1001, setting this bit will latch GPO to gpo_reg0/1.",
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
                    name: "sw_latch_gpi",
                    description: Some(
                        "if gpi_trig_sel is set to 4'b1001, setting this bit will latch GPI to gpi_reg0/1.",
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
            name: "Gpo",
            extends: None,
            description: Some(
                "General Purpose Outputs.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "gpod",
                    description: Some(
                        "General Purpose Output Data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GpoReg0",
            extends: None,
            description: Some(
                "GPO low word read value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
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
            name: "GpoReg1",
            extends: None,
            description: Some(
                "GPO high word read value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
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
            name: "GprCfg0",
            extends: None,
            description: Some(
                "General Purpose Configure 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "prom_size",
                    description: Some(
                        "Sets EEPROM size: 0:up to 16 kbit EEPROM 1:32 kbit-4Mbit EEPROM.",
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
                    name: "i2c_sclk_en",
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
                Field {
                    name: "eeprom_emu",
                    description: Some(
                        "1 is EEPROM emulation mode (default).",
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
                    name: "clk100_en",
                    description: Some(
                        "No description available.",
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
            name: "GprCfg1",
            extends: None,
            description: Some(
                "General Purpose Configure 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsto_ovrd_enj",
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
                    name: "rsto_ovrd",
                    description: Some(
                        "No description available.",
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
                    name: "latch0_from_io",
                    description: Some(
                        "0:from TRIGGER_MUX.",
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
                    name: "latch1_from_io",
                    description: Some(
                        "0:from NTM.",
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
                    name: "sync0_dma_en",
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
                    name: "sync1_dma_en",
                    description: Some(
                        "No description available.",
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
                    name: "rsto_irq_en",
                    description: Some(
                        "No description available.",
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
                    name: "sync0_irq_en",
                    description: Some(
                        "No description available.",
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
                    name: "sync1_irq_en",
                    description: Some(
                        "No description available.",
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
            name: "GprCfg2",
            extends: None,
            description: Some(
                "General Purpose Configure 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nmii_link0_gpr",
                    description: Some(
                        "No description available.",
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
                    name: "nmii_link0_from_io",
                    description: Some(
                        "No description available.",
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
                    name: "nmii_link1_gpr",
                    description: Some(
                        "No description available.",
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
                    name: "nmii_link1_from_io",
                    description: Some(
                        "No description available.",
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
                    name: "nmii_link2_gpr",
                    description: Some(
                        "No description available.",
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
                    name: "nmii_link2_from_io",
                    description: Some(
                        "No description available.",
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
            name: "GprStatus",
            extends: None,
            description: Some(
                "global status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "link_act",
                    description: Some(
                        "No description available.",
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
                    name: "dev_state",
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
                Field {
                    name: "led_run",
                    description: Some(
                        "No description available.",
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
                    name: "led_err",
                    description: Some(
                        "No description available.",
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
                    name: "led_state_run",
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
                    name: "sync_out0",
                    description: Some(
                        "No description available.",
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
                    name: "sync_out1",
                    description: Some(
                        "No description available.",
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
                    name: "pdi_wd_state",
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
                    name: "pdi_wd_trigger",
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
                    name: "pdi_eof",
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
                Field {
                    name: "pdi_sof",
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
                    name: "nlink0_padsel",
                    description: Some(
                        "No description available.",
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
                    name: "nlink1_padsel",
                    description: Some(
                        "No description available.",
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
                    name: "nlink2_padsel",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IoCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "func_alt",
                    description: Some(
                        "IO usage: 0:NMII_LINK0 1:NMII_LINK1 2:NMII_LINK2 3:LINK_ACT0 4:LINK_ACT1 5:LINK_ACT2 6:LED_RUN 7:LED_ERR 8:RESET_OUT.",
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
                    name: "invert",
                    description: Some(
                        "1:invert the IO.",
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
            name: "Latch0Ctrl",
            extends: None,
            description: Some(
                "Latch0 Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pos_edge",
                    description: Some(
                        "Latch0 positive edge: 0:Continuous Latch active 1:Single event (only first event active).",
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
                    name: "neg_edge",
                    description: Some(
                        "Latch0 negative edge: 0:Continuous Latch active 1:Single event (only first event active).",
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
            name: "Latch0Stat",
            extends: None,
            description: Some(
                "Latch0 Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pos_edge",
                    description: Some(
                        "Event Latch0 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch0 Time Positive Edge.",
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
                    name: "neg_edge",
                    description: Some(
                        "Event Latch0 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch0 Time Negative Edge.",
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
                    name: "pin_stat",
                    description: Some(
                        "Latch0 pin state.",
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
            ],
        },
        FieldSet {
            name: "Latch0TimeNe",
            extends: None,
            description: Some(
                "Latch0 Time Negative Edge.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "System time at the negative edge of the Latch0 signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Latch0TimePe",
            extends: None,
            description: Some(
                "Latch0 Time Positive Edge.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "System time at the positive edge of the Latch0 signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Latch1Ctrl",
            extends: None,
            description: Some(
                "Latch1 Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pos_edge",
                    description: Some(
                        "Latch1 positive edge: 0:Continuous Latch active 1:Single event (only first event active).",
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
                    name: "neg_edge",
                    description: Some(
                        "Latch1 negative edge: 0:Continuous Latch active 1:Single event (only first event active).",
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
            name: "Latch1Stat",
            extends: None,
            description: Some(
                "Latch1 Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pos_edge",
                    description: Some(
                        "Event Latch1 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch1 Time Positive Edge.",
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
                    name: "neg_edge",
                    description: Some(
                        "Event Latch1 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch1 Time Negative Edge.",
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
                    name: "pin_stat",
                    description: Some(
                        "Latch1 pin state.",
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
            ],
        },
        FieldSet {
            name: "Latch1TimeNe",
            extends: None,
            description: Some(
                "Latch1 Time Negative Edge.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "System time at the negative edge of the Latch1 signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Latch1TimePe",
            extends: None,
            description: Some(
                "Latch1 Time Positive Edge.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "System time at the positive edge of the Latch1 signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LogicStartAddr",
            extends: None,
            description: Some(
                "Logical Start Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Logical start address within the EtherCAT Address Space.",
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
            name: "LogicStartBit",
            extends: None,
            description: Some(
                "Logical Start Bit.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7).",
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
            ],
        },
        FieldSet {
            name: "LogicStopBit",
            extends: None,
            description: Some(
                "Logical Stop Bit.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "stop",
                    description: Some(
                        "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7).",
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
            ],
        },
        FieldSet {
            name: "LostLinkCnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Lost Link counter of Port y (counting is stopped when 0xff is reached). Counts only if port is open and loop is Auto.",
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
            name: "MiiMngCs",
            extends: None,
            description: Some(
                "MII Management Control/Status.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "wen",
                    description: Some(
                        "Write enable*: 0:Write disabled 1:Write enabled This bit is always 1 if PDI has MI control. ET1100-0000/-0001 exception: Bit is not always 1 if PDI has MI control, and bit is writable by PDI.",
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
                    name: "pdi",
                    description: Some(
                        "Management Interface can be controlled by PDI (registers 0x0516-0x0517): 0:Only ECAT control 1:PDI control possible.",
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
                    name: "link_dc",
                    description: Some(
                        "MI link detection and configuration: 0:Disabled for all ports 1:Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details.",
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
                    name: "phy_addr",
                    description: Some(
                        "PHY address of port 0 (this is equal to the PHY address offset, if the PHY addresses are consecutive) IP Core since V3.0.0/3.00c: Translation 0x0512[7]=0: Register 0x0510[7:3] shows PHY address of port 0 Translation 0x0512[7]=1: Register 0x0510[7:3] shows the PHY address which will be used for port 0-3 as requested by 0x0512[4:0] (valid values 0-3).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmd",
                    description: Some(
                        "Command register*: Write:Initiate command. Read:Currently executed command 00:No command/MI idle (clear error bits) 01:Read 10:Write Others:Reserved/invalid command (do not issue).",
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
                    name: "rd_err",
                    description: Some(
                        "Read error: 0:No read error 1:Read error occurred (PHY or register not available) Cleared by writing to register 0x0511.",
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
                    name: "cmd_err",
                    description: Some(
                        "Command error: 0:Last Command was successful 1:Invalid command or write command without Write Enable Cleared by executing a valid command or by writing “00” to Command register bits [9:8].",
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
                    name: "busy",
                    description: Some(
                        "Busy: 0:MII Management Interface is idle 1:MII Management Interface is busy.",
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
            ],
        },
        FieldSet {
            name: "MiimEcatAccStat",
            extends: None,
            description: Some(
                "MII Management ECAT Access State.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "Access to MII management: 0:ECAT enables PDI takeover of MII management interface 1:ECAT claims exclusive access to MII management interface.",
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
            name: "MiimPdiAccStat",
            extends: None,
            description: Some(
                "MII Management PDI Access State.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "Access to MII management: 0:ECAT has access to MII management 1:PDI has access to MII management.",
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
                    name: "force",
                    description: Some(
                        "Force PDI Access State: 0:Do not change Bit 0x0517[0] 1:Reset Bit 0x0517[0] to 0.",
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
            name: "NxtSync1Pulse",
            extends: None,
            description: Some(
                "Next SYNC1 Pulse.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "System time of next SYNC1 pulse in ns.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PdiAlEvtMsk",
            extends: None,
            description: Some(
                "PDI AL Event Mask.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask",
                    description: Some(
                        "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal: 0:Corresponding AL Event Request register bit is not mapped 1:Corresponding AL Event Request register bit is mapped.",
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
            name: "PdiBufCet",
            extends: None,
            description: Some(
                "PDI Buffer Change Event Time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Local time when at least one SyncManager asserts a PDI buffer change event.",
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
            name: "PdiBufSet",
            extends: None,
            description: Some(
                "PDI Buffer Start Event Time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Local time when at least one SyncManager asserts a PDI buffer start event.",
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
            name: "PdiCfg",
            extends: None,
            description: Some(
                "PDI Configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk",
                    description: Some(
                        "On-chip bus clock: 0:asynchronous 1-31:synchronous multiplication factor (N * 25 MHz).",
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
                    name: "bus",
                    description: Some(
                        "On-chip bus: 000:Intel® Avalon® 001:AXI® 010:Xilinx® PLB v4.6 100:Xilinx OPB others:reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PdiErrCnt",
            extends: None,
            description: Some(
                "PDI Error Counter.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "PDI Error counter (counting is stopped when 0xFF is reached). Counts if a PDI access has an interface error.",
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
            name: "PdiExtCfg",
            extends: None,
            description: Some(
                "PDI Extended Configuration.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rps",
                    description: Some(
                        "Read prefetch size (in cycles of PDI width): 0:4 cycles 1:1 cycle (typical) 2:2 cycles 3:Reserved.",
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
                    name: "ocbst",
                    description: Some(
                        "On-chip bus sub-type for AXI: 000:AXI3 001:AXI4 010:AXI4 LITE others:reserved.",
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
            ],
        },
        FieldSet {
            name: "PdiInfo",
            extends: None,
            description: Some(
                "PDI Information.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pfabw",
                    description: Some(
                        "DI function acknowledge by write: 0:Disabled 1:Enabled.",
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
                    name: "eclfe",
                    description: Some(
                        "ESC configuration area loaded from EEPROM: 0:not loaded 1:loaded.",
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
                    name: "pdia",
                    description: Some(
                        "PDI active: 0:PDI not active 1:PDI active.",
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
                    name: "pdicn",
                    description: Some(
                        "PDI configuration invalid: 0:PDI configuration ok 1:PDI configuration invalid.",
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
            name: "PdiSlCfg",
            extends: None,
            description: Some(
                "PDI Sync/Latch[1:0] Configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sync0_odp",
                    description: Some(
                        "SYNC0 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high).",
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
                    name: "sync0_cfg",
                    description: Some(
                        "SYNC0/LATCH0 configuration*: 0:LATCH0 Input 1:SYNC0 Output.",
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
                    name: "sync0_maer",
                    description: Some(
                        "SYNC0 mapped to AL Event Request register 0x0220[2]: 0:Disabled 1:Enabled.",
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
                    name: "sync1_odp",
                    description: Some(
                        "SYNC1 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high).",
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
                    name: "sync1_cfg",
                    description: Some(
                        "SYNC1/LATCH1 configuration*: 0:LATCH1 input 1:SYNC1 output.",
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
                    name: "sync1_maer",
                    description: Some(
                        "SYNC1 mapped to AL Event Request register 0x0220[3]: 0:Disabled 1:Enabled.",
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
            name: "Pdram",
            extends: None,
            description: Some(
                "Process Data Ram.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Input Data.",
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
            name: "PdramAls",
            extends: None,
            description: Some(
                "Process Data Ram Alias.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
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
            name: "PhyAddr",
            extends: None,
            description: Some(
                "PHY Address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Target PHY Address Translation 0x0512[7]=0: 0-3:Target PHY Addresses 0-3 are used to access the PHYs at port 0-3, when the PHY addresses are properly configured 4-31:The configured PHY address of port 0 (PHY address offset) is added to the Target PHY Address values 4-31 when accessing a PHY Translation 0x0512[7]=1: 0-31:Target PHY Addresses is used when accessing a PHY without translation.",
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
                    name: "show",
                    description: Some(
                        "Target PHY Address translation: 0:Enabled 1:Disabled Refer to 0x0512[4:0] and 0x0510[7:3] for details.",
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
            name: "PhyCfg0",
            extends: None,
            description: Some(
                "PHY Configure 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port0_rmii_en",
                    description: Some(
                        "No description available.",
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
                    name: "port1_rmii_en",
                    description: Some(
                        "No description available.",
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
                    name: "port2_rmii_en",
                    description: Some(
                        "No description available.",
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
                    name: "mac_speed",
                    description: Some(
                        "1:100M.",
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
            ],
        },
        FieldSet {
            name: "PhyCfg1",
            extends: None,
            description: Some(
                "PHY Configure 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmii_p0_txck_refclk_oe",
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
                    name: "rmii_p1_txck_refclk_oe",
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
                    name: "rmii_p2_txck_refclk_oe",
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
                    name: "refck_25m_oe",
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
                Field {
                    name: "rmii_p0_rxck_refclk_oe",
                    description: Some(
                        "No description available.",
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
                    name: "rmii_p1_rxck_refclk_oe",
                    description: Some(
                        "No description available.",
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
                    name: "rmii_p2_rxck_refclk_oe",
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
                    name: "refck_25m_inv",
                    description: Some(
                        "No description available.",
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
                    name: "rmii_refclk_sel",
                    description: Some(
                        "0:use RXCK as 50M refclk. 1:use TXCK as 50M refclk.",
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
            ],
        },
        FieldSet {
            name: "PhyData",
            extends: None,
            description: Some(
                "PHY Data.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "PHY Read/Write Data.",
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
            name: "PhyRegAddr",
            extends: None,
            description: Some(
                "PHY Register Address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Address of PHY Register that shall be read/written.",
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
            ],
        },
        FieldSet {
            name: "PhyStat",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pls",
                    description: Some(
                        "Physical link status (PHY status register 1.2): 0:No physical link 1:Physical link detected.",
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
                    name: "ls",
                    description: Some(
                        "Link status (100 Mbit/s, Full Duplex, Auto negotiation): 0:No link 1:Link detected.",
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
                    name: "lse",
                    description: Some(
                        "Link status error: 0:No error 1:Link error, link inhibited.",
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
                    name: "re",
                    description: Some(
                        "Read error: 0:No read error occurred 1:A read error has occurred Cleared by writing any value to at least one of the PHY Port y Status registers.",
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
                    name: "lpe",
                    description: Some(
                        "Link partner error: 0:No error detected 1:Link partner error.",
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
                    name: "pcu",
                    description: Some(
                        "PHY configuration updated: 0:No update 1:PHY configuration was updated Cleared by writing any value to at least one of the PHY Port y Status registers.",
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
            name: "PhysicalRwOffset",
            extends: None,
            description: Some(
                "Physical Read/Write Offset.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "This register is used for ReadWrite commands in Device Addressing mode (FPRW, APRW, BRW). The internal read address is directly taken from the offset address field of the EtherCAT datagram header, while the internal write address is calculated by adding the Physical Read/Write Offset value to the offset address field. Internal read address = ADR, internal write address = ADR + R/W-Offset.",
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
            name: "PhysicalStartBit",
            extends: None,
            description: Some(
                "Physical Start Bit.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7).",
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
            ],
        },
        FieldSet {
            name: "Pid",
            extends: None,
            description: Some(
                "Product ID.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "pid",
                    description: Some(
                        "Product ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PortDesc",
            extends: None,
            description: Some(
                "Port Descriptor.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port0",
                    description: Some(
                        "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII.",
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
                    name: "port1",
                    description: Some(
                        "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII.",
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
                    name: "port2",
                    description: Some(
                        "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII.",
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
                    name: "port3",
                    description: Some(
                        "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII.",
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
            ],
        },
        FieldSet {
            name: "PulseLen",
            extends: None,
            description: Some(
                "Pulse Length of SyncSignals.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Pulse length of SyncSignals (in Units of 10ns) 0:Acknowledge mode:SyncSignal will be cleared by reading SYNC[1:0] Status register.",
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
            name: "RamSize",
            extends: None,
            description: Some(
                "RAM Size.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "size",
                    description: Some(
                        "Process Data RAM size supported in KByte.",
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
            name: "RcvTime",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "req",
                    description: Some(
                        "Write: A write access to register 0x0900 with BWR or FPWR latches the local time at the beginning of the receive frame (start first bit of preamble) at each port. Write (ESC20, ET1200 exception): A write access latches the local time at the beginning of the receive frame at port 0. It enables the time stamping at the other ports. Read: Local time at the beginning of the last receive frame containing a write access to this register. NOTE:FPWR requires an address match for accessing this register like any FPWR command. All write commands with address match will increment the working counter (e.g., APWR), but they will not trigger receive time latching.",
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
                    name: "lt",
                    description: Some(
                        "Local time at the beginning of the last receive frame containing a write access to register 0x0900.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RcvTimeLm",
            extends: None,
            description: Some(
                "Receive Time Latch Mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "latch_mode",
                    description: Some(
                        "Receive Time Latch Mode: 0:Forwarding mode (used if frames are entering the ESC at port 0 first): Receive time stamps of ports 1-3 are enabled after the write access to 0x0900, so the following frame at ports 1-3 will be time stamped (this is typically the write frame to 0x0900 coming back from the network behind the ESC). 1:Reverse mode (used if frames are entering ESC at port 1-3 first): Receive time stamps of ports 1-3 are immediately taken over from the internal hidden time stamp registers, so the previous frame entering the ESC at ports 1-3 will be time stamped when the write frame to 0x0900 enters port 0 (the previous frame at ports 1-3 is typically the write frame to 0x0900 coming from the master, which will enable time stamp.",
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
            name: "RcvtEcatPu",
            extends: None,
            description: Some(
                "Receive Time ECAT Processing Unit.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "lt",
                    description: Some(
                        "Local time at the beginning of a frame (start first bit of preamble) received at the ECAT Processing Unit containing a write access to register 0x0900 NOTE:E.g., if port 0 is open, this register reflects the Receive Time Port 0 as a 64 Bit value. Any valid EtherCAT write access to register 0x0900 triggers latching, not only BWR/FPWR commands as with register 0x0900.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RegWen",
            extends: None,
            description: Some(
                "Register Write Enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "If register write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if Register Write Protection is disabled.",
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
            name: "RegWp",
            extends: None,
            description: Some(
                "Register Write Protection.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Register write protection: 0:Protection disabled 1:Protection enabled Registers 0x0000:0x0F7F are write-protected, except for 0x0020 and 0x0030.",
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
            name: "Revision",
            extends: None,
            description: Some(
                "Revision of EtherCAT controller.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "major version X.",
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
            name: "RunLedOvrd",
            extends: None,
            description: Some(
                "RUN LED Override.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "led_code",
                    description: Some(
                        "LED code: 0x0:Off 0x1:Flash 1x 0x2-0xC:Flash 2x – 12x 0xD:Blinking 0xE:Flickering 0xF:On.",
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
                    name: "en_ovrd",
                    description: Some(
                        "Enable Override: 0:Override disabled 1:Override enabled.",
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
            name: "RxErrCnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ivd_frm",
                    description: Some(
                        "Invalid frame counter of Port y (counting is stopped when 0xFF is reached).",
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
                    name: "rx_err",
                    description: Some(
                        "RX Error counter of Port y (counting is stopped when 0xFF is reached).",
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
            name: "SpdCntDiff",
            extends: None,
            description: Some(
                "Speed Counter Diff.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "diff",
                    description: Some(
                        "Representation of the deviation between local clock period and Reference Clock's clock period (representation:two's complement) Range:±(Speed Counter Start – 0x7F).",
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
            name: "SpdCntFd",
            extends: None,
            description: Some(
                "Speed Counter Filter Depth.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "depth",
                    description: Some(
                        "Filter depth for averaging the clock period deviation IP Core since V2.2.0/V2.02a: A write access resets the internal speed counter filter.",
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
            name: "SpdCntStart",
            extends: None,
            description: Some(
                "Speed Counter Start.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "bw",
                    description: Some(
                        "Bandwidth for adjustment of local copy of System Time (larger values → smaller bandwidth and smoother adjustment) A write access resets System Time Difference (0x092C:0x092F) and Speed Counter Diff (0x0932:0x0933). Valid values:0x0080 to 0x3FFF.",
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
            name: "StartTimeCo",
            extends: None,
            description: Some(
                "Start Time Cyclic Operation.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "st",
                    description: Some(
                        "Write:Start time (System time) of cyclic operation in ns Read:System time of next SYNC0 pulse in ns.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "StationAddr",
            extends: None,
            description: Some(
                "Configured Station Address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands).",
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
            name: "StationAls",
            extends: None,
            description: Some(
                "Configured Station Alias.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Alias Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands). The use of this alias is activated by Register DL Control Bit 0x0100[24]. NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset. ESC20 exception:EEPROM value is transferred into this register after each EEPROM reload command.",
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
            name: "Status",
            extends: None,
            description: Some(
                "Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "int_wr",
                    description: Some(
                        "Interrupt Write: 1:Interrupt after buffer was completely and successfully written 0:Interrupt cleared after first byte of buffer was read NOTE:This interrupt is signalled to the reading side if enabled in the SM Control register.",
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
                    name: "int_rd",
                    description: Some(
                        "Interrupt Read: 1:Interrupt after buffer was completely and successfully read 0:Interrupt cleared after first byte of buffer was written NOTE:This interrupt is signalled to the writing side if enabled in the SM Control register.",
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
                    name: "mbx_mode",
                    description: Some(
                        "Mailbox mode:mailbox status: 0:Mailbox empty 1:Mailbox full Buffered mode:reserved.",
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
                    name: "buf_mode",
                    description: Some(
                        "Buffered mode:buffer status (last written buffer): 00:1 st buffer 01:2 nd buffer 10:3 rd buffer 11:(no buffer written) Mailbox mode:reserved.",
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
                    name: "rb_inuse",
                    description: Some(
                        "Read buffer in use (opened).",
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
                    name: "wb_inuse",
                    description: Some(
                        "Write buffer in use (opened).",
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
            name: "Sync0CycTime",
            extends: None,
            description: Some(
                "SYNC0 Cycle Time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cyc",
                    description: Some(
                        "Time between two consecutive SYNC0 pulses in ns. 0:Single shot mode, generate only one SYNC0 pulse.",
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
            name: "Sync0Stat",
            extends: None,
            description: Some(
                "SYNC0 Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ack",
                    description: Some(
                        "SYNC0 state for Acknowledge mode. SYNC0 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode.",
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
            name: "Sync1CycTime",
            extends: None,
            description: Some(
                "SYNC1 Cycle Time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cyc",
                    description: Some(
                        "Time between SYNC0 pulse and SYNC1 pulse in ns.",
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
            name: "Sync1Stat",
            extends: None,
            description: Some(
                "SYNC1 Status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ack",
                    description: Some(
                        "SYNC1 state for Acknowledge mode. SYNC1 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode.",
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
            name: "SyncmActivate",
            extends: None,
            description: Some(
                "Activate.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "SyncManager Enable/Disable: 0:Disable:Access to Memory without SyncManager control 1:Enable:SyncManager is active and controls Memory area set in configuration.",
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
                    name: "repeat",
                    description: Some(
                        "Repeat Request: A toggle of Repeat Request means that a mailbox retry is needed (primarily used in conjunction with ECAT Read Mailbox).",
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
                    name: "latch_ecat",
                    description: Some(
                        "Latch Event ECAT: 0:No 1:Generate Latch event when EtherCAT master issues a buffer exchange.",
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
                    name: "latch_pdi",
                    description: Some(
                        "Latch Event PDI: 0:No 1:Generate Latch events when PDI issues a buffer exchange or when PDI accesses buffer start address.",
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
            name: "SyncmLength",
            extends: None,
            description: Some(
                "Length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Number of bytes assigned to SyncManager (shall be greater than 1, otherwise SyncManager is not activated. If set to 1, only Watchdog Trigger is generated if configured).",
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
            name: "SyncmNum",
            extends: None,
            description: Some(
                "SyncManagers supported.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "num",
                    description: Some(
                        "Number of supported SyncManager channels (or entities).",
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
            name: "SyncmPdiCtrl",
            extends: None,
            description: Some(
                "PDI Control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "deact",
                    description: Some(
                        "Deactivate SyncManager: Read: 0:Normal operation, SyncManager activated. 1:SyncManager deactivated and reset. SyncManager locks access to Memory area. Write: 0:Activate SyncManager 1:Request SyncManager deactivation NOTE:Writing 1 is delayed until the end of the frame, which is currently processed.",
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
                    name: "repeat_ack",
                    description: Some(
                        "Repeat Ack: If this is set to the same value as that set by Repeat Request, the PDI acknowledges the execution of a previous set Repeat request.",
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
            name: "SyncmPhysicalStartAddr",
            extends: None,
            description: Some(
                "Physical Start Address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "First byte that will be handled by SyncManager.",
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
            name: "SyncoAct",
            extends: None,
            description: Some(
                "SYNC Out Unit Activation.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "soua",
                    description: Some(
                        "Sync Out Unit activation: 0:Deactivated 1:Activated.",
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
                    name: "sync0_gen",
                    description: Some(
                        "SYNC0 generation: 0:Deactivated 1:SYNC0 pulse is generated.",
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
                    name: "sync1_gen",
                    description: Some(
                        "SYNC1 generation: 0:Deactivated 1:SYNC1 pulse is generated.",
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
                    name: "ac",
                    description: Some(
                        "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997): 0:Disabled 1:Auto-activation enabled. 0x0981[0] is set automatically after Start Time is written.",
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
                    name: "ext",
                    description: Some(
                        "Extension of Start Time Cyclic Operation (0x0990:0x0993): 0:No extension 1:Extend 32 bit written Start Time to 64 bit.",
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
                    name: "stpc",
                    description: Some(
                        "Start Time plausibility check: 0:Disabled. SyncSignal generation if Start Time is reached. 1:Immediate SyncSignal generation if Start Time is outside near future (see 0x0981[6]).",
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
                    name: "nfc",
                    description: Some(
                        "Near future configuration (approx.): 0:½ DC width future (231 ns or 263 ns) 1:~2.1 sec. future (231 ns).",
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
                    name: "ssdp",
                    description: Some(
                        "SyncSignal debug pulse (Vasily bit): 0:Deactivated 1:Immediately generate one ping only on SYNC0-1 according to 0x0981[2:1 for debugging This bit is self-clearing, always read 0. All pulses are generated at the same time, the cycle time is ignored. The configured pulse length is used.",
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
            name: "SysTime",
            extends: None,
            description: Some(
                "System Time.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "st",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SysTimeDelay",
            extends: None,
            description: Some(
                "System Time Delay.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dly",
                    description: Some(
                        "Delay between Reference Clock and the ESC.",
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
            name: "SysTimeDiff",
            extends: None,
            description: Some(
                "System Time Difference.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num",
                    description: Some(
                        "Mean difference between local copy of System Time and received System Time values Difference = Received System Time – local copy of System Time.",
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
                    name: "diff",
                    description: Some(
                        "0:Local copy of System Time less than received System Time 1:Local copy of System Time greater than or equal to received System Time.",
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
            name: "SysTimeDiffFd",
            extends: None,
            description: Some(
                "System Time Difference Filter Depth.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "depth",
                    description: Some(
                        "Filter depth for averaging the received System Time deviation IP Core since V2.2.0/V2.02a: A write access resets System Time Difference (0x092C:0x092F).",
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
            name: "SysTimeOffset",
            extends: None,
            description: Some(
                "System Time Offset.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "offset",
                    description: Some(
                        "Difference between local time and System Time. Offset is added to the local time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 64,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UserRamByte0",
            extends: None,
            description: Some(
                "User Ram Byte 0.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "extf",
                    description: Some(
                        "Number of extended feature bits.",
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
            name: "UserRamByte1",
            extends: None,
            description: Some(
                "User Ram Byte 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "edlcr",
                    description: Some(
                        "Extended DL Control Register (0x0102:0x0103).",
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
                    name: "alscr",
                    description: Some(
                        "AL Status Code Register (0x0134:0x0135).",
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
                    name: "eim",
                    description: Some(
                        "ECAT Interrupt Mask (0x0200:0x0201).",
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
                    name: "csa",
                    description: Some(
                        "Configured Station Alias (0x0012:0x0013).",
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
                    name: "gpi",
                    description: Some(
                        "General Purpose Inputs (0x0F18:0x0F1F).",
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
                    name: "gpo",
                    description: Some(
                        "General Purpose Outputs (0x0F10:0x0F17).",
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
                    name: "aemw",
                    description: Some(
                        "AL Event Mask writable (0x0204:0x0207).",
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
                    name: "prwo",
                    description: Some(
                        "Physical Read/Write Offset (0x0108:0x0109).",
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
            name: "UserRamByte10",
            extends: None,
            description: Some(
                "User Ram Byte 10.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dcl1d",
                    description: Some(
                        "DC Latch1 disable.",
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
                    name: "apdi",
                    description: Some(
                        "AXI PDI.",
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
                    name: "pdifa",
                    description: Some(
                        "PDI function acknowledge by PDI write.",
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
                    name: "pdiir",
                    description: Some(
                        "PDI Information register (0x014E:0x014F).",
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
            name: "UserRamByte11",
            extends: None,
            description: Some(
                "User Ram Byte 11.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ledtst",
                    description: Some(
                        "LED test.",
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
            name: "UserRamByte14",
            extends: None,
            description: Some(
                "User Ram Byte 14.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "diobs",
                    description: Some(
                        "Digital I/O PDI byte size.",
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
            ],
        },
        FieldSet {
            name: "UserRamByte15",
            extends: None,
            description: Some(
                "User Ram Byte 15.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "diopdi",
                    description: Some(
                        "Digital I/O PDI.",
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
                    name: "sspdi",
                    description: Some(
                        "SPI Slave PDI.",
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
                    name: "aucpdi",
                    description: Some(
                        "Asynchronous µC PDI.",
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
            name: "UserRamByte19",
            extends: None,
            description: Some(
                "User Ram Byte 19.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rgmii",
                    description: Some(
                        "RGMII.",
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
                    name: "iparo",
                    description: Some(
                        "Individual PHY address read out (0x0510[7:3]).",
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
                    name: "cia",
                    description: Some(
                        "CLK_PDI_EXT is asynchronous.",
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
                    name: "urgp",
                    description: Some(
                        "Use RGMII GTX_CLK phase shifted clock input.",
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
                    name: "rmii",
                    description: Some(
                        "RMII.",
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
                    name: "scp",
                    description: Some(
                        "Security CPLD protection.",
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
            name: "UserRamByte2",
            extends: None,
            description: Some(
                "User Ram Byte 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wdw",
                    description: Some(
                        "Watchdog divider writable (0x0400:0x0401) and Watchdog PDI (0x0410:0x0411).",
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
                    name: "wdgcnt",
                    description: Some(
                        "Watchdog counters (0x0442:0x0443).",
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
                    name: "wp",
                    description: Some(
                        "Write Protection (0x0020:0x0031).",
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
                    name: "reset",
                    description: Some(
                        "Reset (0x0040:0x0041).",
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
                    name: "dcsmet",
                    description: Some(
                        "DC SyncManager Event Times (0x09F0:0x09FF).",
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
                    name: "epupec",
                    description: Some(
                        "ECAT Processing Unit/PDI Error Counter (0x030C:0x030D).",
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
                    name: "escfg",
                    description: Some(
                        "EEPROM Size configurable (0x0502[7]): 0:EEPROM Size fixed to sizes up to 16 Kbit 1:EEPROM Size configurable.",
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
            name: "UserRamByte3",
            extends: None,
            description: Some(
                "User Ram Byte 3.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "llc",
                    description: Some(
                        "Lost Link Counter (0x0310:0x0313).",
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
                    name: "mmi",
                    description: Some(
                        "MII Management Interface (0x0510:0x0515).",
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
                    name: "eldm",
                    description: Some(
                        "Enhanced Link Detection MII.",
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
                    name: "elde",
                    description: Some(
                        "Enhanced Link Detection EBUS.",
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
                    name: "rled",
                    description: Some(
                        "Run LED (DEV_STATE LED).",
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
            name: "UserRamByte4",
            extends: None,
            description: Some(
                "User Ram Byte 4.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "laled",
                    description: Some(
                        "Link/Activity LED.",
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
                    name: "dliu",
                    description: Some(
                        "DC Latch In Unit.",
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
                    name: "dsou",
                    description: Some(
                        "DC Sync Out Unit.",
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
                    name: "dtlc",
                    description: Some(
                        "DC Time loop control assigned to PDI.",
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
                    name: "ldcm",
                    description: Some(
                        "Link detection and configuration by MI.",
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
            name: "UserRamByte5",
            extends: None,
            description: Some(
                "User Ram Byte 5.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mcpp",
                    description: Some(
                        "MI control by PDI possible.",
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
                    name: "ats",
                    description: Some(
                        "Automatic TX shift.",
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
                    name: "eeu",
                    description: Some(
                        "EEPROM emulation by µController.",
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
                    name: "ddior",
                    description: Some(
                        "Disable Digital I/O register (0x0F00:0x0F03).",
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
            name: "UserRamByte6",
            extends: None,
            description: Some(
                "User Ram Byte 6.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reledor",
                    description: Some(
                        "RUN/ERR LED Override (0x0138:0x0139).",
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
            ],
        },
        FieldSet {
            name: "UserRamByte7",
            extends: None,
            description: Some(
                "User Ram Byte 7.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dcs1d",
                    description: Some(
                        "DC Sync1 disable.",
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
                    name: "dcrt",
                    description: Some(
                        "DC Receive Times (0x0900:0x090F).",
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
                    name: "dcst",
                    description: Some(
                        "DC System Time (0x0910:0x0936).",
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
            name: "UserRamByte8",
            extends: None,
            description: Some(
                "User Ram Byte 8.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dc64",
                    description: Some(
                        "DC 64 bit.",
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
                    name: "pdicec",
                    description: Some(
                        "PDI clears error counter.",
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
                    name: "apdi",
                    description: Some(
                        "Avalon PDI.",
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
                    name: "opdi",
                    description: Some(
                        "OPB PDI.",
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
                    name: "ppdi",
                    description: Some(
                        "PLB PDI.",
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
            name: "UserRamByte9",
            extends: None,
            description: Some(
                "User Ram Byte 9.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "Direct RESET.",
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
            name: "Vid",
            extends: None,
            description: Some(
                "Vendor ID.",
            ),
            bit_size: 64,
            fields: &[
                Field {
                    name: "vid",
                    description: Some(
                        "Vendor ID: &[23-0] Company [31-24] Department NOTE:Test Vendor IDs have [31:28]=0xE.",
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
            name: "WdgCntPdat",
            extends: None,
            description: Some(
                "Watchdog Counter Process Data.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Watchdog Counter Process Data (counting is stopped when 0xFF is reached). Counts if Process Data Watchdog expires.",
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
            name: "WdgCntPdi",
            extends: None,
            description: Some(
                "Watchdog Counter PDI.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Watchdog PDI counter (counting is stopped when 0xFF is reached). Counts if PDI Watchdog expires.",
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
            name: "WdgDiv",
            extends: None,
            description: Some(
                "Watchdog Divider.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Watchdog divider:Number of 25 MHz tics (minus 2) that represent the basic watchdog increment. (Default value is 100µs = 2498).",
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
            name: "WdgStatPdat",
            extends: None,
            description: Some(
                "Watchdog Status Process Data.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "st",
                    description: Some(
                        "Watchdog Status of Process Data (triggered by SyncManagers) 0:Watchdog Process Data expired 1:Watchdog Process Data is active or disabled.",
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
            name: "WdgTimePdat",
            extends: None,
            description: Some(
                "Watchdog Time Process Data.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Watchdog Time Process Data:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog).",
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
            name: "WdgTimePdi",
            extends: None,
            description: Some(
                "Watchdog Time PDI.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Watchdog Time PDI:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog).",
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
    ],
    enums: &[],
};
