use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mcan",
            extends: None,
            description: Some(
                "MCAN0.",
            ),
            items: &[
                BlockItem {
                    name: "endn",
                    description: Some(
                        "endian register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Endn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbtp",
                    description: Some(
                        "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test",
                    description: Some(
                        "test register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Test",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwd",
                    description: Some(
                        "ram watchdog.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cccr",
                    description: Some(
                        "CC control register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nbtp",
                    description: Some(
                        "nominal bit timing and prescaler register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nbtp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscc",
                    description: Some(
                        "timestamp counter configuration.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscv",
                    description: Some(
                        "timestamp counter value.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocc",
                    description: Some(
                        "timeout counter configuration.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tocv",
                    description: Some(
                        "timeout counter value.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tocv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ecr",
                    description: Some(
                        "error counter register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psr",
                    description: Some(
                        "protocol status register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Psr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdcr",
                    description: Some(
                        "transmitter delay compensation.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ir",
                    description: Some(
                        "interrupt register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ie",
                    description: Some(
                        "interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ils",
                    description: Some(
                        "interrupt line select.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ils",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ile",
                    description: Some(
                        "interrupt line enable.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ile",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gfc",
                    description: Some(
                        "global filter configuration.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sidfc",
                    description: Some(
                        "standard ID filter configuration.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sidfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xidfc",
                    description: Some(
                        "extended ID filter configuration.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xidfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xidam",
                    description: Some(
                        "extended id and mask.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xidam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpms",
                    description: Some(
                        "high priority message status.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpms",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ndat1",
                    description: Some(
                        "new data1.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ndat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ndat2",
                    description: Some(
                        "new data2.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ndat2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf0c",
                    description: Some(
                        "rx fifo 0 configuration.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf0c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf0s",
                    description: Some(
                        "rx fifo 0 status.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf0s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf0a",
                    description: Some(
                        "rx fifo0 acknowledge.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf0a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxbc",
                    description: Some(
                        "rx buffer configuration.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxbc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf1c",
                    description: Some(
                        "rx fifo1 configuration.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf1c",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf1s",
                    description: Some(
                        "rx fifo1 status.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf1s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxf1a",
                    description: Some(
                        "rx fifo 1 acknowledge.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxf1a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxesc",
                    description: Some(
                        "rx buffer/fifo element size configuration.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxesc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbc",
                    description: Some(
                        "tx buffer configuration.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txfqs",
                    description: Some(
                        "tx fifo/queue status.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txfqs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txesc",
                    description: Some(
                        "tx buffer element size configuration.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txesc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbrp",
                    description: Some(
                        "tx buffer request pending.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbrp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbar",
                    description: Some(
                        "tx buffer add request.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcr",
                    description: Some(
                        "tx buffer cancellation request.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbto",
                    description: Some(
                        "tx buffer transmission occurred.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcf",
                    description: Some(
                        "tx buffer cancellation finished.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbtie",
                    description: Some(
                        "tx buffer transmission interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbtie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txbcie",
                    description: Some(
                        "tx buffer cancellation finished interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txbcie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefc",
                    description: Some(
                        "tx event fifo configuration.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefs",
                    description: Some(
                        "tx event fifo status.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txefa",
                    description: Some(
                        "tx event fifo acknowledge.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txefa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ts_sel",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crel",
                    description: Some(
                        "core release register.",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tscfg",
                    description: Some(
                        "timestamp configuration.",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tscfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tss1",
                    description: Some(
                        "timestamp status1.",
                    ),
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tss1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tss2",
                    description: Some(
                        "timestamp status2.",
                    ),
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tss2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "atb",
                    description: Some(
                        "actual timebase.",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Atb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "atbh",
                    description: Some(
                        "actual timebase high.",
                    ),
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Atbh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glb_ctl",
                    description: Some(
                        "global control.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GlbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glb_status",
                    description: Some(
                        "global status.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GlbStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "message_buff",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 640,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MessageBuff",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Atb",
            extends: None,
            description: Some(
                "actual timebase.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tb",
                    description: Some(
                        "timebase for timestamp generation 31-0.",
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
            name: "Atbh",
            extends: None,
            description: Some(
                "actual timebase high.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbh",
                    description: Some(
                        "timebase for timestamp generation 63-32.",
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
            name: "Cccr",
            extends: None,
            description: Some(
                "CC control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "init",
                    description: Some(
                        "Initialization 0= Normal Operation 1= Initialization is started.",
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
                    name: "cce",
                    description: Some(
                        "Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = ‘1’).",
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
                    name: "asm",
                    description: Some(
                        "Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active.",
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
                        "Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_CAN may be set in power down by stopping m_can_hclk and m_can_cclk.",
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
                    name: "csr",
                    description: Some(
                        "Clock Stop Request 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle.",
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
                    name: "mon",
                    description: Some(
                        "Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to ‘1’. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled.",
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
                    name: "dar",
                    description: Some(
                        "Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled.",
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
                    name: "test",
                    description: Some(
                        "Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled.",
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
                    name: "fdoe",
                    description: Some(
                        "FD Operation Enable 0= FD operation disabled 1= FD operation enabled.",
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
                    name: "brse",
                    description: Some(
                        "Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled Note: When CAN FD operation is disabled FDOE = ‘0’, BRSE is not evaluated.",
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
                    name: "utsu",
                    description: Some(
                        "Use Timestamping Unit When UTSU is set, 16-bit Wide Message Markers are also enabled regardless of the value of WMM. 0= Internal time stamping 1= External time stamping by TSU Note: When generic parameter connected_tsu_g = ‘0’, there is no TSU connected to the M_CAN. In this case bit UTSU is fixed to zero by synthesis.",
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
                    name: "wmm",
                    description: Some(
                        "Wide Message Marker Enables the use of 16-bit Wide Message Markers. When 16-bit Wide Message Markers are used (WMM = ‘1’), 16-bit internal timestamping is disabled for the Tx Event FIFO. 0= 8-bit Message Marker used 1= 16-bit Message Marker used, replacing 16-bit timestamps in Tx Event FIFO.",
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
                    name: "pxhd",
                    description: Some(
                        "Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled Note: When protocol exception handling is disabled, the M_CAN will transmit an error frame when it detects a protocol exception condition.",
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
                    name: "efbi",
                    description: Some(
                        "Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization.",
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
                    name: "txp",
                    description: Some(
                        "Transmit Pause If this bit is set, the M_CAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled.",
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
                    name: "niso",
                    description: Some(
                        "Non ISO Operation If this bit is set, the M_CAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 Note: When the generic parameter iso_only_g is set to ‘1’ in hardware synthesis, this bit becomes reserved and is read as ‘0’. The M_CAN always operates with the CAN FD frame format according to ISO 11898-1:2015.",
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
            name: "Crel",
            extends: None,
            description: Some(
                "core release register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "day",
                    description: Some(
                        "Timestamp Day Two digits, BCD-coded. This field is set by generic parameter on synthesis.",
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
                    name: "mon",
                    description: Some(
                        "Timestamp Month Two digits, BCD-coded. This field is set by generic parameter on synthesis.",
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
                    name: "year",
                    description: Some(
                        "Timestamp Year One digit, BCD-coded. This field is set by generic parameter on synthesis.",
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
                    name: "substep",
                    description: Some(
                        "Sub-step of Core Release One digit, BCD-coded.",
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
                    name: "step",
                    description: Some(
                        "Step of Core Release One digit, BCD-coded.",
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
                    name: "rel",
                    description: Some(
                        "Core Release One digit, BCD-coded.",
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
            name: "Dbtp",
            extends: None,
            description: Some(
                "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsjw",
                    description: Some(
                        "Data (Re)Synchronization Jump Width Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.",
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
                    name: "dtseg2",
                    description: Some(
                        "Data time segment after sample point Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.",
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
                    name: "dtseg1",
                    description: Some(
                        "Data time segment before sample point Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.",
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
                    name: "dbrp",
                    description: Some(
                        "Data Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. When TDC = ‘1’, the range is limited to 0,1. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.",
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
                    name: "tdc",
                    description: Some(
                        "transmitter delay compensation enable 0= Transmitter Delay Compensation disabled 1= Transmitter Delay Compensation enabled.",
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
            name: "Ecr",
            extends: None,
            description: Some(
                "error counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tec",
                    description: Some(
                        "Transmit Error Counter Actual state of the Transmit Error Counter, values between 0 and 255 Note: When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented.",
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
                    name: "rec",
                    description: Some(
                        "Receive Error Counter Actual state of the Receive Error Counter, values between 0 and 127.",
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
                Field {
                    name: "rp",
                    description: Some(
                        "Receive Error Passive 0= The Receive Error Counter is below the error passive level of 128 1= The Receive Error Counter has reached the error passive level of 128.",
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
                    name: "cel",
                    description: Some(
                        "CAN Error Logging The counter is incremented each time when a CAN protocol error causes the 8-bit Transmit Error Counter TEC or the 7-bit Receive Error Counter REC to be incremented. The counter is also incremented when the Bus_Off limit is reached. It is not incremented when only RP is set without changing REC. The increment of CEL follows after the increment of REC or TEC. The counter is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR.ELO. Note: Byte access: Reading byte 2 will reset CEL to zero, reading bytes 3/1/0 has no impact.",
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
            name: "Endn",
            extends: None,
            description: Some(
                "endian register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evt",
                    description: Some(
                        "Endianness Test Value The endianness test value is 0x87654321.",
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
            name: "Gfc",
            extends: None,
            description: Some(
                "global filter configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rrfe",
                    description: Some(
                        "Reject Remote Frames Extended 0= Filter remote frames with 29-bit extended IDs 1= Reject all remote frames with 29-bit extended IDs.",
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
                    name: "rrfs",
                    description: Some(
                        "Reject Remote Frames Standard 0= Filter remote frames with 11-bit standard IDs 1= Reject all remote frames with 11-bit standard IDs.",
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
                    name: "anfe",
                    description: Some(
                        "Accept Non-matching Frames Extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject.",
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
                    name: "anfs",
                    description: Some(
                        "Accept Non-matching Frames Standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00= Accept in Rx FIFO 0 01= Accept in Rx FIFO 1 10= Reject 11= Reject.",
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
            ],
        },
        FieldSet {
            name: "GlbCtl",
            extends: None,
            description: Some(
                "global control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsu_tbin_sel",
                    description: Some(
                        "external timestamp select. each CAN block has 4 timestamp input, this register is used to select one of them as timestame if TSCFG.TBCS is set to 1.",
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
                    name: "stby_pol",
                    description: Some(
                        "standby polarity selection.",
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
                    name: "stby_clr_en",
                    description: Some(
                        "m_can standby clear control 0:controlled by software by standby bit[bit31] 1:auto clear standby by hardware when rx data is 0.",
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
                    name: "m_can_stby",
                    description: Some(
                        "m_can standby control.",
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
            name: "GlbStatus",
            extends: None,
            description: Some(
                "global status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "m_can_int0",
                    description: Some(
                        "m_can interrupt status0.",
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
                    name: "m_can_int1",
                    description: Some(
                        "m_can interrupt status1.",
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
            name: "Hpms",
            extends: None,
            description: Some(
                "high priority message status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidx",
                    description: Some(
                        "Buffer Index Index of Rx FIFO element to which the message was stored. Only valid when MSI[1] = ‘1’.",
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
                    name: "msi",
                    description: Some(
                        "Message Storage Indicator 00= No FIFO selected 01= FIFO message lost 10= Message stored in FIFO 0 11= Message stored in FIFO 1.",
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
                    name: "fidx",
                    description: Some(
                        "Filter Index Index of matching filter element. Range is 0 to SIDFC.LSS - 1 resp. XIDFC.LSE - 1.",
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
                Field {
                    name: "flst",
                    description: Some(
                        "Filter List Indicates the filter list of the matching filter element. 0= Standard Filter List 1= Extended Filter List.",
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
            name: "Ie",
            extends: None,
            description: Some(
                "interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rf0ne",
                    description: Some(
                        "Rx FIFO 0 New Message Interrupt Enable.",
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
                    name: "rf0we",
                    description: Some(
                        "Rx FIFO 0 Watermark Reached Interrupt Enable.",
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
                    name: "rf0fe",
                    description: Some(
                        "Rx FIFO 0 Full Interrupt Enable.",
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
                    name: "rf0le",
                    description: Some(
                        "Rx FIFO 0 Message Lost Interrupt Enable.",
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
                    name: "rf1ne",
                    description: Some(
                        "Rx FIFO 1 New Message Interrupt Enable.",
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
                    name: "rf1we",
                    description: Some(
                        "Rx FIFO 1 Watermark Reached Interrupt Enable.",
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
                    name: "rf1fe",
                    description: Some(
                        "Rx FIFO 1 Full Interrupt Enable.",
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
                    name: "rf1le",
                    description: Some(
                        "Rx FIFO 1 Message Lost Interrupt Enable.",
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
                    name: "hpme",
                    description: Some(
                        "High Priority Message Interrupt Enable.",
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
                    name: "tce",
                    description: Some(
                        "Transmission Completed Interrupt Enable.",
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
                    name: "tcfe",
                    description: Some(
                        "Transmission Cancellation Finished Interrupt Enable.",
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
                    name: "tfee",
                    description: Some(
                        "Tx FIFO Empty Interrupt Enable.",
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
                    name: "tefne",
                    description: Some(
                        "Tx Event FIFO New Entry Interrupt Enable.",
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
                    name: "tefwe",
                    description: Some(
                        "Tx Event FIFO Watermark Reached Interrupt Enable.",
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
                    name: "teffe",
                    description: Some(
                        "Tx Event FIFO Full Interrupt Enable.",
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
                    name: "tefle",
                    description: Some(
                        "Tx Event FIFO Event Lost Interrupt Enable.",
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
                    name: "tswe",
                    description: Some(
                        "Timestamp Wraparound Interrupt Enable.",
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
                    name: "mrafe",
                    description: Some(
                        "Message RAM Access Failure Interrupt Enable.",
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
                    name: "tooe",
                    description: Some(
                        "Timeout Occurred Interrupt Enable.",
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
                    name: "drxe",
                    description: Some(
                        "Message stored to Dedicated Rx Buffer Interrupt Enable.",
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
                    name: "bece",
                    description: Some(
                        "Bit Error Corrected Interrupt Enable.",
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
                    name: "beue",
                    description: Some(
                        "Bit Error Uncorrected Interrupt Enable.",
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
                    name: "eloe",
                    description: Some(
                        "Error Logging Overflow Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epe",
                    description: Some(
                        "Error Passive Interrupt Enable.",
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
                    name: "ewe",
                    description: Some(
                        "Warning Status Interrupt Enable.",
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
                    name: "boe",
                    description: Some(
                        "Bus_Off Status Interrupt Enable.",
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
                    name: "wdie",
                    description: Some(
                        "Watchdog Interrupt Enable.",
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
                    name: "peae",
                    description: Some(
                        "Protocol Error in Arbitration Phase Enable.",
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
                    name: "pede",
                    description: Some(
                        "Protocol Error in Data Phase Enable.",
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
                    name: "arae",
                    description: Some(
                        "Access to Reserved Address Enable.",
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
            name: "Ile",
            extends: None,
            description: Some(
                "interrupt line enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eint0",
                    description: Some(
                        "Enable Interrupt Line 0 0= Interrupt line m_can_int0 disabled 1= Interrupt line m_can_int0 enabled.",
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
                    name: "eint1",
                    description: Some(
                        "Enable Interrupt Line 1 0= Interrupt line m_can_int1 disabled 1= Interrupt line m_can_int1 enabled.",
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
            name: "Ils",
            extends: None,
            description: Some(
                "interrupt line select.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rf0nl",
                    description: Some(
                        "Rx FIFO 0 New Message Interrupt Line.",
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
                    name: "rf0wl",
                    description: Some(
                        "Rx FIFO 0 Watermark Reached Interrupt Line.",
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
                    name: "rf0fl",
                    description: Some(
                        "Rx FIFO 0 Full Interrupt Line.",
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
                    name: "rf0ll",
                    description: Some(
                        "Rx FIFO 0 Message Lost Interrupt Line.",
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
                    name: "rf1nl",
                    description: Some(
                        "Rx FIFO 1 New Message Interrupt Line.",
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
                    name: "rf1wl",
                    description: Some(
                        "Rx FIFO 1 Watermark Reached Interrupt Line.",
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
                    name: "rf1fl",
                    description: Some(
                        "Rx FIFO 1 Full Interrupt Line.",
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
                    name: "rf1ll",
                    description: Some(
                        "Rx FIFO 1 Message Lost Interrupt Line.",
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
                    name: "hpml",
                    description: Some(
                        "High Priority Message Interrupt Line.",
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
                    name: "tcl",
                    description: Some(
                        "Transmission Completed Interrupt Line.",
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
                    name: "tcfl",
                    description: Some(
                        "Transmission Cancellation Finished Interrupt Line.",
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
                    name: "tfel",
                    description: Some(
                        "Tx FIFO Empty Interrupt Line.",
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
                    name: "tefnl",
                    description: Some(
                        "Tx Event FIFO New Entry Interrupt Line.",
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
                    name: "tefwl",
                    description: Some(
                        "Tx Event FIFO Watermark Reached Interrupt Line.",
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
                    name: "teffl",
                    description: Some(
                        "Tx Event FIFO Full Interrupt Line.",
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
                    name: "tefll",
                    description: Some(
                        "Tx Event FIFO Event Lost Interrupt Line.",
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
                    name: "tswl",
                    description: Some(
                        "Timestamp Wraparound Interrupt Line.",
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
                    name: "mrafl",
                    description: Some(
                        "Message RAM Access Failure Interrupt Line.",
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
                    name: "tool",
                    description: Some(
                        "Timeout Occurred Interrupt Line.",
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
                    name: "drxl",
                    description: Some(
                        "Message stored to Dedicated Rx Buffer Interrupt Line.",
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
                    name: "becl",
                    description: Some(
                        "Bit Error Corrected Interrupt Line.",
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
                    name: "beul",
                    description: Some(
                        "Bit Error Uncorrected Interrupt Line.",
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
                    name: "elol",
                    description: Some(
                        "Error Logging Overflow Interrupt Line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epl",
                    description: Some(
                        "Error Passive Interrupt Line.",
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
                    name: "ewl",
                    description: Some(
                        "Warning Status Interrupt Line.",
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
                    name: "bol",
                    description: Some(
                        "Bus_Off Status Interrupt Line.",
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
                    name: "wdil",
                    description: Some(
                        "Watchdog Interrupt Line.",
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
                    name: "peal",
                    description: Some(
                        "Protocol Error in Arbitration Phase Line.",
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
                    name: "pedl",
                    description: Some(
                        "Protocol Error in Data Phase Line.",
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
                    name: "aral",
                    description: Some(
                        "Access to Reserved Address Line.",
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
            name: "Ir",
            extends: None,
            description: Some(
                "interrupt register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rf0n",
                    description: Some(
                        "Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0.",
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
                    name: "rf0w",
                    description: Some(
                        "Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark.",
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
                    name: "rf0f",
                    description: Some(
                        "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full.",
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
                    name: "rf0l",
                    description: Some(
                        "Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero.",
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
                    name: "rf1n",
                    description: Some(
                        "Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1.",
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
                    name: "rf1w",
                    description: Some(
                        "Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark.",
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
                    name: "rf1f",
                    description: Some(
                        "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full.",
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
                    name: "rf1l",
                    description: Some(
                        "Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero.",
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
                    name: "hpm",
                    description: Some(
                        "High Priority Message 0= No high priority message received 1= High priority message received.",
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
                    name: "tc",
                    description: Some(
                        "Transmission Completed 0= No transmission completed 1= Transmission completed.",
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
                    name: "tcf",
                    description: Some(
                        "Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished.",
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
                    name: "tfe",
                    description: Some(
                        "Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty.",
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
                    name: "tefn",
                    description: Some(
                        "Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element.",
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
                    name: "tefw",
                    description: Some(
                        "Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark.",
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
                    name: "teff",
                    description: Some(
                        "Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full.",
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
                    name: "tefl",
                    description: Some(
                        "Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero.",
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
                    name: "tsw",
                    description: Some(
                        "Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around.",
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
                    name: "mraf",
                    description: Some(
                        "Message RAM Access Failure The flag is set, when the Rx Handler .has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. .was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_CAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred.",
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
                    name: "too",
                    description: Some(
                        "Timeout Occurred 0= No timeout 1= Timeout reached.",
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
                    name: "drx",
                    description: Some(
                        "Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into an Rx Buffer.",
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
                    name: "bec",
                    description: Some(
                        "Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_can_aeim_berr[0] generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC).",
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
                    name: "beu",
                    description: Some(
                        "Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_can_aeim_berr[1] generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to ‘1’. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic).",
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
                    name: "elo",
                    description: Some(
                        "Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep",
                    description: Some(
                        "Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed.",
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
                    name: "ew",
                    description: Some(
                        "Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed.",
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
                    name: "bo",
                    description: Some(
                        "Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed.",
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
                    name: "wdi",
                    description: Some(
                        "Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY.",
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
                    name: "pea",
                    description: Some(
                        "Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC ≠ 0,7).",
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
                    name: "ped",
                    description: Some(
                        "Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC ≠ 0,7).",
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
                    name: "ara",
                    description: Some(
                        "Access to Reserved Address 0= No access to reserved address occurred 1= Access to reserved address occurred.",
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
            name: "MessageBuff",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "m_can message buffer.",
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
            name: "Nbtp",
            extends: None,
            description: Some(
                "nominal bit timing and prescaler register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ntseg2",
                    description: Some(
                        "Nominal Time segment after sample point Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.",
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
                    name: "ntseg1",
                    description: Some(
                        "Nominal Time segment before sample point Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used.",
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
                    name: "nbrp",
                    description: Some(
                        "Nominal Bit Rate Prescaler The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nsjw",
                    description: Some(
                        "Nominal (Re)Synchronization Jump Width Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ndat1",
            extends: None,
            description: Some(
                "new data1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nd1",
                    description: Some(
                        "New Data[31:0] The register holds the New Data flags of Rx Buffers 0 to 31. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them.A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message.",
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
            name: "Ndat2",
            extends: None,
            description: Some(
                "new data2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nd2",
                    description: Some(
                        "New Data[63:32] The register holds the New Data flags of Rx Buffers 32 to 63. The flags are set when the respective Rx Buffer has been updated from a received frame. The flags remain set until the Host clears them. A flag is cleared by writing a ’1’ to the corresponding bit position. Writing a ’0’ has no effect. A hard reset will clear the register. 0= Rx Buffer not updated 1= Rx Buffer updated from new message.",
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
            name: "Psr",
            extends: None,
            description: Some(
                "protocol status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lec",
                    description: Some(
                        "Last Error Code The LEC indicates the type of the last error to occur on the CAN bus. This field will be cleared to ‘0’when a message has been transferred (reception or transmission) without error. 0= No Error: No error occurred since LEC has been reset by successful reception or transmission. 1= Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed. 2= Form Error: A fixed format part of a received frame has the wrong format. 3= AckError: The message transmitted by the M_CAN was not acknowledged by another node. 4= Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value ‘1’), but the monitored bus value was dominant. 5= Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value‘0’), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed). 6= CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data. 7= NoChange: Any read access to the Protocol Status Register re-initializes the LEC to ‘7’. When the LEC shows the value ‘7’, no CAN bus event was detected since the last CPU read access to the Protocol Status Register. Note: When a frame in CAN FD format has reached the data phase with BRS flag set, the next CAN event (error or valid frame) will be shown in DLEC instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence will be shown as a Form Error, not Stuff Error. Note: The Bus_Off recovery sequence (see ISO 11898-1:2015) cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus_Off, it will set CCCR.INIT of its own accord,stopping all bus activities. Once CCCR.INIT has been cleared by the CPU, the device will then wait for 129 occurrences of Bus Idle (129 * 11 consecutive recessive bits) before resuming normal operation. At the end of the Bus_Off recovery sequence, the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT, each time a sequence of 11 recessive bits has been monitored, a Bit0Error code is written to PSR.LEC, enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus_Off recovery sequence. ECR.REC is used to count these sequences. Note: Byte access: Reading byte 0 will set LEC to “111”, reading bytes 3/2/1 has no impact.",
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
                    name: "act",
                    description: Some(
                        "Activity Monitors the module’s CAN communication state. 00= Synchronizing - node is synchronizing on CAN communication 01= Idle - node is neither receiver nor transmitter 10= Receiver - node is operating as receiver 11= Transmitter - node is operating as transmitter Note: ACT is set to “00” by a Protocol Exception Event.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ep",
                    description: Some(
                        "Error Passive 0= The M_CAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected 1= The M_CAN is in the Error_Passive state.",
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
                    name: "ew",
                    description: Some(
                        "Warning Status 0= Both error counters are below the Error_Warning limit of 96 1= At least one of error counter has reached the Error_Warning limit of 96.",
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
                    name: "bo",
                    description: Some(
                        "Bus_Off Status 0= The M_CAN is not Bus_Off 1= The M_CAN is in Bus_Off state.",
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
                    name: "dlec",
                    description: Some(
                        "Data Phase Last Error Code Type of last error that occurred in the data phase of a CAN FD format frame with its BRS flag set.Coding is the same as for LEC. This field will be cleared to zero when a CAN FD format frame with its BRS flag set has been transferred (reception or transmission) without error. Note: Byte access: Reading byte 0 will set DLEC to “111”, reading bytes 3/2/1 has no impact.",
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
                    name: "resi",
                    description: Some(
                        "ESI flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its ESI flag set 1= Last received CAN FD message had its ESI flag set Note: Byte access: Reading byte 0 will reset RESI, reading bytes 3/2/1 has no impact.",
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
                    name: "rbrs",
                    description: Some(
                        "BRS flag of last received CAN FD Message This bit is set together with RFDF, independent of acceptance filtering. 0= Last received CAN FD message did not have its BRS flag set 1= Last received CAN FD message had its BRS flag set Note: Byte access: Reading byte 0 will reset RBRS, reading bytes 3/2/1 has no impact.",
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
                    name: "rfdf",
                    description: Some(
                        "Received a CAN FD Message This bit is set independent of acceptance filtering. 0= Since this bit was reset by the CPU, no CAN FD message has been received 1= Message in CAN FD format with FDF flag set has been received Note: Byte access: Reading byte 0 will reset RFDF, reading bytes 3/2/1 has no impact.",
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
                    name: "pxe",
                    description: Some(
                        "Protocol Exception Event 0= No protocol exception event occurred since last read access 1= Protocol exception event occurred Note: Byte access: Reading byte 0 will reset PXE, reading bytes 3/2/1 has no impact.",
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
                    name: "tdcv",
                    description: Some(
                        "Transmitter Delay Compensation Value Position of the secondary sample point, defined by the sum of the measured delay from m_can_tx to m_can_rx and TDCR.TDCO. The SSP position is, in the data phase, the number of mtq between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwd",
            extends: None,
            description: Some(
                "ram watchdog.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdc",
                    description: Some(
                        "Watchdog Configuration Start value of the Message RAM Watchdog Counter. With the reset value of “00” the counter is disabled.",
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
                    name: "wdv",
                    description: Some(
                        "Watchdog Value Actual Message RAM Watchdog Counter Value.",
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
            name: "Rxbc",
            extends: None,
            description: Some(
                "rx buffer configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rbsa",
                    description: Some(
                        "Rx Buffer Start Address Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address).Also used to reference debug messages A,B,C.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxesc",
            extends: None,
            description: Some(
                "rx buffer/fifo element size configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f0ds",
                    description: Some(
                        "Rx FIFO 0 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO, only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame’s data field is ignored.",
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
                    name: "f1ds",
                    description: Some(
                        "Rx FIFO 1 Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rbds",
                    description: Some(
                        "Rx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field.",
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
            name: "Rxf0a",
            extends: None,
            description: Some(
                "rx fifo0 acknowledge.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f0ai",
                    description: Some(
                        "Rx FIFO 0 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL.",
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
            name: "Rxf0c",
            extends: None,
            description: Some(
                "rx fifo 0 configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f0sa",
                    description: Some(
                        "Rx FIFO 0 Start Address Start address of Rx FIFO 0 in Message RAM (32-bit word address).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0s",
                    description: Some(
                        "Rx FIFO 0 Size 0= No Rx FIFO 0 1-64= Number of Rx FIFO 0 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 0 elements are indexed from 0 to F0S-1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0wm",
                    description: Some(
                        "Rx FIFO 0 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 0 watermark interrupt (IR.RF0W) >64= Watermark interrupt disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0om",
                    description: Some(
                        "FIFO 0 Operation Mode FIFO 0 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 0 blocking mode 1= FIFO 0 overwrite mode.",
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
            name: "Rxf0s",
            extends: None,
            description: Some(
                "rx fifo 0 status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f0fl",
                    description: Some(
                        "Rx FIFO 0 Fill Level Number of elements stored in Rx FIFO 0, range 0 to 64.",
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
                    name: "f0gi",
                    description: Some(
                        "Rx FIFO 0 Get Index Rx FIFO 0 read index pointer, range 0 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0pi",
                    description: Some(
                        "Rx FIFO 0 Put Index Rx FIFO 0 write index pointer, range 0 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f0f",
                    description: Some(
                        "Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full.",
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
                    name: "rf0l",
                    description: Some(
                        "Rx FIFO 0 Message Lost This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = ‘1’ will not set this flag.",
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
            ],
        },
        FieldSet {
            name: "Rxf1a",
            extends: None,
            description: Some(
                "rx fifo 1 acknowledge.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f1ai",
                    description: Some(
                        "Rx FIFO 1 Acknowledge Index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL.",
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
            name: "Rxf1c",
            extends: None,
            description: Some(
                "rx fifo1 configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f1sa",
                    description: Some(
                        "Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1s",
                    description: Some(
                        "Rx FIFO 1 Size 0= No Rx FIFO 1 1-64= Number of Rx FIFO 1 elements >64= Values greater than 64 are interpreted as 64 The Rx FIFO 1 elements are indexed from 0 to F1S - 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1wm",
                    description: Some(
                        "Rx FIFO 1 Watermark 0= Watermark interrupt disabled 1-64= Level for Rx FIFO 1 watermark interrupt (IR.RF1W) >64= Watermark interrupt disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1om",
                    description: Some(
                        "FIFO 1 Operation Mode FIFO 1 can be operated in blocking or in overwrite mode (see Section 3.4.2). 0= FIFO 1 blocking mode 1= FIFO 1 overwrite mode.",
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
            name: "Rxf1s",
            extends: None,
            description: Some(
                "rx fifo1 status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f1fl",
                    description: Some(
                        "Rx FIFO 1 Fill Level Number of elements stored in Rx FIFO 1, range 0 to 64.",
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
                    name: "f1gi",
                    description: Some(
                        "Rx FIFO 1 Get Index Rx FIFO 1 read index pointer, range 0 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1pi",
                    description: Some(
                        "Rx FIFO 1 Put Index Rx FIFO 1 write index pointer, range 0 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "f1f",
                    description: Some(
                        "Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full.",
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
                    name: "rf1l",
                    description: Some(
                        "Rx FIFO 1 Message Lost This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = ‘1’ will not set this flag.",
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
                    name: "dms",
                    description: Some(
                        "Debug Message Status 00= Idle state, wait for reception of debug messages, DMA request is cleared 01= Debug message A received 10= Debug messages A, B received 11= Debug messages A, B, C received, DMA request is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sidfc",
            extends: None,
            description: Some(
                "standard ID filter configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flssa",
                    description: Some(
                        "Filter List Standard Start Address Start address of standard Message ID filter list (32-bit word address).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lss",
                    description: Some(
                        "List Size Standard 0= No standard Message ID filter 1-128= Number of standard Message ID filter elements >128= Values greater than 128 are interpreted as 128.",
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
            name: "Tdcr",
            extends: None,
            description: Some(
                "transmitter delay compensation.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdcf",
                    description: Some(
                        "Transmitter Delay Compensation Filter Window Length Defines the minimum value for the SSP position, dominant edges on m_can_rx that would result in an earlier SSP position are ignored for transmitter delay measurement. The feature is enabled when TDCF is configured to a value greater than TDCO. Valid values are 0 to 127 mtq.",
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
                    name: "tdco",
                    description: Some(
                        "Transmitter Delay Compensation SSP Offset Offset value defining the distance between the measured delay from m_can_tx to m_can_rx and the secondary sample point. Valid values are 0 to 127 mtq.",
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
            name: "Test",
            extends: None,
            description: Some(
                "test register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lbck",
                    description: Some(
                        "Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled.",
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
                    name: "tx",
                    description: Some(
                        "Control of Transmit Pin 00 Reset value, m_can_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_can_tx 10 Dominant (‘0’) level at pin m_can_tx 11 Recessive (‘1’) at pin m_can_tx.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rx",
                    description: Some(
                        "Receive Pin Monitors the actual value of pin m_can_rx 0= The CAN bus is dominant (m_can_rx = ‘0’) 1= The CAN bus is recessive (m_can_rx = ‘1’).",
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
                    name: "txbnp",
                    description: Some(
                        "Tx Buffer Number Prepared Tx Buffer number of message that is ready for transmission. Valid when PVAL is set.Valid values are 0 to 31.",
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
                    name: "pval",
                    description: Some(
                        "Prepared Valid 0= Value of TXBNP not valid 1= Value of TXBNP valid.",
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
                    name: "txbns",
                    description: Some(
                        "Tx Buffer Number Started Tx Buffer number of message whose transmission was started last. Valid when SVAL is set. Valid values are 0 to 31.",
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
                    name: "sval",
                    description: Some(
                        "Started Valid 0= Value of TXBNS not valid 1= Value of TXBNS valid.",
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
            ],
        },
        FieldSet {
            name: "Tocc",
            extends: None,
            description: Some(
                "timeout counter configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rp",
                    description: Some(
                        "Enable Timeout Counter 0= Timeout Counter disabled 1= Timeout Counter enabled.",
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
                    name: "tos",
                    description: Some(
                        "Timeout Select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down-counting. When the Timeout Counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC.TOP. Down-counting is started when the first FIFO element is stored. 00= Continuous operation 01= Timeout controlled by Tx Event FIFO 10= Timeout controlled by Rx FIFO 0 11= Timeout controlled by Rx FIFO 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "top",
                    description: Some(
                        "Timeout Period Start value of the Timeout Counter (down-counter). Configures the Timeout Period.",
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
            name: "Tocv",
            extends: None,
            description: Some(
                "timeout counter value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toc",
                    description: Some(
                        "Timeout Counter The Timeout Counter is decremented in multiples of CAN bit times [1…16] depending on the configuration of TSCC.TCP. When decremented to zero, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS. Note: Byte access: when TOCC.TOS = “00，writing one of the register bytes 3/2/1/0 will preset the Timeout Counter.",
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
            name: "TsSel",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts",
                    description: Some(
                        "Timestamp Word TS default can save 16 timestamps with 32bit; if ts64_en is set, then work at 64bit mode, can save 8 timestamps with 01/23/45….",
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
            name: "Tscc",
            extends: None,
            description: Some(
                "timestamp counter configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tss",
                    description: Some(
                        "timestamp Select 00= Timestamp counter value always 0x0000 01= Timestamp counter value incremented according to TCP 10= External timestamp counter value used 11= Same as “00”.",
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
                    name: "tcp",
                    description: Some(
                        "Timestamp Counter Prescaler Configures the timestamp and timeout counters time unit in multiples of CAN bit times [1…16]. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used.",
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
            ],
        },
        FieldSet {
            name: "Tscfg",
            extends: None,
            description: Some(
                "timestamp configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsue",
                    description: Some(
                        "Timestamp Unit Enable 0: TSU disabled 1: TSU enabled.",
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
                    name: "tbcs",
                    description: Some(
                        "Timebase Counter Select When the internal timebase is excluded by synthesis, TBCS is fixed to ‘1’. 0: Timestamp value captured from internal timebase counter, ATB.TB[31:0] is the internal timbase counter 1: Timestamp value captured from input tsu_tbin[31:0],ATB.TB[31:0] is tsu_tbin[31:0].",
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
                    name: "scp",
                    description: Some(
                        "Select Capturing Position 0: Capture Timestamp at EOF 1: Capture Timestamp at SOF.",
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
                    name: "en64",
                    description: Some(
                        "set to use 64bit timestamp. when enabled, tsu can save up to 8 different timestamps, TS(k) and TS(k+1) are used for one 64bit timestamp, k is 0~7. TSP can be used to select different one.",
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
                    name: "tbpre",
                    description: Some(
                        "Timebase Prescaler 0x00 to 0xFF The value by which the oscillator frequency is divided for generating the timebase counter clock. Valid values for the Timebase Prescaler are 0 to 255. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Affects only the TSU internal timebase. When the internal timebase is excluded by synthesis, TBPRE[7:0] is fixed to 0x00, the Timestamp Prescaler is not used.",
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
            name: "Tscv",
            extends: None,
            description: Some(
                "timestamp counter value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsc",
                    description: Some(
                        "Timestamp Counter The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx).When TSCC.TSS = “01”, the Timestamp Counter is incremented in multiples of CAN bit times [1…16] depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = “10”, TSC reflects the external Timestamp Counter value. A write access has no impact.",
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
            name: "Tss1",
            extends: None,
            description: Some(
                "timestamp status1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsn",
                    description: Some(
                        "Timestamp New Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when a timestamp was stored in the related Timestamp register. Reading a Timestamp register resets the related bit.",
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
                    name: "tsl",
                    description: Some(
                        "Timestamp Lost Each Timestamp register (TS0-TS15) is assigned one bit. The bits are set when the timestamp stored in the related Timestamp register was overwritten before it was read. Reading a Timestamp register resets the related bit.",
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
            name: "Tss2",
            extends: None,
            description: Some(
                "timestamp status2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsp",
                    description: Some(
                        "Timestamp Pointer The Timestamp Pointer is incremented by one each time a timestamp is captured. From its maximum value (3, 7, or 15 depending on number_ts_g), it is incremented to 0. Value also signalled on output m_can_tsp[3:0].",
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
            name: "Txbar",
            extends: None,
            description: Some(
                "tx buffer add request.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ar",
                    description: Some(
                        "Add Request Each Tx Buffer has its own Add Request bit. Writing a ‘1’ will set the corresponding Add Request bit; writing a ‘0’ has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0= No transmission request added 1= Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored.",
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
            name: "Txbc",
            extends: None,
            description: Some(
                "tx buffer configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbsa",
                    description: Some(
                        "Tx Buffers Start Address Start address of Tx Buffers section in Message RAM (32-bit word address, see Figure 2). Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ndtb",
                    description: Some(
                        "Number of Dedicated Transmit Buffers 0= No Dedicated Tx Buffers 1-32= Number of Dedicated Tx Buffers >32= Values greater than 32 are interpreted as 32.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqs",
                    description: Some(
                        "Transmit FIFO/Queue Size 0= No Tx FIFO/Queue 1-32= Number of Tx Buffers used for Tx FIFO/Queue >32= Values greater than 32 are interpreted as 32.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfqm",
                    description: Some(
                        "Tx FIFO/Queue Mode 0= Tx FIFO operation 1= Tx Queue operation.",
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
            name: "Txbcf",
            extends: None,
            description: Some(
                "tx buffer cancellation finished.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cf",
                    description: Some(
                        "Cancellation Finished Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmit buffer cancellation 1= Transmit buffer cancellation finished.",
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
            name: "Txbcie",
            extends: None,
            description: Some(
                "tx buffer cancellation finished interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfie",
                    description: Some(
                        "Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled.",
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
            name: "Txbcr",
            extends: None,
            description: Some(
                "tx buffer cancellation request.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cr",
                    description: Some(
                        "Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending.",
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
            name: "Txbrp",
            extends: None,
            description: Some(
                "tx buffer request pending.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trp",
                    description: Some(
                        "Transmission Request Pending Each Tx Buffer has its own Transmission Request Pending bit. The bits are set via register TXBAR.The bits are reset after a requested transmission has completed or has been cancelled via register TXBCR. TXBRP bits are set only for those Tx Buffers configured via TXBC. After a TXBRP bit has been set, a Tx scan (see Section 3.5, Tx Handling) is started to check for the pending Tx request with the highest priority (Tx Buffer with lowest Message ID). A cancellation request resets the corresponding transmission request pending bit of register TXBRP. In case a transmission has already been started when a cancellation is requested, this is done at the end of the transmission, regardless whether the transmission was successful or not. The cancellation request bits are reset directly after the corresponding TXBRP bit has been reset. After a cancellation has been requested, a finished cancellation is signalled via TXBCF ? after successful transmission together with the corresponding TXBTO bit ? when the transmission has not yet been started at the point of cancellation ? when the transmission has been aborted due to lost arbitration ? when an error occurred during frame transmission In DAR mode all transmissions are automatically cancelled if they are not successful. The corresponding TXBCF bit is set for all unsuccessful transmissions. 0= No transmission request pending 1= Transmission request pending Note: TXBRP bits which are set while a Tx scan is in progress are not considered during this particular Tx scan. In case a cancellation is requested for such a Tx Buffer, this Add Request is cancelled immediately, the corresponding TXBRP bit is reset.",
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
            name: "Txbtie",
            extends: None,
            description: Some(
                "tx buffer transmission interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmission Interrupt Enable Each Tx Buffer has its own Transmission Interrupt Enable bit. 0= Transmission interrupt disabled 1= Transmission interrupt enable.",
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
            name: "Txbto",
            extends: None,
            description: Some(
                "tx buffer transmission occurred.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "to",
                    description: Some(
                        "Transmission Occurred Each Tx Buffer has its own Transmission Occurred bit. The bits are set when the corresponding TXBRP bit is cleared after a successful transmission. The bits are reset when a new transmission is requested by writing a ‘1’ to the corresponding bit of register TXBAR. 0= No transmission occurred 1= Transmission occurred.",
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
            name: "Txefa",
            extends: None,
            description: Some(
                "tx event fifo acknowledge.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efai",
                    description: Some(
                        "Event FIFO Acknowledge Index After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL.",
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
            name: "Txefc",
            extends: None,
            description: Some(
                "tx event fifo configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efsa",
                    description: Some(
                        "Event FIFO Start Address Start address of Tx Event FIFO in Message RAM (32-bit word address).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efs",
                    description: Some(
                        "Event FIFO Size 0= Tx Event FIFO disabled 1-32= Number of Tx Event FIFO elements >32= Values greater than 32 are interpreted as 32 The Tx Event FIFO elements are indexed from 0 to EFS - 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "efwm",
                    description: Some(
                        "Event FIFO Watermark 0= Watermark interrupt disabled 1-32= Level for Tx Event FIFO watermark interrupt (IR.TEFW) >32= Watermark interrupt disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Txefs",
            extends: None,
            description: Some(
                "tx event fifo status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "effl",
                    description: Some(
                        "Event FIFO Fill Level Number of elements stored in Tx Event FIFO, range 0 to 32.",
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
                    name: "efgi",
                    description: Some(
                        "Event FIFO Get Index Tx Event FIFO read index pointer, range 0 to 31.",
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
                    name: "efpi",
                    description: Some(
                        "Event FIFO Put Index Tx Event FIFO write index pointer, range 0 to 31.",
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
                    name: "eff",
                    description: Some(
                        "Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full.",
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
                    name: "tefl",
                    description: Some(
                        "Tx Event FIFO Element Lost This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero.",
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
            ],
        },
        FieldSet {
            name: "Txesc",
            extends: None,
            description: Some(
                "tx buffer element size configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbds",
                    description: Some(
                        "Tx Buffer Data Field Size 000= 8 byte data field 001= 12 byte data field 010= 16 byte data field 011= 20 byte data field 100= 24 byte data field 101= 32 byte data field 110= 48 byte data field 111= 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as “0xCC” (padding bytes).",
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
            name: "Txfqs",
            extends: None,
            description: Some(
                "tx fifo/queue status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tffl",
                    description: Some(
                        "Tx FIFO Free Level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’) Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO.",
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
                    name: "tfgi",
                    description: Some(
                        "Tx FIFO Get Index Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = ‘1’).",
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
                    name: "tfqpi",
                    description: Some(
                        "Tx FIFO/Queue Put Index Tx FIFO/Queue write index pointer, range 0 to 31.",
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
                    name: "tfqf",
                    description: Some(
                        "Tx FIFO/Queue Full 0= Tx FIFO/Queue not full 1= Tx FIFO/Queue full.",
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
            ],
        },
        FieldSet {
            name: "Xidam",
            extends: None,
            description: Some(
                "extended id and mask.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eidm",
                    description: Some(
                        "Extended ID Mask For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Xidfc",
            extends: None,
            description: Some(
                "extended ID filter configuration.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flesa",
                    description: Some(
                        "Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lse",
                    description: Some(
                        "List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
