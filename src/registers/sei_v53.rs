use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "CmdCmdTable",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "min",
                    description: Some(
                        "command start value.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Min",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "max",
                    description: Some(
                        "command end value.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Max",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msk",
                    description: Some(
                        "command compare bit enable.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Msk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pta",
                    description: Some(
                        "command pointer 0 - 3.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptb",
                    description: Some(
                        "command pointer 4 - 7.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptb",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "CmdLatch",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "tran",
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
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tran",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "Latch configuration.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "time",
                    description: Some(
                        "Latch time.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Time",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sts",
                    description: Some(
                        "Latch status.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdLatchSts",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Ctrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "engine_ctrl",
                    description: Some(
                        "Engine control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_ptr_cfg",
                    description: Some(
                        "Pointer configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EnginePtrCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_wdg_cfg",
                    description: Some(
                        "Watch dog configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineWdgCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_exe_sta",
                    description: Some(
                        "Execution status.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineExeSta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_exe_ptr",
                    description: Some(
                        "Execution pointer.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineExePtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_exe_inst",
                    description: Some(
                        "Execution instruction.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineExeInst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "engine_wdg_sta",
                    description: Some(
                        "Watch dog status.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EngineWdgSta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_ctrl",
                    description: Some(
                        "Transceiver control register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_type_cfg",
                    description: Some(
                        "Transceiver configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrTypeCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_baud_cfg",
                    description: Some(
                        "Transceiver baud rate register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrBaudCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_data_cfg",
                    description: Some(
                        "Transceiver data timing configuration.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrDataCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_clk_cfg",
                    description: Some(
                        "Transceiver clock timing configuration.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrClkCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_pin",
                    description: Some(
                        "Transceiver pin status.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrPin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xcvr_state",
                    description: Some(
                        "FSM of asynchronous.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "XcvrState",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_in_cfg",
                    description: Some(
                        "Trigger input configuration.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgInCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_sw",
                    description: Some(
                        "Software trigger.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_prd_cfg",
                    description: Some(
                        "Period trigger configuration.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgPrdCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_prd",
                    description: Some(
                        "Trigger period.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgPrd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_out_cfg",
                    description: Some(
                        "Trigger output configuration.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgOutCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_prd_sts",
                    description: Some(
                        "Period trigger status.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgPrdSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_prd_cnt",
                    description: Some(
                        "Period trigger counter.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgPrdCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_table_cmd",
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
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgTableCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trg_table_time",
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
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrgTableTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_mode",
                    description: Some(
                        "command register mode.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_idx",
                    description: Some(
                        "command register configuration.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdIdx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_cmd",
                    description: Some(
                        "command.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_set",
                    description: Some(
                        "command bit set register.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_clr",
                    description: Some(
                        "command bit clear register.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdClr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_inv",
                    description: Some(
                        "command bit invert register.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdInv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_in",
                    description: Some(
                        "Commad input.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_out",
                    description: Some(
                        "Command output.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_sts",
                    description: Some(
                        "Command status.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_cmd_table",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "CmdCmdTable",
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_latch",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "CmdLatch",
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_en",
                    description: Some(
                        "Sample selection register.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_cfg",
                    description: Some(
                        "Sample configuration.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_dat",
                    description: Some(
                        "Sample data.",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpDat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_pos",
                    description: Some(
                        "Sample override position.",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_rev",
                    description: Some(
                        "Sample override revolution.",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_spd",
                    description: Some(
                        "Sample override speed.",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpSpd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_acc",
                    description: Some(
                        "Sample override accelerate.",
                    ),
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpAcc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_en",
                    description: Some(
                        "Update configuration.",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_cfg",
                    description: Some(
                        "Update configuration.",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_dat",
                    description: Some(
                        "Update data.",
                    ),
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdDat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_time",
                    description: Some(
                        "Update overide time.",
                    ),
                    array: None,
                    byte_offset: 0x2ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_pos",
                    description: Some(
                        "Update override position.",
                    ),
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_rev",
                    description: Some(
                        "Update override revolution.",
                    ),
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_spd",
                    description: Some(
                        "Update override speed.",
                    ),
                    array: None,
                    byte_offset: 0x2b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdSpd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_acc",
                    description: Some(
                        "Update override accelerate.",
                    ),
                    array: None,
                    byte_offset: 0x2bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdAcc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_val",
                    description: Some(
                        "Sample valid.",
                    ),
                    array: None,
                    byte_offset: 0x2c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpVal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_smp_sts",
                    description: Some(
                        "Sample status.",
                    ),
                    array: None,
                    byte_offset: 0x2c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSmpSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_time_in",
                    description: Some(
                        "input time.",
                    ),
                    array: None,
                    byte_offset: 0x2cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosTimeIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_pos_in",
                    description: Some(
                        "Input position.",
                    ),
                    array: None,
                    byte_offset: 0x2d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosPosIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_rev_in",
                    description: Some(
                        "Input revolution.",
                    ),
                    array: None,
                    byte_offset: 0x2d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosRevIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_spd_in",
                    description: Some(
                        "Input speed.",
                    ),
                    array: None,
                    byte_offset: 0x2d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosSpdIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_acc_in",
                    description: Some(
                        "Input accelerate.",
                    ),
                    array: None,
                    byte_offset: 0x2dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosAccIn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_upd_sts",
                    description: Some(
                        "Update status.",
                    ),
                    array: None,
                    byte_offset: 0x2e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosUpdSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_int_en",
                    description: Some(
                        "Interrupt Enable.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_int_flag",
                    description: Some(
                        "Interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqIntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_int_sts",
                    description: Some(
                        "Interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqIntSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_pointer0",
                    description: Some(
                        "Match pointer 0.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqPointer0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_pointer1",
                    description: Some(
                        "Match pointer 1.",
                    ),
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqPointer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_instr0",
                    description: Some(
                        "Match instruction 0.",
                    ),
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqInstr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_instr1",
                    description: Some(
                        "Match instruction 1.",
                    ),
                    array: None,
                    byte_offset: 0x31c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqInstr1",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Dat",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "mode",
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
                                "Mode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idx",
                    description: Some(
                        "Data register bit index.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gold",
                    description: Some(
                        "Gold data for data check.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcinit",
                    description: Some(
                        "CRC calculation initial vector.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcinit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcpoly",
                    description: Some(
                        "CRC calculation polynomial.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcpoly",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "Data value.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "set",
                    description: Some(
                        "Data bit set.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Set",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clr",
                    description: Some(
                        "Data bit clear.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inv",
                    description: Some(
                        "Data bit invert.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_",
                    description: Some(
                        "Data input.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out",
                    description: Some(
                        "Data output.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sts",
                    description: Some(
                        "Data status.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DatSts",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Sei",
            extends: None,
            description: Some(
                "SEI.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1024,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ctrl",
                        },
                    ),
                },
                BlockItem {
                    name: "instr",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 64,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x3400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Instr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dat",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 10,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x3800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Dat",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some(
                "Latch configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delay",
                    description: Some(
                        "Delay in system clock cycle, for state transition.",
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
                    name: "select",
                    description: Some(
                        "Output select 0: state0-state1 1: state1-state2 2: state2-state3 3: state3-state0.",
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
                    name: "en",
                    description: Some(
                        "Enable latch 0: disable 1: enable.",
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
            name: "Clr",
            extends: None,
            description: Some(
                "Data bit clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_clr",
                    description: Some(
                        "DATA bit clear.",
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
            name: "CmdClr",
            extends: None,
            description: Some(
                "command bit clear register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_clr",
                    description: Some(
                        "DATA bit clear.",
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
            name: "CmdCmd",
            extends: None,
            description: Some(
                "command.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA.",
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
            name: "CmdIdx",
            extends: None,
            description: Some(
                "command register configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "min_bit",
                    description: Some(
                        "Lowest bit index.",
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
                    name: "max_bit",
                    description: Some(
                        "Highest bit index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "first_bit",
                    description: Some(
                        "First bit index for tranceive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "last_bit",
                    description: Some(
                        "Last bit index for tranceive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CmdIn",
            extends: None,
            description: Some(
                "Commad input.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_in",
                    description: Some(
                        "Commad input.",
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
            name: "CmdInv",
            extends: None,
            description: Some(
                "command bit invert register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_tgl",
                    description: Some(
                        "DATA bit toggle.",
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
            name: "CmdLatchSts",
            extends: None,
            description: Some(
                "Latch status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lat_cnt",
                    description: Some(
                        "Latch counter.",
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
                    name: "state",
                    description: Some(
                        "State.",
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
            ],
        },
        FieldSet {
            name: "CmdMode",
            extends: None,
            description: Some(
                "command register mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Data mode(CMD register only support data mode) 0: data mode 1: check mode 2: CRC mode.",
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
                    name: "rewind",
                    description: Some(
                        "Write 1 to rewind read/write pointer, this is a self clear bit.",
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
                    name: "signed",
                    description: Some(
                        "Signed 0: unsigned value 1: signed value.",
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
                    name: "border",
                    description: Some(
                        "bit order 0: LSB first 1: MSB first.",
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
                    name: "worder",
                    description: Some(
                        "word order 0: sample as bit order 1: different from bit order.",
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
                    name: "wlen",
                    description: Some(
                        "word length 0: 1 bit 1: 2 bit ... 31: 32 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CmdOut",
            extends: None,
            description: Some(
                "Command output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_out",
                    description: Some(
                        "Command output.",
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
            name: "CmdSet",
            extends: None,
            description: Some(
                "command bit set register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_set",
                    description: Some(
                        "DATA bit set.",
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
            name: "CmdSts",
            extends: None,
            description: Some(
                "Command status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bit_idx",
                    description: Some(
                        "Bit index.",
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
                    name: "word_cnt",
                    description: Some(
                        "Word counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "word_idx",
                    description: Some(
                        "Word index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crcinit",
            extends: None,
            description: Some(
                "CRC calculation initial vector.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_init",
                    description: Some(
                        "CRC initial value.",
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
            name: "Crcpoly",
            extends: None,
            description: Some(
                "CRC calculation polynomial.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crc_poly",
                    description: Some(
                        "CRC polymonial.",
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
            name: "DatSts",
            extends: None,
            description: Some(
                "Data status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bit_idx",
                    description: Some(
                        "Bit index.",
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
                    name: "word_cnt",
                    description: Some(
                        "Word counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "word_idx",
                    description: Some(
                        "Word index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crc_idx",
                    description: Some(
                        "CRC index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Data",
            extends: None,
            description: Some(
                "Data value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "DATA.",
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
            name: "EngineCtrl",
            extends: None,
            description: Some(
                "Engine control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Enable 0: disable 1: enable.",
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
                    name: "rewind",
                    description: Some(
                        "Rewind execution pointer 0: run 1: clean status and rewind.",
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
                    name: "except",
                    description: Some(
                        "Explain timout as exception 0: when timeout, pointer move to next instruction 1: when timeout, pointer jump to timeout vector.",
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
                    name: "arming",
                    description: Some(
                        "Wait for trigger before excuting 0: Execute on enable 1: Wait trigger before exection after enabled.",
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
                    name: "watch",
                    description: Some(
                        "Enable watch dog 0: Watch dog disabled 1: Watch dog enabled.",
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
            name: "EngineExeInst",
            extends: None,
            description: Some(
                "Execution instruction.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inst",
                    description: Some(
                        "Current instruction.",
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
            name: "EngineExePtr",
            extends: None,
            description: Some(
                "Execution pointer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pointer",
                    description: Some(
                        "Current program pointer.",
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
                    name: "bit_cnt",
                    description: Some(
                        "Bit count in send and receive instruction execution.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "halt_cnt",
                    description: Some(
                        "Halt count in halt instrution.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EngineExeSta",
            extends: None,
            description: Some(
                "Execution status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stall",
                    description: Some(
                        "Program finished 0: Program is executing 1: Program finished.",
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
                    name: "expire",
                    description: Some(
                        "Watchdog timer expired 0: Not expired 1: Expired.",
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
                    name: "armed",
                    description: Some(
                        "Waiting for trigger for execution 0: Not in waiting status 1: In waiting status.",
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
                    name: "trigered",
                    description: Some(
                        "Execution has been triggered 0: Execution not triggered 1: Execution triggered.",
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
            name: "EnginePtrCfg",
            extends: None,
            description: Some(
                "Pointer configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pointer_init",
                    description: Some(
                        "Initial execute pointer.",
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
                    name: "pointer_wdog",
                    description: Some(
                        "Pointer to the instruction that the program starts executing after the instruction timeout. The timeout is WDOG_TIME.",
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
                    name: "dat_base",
                    description: Some(
                        "Bias for data register access, if calculated index bigger than 32, index will wrap around 0: real data index 1: access index is 1 greater than instruction address 2: access index is 2 greater than instruction address ... 31: access index is 31 greater than instruction address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dat_cdm",
                    description: Some(
                        "Select DATA register to receive CDM bit in BiSSC slave mode 0: ignore 1: command 2: data register 2 3: data register 3 ... 29:data register 29 30: value 0 when send, ignore in receive 31: value1 when send, ignore in receive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EngineWdgCfg",
            extends: None,
            description: Some(
                "Watch dog configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdog_time",
                    description: Some(
                        "Time out count for each instruction, counter in bit time.",
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
            name: "EngineWdgSta",
            extends: None,
            description: Some(
                "Watch dog status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdog_cnt",
                    description: Some(
                        "Current watch dog counter value.",
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
            name: "Gold",
            extends: None,
            description: Some(
                "Gold data for data check.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gold_value",
                    description: Some(
                        "Gold value for check mode.",
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
            name: "Idx",
            extends: None,
            description: Some(
                "Data register bit index.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "min_bit",
                    description: Some(
                        "Lowest bit index.",
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
                    name: "max_bit",
                    description: Some(
                        "Highest bit index.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "first_bit",
                    description: Some(
                        "First bit index for tranceive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "last_bit",
                    description: Some(
                        "Last bit index for tranceive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "In",
            extends: None,
            description: Some(
                "Data input.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_in",
                    description: Some(
                        "Data input.",
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
            name: "Instr",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opr",
                    description: Some(
                        "[1] When OP is 0, this area is the halt time in baudrate, 0 represents infinite time. [2] When OP is 1, this area is the the pointer to the command table. OPR[4]=1, OPR[3:0] value is CMD_TABLE instruct pointer; OPR[4]=0, OPR[3:0]=0 is INIT_POINTER; OPR[4]=0, OPR[3:0]=1 is WDG_POINTER. [3] When OP is 2-7, this area is the data length as fellow: 0: 1 bit 1: 2 bit ... 31: 32 bit.",
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
                    name: "dat",
                    description: Some(
                        "DATA register 0: ignore data 1: command 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crc",
                    description: Some(
                        "CRC register 0: don't calculate CRC 1: do not set this value 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ck",
                    description: Some(
                        "clock 0: low 1: rise-fall 2: fall-rise 3: high.",
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
                    name: "op",
                    description: Some(
                        "operation 0: halt 1: jump 2: send with timeout check 3: send without timout check 4: wait with timeout check 5: wait without timout check 6: receive with timeout check 7: receive without timout check.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Inv",
            extends: None,
            description: Some(
                "Data bit invert.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_inv",
                    description: Some(
                        "DATA bit toggle.",
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
            name: "IrqInstr0",
            extends: None,
            description: Some(
                "Match instruction 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instr",
                    description: Some(
                        "Match instruction 0.",
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
            name: "IrqInstr1",
            extends: None,
            description: Some(
                "Match instruction 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "instr",
                    description: Some(
                        "Match instruction 1.",
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
            name: "IrqIntEn",
            extends: None,
            description: Some(
                "Interrupt Enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stall",
                    description: Some(
                        "Stall.",
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
                    name: "except",
                    description: Some(
                        "Exception.",
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
                    name: "wdog",
                    description: Some(
                        "Watch dog.",
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
                    name: "ptr0_st",
                    description: Some(
                        "Pointer 0 start.",
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
                    name: "ptr1_st",
                    description: Some(
                        "Pointer 1 start.",
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
                    name: "instr0_st",
                    description: Some(
                        "Instruction 0 start.",
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
                    name: "instr1_st",
                    description: Some(
                        "Instruction 1 start.",
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
                    name: "ptr0_end",
                    description: Some(
                        "Pointer 0 end.",
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
                    name: "ptr1_end",
                    description: Some(
                        "Pointer 1 end.",
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
                    name: "instr0_end",
                    description: Some(
                        "Instruction 0 end.",
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
                    name: "instr1_end",
                    description: Some(
                        "Instruction 1 end.",
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
                    name: "trx_err",
                    description: Some(
                        "Transfer error.",
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
                    name: "timeout",
                    description: Some(
                        "Timeout.",
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
                    name: "latch0",
                    description: Some(
                        "Latch0.",
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
                    name: "latch1",
                    description: Some(
                        "Latch1.",
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
                    name: "latch2",
                    description: Some(
                        "Latch2.",
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
                    name: "latch3",
                    description: Some(
                        "Latch3.",
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
                    name: "smp_err",
                    description: Some(
                        "Sample error.",
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
                    name: "triger0",
                    description: Some(
                        "Trigger0.",
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
                    name: "triger1",
                    description: Some(
                        "Trigger1.",
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
                    name: "triger2",
                    description: Some(
                        "Trigger2.",
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
                    name: "triger3",
                    description: Some(
                        "Trigger3.",
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
                    name: "trg_err0",
                    description: Some(
                        "Trigger0 failed.",
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
                    name: "trg_err1",
                    description: Some(
                        "Trigger1 failed.",
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
                    name: "trg_err2",
                    description: Some(
                        "Trigger2 failed.",
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
                    name: "trg_err3",
                    description: Some(
                        "Trigger3 failed.",
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
            name: "IrqIntFlag",
            extends: None,
            description: Some(
                "Interrupt flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stall",
                    description: Some(
                        "Stall.",
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
                    name: "except",
                    description: Some(
                        "Exception.",
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
                    name: "wdog",
                    description: Some(
                        "Watch dog.",
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
                    name: "ptr0_st",
                    description: Some(
                        "Pointer 0 start.",
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
                    name: "ptr1_st",
                    description: Some(
                        "Pointer 1 start.",
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
                    name: "instr0_st",
                    description: Some(
                        "Instruction 0 start.",
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
                    name: "instr1_st",
                    description: Some(
                        "Instruction 1 start.",
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
                    name: "ptr0_end",
                    description: Some(
                        "Pointer 0 end.",
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
                    name: "ptr1_end",
                    description: Some(
                        "Pointer 1 end.",
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
                    name: "instr0_end",
                    description: Some(
                        "Instruction 0 end.",
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
                    name: "instr1_end",
                    description: Some(
                        "Instruction 1 end.",
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
                    name: "trx_err",
                    description: Some(
                        "Transfer error.",
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
                    name: "timeout",
                    description: Some(
                        "Timeout.",
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
                    name: "latch0",
                    description: Some(
                        "Latch0.",
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
                    name: "latch1",
                    description: Some(
                        "Latch1.",
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
                    name: "latch2",
                    description: Some(
                        "Latch2.",
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
                    name: "latch3",
                    description: Some(
                        "Latch3.",
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
                    name: "smp_err",
                    description: Some(
                        "Sample error.",
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
                    name: "triger0",
                    description: Some(
                        "Trigger0.",
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
                    name: "triger1",
                    description: Some(
                        "Trigger1.",
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
                    name: "triger2",
                    description: Some(
                        "Trigger2.",
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
                    name: "triger3",
                    description: Some(
                        "Trigger3.",
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
                    name: "trg_err0",
                    description: Some(
                        "Trigger0 failed.",
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
                    name: "trg_err1",
                    description: Some(
                        "Trigger1 failed.",
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
                    name: "trg_err2",
                    description: Some(
                        "Trigger2 failed.",
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
                    name: "trg_err3",
                    description: Some(
                        "Trigger3 failed.",
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
            name: "IrqIntSts",
            extends: None,
            description: Some(
                "Interrupt status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stall",
                    description: Some(
                        "Stall.",
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
                    name: "except",
                    description: Some(
                        "Exception.",
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
                    name: "wdog",
                    description: Some(
                        "Watch dog.",
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
                    name: "ptr0_st",
                    description: Some(
                        "Pointer 0 start.",
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
                    name: "ptr1_st",
                    description: Some(
                        "Pointer 1 start.",
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
                    name: "instr0_st",
                    description: Some(
                        "Instruction 0 start.",
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
                    name: "instr1_st",
                    description: Some(
                        "Instruction 1 start.",
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
                    name: "ptr0_end",
                    description: Some(
                        "Pointer 0 end.",
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
                    name: "ptr1_end",
                    description: Some(
                        "Pointer 1 end.",
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
                    name: "instr0_end",
                    description: Some(
                        "Instruction 0 end.",
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
                    name: "instr1_end",
                    description: Some(
                        "Instruction 1 end.",
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
                    name: "trx_err",
                    description: Some(
                        "Transfer error.",
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
                    name: "timeout",
                    description: Some(
                        "Timeout.",
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
                    name: "latch0",
                    description: Some(
                        "Latch0.",
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
                    name: "latch1",
                    description: Some(
                        "Latch1.",
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
                    name: "latch2",
                    description: Some(
                        "Latch2.",
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
                    name: "latch3",
                    description: Some(
                        "Latch3.",
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
                    name: "smp_err",
                    description: Some(
                        "Sample error.",
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
                    name: "triger0",
                    description: Some(
                        "Trigger0.",
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
                    name: "triger1",
                    description: Some(
                        "Trigger1.",
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
                    name: "triger2",
                    description: Some(
                        "Trigger2.",
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
                    name: "triger3",
                    description: Some(
                        "Trigger3.",
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
                    name: "trg_err0",
                    description: Some(
                        "Trigger0 failed.",
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
                    name: "trg_err1",
                    description: Some(
                        "Trigger1 failed.",
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
                    name: "trg_err2",
                    description: Some(
                        "Trigger2 failed.",
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
                    name: "trg_err3",
                    description: Some(
                        "Trigger3 failed.",
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
            name: "IrqPointer0",
            extends: None,
            description: Some(
                "Match pointer 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pointer",
                    description: Some(
                        "Match pointer 0.",
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
            name: "IrqPointer1",
            extends: None,
            description: Some(
                "Match pointer 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pointer",
                    description: Some(
                        "Match pointer 1.",
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
            name: "Max",
            extends: None,
            description: Some(
                "command end value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd_max",
                    description: Some(
                        "maximum command value.",
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
            name: "Min",
            extends: None,
            description: Some(
                "command start value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd_min",
                    description: Some(
                        "minimum command value.",
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
            name: "Mode",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Data mode 0: data mode 1: check mode 2: CRC mode.",
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
                    name: "rewind",
                    description: Some(
                        "Write 1 to rewind read/write pointer, this is a self clear bit.",
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
                    name: "signed",
                    description: Some(
                        "Signed 0: unsigned value 1: signed value.",
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
                    name: "border",
                    description: Some(
                        "bit order 0: LSB first 1: MSB first.",
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
                    name: "worder",
                    description: Some(
                        "word order 0: sample as bit order 1: different from bit order.",
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
                    name: "crc_inv",
                    description: Some(
                        "CRC invert 0: use CRC 1: use inverted CRC.",
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
                    name: "crc_shift",
                    description: Some(
                        "CRC shift mode, this mode is used to perform repeat code check 0: CRC 1: shift mode.",
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
                    name: "wlen",
                    description: Some(
                        "word length 0: 1 bit 1: 2 bit ... 31: 32 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crc_len",
                    description: Some(
                        "CRC length 0: 1 bit 1: 2 bit ... 31: 32 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Msk",
            extends: None,
            description: Some(
                "command compare bit enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd_mask",
                    description: Some(
                        "compare mask.",
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
            name: "Out",
            extends: None,
            description: Some(
                "Data output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_out",
                    description: Some(
                        "Data output.",
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
            name: "PosAccIn",
            extends: None,
            description: Some(
                "Input accelerate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "Input accelerate.",
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
            name: "PosPosIn",
            extends: None,
            description: Some(
                "Input position.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "Input position.",
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
            name: "PosRevIn",
            extends: None,
            description: Some(
                "Input revolution.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rev",
                    description: Some(
                        "Input revolution.",
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
            name: "PosSmpAcc",
            extends: None,
            description: Some(
                "Sample override accelerate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "Sample override accelerate.",
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
            name: "PosSmpCfg",
            extends: None,
            description: Some(
                "Sample configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "window",
                    description: Some(
                        "Sample window, in clock cycle.",
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
                    name: "lat_sel",
                    description: Some(
                        "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3.",
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
                    name: "once",
                    description: Some(
                        "Sample one time 0: Sample during windows time 1: Close sample window after first sample.",
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
            name: "PosSmpDat",
            extends: None,
            description: Some(
                "Sample data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dat_sel",
                    description: Some(
                        "Data register sampled, each bit represent a data register.",
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
            name: "PosSmpEn",
            extends: None,
            description: Some(
                "Sample selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_sel",
                    description: Some(
                        "Data register for position transfer.",
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
                    name: "pos_en",
                    description: Some(
                        "Position include position.",
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
                    name: "rev_sel",
                    description: Some(
                        "Data register for revolution transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rev_en",
                    description: Some(
                        "Position include revolution.",
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
                    name: "spd_sel",
                    description: Some(
                        "Data register for speed transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spd_en",
                    description: Some(
                        "Position include speed.",
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
                    name: "acc_sel",
                    description: Some(
                        "Data register for acceleration transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acc_en",
                    description: Some(
                        "Position include acceleration.",
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
            name: "PosSmpPos",
            extends: None,
            description: Some(
                "Sample override position.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "Sample override position.",
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
            name: "PosSmpRev",
            extends: None,
            description: Some(
                "Sample override revolution.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rev",
                    description: Some(
                        "Sample override revolution.",
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
            name: "PosSmpSpd",
            extends: None,
            description: Some(
                "Sample override speed.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spd",
                    description: Some(
                        "Sample override speed.",
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
            name: "PosSmpSts",
            extends: None,
            description: Some(
                "Sample status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "win_cnt",
                    description: Some(
                        "Sample window counter.",
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
                    name: "occur",
                    description: Some(
                        "Sample occured 0: Sample not happened 1: Sample occured.",
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
            name: "PosSmpVal",
            extends: None,
            description: Some(
                "Sample valid.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "Position include position.",
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
                    name: "rev",
                    description: Some(
                        "Position include revolution.",
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
                    name: "spd",
                    description: Some(
                        "Position include speed.",
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
                    name: "acc",
                    description: Some(
                        "Position include acceleration.",
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
            name: "PosSpdIn",
            extends: None,
            description: Some(
                "Input speed.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spd",
                    description: Some(
                        "Input speed.",
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
            name: "PosTimeIn",
            extends: None,
            description: Some(
                "input time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "input time.",
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
            name: "PosUpdAcc",
            extends: None,
            description: Some(
                "Update override accelerate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acc",
                    description: Some(
                        "Update override accelerate.",
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
            name: "PosUpdCfg",
            extends: None,
            description: Some(
                "Update configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lat_sel",
                    description: Some(
                        "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3.",
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
                    name: "onerr",
                    description: Some(
                        "Sample one time 0: Sample during windows time 1: Close sample window after first sample.",
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
                    name: "time_ovrd",
                    description: Some(
                        "Use override time 0: use time sample from motor group 1: use override time.",
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
            name: "PosUpdDat",
            extends: None,
            description: Some(
                "Update data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dat_sel",
                    description: Some(
                        "Data register sampled, each bit represent a data register.",
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
            name: "PosUpdEn",
            extends: None,
            description: Some(
                "Update configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_sel",
                    description: Some(
                        "Data register for position transfer.",
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
                    name: "pos_en",
                    description: Some(
                        "Position include position.",
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
                    name: "rev_sel",
                    description: Some(
                        "Data register for revolution transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rev_en",
                    description: Some(
                        "Position include revolution.",
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
                    name: "spd_sel",
                    description: Some(
                        "Data register for speed transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spd_en",
                    description: Some(
                        "Position include speed.",
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
                    name: "acc_sel",
                    description: Some(
                        "Data register for acceleration transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acc_en",
                    description: Some(
                        "Position include acceleration.",
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
            name: "PosUpdPos",
            extends: None,
            description: Some(
                "Update override position.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos",
                    description: Some(
                        "Update override position.",
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
            name: "PosUpdRev",
            extends: None,
            description: Some(
                "Update override revolution.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rev",
                    description: Some(
                        "Update override revolution.",
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
            name: "PosUpdSpd",
            extends: None,
            description: Some(
                "Update override speed.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spd",
                    description: Some(
                        "Update override speed.",
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
            name: "PosUpdSts",
            extends: None,
            description: Some(
                "Update status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "upd_err",
                    description: Some(
                        "Update error 0: data receive normally 1: data receive error.",
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
            name: "PosUpdTime",
            extends: None,
            description: Some(
                "Update overide time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Update override time.",
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
            name: "Pta",
            extends: None,
            description: Some(
                "command pointer 0 - 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr0",
                    description: Some(
                        "pointer0.",
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
                    name: "ptr1",
                    description: Some(
                        "pointer1.",
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
                    name: "ptr2",
                    description: Some(
                        "pointer2.",
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
                    name: "ptr3",
                    description: Some(
                        "pointer3.",
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
            name: "Ptb",
            extends: None,
            description: Some(
                "command pointer 4 - 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr4",
                    description: Some(
                        "pointer4.",
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
                    name: "ptr5",
                    description: Some(
                        "pointer5.",
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
                    name: "ptr6",
                    description: Some(
                        "pointer6.",
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
                    name: "ptr7",
                    description: Some(
                        "pointer7.",
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
            name: "Set",
            extends: None,
            description: Some(
                "Data bit set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_set",
                    description: Some(
                        "DATA bit set.",
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
            name: "Time",
            extends: None,
            description: Some(
                "Latch time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lat_time",
                    description: Some(
                        "Latch time.",
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
            name: "Tran",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ov_ptr",
                    description: Some(
                        "override pointer check.",
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
                    name: "ov_clk",
                    description: Some(
                        "override clock check.",
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
                    name: "ov_txd",
                    description: Some(
                        "override TX data check.",
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
                    name: "ov_tm",
                    description: Some(
                        "override timeout check.",
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
                    name: "cfg_ptr",
                    description: Some(
                        "pointer 0: match 1: not match 2:entry 3:leave.",
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
                    name: "cfg_clk",
                    description: Some(
                        "clock 0: high 1: low 2: rise 3: fall.",
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
                    name: "cfg_txd",
                    description: Some(
                        "data send 0: high 1: low 2: rise 3: fall.",
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
                    name: "cfg_tm",
                    description: Some(
                        "timeout 0: high 1: low 2: rise 3: fall.",
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
                    name: "pointer",
                    description: Some(
                        "pointer.",
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
            name: "TrgInCfg",
            extends: None,
            description: Some(
                "Trigger input configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "in0_sel",
                    description: Some(
                        "Trigger 0 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7.",
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
                    name: "in0_en",
                    description: Some(
                        "Enable trigger 0 0: disable trigger 1 1: enable trigger 1.",
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
                    name: "in1_sel",
                    description: Some(
                        "Trigger 1 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7.",
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
                    name: "in1_en",
                    description: Some(
                        "Enable trigger 1 0: disable trigger 1 1: enable trigger 1.",
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
                    name: "sync_sel",
                    description: Some(
                        "Synchronize sigal selection (tigger 2) 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7.",
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
                    name: "prd_en",
                    description: Some(
                        "Enable period trigger (tigger 2) 0: periodical trigger disabled 1: periodical trigger enabled.",
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
            ],
        },
        FieldSet {
            name: "TrgOutCfg",
            extends: None,
            description: Some(
                "Trigger output configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_sel",
                    description: Some(
                        "Trigger 0 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7.",
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
                    name: "out0_en",
                    description: Some(
                        "Enable trigger 0 0: disable trigger 1 1: enable trigger 1.",
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
                    name: "out1_sel",
                    description: Some(
                        "Trigger 1 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7.",
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
                    name: "out1_en",
                    description: Some(
                        "Enable trigger 1 0: disable trigger 1 1: enable trigger 1.",
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
                    name: "out2_sel",
                    description: Some(
                        "Trigger 2 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7.",
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
                    name: "out2_en",
                    description: Some(
                        "Enable trigger 2 0: disable trigger 2 1: enable trigger 2.",
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
                    name: "out3_sel",
                    description: Some(
                        "Trigger 3 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7.",
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
                    name: "out3_en",
                    description: Some(
                        "Enable trigger 3 0: disable trigger 3 1: enable trigger 3.",
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
            name: "TrgPrd",
            extends: None,
            description: Some(
                "Trigger period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "period",
                    description: Some(
                        "Trigger period.",
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
            name: "TrgPrdCfg",
            extends: None,
            description: Some(
                "Period trigger configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sync",
                    description: Some(
                        "Synchronous 0: Not synchronous 1: Synchronous every trigger source.",
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
                    name: "arming",
                    description: Some(
                        "Wait for trigger synchronous before trigger 0: Trigger directly 1: Wait trigger source before period trigger.",
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
            name: "TrgPrdCnt",
            extends: None,
            description: Some(
                "Period trigger counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "period_cnt",
                    description: Some(
                        "Trigger period counter.",
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
            name: "TrgPrdSts",
            extends: None,
            description: Some(
                "Period trigger status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "armed",
                    description: Some(
                        "Waiting for trigger 0: Not in waiting status 1: In waiting status.",
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
                    name: "trigered",
                    description: Some(
                        "Period has been triggered 0: Not triggered 1: Triggered.",
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
            name: "TrgSw",
            extends: None,
            description: Some(
                "Software trigger.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soft",
                    description: Some(
                        "Software trigger (tigger 3). this bit is self-clear 0: trigger source disabled 1: trigger source enabled.",
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
            name: "TrgTableCmd",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd_trigger0",
                    description: Some(
                        "Trigger command.",
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
            name: "TrgTableTime",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigger0_time",
                    description: Some(
                        "Trigger time.",
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
            name: "XcvrBaudCfg",
            extends: None,
            description: Some(
                "Transceiver baud rate register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baud_div",
                    description: Some(
                        "Baud rate, bit time in system clock cycle.",
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
                    name: "sync_point",
                    description: Some(
                        "Baud synchronous time, minmum bit time.",
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
            name: "XcvrClkCfg",
            extends: None,
            description: Some(
                "Transceiver clock timing configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ck0_point",
                    description: Some(
                        "clock point 0 in system clcok cycle.",
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
                    name: "ck1_point",
                    description: Some(
                        "clock point 1 in system clcok cycle.",
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
            name: "XcvrCtrl",
            extends: None,
            description: Some(
                "Transceiver control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "Tranceiver mode 0: synchronous maaster 1: synchronous slave 2: asynchronous mode 3: asynchronous mode.",
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
                    name: "restart",
                    description: Some(
                        "Restart tranceiver, this is a self clear bit 0: no effect 1: reset tranceiver.",
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
                    name: "par_clr",
                    description: Some(
                        "Clear parity error, this is a self clear bit 0: no effect 1: clear parity error.",
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
                    name: "trismp",
                    description: Some(
                        "Tipple sampe 0: sample 1 time for data transition 1: sample 3 times in receive and result in 2oo3.",
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
            name: "XcvrDataCfg",
            extends: None,
            description: Some(
                "Transceiver data timing configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxd_point",
                    description: Some(
                        "data receive point in system clcok cycle.",
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
                    name: "txd_point",
                    description: Some(
                        "data transmit point in system clcok cycle.",
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
            name: "XcvrPin",
            extends: None,
            description: Some(
                "Transceiver pin status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do_tx",
                    description: Some(
                        "TX output 0: data 0 1: data 1.",
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
                    name: "di_tx",
                    description: Some(
                        "TX state 0: data 0 1: data 1.",
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
                    name: "oe_tx",
                    description: Some(
                        "TX drive state 0: input 1: output.",
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
                    name: "do_de",
                    description: Some(
                        "DE output 0: data 0 1: data 1.",
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
                    name: "di_de",
                    description: Some(
                        "DE state 0: data 0 1: data 1.",
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
                    name: "oe_de",
                    description: Some(
                        "DE drive state 0: input 1: output.",
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
                    name: "do_rx",
                    description: Some(
                        "RX output 0: data 0 1: data 1.",
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
                    name: "di_rx",
                    description: Some(
                        "RX state 0: data 0 1: data 1.",
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
                    name: "oe_rx",
                    description: Some(
                        "RX drive state 0: input 1: output.",
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
                    name: "do_ck",
                    description: Some(
                        "CK output 0: data 0 1: data 1.",
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
                    name: "di_ck",
                    description: Some(
                        "CK state 0: data 0 1: data 1.",
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
                    name: "oe_ck",
                    description: Some(
                        "CK drive state 0: input 1: output.",
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
            ],
        },
        FieldSet {
            name: "XcvrState",
            extends: None,
            description: Some(
                "FSM of asynchronous.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "send_state",
                    description: Some(
                        "FSM of asynchronous transmit.",
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
                    name: "recv_state",
                    description: Some(
                        "FSM of asynchronous receive.",
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
            ],
        },
        FieldSet {
            name: "XcvrTypeCfg",
            extends: None,
            description: Some(
                "Transceiver configuration register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ck_idlev",
                    description: Some(
                        "Idle state value of clock line 0: data'0' 1: data'1'.",
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
                    name: "da_idlev",
                    description: Some(
                        "Idle state value of data line 0: data'0' 1: data'1'.",
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
                    name: "ck_idlez",
                    description: Some(
                        "Idle state driver of clock line 0: output 1: high-Z.",
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
                    name: "da_idlez",
                    description: Some(
                        "Idle state driver of data line 0: output 1: high-Z.",
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
                    name: "par_en",
                    description: Some(
                        "enable parity check for asynchronous mode 0: disable 1: enable.",
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
                    name: "par_pol",
                    description: Some(
                        "Polarity of parity for asynchronous mode 0: even 1: odd.",
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
                    name: "data_len",
                    description: Some(
                        "Number of data bit for asynchronous mode 0: 1 bit 1: 2 bit ... 31: 32 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wait_len",
                    description: Some(
                        "Number of extra stop bit for asynchronous mode 0: 1 bit 1: 2 bit ... 255: 256 bit.",
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
    ],
    enums: &[],
};
