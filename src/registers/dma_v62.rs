use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Chctrl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Channel n Control Register.",
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
                    name: "tran_size",
                    description: Some(
                        "Channel n Transfer Size Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TranSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "src_addr",
                    description: Some(
                        "Channel n Source Address Low Part Register.",
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
                    name: "src_addr_h",
                    description: Some(
                        "Channel n Source Address High Part Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SrcAddrH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dst_addr",
                    description: Some(
                        "Channel n Destination Address Low Part Register.",
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
                    name: "dst_addr_h",
                    description: Some(
                        "Channel n Destination Address High Part Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DstAddrH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llpointer",
                    description: Some(
                        "Channel n Linked List Pointer Low Part Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Llpointer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "llpointer_h",
                    description: Some(
                        "Channel n Linked List Pointer High Part Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LlpointerH",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Dma",
            extends: None,
            description: Some(
                "HDMA.",
            ),
            items: &[
                BlockItem {
                    name: "idmisc",
                    description: Some(
                        "ID Misc.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idmisc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmacfg",
                    description: Some(
                        "DMAC Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmacfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactrl",
                    description: Some(
                        "DMAC Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmactrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch_abort",
                    description: Some(
                        "Channel Abort Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChAbort",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_status",
                    description: Some(
                        "Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch_en",
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
                                "ChEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chctrl",
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
                    byte_offset: 0x40,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Chctrl",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChAbort",
            extends: None,
            description: Some(
                "Channel Abort Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chabort",
                    description: Some(
                        "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels).",
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
            name: "ChEn",
            extends: None,
            description: Some(
                "Channel Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Alias of the Enable field of all ChnCtrl registers.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Channel n Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable",
                    description: Some(
                        "Channel enable bit 0x0: Disable 0x1: Enable.",
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
                    name: "inttcmask",
                    description: Some(
                        "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt.",
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
                    name: "interrmask",
                    description: Some(
                        "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt.",
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
                    name: "intabtmask",
                    description: Some(
                        "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt.",
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
                    name: "dstreqsel",
                    description: Some(
                        "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to.",
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
                    name: "srcreqsel",
                    description: Some(
                        "Source DMA request select. Select the request/ack handshake pair that the source device is connected to.",
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
                    name: "dstaddrctrl",
                    description: Some(
                        "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception.",
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
                    name: "srcaddrctrl",
                    description: Some(
                        "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dstmode",
                    description: Some(
                        "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode.",
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
                    name: "srcmode",
                    description: Some(
                        "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode.",
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
                    name: "dstwidth",
                    description: Some(
                        "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcwidth",
                    description: Some(
                        "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2.",
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
                    name: "srcburstsize",
                    description: Some(
                        "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7.",
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
                    name: "priority",
                    description: Some(
                        "Channel priority level 0x0: Lower priority 0x1: Higher priority.",
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
                    name: "dstbusinfidx",
                    description: Some(
                        "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1.",
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
                    name: "srcbusinfidx",
                    description: Some(
                        "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface.",
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
            name: "Dmacfg",
            extends: None,
            description: Some(
                "DMAC Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "channelnum",
                    description: Some(
                        "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid.",
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
                    name: "fifodepth",
                    description: Some(
                        "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqnum",
                    description: Some(
                        "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busnum",
                    description: Some(
                        "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses.",
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
                    name: "corenum",
                    description: Some(
                        "DMA core number 0x0: 1 core 0x1: 2 cores.",
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
                    name: "addrwidth",
                    description: Some(
                        "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datawidth",
                    description: Some(
                        "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits.",
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
                    name: "reqsync",
                    description: Some(
                        "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured.",
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
                    name: "chainxfr",
                    description: Some(
                        "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured.",
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
            name: "Dmactrl",
            extends: None,
            description: Some(
                "DMAC Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction.",
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
            name: "DstAddr",
            extends: None,
            description: Some(
                "Channel n Destination Address Low Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstaddrl",
                    description: Some(
                        "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered.",
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
            name: "DstAddrH",
            extends: None,
            description: Some(
                "Channel n Destination Address High Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstaddrh",
                    description: Some(
                        "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits.",
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
            name: "Idmisc",
            extends: None,
            description: Some(
                "ID Misc.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idle_flag",
                    description: Some(
                        "DMA Idle Flag 0 - DMA is busy 1 - DMA is dile.",
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
            name: "IntStatus",
            extends: None,
            description: Some(
                "Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "error",
                    description: Some(
                        "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status.",
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
                    name: "abort",
                    description: Some(
                        "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status.",
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
                    name: "tc",
                    description: Some(
                        "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status.",
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
            name: "Llpointer",
            extends: None,
            description: Some(
                "Channel n Linked List Pointer Low Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lldbusinfidx",
                    description: Some(
                        "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0.",
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
                    name: "llpointerl",
                    description: Some(
                        "Low part of the pointer to the next descriptor. The pointer must be double word aligned.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LlpointerH",
            extends: None,
            description: Some(
                "Channel n Linked List Pointer High Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "llpointerh",
                    description: Some(
                        "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits.",
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
            name: "SrcAddr",
            extends: None,
            description: Some(
                "Channel n Source Address Low Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srcaddrl",
                    description: Some(
                        "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered.",
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
            name: "SrcAddrH",
            extends: None,
            description: Some(
                "Channel n Source Address High Part Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "srcaddrh",
                    description: Some(
                        "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits.",
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
            name: "TranSize",
            extends: None,
            description: Some(
                "Channel n Transfer Size Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "transize",
                    description: Some(
                        "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated.",
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
