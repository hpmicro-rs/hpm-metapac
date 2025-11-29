
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PLIC",
        address: 0xe4000000,
        registers: Some(PeripheralRegisters {
            kind: "plic",
            version: "common",
            block: "PLIC",
            ir: &plic::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PLICSW",
        address: 0xe6400000,
        registers: Some(PeripheralRegisters {
            kind: "plicsw",
            version: "common",
            block: "PLICSW",
            ir: &plicsw::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCHTMR",
        address: 0xe6000000,
        registers: Some(PeripheralRegisters {
            kind: "mchtmr",
            version: "common",
            block: "MCHTMR",
            ir: &mchtmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "GPIO0_A",
        number: 1,
    },
    Interrupt {
        name: "GPIO0_B",
        number: 2,
    },
    Interrupt {
        name: "GPIO0_C",
        number: 3,
    },
    Interrupt {
        name: "GPIO0_D",
        number: 4,
    },
    Interrupt {
        name: "GPIO0_E",
        number: 5,
    },
    Interrupt {
        name: "GPIO0_F",
        number: 6,
    },
    Interrupt {
        name: "GPIO0_X",
        number: 7,
    },
    Interrupt {
        name: "GPIO0_Y",
        number: 8,
    },
    Interrupt {
        name: "GPIO0_Z",
        number: 9,
    },
    Interrupt {
        name: "MCAN0",
        number: 10,
    },
    Interrupt {
        name: "MCAN1",
        number: 11,
    },
    Interrupt {
        name: "MCAN2",
        number: 12,
    },
    Interrupt {
        name: "MCAN3",
        number: 13,
    },
    Interrupt {
        name: "MCAN4",
        number: 14,
    },
    Interrupt {
        name: "MCAN5",
        number: 15,
    },
    Interrupt {
        name: "MCAN6",
        number: 16,
    },
    Interrupt {
        name: "MCAN7",
        number: 17,
    },
    Interrupt {
        name: "PTPC",
        number: 18,
    },
    Interrupt {
        name: "UART0",
        number: 27,
    },
    Interrupt {
        name: "UART1",
        number: 28,
    },
    Interrupt {
        name: "UART2",
        number: 29,
    },
    Interrupt {
        name: "UART3",
        number: 30,
    },
    Interrupt {
        name: "UART4",
        number: 31,
    },
    Interrupt {
        name: "UART5",
        number: 32,
    },
    Interrupt {
        name: "UART6",
        number: 33,
    },
    Interrupt {
        name: "UART7",
        number: 34,
    },
    Interrupt {
        name: "I2C0",
        number: 35,
    },
    Interrupt {
        name: "I2C1",
        number: 36,
    },
    Interrupt {
        name: "I2C2",
        number: 37,
    },
    Interrupt {
        name: "I2C3",
        number: 38,
    },
    Interrupt {
        name: "SPI0",
        number: 39,
    },
    Interrupt {
        name: "SPI1",
        number: 40,
    },
    Interrupt {
        name: "SPI2",
        number: 41,
    },
    Interrupt {
        name: "SPI3",
        number: 42,
    },
    Interrupt {
        name: "GPTMR0",
        number: 43,
    },
    Interrupt {
        name: "GPTMR1",
        number: 44,
    },
    Interrupt {
        name: "GPTMR2",
        number: 45,
    },
    Interrupt {
        name: "GPTMR3",
        number: 46,
    },
    Interrupt {
        name: "GPTMR4",
        number: 47,
    },
    Interrupt {
        name: "GPTMR5",
        number: 48,
    },
    Interrupt {
        name: "GPTMR6",
        number: 49,
    },
    Interrupt {
        name: "GPTMR7",
        number: 50,
    },
    Interrupt {
        name: "EWDG0",
        number: 51,
    },
    Interrupt {
        name: "EWDG1",
        number: 52,
    },
    Interrupt {
        name: "MBX0A",
        number: 53,
    },
    Interrupt {
        name: "MBX0B",
        number: 54,
    },
    Interrupt {
        name: "MBX1A",
        number: 55,
    },
    Interrupt {
        name: "MBX1B",
        number: 56,
    },
    Interrupt {
        name: "RNG",
        number: 57,
    },
    Interrupt {
        name: "HDMA",
        number: 58,
    },
    Interrupt {
        name: "ADC0",
        number: 59,
    },
    Interrupt {
        name: "ADC1",
        number: 60,
    },
    Interrupt {
        name: "SDM",
        number: 61,
    },
    Interrupt {
        name: "OPAMP",
        number: 62,
    },
    Interrupt {
        name: "I2S0",
        number: 63,
    },
    Interrupt {
        name: "I2S1",
        number: 64,
    },
    Interrupt {
        name: "I2S2",
        number: 65,
    },
    Interrupt {
        name: "I2S3",
        number: 66,
    },
    Interrupt {
        name: "DAO",
        number: 67,
    },
    Interrupt {
        name: "PDM",
        number: 68,
    },
    Interrupt {
        name: "SMIX_DMA",
        number: 69,
    },
    Interrupt {
        name: "SMIX_ASRC",
        number: 70,
    },
    Interrupt {
        name: "CAM0",
        number: 71,
    },
    Interrupt {
        name: "CAM1",
        number: 72,
    },
    Interrupt {
        name: "LCDC",
        number: 73,
    },
    Interrupt {
        name: "LCDC1",
        number: 74,
    },
    Interrupt {
        name: "PDMA",
        number: 75,
    },
    Interrupt {
        name: "JPEG",
        number: 76,
    },
    Interrupt {
        name: "GWCK0_FUNC",
        number: 77,
    },
    Interrupt {
        name: "GWCK0_ERR",
        number: 78,
    },
    Interrupt {
        name: "GWCK1_FUNC",
        number: 79,
    },
    Interrupt {
        name: "GWCK1_ERR",
        number: 80,
    },
    Interrupt {
        name: "MIPI_DSI0",
        number: 81,
    },
    Interrupt {
        name: "MIPI_DSI1",
        number: 82,
    },
    Interrupt {
        name: "MIPI_CSI0",
        number: 83,
    },
    Interrupt {
        name: "MIPI_CSI0_AP",
        number: 84,
    },
    Interrupt {
        name: "MIPI_CSI0_DIAG",
        number: 85,
    },
    Interrupt {
        name: "MIPI_CSI1_AP",
        number: 86,
    },
    Interrupt {
        name: "MIPI_CSI1_DIAG",
        number: 87,
    },
    Interrupt {
        name: "MIPI_CSI1",
        number: 88,
    },
    Interrupt {
        name: "LCB0",
        number: 89,
    },
    Interrupt {
        name: "LCB1",
        number: 90,
    },
    Interrupt {
        name: "GPU",
        number: 91,
    },
    Interrupt {
        name: "ENET0",
        number: 92,
    },
    Interrupt {
        name: "NTMR0",
        number: 93,
    },
    Interrupt {
        name: "USB0",
        number: 94,
    },
    Interrupt {
        name: "SDXC0",
        number: 95,
    },
    Interrupt {
        name: "SDXC1",
        number: 96,
    },
    Interrupt {
        name: "SDP",
        number: 97,
    },
    Interrupt {
        name: "XPI0",
        number: 98,
    },
    Interrupt {
        name: "XDMA",
        number: 99,
    },
    Interrupt {
        name: "DDR",
        number: 100,
    },
    Interrupt {
        name: "FFA",
        number: 101,
    },
    Interrupt {
        name: "PSEC",
        number: 102,
    },
    Interrupt {
        name: "TSNS",
        number: 103,
    },
    Interrupt {
        name: "VAD",
        number: 104,
    },
    Interrupt {
        name: "PGPIO",
        number: 105,
    },
    Interrupt {
        name: "PWDG",
        number: 106,
    },
    Interrupt {
        name: "PTMR",
        number: 107,
    },
    Interrupt {
        name: "PUART",
        number: 108,
    },
    Interrupt {
        name: "FUSE",
        number: 109,
    },
    Interrupt {
        name: "SECMON",
        number: 110,
    },
    Interrupt {
        name: "RTC",
        number: 111,
    },
    Interrupt {
        name: "BGPIO",
        number: 112,
    },
    Interrupt {
        name: "BVIO",
        number: 113,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 114,
    },
    Interrupt {
        name: "SYSCTL",
        number: 115,
    },
    Interrupt {
        name: "DEBUG0",
        number: 116,
    },
    Interrupt {
        name: "DEBUG1",
        number: 117,
    },
    Interrupt {
        name: "CORE_LOCAL",
        number: 0,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "HDMA_CH0",
        dma: "HDMA",
        channel: 0,
        dmamux_channel: 0,
    },
    DmaChannel {
        name: "HDMA_CH1",
        dma: "HDMA",
        channel: 1,
        dmamux_channel: 1,
    },
    DmaChannel {
        name: "HDMA_CH2",
        dma: "HDMA",
        channel: 2,
        dmamux_channel: 2,
    },
    DmaChannel {
        name: "HDMA_CH3",
        dma: "HDMA",
        channel: 3,
        dmamux_channel: 3,
    },
    DmaChannel {
        name: "HDMA_CH4",
        dma: "HDMA",
        channel: 4,
        dmamux_channel: 4,
    },
    DmaChannel {
        name: "HDMA_CH5",
        dma: "HDMA",
        channel: 5,
        dmamux_channel: 5,
    },
    DmaChannel {
        name: "HDMA_CH6",
        dma: "HDMA",
        channel: 6,
        dmamux_channel: 6,
    },
    DmaChannel {
        name: "HDMA_CH7",
        dma: "HDMA",
        channel: 7,
        dmamux_channel: 7,
    },
    DmaChannel {
        name: "HDMA_CH8",
        dma: "HDMA",
        channel: 8,
        dmamux_channel: 8,
    },
    DmaChannel {
        name: "HDMA_CH9",
        dma: "HDMA",
        channel: 9,
        dmamux_channel: 9,
    },
    DmaChannel {
        name: "HDMA_CH10",
        dma: "HDMA",
        channel: 10,
        dmamux_channel: 10,
    },
    DmaChannel {
        name: "HDMA_CH11",
        dma: "HDMA",
        channel: 11,
        dmamux_channel: 11,
    },
    DmaChannel {
        name: "HDMA_CH12",
        dma: "HDMA",
        channel: 12,
        dmamux_channel: 12,
    },
    DmaChannel {
        name: "HDMA_CH13",
        dma: "HDMA",
        channel: 13,
        dmamux_channel: 13,
    },
    DmaChannel {
        name: "HDMA_CH14",
        dma: "HDMA",
        channel: 14,
        dmamux_channel: 14,
    },
    DmaChannel {
        name: "HDMA_CH15",
        dma: "HDMA",
        channel: 15,
        dmamux_channel: 15,
    },
    DmaChannel {
        name: "HDMA_CH16",
        dma: "HDMA",
        channel: 16,
        dmamux_channel: 16,
    },
    DmaChannel {
        name: "HDMA_CH17",
        dma: "HDMA",
        channel: 17,
        dmamux_channel: 17,
    },
    DmaChannel {
        name: "HDMA_CH18",
        dma: "HDMA",
        channel: 18,
        dmamux_channel: 18,
    },
    DmaChannel {
        name: "HDMA_CH19",
        dma: "HDMA",
        channel: 19,
        dmamux_channel: 19,
    },
    DmaChannel {
        name: "HDMA_CH20",
        dma: "HDMA",
        channel: 20,
        dmamux_channel: 20,
    },
    DmaChannel {
        name: "HDMA_CH21",
        dma: "HDMA",
        channel: 21,
        dmamux_channel: 21,
    },
    DmaChannel {
        name: "HDMA_CH22",
        dma: "HDMA",
        channel: 22,
        dmamux_channel: 22,
    },
    DmaChannel {
        name: "HDMA_CH23",
        dma: "HDMA",
        channel: 23,
        dmamux_channel: 23,
    },
    DmaChannel {
        name: "HDMA_CH24",
        dma: "HDMA",
        channel: 24,
        dmamux_channel: 24,
    },
    DmaChannel {
        name: "HDMA_CH25",
        dma: "HDMA",
        channel: 25,
        dmamux_channel: 25,
    },
    DmaChannel {
        name: "HDMA_CH26",
        dma: "HDMA",
        channel: 26,
        dmamux_channel: 26,
    },
    DmaChannel {
        name: "HDMA_CH27",
        dma: "HDMA",
        channel: 27,
        dmamux_channel: 27,
    },
    DmaChannel {
        name: "HDMA_CH28",
        dma: "HDMA",
        channel: 28,
        dmamux_channel: 28,
    },
    DmaChannel {
        name: "HDMA_CH29",
        dma: "HDMA",
        channel: 29,
        dmamux_channel: 29,
    },
    DmaChannel {
        name: "HDMA_CH30",
        dma: "HDMA",
        channel: 30,
        dmamux_channel: 30,
    },
    DmaChannel {
        name: "HDMA_CH31",
        dma: "HDMA",
        channel: 31,
        dmamux_channel: 31,
    },
];
pub(crate) static RESOURCES: &[Resource] = &[
    Resource {
        name: "CPU0",
        index: 0,
    },
    Resource {
        name: "CPX0",
        index: 1,
    },
    Resource {
        name: "POW_VIS",
        index: 21,
    },
    Resource {
        name: "POW_CPU0",
        index: 22,
    },
    Resource {
        name: "POW_GPU",
        index: 23,
    },
    Resource {
        name: "RST_SOC",
        index: 25,
    },
    Resource {
        name: "RST_CON",
        index: 26,
    },
    Resource {
        name: "RST_VIS",
        index: 27,
    },
    Resource {
        name: "RST_CPU0",
        index: 28,
    },
    Resource {
        name: "RST_GPU",
        index: 29,
    },
    Resource {
        name: "CLK_SRC_XTAL",
        index: 32,
    },
    Resource {
        name: "CLK_SRC_PLL0",
        index: 33,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL0",
        index: 34,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 35,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL1",
        index: 36,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL1",
        index: 37,
    },
    Resource {
        name: "CLK_SRC_PLL2",
        index: 38,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL2",
        index: 39,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL2",
        index: 40,
    },
    Resource {
        name: "CLK_SRC_PLL3",
        index: 41,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL3",
        index: 42,
    },
    Resource {
        name: "CLK_SRC_PLL4",
        index: 43,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL4",
        index: 44,
    },
    Resource {
        name: "CLK_SRC_PLL0_REF",
        index: 45,
    },
    Resource {
        name: "CLK_SRC_PLL1_REF",
        index: 46,
    },
    Resource {
        name: "CLK_SRC_PLL2_REF",
        index: 47,
    },
    Resource {
        name: "CLK_SRC_PLL3_REF",
        index: 48,
    },
    Resource {
        name: "CLK_SRC_PLL4_REF",
        index: 49,
    },
    Resource {
        name: "CLK_TOP_CPU0",
        index: 64,
    },
    Resource {
        name: "CLK_TOP_MCT0",
        index: 65,
    },
    Resource {
        name: "CLK_TOP_GPU0",
        index: 66,
    },
    Resource {
        name: "CLK_TOP_AXIF",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_AXIS",
        index: 68,
    },
    Resource {
        name: "CLK_TOP_AXIC",
        index: 69,
    },
    Resource {
        name: "CLK_TOP_AXIV",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_AXID",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_CAN4",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_CAN5",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_CAN6",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_CAN7",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_LIN0",
        index: 80,
    },
    Resource {
        name: "CLK_TOP_LIN1",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_LIN2",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_LIN3",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_LIN4",
        index: 84,
    },
    Resource {
        name: "CLK_TOP_LIN5",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_LIN6",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_LIN7",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 99,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 101,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 104,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 105,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 106,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 107,
    },
    Resource {
        name: "CLK_TOP_TMR4",
        index: 108,
    },
    Resource {
        name: "CLK_TOP_TMR5",
        index: 109,
    },
    Resource {
        name: "CLK_TOP_TMR6",
        index: 110,
    },
    Resource {
        name: "CLK_TOP_TMR7",
        index: 111,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 112,
    },
    Resource {
        name: "CLK_TOP_XRAM",
        index: 113,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 114,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 115,
    },
    Resource {
        name: "CLK_TOP_AUD0",
        index: 116,
    },
    Resource {
        name: "CLK_TOP_AUD1",
        index: 117,
    },
    Resource {
        name: "CLK_TOP_AUD2",
        index: 118,
    },
    Resource {
        name: "CLK_TOP_AUD3",
        index: 119,
    },
    Resource {
        name: "CLK_TOP_ETH0",
        index: 120,
    },
    Resource {
        name: "CLK_TOP_PTP0",
        index: 121,
    },
    Resource {
        name: "CLK_TOP_SDC0",
        index: 122,
    },
    Resource {
        name: "CLK_TOP_SDC1",
        index: 123,
    },
    Resource {
        name: "CLK_TOP_NTM0",
        index: 124,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 125,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 126,
    },
    Resource {
        name: "CLK_TOP_CAM0",
        index: 127,
    },
    Resource {
        name: "CLK_TOP_CAM1",
        index: 128,
    },
    Resource {
        name: "CLK_TOP_LCD0",
        index: 129,
    },
    Resource {
        name: "CLK_TOP_LCD1",
        index: 130,
    },
    Resource {
        name: "CLK_TOP_CSI0",
        index: 131,
    },
    Resource {
        name: "CLK_TOP_CSI1",
        index: 132,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 133,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 134,
    },
    Resource {
        name: "CLK_TOP_I2S0",
        index: 135,
    },
    Resource {
        name: "CLK_TOP_I2S1",
        index: 136,
    },
    Resource {
        name: "CLK_TOP_I2S2",
        index: 137,
    },
    Resource {
        name: "CLK_TOP_I2S3",
        index: 138,
    },
    Resource {
        name: "AXIS",
        index: 256,
    },
    Resource {
        name: "AXIC",
        index: 257,
    },
    Resource {
        name: "AXIV",
        index: 258,
    },
    Resource {
        name: "AXIG",
        index: 259,
    },
    Resource {
        name: "LMM0",
        index: 260,
    },
    Resource {
        name: "MCT0",
        index: 261,
    },
    Resource {
        name: "ROM0",
        index: 262,
    },
    Resource {
        name: "DDR0",
        index: 263,
    },
    Resource {
        name: "XRAM",
        index: 264,
    },
    Resource {
        name: "CAN0",
        index: 265,
    },
    Resource {
        name: "CAN1",
        index: 266,
    },
    Resource {
        name: "CAN2",
        index: 267,
    },
    Resource {
        name: "CAN3",
        index: 268,
    },
    Resource {
        name: "CAN4",
        index: 269,
    },
    Resource {
        name: "CAN5",
        index: 270,
    },
    Resource {
        name: "CAN6",
        index: 271,
    },
    Resource {
        name: "CAN7",
        index: 272,
    },
    Resource {
        name: "PTPC",
        index: 273,
    },
    Resource {
        name: "CRC0",
        index: 274,
    },
    Resource {
        name: "OAMP",
        index: 275,
    },
    Resource {
        name: "LIN0",
        index: 276,
    },
    Resource {
        name: "LIN1",
        index: 277,
    },
    Resource {
        name: "LIN2",
        index: 278,
    },
    Resource {
        name: "LIN3",
        index: 279,
    },
    Resource {
        name: "LIN4",
        index: 280,
    },
    Resource {
        name: "LIN5",
        index: 281,
    },
    Resource {
        name: "LIN6",
        index: 282,
    },
    Resource {
        name: "LIN7",
        index: 283,
    },
    Resource {
        name: "I2C0",
        index: 284,
    },
    Resource {
        name: "I2C1",
        index: 285,
    },
    Resource {
        name: "I2C2",
        index: 286,
    },
    Resource {
        name: "I2C3",
        index: 287,
    },
    Resource {
        name: "SPI0",
        index: 288,
    },
    Resource {
        name: "SPI1",
        index: 289,
    },
    Resource {
        name: "SPI2",
        index: 290,
    },
    Resource {
        name: "SPI3",
        index: 291,
    },
    Resource {
        name: "URT0",
        index: 292,
    },
    Resource {
        name: "URT1",
        index: 293,
    },
    Resource {
        name: "URT2",
        index: 294,
    },
    Resource {
        name: "URT3",
        index: 295,
    },
    Resource {
        name: "URT4",
        index: 296,
    },
    Resource {
        name: "URT5",
        index: 297,
    },
    Resource {
        name: "URT6",
        index: 298,
    },
    Resource {
        name: "URT7",
        index: 299,
    },
    Resource {
        name: "WDG0",
        index: 300,
    },
    Resource {
        name: "WDG1",
        index: 301,
    },
    Resource {
        name: "MBX0",
        index: 302,
    },
    Resource {
        name: "MBX1",
        index: 303,
    },
    Resource {
        name: "TMR0",
        index: 304,
    },
    Resource {
        name: "TMR1",
        index: 305,
    },
    Resource {
        name: "TMR2",
        index: 306,
    },
    Resource {
        name: "TMR3",
        index: 307,
    },
    Resource {
        name: "TMR4",
        index: 308,
    },
    Resource {
        name: "TMR5",
        index: 309,
    },
    Resource {
        name: "TMR6",
        index: 310,
    },
    Resource {
        name: "TMR7",
        index: 311,
    },
    Resource {
        name: "I2S0",
        index: 312,
    },
    Resource {
        name: "I2S1",
        index: 313,
    },
    Resource {
        name: "I2S2",
        index: 314,
    },
    Resource {
        name: "I2S3",
        index: 315,
    },
    Resource {
        name: "PDM0",
        index: 316,
    },
    Resource {
        name: "DAO0",
        index: 317,
    },
    Resource {
        name: "SMIX",
        index: 318,
    },
    Resource {
        name: "RNG0",
        index: 319,
    },
    Resource {
        name: "SDP0",
        index: 320,
    },
    Resource {
        name: "KMAN",
        index: 321,
    },
    Resource {
        name: "GPIO",
        index: 322,
    },
    Resource {
        name: "ADC0",
        index: 323,
    },
    Resource {
        name: "ADC1",
        index: 324,
    },
    Resource {
        name: "SDM0",
        index: 325,
    },
    Resource {
        name: "HDMA",
        index: 326,
    },
    Resource {
        name: "XDMA",
        index: 327,
    },
    Resource {
        name: "XPI0",
        index: 328,
    },
    Resource {
        name: "FFA0",
        index: 329,
    },
    Resource {
        name: "TSNS",
        index: 330,
    },
    Resource {
        name: "ETH0",
        index: 331,
    },
    Resource {
        name: "USB0",
        index: 332,
    },
    Resource {
        name: "SDC0",
        index: 333,
    },
    Resource {
        name: "SDC1",
        index: 334,
    },
    Resource {
        name: "NTM0",
        index: 335,
    },
    Resource {
        name: "REF0",
        index: 336,
    },
    Resource {
        name: "REF1",
        index: 337,
    },
    Resource {
        name: "CAM0",
        index: 338,
    },
    Resource {
        name: "CAM1",
        index: 339,
    },
    Resource {
        name: "PDMA",
        index: 340,
    },
    Resource {
        name: "JPEG",
        index: 341,
    },
    Resource {
        name: "LCD0",
        index: 342,
    },
    Resource {
        name: "LCD1",
        index: 343,
    },
    Resource {
        name: "GWC0",
        index: 344,
    },
    Resource {
        name: "GWC1",
        index: 345,
    },
    Resource {
        name: "CSI0",
        index: 346,
    },
    Resource {
        name: "CSI1",
        index: 347,
    },
    Resource {
        name: "DSI0",
        index: 348,
    },
    Resource {
        name: "DSI1",
        index: 349,
    },
    Resource {
        name: "LVB0",
        index: 350,
    },
    Resource {
        name: "LCB0",
        index: 351,
    },
    Resource {
        name: "GPU0",
        index: 352,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "CPU0",
        index: 0,
    },
    Clock {
        name: "MCT0",
        index: 1,
    },
    Clock {
        name: "GPU0",
        index: 2,
    },
    Clock {
        name: "AXIF",
        index: 3,
    },
    Clock {
        name: "AXIS",
        index: 4,
    },
    Clock {
        name: "AXIC",
        index: 5,
    },
    Clock {
        name: "AXIV",
        index: 6,
    },
    Clock {
        name: "AXID",
        index: 7,
    },
    Clock {
        name: "CAN0",
        index: 8,
    },
    Clock {
        name: "CAN1",
        index: 9,
    },
    Clock {
        name: "CAN2",
        index: 10,
    },
    Clock {
        name: "CAN3",
        index: 11,
    },
    Clock {
        name: "CAN4",
        index: 12,
    },
    Clock {
        name: "CAN5",
        index: 13,
    },
    Clock {
        name: "CAN6",
        index: 14,
    },
    Clock {
        name: "CAN7",
        index: 15,
    },
    Clock {
        name: "LIN0",
        index: 16,
    },
    Clock {
        name: "LIN1",
        index: 17,
    },
    Clock {
        name: "LIN2",
        index: 18,
    },
    Clock {
        name: "LIN3",
        index: 19,
    },
    Clock {
        name: "LIN4",
        index: 20,
    },
    Clock {
        name: "LIN5",
        index: 21,
    },
    Clock {
        name: "LIN6",
        index: 22,
    },
    Clock {
        name: "LIN7",
        index: 23,
    },
    Clock {
        name: "I2C0",
        index: 24,
    },
    Clock {
        name: "I2C1",
        index: 25,
    },
    Clock {
        name: "I2C2",
        index: 26,
    },
    Clock {
        name: "I2C3",
        index: 27,
    },
    Clock {
        name: "SPI0",
        index: 28,
    },
    Clock {
        name: "SPI1",
        index: 29,
    },
    Clock {
        name: "SPI2",
        index: 30,
    },
    Clock {
        name: "SPI3",
        index: 31,
    },
    Clock {
        name: "URT0",
        index: 32,
    },
    Clock {
        name: "URT1",
        index: 33,
    },
    Clock {
        name: "URT2",
        index: 34,
    },
    Clock {
        name: "URT3",
        index: 35,
    },
    Clock {
        name: "URT4",
        index: 36,
    },
    Clock {
        name: "URT5",
        index: 37,
    },
    Clock {
        name: "URT6",
        index: 38,
    },
    Clock {
        name: "URT7",
        index: 39,
    },
    Clock {
        name: "TMR0",
        index: 40,
    },
    Clock {
        name: "TMR1",
        index: 41,
    },
    Clock {
        name: "TMR2",
        index: 42,
    },
    Clock {
        name: "TMR3",
        index: 43,
    },
    Clock {
        name: "TMR4",
        index: 44,
    },
    Clock {
        name: "TMR5",
        index: 45,
    },
    Clock {
        name: "TMR6",
        index: 46,
    },
    Clock {
        name: "TMR7",
        index: 47,
    },
    Clock {
        name: "XPI0",
        index: 48,
    },
    Clock {
        name: "XRAM",
        index: 49,
    },
    Clock {
        name: "ANA0",
        index: 50,
    },
    Clock {
        name: "ANA1",
        index: 51,
    },
    Clock {
        name: "AUD0",
        index: 52,
    },
    Clock {
        name: "AUD1",
        index: 53,
    },
    Clock {
        name: "AUD2",
        index: 54,
    },
    Clock {
        name: "AUD3",
        index: 55,
    },
    Clock {
        name: "ETH0",
        index: 56,
    },
    Clock {
        name: "PTP0",
        index: 57,
    },
    Clock {
        name: "SDC0",
        index: 58,
    },
    Clock {
        name: "SDC1",
        index: 59,
    },
    Clock {
        name: "NTM0",
        index: 60,
    },
    Clock {
        name: "REF0",
        index: 61,
    },
    Clock {
        name: "REF1",
        index: 62,
    },
    Clock {
        name: "CAM0",
        index: 63,
    },
    Clock {
        name: "CAM1",
        index: 64,
    },
    Clock {
        name: "LCD0",
        index: 65,
    },
    Clock {
        name: "LCD1",
        index: 66,
    },
    Clock {
        name: "CSI0",
        index: 67,
    },
    Clock {
        name: "CSI1",
        index: 68,
    },
];
pub(crate) static PINS: &[IoPin] = &[
    IoPin {
        name: "PA00",
        index: 0,
    },
    IoPin {
        name: "PA01",
        index: 1,
    },
    IoPin {
        name: "PA02",
        index: 2,
    },
    IoPin {
        name: "PA03",
        index: 3,
    },
    IoPin {
        name: "PA04",
        index: 4,
    },
    IoPin {
        name: "PA05",
        index: 5,
    },
    IoPin {
        name: "PA06",
        index: 6,
    },
    IoPin {
        name: "PA07",
        index: 7,
    },
    IoPin {
        name: "PA08",
        index: 8,
    },
    IoPin {
        name: "PA09",
        index: 9,
    },
    IoPin {
        name: "PA10",
        index: 10,
    },
    IoPin {
        name: "PA11",
        index: 11,
    },
    IoPin {
        name: "PA12",
        index: 12,
    },
    IoPin {
        name: "PA13",
        index: 13,
    },
    IoPin {
        name: "PA14",
        index: 14,
    },
    IoPin {
        name: "PA15",
        index: 15,
    },
    IoPin {
        name: "PA16",
        index: 16,
    },
    IoPin {
        name: "PA17",
        index: 17,
    },
    IoPin {
        name: "PA18",
        index: 18,
    },
    IoPin {
        name: "PA19",
        index: 19,
    },
    IoPin {
        name: "PA20",
        index: 20,
    },
    IoPin {
        name: "PA21",
        index: 21,
    },
    IoPin {
        name: "PA22",
        index: 22,
    },
    IoPin {
        name: "PA23",
        index: 23,
    },
    IoPin {
        name: "PA24",
        index: 24,
    },
    IoPin {
        name: "PA25",
        index: 25,
    },
    IoPin {
        name: "PA26",
        index: 26,
    },
    IoPin {
        name: "PA27",
        index: 27,
    },
    IoPin {
        name: "PA28",
        index: 28,
    },
    IoPin {
        name: "PA29",
        index: 29,
    },
    IoPin {
        name: "PA30",
        index: 30,
    },
    IoPin {
        name: "PA31",
        index: 31,
    },
    IoPin {
        name: "PB00",
        index: 32,
    },
    IoPin {
        name: "PB01",
        index: 33,
    },
    IoPin {
        name: "PB02",
        index: 34,
    },
    IoPin {
        name: "PB03",
        index: 35,
    },
    IoPin {
        name: "PB04",
        index: 36,
    },
    IoPin {
        name: "PB05",
        index: 37,
    },
    IoPin {
        name: "PB06",
        index: 38,
    },
    IoPin {
        name: "PB07",
        index: 39,
    },
    IoPin {
        name: "PB08",
        index: 40,
    },
    IoPin {
        name: "PB09",
        index: 41,
    },
    IoPin {
        name: "PB10",
        index: 42,
    },
    IoPin {
        name: "PB11",
        index: 43,
    },
    IoPin {
        name: "PB12",
        index: 44,
    },
    IoPin {
        name: "PB13",
        index: 45,
    },
    IoPin {
        name: "PB14",
        index: 46,
    },
    IoPin {
        name: "PB15",
        index: 47,
    },
    IoPin {
        name: "PB16",
        index: 48,
    },
    IoPin {
        name: "PB17",
        index: 49,
    },
    IoPin {
        name: "PB18",
        index: 50,
    },
    IoPin {
        name: "PB19",
        index: 51,
    },
    IoPin {
        name: "PB20",
        index: 52,
    },
    IoPin {
        name: "PB21",
        index: 53,
    },
    IoPin {
        name: "PB22",
        index: 54,
    },
    IoPin {
        name: "PB23",
        index: 55,
    },
    IoPin {
        name: "PB24",
        index: 56,
    },
    IoPin {
        name: "PB25",
        index: 57,
    },
    IoPin {
        name: "PB26",
        index: 58,
    },
    IoPin {
        name: "PB27",
        index: 59,
    },
    IoPin {
        name: "PB28",
        index: 60,
    },
    IoPin {
        name: "PB29",
        index: 61,
    },
    IoPin {
        name: "PB30",
        index: 62,
    },
    IoPin {
        name: "PB31",
        index: 63,
    },
    IoPin {
        name: "PC00",
        index: 64,
    },
    IoPin {
        name: "PC01",
        index: 65,
    },
    IoPin {
        name: "PC02",
        index: 66,
    },
    IoPin {
        name: "PC03",
        index: 67,
    },
    IoPin {
        name: "PC04",
        index: 68,
    },
    IoPin {
        name: "PC05",
        index: 69,
    },
    IoPin {
        name: "PC06",
        index: 70,
    },
    IoPin {
        name: "PC07",
        index: 71,
    },
    IoPin {
        name: "PC08",
        index: 72,
    },
    IoPin {
        name: "PC09",
        index: 73,
    },
    IoPin {
        name: "PC10",
        index: 74,
    },
    IoPin {
        name: "PC11",
        index: 75,
    },
    IoPin {
        name: "PC12",
        index: 76,
    },
    IoPin {
        name: "PC13",
        index: 77,
    },
    IoPin {
        name: "PC14",
        index: 78,
    },
    IoPin {
        name: "PC15",
        index: 79,
    },
    IoPin {
        name: "PC16",
        index: 80,
    },
    IoPin {
        name: "PC17",
        index: 81,
    },
    IoPin {
        name: "PC18",
        index: 82,
    },
    IoPin {
        name: "PC19",
        index: 83,
    },
    IoPin {
        name: "PC20",
        index: 84,
    },
    IoPin {
        name: "PC21",
        index: 85,
    },
    IoPin {
        name: "PC22",
        index: 86,
    },
    IoPin {
        name: "PC23",
        index: 87,
    },
    IoPin {
        name: "PC24",
        index: 88,
    },
    IoPin {
        name: "PC25",
        index: 89,
    },
    IoPin {
        name: "PC26",
        index: 90,
    },
    IoPin {
        name: "PC27",
        index: 91,
    },
    IoPin {
        name: "PC28",
        index: 92,
    },
    IoPin {
        name: "PC29",
        index: 93,
    },
    IoPin {
        name: "PC30",
        index: 94,
    },
    IoPin {
        name: "PC31",
        index: 95,
    },
    IoPin {
        name: "PD00",
        index: 96,
    },
    IoPin {
        name: "PD01",
        index: 97,
    },
    IoPin {
        name: "PD02",
        index: 98,
    },
    IoPin {
        name: "PD03",
        index: 99,
    },
    IoPin {
        name: "PD04",
        index: 100,
    },
    IoPin {
        name: "PD05",
        index: 101,
    },
    IoPin {
        name: "PD06",
        index: 102,
    },
    IoPin {
        name: "PD07",
        index: 103,
    },
    IoPin {
        name: "PD08",
        index: 104,
    },
    IoPin {
        name: "PD09",
        index: 105,
    },
    IoPin {
        name: "PD10",
        index: 106,
    },
    IoPin {
        name: "PD11",
        index: 107,
    },
    IoPin {
        name: "PD12",
        index: 108,
    },
    IoPin {
        name: "PD13",
        index: 109,
    },
    IoPin {
        name: "PD14",
        index: 110,
    },
    IoPin {
        name: "PD15",
        index: 111,
    },
    IoPin {
        name: "PD16",
        index: 112,
    },
    IoPin {
        name: "PD17",
        index: 113,
    },
    IoPin {
        name: "PD18",
        index: 114,
    },
    IoPin {
        name: "PD19",
        index: 115,
    },
    IoPin {
        name: "PD20",
        index: 116,
    },
    IoPin {
        name: "PD21",
        index: 117,
    },
    IoPin {
        name: "PD22",
        index: 118,
    },
    IoPin {
        name: "PD23",
        index: 119,
    },
    IoPin {
        name: "PD24",
        index: 120,
    },
    IoPin {
        name: "PD25",
        index: 121,
    },
    IoPin {
        name: "PD26",
        index: 122,
    },
    IoPin {
        name: "PD27",
        index: 123,
    },
    IoPin {
        name: "PD28",
        index: 124,
    },
    IoPin {
        name: "PD29",
        index: 125,
    },
    IoPin {
        name: "PD30",
        index: 126,
    },
    IoPin {
        name: "PD31",
        index: 127,
    },
    IoPin {
        name: "PE00",
        index: 128,
    },
    IoPin {
        name: "PE01",
        index: 129,
    },
    IoPin {
        name: "PE02",
        index: 130,
    },
    IoPin {
        name: "PE03",
        index: 131,
    },
    IoPin {
        name: "PE04",
        index: 132,
    },
    IoPin {
        name: "PE05",
        index: 133,
    },
    IoPin {
        name: "PE06",
        index: 134,
    },
    IoPin {
        name: "PE07",
        index: 135,
    },
    IoPin {
        name: "PE08",
        index: 136,
    },
    IoPin {
        name: "PE09",
        index: 137,
    },
    IoPin {
        name: "PE10",
        index: 138,
    },
    IoPin {
        name: "PE11",
        index: 139,
    },
    IoPin {
        name: "PE12",
        index: 140,
    },
    IoPin {
        name: "PE13",
        index: 141,
    },
    IoPin {
        name: "PE14",
        index: 142,
    },
    IoPin {
        name: "PE15",
        index: 143,
    },
    IoPin {
        name: "PE16",
        index: 144,
    },
    IoPin {
        name: "PE17",
        index: 145,
    },
    IoPin {
        name: "PE18",
        index: 146,
    },
    IoPin {
        name: "PE19",
        index: 147,
    },
    IoPin {
        name: "PE20",
        index: 148,
    },
    IoPin {
        name: "PE21",
        index: 149,
    },
    IoPin {
        name: "PE22",
        index: 150,
    },
    IoPin {
        name: "PE23",
        index: 151,
    },
    IoPin {
        name: "PE24",
        index: 152,
    },
    IoPin {
        name: "PE25",
        index: 153,
    },
    IoPin {
        name: "PE26",
        index: 154,
    },
    IoPin {
        name: "PE27",
        index: 155,
    },
    IoPin {
        name: "PE28",
        index: 156,
    },
    IoPin {
        name: "PE29",
        index: 157,
    },
    IoPin {
        name: "PE30",
        index: 158,
    },
    IoPin {
        name: "PE31",
        index: 159,
    },
    IoPin {
        name: "PF00",
        index: 160,
    },
    IoPin {
        name: "PF01",
        index: 161,
    },
    IoPin {
        name: "PF02",
        index: 162,
    },
    IoPin {
        name: "PF03",
        index: 163,
    },
    IoPin {
        name: "PF04",
        index: 164,
    },
    IoPin {
        name: "PF05",
        index: 165,
    },
    IoPin {
        name: "PF06",
        index: 166,
    },
    IoPin {
        name: "PF07",
        index: 167,
    },
    IoPin {
        name: "PF08",
        index: 168,
    },
    IoPin {
        name: "PF09",
        index: 169,
    },
    IoPin {
        name: "PF10",
        index: 170,
    },
    IoPin {
        name: "PF11",
        index: 171,
    },
    IoPin {
        name: "PF12",
        index: 172,
    },
    IoPin {
        name: "PF13",
        index: 173,
    },
    IoPin {
        name: "PF14",
        index: 174,
    },
    IoPin {
        name: "PF15",
        index: 175,
    },
    IoPin {
        name: "PX00",
        index: 416,
    },
    IoPin {
        name: "PX01",
        index: 417,
    },
    IoPin {
        name: "PX02",
        index: 418,
    },
    IoPin {
        name: "PX03",
        index: 419,
    },
    IoPin {
        name: "PX04",
        index: 420,
    },
    IoPin {
        name: "PX05",
        index: 421,
    },
    IoPin {
        name: "PX06",
        index: 422,
    },
    IoPin {
        name: "PX07",
        index: 423,
    },
    IoPin {
        name: "PX08",
        index: 424,
    },
    IoPin {
        name: "PX09",
        index: 425,
    },
    IoPin {
        name: "PX10",
        index: 426,
    },
    IoPin {
        name: "PX11",
        index: 427,
    },
    IoPin {
        name: "PX12",
        index: 428,
    },
    IoPin {
        name: "PX13",
        index: 429,
    },
    IoPin {
        name: "PX14",
        index: 430,
    },
    IoPin {
        name: "PX15",
        index: 431,
    },
    IoPin {
        name: "PY00",
        index: 448,
    },
    IoPin {
        name: "PY01",
        index: 449,
    },
    IoPin {
        name: "PY02",
        index: 450,
    },
    IoPin {
        name: "PY03",
        index: 451,
    },
    IoPin {
        name: "PY04",
        index: 452,
    },
    IoPin {
        name: "PY05",
        index: 453,
    },
    IoPin {
        name: "PY06",
        index: 454,
    },
    IoPin {
        name: "PY07",
        index: 455,
    },
    IoPin {
        name: "PY08",
        index: 456,
    },
    IoPin {
        name: "PY09",
        index: 457,
    },
    IoPin {
        name: "PY10",
        index: 458,
    },
    IoPin {
        name: "PY11",
        index: 459,
    },
    IoPin {
        name: "PY12",
        index: 460,
    },
    IoPin {
        name: "PY13",
        index: 461,
    },
    IoPin {
        name: "PY14",
        index: 462,
    },
    IoPin {
        name: "PY15",
        index: 463,
    },
    IoPin {
        name: "PZ00",
        index: 480,
    },
    IoPin {
        name: "PZ01",
        index: 481,
    },
    IoPin {
        name: "PZ02",
        index: 482,
    },
    IoPin {
        name: "PZ03",
        index: 483,
    },
    IoPin {
        name: "PZ04",
        index: 484,
    },
    IoPin {
        name: "PZ05",
        index: 485,
    },
    IoPin {
        name: "PZ06",
        index: 486,
    },
    IoPin {
        name: "PZ07",
        index: 487,
    },
    IoPin {
        name: "PZ08",
        index: 488,
    },
    IoPin {
        name: "PZ09",
        index: 489,
    },
    IoPin {
        name: "PZ10",
        index: 490,
    },
    IoPin {
        name: "PZ11",
        index: 491,
    },
    IoPin {
        name: "PZ12",
        index: 492,
    },
    IoPin {
        name: "PZ13",
        index: 493,
    },
    IoPin {
        name: "PZ14",
        index: 494,
    },
    IoPin {
        name: "PZ15",
        index: 495,
    },
];
pub(crate) static TRGMMUX: &[TrgmMux] = &[];
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
