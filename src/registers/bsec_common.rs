use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Bsec",
            extends: None,
            description: Some(
                "BSEC.",
            ),
            items: &[
                BlockItem {
                    name: "secure_state",
                    description: Some(
                        "Secure state.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecureState",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secure_state_config",
                    description: Some(
                        "secure state configuration.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecureStateConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "violation_config",
                    description: Some(
                        "Security violation config.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ViolationConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "escalate_config",
                    description: Some(
                        "Escalate behavior on security event.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EscalateConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "event",
                    description: Some(
                        "Event and escalate status.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Event",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "EscalateConfig",
            extends: None,
            description: Some(
                "Escalate behavior on security event.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_vio_cfg",
                    description: Some(
                        "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate.",
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
                Field {
                    name: "lock_sec",
                    description: Some(
                        "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored.",
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
                    name: "nsc_vio_cfg",
                    description: Some(
                        "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock_nsc",
                    description: Some(
                        "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored.",
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
            name: "Event",
            extends: None,
            description: Some(
                "Event and escalate status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "batt_esc_sec",
                    description: Some(
                        "BATT is escalting ssecure event.",
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
                    name: "batt_esc_nsc",
                    description: Some(
                        "BATT is escalating non-secure event.",
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
                    name: "event",
                    description: Some(
                        "local event statue, each bit represents one security event.",
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
            name: "SecureState",
            extends: None,
            description: Some(
                "Secure state.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "batt_ins",
                    description: Some(
                        "BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state.",
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
                    name: "batt_sec",
                    description: Some(
                        "BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state.",
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
                    name: "batt_nsc",
                    description: Some(
                        "BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state.",
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
                    name: "batt_fail",
                    description: Some(
                        "BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state.",
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
                    name: "allow_sec",
                    description: Some(
                        "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state.",
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
                    name: "allow_nsc",
                    description: Some(
                        "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state.",
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
            name: "SecureStateConfig",
            extends: None,
            description: Some(
                "secure state configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "allow_restart",
                    description: Some(
                        "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state.",
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
                    name: "lock",
                    description: Some(
                        "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored.",
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
            name: "ViolationConfig",
            extends: None,
            description: Some(
                "Security violation config.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec_vio_cfg",
                    description: Some(
                        "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation.",
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
                Field {
                    name: "lock_sec",
                    description: Some(
                        "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored.",
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
                    name: "nsc_vio_cfg",
                    description: Some(
                        "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lock_nsc",
                    description: Some(
                        "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored.",
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
