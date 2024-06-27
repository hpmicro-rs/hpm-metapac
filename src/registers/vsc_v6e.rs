use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Vsc",
            extends: None,
            description: Some(
                "VSC0.",
            ),
            items: &[
                BlockItem {
                    name: "abc_mode",
                    description: Some(
                        "abc mode.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AbcMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_chan_assign",
                    description: Some(
                        "assign adc_chan for value_a/b/c.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcChanAssign",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_a_data_opt",
                    description: Some(
                        "value_a data operation mode.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueADataOpt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_b_data_opt",
                    description: Some(
                        "value_b data operation mode.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueBDataOpt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_c_data_opt",
                    description: Some(
                        "value_c data operation mode.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueCDataOpt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_a_offset",
                    description: Some(
                        "value_a offset.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueAOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_b_offset",
                    description: Some(
                        "value_b_offset.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueBOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_c_offset",
                    description: Some(
                        "value_c offset.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueCOffset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_status",
                    description: Some(
                        "irq status.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_a_sw",
                    description: Some(
                        "value_a software inject value.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueASw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_b_sw",
                    description: Some(
                        "value_b software inject value.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueBSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_c_sw",
                    description: Some(
                        "value_c software inject value.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueCSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "value_sw_ready",
                    description: Some(
                        "software inject value_a/value_b/value_c ready.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValueSwReady",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trigger_sw",
                    description: Some(
                        "software trigger event.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TriggerSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timelock",
                    description: Some(
                        "timestamp mode and postion capture ctrl.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timelock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "position_sw",
                    description: Some(
                        "position software inject value.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PositionSw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_wait_cycle",
                    description: Some(
                        "adc wait cycle after trigger adc capture event.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcWaitCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_wait_cycle",
                    description: Some(
                        "pos wait cycle after trigger adc capture event.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosWaitCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irq_enable",
                    description: Some(
                        "irq bit enable.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IrqEnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_phase_tolerate",
                    description: Some(
                        "adc phase tolerate.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AdcPhaseTolerate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pos_pole",
                    description: Some(
                        "position pole num.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PosPole",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "id_posedge",
                    description: Some(
                        "posedge order Id value.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IdPosedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iq_posedge",
                    description: Some(
                        "posedge order Iq value.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IqPosedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "id_negedge",
                    description: Some(
                        "negedge order Id value.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IdNegedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iq_negedge",
                    description: Some(
                        "negedge order Iq value.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IqNegedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alpha_posedge",
                    description: Some(
                        "posedge order alpha value.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlphaPosedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "beta_posedge",
                    description: Some(
                        "posedge order beta value.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BetaPosedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alpha_negedge",
                    description: Some(
                        "negedge order alpha value.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlphaNegedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "beta_negedge",
                    description: Some(
                        "negedge order beta value.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BetaNegedge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timestamp_locked",
                    description: Some(
                        "timestamp_locked.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TimestampLocked",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "debug_status0",
                    description: Some(
                        "debug_status0.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DebugStatus0",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AbcMode",
            extends: None,
            description: Some(
                "abc mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable_vsc",
                    description: Some(
                        "enable vsc convert: 0: disable vsc convert 1: enable vsc convert.",
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
                    name: "value_a_loc",
                    description: Some(
                        "the adc index of value_a: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;.",
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
                    name: "value_b_loc",
                    description: Some(
                        "the adc index of value_b: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;.",
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
                    name: "value_c_loc",
                    description: Some(
                        "the adc index of value_c: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;.",
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
                    name: "value_a_width",
                    description: Some(
                        "numbers of value_a for each convert.",
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
                    name: "value_b_width",
                    description: Some(
                        "numbers of value_b for each convert.",
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
                    name: "value_c_width",
                    description: Some(
                        "numbers of value_c for each convert.",
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
                    name: "phase_absent_mode",
                    description: Some(
                        "whether using value_a and value_b instead of three phase.",
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
            name: "AdcChanAssign",
            extends: None,
            description: Some(
                "assign adc_chan for value_a/b/c.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_a_chan",
                    description: Some(
                        "value_a's adc chan.",
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
                    name: "value_b_chan",
                    description: Some(
                        "value_b's adc chan.",
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
                    name: "value_c_chan",
                    description: Some(
                        "value_c's adc chan.",
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
            name: "AdcPhaseTolerate",
            extends: None,
            description: Some(
                "adc phase tolerate.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_phase_tolerate",
                    description: Some(
                        "in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq.",
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
            name: "AdcWaitCycle",
            extends: None,
            description: Some(
                "adc wait cycle after trigger adc capture event.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc_wait_cycle",
                    description: Some(
                        "adc wait cycle after trigger adc capture event.",
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
            name: "AlphaNegedge",
            extends: None,
            description: Some(
                "negedge order alpha value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alpha_negedge",
                    description: Some(
                        "negedge order alpha value.",
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
            name: "AlphaPosedge",
            extends: None,
            description: Some(
                "posedge order alpha value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alpha_posedge",
                    description: Some(
                        "posedge order alpha value.",
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
            name: "BetaNegedge",
            extends: None,
            description: Some(
                "negedge order beta value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "beta_negedge",
                    description: Some(
                        "negedge order beta value.",
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
            name: "BetaPosedge",
            extends: None,
            description: Some(
                "posedge order beta value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "beta_posedge",
                    description: Some(
                        "posedge order beta value.",
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
            name: "DebugStatus0",
            extends: None,
            description: Some(
                "debug_status0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_c_counter",
                    description: Some(
                        "value_c_counter.",
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
                    name: "value_b_counter",
                    description: Some(
                        "value_b_counter.",
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
                    name: "value_a_counter",
                    description: Some(
                        "value_a_counter.",
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
            ],
        },
        FieldSet {
            name: "IdNegedge",
            extends: None,
            description: Some(
                "negedge order Id value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id_negedge",
                    description: Some(
                        "negedge order Id value.",
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
            name: "IdPosedge",
            extends: None,
            description: Some(
                "posedge order Id value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id_posedge",
                    description: Some(
                        "posedge order Id value.",
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
            name: "IqNegedge",
            extends: None,
            description: Some(
                "negedge order Iq value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iq_negedge",
                    description: Some(
                        "negedge order Iq value.",
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
            name: "IqPosedge",
            extends: None,
            description: Some(
                "posedge order Iq value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iq_posedge",
                    description: Some(
                        "posedge order Iq value.",
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
            name: "IrqEnable",
            extends: None,
            description: Some(
                "irq bit enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_enable",
                    description: Some(
                        "irq enable bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready.",
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
            name: "IrqStatus",
            extends: None,
            description: Some(
                "irq status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_status",
                    description: Some(
                        "irq status bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready.",
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
            name: "PosPole",
            extends: None,
            description: Some(
                "position pole num.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_pole",
                    description: Some(
                        "pole number.",
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
            name: "PosWaitCycle",
            extends: None,
            description: Some(
                "pos wait cycle after trigger adc capture event.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pos_wait_cycle",
                    description: Some(
                        "position wait cycle after trigger adc capture event.",
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
            name: "PositionSw",
            extends: None,
            description: Some(
                "position software inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "position_sw",
                    description: Some(
                        "position_sw.",
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
            name: "Timelock",
            extends: None,
            description: Some(
                "timestamp mode and postion capture ctrl.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_counter_sel",
                    description: Some(
                        "adc timestamp use which number index of adc_timestamp_sel used.",
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
                    name: "adc_timestamp_sel",
                    description: Some(
                        "adc timestamp select： 0：reserved; 1: from value_a; 2: from value_b; 3: from value_c;.",
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
                    name: "position_capture_mode",
                    description: Some(
                        "postion capture mode: 00: position use last valid data when adc value capture finish 01: position use frist valid data after adc value capture 10: position use last valid data before adc value capture other: reserved.",
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
            ],
        },
        FieldSet {
            name: "TimestampLocked",
            extends: None,
            description: Some(
                "timestamp_locked.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timestamp_locked",
                    description: Some(
                        "timestamp_locked.",
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
            name: "TriggerSw",
            extends: None,
            description: Some(
                "software trigger event.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigger_sw",
                    description: Some(
                        "software trigger to start waiting adc capture value, same as hardwire trigger_in.",
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
            name: "ValueADataOpt",
            extends: None,
            description: Some(
                "value_a data operation mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opt_0",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_1",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_2",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_3",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
            name: "ValueAOffset",
            extends: None,
            description: Some(
                "value_a offset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_a_offset",
                    description: Some(
                        "value_a offset.",
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
            name: "ValueASw",
            extends: None,
            description: Some(
                "value_a software inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_a_sw",
                    description: Some(
                        "value_a_sw.",
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
            name: "ValueBDataOpt",
            extends: None,
            description: Some(
                "value_b data operation mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opt_0",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_1",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_2",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_3",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
            name: "ValueBOffset",
            extends: None,
            description: Some(
                "value_b_offset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_b_offset",
                    description: Some(
                        "value_b_offset.",
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
            name: "ValueBSw",
            extends: None,
            description: Some(
                "value_b software inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_b_sw",
                    description: Some(
                        "value_b_sw.",
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
            name: "ValueCDataOpt",
            extends: None,
            description: Some(
                "value_c data operation mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "opt_0",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_1",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_2",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
                    name: "opt_3",
                    description: Some(
                        "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4.",
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
            name: "ValueCOffset",
            extends: None,
            description: Some(
                "value_c offset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_c_offset",
                    description: Some(
                        "value_c offset.",
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
            name: "ValueCSw",
            extends: None,
            description: Some(
                "value_c software inject value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_c_sw",
                    description: Some(
                        "value_c_sw.",
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
            name: "ValueSwReady",
            extends: None,
            description: Some(
                "software inject value_a/value_b/value_c ready.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value_sw_ready",
                    description: Some(
                        "software inject value_a/value_b/value_c ready.",
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
    ],
    enums: &[],
};
