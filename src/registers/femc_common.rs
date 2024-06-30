use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Femc",
            extends: None,
            description: Some(
                "FEMC.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
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
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioctrl",
                    description: Some(
                        "IO Mux Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmw0",
                    description: Some(
                        "Bus (AXI) Weight Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmw0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmw1",
                    description: Some(
                        "Bus (AXI) Weight Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmw1",
                            ),
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
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Br",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intr",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrctrl0",
                    description: Some(
                        "SDRAM Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrctrl1",
                    description: Some(
                        "SDRAM Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrctrl2",
                    description: Some(
                        "SDRAM Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrctrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sdrctrl3",
                    description: Some(
                        "SDRAM Control Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sdrctrl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srctrl0",
                    description: Some(
                        "SRAM control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srctrl1",
                    description: Some(
                        "SRAM control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "saddr",
                    description: Some(
                        "IP Command Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "datsz",
                    description: Some(
                        "IP Command Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Datsz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bytemsk",
                    description: Some(
                        "IP Command Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bytemsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipcmd",
                    description: Some(
                        "IP Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipcmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iptx",
                    description: Some(
                        "TX DATA Register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iptx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprx",
                    description: Some(
                        "RX DATA Register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "Status Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dlycfg",
                    description: Some(
                        "Delay Line Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dlycfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bmw0",
            extends: None,
            description: Some(
                "Bus (AXI) Weight Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qos",
                    description: Some(
                        "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score.",
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
                    name: "age",
                    description: Some(
                        "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score.",
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
                    name: "sh",
                    description: Some(
                        "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch.",
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
                    name: "rws",
                    description: Some(
                        "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch.",
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
            name: "Bmw1",
            extends: None,
            description: Some(
                "Bus (AXI) Weight Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qos",
                    description: Some(
                        "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score.",
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
                    name: "age",
                    description: Some(
                        "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score.",
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
                    name: "ph",
                    description: Some(
                        "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch.",
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
                    name: "rws",
                    description: Some(
                        "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch.",
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
                    name: "br",
                    description: Some(
                        "Weight of Bank Rotation. This weight score is valid when queue command's bank is not same as current executing command.",
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
            name: "Br",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vld",
                    description: Some(
                        "Valid.",
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
                    name: "size",
                    description: Some(
                        "Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "MemorySize",
                    ),
                },
                Field {
                    name: "base",
                    description: Some(
                        "Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bytemsk",
            extends: None,
            description: Some(
                "IP Command Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bm0",
                    description: Some(
                        "Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked.",
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
                    name: "bm1",
                    description: Some(
                        "Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked.",
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
                    name: "bm2",
                    description: Some(
                        "Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked.",
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
                    name: "bm3",
                    description: Some(
                        "Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rst",
                    description: Some(
                        "Software Reset Reset all internal logic in SEMC except configuration register.",
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
                    name: "dis",
                    description: Some(
                        "Module Disable 0b - Module enabled 1b - Module disabled.",
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
                    name: "dqs",
                    description: Some(
                        "DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad.",
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
                    name: "cto",
                    description: Some(
                        "Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle.",
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
                    name: "bto",
                    description: Some(
                        "Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31.",
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
            name: "Datsz",
            extends: None,
            description: Some(
                "IP Command Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datsz",
                    description: Some(
                        "Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "DataSize",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dlycfg",
            extends: None,
            description: Some(
                "Delay Line Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlyen",
                    description: Some(
                        "delay line enable.",
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
                    name: "dlysel",
                    description: Some(
                        "delay line select, 0 for 1 cell, 31 for all 32 cells.",
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
                    name: "oe",
                    description: Some(
                        "delay clock output enable, should be set after setting DLYEN and DLYSEL.",
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
            name: "Inten",
            extends: None,
            description: Some(
                "Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipcmddone",
                    description: Some(
                        "IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled.",
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
                    name: "ipcmderr",
                    description: Some(
                        "IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled.",
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
                    name: "axicmderr",
                    description: Some(
                        "AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled.",
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
                    name: "axibuserr",
                    description: Some(
                        "AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled.",
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
            name: "Intr",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipcmddone",
                    description: Some(
                        "IP command normal done interrupt.",
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
                    name: "ipcmderr",
                    description: Some(
                        "IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command.",
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
                    name: "axicmderr",
                    description: Some(
                        "AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout.",
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
                    name: "axibuserr",
                    description: Some(
                        "AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read.",
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
            name: "Ioctrl",
            extends: None,
            description: Some(
                "IO Mux Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "io_csx",
                    description: Some(
                        "IO_CSX output selection 0001b - SDRAM CS1 0110b - SRAM CE#.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "IoCsx",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ipcmd",
            extends: None,
            description: Some(
                "IP Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd",
                    description: Some(
                        "SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: Some(
                        "Cmd",
                    ),
                },
                Field {
                    name: "key",
                    description: Some(
                        "This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers.",
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
            name: "Iprx",
            extends: None,
            description: Some(
                "RX DATA Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dat",
                    description: Some(
                        "Data.",
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
            name: "Iptx",
            extends: None,
            description: Some(
                "TX DATA Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dat",
                    description: Some(
                        "Data.",
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
            name: "Saddr",
            extends: None,
            description: Some(
                "IP Command Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa",
                    description: Some(
                        "Slave address.",
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
            name: "Sdrctrl0",
            extends: None,
            description: Some(
                "SDRAM Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "portsz",
                    description: Some(
                        "Port Size 00b - 8bit 01b - 16bit 10b - 32bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "SdramPortSize",
                    ),
                },
                Field {
                    name: "highband",
                    description: Some(
                        "high band select 0: use data[15:0] for 16bit SDRAM; 1: use data[31:16] for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b).",
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
                    name: "burstlen",
                    description: Some(
                        "Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "BurstLen",
                    ),
                },
                Field {
                    name: "col8",
                    description: Some(
                        "Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored.",
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
                    name: "col",
                    description: Some(
                        "Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ColBits",
                    ),
                },
                Field {
                    name: "cas",
                    description: Some(
                        "CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Cas",
                    ),
                },
                Field {
                    name: "bank2",
                    description: Some(
                        "2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Bank2Sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sdrctrl1",
            extends: None,
            description: Some(
                "SDRAM Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pre2act",
                    description: Some(
                        "PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device.",
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
                    name: "act2rw",
                    description: Some(
                        "ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device.",
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
                    name: "rfrc",
                    description: Some(
                        "Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device.",
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
                    name: "wrc",
                    description: Some(
                        "Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckeoff",
                    description: Some(
                        "CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles.",
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
                    name: "act2pre",
                    description: Some(
                        "ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command.",
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
            ],
        },
        FieldSet {
            name: "Sdrctrl2",
            extends: None,
            description: Some(
                "SDRAM Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srrc",
                    description: Some(
                        "Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command.",
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
                    name: "ref2ref",
                    description: Some(
                        "Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device.",
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
                    name: "act2act",
                    description: Some(
                        "ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device.",
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
                    name: "ito",
                    description: Some(
                        "SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period.",
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
            name: "Sdrctrl3",
            extends: None,
            description: Some(
                "SDRAM Control Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ren",
                    description: Some(
                        "Refresh enable.",
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
                    name: "rebl",
                    description: Some(
                        "Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prescale",
                    description: Some(
                        "Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles.",
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
                    name: "rt",
                    description: Some(
                        "Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period.",
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
                    name: "ut",
                    description: Some(
                        "Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period.",
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
            name: "Srctrl0",
            extends: None,
            description: Some(
                "SRAM control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "portsz",
                    description: Some(
                        "port size 0b - 8bit 1b - 16bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "SramPortSize",
                    ),
                },
                Field {
                    name: "adm",
                    description: Some(
                        "address data mode 00b - address and data MUX mode 11b - address and data non-MUX mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "AddressDataMux",
                    ),
                },
                Field {
                    name: "advp",
                    description: Some(
                        "ADV polarity 0b - ADV is active low 1b - ADV is active high.",
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
                    name: "advh",
                    description: Some(
                        "ADV hold state 0b - ADV is high during address hold state 1b - ADV is low during address hold state.",
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
            name: "Srctrl1",
            extends: None,
            description: Some(
                "SRAM control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ces",
                    description: Some(
                        "Chip enable setup time, is CES+1 clock cycles.",
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
                    name: "ceh",
                    description: Some(
                        "Chip enable hold time, is CEH+1 clock cycles.",
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
                    name: "as_",
                    description: Some(
                        "Address setup time, is AS+1 clock cycles.",
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
                    name: "ah",
                    description: Some(
                        "Address hold time, is AH+1 clock cycles.",
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
                Field {
                    name: "wel",
                    description: Some(
                        "WE low time, is WEL+1 clock cycles.",
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
                    name: "weh",
                    description: Some(
                        "WE high time, is WEH+1 clock cycles.",
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
                    name: "oel",
                    description: Some(
                        "OE low time, is OEL+1 clock cycles.",
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
                    name: "oeh",
                    description: Some(
                        "OE high time, is OEH+1 clock cycles.",
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
            name: "Stat0",
            extends: None,
            description: Some(
                "Status Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idle",
                    description: Some(
                        "Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access.",
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
    enums: &[
        Enum {
            name: "AddressDataMux",
            description: Some(
                "address data mode.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MUX",
                    description: Some(
                        "address and data MUX mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "address and data non-MUX mode",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Bank2Sel",
            description: Some(
                "2 Bank selection bit.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BANK_NUM_4",
                    description: Some(
                        "SDRAM device has 4 banks",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BANK_NUM_2",
                    description: Some(
                        "SDRAM device has 2 banks",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "BurstLen",
            description: Some(
                "Burst Length.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "_1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "_2",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "_4",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "_8",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cas",
            description: Some(
                "CAS Latency.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_1",
                    description: None,
                    value: 0,
                },
                EnumVariant {
                    name: "_2",
                    description: None,
                    value: 2,
                },
                EnumVariant {
                    name: "_3",
                    description: None,
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Cmd",
            description: Some(
                "SDRAM Commands.",
            ),
            bit_size: 16,
            variants: &[
                EnumVariant {
                    name: "READ",
                    description: Some(
                        "READ",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "WRITE",
                    description: Some(
                        "WRITE",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MODESET",
                    description: Some(
                        "MODESET",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "ACTIVE",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "AUTO_REFRESH",
                    description: Some(
                        "AUTO REFRESH",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "SELF_REFRESH",
                    description: Some(
                        "SELF REFRESH",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "PRECHARGE",
                    description: Some(
                        "PRECHARGE",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "PRECHARGE_ALL",
                    description: Some(
                        "PRECHARGE ALL",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "ColBits",
            description: Some(
                "Column address bit number.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_12BIT",
                    description: Some(
                        "12 bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_11BIT",
                    description: Some(
                        "11 bit",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_10BIT",
                    description: Some(
                        "10 bit",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "_9BIT",
                    description: Some(
                        "9 bit",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "DataSize",
            description: Some(
                "Data Size.",
            ),
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "_8BIT",
                    description: Some(
                        "4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_16BIT",
                    description: Some(
                        "1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "_24BIT",
                    description: Some(
                        "2",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "_32BIT",
                    description: Some(
                        "3",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "IoCsx",
            description: Some(
                "IO_CSX output selection.",
            ),
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "SDRAM_CS1",
                    description: Some(
                        "SDRAM CS1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SRAM_CE",
                    description: Some(
                        "SRAM CE#",
                    ),
                    value: 6,
                },
            ],
        },
        Enum {
            name: "MemorySize",
            description: Some(
                "Memory size.",
            ),
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "_4KB",
                    description: Some(
                        "4KB",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_8KB",
                    description: Some(
                        "8KB",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_16KB",
                    description: Some(
                        "16KB",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "_32KB",
                    description: Some(
                        "32KB",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "_64KB",
                    description: Some(
                        "64KB",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "_128KB",
                    description: Some(
                        "128KB",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "_256KB",
                    description: Some(
                        "256KB",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "_512KB",
                    description: Some(
                        "512KB",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "_1MB",
                    description: Some(
                        "1MB",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "_2MB",
                    description: Some(
                        "2MB",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "_4MB",
                    description: Some(
                        "4MB",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "_8MB",
                    description: Some(
                        "8MB",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "_16MB",
                    description: Some(
                        "16MB",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "_32MB",
                    description: Some(
                        "32MB",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "_64MB",
                    description: Some(
                        "64MB",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "_128MB",
                    description: Some(
                        "128MB",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "_256MB",
                    description: Some(
                        "256MB",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "_512MB",
                    description: Some(
                        "512MB",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "_1GB",
                    description: Some(
                        "1GB",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "_2GB",
                    description: Some(
                        "2GB",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "_4GB",
                    description: Some(
                        "4GB",
                    ),
                    value: 20,
                },
            ],
        },
        Enum {
            name: "SdramPortSize",
            description: Some(
                "Port Size.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "_8BIT",
                    description: Some(
                        "8bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_16BIT",
                    description: Some(
                        "16bit",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "_32BIT",
                    description: Some(
                        "32bit",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "SramPortSize",
            description: Some(
                "port size.",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "_8BIT",
                    description: Some(
                        "8bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "_16BIT",
                    description: Some(
                        "16bit",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
