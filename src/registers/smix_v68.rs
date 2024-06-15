use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "DmaCh",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "Channel N Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "burst_count",
                    description: Some(
                        "Channel N Source Total Beats Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BurstCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "src_addr",
                    description: Some(
                        "Channel N Source Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrcAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dst_addr",
                    description: Some(
                        "Channel N Destination Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llp",
                    description: Some(
                        "Channel N Linked List Pointer Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llp",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "DstCh",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "SMIX Dstination N Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gain",
                    description: Some(
                        "SMIX Dstination N Gain Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChGain",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bufsize",
                    description: Some(
                        "SMIX Dstination N Max Index Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bufsize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fadein",
                    description: Some(
                        "SMIX Dstination N Fade-In Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChFadein",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fadeout",
                    description: Some(
                        "SMIX Dstination N Fade-Out Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChFadeout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "SMIX Dstination N Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "SMIX Dstination N Data Out Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstChData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "source_en",
                    description: Some(
                        "SMIX Dstination N Source Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "source_act",
                    description: Some(
                        "SMIX Dstination N Source Activation Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceAct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "source_deact",
                    description: Some(
                        "SMIX Dstination N Source De-Activation Register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceDeact",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "source_fadein_ctrl",
                    description: Some(
                        "SMIX Dstination N Source Fade-in Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceFadeinCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "deact_st",
                    description: Some(
                        "SMIX Dstination N Source Deactivation Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DeactSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "source_mfadeout_ctrl",
                    description: Some(
                        "SMIX Dstination N Source Manual Fade-out Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceMfadeoutCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Smix",
            extends: None,
            description: Some(
                "SMIX.",
            ),
            items: &[
                BlockItem {
                    name: "dmac_id",
                    description: Some(
                        "DMAC_ID Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_tc_st",
                    description: Some(
                        "Transfer Complete Status.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacTcSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_abrt_st",
                    description: Some(
                        "Transfer Abort Status.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacAbrtSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_err_st",
                    description: Some(
                        "Transfer Error Status.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacErrSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_ctrl",
                    description: Some(
                        "Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_abrt_cmd",
                    description: Some(
                        "Abort Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacAbrtCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmac_chen",
                    description: Some(
                        "Channel Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmacChen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_ch",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 26,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "DmaCh",
                        },
                    ),
                },
                BlockItem {
                    name: "calsat_st",
                    description: Some(
                        "SMIX Cal Saturation Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CalsatSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fdot_done_st",
                    description: Some(
                        "SMIX Fade-Out Done Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FdotDoneSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data_st",
                    description: Some(
                        "SMIX Data Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DataSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dst_ch",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x840,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "DstCh",
                        },
                    ),
                },
                BlockItem {
                    name: "source_ch",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 14,
                                stride: 32,
                            },
                        ),
                    ),
                    byte_offset: 0x900,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "SourceCh",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "SourceCh",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "SMIX Source N Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gain",
                    description: Some(
                        "SMIX Source N Gain Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChGain",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fadein",
                    description: Some(
                        "SMIX Source N Fade-in Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChFadein",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fadeout",
                    description: Some(
                        "SMIX Source N Fade-out Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChFadeout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_size",
                    description: Some(
                        "SMIX Source N Buffer Size Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
                    description: Some(
                        "SMIX Source N Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "SMIX Source N Data Input Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SourceChData",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BufSize",
            extends: None,
            description: Some(
                "SMIX Source N Buffer Size Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maxidx",
                    description: Some(
                        "unit as 16-bits per sample. Zero means no length limit. = Act Len-1. The actual length is the up_rate*(input_data_length-4). If the filter processing is down-sampling, the value of up_rate above is 1. If the filter processing is up-sampling, the value of up_rate above is the up-sampling rate.",
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
            name: "Bufsize",
            extends: None,
            description: Some(
                "SMIX Dstination N Max Index Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_idx",
                    description: Some(
                        "The total length of the dst stream -1. If zero, means there is no end of the stream.",
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
            name: "BurstCount",
            extends: None,
            description: Some(
                "Channel N Source Total Beats Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "num",
                    description: Some(
                        "the total number of source beats.",
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
            name: "CalsatSt",
            extends: None,
            description: Some(
                "SMIX Cal Saturation Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src",
                    description: Some(
                        "SRC CAL_SAT_ERR. W1C.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dst",
                    description: Some(
                        "DST CAL_SAT_ERR. W1C.",
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
            name: "Ctl",
            extends: None,
            description: Some(
                "Channel N Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "channel enable bit.",
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
                    name: "tc_int_en",
                    description: Some(
                        "TC interrupt enable.",
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
                    name: "err_int_en",
                    description: Some(
                        "Err interrupt enable.",
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
                    name: "abrt_int_en",
                    description: Some(
                        "Abort interrupt enable.",
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
                    name: "dstaddrctrl",
                    description: Some(
                        "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception.",
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
                    name: "srcaddrctrl",
                    description: Some(
                        "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dstmode",
                    description: Some(
                        "DMA Destination handshake mode 0x0: Normal mode 0x1: Handshake mode.",
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
                    name: "srcmode",
                    description: Some(
                        "DMA Source handshake mode 0x0: Normal mode 0x1: Handshake mode.",
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
                    name: "dstwidth",
                    description: Some(
                        "Destination Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcwidth",
                    description: Some(
                        "Source Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcburstsize",
                    description: Some(
                        "0x0: 1 beat per transfer 0x1: 2 beats per transfer 0x2: 4 beats per transfer 0x3: 8 beats per transfer 0x4: 16 beats per transfer 0x5: 32 beats per transfer 0x6: 64 beats per transfer 0x7: 128 beats per transfer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "priority",
                    description: Some(
                        "0x0: Lower priority 0x1: Higher priority.",
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
                    name: "dstreqsel",
                    description: Some(
                        "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcreqsel",
                    description: Some(
                        "Source DMA request select. Select the request/ack handshake pair that the source device is connected to.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DataSt",
            extends: None,
            description: Some(
                "SMIX Data Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src_dn",
                    description: Some(
                        "SRC data needed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dst_undl",
                    description: Some(
                        "DST data underflow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dst_da",
                    description: Some(
                        "DST data available.",
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
            name: "DeactSt",
            extends: None,
            description: Some(
                "SMIX Dstination N Source Deactivation Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src_deact_st",
                    description: Some(
                        "Asserted when in de-active mode.",
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
                    name: "dst_deact",
                    description: Some(
                        "Asserted when in de-active mode.",
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
            name: "DmacAbrtCmd",
            extends: None,
            description: Some(
                "Abort Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Write 1 to force the corresponding channel into abort status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmacAbrtSt",
            extends: None,
            description: Some(
                "Transfer Abort Status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "The abort status is set when a channel transfer is aborted.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmacChen",
            extends: None,
            description: Some(
                "Channel Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Write 1 to enable the corresponding channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmacCtrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srst",
                    description: Some(
                        "Software Reset.",
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
            name: "DmacErrSt",
            extends: None,
            description: Some(
                "Transfer Error Status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "The error status is set when a channel transfer encounters the following error events: . Bus error . Unaligned address . Unaligned transfer width . Reserved configuration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmacId",
            extends: None,
            description: Some(
                "DMAC_ID Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rev",
                    description: Some(
                        "Revision.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmacTcSt",
            extends: None,
            description: Some(
                "Transfer Complete Status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "The terminal count status is set when a channel transfer finishes without abort or error events.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DstAddr",
            extends: None,
            description: Some(
                "Channel N Destination Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "destination address.",
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
            name: "DstChCtrl",
            extends: None,
            description: Some(
                "SMIX Dstination N Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mixer_en",
                    description: Some(
                        "mixer function enable.",
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
                    name: "softrst",
                    description: Some(
                        "Soft reset.",
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
                    name: "dst_en",
                    description: Some(
                        "Dst enabled. When disabled, clear the FIFO pointers.",
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
                    name: "dstfadin_en",
                    description: Some(
                        "FadeIn_Ctrl for destionation. Auto clear.",
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
                    name: "dstfadout_aen",
                    description: Some(
                        "Automatically FadeOut_Ctrl for destionation. Only effective after DST_AFADEOUT is assigned a non-zero value.",
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
                    name: "dstfadout_men",
                    description: Some(
                        "Manual FadeOut_Ctrl for destionation. Auto clear.",
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
                    name: "dst_act",
                    description: Some(
                        "activate the destination channel.",
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
                    name: "dst_deact",
                    description: Some(
                        "de-activate the destination channel.",
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
                    name: "fadeout_done_ie",
                    description: Some(
                        "Fade-Out interrupt enable.",
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
                    name: "adeactfadeout_en",
                    description: Some(
                        "AutoDeactAfterFadeOut_En: Asserted to enter de-activated mode after fade-out done.",
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
                    name: "da_int_en",
                    description: Some(
                        "Data Available IntEn.",
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
                    name: "calsat_int_en",
                    description: Some(
                        "Cal Saturation IntEn.",
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
                    name: "thrsh",
                    description: Some(
                        "FIFO threshold for DMA or Int. >= will generate req. Must be greater or equal than 8. The read burst of DMA should make the fillings in the buffer be greater than 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data_unfl_ie",
                    description: Some(
                        "Data Underflow Error IntEn.",
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
            name: "DstChData",
            extends: None,
            description: Some(
                "SMIX Dstination N Data Out Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Output data buffer.",
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
            name: "DstChFadein",
            extends: None,
            description: Some(
                "SMIX Dstination N Fade-In Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delta",
                    description: Some(
                        "Fade-in delta for linear fading in from 0 to 1 (about at most 20s for 48kHz sampled sound) (Using only top 14 bits for mul).",
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
            name: "DstChFadeout",
            extends: None,
            description: Some(
                "SMIX Dstination N Fade-Out Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delta",
                    description: Some(
                        "Fade out in 2^DELTA samples. Now DELTA can be at most 14。.",
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
            name: "DstChGain",
            extends: None,
            description: Some(
                "SMIX Dstination N Gain Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Unsigned Int, with 12 fractional bits. . The top 3 bits are for shift. Same as SHFT_CTR[2:0].",
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
            name: "DstChSt",
            extends: None,
            description: Some(
                "SMIX Dstination N Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTn_CTRL [DST_EN] to enter Mode 1. Mode 1: Enabled and not-activated. wait for DSTn_CTRL [DSTFADIN_EN] or DSTn_CTRL [DST_ACT], jump to Mode 3 or Mode 4 based on whether Fade-in enabled. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will send data to DMA. Jump to Mode 4 after fadin op done. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will send data to DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out OP. Will send data to DMA. Will transfer to mode 6 or mode 7 depending on the DSTn_CTRL [ADeactFadeOut_En] cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will send data to DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't send data to DMA. Won't gen data avail signals. Intf register can be programmed. Will change to Mode 3 or Mode 4 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTn_CTRL [DST_EN] is assigned 0. To support a new stream or, to continue the old stream after a pause.",
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
                    name: "da",
                    description: Some(
                        "Data Available.",
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
                    name: "calsat",
                    description: Some(
                        "Saturate Error Found. W1C.",
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
                    name: "fdout_done",
                    description: Some(
                        "Fade-Out Done. W1C.",
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
                    name: "fifo_fillings",
                    description: Some(
                        "destination channel output FIFO fillings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FdotDoneSt",
            extends: None,
            description: Some(
                "SMIX Fade-Out Done Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "src",
                    description: Some(
                        "SRC fadeout done. W1C.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dst",
                    description: Some(
                        "DST fadeout done. W1C.",
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
            name: "Llp",
            extends: None,
            description: Some(
                "Channel N Linked List Pointer Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "the address pointer for the linked list descriptor.",
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
            name: "SourceAct",
            extends: None,
            description: Some(
                "SMIX Dstination N Source Activation Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Manually Activate the channel.",
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
            name: "SourceChCtrl",
            extends: None,
            description: Some(
                "SMIX Source N Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rateconv",
                    description: Some(
                        "0: no rate conversion 1: up-conversion x2 2: up-conversion x3 3: up-conversion x4 4: up-conversion x6 5: up-conversion x8 6: up-conversion x12 7: down-conversion /2.",
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
                    name: "fadeout_done_ie",
                    description: Some(
                        "Fade-Out interrupt enable.",
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
                    name: "autodeactafterfadeout_en",
                    description: Some(
                        "Asserted to enter de-activated mode after fade-out done.",
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
                    name: "shft_ctrl",
                    description: Some(
                        "Shift operation after FIR 0: no shift (when no upsampling or up-sampling-by-2 or up-sampling-by-3) 1: left-shift-by-1 (when up-sampling-by-4 or up-sampling-by-6) 2: left-shift-by-1 (when up-sampling-by-8 or up-sampling-by-12) 7: /2 (when rate /2) Other n: shift-left-by-n, but not suggested to be used.",
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
                    name: "dn_int_en",
                    description: Some(
                        "Data Needed IntEn.",
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
                    name: "calsat_int_en",
                    description: Some(
                        "Cal Saturation IntEn.",
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
                    name: "thrsh",
                    description: Some(
                        "FIFO threshold for DMA or Int. <= will generate req. Must be greater or equal than 8. This threshold is also used to trgger the internal FIR operation. To avoid the reading and writing to the same address in the memory block, the threshold should greater than 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fifo_reset",
                    description: Some(
                        "Asserted to reset FIFO pointer. Cleared to exit reset state.",
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
            name: "SourceChData",
            extends: None,
            description: Some(
                "SMIX Source N Data Input Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Data input register.",
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
            name: "SourceChFadein",
            extends: None,
            description: Some(
                "SMIX Source N Fade-in Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delta",
                    description: Some(
                        "Fade -in confg.",
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
            name: "SourceChFadeout",
            extends: None,
            description: Some(
                "SMIX Source N Fade-out Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "delta",
                    description: Some(
                        "Fade out in 2^DELTA samples. Now DELTA can be at most 14。.",
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
            name: "SourceChGain",
            extends: None,
            description: Some(
                "SMIX Source N Gain Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Unsigned Int, with 12 fractional bits. The top 3 bits are for shift. Same as SHFT_CTR[2:0].",
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
            name: "SourceChSt",
            extends: None,
            description: Some(
                "SMIX Source N Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTx_SRC_EN[n] to enter Mode 1. Mode 1: Enabled but not activated: After Enabled. Data needed signal can send out, can receive DMA data. Will enter Mode 2 after manual ACT or Fade-in CMD Mode 2: Enabled and activated and buffer feed-in in progress: Can not be fade out. Will consume data from DMA. If not enter due to Fade-in CMD, will enter Mode 4, else enter Mode 3. This mode is used to make the channel in MIX only after initial data are ready, thus will not stall mix operation due to the lackness of data of this channel omly. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will consume data from DMA. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will consume data from DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out done. Will consume data from DMA. Will transfer to mode 6 or mode 7 depending on the SRCn_CTRL[AutoDeactAfterFadeOut_En] cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will consume data from DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't consume data from DMA. Won't gen data needed signals. Intf register can be programmed. Will change to Mode 2 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTx_SRC_EN[n] is assigned 0. To support a new stream or, to continue the old stream after a pause.",
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
                    name: "firphase",
                    description: Some(
                        "the poly phase counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dn",
                    description: Some(
                        "Data needed flag.",
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
                    name: "calsat",
                    description: Some(
                        "Calculation saturation status. W1C.",
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
                    name: "fdout_done",
                    description: Some(
                        "Fade-Out Done. W1C.",
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
                    name: "fifo_fillings",
                    description: Some(
                        "The fillings of input FIFO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SourceDeact",
            extends: None,
            description: Some(
                "SMIX Dstination N Source De-Activation Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "Manually DeActivate the channel.",
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
            name: "SourceEn",
            extends: None,
            description: Some(
                "SMIX Dstination N Source Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "After enabled, Data needed req will be asserted. DMA can feed in data. The channel will join in the sum operation of mixer operation.",
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
            name: "SourceFadeinCtrl",
            extends: None,
            description: Some(
                "SMIX Dstination N Source Fade-in Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aop",
                    description: Some(
                        "Asserted to start fade-in operation. When the amplification factors are stable, auto clear.",
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
            name: "SourceMfadeoutCtrl",
            extends: None,
            description: Some(
                "SMIX Dstination N Source Manual Fade-out Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "op",
                    description: Some(
                        "Asserted to start fade-out operation. When the amplification factors are stable, auto clear.",
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
            name: "SrcAddr",
            extends: None,
            description: Some(
                "Channel N Source Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "source address.",
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
