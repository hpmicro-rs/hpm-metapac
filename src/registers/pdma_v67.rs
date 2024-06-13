use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "OutPs",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ulc",
                    description: Some(
                        "Layer Upper Left Corner Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ulc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lrc",
                    description: Some(
                        "Layer Lower Right Corner Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lrc",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pdma",
            extends: None,
            description: Some(
                "PDMA.",
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
                                "PdmaCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_ctrl",
                    description: Some(
                        "Out Layer Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_buf",
                    description: Some(
                        "Output buffer address.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutBuf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_pitch",
                    description: Some(
                        "Outlayer Pitch Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutPitch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_lrc",
                    description: Some(
                        "Output Lower Right Corner Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutLrc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_ps",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "OutPs",
                        },
                    ),
                },
                BlockItem {
                    name: "ps",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 48,
                            },
                        ),
                    ),
                    byte_offset: 0x30,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Ps",
                        },
                    ),
                },
                BlockItem {
                    name: "yuv2rgb_coef0",
                    description: Some(
                        "YUV2RGB coefficients register 0.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Yuv2rgbCoef0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "yuv2rgb_coef1",
                    description: Some(
                        "YUV2RGB coefficients register 1.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Yuv2rgbCoef1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "yuv2rgb_coef2",
                    description: Some(
                        "YUV2RGB coefficients register 2.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Yuv2rgbCoef2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgb2yuv_coef0",
                    description: Some(
                        "RGB2YUV coefficients register 0.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgb2yuvCoef0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgb2yuv_coef1",
                    description: Some(
                        "RGB2YUV coefficients register 1.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgb2yuvCoef1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgb2yuv_coef2",
                    description: Some(
                        "RGB2YUV coefficients register 2.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgb2yuvCoef2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgb2yuv_coef3",
                    description: Some(
                        "RGB2YUV coefficients register 3.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgb2yuvCoef3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rgb2yuv_coef4",
                    description: Some(
                        "RGB2YUV coefficients register 4.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rgb2yuvCoef4",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Ps",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Layer Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PsCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf",
                    description: Some(
                        "Layer data buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Buf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pitch",
                    description: Some(
                        "Layer data pitch register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pitch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bkgd",
                    description: Some(
                        "Layer background color register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bkgd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scale",
                    description: Some(
                        "Layer scale register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scale",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "offset",
                    description: Some(
                        "Layer offset register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Offset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clrkey_low",
                    description: Some(
                        "Layer low color key register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                        "Layer high color key register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "org",
                    description: Some(
                        "Layer original size register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Org",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bkgd",
            extends: None,
            description: Some(
                "Layer background color register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "color",
                    description: Some(
                        "Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel.",
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
            name: "Buf",
            extends: None,
            description: Some(
                "Layer data buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Address pointer for the PS RGB or Y (luma) input buffer.",
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
            name: "ClrkeyHigh",
            extends: None,
            description: Some(
                "Layer high color key register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "limit",
                    description: Some(
                        "High range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000.",
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
                "Layer low color key register.",
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
            name: "Lrc",
            extends: None,
            description: Some(
                "Layer Lower Right Corner Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer.",
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
                    name: "y",
                    description: Some(
                        "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Offset",
            extends: None,
            description: Some(
                "Layer offset register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage.",
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
                    name: "y",
                    description: Some(
                        "This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Org",
            extends: None,
            description: Some(
                "Layer original size register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "width",
                    description: Some(
                        "The number of horizontal pixels of the original frame (not -1).",
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
                    name: "hight",
                    description: Some(
                        "The number of vertical pixels of the original frame (not -1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OutBuf",
            extends: None,
            description: Some(
                "Output buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Current address pointer for the output frame buffer. The address can have any byte alignment. 64B alignment is recommended for optimal performance.",
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
            name: "OutCtrl",
            extends: None,
            description: Some(
                "Out Layer Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1).",
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
                    name: "ablend_mode",
                    description: Some(
                        "Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL[BKGNDCL4CLR] is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved.",
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
                    name: "srcalpha_op",
                    description: Some(
                        "The usage of the SRCALPHA[7:0]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA[7:0] is invalid, use the alpha value embedded in the stream 1: the SRCALPHA[7:0] is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA[7:0] is used to scale the alpha value embedded in the stream Others: Reserved.",
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
                    name: "dstalpha_op",
                    description: Some(
                        "The usage of the DSTALPHA[7:0]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA[7:0] is invalid, use the alpha value embedded in the stream 1: the DSTALPHA[7:0] is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA[7:0] is used to scale the alpha value embedded in the stream Others: Reserved.",
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
                    name: "srcalpha",
                    description: Some(
                        "The source (P0) system ALPHA value.",
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
                    name: "dstalpha",
                    description: Some(
                        "The destination (P1) system ALPHA value.",
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
            name: "OutLrc",
            extends: None,
            description: Some(
                "Output Lower Right Corner Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "This field indicates the lower right X-coordinate (in pixels) of the output frame buffer. Should be the width of the output image size.",
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
                    name: "y",
                    description: Some(
                        "This field indicates the lower right Y-coordinate (in pixels) of the output frame buffer. The value is the height of the output image size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OutPitch",
            extends: None,
            description: Some(
                "Outlayer Pitch Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bytelen",
                    description: Some(
                        "Indicates the number of bytes in memory between two vertically adjacent pixels.",
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
            name: "PdmaCtrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdma_en",
                    description: Some(
                        "1b - Enabled.",
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
                    name: "pdma_sftrst",
                    description: Some(
                        "Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode.",
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
                    name: "p0_en",
                    description: Some(
                        "Plane 0 Enable.",
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
                    name: "p1_en",
                    description: Some(
                        "Plane 1 Enable.",
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
                    name: "bs16",
                    description: Some(
                        "Asserted when the Block Size is 16x16, else 8x8.",
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
                    name: "irq_en",
                    description: Some(
                        "Enable normal interrupt.",
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
                    name: "clkgate",
                    description: Some(
                        "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on.",
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
                    name: "pdma_done_irq_en",
                    description: Some(
                        "Enable interrupt of PDMA_DONE.",
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
                    name: "axierr_irq_en",
                    description: Some(
                        "Enable interrupt of AXI bus error.",
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
                    name: "pack_dir",
                    description: Some(
                        "Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}.",
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
                    name: "awqos",
                    description: Some(
                        "QoS for AXI write bus.",
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
                    name: "arqos",
                    description: Some(
                        "QoS for AXI read bus.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pitch",
            extends: None,
            description: Some(
                "Layer data pitch register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bytelen",
                    description: Some(
                        "Indicates the number of bytes in memory between two vertically adjacent pixels.",
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
            name: "PsCtrl",
            extends: None,
            description: Some(
                "Layer Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "format",
                    description: Some(
                        "PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes).",
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
                    name: "hw_byte_swap",
                    description: Some(
                        "Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped.",
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
                    name: "decx",
                    description: Some(
                        "Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8.",
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
                    name: "decy",
                    description: Some(
                        "Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rotate",
                    description: Some(
                        "Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270.",
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
                    name: "hflip",
                    description: Some(
                        "Indicates that the input should be flipped horizontally (effect applied before rotation).",
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
                    name: "vflip",
                    description: Some(
                        "Indicates that the input should be flipped vertically (effect applied before rotation).",
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
                    name: "bypass",
                    description: Some(
                        "Asserted to bypass the CSC stage.",
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
                    name: "ycbcr_mode",
                    description: Some(
                        "YCbCr mode or YUV mode.",
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
                    name: "bkgcl4clr",
                    description: Some(
                        "Enable to use background color for clear area.",
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
                    name: "pack_dir",
                    description: Some(
                        "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inb13_swap",
                    description: Some(
                        "Swap bit[31:24] and bit [15:8] before pack_dir operation.",
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
            name: "Rgb2yuvCoef0",
            extends: None,
            description: Some(
                "RGB2YUV coefficients register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_offset",
                    description: Some(
                        "CSC parameters Y_OFFSET.",
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
                        "CSC parameters UV_OFFSET.",
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
                        "CSC parameters C0.",
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
                        "Asserted to enable this RGB2YUV CSC stage.",
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
                        "Asserted to use YCrCb mode.",
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
            name: "Rgb2yuvCoef1",
            extends: None,
            description: Some(
                "RGB2YUV coefficients register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c4",
                    description: Some(
                        "CSC parameters C4.",
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
                        "CSC parameters C1.",
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
            name: "Rgb2yuvCoef2",
            extends: None,
            description: Some(
                "RGB2YUV coefficients register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c3",
                    description: Some(
                        "CSC parameters C3.",
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
                        "CSC parameters C2.",
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
            name: "Rgb2yuvCoef3",
            extends: None,
            description: Some(
                "RGB2YUV coefficients register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c5",
                    description: Some(
                        "CSC parameters C5.",
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
                    name: "c6",
                    description: Some(
                        "CSC parameters C6.",
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
            name: "Rgb2yuvCoef4",
            extends: None,
            description: Some(
                "RGB2YUV coefficients register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c7",
                    description: Some(
                        "CSC parameters C7.",
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
                    name: "c8",
                    description: Some(
                        "CSC parameters C8.",
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
            name: "Scale",
            extends: None,
            description: Some(
                "Layer scale register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2.",
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
                    name: "y",
                    description: Some(
                        "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2.",
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
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq",
                    description: Some(
                        "Asserted to indicate a IRQ event.",
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
                    name: "axi_0_read_err",
                    description: Some(
                        "AXI0 read err.",
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
                    name: "axi_1_read_err",
                    description: Some(
                        "AXI1 read err.",
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
                    name: "axi_0_write_err",
                    description: Some(
                        "AXI0 write err.",
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
                    name: "axi_err_id",
                    description: Some(
                        "AXI error ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdma_done",
                    description: Some(
                        "PDMA one image done.",
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
                    name: "blockx",
                    description: Some(
                        "X block that is processing.",
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
                    name: "blocky",
                    description: Some(
                        "Y block that is processing.",
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
            name: "Ulc",
            extends: None,
            description: Some(
                "Layer Upper Left Corner Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "This field indicates the upper left X-coordinate (in pixels) of the processed surface in the output frame buffer.",
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
                    name: "y",
                    description: Some(
                        "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output frame buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Yuv2rgbCoef0",
            extends: None,
            description: Some(
                "YUV2RGB coefficients register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "y_offset",
                    description: Some(
                        "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0).",
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
                        "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range).",
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
                        "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164).",
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
            ],
        },
        FieldSet {
            name: "Yuv2rgbCoef1",
            extends: None,
            description: Some(
                "YUV2RGB coefficients register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c4",
                    description: Some(
                        "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017).",
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
                        "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596).",
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
            name: "Yuv2rgbCoef2",
            extends: None,
            description: Some(
                "YUV2RGB coefficients register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "c3",
                    description: Some(
                        "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392).",
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
                        "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813).",
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
    ],
    enums: &[],
};
