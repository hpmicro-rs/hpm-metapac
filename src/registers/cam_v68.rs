use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cam",
            extends: None,
            description: Some(
                "CAM0.",
            ),
            items: &[
                BlockItem {
                    name: "cr1",
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
                                "Cr1",
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
                    byte_offset: 0x4,
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
                    name: "cr2",
                    description: Some(
                        "Control 2 Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
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
                    byte_offset: 0x24,
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
                    name: "dmasa_fb1",
                    description: Some(
                        "Pixel DMA Frame Buffer 1 Address.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmasaFb1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasa_fb2",
                    description: Some(
                        "Pixel DMA Frame Buffer 2 Address.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmasaFb2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_para",
                    description: Some(
                        "Buffer Parameters Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ideal_wn_size",
                    description: Some(
                        "Ideal Image Size Register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IdealWnSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr18",
                    description: Some(
                        "Control CR18 Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasa_uv1",
                    description: Some(
                        "Pixel UV DMA Frame Buffer 1 Address.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmasaUv1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasa_uv2",
                    description: Some(
                        "Pixel UV DMA Frame Buffer 2 Address.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmasaUv2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr20",
                    description: Some(
                        "Control CR20 Register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csc_coef0",
                    description: Some(
                        "Color Space Conversion Config Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CscCoef0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csc_coef1",
                    description: Some(
                        "Color Space Conversion Config Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CscCoef1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csc_coef2",
                    description: Some(
                        "Color Space Conversion Config Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CscCoef2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clrkey_low",
                    description: Some(
                        "Low Color Key Register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClrkeyLow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clrkey_high",
                    description: Some(
                        "High Color Key Register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClrkeyHigh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "histogram_fifo",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 256,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HistogramFifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "roi_width",
                    description: Some(
                        "Roi Width Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x490,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RoiWidth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "roi_height",
                    description: Some(
                        "Roi Width Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x494,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RoiHeight",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pro_ctrl",
                    description: Some(
                        "Pro Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x498,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ProCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "act_size",
                    description: Some(
                        "actual size.",
                    ),
                    array: None,
                    byte_offset: 0x49c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ActSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vsync_valid_cnt",
                    description: Some(
                        "vsync valid counter.",
                    ),
                    array: None,
                    byte_offset: 0x4a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VsyncValidCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hsync_valid_cnt",
                    description: Some(
                        "hsync valid counter.",
                    ),
                    array: None,
                    byte_offset: 0x4a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HsyncValidCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "valid_margin",
                    description: Some(
                        "valid margin.",
                    ),
                    array: None,
                    byte_offset: 0x4a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ValidMargin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alarm_set",
                    description: Some(
                        "alarm set.",
                    ),
                    array: None,
                    byte_offset: 0x4ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AlarmSet",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ActSize",
            extends: None,
            description: Some(
                "actual size.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "act_width",
                    description: Some(
                        "actual width after scale and/or roi.",
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
                    name: "act_height",
                    description: Some(
                        "actual height after scale and/or roi.",
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
            name: "AlarmSet",
            extends: None,
            description: Some(
                "alarm set.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pre_div",
                    description: Some(
                        "frequency division.",
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
                    name: "fatal_normal",
                    description: Some(
                        "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1.",
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
                    name: "sig_normal",
                    description: Some(
                        "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1.",
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
            name: "BufPara",
            extends: None,
            description: Some(
                "Buffer Parameters Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linebsp_stride",
                    description: Some(
                        "Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image.",
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
            name: "ClrkeyHigh",
            extends: None,
            description: Some(
                "High Color Key Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit",
                    description: Some(
                        "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ClrkeyLow",
            extends: None,
            description: Some(
                "Low Color Key Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit",
                    description: Some(
                        "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sensor_bit_width",
                    description: Some(
                        "the bit width of the sensor 0: 8 bits 1: 10 bits 3:24bits Others: Undefined.",
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
                    name: "color_formats",
                    description: Some(
                        "input color formats: 0010b:24bit:RGB888 0011b:24bit:RGB666 0100b:16bit:RGB565 0101b:16bit:RGB444 0110b:16bit:RGB555 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444.",
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
                    name: "storage_mode",
                    description: Some(
                        "00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20[BIG_END]=0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inv_data",
                    description: Some(
                        "Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry.",
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
                    name: "sof_int_pol",
                    description: Some(
                        "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge.",
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
                    name: "sync_rxfifo_clr",
                    description: Some(
                        "Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF.",
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
                    name: "async_rxfifo_clr",
                    description: Some(
                        "ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared.",
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
                    name: "restart_busptr",
                    description: Some(
                        "force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer.",
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
                    name: "pack_dir",
                    description: Some(
                        "Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO.",
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
                    name: "swap16_en",
                    description: Some(
                        "SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping.",
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
                    name: "inv_vsync",
                    description: Some(
                        "invert vsync pad input before it is used.",
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
                    name: "inv_hsync",
                    description: Some(
                        "invert hsync pad input before it is used.",
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
                    name: "inv_pixclk",
                    description: Some(
                        "invert pixclk pad input before it is used.",
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
                    name: "color_ext",
                    description: Some(
                        "If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2[ClrBitFormat] configuration.",
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
                    name: "inv_den",
                    description: Some(
                        "invert den pad input before it is used.",
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
            name: "Cr18",
            extends: None,
            description: Some(
                "Control CR18 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awqos",
                    description: Some(
                        "AWQOS for bus fabric arbitration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Control 2 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clrbitformat",
                    description: Some(
                        "Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion.",
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
                    name: "dma_req_en_rff",
                    description: Some(
                        "DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode.",
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
                    name: "rxff_level",
                    description: Some(
                        "RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "frmcnt_rst",
                    description: Some(
                        "Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately.",
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
                    name: "frmcnt_15_0",
                    description: Some(
                        "Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum).",
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
            name: "Cr20",
            extends: None,
            description: Some(
                "Control CR20 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "threshold",
                    description: Some(
                        "Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold.",
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
                    name: "big_end",
                    description: Some(
                        "Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word.",
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
                    name: "histogram_en",
                    description: Some(
                        "histogarm enable.",
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
                    name: "binary_en",
                    description: Some(
                        "binary picture output enable.",
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
            name: "CscCoef0",
            extends: None,
            description: Some(
                "Color Space Conversion Config Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_offset",
                    description: Some(
                        "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uv_offset",
                    description: Some(
                        "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c0",
                    description: Some(
                        "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enable",
                    description: Some(
                        "Enable the CSC unit 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data.",
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
                    name: "ycbcr_mode",
                    description: Some(
                        "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data.",
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
            name: "CscCoef1",
            extends: None,
            description: Some(
                "Color Space Conversion Config Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c4",
                    description: Some(
                        "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c1",
                    description: Some(
                        "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CscCoef2",
            extends: None,
            description: Some(
                "Color Space Conversion Config Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c3",
                    description: Some(
                        "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "c2",
                    description: Some(
                        "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmasaFb1",
            extends: None,
            description: Some(
                "Pixel DMA Frame Buffer 1 Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmasaFb2",
            extends: None,
            description: Some(
                "Pixel DMA Frame Buffer 2 Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "DMA Start Address in Frame Buffer2. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmasaUv1",
            extends: None,
            description: Some(
                "Pixel UV DMA Frame Buffer 1 Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "Two Plane UV Buffer Start Address 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmasaUv2",
            extends: None,
            description: Some(
                "Pixel UV DMA Frame Buffer 2 Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ptr",
                    description: Some(
                        "Two Plane UV Buffer Start Address 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HistogramFifo",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hist_y",
                    description: Some(
                        "the appearance of bin x (x=(address-DATA0)/4).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HsyncValidCnt",
            extends: None,
            description: Some(
                "hsync valid counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsync_valid_cnt",
                    description: Some(
                        "hsync valid counter.",
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
            name: "IdealWnSize",
            extends: None,
            description: Some(
                "Ideal Image Size Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "width",
                    description: Some(
                        "Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transferred is re-calculated automatically in hardware based on cr1[color_ext] and cr1[store_mode]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels.",
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
                    name: "height",
                    description: Some(
                        "Image Height. Indicates how many active pixels in a column of the image from the sensor.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sof_int_en",
                    description: Some(
                        "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable.",
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
                    name: "fb1_dma_done_inten",
                    description: Some(
                        "Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable.",
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
                    name: "fb2_dma_done_inten",
                    description: Some(
                        "Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable.",
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
                    name: "rf_or_inten",
                    description: Some(
                        "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled.",
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
                    name: "eof_int_en",
                    description: Some(
                        "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached.",
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
                    name: "hresp_err_en",
                    description: Some(
                        "Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt.",
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
                    name: "hist_done_int_en",
                    description: Some(
                        "Enable hist done int.",
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
                    name: "err_cl_bwid_cfg_int_en",
                    description: Some(
                        "The unsupported color (color_formats[3:0]) and bitwidth (sensor_bit_width[2:0]) configuation interrupt enable.",
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
            name: "ProCtrl",
            extends: None,
            description: Some(
                "Pro Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scale_width_select",
                    description: Some(
                        "000 keep all pixel for width 001 keep 1 for every 2 pixel for width 010 keep 1 for every 3 pixel for width 011 keep 1 for every 4 pixel for width 100 keep 1 for every 5 pixel for width 101 keep 1 for every 6 pixel for width 110 keep 1 for every 7 pixel for width 111 keep 1 for every 8 pixel for width.",
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
                    name: "scale_height_select",
                    description: Some(
                        "000 keep all pixel for height 001 keep 1 for every 2 pixel for height 010 keep 1 for every 3 pixel for height 011 keep 1 for every 4 pixel for height 100 keep 1 for every 5 pixel for height 101 keep 1 for every 6 pixel for height 110 keep 1 for every 7 pixel for height 111 keep 1 for every 8 pixel for height.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scale_update",
                    description: Some(
                        "scale configration update.",
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
                    name: "roi_update",
                    description: Some(
                        "roi configration update.",
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
                    name: "err_inject",
                    description: Some(
                        "0 generate alarm in normal mode 1 force to generate fatal alarm.",
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
            name: "RoiHeight",
            extends: None,
            description: Some(
                "Roi Width Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roi_height_start",
                    description: Some(
                        "start address of height for roi.",
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
                    name: "roi_height_end",
                    description: Some(
                        "end address of height for roi.",
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
            name: "RoiWidth",
            extends: None,
            description: Some(
                "Roi Width Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "roi_width_start",
                    description: Some(
                        "start address of width for roi.",
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
                    name: "roi_width_end",
                    description: Some(
                        "end address of width for roi.",
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
            name: "Sta",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hresp_err_int",
                    description: Some(
                        "Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected.",
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
                    name: "sof_int",
                    description: Some(
                        "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected.",
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
                    name: "eof_int",
                    description: Some(
                        "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected.",
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
                    name: "dma_tsf_done_fb1",
                    description: Some(
                        "DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed.",
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
                    name: "dma_tsf_done_fb2",
                    description: Some(
                        "DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed.",
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
                    name: "rf_or_int",
                    description: Some(
                        "RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed.",
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
                    name: "hist_done",
                    description: Some(
                        "hist cal done.",
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
                    name: "err_cl_bwid_cfg",
                    description: Some(
                        "The unsupported color (color_formats[3:0]) and bitwidth (sensor_bit_width[2:0]) configuation found.",
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
            ],
        },
        FieldSet {
            name: "ValidMargin",
            extends: None,
            description: Some(
                "valid margin.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsync_valid_margin",
                    description: Some(
                        "vsync valid margin.",
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
                    name: "hsync_valid_margin",
                    description: Some(
                        "hsync valid margin.",
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
            name: "VsyncValidCnt",
            extends: None,
            description: Some(
                "vsync valid counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsync_valid_cnt",
                    description: Some(
                        "vsync valid counter.",
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
