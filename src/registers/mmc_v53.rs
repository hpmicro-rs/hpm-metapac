use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Br",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "br_ctrl",
                    description: Some(
                        "Prediction Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_timeoff",
                    description: Some(
                        "Prediction Timing Offset Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTimeoff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_period",
                    description: Some(
                        "Prediction Triggering Period Offset Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_f_time",
                    description: Some(
                        "Prediction Triggering First Offset Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgFTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_st",
                    description: Some(
                        "Prediction Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_pos_cfg",
                    description: Some(
                        "Prediction Configuration postion trigger cfg.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgPosCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_pos_thr",
                    description: Some(
                        "Prediction Configuration postion threshold.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgPosThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_rev_thr",
                    description: Some(
                        "Prediction Configuration revolutiom threshold.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgRevThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_speed_cfg",
                    description: Some(
                        "Prediction Configuration speed trigger cfg.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgSpeedCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_trg_speed_thr",
                    description: Some(
                        "Prediction Configuration speed threshold.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrTrgSpeedThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_pos_time",
                    description: Some(
                        "Initialization timestamp for open-loop mode.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniPosTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_pos",
                    description: Some(
                        "Initialization position for open-loop mode.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_rev",
                    description: Some(
                        "Initialization revolution for open-loop mode.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_speed",
                    description: Some(
                        "Initialization speed for open-loop mode.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_accel",
                    description: Some(
                        "Initialization acceleration for open-loop mode.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniAccel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_delta_pos_time",
                    description: Some(
                        "Initialization timestamp for delta mode in prediction mode.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniDeltaPosTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_delta_pos",
                    description: Some(
                        "Initialization delta position for delta mode in prediction mode.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniDeltaPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_delta_rev",
                    description: Some(
                        "Initialization delta revolution for delta mode in prediction mode.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniDeltaRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_delta_speed",
                    description: Some(
                        "Initialization delta speed for delta mode in prediction mode.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniDeltaSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_ini_delta_accel",
                    description: Some(
                        "Initialization delta acceleration for delta mode in prediction mode.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrIniDeltaAccel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_cur_pos_time",
                    description: Some(
                        "Monitor of the output timestamp.",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCurPosTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_cur_pos",
                    description: Some(
                        "Monitor of the output position.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCurPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_cur_rev",
                    description: Some(
                        "Monitor of the output revolution.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCurRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_cur_speed",
                    description: Some(
                        "Monitor of the output speed.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCurSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "br_cur_accel",
                    description: Some(
                        "Monitor of the output acceleration.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BrCurAccel",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "CoefTrgCfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "err_thr",
                    description: Some(
                        "Tracking Configuration coef trigger cfg&index0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ErrThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "p",
                    description: Some(
                        "Tracking Configuration coef trigger cfg&index0 P.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "P",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "i",
                    description: Some(
                        "Tracking Configuration coef trigger cfg&index0 I.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "I",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "a",
                    description: Some(
                        "Tracking Configuration coef trigger cfg&index0 A.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "A",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "time",
                    description: Some(
                        "Tracking Configuration coef trigger cfg&index0 time.",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
            ],
        },
        Block {
            name: "Mmc",
            extends: None,
            description: Some(
                "MMC0.",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sta",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "sysclk_freq",
                    description: Some(
                        "System Clock Frequency Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SysclkFreq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sysclk_period",
                    description: Some(
                        "System Clock Period Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SysclkPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oosync_theta_thr",
                    description: Some(
                        "Position Out-Of-Sync Threshold Regster.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OosyncThetaThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "discrete_cfg0",
                    description: Some(
                        "Discrete Mode Configuration 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DiscreteCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "discrete_cfg1",
                    description: Some(
                        "Discrete Mode Configuration 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DiscreteCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cont_cfg0",
                    description: Some(
                        "Continuous Mode Configuration 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ContCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_pos_time",
                    description: Some(
                        "The destined timestamp register for position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniPosTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_pos",
                    description: Some(
                        "The destined position register for position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_rev",
                    description: Some(
                        "The destined revolution register for position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_speed",
                    description: Some(
                        "The destined speed register for position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_accel",
                    description: Some(
                        "The destined accelerator register for position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniAccel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_coef_time",
                    description: Some(
                        "The destined timestamp register for coefficients initialization.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniCoefTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_pcoef",
                    description: Some(
                        "The destined coefficient P register for coefficients initialization.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniPcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_icoef",
                    description: Some(
                        "The destined coefficient I register for coefficients initialization.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniIcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_acoef",
                    description: Some(
                        "The destined coefficient A register for coefficients initialization.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniAcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "estm_tim",
                    description: Some(
                        "The timestamp register for internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EstmTim",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "estm_pos",
                    description: Some(
                        "The position register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EstmPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "estm_rev",
                    description: Some(
                        "The revolution register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EstmRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "estm_speed",
                    description: Some(
                        "The speed register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EstmSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "estm_accel",
                    description: Some(
                        "The accelerator register for theinternal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EstmAccel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cur_pcoef",
                    description: Some(
                        "The coefficient P register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CurPcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cur_icoef",
                    description: Some(
                        "The coefficient I register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CurIcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cur_acoef",
                    description: Some(
                        "The coefficient A register for the internal estimation.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CurAcoef",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_delta_pos_time",
                    description: Some(
                        "The destined timestamp register for delta position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniDeltaPosTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_delta_pos",
                    description: Some(
                        "The destined delta position register for delta position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniDeltaPos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_delta_rev",
                    description: Some(
                        "The destined delta revolution register for delta position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniDeltaRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_delta_speed",
                    description: Some(
                        "The destined delta speed register for delta position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniDeltaSpeed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ini_delta_accel",
                    description: Some(
                        "The destined delta accelerator register for delta position initialization.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IniDeltaAccel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_trg_cfg",
                    description: Some(
                        "Tracking Configuration pos trigger cfg.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosTrgCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_trg_pos_thr",
                    description: Some(
                        "Tracking Configuration position threshold.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosTrgPosThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_trg_rev_thr",
                    description: Some(
                        "Tracking Configuration revolution threshold.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosTrgRevThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "speed_trg_cfg",
                    description: Some(
                        "Tracking Configuration speed trigger cfg.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SpeedTrgCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "speed_trg_thr",
                    description: Some(
                        "Tracking Configuration speed threshold.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SpeedTrgThr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "coef_trg_cfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 20,
                            },
                        ),
                    ),
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "CoefTrgCfg",
                        },
                    ),
                },
                BlockItem {
                    name: "br",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Br",
                        },
                    ),
                },
                BlockItem {
                    name: "bk0_timestamp",
                    description: Some(
                        "Monitor of the just received input timestamp for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0Timestamp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0_position",
                    description: Some(
                        "Monitor of the just received input position for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0Position",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0_revolution",
                    description: Some(
                        "Monitor of the just received input revolution for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0Revolution",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0_speed",
                    description: Some(
                        "Monitor of the just received input speed for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0Speed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0_accelerator",
                    description: Some(
                        "Monitor of the just received input acceleration for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0Accelerator",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1_timestamp",
                    description: Some(
                        "Monitor of the previous received input timestamp for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1Timestamp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1_position",
                    description: Some(
                        "Monitor of the previous received input position for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x324,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1Position",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1_revolution",
                    description: Some(
                        "Monitor of the previous received input revolution for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1Revolution",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1_speed",
                    description: Some(
                        "Monitor of the previous received input speed for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x32c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1Speed",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1_accelerator",
                    description: Some(
                        "Monitor of the previous received input acceleration for tracing logic.",
                    ),
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1Accelerator",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "A",
            extends: None,
            description: Some(
                "Tracking Configuration coef trigger cfg&index0 A.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "A0_Coef，fix<32, 19>.",
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
            name: "Bk0Accelerator",
            extends: None,
            description: Some(
                "Monitor of the just received input acceleration for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk0Position",
            extends: None,
            description: Some(
                "Monitor of the just received input position for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk0Revolution",
            extends: None,
            description: Some(
                "Monitor of the just received input revolution for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk0Speed",
            extends: None,
            description: Some(
                "Monitor of the just received input speed for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk0Timestamp",
            extends: None,
            description: Some(
                "Monitor of the just received input timestamp for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk1Accelerator",
            extends: None,
            description: Some(
                "Monitor of the previous received input acceleration for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk1Position",
            extends: None,
            description: Some(
                "Monitor of the previous received input position for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk1Revolution",
            extends: None,
            description: Some(
                "Monitor of the previous received input revolution for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk1Speed",
            extends: None,
            description: Some(
                "Monitor of the previous received input speed for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "Bk1Timestamp",
            extends: None,
            description: Some(
                "Monitor of the previous received input timestamp for tracing logic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrCtrl",
            extends: None,
            description: Some(
                "Prediction Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "br_en",
                    description: Some(
                        "Branch Enable.",
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
                    name: "f_trg_type",
                    description: Some(
                        "1. First trigger by external trigger pin 0. First trigger by the timer When in CR[MANUAL_IO]=1 mode, it is the prediction trigger.",
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
                    name: "nf_trg_type",
                    description: Some(
                        "1. Each non-first trigger by external trigger pin 0. Each non-first trigger by the timer.",
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
                    name: "pred_mode",
                    description: Some(
                        "1:continuously repeat pred, 0:cal the pred based on a definite time-stamp offset, 2:programed one-shot prediction mode.",
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
                    name: "open_loop_mode",
                    description: Some(
                        "1: in open loop mode 0: not in open loop mode.",
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
                    name: "ini_delta_pos_req",
                    description: Some(
                        "1: Command to reload the delta pos. Auto clear 0:.",
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
                    name: "ini_delta_pos_cmd_msk",
                    description: Some(
                        "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position.",
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
                    name: "ini_delta_pos_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_DELTA_POS_DONE.",
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
                    name: "ini_delta_pos_trg_type",
                    description: Some(
                        "0: Time Stamp in the configuration 1: Risedge of In Trg[0] 2: Risedge of In Trg[1] 3: Risedge of out trg[0] 4: Risedge of out trg[1] 5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ini_pos_cmd_msk",
                    description: Some(
                        "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ini_pos_trg_type",
                    description: Some(
                        "0: Time Stamp in the configuration 1: Risedge of In Trg[0] 2: Risedge of In Trg[1] 3: Risedge of out trg[0] 4: Risedge of out trg[1] 5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pos_trg_valid_ie",
                    description: Some(
                        "Interrupt Enable for POS_TRG_VALID.",
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
                    name: "speed_trg_valid_ie",
                    description: Some(
                        "Interrupt Enable for SPEED_TRG_VALID.",
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
            name: "BrCurAccel",
            extends: None,
            description: Some(
                "Monitor of the output acceleration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrCurPos",
            extends: None,
            description: Some(
                "Monitor of the output position.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrCurPosTime",
            extends: None,
            description: Some(
                "Monitor of the output timestamp.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrCurRev",
            extends: None,
            description: Some(
                "Monitor of the output revolution.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrCurSpeed",
            extends: None,
            description: Some(
                "Monitor of the output speed.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "BrIniAccel",
            extends: None,
            description: Some(
                "Initialization acceleration for open-loop mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 19>.",
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
            name: "BrIniDeltaAccel",
            extends: None,
            description: Some(
                "Initialization delta acceleration for delta mode in prediction mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 19>.",
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
            name: "BrIniDeltaPos",
            extends: None,
            description: Some(
                "Initialization delta position for delta mode in prediction mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: ufix<32, 32>.",
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
            name: "BrIniDeltaPosTime",
            extends: None,
            description: Some(
                "Initialization timestamp for delta mode in prediction mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "indicate the time to change the values. 0: instant change.",
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
            name: "BrIniDeltaRev",
            extends: None,
            description: Some(
                "Initialization delta revolution for delta mode in prediction mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 0>.",
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
            name: "BrIniDeltaSpeed",
            extends: None,
            description: Some(
                "Initialization delta speed for delta mode in prediction mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 19>.",
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
            name: "BrIniPos",
            extends: None,
            description: Some(
                "Initialization position for open-loop mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value ufix<32, 32>.",
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
            name: "BrIniPosTime",
            extends: None,
            description: Some(
                "Initialization timestamp for open-loop mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "indicate the time to change the values. 0: instant change.",
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
            name: "BrIniRev",
            extends: None,
            description: Some(
                "Initialization revolution for open-loop mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value ufix<32, 0>.",
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
            name: "BrIniSpeed",
            extends: None,
            description: Some(
                "Initialization speed for open-loop mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value fix<32, 19>.",
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
            name: "BrSt",
            extends: None,
            description: Some(
                "Prediction Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_id",
                    description: Some(
                        "The module's error ID output.",
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
                    name: "idle",
                    description: Some(
                        "1: The prediction module is idle. 0: The prediction module is not idle.",
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
                    name: "ini_delta_pos_done",
                    description: Some(
                        "1: the initialization of delta position command is done 0: the initialization of delta position command is not done.",
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
                    name: "pos_trg_vld",
                    description: Some(
                        "1：self position trigger event found 0：self position trigger event not found yet.",
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
                    name: "speed_trg_vld",
                    description: Some(
                        "1：self speed trigger event found 0：self speed trigger event not found yet.",
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
                    name: "open_loop_st",
                    description: Some(
                        "1：in open loop mode 0：in closed loop mode.",
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
            ],
        },
        FieldSet {
            name: "BrTimeoff",
            extends: None,
            description: Some(
                "Prediction Timing Offset Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "ufix<32, 0> time offset incycles from the trigger time.",
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
            name: "BrTrgFTime",
            extends: None,
            description: Some(
                "Prediction Triggering First Offset Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "uifx<32, 0> the time for the first trigger.",
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
            name: "BrTrgPeriod",
            extends: None,
            description: Some(
                "Prediction Triggering Period Offset Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "uifx<32, 0>, time offset incycles between each trigger time.",
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
            name: "BrTrgPosCfg",
            extends: None,
            description: Some(
                "Prediction Configuration postion trigger cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "1-trigger valid; 0-Trigger not valid.",
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
                    name: "edge",
                    description: Some(
                        "bit1: 0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than.",
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
            name: "BrTrgPosThr",
            extends: None,
            description: Some(
                "Prediction Configuration postion threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For pos out trigger (pos). ufix<32, 32>.",
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
            name: "BrTrgRevThr",
            extends: None,
            description: Some(
                "Prediction Configuration revolutiom threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For pos out trigger (rev) ufix<32, 0>.",
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
            name: "BrTrgSpeedCfg",
            extends: None,
            description: Some(
                "Prediction Configuration speed trigger cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed.",
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
                    name: "edge_sel",
                    description: Some(
                        "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than.",
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
                    name: "comp_type",
                    description: Some(
                        "Use abs value for comparion. 0: Use the speed with direction info (so not the abs value).",
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
            name: "BrTrgSpeedThr",
            extends: None,
            description: Some(
                "Prediction Configuration speed threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For speed trigger. continuous mode: fix<32, 19>.",
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
            name: "ContCfg0",
            extends: None,
            description: Some(
                "Continuous Mode Configuration 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "half_circ_theta",
                    description: Some(
                        "the theta for cal the clockwise or anticlockwise rotation between two adjacent inputs, ufix<32, 32>.",
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
            name: "Cr",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mod_en",
                    description: Some(
                        "Module Enable.",
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
                    name: "discretetrc",
                    description: Some(
                        "1: Discrete position input 0: Continuous position input.",
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
                    name: "adjop",
                    description: Some(
                        "1: use the input iposition whenever a new iposition comes, and force the predicted output stop at the boundaries. 0: Continuous tracking mode, without any boundary check.",
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
                    name: "shadow_rd_req",
                    description: Some(
                        "1: Shadow Request for read of tracking parameters. Auto clear 0:.",
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
                    name: "ini_coefs_cmd",
                    description: Some(
                        "1: Command to reload the coefs. Auto clear 0:.",
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
                    name: "ini_coefs_cmd_msk",
                    description: Some(
                        "1: change 0: won't change bit 2: for ACOEF bit 1: for ICOEF bit 0: for PCOEF.",
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
                Field {
                    name: "ini_pos_req",
                    description: Some(
                        "1: Command to reload the positions. Auto clear 0:.",
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
                    name: "ini_pos_cmd_msk",
                    description: Some(
                        "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position.",
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
                    name: "pos_type",
                    description: Some(
                        "1: 32-bit for rev+pos, with each element occupying 16 bits 0: 32-bit for rev, and 32 bit for pos When CR[MANUAL_IO]==1, 1: means that the INI_POS is acting as INI_POS cmds 0: means that the INI_POS is simulating the input of iposition and itimestamp.",
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
                    name: "open_loop_mode",
                    description: Some(
                        "1: in open loop mode 0: not in open loop mode.",
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
                    name: "ini_delta_pos_req",
                    description: Some(
                        "1: Command to reload the delta pos. Auto clear 0:.",
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
                    name: "ini_delta_pos_cmd_msk",
                    description: Some(
                        "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position.",
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
                    name: "ini_pos_trg_type",
                    description: Some(
                        "0: Time Stamp in the configuration 1: Risedge of In Trg[0] 2: Risedge of In Trg[1] 3: Risedge of out trg[0] 4: Risedge of out trg[1] 5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ini_delta_pos_trg_type",
                    description: Some(
                        "0: Time Stamp in the configuration 1: Risedge of In Trg[0] 2: Risedge of In Trg[1] 3: Risedge of out trg[0] 4: Risedge of out trg[1] 5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ms_coef_en",
                    description: Some(
                        "Multiple Coefficients Enable.",
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
                    name: "frcaccelzero",
                    description: Some(
                        "Zeroise the accelerator calculation.",
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
                    name: "ini_br1_pos_req",
                    description: Some(
                        "Auto clear. Only effective in open_loop mode.",
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
                    name: "ini_br0_pos_req",
                    description: Some(
                        "Auto clear. Only effective in open_loop mode.",
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
                    name: "sftrst",
                    description: Some(
                        "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All MMC internal registers are forced into their reset state. Interface registers are not affected.",
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
            name: "CurAcoef",
            extends: None,
            description: Some(
                "The coefficient A register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "CurIcoef",
            extends: None,
            description: Some(
                "The coefficient I register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "CurPcoef",
            extends: None,
            description: Some(
                "The coefficient P register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "DiscreteCfg0",
            extends: None,
            description: Some(
                "Discrete Mode Configuration 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "posmax",
                    description: Some(
                        "Max ID Of Lines. For example-1, for 512 lines, it is 511. ufix<32, 0>.",
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
            name: "DiscreteCfg1",
            extends: None,
            description: Some(
                "Discrete Mode Configuration 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inv_posmax",
                    description: Some(
                        "discrete mode: ufix<32, 0> of 1/(Number Of Lines) continuous mode: the max delta for tracking from the last received position, ufix<32, 32>.",
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
            name: "ErrThr",
            extends: None,
            description: Some(
                "Tracking Configuration coef trigger cfg&index0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "ErrThr0: Error Threshold 0, (abs(tracking error)>= will choose the coefs as below) Note: ErrThr0>ErrThr1>ErrThr2 ufix<31, 28>.",
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
            name: "EstmAccel",
            extends: None,
            description: Some(
                "The accelerator register for theinternal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "EstmPos",
            extends: None,
            description: Some(
                "The position register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "EstmRev",
            extends: None,
            description: Some(
                "The revolution register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "EstmSpeed",
            extends: None,
            description: Some(
                "The speed register for the internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "EstmTim",
            extends: None,
            description: Some(
                "The timestamp register for internal estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value.",
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
            name: "I",
            extends: None,
            description: Some(
                "Tracking Configuration coef trigger cfg&index0 I.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "I0_Coef, fix<32, 21>.",
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
            name: "IniAccel",
            extends: None,
            description: Some(
                "The destined accelerator register for position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 19>.",
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
            name: "IniAcoef",
            extends: None,
            description: Some(
                "The destined coefficient A register for coefficients initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value, fix<32, 19>.",
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
            name: "IniCoefTime",
            extends: None,
            description: Some(
                "The destined timestamp register for coefficients initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "indicate the time to change the values. 0: instant change.",
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
            name: "IniDeltaAccel",
            extends: None,
            description: Some(
                "The destined delta accelerator register for delta position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 19>.",
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
            name: "IniDeltaPos",
            extends: None,
            description: Some(
                "The destined delta position register for delta position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: ufix <32, 32>.",
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
            name: "IniDeltaPosTime",
            extends: None,
            description: Some(
                "The destined timestamp register for delta position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "indicate the time to change the values. 0: instant change.",
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
            name: "IniDeltaRev",
            extends: None,
            description: Some(
                "The destined delta revolution register for delta position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value continuous mode: fix<32, 0>.",
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
            name: "IniDeltaSpeed",
            extends: None,
            description: Some(
                "The destined delta speed register for delta position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value； continuous mode: fix<32, 19>.",
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
            name: "IniIcoef",
            extends: None,
            description: Some(
                "The destined coefficient I register for coefficients initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value, fix<32, 21>.",
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
            name: "IniPcoef",
            extends: None,
            description: Some(
                "The destined coefficient P register for coefficients initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value, fix<32, 15>.",
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
            name: "IniPos",
            extends: None,
            description: Some(
                "The destined position register for position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value； continuous mode: ufix<32, 32>.",
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
            name: "IniPosTime",
            extends: None,
            description: Some(
                "The destined timestamp register for position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "indicate the time to change the values. 0: instant change.",
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
            name: "IniRev",
            extends: None,
            description: Some(
                "The destined revolution register for position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value； continuous mode: ufix<32, 0>.",
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
            name: "IniSpeed",
            extends: None,
            description: Some(
                "The destined speed register for position initialization.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the value; continuous mode: fix<32, 19>.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow_rd_done_ie",
                    description: Some(
                        "Interrupt Enable for SHADOW_RD_DONE.",
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
                    name: "ini_coefs_cmd_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_COEFS_CMD_DONE.",
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
                    name: "ini_pos_req_cmd_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_POS_REQ_CMD_DONE.",
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
                    name: "oosync_ie",
                    description: Some(
                        "Interrupt Enable for OOSYNC.",
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
                    name: "ini_br1_pos_req_cmd_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_BR1_POS_REQ_CMD_DONE.",
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
                    name: "ini_br0_pos_req_cmd_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_BR0_POS_REQ_CMD_DONE.",
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
                    name: "ini_delta_pos_req_cmd_done_ie",
                    description: Some(
                        "Interrupt Enable for INI_DELTA_POS_REQ_CMD_DONE.",
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
                    name: "pos_trg_vld_ie",
                    description: Some(
                        "Interrupt Enable for POS_TRG_VALID.",
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
                    name: "speed_trg_vld_ie",
                    description: Some(
                        "Interrupt Enable for SPEED_TRG_VALID.",
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
            ],
        },
        FieldSet {
            name: "OosyncThetaThr",
            extends: None,
            description: Some(
                "Position Out-Of-Sync Threshold Regster.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "the threshold of theta difference between actual and prediction for out-of-sync determination，ufix<32, 32>.",
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
            name: "P",
            extends: None,
            description: Some(
                "Tracking Configuration coef trigger cfg&index0 P.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "P0_Coef, fix<32, 15>.",
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
            name: "PosTrgCfg",
            extends: None,
            description: Some(
                "Tracking Configuration pos trigger cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "1-trigger valid; 0-Trigger not valid\".",
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
                    name: "edge",
                    description: Some(
                        "0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than.",
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
            name: "PosTrgPosThr",
            extends: None,
            description: Some(
                "Tracking Configuration position threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For pos out trigger (pos). ufix<32, 32>.",
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
            name: "PosTrgRevThr",
            extends: None,
            description: Some(
                "Tracking Configuration revolution threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For pos out trigger (rev) fix<32, 0>.",
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
            name: "SpeedTrgCfg",
            extends: None,
            description: Some(
                "Tracking Configuration speed trigger cfg.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed.",
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
                    name: "edge",
                    description: Some(
                        "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than.",
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
                    name: "comp_type",
                    description: Some(
                        "1: Use abs value for comparion. 0: Use the speed with direction info (so not the abs value).",
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
            name: "SpeedTrgThr",
            extends: None,
            description: Some(
                "Tracking Configuration speed threshold.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "For speed trigger. continuous mode: fix<32, 19>.",
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
            name: "Sta",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow_rd_done",
                    description: Some(
                        "Shadow ready for read. Auto cleared by setting CR[SHADOW_RD_REQ] as 1.",
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
                    name: "ini_coefs_cmd_done",
                    description: Some(
                        "W1C.",
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
                    name: "ini_pos_req_cmd_done",
                    description: Some(
                        "W1C.",
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
                    name: "oosync",
                    description: Some(
                        "Tracking module out-of sync. W1C.",
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
                    name: "idle",
                    description: Some(
                        "Tracking Module in Idle status.",
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
                    name: "ini_br1_pos_req_cmd_done",
                    description: Some(
                        "W1C.",
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
                    name: "ini_br0_pos_req_cmd_done",
                    description: Some(
                        "W1C.",
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
                    name: "ini_delta_pos_req_cmd_done",
                    description: Some(
                        "W1C.",
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
                    name: "pos_trg_valid",
                    description: Some(
                        "W1C.",
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
                    name: "speed_trg_valid",
                    description: Some(
                        "W1C.",
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
                    name: "err_id",
                    description: Some(
                        "Tracking ERR_ID.",
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
            name: "SysclkFreq",
            extends: None,
            description: Some(
                "System Clock Frequency Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "system clock frequency, ufix<32, 0>.",
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
            name: "SysclkPeriod",
            extends: None,
            description: Some(
                "System Clock Period Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "round( the value of clock period * (2^24)*(2^20) ), ufix<32, 0>.",
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
                "Tracking Configuration coef trigger cfg&index0 time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "CoefTime0: Time Stayed using this coefs (counted in input samples). Ideal value of tracing cycles should +1. ufix<32,0>.",
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
    ],
    enums: &[],
};
