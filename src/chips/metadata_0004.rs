
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
        name: "GPIO0_V",
        number: 7,
    },
    Interrupt {
        name: "GPIO0_W",
        number: 8,
    },
    Interrupt {
        name: "GPIO0_X",
        number: 9,
    },
    Interrupt {
        name: "GPIO0_Y",
        number: 10,
    },
    Interrupt {
        name: "GPTMR0",
        number: 11,
    },
    Interrupt {
        name: "GPTMR1",
        number: 12,
    },
    Interrupt {
        name: "GPTMR2",
        number: 13,
    },
    Interrupt {
        name: "GPTMR3",
        number: 14,
    },
    Interrupt {
        name: "OWR0",
        number: 15,
    },
    Interrupt {
        name: "OWR1",
        number: 16,
    },
    Interrupt {
        name: "EUI0",
        number: 17,
    },
    Interrupt {
        name: "EUI1",
        number: 18,
    },
    Interrupt {
        name: "UART0",
        number: 19,
    },
    Interrupt {
        name: "UART1",
        number: 20,
    },
    Interrupt {
        name: "UART2",
        number: 21,
    },
    Interrupt {
        name: "UART3",
        number: 22,
    },
    Interrupt {
        name: "UART4",
        number: 23,
    },
    Interrupt {
        name: "UART5",
        number: 24,
    },
    Interrupt {
        name: "UART6",
        number: 25,
    },
    Interrupt {
        name: "UART7",
        number: 26,
    },
    Interrupt {
        name: "I2C0",
        number: 27,
    },
    Interrupt {
        name: "I2C1",
        number: 28,
    },
    Interrupt {
        name: "I2C2",
        number: 29,
    },
    Interrupt {
        name: "I2C3",
        number: 30,
    },
    Interrupt {
        name: "SPI0",
        number: 31,
    },
    Interrupt {
        name: "SPI1",
        number: 32,
    },
    Interrupt {
        name: "SPI2",
        number: 33,
    },
    Interrupt {
        name: "SPI3",
        number: 34,
    },
    Interrupt {
        name: "TSNS",
        number: 35,
    },
    Interrupt {
        name: "MBX0A",
        number: 36,
    },
    Interrupt {
        name: "MBX0B",
        number: 37,
    },
    Interrupt {
        name: "EWDG0",
        number: 38,
    },
    Interrupt {
        name: "EWDG1",
        number: 39,
    },
    Interrupt {
        name: "HDMA",
        number: 40,
    },
    Interrupt {
        name: "LOBS",
        number: 41,
    },
    Interrupt {
        name: "ADC0",
        number: 42,
    },
    Interrupt {
        name: "ADC1",
        number: 43,
    },
    Interrupt {
        name: "ACMP0_0",
        number: 44,
    },
    Interrupt {
        name: "ACMP0_1",
        number: 45,
    },
    Interrupt {
        name: "MCAN0",
        number: 46,
    },
    Interrupt {
        name: "MCAN1",
        number: 47,
    },
    Interrupt {
        name: "MCAN2",
        number: 48,
    },
    Interrupt {
        name: "MCAN3",
        number: 49,
    },
    Interrupt {
        name: "PTPC",
        number: 50,
    },
    Interrupt {
        name: "QEI0",
        number: 51,
    },
    Interrupt {
        name: "QEI1",
        number: 52,
    },
    Interrupt {
        name: "PWM0",
        number: 53,
    },
    Interrupt {
        name: "PWM1",
        number: 54,
    },
    Interrupt {
        name: "SDM0",
        number: 55,
    },
    Interrupt {
        name: "TRGM_0",
        number: 56,
    },
    Interrupt {
        name: "TRGM_1",
        number: 57,
    },
    Interrupt {
        name: "ENET0",
        number: 58,
    },
    Interrupt {
        name: "NTMR0",
        number: 59,
    },
    Interrupt {
        name: "USB0",
        number: 60,
    },
    Interrupt {
        name: "ESC",
        number: 61,
    },
    Interrupt {
        name: "ESC_SYNC0",
        number: 62,
    },
    Interrupt {
        name: "ESC_SYNC1",
        number: 63,
    },
    Interrupt {
        name: "ESC_RESET",
        number: 64,
    },
    Interrupt {
        name: "XPI0",
        number: 65,
    },
    Interrupt {
        name: "PPI",
        number: 66,
    },
    Interrupt {
        name: "XDMA",
        number: 67,
    },
    Interrupt {
        name: "PGPIO",
        number: 68,
    },
    Interrupt {
        name: "PEWDG",
        number: 69,
    },
    Interrupt {
        name: "PTMR",
        number: 70,
    },
    Interrupt {
        name: "PUART",
        number: 71,
    },
    Interrupt {
        name: "FUSE",
        number: 72,
    },
    Interrupt {
        name: "DGO_PAD_WAKEUP",
        number: 73,
    },
    Interrupt {
        name: "DGO_CNT_WAKEUP",
        number: 74,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 75,
    },
    Interrupt {
        name: "SYSCTL",
        number: 76,
    },
    Interrupt {
        name: "CPU0",
        number: 77,
    },
    Interrupt {
        name: "DEBUG0",
        number: 78,
    },
    Interrupt {
        name: "DEBUG1",
        number: 79,
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
        name: "POW_CPU0",
        index: 21,
    },
    Resource {
        name: "RST_SOC",
        index: 22,
    },
    Resource {
        name: "RST_CPU0",
        index: 23,
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
        name: "CLK_SRC_CLK1_PLL0",
        index: 35,
    },
    Resource {
        name: "CLK_SRC_CLK2_PLL0",
        index: 36,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 37,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL1",
        index: 38,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL1",
        index: 39,
    },
    Resource {
        name: "CLK_SRC_CLK2_PLL1",
        index: 40,
    },
    Resource {
        name: "CLK_SRC_CLK3_PLL1",
        index: 41,
    },
    Resource {
        name: "CLK_SRC_PLL0_REF",
        index: 42,
    },
    Resource {
        name: "CLK_SRC_PLL1_REF",
        index: 43,
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
        name: "CLK_TOP_CAN0",
        index: 66,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 68,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 69,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 80,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 84,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_ANA3",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 99,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 101,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_DAC0",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_DAC1",
        index: 104,
    },
    Resource {
        name: "AHB0",
        index: 256,
    },
    Resource {
        name: "LMM0",
        index: 257,
    },
    Resource {
        name: "MCT0",
        index: 258,
    },
    Resource {
        name: "ROM0",
        index: 259,
    },
    Resource {
        name: "CAN0",
        index: 260,
    },
    Resource {
        name: "CAN1",
        index: 261,
    },
    Resource {
        name: "CAN2",
        index: 262,
    },
    Resource {
        name: "CAN3",
        index: 263,
    },
    Resource {
        name: "PTPC",
        index: 264,
    },
    Resource {
        name: "TMR0",
        index: 269,
    },
    Resource {
        name: "TMR1",
        index: 270,
    },
    Resource {
        name: "TMR2",
        index: 271,
    },
    Resource {
        name: "TMR3",
        index: 272,
    },
    Resource {
        name: "I2C0",
        index: 273,
    },
    Resource {
        name: "I2C1",
        index: 274,
    },
    Resource {
        name: "I2C2",
        index: 275,
    },
    Resource {
        name: "I2C3",
        index: 276,
    },
    Resource {
        name: "SPI0",
        index: 277,
    },
    Resource {
        name: "SPI1",
        index: 278,
    },
    Resource {
        name: "SPI2",
        index: 279,
    },
    Resource {
        name: "SPI3",
        index: 280,
    },
    Resource {
        name: "URT0",
        index: 281,
    },
    Resource {
        name: "URT1",
        index: 282,
    },
    Resource {
        name: "URT2",
        index: 283,
    },
    Resource {
        name: "URT3",
        index: 284,
    },
    Resource {
        name: "URT4",
        index: 285,
    },
    Resource {
        name: "URT5",
        index: 286,
    },
    Resource {
        name: "URT6",
        index: 287,
    },
    Resource {
        name: "URT7",
        index: 288,
    },
    Resource {
        name: "WDG0",
        index: 289,
    },
    Resource {
        name: "WDG1",
        index: 290,
    },
    Resource {
        name: "MBX0",
        index: 291,
    },
    Resource {
        name: "TSNS",
        index: 292,
    },
    Resource {
        name: "CRC0",
        index: 293,
    },
    Resource {
        name: "ADC0",
        index: 294,
    },
    Resource {
        name: "ADC1",
        index: 295,
    },
    Resource {
        name: "DAC0",
        index: 296,
    },
    Resource {
        name: "DAC1",
        index: 297,
    },
    Resource {
        name: "ACMP",
        index: 298,
    },
    Resource {
        name: "OPA0",
        index: 299,
    },
    Resource {
        name: "OPA1",
        index: 300,
    },
    Resource {
        name: "MOT0",
        index: 301,
    },
    Resource {
        name: "RNG0",
        index: 302,
    },
    Resource {
        name: "SDP0",
        index: 303,
    },
    Resource {
        name: "KMAN",
        index: 304,
    },
    Resource {
        name: "GPIO",
        index: 305,
    },
    Resource {
        name: "HDMA",
        index: 306,
    },
    Resource {
        name: "XPI0",
        index: 307,
    },
    Resource {
        name: "USB0",
        index: 308,
    },
    Resource {
        name: "REF0",
        index: 309,
    },
    Resource {
        name: "REF1",
        index: 310,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "MCT0",
        index: 0,
    },
    Clock {
        name: "CAN0",
        index: 1,
    },
    Clock {
        name: "CAN1",
        index: 2,
    },
    Clock {
        name: "CAN2",
        index: 3,
    },
    Clock {
        name: "CAN3",
        index: 4,
    },
    Clock {
        name: "TMR0",
        index: 9,
    },
    Clock {
        name: "TMR1",
        index: 10,
    },
    Clock {
        name: "TMR2",
        index: 11,
    },
    Clock {
        name: "TMR3",
        index: 12,
    },
    Clock {
        name: "I2C0",
        index: 13,
    },
    Clock {
        name: "I2C1",
        index: 14,
    },
    Clock {
        name: "I2C2",
        index: 15,
    },
    Clock {
        name: "I2C3",
        index: 16,
    },
    Clock {
        name: "SPI0",
        index: 17,
    },
    Clock {
        name: "SPI1",
        index: 18,
    },
    Clock {
        name: "SPI2",
        index: 19,
    },
    Clock {
        name: "SPI3",
        index: 20,
    },
    Clock {
        name: "URT0",
        index: 21,
    },
    Clock {
        name: "URT1",
        index: 22,
    },
    Clock {
        name: "URT2",
        index: 23,
    },
    Clock {
        name: "URT3",
        index: 24,
    },
    Clock {
        name: "URT4",
        index: 25,
    },
    Clock {
        name: "URT5",
        index: 26,
    },
    Clock {
        name: "URT6",
        index: 27,
    },
    Clock {
        name: "URT7",
        index: 28,
    },
    Clock {
        name: "XPI0",
        index: 29,
    },
    Clock {
        name: "ANA0",
        index: 30,
    },
    Clock {
        name: "ANA1",
        index: 31,
    },
    Clock {
        name: "ANA2",
        index: 32,
    },
    Clock {
        name: "ANA3",
        index: 33,
    },
    Clock {
        name: "REF0",
        index: 34,
    },
    Clock {
        name: "REF1",
        index: 35,
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
        name: "PF16",
        index: 176,
    },
    IoPin {
        name: "PF17",
        index: 177,
    },
    IoPin {
        name: "PF18",
        index: 178,
    },
    IoPin {
        name: "PF19",
        index: 179,
    },
    IoPin {
        name: "PF20",
        index: 180,
    },
    IoPin {
        name: "PF21",
        index: 181,
    },
    IoPin {
        name: "PF22",
        index: 182,
    },
    IoPin {
        name: "PF23",
        index: 183,
    },
    IoPin {
        name: "PF24",
        index: 184,
    },
    IoPin {
        name: "PF25",
        index: 185,
    },
    IoPin {
        name: "PF26",
        index: 186,
    },
    IoPin {
        name: "PF27",
        index: 187,
    },
    IoPin {
        name: "PF28",
        index: 188,
    },
    IoPin {
        name: "PF29",
        index: 189,
    },
    IoPin {
        name: "PF30",
        index: 190,
    },
    IoPin {
        name: "PF31",
        index: 191,
    },
    IoPin {
        name: "PV00",
        index: 352,
    },
    IoPin {
        name: "PV01",
        index: 353,
    },
    IoPin {
        name: "PV02",
        index: 354,
    },
    IoPin {
        name: "PV03",
        index: 355,
    },
    IoPin {
        name: "PV04",
        index: 356,
    },
    IoPin {
        name: "PV05",
        index: 357,
    },
    IoPin {
        name: "PV06",
        index: 358,
    },
    IoPin {
        name: "PV07",
        index: 359,
    },
    IoPin {
        name: "PV08",
        index: 360,
    },
    IoPin {
        name: "PV09",
        index: 361,
    },
    IoPin {
        name: "PV10",
        index: 362,
    },
    IoPin {
        name: "PV11",
        index: 363,
    },
    IoPin {
        name: "PV12",
        index: 364,
    },
    IoPin {
        name: "PV15",
        index: 367,
    },
    IoPin {
        name: "PW00",
        index: 384,
    },
    IoPin {
        name: "PW01",
        index: 385,
    },
    IoPin {
        name: "PW02",
        index: 386,
    },
    IoPin {
        name: "PW03",
        index: 387,
    },
    IoPin {
        name: "PW04",
        index: 388,
    },
    IoPin {
        name: "PW05",
        index: 389,
    },
    IoPin {
        name: "PW06",
        index: 390,
    },
    IoPin {
        name: "PW07",
        index: 391,
    },
    IoPin {
        name: "PW08",
        index: 392,
    },
    IoPin {
        name: "PW09",
        index: 393,
    },
    IoPin {
        name: "PW10",
        index: 394,
    },
    IoPin {
        name: "PW11",
        index: 395,
    },
    IoPin {
        name: "PW12",
        index: 396,
    },
    IoPin {
        name: "PW15",
        index: 399,
    },
    IoPin {
        name: "PW16",
        index: 400,
    },
    IoPin {
        name: "PW17",
        index: 401,
    },
    IoPin {
        name: "PW20",
        index: 404,
    },
    IoPin {
        name: "PW21",
        index: 405,
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
];
pub(crate) static TRGMMUX: &[TrgmMux] = &[];
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
