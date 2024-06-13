use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Jpeg",
            extends: None,
            description: Some(
                "JPEG.",
            ),
            items: &[
                BlockItem {
                    name: "in_dma_misc",
                    description: Some(
                        "In DMA Misc Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InDmaMisc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_dmabase",
                    description: Some(
                        "In DMA Buf Address.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InDmabase",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_dma_ctrl0",
                    description: Some(
                        "In DMA Buf Control 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InDmaCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in_dma_ctrl1",
                    description: Some(
                        "In DMA Buf Control 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InDmaCtrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inxt_cmd",
                    description: Some(
                        "In DMA Next Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "InxtCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_dma_misc",
                    description: Some(
                        "Out DMA Misc Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDmaMisc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_dmabase",
                    description: Some(
                        "Out DMA Buf Address.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDmabase",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_dma_ctrl0",
                    description: Some(
                        "Out DMA Buf Control 0 Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDmaCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_dma_ctrl1",
                    description: Some(
                        "Out DMA Buf Control 1 Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDmaCtrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "onxt_cmd",
                    description: Some(
                        "Out DMA Next Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OnxtCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                    name: "stat",
                    description: Some(
                        "Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                    name: "width",
                    description: Some(
                        "Image width register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Width",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "height",
                    description: Some(
                        "Image height register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Height",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_addr",
                    description: Some(
                        "Buf Access Addr.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "buf_data",
                    description: Some(
                        "Buf Access Data.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BufData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_dmacnt",
                    description: Some(
                        "Out DMA Bytes Counter.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDmacnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csc_coef0",
                    description: Some(
                        "YUV2RGB coefficients Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
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
                        "YUV2RGB coefficients Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                        "YUV2RGB coefficients Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                    name: "rgb2yuv_coef0",
                    description: Some(
                        "RGB2YUV coefficients Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x68,
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
                        "RGB2YUV coefficients Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
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
                        "RGB2YUV coefficients Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x70,
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
                        "RGB2YUV coefficients Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x74,
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
                        "RGB2YUV coefficients Register 4.",
                    ),
                    array: None,
                    byte_offset: 0x78,
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
                BlockItem {
                    name: "img_reg1",
                    description: Some(
                        "Image Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ImgReg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "img_reg2",
                    description: Some(
                        "Image Control Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ImgReg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "img_reg3",
                    description: Some(
                        "Image Control Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ImgReg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "imgreg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
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
                                "Imgreg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BufAddr",
            extends: None,
            description: Some(
                "Buf Access Addr.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "ADDR[31:28] denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR[27:0] is the address inside the buffer.",
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
            name: "BufData",
            extends: None,
            description: Some(
                "Buf Access Data.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address.",
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
            name: "Cfg",
            extends: None,
            description: Some(
                "Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jpeg_en",
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
                    name: "mode",
                    description: Some(
                        "1: decoder, 0:encoder.",
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
                    name: "start",
                    description: Some(
                        "Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear.",
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
                    name: "jpeg_sftrst",
                    description: Some(
                        "Software Reset.",
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
                    name: "jdata_format",
                    description: Some(
                        "3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined.",
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
                    name: "cfg_opath_sel",
                    description: Some(
                        "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0.",
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
                    name: "mem_debug_clk_sel",
                    description: Some(
                        "asserted to use APB clock, so that the memory contents could be read out through APB interface.",
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
                    name: "codec_restart_err_irq_en",
                    description: Some(
                        "The jpg endec restart error interrupt enable.",
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
                    name: "codec_over_irq_en",
                    description: Some(
                        "The jpg endec process done interrupt enable.",
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
                    name: "cfg_ipath_sel",
                    description: Some(
                        "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jd_uvswap",
                    description: Some(
                        "Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB.",
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
            ],
        },
        FieldSet {
            name: "CscCoef0",
            extends: None,
            description: Some(
                "YUV2RGB coefficients Register 0.",
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
                Field {
                    name: "enable",
                    description: Some(
                        "Enable the CSC unit. 0b - The CSC is bypassed 1b - The CSC is enabled.",
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
                "YUV2RGB coefficients Register 1.",
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
            name: "CscCoef2",
            extends: None,
            description: Some(
                "YUV2RGB coefficients Register 2.",
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
        FieldSet {
            name: "Height",
            extends: None,
            description: Some(
                "Image height register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "img",
                    description: Some(
                        "Image Height (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as [0,0]).",
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
            name: "ImgReg1",
            extends: None,
            description: Some(
                "Image Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncol",
                    description: Some(
                        "Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2.",
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
                    name: "re",
                    description: Some(
                        "Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs.",
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
            ],
        },
        FieldSet {
            name: "ImgReg2",
            extends: None,
            description: Some(
                "Image Control Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nmcu",
                    description: Some(
                        "Encoder Use only. The number of NMCU to be generated in encoder mode.",
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
            name: "ImgReg3",
            extends: None,
            description: Some(
                "Image Control Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nrst",
                    description: Some(
                        "Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set.",
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
            name: "Imgreg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hd",
                    description: Some(
                        "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component.",
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
                    name: "ha",
                    description: Some(
                        "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component.",
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
                    name: "qt",
                    description: Some(
                        "Encoder use only. The selection of the quantization table.",
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
                    name: "nblock",
                    description: Some(
                        "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1.",
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
            ],
        },
        FieldSet {
            name: "InDmaCtrl0",
            extends: None,
            description: Some(
                "In DMA Buf Control 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Pitch between the starting point of Rows. Only active when In_DMA_ID=Pixel..",
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
                    name: "ttlen",
                    description: Some(
                        "Total length (Low 16 bits) in Bytes -1 for transfer when In_DMA_ID!=Pixel.",
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
            name: "InDmaCtrl1",
            extends: None,
            description: Some(
                "In DMA Buf Control 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rowlen",
                    description: Some(
                        "Total length (High 16 bits) in Bytes -1 for transfer. See reference in InDMA_Ctrl0[TTLEN].",
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
            name: "InDmaMisc",
            extends: None,
            description: Some(
                "In DMA Misc Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indma2d",
                    description: Some(
                        "Asserted if In_DMA_ID=Pixel.",
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
                    name: "in_dma_req",
                    description: Some(
                        "Asserted to request DMA. Automatically clear after DMA is done.",
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
                    name: "in_dma_id",
                    description: Some(
                        "0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb.",
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
                    name: "irq_en",
                    description: Some(
                        "interrupt enable for all interrupt sources of In DMA module.",
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
                    name: "axi_err_irq_en",
                    description: Some(
                        "In DMA axi bus error inetrrupt enable.",
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
                    name: "in_dma_done_irq_en",
                    description: Some(
                        "In DMA Done enable.",
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
                    name: "nxt_irq_en",
                    description: Some(
                        "In DMA Next Interrupt Enable.",
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
                    name: "indma_renew",
                    description: Some(
                        "Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter.",
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
                    name: "pack_dir",
                    description: Some(
                        "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}.",
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
                    name: "inb13_swap",
                    description: Some(
                        "Swap bit[31:24] and bit [15:8] before pack dir operation. Only work for pixel data.",
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
                    name: "max_ot",
                    description: Some(
                        "max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2.",
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
                        "QoS for AXI read channel.",
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
            name: "InDmabase",
            extends: None,
            description: Some(
                "In DMA Buf Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Y plane (or Encoded Bit Plane).",
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
            name: "InxtCmd",
            extends: None,
            description: Some(
                "In DMA Next Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "NXTCMD phase Enable Bit.",
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
                    name: "op_valid",
                    description: Some(
                        "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the InDMA transfer if CFG[JPEG_EN] is 1.",
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
                    name: "addr",
                    description: Some(
                        "The address pointing to the next command.",
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
            name: "OnxtCmd",
            extends: None,
            description: Some(
                "Out DMA Next Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "NXTCMD phase Enable Bit.",
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
                    name: "op_valid",
                    description: Some(
                        "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG[JPEG_EN] is 1.",
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
                    name: "addr",
                    description: Some(
                        "The address pointing to the next command.",
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
            name: "OutDmaCtrl0",
            extends: None,
            description: Some(
                "Out DMA Buf Control 0 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Pitch between the starting point of Rows when Out_DMA_ID==Pixel.",
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
                    name: "ttlen",
                    description: Some(
                        "Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes.",
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
            name: "OutDmaCtrl1",
            extends: None,
            description: Some(
                "Out DMA Buf Control 1 Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rowlen",
                    description: Some(
                        "Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0[TTLEN].",
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
            name: "OutDmaMisc",
            extends: None,
            description: Some(
                "Out DMA Misc Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "outdma2d",
                    description: Some(
                        "Asserted if Out_DMA_ID==Pixel.",
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
                    name: "out_dma_req",
                    description: Some(
                        "Asserted to enable Out DMA request.",
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
                    name: "out_dma_id",
                    description: Some(
                        "0: Pixel (Out) 1: ECS (Out).",
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
                    name: "irq_en",
                    description: Some(
                        "interrupt enable for all interrupt sources of Out DMA module.",
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
                    name: "axi_err_irq_en",
                    description: Some(
                        "Out DMA axi bus error inetrrupt enable.",
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
                    name: "out_dma_done_irq_en",
                    description: Some(
                        "Out DMA Done interrupt Enable.",
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
                    name: "nxt_irq_en",
                    description: Some(
                        "Out DMA Next Interrupt Enable.",
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
                    name: "add_odma_endings",
                    description: Some(
                        "Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module.",
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
                    name: "ini_outcnt",
                    description: Some(
                        "Asserted to ini output counter.",
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
                    name: "en_outcnt",
                    description: Some(
                        "Enable output counter (unit as bytes).",
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
                    name: "pack_dir",
                    description: Some(
                        "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}.",
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
                    name: "awqos",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OutDmabase",
            extends: None,
            description: Some(
                "Out DMA Buf Address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Y plane (or Encoded Bit Plane).",
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
            name: "OutDmacnt",
            extends: None,
            description: Some(
                "Out DMA Bytes Counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "val",
                    description: Some(
                        "The out DMA counter.",
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
            name: "Rgb2yuvCoef0",
            extends: None,
            description: Some(
                "RGB2YUV coefficients Register 0.",
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
                        "Asserted to enable this RGB2YCbCr CSC stage.",
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
                        "Asserted to use YCrCb mode. Must be assigned as 1.",
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
                "RGB2YUV coefficients Register 1.",
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
                "RGB2YUV coefficients Register 2.",
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
                "RGB2YUV coefficients Register 3.",
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
                "RGB2YUV coefficients Register 4.",
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
            name: "Stat",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "restart_marker_error",
                    description: Some(
                        "codec restart marker error interrupt.",
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
                    name: "codec_over",
                    description: Some(
                        "Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted.",
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
                    name: "in_dma_transfer_done",
                    description: Some(
                        "InDMA process done.",
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
                    name: "out_dma_transfer_done",
                    description: Some(
                        "OutDMA process done.",
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
                    name: "inxt_irq",
                    description: Some(
                        "InDMA next interrupt.",
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
                    name: "onxt_irq",
                    description: Some(
                        "OutDMA next interrupt.",
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
                    name: "axi_err",
                    description: Some(
                        "axi bus error.",
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
                    name: "axi_write_err",
                    description: Some(
                        "out-dma axi bus error.",
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
                    name: "axi_read_err",
                    description: Some(
                        "in-dma axi bus error.",
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
                    name: "axi_err_id",
                    description: Some(
                        "the axi err id.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some(
                        "When 1 means that the module is busy doing conversion and data transfer.",
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
            name: "Width",
            extends: None,
            description: Some(
                "Image width register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "img",
                    description: Some(
                        "Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as [0,0]).",
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
    ],
    enums: &[],
};
