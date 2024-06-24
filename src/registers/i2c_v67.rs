use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "I2c",
            extends: None,
            description: Some(
                "I2C0.",
            ),
            items: &[
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "Configuration Register.",
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
                    name: "int_en",
                    description: Some(
                        "Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    name: "status",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                BlockItem {
                    name: "addr",
                    description: Some(
                        "Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "Data Register.",
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
                    name: "ctrl",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "cmd",
                    description: Some(
                        "Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "setup",
                    description: Some(
                        "Setup Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Setup",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tpm",
                    description: Some(
                        "I2C Timing Paramater Multiplier.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tpm",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr",
            extends: None,
            description: Some(
                "Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "The slave address. For 7-bit addressing mode, the most significant 3 bits are ignored and only the least-significant 7 bits of Addr are valid.",
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
            name: "Cfg",
            extends: None,
            description: Some(
                "Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fifosize",
                    description: Some(
                        "FIFO Size: 0: 2 bytes 1: 4 bytes 2: 8 bytes 3: 16 bytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "FifoSize",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cmd",
            extends: None,
            description: Some(
                "Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd",
                    description: Some(
                        "Write this register with the following values to perform the corresponding actions: 0x0: no action 0x1: issue a data transaction (Master only) 0x2: respond with an ACK to the received byte 0x3: respond with a NACK to the received byte 0x4: clear the FIFO 0x5: reset the I2C controller (abort current transaction, set the SDA and SCL line to the open-drain mode, reset the Status Register and the Interrupt Enable Register, and empty the FIFO) When issuing a data transaction by writing 0x1 to this register, the CMD field stays at 0x1 for the duration of the entire transaction, and it is only cleared to 0x0 after when the transaction has completed or when the controller loses the arbitration. Note: No transaction will be issued by the controller when all phases (Start, Address, Data and Stop) are disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Cmd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datacnt",
                    description: Some(
                        "Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received.",
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
                    name: "dir",
                    description: Some(
                        "Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dir",
                    ),
                },
                Field {
                    name: "phase_stop",
                    description: Some(
                        "Enable this bit to send a STOP condition at the end of a transaction. Master mode only.",
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
                    name: "phase_data",
                    description: Some(
                        "Enable this bit to send the data after Address phase. Master mode only.",
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
                    name: "phase_addr",
                    description: Some(
                        "Enable this bit to send the address after START condition. Master mode only.",
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
                    name: "phase_start",
                    description: Some(
                        "Enable this bit to send a START condition at the beginning of transaction. Master mode only.",
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
            name: "Data",
            extends: None,
            description: Some(
                "Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "Write this register to put one byte of data to the FIFO. Read this register to get one byte of data from the FIFO.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fifoempty",
                    description: Some(
                        "Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty.",
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
                    name: "fifofull",
                    description: Some(
                        "Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full.",
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
                    name: "fifohalf",
                    description: Some(
                        "Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered.",
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
                    name: "addrhit",
                    description: Some(
                        "Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed.",
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
                    name: "arblose",
                    description: Some(
                        "Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode.",
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
                    name: "stop",
                    description: Some(
                        "Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected.",
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
                    name: "start",
                    description: Some(
                        "Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected.",
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
                    name: "bytetrans",
                    description: Some(
                        "Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted.",
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
                    name: "byterecv",
                    description: Some(
                        "Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually.",
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
                    name: "cmpl",
                    description: Some(
                        "Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed.",
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
            name: "Setup",
            extends: None,
            description: Some(
                "Setup Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iicen",
                    description: Some(
                        "Enable the I2C controller. 1: Enable 0: Disable.",
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
                    name: "addressing",
                    description: Some(
                        "I2C addressing mode: 1: 10-bit addressing mode 0: 7-bit addressing mode.",
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
                    name: "master",
                    description: Some(
                        "Configure this device as a master or a slave. 1: Master mode 0: Slave mode.",
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
                    name: "dmaen",
                    description: Some(
                        "Enable the direct memory access mode data transfer. 1: Enable 0: Disable.",
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
                    name: "t_sclhi",
                    description: Some(
                        "The HIGH period of generated SCL clock is defined by T_SCLHi. SCL HIGH period = (2 * tpclk) + (2 + T_SP + T_SCLHi) * tpclk* (TPM+1) The T_SCLHi value must be greater than T_SP and T_HDDAT values. This field is only valid when the controller is in the master mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_sclradio",
                    description: Some(
                        "The LOW period of the generated SCL clock is defined by the combination of T_SCLRatio and T_SCLHi values. When T_SCLRatio = 0, the LOW period is equal to HIGH period. When T_SCLRatio = 1, the LOW period is roughly two times of HIGH period. SCL LOW period = (2 * tpclk) + (2 + T_SP + T_SCLHi * ratio) * tpclk * (TPM+1) 1: ratio = 2 0: ratio = 1 This field is only valid when the controller is in the master mode.",
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
                    name: "t_hddat",
                    description: Some(
                        "T_HDDAT defines the data hold time after SCL goes LOW Hold time = (2 * tpclk) + (2 + T_SP + T_HDDAT) * tpclk* (TPM+1).",
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
                    name: "t_sp",
                    description: Some(
                        "T_SP defines the pulse width of spikes that must be suppressed by the input filter. Pulse width = T_SP * tpclk* (TPM+1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_sudat",
                    description: Some(
                        "T_SUDAT defines the data setup time before releasing the SCL. Setup time = (2 * tpclk) + (2 + T_SP + T_SUDAT) * tpclk* (TPM+1) tpclk = PCLK period TPM = The multiplier value in Timing Parameter Multiplier Register.",
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
            name: "Status",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fifoempty",
                    description: Some(
                        "Indicates that the FIFO is empty.",
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
                    name: "fifofull",
                    description: Some(
                        "Indicates that the FIFO is full.",
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
                    name: "fifohalf",
                    description: Some(
                        "Transmitter: Indicates that the FIFO is half-empty.",
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
                    name: "addrhit",
                    description: Some(
                        "Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call).",
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
                    name: "arblose",
                    description: Some(
                        "Indicates that the controller has lost the bus arbitration.",
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
                    name: "stop",
                    description: Some(
                        "Indicates that a STOP Condition has been transmitted/received.",
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
                    name: "start",
                    description: Some(
                        "Indicates that a START Condition or a repeated START condition has been transmitted/received.",
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
                    name: "bytetrans",
                    description: Some(
                        "Indicates that a byte of data has been transmitted.",
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
                    name: "byterecv",
                    description: Some(
                        "Indicates that a byte of data has been received.",
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
                    name: "cmpl",
                    description: Some(
                        "Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked.",
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
                    name: "ack",
                    description: Some(
                        "Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK.",
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
                    name: "busbusy",
                    description: Some(
                        "Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy.",
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
                    name: "gencall",
                    description: Some(
                        "Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call.",
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
                    name: "linescl",
                    description: Some(
                        "Indicates the current status of the SCL line on the bus 1: high 0: low.",
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
                    name: "linesda",
                    description: Some(
                        "Indicates the current status of the SDA line on the bus 1: high 0: low.",
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
            ],
        },
        FieldSet {
            name: "Tpm",
            extends: None,
            description: Some(
                "I2C Timing Paramater Multiplier.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpm",
                    description: Some(
                        "A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1).",
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
    ],
    enums: &[
        Enum {
            name: "Cmd",
            description: Some(
                "Command Register",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NO_ACTION",
                    description: Some(
                        "No action",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DATA_TRANSACTION",
                    description: Some(
                        "Issue a data transaction (Master only)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ACK",
                    description: Some(
                        "Respond with an ACK to the received byte",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NACK",
                    description: Some(
                        "Respond with a NACK to the received byte",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CLEAR_FIFO",
                    description: Some(
                        "Clear the FIFO",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RESET",
                    description: Some(
                        "Reset the I2C controller (abort current transaction, set the SDA and SCL line to the open-drain mode, reset the Status Register and the Interrupt Enable Register, and empty the FIFO)",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Dir",
            description: Some(
                "Transaction direction",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MASTER_WRITE_SLAVE_READ",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "MASTER_READ_SLAVE_WRITE",
                    description: None,
                    value: 1,
                },
            ],
        },
        Enum {
            name: "FifoSize",
            description: Some(
                "FIFO Size",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_2BYTES",
                    description: Some(
                        "2 bytes",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_4BYTES",
                    description: Some(
                        "4 bytes",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_8BYTES",
                    description: Some(
                        "8 bytes",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "_16BYTES",
                    description: Some(
                        "16 bytes",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
