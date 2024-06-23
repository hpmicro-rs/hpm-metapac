use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lin",
            extends: None,
            description: Some(
                "LIN0.",
            ),
            items: &[
                BlockItem {
                    name: "databyte",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
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
                                "Databyte",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "control",
                    description: Some(
                        "control register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Control",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "state",
                    description: Some(
                        "state register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "State",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "error",
                    description: Some(
                        "error register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Error",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data_len",
                    description: Some(
                        "data lenth register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DataLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "baudrate_ctl_low",
                    description: Some(
                        "baudrate control low register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BaudrateCtlLow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bardrate_ctl_high",
                    description: Some(
                        "baudrate control high register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BardrateCtlHigh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "id",
                    description: Some(
                        "id register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Id",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tv",
                    description: Some(
                        "timeout control register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tv",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BardrateCtlHigh",
            extends: None,
            description: Some(
                "baudrate control high register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bt_div_high",
                    description: Some(
                        "bit div register 8.",
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
                    name: "bt_mul",
                    description: Some(
                        "bt_mul register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prescl",
                    description: Some(
                        "prescl register.",
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
            name: "BaudrateCtlLow",
            extends: None,
            description: Some(
                "baudrate control low register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bt_div_low",
                    description: Some(
                        "bit div register 7:0.",
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
            name: "Control",
            extends: None,
            description: Some(
                "control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start_req",
                    description: Some(
                        "master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred.",
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
                    name: "wakeup_req",
                    description: Some(
                        "wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core.",
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
                    name: "reset_error",
                    description: Some(
                        "assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0.",
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
                    name: "reset_int",
                    description: Some(
                        "write 1 to reset the int bit in the status register and the interrupt request output of LIN.",
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
                    name: "data_ack",
                    description: Some(
                        "slave only. Write 1 after handling a data request interrupt.",
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
                    name: "transmit",
                    description: Some(
                        "1: transmit operation 0: receive operation.",
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
                    name: "sleep",
                    description: Some(
                        "The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core.",
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
                    name: "stop",
                    description: Some(
                        "slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID.",
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
            name: "DataLen",
            extends: None,
            description: Some(
                "data lenth register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_length",
                    description: Some(
                        "data length.",
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
                    name: "enh_check",
                    description: Some(
                        "1:enhence check mode.",
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
            name: "Databyte",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data_byte",
                    description: Some(
                        "data byte.",
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
            name: "Error",
            extends: None,
            description: Some(
                "error register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bit_error",
                    description: Some(
                        "bit error.",
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
                    name: "chk_error",
                    description: Some(
                        "checksum error.",
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
                    name: "timeout",
                    description: Some(
                        "timeout error. The master detects a timeout error if it is expecting data from the bus but no slave does respond. The slave detects a timeout error if it is requesting a data acknowledge to the host controller. The slave detects a timeout if it has transmitted a wakeup signal and it detects no sync field within 150ms.",
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
                    name: "parity_error",
                    description: Some(
                        "slave only. identifier parity error.",
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
            name: "Id",
            extends: None,
            description: Some(
                "id register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id",
                    description: Some(
                        "id register.",
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
            ],
        },
        FieldSet {
            name: "State",
            extends: None,
            description: Some(
                "state register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "complete",
                    description: Some(
                        "set after a transmission has been successful finished and it will reset at the start of a transmission.",
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
                    name: "wakeup",
                    description: Some(
                        "set when transmitting a wakeup signal or when received a wakeup signal. Clear when reset_error bit is 1.",
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
                    name: "error",
                    description: Some(
                        "set when detecte an error, clear by reset_error.",
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
                    name: "int",
                    description: Some(
                        "set when request an interrupt. Reset by reset_int.",
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
                    name: "data_req",
                    description: Some(
                        "slave only. Sets after receiving the identifier and requests an interrupt to the host controller.",
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
                    name: "aborted",
                    description: Some(
                        "slave only. This bit is set by LIN core slave if a transmission is aborted after the bneginning of the data field due to a timeout or bit error.",
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
                    name: "bus_idle_tv",
                    description: Some(
                        "slave only. This bit is set by LIN core if bit sleep is not set and no bus activity is detected for 4s.",
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
                    name: "lin_active",
                    description: Some(
                        "The bit indicates whether the LIN bus is active or not.",
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
            name: "Tv",
            extends: None,
            description: Some(
                "timeout control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wup_repeat_time",
                    description: Some(
                        "slave only. wakeup repeat interval time 00-180ms 01-200ms 10-220ms 11-240ms.",
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
                    name: "bus_inactivity_time",
                    description: Some(
                        "slave only. LIN bus idle timeout register： 00-4s 01-6s 10-8s 11-10s.",
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
                    name: "master_mode",
                    description: Some(
                        "master_mode.",
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
                    name: "initial_mode",
                    description: Some(
                        "initial_mode.",
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
    ],
    enums: &[],
};
