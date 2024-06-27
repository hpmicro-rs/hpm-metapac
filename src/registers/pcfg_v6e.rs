use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pcfg",
            extends: None,
            description: Some(
                "PCFG.",
            ),
            items: &[
                BlockItem {
                    name: "bandgap",
                    description: Some(
                        "BANGGAP control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bandgap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ldo1p1",
                    description: Some(
                        "1V LDO config.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ldo1p1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ldo2p5",
                    description: Some(
                        "2.5V LDO config.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ldo2p5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_mode",
                    description: Some(
                        "DCDC mode select.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_lpmode",
                    description: Some(
                        "DCDC low power mode.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcLpmode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_prot",
                    description: Some(
                        "DCDC protection.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcProt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_current",
                    description: Some(
                        "DCDC current estimation.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcCurrent",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_advmode",
                    description: Some(
                        "DCDC advance setting.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcAdvmode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_advparam",
                    description: Some(
                        "DCDC advance parameter.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcAdvparam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_misc",
                    description: Some(
                        "DCDC misc parameter.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcMisc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_debug",
                    description: Some(
                        "DCDC Debug.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcDebug",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_start_time",
                    description: Some(
                        "DCDC ramp time.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcStartTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcdc_resume_time",
                    description: Some(
                        "DCDC resume time.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcdcResumeTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "power_trap",
                    description: Some(
                        "SOC power trap.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PowerTrap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wake_cause",
                    description: Some(
                        "Wake up source.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WakeCause",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wake_mask",
                    description: Some(
                        "Wake up mask.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WakeMask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scg_ctrl",
                    description: Some(
                        "Clock gate control in PMIC.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ScgCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rc24m",
                    description: Some(
                        "RC 24M config.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rc24m",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rc24m_track",
                    description: Some(
                        "RC 24M track mode.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rc24mTrack",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "track_target",
                    description: Some(
                        "RC 24M track target.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TrackTarget",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "RC 24M track status.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bandgap",
            extends: None,
            description: Some(
                "BANGGAP control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbg_p50_trim",
                    description: Some(
                        "Banggap 1.0V output trim value.",
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
                    name: "vbg_p65_trim",
                    description: Some(
                        "Banggap 1.0V output trim value.",
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
                    name: "vbg_1p0_trim",
                    description: Some(
                        "Banggap 1.0V output trim value.",
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
                    name: "vbg_trimmed",
                    description: Some(
                        "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed.",
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
            name: "DcdcAdvmode",
            extends: None,
            description: Some(
                "DCDC advance setting.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_dcm",
                    description: Some(
                        "DCM mode 0: CCM mode 1: DCM mode.",
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
                    name: "en_idle",
                    description: Some(
                        "enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess.",
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
                    name: "en_skip",
                    description: Some(
                        "enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse.",
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
                    name: "en_dcm_exit",
                    description: Some(
                        "avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess.",
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
                    name: "en_autolp",
                    description: Some(
                        "enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low.",
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
                    name: "en_ff_loop",
                    description: Some(
                        "enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled.",
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
                    name: "en_ff_det",
                    description: Some(
                        "enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled.",
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
                    name: "dc_r",
                    description: Some(
                        "Loop R number.",
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
                    name: "dc_c",
                    description: Some(
                        "Loop C number.",
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
                    name: "en_rcscale",
                    description: Some(
                        "Enable RC scale.",
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
            name: "DcdcAdvparam",
            extends: None,
            description: Some(
                "DCDC advance parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_dut",
                    description: Some(
                        "maximum duty cycle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "min_dut",
                    description: Some(
                        "minimum duty cycle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DcdcCurrent",
            extends: None,
            description: Some(
                "DCDC current estimation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "level",
                    description: Some(
                        "DCDC current level, current level is num * 50mA.",
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
                    name: "valid",
                    description: Some(
                        "Current level valid 0: data is invalid 1: data is valid.",
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
                    name: "esti_en",
                    description: Some(
                        "enable current measure.",
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
            name: "DcdcDebug",
            extends: None,
            description: Some(
                "DCDC Debug.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "update_time",
                    description: Some(
                        "DCDC voltage change time in 24M clock cycles, default value is 1mS.",
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
            name: "DcdcLpmode",
            extends: None,
            description: Some(
                "DCDC low power mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stby_volt",
                    description: Some(
                        "DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV.",
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
            name: "DcdcMisc",
            extends: None,
            description: Some(
                "DCDC misc parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_step",
                    description: Some(
                        "enable stepping in voltage change 0: stepping disabled, 1: steping enabled.",
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
                    name: "clk_sel",
                    description: Some(
                        "clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator.",
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
                    name: "delay",
                    description: Some(
                        "enable delay 0: delay disabled, 1: delay enabled.",
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
                    name: "ol_hyst",
                    description: Some(
                        "current hysteres range 0: 12.5mV 1: 25mV.",
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
                    name: "ol_thre",
                    description: Some(
                        "overload for threshold for lod power mode.",
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
                    name: "dc_ff",
                    description: Some(
                        "Loop feed forward number.",
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
                    name: "rc_scale",
                    description: Some(
                        "Loop RC scale threshold.",
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
                    name: "hyst_thrs",
                    description: Some(
                        "hysteres threshold.",
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
                    name: "hyst_sign",
                    description: Some(
                        "hysteres sign.",
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
                    name: "en_hyst",
                    description: Some(
                        "hysteres enable.",
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
            ],
        },
        FieldSet {
            name: "DcdcMode",
            extends: None,
            description: Some(
                "DCDC mode select.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "volt",
                    description: Some(
                        "DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV.",
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
                Field {
                    name: "mode",
                    description: Some(
                        "DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode.",
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
                    name: "ready",
                    description: Some(
                        "Ready flag 0: DCDC is applying new change 1: DCDC is ready.",
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
            ],
        },
        FieldSet {
            name: "DcdcProt",
            extends: None,
            description: Some(
                "DCDC protection.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "short_flag",
                    description: Some(
                        "short circuit flag 0: current is within limit 1: short circuits detected.",
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
                    name: "short_current",
                    description: Some(
                        "short circuit current setting 0: 2.0A, 1: 1.3A.",
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
                    name: "disable_short",
                    description: Some(
                        "disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled.",
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
                    name: "overvolt_flag",
                    description: Some(
                        "output over voltage flag 0: output is normal 1: output is unexpected high.",
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
                    name: "disable_overvoltage",
                    description: Some(
                        "ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage.",
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
                    name: "power_loss_flag",
                    description: Some(
                        "power loss 0: input power is good 1: input power is too low.",
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
                    name: "overload_lp",
                    description: Some(
                        "over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode.",
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
                    name: "ilimit_lp",
                    description: Some(
                        "over current setting for low power mode 0:250mA 1:200mA.",
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
            ],
        },
        FieldSet {
            name: "DcdcResumeTime",
            extends: None,
            description: Some(
                "DCDC resume time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "resume_time",
                    description: Some(
                        "Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS.",
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
            name: "DcdcStartTime",
            extends: None,
            description: Some(
                "DCDC ramp time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start_time",
                    description: Some(
                        "Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS.",
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
            name: "Ldo1p1",
            extends: None,
            description: Some(
                "1V LDO config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "volt",
                    description: Some(
                        "LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "LDO enable 0: turn off LDO 1: turn on LDO.",
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
            name: "Ldo2p5",
            extends: None,
            description: Some(
                "2.5V LDO config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "volt",
                    description: Some(
                        "LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "LDO enable 0: turn off LDO 1: turn on LDO.",
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
                    name: "ready",
                    description: Some(
                        "Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready.",
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
            ],
        },
        FieldSet {
            name: "PowerTrap",
            extends: None,
            description: Some(
                "SOC power trap.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trap",
                    description: Some(
                        "Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned.",
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
                    name: "retention",
                    description: Some(
                        "DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage.",
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
                    name: "triggered",
                    description: Some(
                        "Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered.",
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
            name: "Rc24m",
            extends: None,
            description: Some(
                "RC 24M config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trim_f",
                    description: Some(
                        "Fine trim for RC24M, bigger value means faster.",
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
                    name: "trim_c",
                    description: Some(
                        "Coarse trim for RC24M, bigger value means faster.",
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
                    name: "rc_trimmed",
                    description: Some(
                        "RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed.",
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
            name: "Rc24mTrack",
            extends: None,
            description: Some(
                "RC 24M track mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "track",
                    description: Some(
                        "track mode 0: RC24M free running 1: track RC24M to external XTAL.",
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
                    name: "return_",
                    description: Some(
                        "Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value.",
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
                    name: "sel24m",
                    description: Some(
                        "Select track reference 0: select 32K as reference 1: select 24M XTAL as reference.",
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
            name: "ScgCtrl",
            extends: None,
            description: Some(
                "Clock gate control in PMIC.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scg",
                    description: Some(
                        "control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart.",
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
            name: "Status",
            extends: None,
            description: Some(
                "RC 24M track status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trim_f",
                    description: Some(
                        "default fine trim value.",
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
                    name: "trim_c",
                    description: Some(
                        "default coarse trim value.",
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
                    name: "en_trim",
                    description: Some(
                        "default value takes effect 0: default value is invalid 1: default value is valid.",
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
                    name: "sel24m",
                    description: Some(
                        "track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M.",
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
                    name: "sel32k",
                    description: Some(
                        "track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K.",
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
            name: "TrackTarget",
            extends: None,
            description: Some(
                "RC 24M track target.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "target",
                    description: Some(
                        "Target frequency multiplier of divided source.",
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
                    name: "pre_div",
                    description: Some(
                        "Divider for reference source.",
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
            name: "WakeCause",
            extends: None,
            description: Some(
                "Wake up source.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cause",
                    description: Some(
                        "wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit16: batt security interrupt bit17:batt gpio interrupt bit19:rtc interrupt bit31: pin wakeup.",
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
            name: "WakeMask",
            extends: None,
            description: Some(
                "Wake up mask.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask",
                    description: Some(
                        "mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit16: batt security interrupt bit17:batt gpio interrupt bit19:rtc interrupt bit31: pin wakeup.",
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
