
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
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 4,
            resource_clock_top: Some(65),
            resource: 260,
            clock_node: Some(0),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSCTL",
        address: 0xf4000000,
        registers: Some(PeripheralRegisters {
            kind: "sysctl",
            version: "v62",
            block: "SYSCTL",
            ir: &sysctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SYSCTL",
            dmamux: Some("DMAMUX"),
            request: Some(87),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SYSCTL",
        }],
    },
    Peripheral {
        name: "XPI0",
        address: 0xf3040000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 24,
            resource_clock_top: Some(67),
            resource: 312,
            clock_node: Some(2),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XPI0",
            dmamux: Some("DMAMUX"),
            request: Some(70),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI0",
        }],
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
        name: "GPIO0_X",
        number: 5,
    },
    Interrupt {
        name: "GPIO0_Y",
        number: 6,
    },
    Interrupt {
        name: "GPIO0_Z",
        number: 7,
    },
    Interrupt {
        name: "GPIO1_A",
        number: 8,
    },
    Interrupt {
        name: "GPIO1_B",
        number: 9,
    },
    Interrupt {
        name: "GPIO1_C",
        number: 10,
    },
    Interrupt {
        name: "GPIO1_D",
        number: 11,
    },
    Interrupt {
        name: "GPIO1_X",
        number: 12,
    },
    Interrupt {
        name: "GPIO1_Y",
        number: 13,
    },
    Interrupt {
        name: "GPIO1_Z",
        number: 14,
    },
    Interrupt {
        name: "ADC0",
        number: 15,
    },
    Interrupt {
        name: "ADC1",
        number: 16,
    },
    Interrupt {
        name: "ADC2",
        number: 17,
    },
    Interrupt {
        name: "SDFM",
        number: 18,
    },
    Interrupt {
        name: "DAC0",
        number: 19,
    },
    Interrupt {
        name: "DAC1",
        number: 20,
    },
    Interrupt {
        name: "ACMP_0",
        number: 21,
    },
    Interrupt {
        name: "ACMP_1",
        number: 22,
    },
    Interrupt {
        name: "ACMP_2",
        number: 23,
    },
    Interrupt {
        name: "ACMP_3",
        number: 24,
    },
    Interrupt {
        name: "SPI0",
        number: 25,
    },
    Interrupt {
        name: "SPI1",
        number: 26,
    },
    Interrupt {
        name: "SPI2",
        number: 27,
    },
    Interrupt {
        name: "SPI3",
        number: 28,
    },
    Interrupt {
        name: "UART0",
        number: 29,
    },
    Interrupt {
        name: "UART1",
        number: 30,
    },
    Interrupt {
        name: "UART2",
        number: 31,
    },
    Interrupt {
        name: "UART3",
        number: 32,
    },
    Interrupt {
        name: "UART4",
        number: 33,
    },
    Interrupt {
        name: "UART5",
        number: 34,
    },
    Interrupt {
        name: "UART6",
        number: 35,
    },
    Interrupt {
        name: "UART7",
        number: 36,
    },
    Interrupt {
        name: "MCAN0",
        number: 37,
    },
    Interrupt {
        name: "MCAN1",
        number: 38,
    },
    Interrupt {
        name: "MCAN2",
        number: 39,
    },
    Interrupt {
        name: "MCAN3",
        number: 40,
    },
    Interrupt {
        name: "PTPC",
        number: 41,
    },
    Interrupt {
        name: "WDG0",
        number: 42,
    },
    Interrupt {
        name: "WDG1",
        number: 43,
    },
    Interrupt {
        name: "TSNS",
        number: 44,
    },
    Interrupt {
        name: "MBX0A",
        number: 45,
    },
    Interrupt {
        name: "MBX0B",
        number: 46,
    },
    Interrupt {
        name: "MBX1A",
        number: 47,
    },
    Interrupt {
        name: "MBX1B",
        number: 48,
    },
    Interrupt {
        name: "GPTMR0",
        number: 49,
    },
    Interrupt {
        name: "GPTMR1",
        number: 50,
    },
    Interrupt {
        name: "GPTMR2",
        number: 51,
    },
    Interrupt {
        name: "GPTMR3",
        number: 52,
    },
    Interrupt {
        name: "I2C0",
        number: 53,
    },
    Interrupt {
        name: "I2C1",
        number: 54,
    },
    Interrupt {
        name: "I2C2",
        number: 55,
    },
    Interrupt {
        name: "I2C3",
        number: 56,
    },
    Interrupt {
        name: "PWM0",
        number: 57,
    },
    Interrupt {
        name: "HALL0",
        number: 58,
    },
    Interrupt {
        name: "QEI0",
        number: 59,
    },
    Interrupt {
        name: "PWM1",
        number: 60,
    },
    Interrupt {
        name: "HALL1",
        number: 61,
    },
    Interrupt {
        name: "QEI1",
        number: 62,
    },
    Interrupt {
        name: "PWM2",
        number: 63,
    },
    Interrupt {
        name: "HALL2",
        number: 64,
    },
    Interrupt {
        name: "QEI2",
        number: 65,
    },
    Interrupt {
        name: "PWM3",
        number: 66,
    },
    Interrupt {
        name: "HALL3",
        number: 67,
    },
    Interrupt {
        name: "QEI3",
        number: 68,
    },
    Interrupt {
        name: "SDP",
        number: 69,
    },
    Interrupt {
        name: "XPI0",
        number: 70,
    },
    Interrupt {
        name: "XDMA",
        number: 71,
    },
    Interrupt {
        name: "HDMA",
        number: 72,
    },
    Interrupt {
        name: "RNG",
        number: 73,
    },
    Interrupt {
        name: "USB0",
        number: 74,
    },
    Interrupt {
        name: "PSEC",
        number: 75,
    },
    Interrupt {
        name: "PGPIO",
        number: 76,
    },
    Interrupt {
        name: "PWDG",
        number: 77,
    },
    Interrupt {
        name: "PTMR",
        number: 78,
    },
    Interrupt {
        name: "PUART",
        number: 79,
    },
    Interrupt {
        name: "FUSE",
        number: 80,
    },
    Interrupt {
        name: "SECMON",
        number: 81,
    },
    Interrupt {
        name: "RTC",
        number: 82,
    },
    Interrupt {
        name: "BUTN",
        number: 83,
    },
    Interrupt {
        name: "BGPIO",
        number: 84,
    },
    Interrupt {
        name: "BVIO",
        number: 85,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 86,
    },
    Interrupt {
        name: "SYSCTL",
        number: 87,
    },
    Interrupt {
        name: "DEBUG_0",
        number: 88,
    },
    Interrupt {
        name: "DEBUG_1",
        number: 89,
    },
    Interrupt {
        name: "LIN0",
        number: 90,
    },
    Interrupt {
        name: "LIN1",
        number: 91,
    },
    Interrupt {
        name: "LIN2",
        number: 92,
    },
    Interrupt {
        name: "LIN3",
        number: 93,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "HDMA_MUX0",
        dma: "HDMA",
        channel: 0,
    },
    DmaChannel {
        name: "HDMA_MUX1",
        dma: "HDMA",
        channel: 1,
    },
    DmaChannel {
        name: "HDMA_MUX2",
        dma: "HDMA",
        channel: 2,
    },
    DmaChannel {
        name: "HDMA_MUX3",
        dma: "HDMA",
        channel: 3,
    },
    DmaChannel {
        name: "HDMA_MUX4",
        dma: "HDMA",
        channel: 4,
    },
    DmaChannel {
        name: "HDMA_MUX5",
        dma: "HDMA",
        channel: 5,
    },
    DmaChannel {
        name: "HDMA_MUX6",
        dma: "HDMA",
        channel: 6,
    },
    DmaChannel {
        name: "HDMA_MUX7",
        dma: "HDMA",
        channel: 7,
    },
    DmaChannel {
        name: "XDMA_MUX0",
        dma: "XDMA",
        channel: 8,
    },
    DmaChannel {
        name: "XDMA_MUX1",
        dma: "XDMA",
        channel: 9,
    },
    DmaChannel {
        name: "XDMA_MUX2",
        dma: "XDMA",
        channel: 10,
    },
    DmaChannel {
        name: "XDMA_MUX3",
        dma: "XDMA",
        channel: 11,
    },
    DmaChannel {
        name: "XDMA_MUX4",
        dma: "XDMA",
        channel: 12,
    },
    DmaChannel {
        name: "XDMA_MUX5",
        dma: "XDMA",
        channel: 13,
    },
    DmaChannel {
        name: "XDMA_MUX6",
        dma: "XDMA",
        channel: 14,
    },
    DmaChannel {
        name: "XDMA_MUX7",
        dma: "XDMA",
        channel: 15,
    },
];
pub(crate) static RESOURCES: &[Resource] = &[
    Resource {
        name: "CLK_SRC_PLL0",
        index: 33,
    },
    Resource {
        name: "CPX1",
        index: 9,
    },
    Resource {
        name: "MSYN",
        index: 311,
    },
    Resource {
        name: "ACMP",
        index: 279,
    },
    Resource {
        name: "CLK_TOP_DAC0",
        index: 131,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 76,
    },
    Resource {
        name: "LMM0",
        index: 259,
    },
    Resource {
        name: "AHBP",
        index: 256,
    },
    Resource {
        name: "URT4",
        index: 289,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 69,
    },
    Resource {
        name: "CPU0",
        index: 0,
    },
    Resource {
        name: "I2C3",
        index: 268,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_LIN0",
        index: 100,
    },
    Resource {
        name: "CAN2",
        index: 300,
    },
    Resource {
        name: "KMAN",
        index: 315,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL1",
        index: 38,
    },
    Resource {
        name: "POW_CPU0",
        index: 21,
    },
    Resource {
        name: "ROM0",
        index: 263,
    },
    Resource {
        name: "CRC0",
        index: 306,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 68,
    },
    Resource {
        name: "I2C0",
        index: 265,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL2",
        index: 41,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_PTPC",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_DAC1",
        index: 132,
    },
    Resource {
        name: "TMR3",
        index: 272,
    },
    Resource {
        name: "URT3",
        index: 288,
    },
    Resource {
        name: "CAN3",
        index: 301,
    },
    Resource {
        name: "HDMA",
        index: 313,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 91,
    },
    Resource {
        name: "RST_SOC",
        index: 23,
    },
    Resource {
        name: "CLK_TOP_LIN3",
        index: 103,
    },
    Resource {
        name: "CLK_SRC_PLL0_REF",
        index: 43,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 82,
    },
    Resource {
        name: "SDM0",
        index: 284,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL1",
        index: 39,
    },
    Resource {
        name: "SPI3",
        index: 283,
    },
    Resource {
        name: "CLK_SRC_PLL1_REF",
        index: 44,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 86,
    },
    Resource {
        name: "CLK_SRC_PLL2",
        index: 40,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 129,
    },
    Resource {
        name: "MOT0",
        index: 307,
    },
    Resource {
        name: "URT5",
        index: 290,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 78,
    },
    Resource {
        name: "RAM0",
        index: 264,
    },
    Resource {
        name: "RST_CPU1",
        index: 25,
    },
    Resource {
        name: "CLK_TOP_ANA3",
        index: 96,
    },
    Resource {
        name: "TMR1",
        index: 270,
    },
    Resource {
        name: "CLK_TOP_CPU0",
        index: 64,
    },
    Resource {
        name: "GPIO",
        index: 273,
    },
    Resource {
        name: "REF1",
        index: 321,
    },
    Resource {
        name: "LIN3",
        index: 296,
    },
    Resource {
        name: "URT2",
        index: 287,
    },
    Resource {
        name: "MOT1",
        index: 308,
    },
    Resource {
        name: "CLK_SRC_CLK2_PLL0",
        index: 36,
    },
    Resource {
        name: "SPI0",
        index: 280,
    },
    Resource {
        name: "LIN2",
        index: 295,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 99,
    },
    Resource {
        name: "DAC0",
        index: 277,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 89,
    },
    Resource {
        name: "SPI2",
        index: 282,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 84,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 37,
    },
    Resource {
        name: "CAN0",
        index: 298,
    },
    Resource {
        name: "CLK_SRC_XTAL",
        index: 32,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 67,
    },
    Resource {
        name: "URT0",
        index: 285,
    },
    Resource {
        name: "SPI1",
        index: 281,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_ANA4",
        index: 97,
    },
    Resource {
        name: "AXIS",
        index: 257,
    },
    Resource {
        name: "I2C2",
        index: 267,
    },
    Resource {
        name: "MBX1",
        index: 305,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 75,
    },
    Resource {
        name: "REF0",
        index: 320,
    },
    Resource {
        name: "CLK_TOP_LIN1",
        index: 101,
    },
    Resource {
        name: "TSNS",
        index: 318,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL2",
        index: 42,
    },
    Resource {
        name: "MCT1",
        index: 262,
    },
    Resource {
        name: "POW_CPU1",
        index: 22,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 128,
    },
    Resource {
        name: "TMR0",
        index: 269,
    },
    Resource {
        name: "PTPC",
        index: 297,
    },
    Resource {
        name: "XDMA",
        index: 314,
    },
    Resource {
        name: "CLK_TOP_MCT0",
        index: 65,
    },
    Resource {
        name: "I2C1",
        index: 266,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL0",
        index: 35,
    },
    Resource {
        name: "CLK_SRC_PLL2_REF",
        index: 45,
    },
    Resource {
        name: "DAC1",
        index: 278,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 71,
    },
    Resource {
        name: "MOT2",
        index: 309,
    },
    Resource {
        name: "CPX0",
        index: 1,
    },
    Resource {
        name: "SDP0",
        index: 316,
    },
    Resource {
        name: "URT6",
        index: 291,
    },
    Resource {
        name: "LIN0",
        index: 293,
    },
    Resource {
        name: "CPU1",
        index: 8,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 79,
    },
    Resource {
        name: "XPI0",
        index: 312,
    },
    Resource {
        name: "CLK_TOP_ADC2",
        index: 130,
    },
    Resource {
        name: "RNG0",
        index: 317,
    },
    Resource {
        name: "WDG0",
        index: 302,
    },
    Resource {
        name: "LMM1",
        index: 261,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL0",
        index: 34,
    },
    Resource {
        name: "URT1",
        index: 286,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 80,
    },
    Resource {
        name: "ADC1",
        index: 275,
    },
    Resource {
        name: "LIN1",
        index: 294,
    },
    Resource {
        name: "WDG1",
        index: 303,
    },
    Resource {
        name: "RST_CPU0",
        index: 24,
    },
    Resource {
        name: "CLK_TOP_LIN2",
        index: 102,
    },
    Resource {
        name: "TMR2",
        index: 271,
    },
    Resource {
        name: "AXIC",
        index: 258,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 81,
    },
    Resource {
        name: "MCT0",
        index: 260,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 85,
    },
    Resource {
        name: "URT7",
        index: 292,
    },
    Resource {
        name: "CLK_TOP_MCT1",
        index: 66,
    },
    Resource {
        name: "CAN1",
        index: 299,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 94,
    },
    Resource {
        name: "MOT3",
        index: 310,
    },
    Resource {
        name: "USB0",
        index: 319,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 95,
    },
    Resource {
        name: "ADC0",
        index: 274,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 72,
    },
    Resource {
        name: "ADC2",
        index: 276,
    },
    Resource {
        name: "MBX0",
        index: 304,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "LIN1",
        index: 36,
    },
    Clock {
        name: "LIN3",
        index: 38,
    },
    Clock {
        name: "URT1",
        index: 8,
    },
    Clock {
        name: "TMR2",
        index: 5,
    },
    Clock {
        name: "ANA1",
        index: 29,
    },
    Clock {
        name: "I2C3",
        index: 18,
    },
    Clock {
        name: "REF0",
        index: 33,
    },
    Clock {
        name: "ANA0",
        index: 28,
    },
    Clock {
        name: "SPI3",
        index: 22,
    },
    Clock {
        name: "ANA4",
        index: 32,
    },
    Clock {
        name: "ANA2",
        index: 30,
    },
    Clock {
        name: "SPI0",
        index: 19,
    },
    Clock {
        name: "URT0",
        index: 7,
    },
    Clock {
        name: "SPI2",
        index: 21,
    },
    Clock {
        name: "CAN2",
        index: 25,
    },
    Clock {
        name: "ANA3",
        index: 31,
    },
    Clock {
        name: "REF1",
        index: 34,
    },
    Clock {
        name: "LIN0",
        index: 35,
    },
    Clock {
        name: "TMR0",
        index: 3,
    },
    Clock {
        name: "CAN1",
        index: 24,
    },
    Clock {
        name: "I2C1",
        index: 16,
    },
    Clock {
        name: "URT6",
        index: 13,
    },
    Clock {
        name: "URT2",
        index: 9,
    },
    Clock {
        name: "I2C2",
        index: 17,
    },
    Clock {
        name: "TMR1",
        index: 4,
    },
    Clock {
        name: "MCT0",
        index: 0,
    },
    Clock {
        name: "URT7",
        index: 14,
    },
    Clock {
        name: "SPI1",
        index: 20,
    },
    Clock {
        name: "I2C0",
        index: 15,
    },
    Clock {
        name: "MCT1",
        index: 1,
    },
    Clock {
        name: "CAN3",
        index: 26,
    },
    Clock {
        name: "URT5",
        index: 12,
    },
    Clock {
        name: "LIN2",
        index: 37,
    },
    Clock {
        name: "TMR3",
        index: 6,
    },
    Clock {
        name: "URT4",
        index: 11,
    },
    Clock {
        name: "XPI0",
        index: 2,
    },
    Clock {
        name: "URT3",
        index: 10,
    },
    Clock {
        name: "PTPC",
        index: 27,
    },
    Clock {
        name: "CAN0",
        index: 23,
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
];

#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/sysctl_v62.rs"]
pub mod sysctl;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
