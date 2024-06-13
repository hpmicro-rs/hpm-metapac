use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Layer",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "layctrl",
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
                                "Layctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "alphas",
                    description: Some(
                        "Layer Alpha Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Alphas",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "laysize",
                    description: Some(
                        "Layer Size Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Laysize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "laypos",
                    description: Some(
                        "Layer Position Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Laypos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "start0",
                    description: Some(
                        "Layer Buffer Pointer Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Start0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "linecfg",
                    description: Some(
                        "Layer Bus Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Linecfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bg_cl",
                    description: Some(
                        "Layer Background Color Register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BgCl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csc_coef0",
                    description: Some(
                        "Layer Color Space Conversion Config Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                        "Layer Color Space Conversion Config Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                        "Layer Color Space Conversion Config Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
            ],
        },
        Block {
            name: "Lcdc",
            extends: None,
            description: Some(
                "LCDC.",
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
                    name: "bgnd_cl",
                    description: Some(
                        "Background Color Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BgndCl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "disp_wn_size",
                    description: Some(
                        "Display Window Size Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DispWnSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hsync_para",
                    description: Some(
                        "HSYNC Config Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HsyncPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vsync_para",
                    description: Some(
                        "VSYNC Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VsyncPara",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_st",
                    description: Some(
                        "DMA Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "st",
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
                                "St",
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
                    byte_offset: 0x1c,
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
                    name: "txfifo",
                    description: Some(
                        "TX FIFO Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txfifo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "layer",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Layer",
                        },
                    ),
                },
                BlockItem {
                    name: "clut_load",
                    description: Some(
                        "Clut Load Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ClutLoad",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Alphas",
            extends: None,
            description: Some(
                "Layer Alpha Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ind",
                    description: Some(
                        "The system alpha value for the input stream from previous stage (DST).",
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
                    name: "locd",
                    description: Some(
                        "The system alpha value for the data stream of current layer stream (SRC).",
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
            name: "BgCl",
            extends: None,
            description: Some(
                "Layer Background Color Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "argb",
                    description: Some(
                        "ARGB8888. It is only useful in the last active stage in the pipeline.",
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
            name: "BgndCl",
            extends: None,
            description: Some(
                "Background Color Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "b",
                    description: Some(
                        "Blue component of the default color displayed in the sectors where no layer is active.",
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
                    name: "g",
                    description: Some(
                        "Green component of the default color displayed in the sectors where no layer is active.",
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
                    name: "r",
                    description: Some(
                        "Red component of the default color displayed in the sectors where no layer is active.",
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
            name: "ClutLoad",
            extends: None,
            description: Some(
                "Clut Load Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "update_en",
                    description: Some(
                        "CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM.",
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
                    name: "sel_num",
                    description: Some(
                        "Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD[UPDATE_EN] is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD[UPDATE_EN] bit.",
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
            ],
        },
        FieldSet {
            name: "CscCoef0",
            extends: None,
            description: Some(
                "Layer Color Space Conversion Config Register 0.",
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
                        "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed.",
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
                "Layer Color Space Conversion Config Register 1.",
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
                "Layer Color Space Conversion Config Register 2.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inv_hsync",
                    description: Some(
                        "Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW.",
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
                    name: "inv_vsync",
                    description: Some(
                        "Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW.",
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
                    name: "inv_href",
                    description: Some(
                        "Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW.",
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
                    name: "inv_pxclk",
                    description: Some(
                        "Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge.",
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
                    name: "inv_pxdata",
                    description: Some(
                        "Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory.",
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
                    name: "arqos",
                    description: Some(
                        "ARQOS for bus fabric arbitration.",
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
                    name: "bgdcl4clr",
                    description: Some(
                        "background color for clear mode when the alpha channel is 0.",
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
                    name: "disp_mode",
                    description: Some(
                        "LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "line_pattern",
                    description: Some(
                        "LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disp_on",
                    description: Some(
                        "Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on.",
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
                    name: "sw_rst",
                    description: Some(
                        "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected.",
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
            name: "DispWnSize",
            extends: None,
            description: Some(
                "Display Window Size Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "Sets the display size horizontal resolution in pixels.",
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
                        "Sets the display size vertical resolution in pixels.",
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
            name: "DmaSt",
            extends: None,
            description: Some(
                "DMA Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma0_done",
                    description: Some(
                        "Plane n frame 0 dma done. W1C.",
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
                    name: "dma1_done",
                    description: Some(
                        "Plane n frame 1 dma done. W1C.",
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
                    name: "dma_err",
                    description: Some(
                        "plane n axi error. W1C.",
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
            name: "HsyncPara",
            extends: None,
            description: Some(
                "HSYNC Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pw",
                    description: Some(
                        "HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1.",
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
                    name: "bp",
                    description: Some(
                        "HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fp",
                    description: Some(
                        "HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 9,
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
                    name: "vsync",
                    description: Some(
                        "Interrupt enable for end of sof.",
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
                    name: "underrun",
                    description: Some(
                        "Interrupt enable for underrun.",
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
                    name: "vs_blank",
                    description: Some(
                        "Interrupt enable for start of sof.",
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
                    name: "urgent_underrun",
                    description: Some(
                        "Asserted when the output buffer urgent underrun condition encountered.",
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
                    name: "dma_done",
                    description: Some(
                        "Interrupt enable for DMA done.",
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
                    name: "dma_err",
                    description: Some(
                        "Interrupt enable for DMA error.",
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
            name: "Layctrl",
            extends: None,
            description: Some(
                "Layer Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane.",
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
                    name: "ab_mode",
                    description: Some(
                        "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inalpha_op",
                    description: Some(
                        "The usage of the INALPHA[7:0]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA[7:0] is invalid, use the alpha value from previous pipeline 1: the INALPHA[7:0] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA[7:0] is used to scale the alpha value from previous pipeline Others: Reserved.",
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
                    name: "localpha_op",
                    description: Some(
                        "The usage of the LOCALPHA[7:0]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA[7:0] is invalid, use the alpha value from the data stream 1: the LOCALPHA[7:0] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA[7:0] is used to scale the alpha value from the data stream Others: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pixformat",
                    description: Some(
                        "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), the low byte contains the full R component. 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL[YUV_FORMAT] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4.",
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
                    name: "yuv_format",
                    description: Some(
                        "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT[0]: asserted to exchange sequence inside the bytes. Org [15:8]-->New[8:15], Org [7:0]-->New[0:7]. (First exchange) FORMAT[1]: asserted to exchange the sequence of the odd and even 8 bits. Org Even [7:0]-->New[15:8], Org Odd [15:8]-->New[7:0]. (Second exchange).",
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
                    name: "shadow_load_en",
                    description: Some(
                        "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters.",
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
                    name: "pack_dir",
                    description: Some(
                        "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order.",
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
            name: "Laypos",
            extends: None,
            description: Some(
                "Layer Position Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "x",
                    description: Some(
                        "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel.",
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
                    name: "y",
                    description: Some(
                        "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel.",
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
            name: "Laysize",
            extends: None,
            description: Some(
                "Layer Size Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "width",
                    description: Some(
                        "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16.",
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
                    name: "height",
                    description: Some(
                        "Height of the layer in pixels.",
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
            name: "Linecfg",
            extends: None,
            description: Some(
                "Layer Bus Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pitch",
                    description: Some(
                        "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundary.",
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
                    name: "max_ot",
                    description: Some(
                        "the number of outstanding axi read transactions. If zero, it means max 8.",
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
                    name: "mpt_size",
                    description: Some(
                        "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "St",
            extends: None,
            description: Some(
                "Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsync",
                    description: Some(
                        "Asserted when in vertical blanking period. At the end of VSYNC.",
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
                    name: "underrun",
                    description: Some(
                        "Asserted when the output buffer underrun condition encountered.",
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
                    name: "vs_blank",
                    description: Some(
                        "Asserted when in vertical blanking period. At the start of VSYNC.",
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
                    name: "urgent_underrun",
                    description: Some(
                        "Asserted when the output buffer urgent underrun condition encountered.",
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
            name: "Start0",
            extends: None,
            description: Some(
                "Layer Buffer Pointer Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr0",
                    description: Some(
                        "Input buffer Start address 0.",
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
            name: "Txfifo",
            extends: None,
            description: Some(
                "TX FIFO Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thrsh",
                    description: Some(
                        "Threshold to start the lcd raster (0--0x7F).",
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
            name: "VsyncPara",
            extends: None,
            description: Some(
                "VSYNC Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pw",
                    description: Some(
                        "VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1.",
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
                    name: "bp",
                    description: Some(
                        "VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fp",
                    description: Some(
                        "VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
