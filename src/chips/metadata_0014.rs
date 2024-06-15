
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
pub(crate) static INTERRUPTS: &[Interrupt] = &[];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
pub(crate) static RESOURCES: &[Resource] = &[
    Resource {
        name: "CLK_TOP_AHB",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_XPI1",
        index: 74,
    },
    Resource {
        name: "REF1",
        index: 349,
    },
    Resource {
        name: "SYNT",
        index: 330,
    },
    Resource {
        name: "UART10",
        index: 300,
    },
    Resource {
        name: "POW_CPU0",
        index: 23,
    },
    Resource {
        name: "CLK_TOP_UART0",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_LCDC",
        index: 118,
    },
    Resource {
        name: "I2S0",
        index: 324,
    },
    Resource {
        name: "CLK_TOP_AUD1",
        index: 116,
    },
    Resource {
        name: "MBX1",
        index: 277,
    },
    Resource {
        name: "CAM0",
        index: 336,
    },
    Resource {
        name: "CLK_TOP_I2S2",
        index: 198,
    },
    Resource {
        name: "CLK_SRC_PLL4CLK0",
        index: 44,
    },
    Resource {
        name: "CLK_TOP_UART9",
        index: 92,
    },
    Resource {
        name: "CLK_SRC_PLL2",
        index: 38,
    },
    Resource {
        name: "LMM1",
        index: 263,
    },
    Resource {
        name: "GPTMR1",
        index: 283,
    },
    Resource {
        name: "CLK_SRC_PLL2CLK0",
        index: 39,
    },
    Resource {
        name: "CLK_TOP_NTMR0",
        index: 127,
    },
    Resource {
        name: "CLK_TOP_ENET1",
        index: 122,
    },
    Resource {
        name: "SPI1",
        index: 311,
    },
    Resource {
        name: "CAN2",
        index: 316,
    },
    Resource {
        name: "CONN_BUS",
        index: 258,
    },
    Resource {
        name: "JPEG",
        index: 338,
    },
    Resource {
        name: "POW_CPU1",
        index: 24,
    },
    Resource {
        name: "RST_CON",
        index: 26,
    },
    Resource {
        name: "CLK_TOP_CPU1",
        index: 66,
    },
    Resource {
        name: "KEYM",
        index: 272,
    },
    Resource {
        name: "CLK_TOP_UART11",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 114,
    },
    Resource {
        name: "CLK_TOP_UART4",
        index: 87,
    },
    Resource {
        name: "XPI1",
        index: 269,
    },
    Resource {
        name: "CLK_TOP_ADC2",
        index: 194,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 35,
    },
    Resource {
        name: "CLK_TOP_GPTMR2",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 192,
    },
    Resource {
        name: "CAN3",
        index: 317,
    },
    Resource {
        name: "PDMA",
        index: 339,
    },
    Resource {
        name: "SDXC1",
        index: 345,
    },
    Resource {
        name: "CLK_TOP_MCHTMR0",
        index: 65,
    },
    Resource {
        name: "PTPC",
        index: 318,
    },
    Resource {
        name: "GPTMR3",
        index: 285,
    },
    Resource {
        name: "CLK_TOP_GPTMR1",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_MCHTMR1",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_GPTMR7",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_ADC3",
        index: 195,
    },
    Resource {
        name: "CLK_TOP_UART13",
        index: 96,
    },
    Resource {
        name: "ENET1",
        index: 341,
    },
    Resource {
        name: "CLK_TOP_UART7",
        index: 90,
    },
    Resource {
        name: "UART15",
        index: 305,
    },
    Resource {
        name: "USB0",
        index: 346,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 193,
    },
    Resource {
        name: "XPI0",
        index: 268,
    },
    Resource {
        name: "SPI2",
        index: 312,
    },
    Resource {
        name: "CLK_SRC_PLL4",
        index: 43,
    },
    Resource {
        name: "ADC2",
        index: 321,
    },
    Resource {
        name: "UART9",
        index: 299,
    },
    Resource {
        name: "MOT1",
        index: 332,
    },
    Resource {
        name: "ACMP",
        index: 323,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 126,
    },
    Resource {
        name: "WDG3",
        index: 281,
    },
    Resource {
        name: "VIS_BUS",
        index: 259,
    },
    Resource {
        name: "CLK_TOP_AUD0",
        index: 115,
    },
    Resource {
        name: "CLK_TOP_UART8",
        index: 91,
    },
    Resource {
        name: "ADC1",
        index: 320,
    },
    Resource {
        name: "UART12",
        index: 302,
    },
    Resource {
        name: "CLK_TOP_NTMR1",
        index: 128,
    },
    Resource {
        name: "I2C1",
        index: 307,
    },
    Resource {
        name: "I2S2",
        index: 326,
    },
    Resource {
        name: "CLK_SRC_PLL2CLK1",
        index: 40,
    },
    Resource {
        name: "RST_CPU1",
        index: 29,
    },
    Resource {
        name: "CPX1_SUBSYS",
        index: 9,
    },
    Resource {
        name: "CLK_TOP_CPU0",
        index: 64,
    },
    Resource {
        name: "CLK_TOP_GPTMR0",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_GPTMR4",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 113,
    },
    Resource {
        name: "WDG2",
        index: 280,
    },
    Resource {
        name: "SPI3",
        index: 313,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 112,
    },
    Resource {
        name: "CLK_SRC_PLL1CLK0",
        index: 36,
    },
    Resource {
        name: "CLK_SRC_PLL1CLK1",
        index: 37,
    },
    Resource {
        name: "UART6",
        index: 296,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 108,
    },
    Resource {
        name: "I2S3",
        index: 327,
    },
    Resource {
        name: "CAM1",
        index: 337,
    },
    Resource {
        name: "XDMA",
        index: 274,
    },
    Resource {
        name: "SDXC0",
        index: 344,
    },
    Resource {
        name: "AHBAPB_BUS",
        index: 256,
    },
    Resource {
        name: "CLK_TOP_CAM1",
        index: 120,
    },
    Resource {
        name: "POW_CON",
        index: 21,
    },
    Resource {
        name: "GPTMR6",
        index: 288,
    },
    Resource {
        name: "ENET0",
        index: 340,
    },
    Resource {
        name: "RST_CPU0",
        index: 28,
    },
    Resource {
        name: "CPU0_SUBSYS",
        index: 1,
    },
    Resource {
        name: "UART8",
        index: 298,
    },
    Resource {
        name: "CLK_TOP_UART3",
        index: 86,
    },
    Resource {
        name: "CAN0",
        index: 314,
    },
    Resource {
        name: "ADC0",
        index: 319,
    },
    Resource {
        name: "CLK_TOP_UART12",
        index: 95,
    },
    Resource {
        name: "PDM",
        index: 328,
    },
    Resource {
        name: "NTMR1",
        index: 343,
    },
    Resource {
        name: "SPI0",
        index: 310,
    },
    Resource {
        name: "USB1",
        index: 347,
    },
    Resource {
        name: "CPU1_CORE",
        index: 8,
    },
    Resource {
        name: "UART5",
        index: 295,
    },
    Resource {
        name: "CLK_TOP_UART2",
        index: 85,
    },
    Resource {
        name: "CLK_SRC_PLL3CLK0",
        index: 42,
    },
    Resource {
        name: "CLK_TOP_VIS",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_GPTMR5",
        index: 80,
    },
    Resource {
        name: "CLK_TOP_GPTMR6",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 107,
    },
    Resource {
        name: "CLK_TOP_UART5",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 103,
    },
    Resource {
        name: "AXI_SRAM0",
        index: 266,
    },
    Resource {
        name: "CLK_TOP_I2S0",
        index: 196,
    },
    Resource {
        name: "AXI_SRAM1",
        index: 267,
    },
    Resource {
        name: "GPTMR2",
        index: 284,
    },
    Resource {
        name: "REF0",
        index: 348,
    },
    Resource {
        name: "GPTMR5",
        index: 287,
    },
    Resource {
        name: "GPTMR4",
        index: 286,
    },
    Resource {
        name: "I2C2",
        index: 308,
    },
    Resource {
        name: "I2S1",
        index: 325,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 109,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 101,
    },
    Resource {
        name: "UART0",
        index: 290,
    },
    Resource {
        name: "CLK_TOP_I2S1",
        index: 197,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 73,
    },
    Resource {
        name: "GPTMR7",
        index: 289,
    },
    Resource {
        name: "UART14",
        index: 304,
    },
    Resource {
        name: "RST_SOC",
        index: 25,
    },
    Resource {
        name: "CLK_SRC_PLL3",
        index: 41,
    },
    Resource {
        name: "CLK_TOP_SDXC1",
        index: 130,
    },
    Resource {
        name: "CLK_SRC_XTAL",
        index: 32,
    },
    Resource {
        name: "FEMC",
        index: 260,
    },
    Resource {
        name: "UART2",
        index: 292,
    },
    Resource {
        name: "UART7",
        index: 297,
    },
    Resource {
        name: "CAN1",
        index: 315,
    },
    Resource {
        name: "ADC3",
        index: 322,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 106,
    },
    Resource {
        name: "MCHTMR1",
        index: 265,
    },
    Resource {
        name: "MOT0",
        index: 331,
    },
    Resource {
        name: "CLK_TOP_FEMC",
        index: 72,
    },
    Resource {
        name: "MOT3",
        index: 334,
    },
    Resource {
        name: "LCDC",
        index: 335,
    },
    Resource {
        name: "CLK_TOP_PTP0",
        index: 123,
    },
    Resource {
        name: "ROM",
        index: 261,
    },
    Resource {
        name: "CLK_SRC_PLL0",
        index: 33,
    },
    Resource {
        name: "CLK_TOP_AXI",
        index: 68,
    },
    Resource {
        name: "RNG",
        index: 271,
    },
    Resource {
        name: "GPTMR0",
        index: 282,
    },
    Resource {
        name: "UART11",
        index: 301,
    },
    Resource {
        name: "MOT2",
        index: 333,
    },
    Resource {
        name: "CLK_TOP_PTP1",
        index: 124,
    },
    Resource {
        name: "UART1",
        index: 291,
    },
    Resource {
        name: "CLK_TOP_CAM0",
        index: 119,
    },
    Resource {
        name: "AXI_BUS",
        index: 257,
    },
    Resource {
        name: "I2C3",
        index: 309,
    },
    Resource {
        name: "NTMR0",
        index: 342,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 105,
    },
    Resource {
        name: "CLK_TOP_I2S3",
        index: 199,
    },
    Resource {
        name: "CLK_SRC_PLL0CLK0",
        index: 34,
    },
    Resource {
        name: "CLK_TOP_PTPC",
        index: 111,
    },
    Resource {
        name: "CLK_TOP_UART15",
        index: 98,
    },
    Resource {
        name: "HDMA",
        index: 273,
    },
    Resource {
        name: "RST_VIS",
        index: 27,
    },
    Resource {
        name: "I2C0",
        index: 306,
    },
    Resource {
        name: "CLK_TOP_ENET0",
        index: 121,
    },
    Resource {
        name: "WDG1",
        index: 279,
    },
    Resource {
        name: "CLK_TOP_SDXC0",
        index: 129,
    },
    Resource {
        name: "POW_VIS",
        index: 22,
    },
    Resource {
        name: "CPU0_CORE",
        index: 0,
    },
    Resource {
        name: "CLK_TOP_GPTMR3",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_AUD2",
        index: 117,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 99,
    },
    Resource {
        name: "WDG0",
        index: 278,
    },
    Resource {
        name: "CLK_TOP_UART6",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_UART10",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 104,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 110,
    },
    Resource {
        name: "MCHTMR0",
        index: 264,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 125,
    },
    Resource {
        name: "MBX0",
        index: 276,
    },
    Resource {
        name: "LMM0",
        index: 262,
    },
    Resource {
        name: "CLK_TOP_UART14",
        index: 97,
    },
    Resource {
        name: "SDP",
        index: 270,
    },
    Resource {
        name: "UART3",
        index: 293,
    },
    Resource {
        name: "UART13",
        index: 303,
    },
    Resource {
        name: "DAO",
        index: 329,
    },
    Resource {
        name: "CLK_TOP_CONN",
        index: 69,
    },
    Resource {
        name: "GPIO",
        index: 275,
    },
    Resource {
        name: "UART4",
        index: 294,
    },
    Resource {
        name: "CLK_TOP_UART1",
        index: 84,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "CPU0",
        index: 0,
    },
    Clock {
        name: "ANA2",
        index: 50,
    },
    Clock {
        name: "UART0",
        index: 19,
    },
    Clock {
        name: "UART10",
        index: 29,
    },
    Clock {
        name: "SPI2",
        index: 41,
    },
    Clock {
        name: "ANA0",
        index: 48,
    },
    Clock {
        name: "UART7",
        index: 26,
    },
    Clock {
        name: "GPTMR0",
        index: 11,
    },
    Clock {
        name: "ENET1",
        index: 58,
    },
    Clock {
        name: "I2C2",
        index: 37,
    },
    Clock {
        name: "SPI3",
        index: 42,
    },
    Clock {
        name: "PTP0",
        index: 59,
    },
    Clock {
        name: "SDXC0",
        index: 65,
    },
    Clock {
        name: "UART8",
        index: 27,
    },
    Clock {
        name: "UART1",
        index: 20,
    },
    Clock {
        name: "GPTMR3",
        index: 14,
    },
    Clock {
        name: "AUD0",
        index: 51,
    },
    Clock {
        name: "REF0",
        index: 61,
    },
    Clock {
        name: "ENET0",
        index: 57,
    },
    Clock {
        name: "PTP1",
        index: 60,
    },
    Clock {
        name: "SDXC1",
        index: 66,
    },
    Clock {
        name: "GPTMR4",
        index: 15,
    },
    Clock {
        name: "UART2",
        index: 21,
    },
    Clock {
        name: "VIS",
        index: 6,
    },
    Clock {
        name: "SPI1",
        index: 40,
    },
    Clock {
        name: "LCDC",
        index: 54,
    },
    Clock {
        name: "CAM0",
        index: 55,
    },
    Clock {
        name: "NTMR1",
        index: 64,
    },
    Clock {
        name: "GPTMR5",
        index: 16,
    },
    Clock {
        name: "UART14",
        index: 33,
    },
    Clock {
        name: "UART15",
        index: 34,
    },
    Clock {
        name: "UART5",
        index: 24,
    },
    Clock {
        name: "CAN0",
        index: 43,
    },
    Clock {
        name: "SPI0",
        index: 39,
    },
    Clock {
        name: "XPI1",
        index: 10,
    },
    Clock {
        name: "MCHTMR",
        index: 3,
    },
    Clock {
        name: "ANA1",
        index: 49,
    },
    Clock {
        name: "REF1",
        index: 62,
    },
    Clock {
        name: "UART3",
        index: 22,
    },
    Clock {
        name: "FEMC",
        index: 8,
    },
    Clock {
        name: "CAN1",
        index: 44,
    },
    Clock {
        name: "AXI",
        index: 4,
    },
    Clock {
        name: "GPTMR7",
        index: 18,
    },
    Clock {
        name: "UART13",
        index: 32,
    },
    Clock {
        name: "CONN",
        index: 5,
    },
    Clock {
        name: "UART11",
        index: 30,
    },
    Clock {
        name: "I2C3",
        index: 38,
    },
    Clock {
        name: "AHB",
        index: 7,
    },
    Clock {
        name: "UART12",
        index: 31,
    },
    Clock {
        name: "UART4",
        index: 23,
    },
    Clock {
        name: "PTPC",
        index: 47,
    },
    Clock {
        name: "NTMR0",
        index: 63,
    },
    Clock {
        name: "GPTMR1",
        index: 12,
    },
    Clock {
        name: "MCHTMR0",
        index: 1,
    },
    Clock {
        name: "AUD2",
        index: 53,
    },
    Clock {
        name: "CPU1",
        index: 2,
    },
    Clock {
        name: "CAM1",
        index: 56,
    },
    Clock {
        name: "GPTMR6",
        index: 17,
    },
    Clock {
        name: "CAN2",
        index: 45,
    },
    Clock {
        name: "CAN3",
        index: 46,
    },
    Clock {
        name: "I2C1",
        index: 36,
    },
    Clock {
        name: "UART6",
        index: 25,
    },
    Clock {
        name: "I2C0",
        index: 35,
    },
    Clock {
        name: "UART9",
        index: 28,
    },
    Clock {
        name: "AUD1",
        index: 52,
    },
    Clock {
        name: "XPI0",
        index: 9,
    },
    Clock {
        name: "GPTMR2",
        index: 13,
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
];

#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
