
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
            group_bit_offset: 6,
            resource_clock_top: Some(65),
            resource: 262,
            clock_node: Some(1),
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
            version: "v6e",
            block: "SYSCTL",
            ir: &sysctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SYSCTL",
        }],
    },
    Peripheral {
        name: "PLLCTL",
        address: 0xf40c0000,
        registers: Some(PeripheralRegisters {
            kind: "pllctl",
            version: "v2",
            block: "PLLCTLV2",
            ir: &pllctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BCFG",
        address: 0xf4208000,
        registers: Some(PeripheralRegisters {
            kind: "bcfg",
            version: "v68",
            block: "BCFG",
            ir: &bcfg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BPOR",
        address: 0xf4204000,
        registers: Some(PeripheralRegisters {
            kind: "bpor",
            version: "v68",
            block: "BPOR",
            ir: &bpor::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PCFG",
        address: 0xf4104000,
        registers: Some(PeripheralRegisters {
            kind: "pcfg",
            version: "v6e",
            block: "PCFG",
            ir: &pcfg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PPOR",
        address: 0xf4100000,
        registers: Some(PeripheralRegisters {
            kind: "ppor",
            version: "v53",
            block: "PPOR",
            ir: &ppor::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "IOC",
        address: 0xf4040000,
        registers: Some(PeripheralRegisters {
            kind: "ioc",
            version: "common",
            block: "IOC",
            ir: &ioc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PIOC",
        address: 0xf4118000,
        registers: Some(PeripheralRegisters {
            kind: "ioc",
            version: "common",
            block: "IOC",
            ir: &ioc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BIOC",
        address: 0xf4210000,
        registers: Some(PeripheralRegisters {
            kind: "ioc",
            version: "common",
            block: "IOC",
            ir: &ioc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FGPIO",
        address: 0x300000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v53",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO0",
        address: 0xf00d0000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v53",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PA",
                interrupt: "GPIO0_A",
            },
            PeripheralInterrupt {
                signal: "PB",
                interrupt: "GPIO0_B",
            },
            PeripheralInterrupt {
                signal: "PC",
                interrupt: "GPIO0_C",
            },
            PeripheralInterrupt {
                signal: "PD",
                interrupt: "GPIO0_D",
            },
            PeripheralInterrupt {
                signal: "PE",
                interrupt: "GPIO0_E",
            },
            PeripheralInterrupt {
                signal: "PF",
                interrupt: "GPIO0_F",
            },
            PeripheralInterrupt {
                signal: "PV",
                interrupt: "GPIO0_V",
            },
            PeripheralInterrupt {
                signal: "PW",
                interrupt: "GPIO0_W",
            },
            PeripheralInterrupt {
                signal: "PX",
                interrupt: "GPIO0_X",
            },
            PeripheralInterrupt {
                signal: "PY",
                interrupt: "GPIO0_Y",
            },
            PeripheralInterrupt {
                signal: "PZ",
                interrupt: "GPIO0_Z",
            },
        ],
    },
    Peripheral {
        name: "GPIO1",
        address: 0xf00d4000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v53",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PA",
                interrupt: "GPIO1_A",
            },
            PeripheralInterrupt {
                signal: "PB",
                interrupt: "GPIO1_B",
            },
            PeripheralInterrupt {
                signal: "PC",
                interrupt: "GPIO1_C",
            },
            PeripheralInterrupt {
                signal: "PD",
                interrupt: "GPIO1_D",
            },
            PeripheralInterrupt {
                signal: "PE",
                interrupt: "GPIO1_E",
            },
            PeripheralInterrupt {
                signal: "PF",
                interrupt: "GPIO1_F",
            },
            PeripheralInterrupt {
                signal: "PV",
                interrupt: "GPIO1_V",
            },
            PeripheralInterrupt {
                signal: "PW",
                interrupt: "GPIO1_W",
            },
            PeripheralInterrupt {
                signal: "PX",
                interrupt: "GPIO1_X",
            },
            PeripheralInterrupt {
                signal: "PY",
                interrupt: "GPIO1_Y",
            },
            PeripheralInterrupt {
                signal: "PZ",
                interrupt: "GPIO1_Z",
            },
        ],
    },
    Peripheral {
        name: "PGPIO",
        address: 0xf411c000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v53",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PGPIO",
        }],
    },
    Peripheral {
        name: "BGPIO",
        address: 0xf4214000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v53",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "BGPIO",
        }],
    },
    Peripheral {
        name: "GPIOM",
        address: 0xf00d8000,
        registers: Some(PeripheralRegisters {
            kind: "gpiom",
            version: "v67",
            block: "GPIOM",
            ir: &gpiom::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LOBS",
        address: 0xf00dc000,
        registers: Some(PeripheralRegisters {
            kind: "lobs",
            version: "v6e",
            block: "LOBS",
            ir: &lobs::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 28,
            resource_clock_top: None,
            resource: 316,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LOBS",
        }],
    },
    Peripheral {
        name: "OTP",
        address: 0xf3158000,
        registers: Some(PeripheralRegisters {
            kind: "otp",
            version: "common",
            block: "OTP",
            ir: &otp::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "XPI0",
        address: 0xf3000000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 15,
            resource_clock_top: Some(127),
            resource: 367,
            clock_node: Some(63),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(117),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(116),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI0",
        }],
    },
    Peripheral {
        name: "XPI1",
        address: 0xf3004000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FEMC",
        address: 0xf300c000,
        registers: Some(PeripheralRegisters {
            kind: "femc",
            version: "common",
            block: "FEMC",
            ir: &femc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 16,
            resource_clock_top: Some(128),
            resource: 368,
            clock_node: Some(64),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD19",
                signal: "DQ11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "DQ23",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DM1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "DM0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "A06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "DQ16",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "DQ06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "DQ03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "A03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "SCS0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "DQ09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "A05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "DQ10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "DQ20",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "A04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "A07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "DQ14",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "DQ28",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "CKE",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DQ19",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "DM3",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "DQ22",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "DQ02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "A09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DQ08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "DQ18",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "SRDY",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "DQ31",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "A00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "A11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "DQ00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "DM2",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "SCLK0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "DQ15",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "DQS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "DQ26",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "A10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "A08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "DQ24",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "DQ21",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "DQ07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "DQ13",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "DQ05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "A02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "DQ01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "DQ27",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "DQ12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "BA0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CS0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "DQ04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "A01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "DQ25",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "SCLK1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "DQ30",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "A12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "DQ17",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "CS1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "WE",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "DQ29",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "BA1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "SCS1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CLK0",
                alt: Some(12),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FEMC",
        }],
    },
    Peripheral {
        name: "PPI",
        address: 0xf3010000,
        registers: Some(PeripheralRegisters {
            kind: "ppi",
            version: "v6e",
            block: "PPI",
            ir: &ppi::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PPI",
        }],
    },
    Peripheral {
        name: "FFA",
        address: 0xf3018000,
        registers: Some(PeripheralRegisters {
            kind: "ffa",
            version: "v6e",
            block: "FFA",
            ir: &ffa::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 20,
            resource_clock_top: None,
            resource: 372,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FFA",
        }],
    },
    Peripheral {
        name: "HDMA",
        address: 0xf00c8000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v6e",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 27,
            resource_clock_top: None,
            resource: 315,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HDMA",
        }],
    },
    Peripheral {
        name: "XDMA",
        address: 0xf3100000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v6e",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 19,
            resource_clock_top: None,
            resource: 371,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XDMA",
        }],
    },
    Peripheral {
        name: "DMAMUX",
        address: 0xf00c4000,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "common",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MBX0A",
        address: 0xf00a0000,
        registers: Some(PeripheralRegisters {
            kind: "mbx",
            version: "common",
            block: "MBX",
            ir: &mbx::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 23,
            resource_clock_top: None,
            resource: 311,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX0A",
        }],
    },
    Peripheral {
        name: "MBX0B",
        address: 0xf00a4000,
        registers: Some(PeripheralRegisters {
            kind: "mbx",
            version: "common",
            block: "MBX",
            ir: &mbx::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX0B",
        }],
    },
    Peripheral {
        name: "MBX1A",
        address: 0xf00a8000,
        registers: Some(PeripheralRegisters {
            kind: "mbx",
            version: "common",
            block: "MBX",
            ir: &mbx::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 24,
            resource_clock_top: None,
            resource: 312,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX1A",
        }],
    },
    Peripheral {
        name: "MBX1B",
        address: 0xf00ac000,
        registers: Some(PeripheralRegisters {
            kind: "mbx",
            version: "common",
            block: "MBX",
            ir: &mbx::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX1B",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 0xf00c0000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "common",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 305,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S0",
        address: 0xf0140000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 5,
            resource_clock_top: Some(141),
            resource: 325,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB03",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "TXD2",
                alt: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(64),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S0",
        }],
    },
    Peripheral {
        name: "I2S1",
        address: 0xf0144000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 6,
            resource_clock_top: Some(142),
            resource: 326,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE30",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "FCLK",
                alt: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(66),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S1",
        }],
    },
    Peripheral {
        name: "PDM",
        address: 0xf0154000,
        registers: Some(PeripheralRegisters {
            kind: "pdm",
            version: "common",
            block: "PDM",
            ir: &pdm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PDM",
        }],
    },
    Peripheral {
        name: "DAO",
        address: 0xf0210000,
        registers: Some(PeripheralRegisters {
            kind: "dao",
            version: "v68",
            block: "DAO",
            ir: &dao::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAO",
        }],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0420000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v6e",
            block: "PWMV2",
            ir: &pwm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 26,
            resource_clock_top: None,
            resource: 346,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "P1",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM0",
        }],
    },
    Peripheral {
        name: "PWM1",
        address: 0xf0424000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v6e",
            block: "PWMV2",
            ir: &pwm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 27,
            resource_clock_top: None,
            resource: 347,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "P0",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM1",
        }],
    },
    Peripheral {
        name: "PWM2",
        address: 0xf0428000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v6e",
            block: "PWMV2",
            ir: &pwm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 28,
            resource_clock_top: None,
            resource: 348,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "P1",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM2",
        }],
    },
    Peripheral {
        name: "PWM3",
        address: 0xf042c000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v6e",
            block: "PWMV2",
            ir: &pwm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 29,
            resource_clock_top: None,
            resource: 349,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "P2",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM3",
        }],
    },
    Peripheral {
        name: "TRGM0",
        address: 0xf047c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v53",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYNT",
        address: 0xf0328000,
        registers: Some(PeripheralRegisters {
            kind: "synt",
            version: "v53",
            block: "SYNT",
            ir: &synt::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QEI0",
        address: 0xf0400000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 18,
            resource_clock_top: None,
            resource: 338,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI0",
        }],
    },
    Peripheral {
        name: "QEI1",
        address: 0xf0404000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 19,
            resource_clock_top: None,
            resource: 339,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI1",
        }],
    },
    Peripheral {
        name: "QEO0",
        address: 0xf0410000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 342,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QEO1",
        address: 0xf0414000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 23,
            resource_clock_top: None,
            resource: 343,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MTG0",
        address: 0xf0490000,
        registers: Some(PeripheralRegisters {
            kind: "mtg",
            version: "v6e",
            block: "MTG",
            ir: &mtg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 4,
            resource_clock_top: None,
            resource: 356,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MTG0",
        }],
    },
    Peripheral {
        name: "PLB",
        address: 0xf0460000,
        registers: Some(PeripheralRegisters {
            kind: "plb",
            version: "v6e",
            block: "PLB",
            ir: &plb::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RDC0",
        address: 0xf0440000,
        registers: Some(PeripheralRegisters {
            kind: "rdc",
            version: "v6e",
            block: "RDC",
            ir: &rdc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 30,
            resource_clock_top: None,
            resource: 350,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RDC0",
        }],
    },
    Peripheral {
        name: "SEI",
        address: 0xf0470000,
        registers: Some(PeripheralRegisters {
            kind: "sei",
            version: "v6e",
            block: "SEI",
            ir: &sei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "0",
                interrupt: "SEI_0",
            },
            PeripheralInterrupt {
                signal: "1",
                interrupt: "SEI_1",
            },
            PeripheralInterrupt {
                signal: "2",
                interrupt: "SEI_2",
            },
            PeripheralInterrupt {
                signal: "3",
                interrupt: "SEI_3",
            },
        ],
    },
    Peripheral {
        name: "VSC0",
        address: 0xf04a0000,
        registers: Some(PeripheralRegisters {
            kind: "vsc",
            version: "v6e",
            block: "VSC",
            ir: &vsc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 6,
            resource_clock_top: None,
            resource: 358,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "VSC0",
        }],
    },
    Peripheral {
        name: "CLC0",
        address: 0xf04b0000,
        registers: Some(PeripheralRegisters {
            kind: "clc",
            version: "v6e",
            block: "CLC",
            ir: &clc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 8,
            resource_clock_top: None,
            resource: 360,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "0",
                interrupt: "CLC0_0",
            },
            PeripheralInterrupt {
                signal: "1",
                interrupt: "CLC0_1",
            },
        ],
    },
    Peripheral {
        name: "GPTMR0",
        address: 0xf0000000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 9,
            resource_clock_top: Some(73),
            resource: 265,
            clock_node: Some(9),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CAPT1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(68),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR0",
        }],
    },
    Peripheral {
        name: "GPTMR1",
        address: 0xf0004000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 10,
            resource_clock_top: Some(74),
            resource: 266,
            clock_node: Some(10),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(72),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR1",
        }],
    },
    Peripheral {
        name: "GPTMR2",
        address: 0xf0008000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 11,
            resource_clock_top: Some(75),
            resource: 267,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(78),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR2",
        }],
    },
    Peripheral {
        name: "GPTMR3",
        address: 0xf000c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 12,
            resource_clock_top: Some(76),
            resource: 268,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "COMP2",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(82),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(83),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(81),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR3",
        }],
    },
    Peripheral {
        name: "NTMR0",
        address: 0xf1410000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "NTMR0",
        }],
    },
    Peripheral {
        name: "NTMR1",
        address: 0xf1414000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PTMR",
        address: 0xf4120000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "COMP3",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CAPT0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "CAPT1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "COMP1",
                alt: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTMR",
        }],
    },
    Peripheral {
        name: "WDG0",
        address: 0xf00b0000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 19,
            resource_clock_top: None,
            resource: 307,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WDG1",
        address: 0xf00b4000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 20,
            resource_clock_top: None,
            resource: 308,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWDG",
        address: 0xf4128000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0xf4244000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "common",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RTC",
        }],
    },
    Peripheral {
        name: "UART0",
        address: 0xf0040000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 1,
            resource_clock_top: Some(97),
            resource: 289,
            clock_node: Some(33),
        }),
        pins: &[
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "RTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(8),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART0",
        }],
    },
    Peripheral {
        name: "UART1",
        address: 0xf0044000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 2,
            resource_clock_top: Some(98),
            resource: 290,
            clock_node: Some(34),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(11),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART1",
        }],
    },
    Peripheral {
        name: "UART2",
        address: 0xf0048000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 3,
            resource_clock_top: Some(99),
            resource: 291,
            clock_node: Some(35),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "RTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(13),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART2",
        }],
    },
    Peripheral {
        name: "UART3",
        address: 0xf004c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 4,
            resource_clock_top: Some(100),
            resource: 292,
            clock_node: Some(36),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "DE",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(14),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART3",
        }],
    },
    Peripheral {
        name: "UART4",
        address: 0xf0050000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 5,
            resource_clock_top: Some(101),
            resource: 293,
            clock_node: Some(37),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(17),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(16),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 0xf0054000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 6,
            resource_clock_top: Some(102),
            resource: 294,
            clock_node: Some(38),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "RXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(19),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
    },
    Peripheral {
        name: "UART6",
        address: 0xf0058000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 7,
            resource_clock_top: Some(103),
            resource: 295,
            clock_node: Some(39),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(20),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART6",
        }],
    },
    Peripheral {
        name: "UART7",
        address: 0xf005c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 8,
            resource_clock_top: Some(104),
            resource: 296,
            clock_node: Some(40),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "DE",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(22),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "PUART",
        address: 0xf4124000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PUART",
        }],
    },
    Peripheral {
        name: "SPI0",
        address: 0xf0070000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: Some(89),
            resource: 281,
            clock_node: Some(25),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CS0",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI0",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0xf0074000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: Some(90),
            resource: 282,
            clock_node: Some(26),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "CS1",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(2),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 0xf0078000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 27,
            resource_clock_top: Some(91),
            resource: 283,
            clock_node: Some(27),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0xf007c000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 28,
            resource_clock_top: Some(92),
            resource: 284,
            clock_node: Some(28),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CS0",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "I2C0",
        address: 0xf0060000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 17,
            resource_clock_top: Some(81),
            resource: 273,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(24),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C0",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0xf0064000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 18,
            resource_clock_top: Some(82),
            resource: 274,
            clock_node: Some(18),
        }),
        pins: &[
            PeripheralPin {
                pin: "PY07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(25),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C1",
        }],
    },
    Peripheral {
        name: "I2C2",
        address: 0xf0068000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 19,
            resource_clock_top: Some(83),
            resource: 275,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(26),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C2",
        }],
    },
    Peripheral {
        name: "I2C3",
        address: 0xf006c000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 20,
            resource_clock_top: Some(84),
            resource: 276,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "SCL",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(27),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C3",
        }],
    },
    Peripheral {
        name: "MCAN0",
        address: 0xf0300000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "STBY",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN0",
            dmamux: Some("DMAMUX"),
            request: Some(56),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN0",
        }],
    },
    Peripheral {
        name: "MCAN1",
        address: 0xf0304000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PE05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "STBY",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN1",
            dmamux: Some("DMAMUX"),
            request: Some(57),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN1",
        }],
    },
    Peripheral {
        name: "MCAN2",
        address: 0xf0308000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN2",
            dmamux: Some("DMAMUX"),
            request: Some(58),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN2",
        }],
    },
    Peripheral {
        name: "MCAN3",
        address: 0xf030c000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN3",
            dmamux: Some("DMAMUX"),
            request: Some(59),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN3",
        }],
    },
    Peripheral {
        name: "PTPC",
        address: 0xf037c000,
        registers: Some(PeripheralRegisters {
            kind: "ptpc",
            version: "common",
            block: "PTPC",
            ir: &ptpc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 337,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTPC",
        }],
    },
    Peripheral {
        name: "USB0",
        address: 0xf1120000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v53",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 374,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF18",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "OC",
                alt: Some(24),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB0",
        }],
    },
    Peripheral {
        name: "ENET0",
        address: 0xf1400000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v68",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET0",
        }],
    },
    Peripheral {
        name: "ENET1",
        address: 0xf1404000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v68",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ESC",
        address: 0xf1408000,
        registers: Some(PeripheralRegisters {
            kind: "esc",
            version: "v6e",
            block: "ESC",
            ir: &esc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SYNC0",
                dmamux: Some("DMAMUX"),
                request: Some(118),
            },
            PeripheralDmaChannel {
                signal: "SYNC1",
                dmamux: Some("DMAMUX"),
                request: Some(119),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "ESC",
            },
            PeripheralInterrupt {
                signal: "SYNC0",
                interrupt: "ESC_SYNC0",
            },
            PeripheralInterrupt {
                signal: "SYNC1",
                interrupt: "ESC_SYNC1",
            },
            PeripheralInterrupt {
                signal: "RESET",
                interrupt: "ESC_RESET",
            },
        ],
    },
    Peripheral {
        name: "ADC0",
        address: 0xf0100000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v6e",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 29,
            resource_clock_top: Some(137),
            resource: 317,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF06",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "IN02",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "IN06",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "IN11",
                alt: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC0",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0xf0104000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v6e",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 30,
            resource_clock_top: Some(138),
            resource: 318,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF06",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "IN02",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "IN06",
                alt: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 0xf0108000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v6e",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 31,
            resource_clock_top: Some(139),
            resource: 319,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF25",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "IN02",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "IN06",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "IN13",
                alt: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
    },
    Peripheral {
        name: "ACMP0",
        address: 0xf0130000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "v6e",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF07",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(109),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "ACMP0_0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "ACMP0_1",
            },
        ],
    },
    Peripheral {
        name: "ACMP1",
        address: 0xf0134000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "v6e",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF01",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP1_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP1_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "CMP1_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "CMP1_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(110),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "ACMP1_0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "ACMP1_1",
            },
        ],
    },
    Peripheral {
        name: "ACMP2",
        address: 0xf0138000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "v6e",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF11",
                signal: "CMP2_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP2_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP2_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CMP2_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP2_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CMP2_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "CMP2_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CMP2_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP2_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP2_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "CMP2_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CMP2_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP2_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CMP2_INP2",
                alt: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(112),
            },
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(113),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "ACMP2_0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "ACMP2_1",
            },
        ],
    },
    Peripheral {
        name: "ACMP3",
        address: 0xf013c000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "v6e",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF07",
                signal: "CMP3_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "CMP3_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP3_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP3_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "CMP3_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP3_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CMP3_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP3_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CMP3_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CMP3_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP3_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CMP3_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "CMP3_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP3_INN4",
                alt: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(114),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "ACMP3_0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "ACMP3_1",
            },
        ],
    },
    Peripheral {
        name: "TSNS",
        address: 0xf0090000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 18,
            resource_clock_top: None,
            resource: 306,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSNS",
        }],
    },
    Peripheral {
        name: "SDM0",
        address: 0xf0450000,
        registers: Some(PeripheralRegisters {
            kind: "sdm",
            version: "v6e",
            block: "SDM",
            ir: &sdm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 0,
            resource_clock_top: None,
            resource: 352,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDM0",
        }],
    },
    Peripheral {
        name: "SDP",
        address: 0xf3140000,
        registers: Some(PeripheralRegisters {
            kind: "sdp",
            version: "v53",
            block: "SDP",
            ir: &sdp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 12,
            resource_clock_top: None,
            resource: 364,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDP",
        }],
    },
    Peripheral {
        name: "RNG",
        address: 0xf314c000,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "common",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
    },
    Peripheral {
        name: "KEYM",
        address: 0xf3154000,
        registers: Some(PeripheralRegisters {
            kind: "keym",
            version: "common",
            block: "KEYM",
            ir: &keym::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 366,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BKEY",
        address: 0xf4248000,
        registers: Some(PeripheralRegisters {
            kind: "bkey",
            version: "common",
            block: "BKEY",
            ir: &bkey::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PSEC",
        address: 0xf3144000,
        registers: Some(PeripheralRegisters {
            kind: "psec",
            version: "common",
            block: "PSEC",
            ir: &psec::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PSEC",
        }],
    },
    Peripheral {
        name: "PMON",
        address: 0xf3148000,
        registers: Some(PeripheralRegisters {
            kind: "pmon",
            version: "common",
            block: "PMON",
            ir: &pmon::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BSEC",
        address: 0xf4240000,
        registers: Some(PeripheralRegisters {
            kind: "bsec",
            version: "common",
            block: "BSEC",
            ir: &bsec::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BMON",
        address: 0xf424c000,
        registers: Some(PeripheralRegisters {
            kind: "bmon",
            version: "common",
            block: "BMON",
            ir: &bmon::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TAMP",
        address: 0xf4250000,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "v62",
            block: "TAMP",
            ir: &tamp::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MONO",
        address: 0xf4254000,
        registers: Some(PeripheralRegisters {
            kind: "mono",
            version: "common",
            block: "MONO",
            ir: &mono::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCAN4",
        address: 0xf0310000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PD16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "RXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN4",
            dmamux: Some("DMAMUX"),
            request: Some(60),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN4",
        }],
    },
    Peripheral {
        name: "MCAN5",
        address: 0xf0314000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PZ05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "RXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN5",
            dmamux: Some("DMAMUX"),
            request: Some(61),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN5",
        }],
    },
    Peripheral {
        name: "MCAN6",
        address: 0xf0318000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN6",
            dmamux: Some("DMAMUX"),
            request: Some(62),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN6",
        }],
    },
    Peripheral {
        name: "MCAN7",
        address: 0xf031c000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PF29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN7",
            dmamux: Some("DMAMUX"),
            request: Some(63),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN7",
        }],
    },
    Peripheral {
        name: "UART8",
        address: 0xf0180000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 9,
            resource_clock_top: Some(105),
            resource: 297,
            clock_node: Some(41),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(36),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
    },
    Peripheral {
        name: "UART9",
        address: 0xf0184000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 10,
            resource_clock_top: Some(106),
            resource: 298,
            clock_node: Some(42),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(38),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART9",
        }],
    },
    Peripheral {
        name: "UART10",
        address: 0xf0188000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 11,
            resource_clock_top: Some(107),
            resource: 299,
            clock_node: Some(43),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(40),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART10",
        }],
    },
    Peripheral {
        name: "UART11",
        address: 0xf018c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 12,
            resource_clock_top: Some(108),
            resource: 300,
            clock_node: Some(44),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(43),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART11",
        }],
    },
    Peripheral {
        name: "UART12",
        address: 0xf0190000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 13,
            resource_clock_top: Some(109),
            resource: 301,
            clock_node: Some(45),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(44),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(45),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART12",
        }],
    },
    Peripheral {
        name: "UART13",
        address: 0xf0194000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 14,
            resource_clock_top: Some(110),
            resource: 302,
            clock_node: Some(46),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(46),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART13",
        }],
    },
    Peripheral {
        name: "UART14",
        address: 0xf0198000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: Some(111),
            resource: 303,
            clock_node: Some(47),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(49),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART14",
        }],
    },
    Peripheral {
        name: "UART15",
        address: 0xf019c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: Some(112),
            resource: 304,
            clock_node: Some(48),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "RTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(50),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART15",
        }],
    },
    Peripheral {
        name: "SPI4",
        address: 0xf01b0000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: Some(93),
            resource: 285,
            clock_node: Some(29),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DAT2",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(29),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
    },
    Peripheral {
        name: "SPI5",
        address: 0xf01b4000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: Some(94),
            resource: 286,
            clock_node: Some(30),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CS0",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(30),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
    },
    Peripheral {
        name: "SPI6",
        address: 0xf01b8000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: Some(95),
            resource: 287,
            clock_node: Some(31),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "CS1",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(33),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
    },
    Peripheral {
        name: "SPI7",
        address: 0xf01bc000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 0,
            resource_clock_top: Some(96),
            resource: 288,
            clock_node: Some(32),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "CS1",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(34),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI7",
        }],
    },
    Peripheral {
        name: "I2C4",
        address: 0xf01a0000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 21,
            resource_clock_top: Some(85),
            resource: 277,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(52),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C4",
        }],
    },
    Peripheral {
        name: "I2C5",
        address: 0xf01a4000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 22,
            resource_clock_top: Some(86),
            resource: 278,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "SCL",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(53),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C5",
        }],
    },
    Peripheral {
        name: "I2C6",
        address: 0xf01a8000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 23,
            resource_clock_top: Some(87),
            resource: 279,
            clock_node: Some(23),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(54),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C6",
        }],
    },
    Peripheral {
        name: "I2C7",
        address: 0xf01ac000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 24,
            resource_clock_top: Some(88),
            resource: 280,
            clock_node: Some(24),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "SCL",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(55),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C7",
        }],
    },
    Peripheral {
        name: "QEI2",
        address: 0xf0408000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 20,
            resource_clock_top: None,
            resource: 340,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI2",
        }],
    },
    Peripheral {
        name: "QEI3",
        address: 0xf040c000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 21,
            resource_clock_top: None,
            resource: 341,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI3",
        }],
    },
    Peripheral {
        name: "QEO2",
        address: 0xf0418000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 24,
            resource_clock_top: None,
            resource: 344,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QEO3",
        address: 0xf041c000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 25,
            resource_clock_top: None,
            resource: 345,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "VSC1",
        address: 0xf04a4000,
        registers: Some(PeripheralRegisters {
            kind: "vsc",
            version: "v6e",
            block: "VSC",
            ir: &vsc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 7,
            resource_clock_top: None,
            resource: 359,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "VSC1",
        }],
    },
    Peripheral {
        name: "CLC1",
        address: 0xf04b4000,
        registers: Some(PeripheralRegisters {
            kind: "clc",
            version: "v6e",
            block: "CLC",
            ir: &clc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 9,
            resource_clock_top: None,
            resource: 361,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "0",
                interrupt: "CLC1_0",
            },
            PeripheralInterrupt {
                signal: "1",
                interrupt: "CLC1_1",
            },
        ],
    },
    Peripheral {
        name: "MTG1",
        address: 0xf0494000,
        registers: Some(PeripheralRegisters {
            kind: "mtg",
            version: "v6e",
            block: "MTG",
            ir: &mtg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 5,
            resource_clock_top: None,
            resource: 357,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MTG1",
        }],
    },
    Peripheral {
        name: "RDC1",
        address: 0xf0444000,
        registers: Some(PeripheralRegisters {
            kind: "rdc",
            version: "v6e",
            block: "RDC",
            ir: &rdc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 31,
            resource_clock_top: None,
            resource: 351,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RDC1",
        }],
    },
    Peripheral {
        name: "GPTMR4",
        address: 0xf0010000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 13,
            resource_clock_top: Some(77),
            resource: 269,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "COMP0",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(84),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR4",
        }],
    },
    Peripheral {
        name: "GPTMR5",
        address: 0xf0014000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 14,
            resource_clock_top: Some(78),
            resource: 270,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(90),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(88),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(89),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(91),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR5",
        }],
    },
    Peripheral {
        name: "GPTMR6",
        address: 0xf0018000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 15,
            resource_clock_top: Some(79),
            resource: 271,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "COMP0",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(94),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(92),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(95),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR6",
        }],
    },
    Peripheral {
        name: "GPTMR7",
        address: 0xf001c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 16,
            resource_clock_top: Some(80),
            resource: 272,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(96),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(99),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR7",
        }],
    },
    Peripheral {
        name: "WDG2",
        address: 0xf00b8000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 21,
            resource_clock_top: None,
            resource: 309,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WDG3",
        address: 0xf00bc000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 310,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SDM1",
        address: 0xf0454000,
        registers: Some(PeripheralRegisters {
            kind: "sdm",
            version: "v6e",
            block: "SDM",
            ir: &sdm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 3,
            group_bit_offset: 1,
            resource_clock_top: None,
            resource: 353,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDM1",
        }],
    },
    Peripheral {
        name: "ADC3",
        address: 0xf010c000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v6e",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 0,
            resource_clock_top: Some(140),
            resource: 320,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF26",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "IN06",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "IN02",
                alt: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
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
        name: "GPIO0_Z",
        number: 11,
    },
    Interrupt {
        name: "GPIO1_A",
        number: 12,
    },
    Interrupt {
        name: "GPIO1_B",
        number: 13,
    },
    Interrupt {
        name: "GPIO1_C",
        number: 14,
    },
    Interrupt {
        name: "GPIO1_D",
        number: 15,
    },
    Interrupt {
        name: "GPIO1_E",
        number: 16,
    },
    Interrupt {
        name: "GPIO1_F",
        number: 17,
    },
    Interrupt {
        name: "GPIO1_V",
        number: 18,
    },
    Interrupt {
        name: "GPIO1_W",
        number: 19,
    },
    Interrupt {
        name: "GPIO1_X",
        number: 20,
    },
    Interrupt {
        name: "GPIO1_Y",
        number: 21,
    },
    Interrupt {
        name: "GPIO1_Z",
        number: 22,
    },
    Interrupt {
        name: "GPTMR0",
        number: 23,
    },
    Interrupt {
        name: "GPTMR1",
        number: 24,
    },
    Interrupt {
        name: "GPTMR2",
        number: 25,
    },
    Interrupt {
        name: "GPTMR3",
        number: 26,
    },
    Interrupt {
        name: "GPTMR4",
        number: 27,
    },
    Interrupt {
        name: "GPTMR5",
        number: 28,
    },
    Interrupt {
        name: "GPTMR6",
        number: 29,
    },
    Interrupt {
        name: "GPTMR7",
        number: 30,
    },
    Interrupt {
        name: "UART0",
        number: 31,
    },
    Interrupt {
        name: "UART1",
        number: 32,
    },
    Interrupt {
        name: "UART2",
        number: 33,
    },
    Interrupt {
        name: "UART3",
        number: 34,
    },
    Interrupt {
        name: "UART4",
        number: 35,
    },
    Interrupt {
        name: "UART5",
        number: 36,
    },
    Interrupt {
        name: "UART6",
        number: 37,
    },
    Interrupt {
        name: "UART7",
        number: 38,
    },
    Interrupt {
        name: "I2C0",
        number: 39,
    },
    Interrupt {
        name: "I2C1",
        number: 40,
    },
    Interrupt {
        name: "I2C2",
        number: 41,
    },
    Interrupt {
        name: "I2C3",
        number: 42,
    },
    Interrupt {
        name: "SPI0",
        number: 43,
    },
    Interrupt {
        name: "SPI1",
        number: 44,
    },
    Interrupt {
        name: "SPI2",
        number: 45,
    },
    Interrupt {
        name: "SPI3",
        number: 46,
    },
    Interrupt {
        name: "TSNS",
        number: 47,
    },
    Interrupt {
        name: "MBX0A",
        number: 48,
    },
    Interrupt {
        name: "MBX0B",
        number: 49,
    },
    Interrupt {
        name: "MBX1A",
        number: 50,
    },
    Interrupt {
        name: "MBX1B",
        number: 51,
    },
    Interrupt {
        name: "EWDG0",
        number: 52,
    },
    Interrupt {
        name: "EWDG1",
        number: 53,
    },
    Interrupt {
        name: "EWDG2",
        number: 54,
    },
    Interrupt {
        name: "EWDG3",
        number: 55,
    },
    Interrupt {
        name: "HDMA",
        number: 56,
    },
    Interrupt {
        name: "LOBS",
        number: 57,
    },
    Interrupt {
        name: "ADC0",
        number: 58,
    },
    Interrupt {
        name: "ADC1",
        number: 59,
    },
    Interrupt {
        name: "ADC2",
        number: 60,
    },
    Interrupt {
        name: "ADC3",
        number: 61,
    },
    Interrupt {
        name: "ACMP0_0",
        number: 62,
    },
    Interrupt {
        name: "ACMP0_1",
        number: 63,
    },
    Interrupt {
        name: "ACMP1_0",
        number: 64,
    },
    Interrupt {
        name: "ACMP1_1",
        number: 65,
    },
    Interrupt {
        name: "ACMP2_0",
        number: 66,
    },
    Interrupt {
        name: "ACMP2_1",
        number: 67,
    },
    Interrupt {
        name: "ACMP3_0",
        number: 68,
    },
    Interrupt {
        name: "ACMP3_1",
        number: 69,
    },
    Interrupt {
        name: "I2S0",
        number: 70,
    },
    Interrupt {
        name: "I2S1",
        number: 71,
    },
    Interrupt {
        name: "DAO",
        number: 72,
    },
    Interrupt {
        name: "PDM",
        number: 73,
    },
    Interrupt {
        name: "UART8",
        number: 74,
    },
    Interrupt {
        name: "UART9",
        number: 75,
    },
    Interrupt {
        name: "UART10",
        number: 76,
    },
    Interrupt {
        name: "UART11",
        number: 77,
    },
    Interrupt {
        name: "UART12",
        number: 78,
    },
    Interrupt {
        name: "UART13",
        number: 79,
    },
    Interrupt {
        name: "UART14",
        number: 80,
    },
    Interrupt {
        name: "UART15",
        number: 81,
    },
    Interrupt {
        name: "I2C4",
        number: 82,
    },
    Interrupt {
        name: "I2C5",
        number: 83,
    },
    Interrupt {
        name: "I2C6",
        number: 84,
    },
    Interrupt {
        name: "I2C7",
        number: 85,
    },
    Interrupt {
        name: "SPI4",
        number: 86,
    },
    Interrupt {
        name: "SPI5",
        number: 87,
    },
    Interrupt {
        name: "SPI6",
        number: 88,
    },
    Interrupt {
        name: "SPI7",
        number: 89,
    },
    Interrupt {
        name: "MCAN0",
        number: 90,
    },
    Interrupt {
        name: "MCAN1",
        number: 91,
    },
    Interrupt {
        name: "MCAN2",
        number: 92,
    },
    Interrupt {
        name: "MCAN3",
        number: 93,
    },
    Interrupt {
        name: "MCAN4",
        number: 94,
    },
    Interrupt {
        name: "MCAN5",
        number: 95,
    },
    Interrupt {
        name: "MCAN6",
        number: 96,
    },
    Interrupt {
        name: "MCAN7",
        number: 97,
    },
    Interrupt {
        name: "PTPC",
        number: 98,
    },
    Interrupt {
        name: "QEI0",
        number: 99,
    },
    Interrupt {
        name: "QEI1",
        number: 100,
    },
    Interrupt {
        name: "QEI2",
        number: 101,
    },
    Interrupt {
        name: "QEI3",
        number: 102,
    },
    Interrupt {
        name: "PWM0",
        number: 103,
    },
    Interrupt {
        name: "PWM1",
        number: 104,
    },
    Interrupt {
        name: "PWM2",
        number: 105,
    },
    Interrupt {
        name: "PWM3",
        number: 106,
    },
    Interrupt {
        name: "RDC0",
        number: 107,
    },
    Interrupt {
        name: "RDC1",
        number: 108,
    },
    Interrupt {
        name: "SDM0",
        number: 109,
    },
    Interrupt {
        name: "SDM1",
        number: 110,
    },
    Interrupt {
        name: "SEI_0",
        number: 111,
    },
    Interrupt {
        name: "SEI_1",
        number: 112,
    },
    Interrupt {
        name: "SEI_2",
        number: 113,
    },
    Interrupt {
        name: "SEI_3",
        number: 114,
    },
    Interrupt {
        name: "MTG0",
        number: 115,
    },
    Interrupt {
        name: "MTG1",
        number: 116,
    },
    Interrupt {
        name: "VSC0",
        number: 117,
    },
    Interrupt {
        name: "VSC1",
        number: 118,
    },
    Interrupt {
        name: "CLC0_0",
        number: 119,
    },
    Interrupt {
        name: "CLC0_1",
        number: 120,
    },
    Interrupt {
        name: "CLC1_0",
        number: 121,
    },
    Interrupt {
        name: "CLC1_1",
        number: 122,
    },
    Interrupt {
        name: "TRGMUX0",
        number: 123,
    },
    Interrupt {
        name: "TRGMUX1",
        number: 124,
    },
    Interrupt {
        name: "ENET0",
        number: 125,
    },
    Interrupt {
        name: "NTMR0",
        number: 126,
    },
    Interrupt {
        name: "USB0",
        number: 127,
    },
    Interrupt {
        name: "TSW_0",
        number: 128,
    },
    Interrupt {
        name: "TSW_1",
        number: 129,
    },
    Interrupt {
        name: "TSW_2",
        number: 130,
    },
    Interrupt {
        name: "TSW_3",
        number: 131,
    },
    Interrupt {
        name: "TSW_PTP_EVT",
        number: 132,
    },
    Interrupt {
        name: "ESC",
        number: 133,
    },
    Interrupt {
        name: "ESC_SYNC0",
        number: 134,
    },
    Interrupt {
        name: "ESC_SYNC1",
        number: 135,
    },
    Interrupt {
        name: "ESC_RESET",
        number: 136,
    },
    Interrupt {
        name: "XPI0",
        number: 137,
    },
    Interrupt {
        name: "FEMC",
        number: 138,
    },
    Interrupt {
        name: "PPI",
        number: 139,
    },
    Interrupt {
        name: "XDMA",
        number: 140,
    },
    Interrupt {
        name: "FFA",
        number: 141,
    },
    Interrupt {
        name: "SDP",
        number: 142,
    },
    Interrupt {
        name: "RNG",
        number: 143,
    },
    Interrupt {
        name: "PKA",
        number: 144,
    },
    Interrupt {
        name: "PSEC",
        number: 145,
    },
    Interrupt {
        name: "PGPIO",
        number: 146,
    },
    Interrupt {
        name: "PEWDG",
        number: 147,
    },
    Interrupt {
        name: "PTMR",
        number: 148,
    },
    Interrupt {
        name: "PUART",
        number: 149,
    },
    Interrupt {
        name: "FUSE",
        number: 150,
    },
    Interrupt {
        name: "SECMON",
        number: 151,
    },
    Interrupt {
        name: "RTC",
        number: 152,
    },
    Interrupt {
        name: "PAD_WAKEUP",
        number: 153,
    },
    Interrupt {
        name: "BGPIO",
        number: 154,
    },
    Interrupt {
        name: "BVIO",
        number: 155,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 156,
    },
    Interrupt {
        name: "SYSCTL",
        number: 157,
    },
    Interrupt {
        name: "CPU0",
        number: 158,
    },
    Interrupt {
        name: "CPU1",
        number: 159,
    },
    Interrupt {
        name: "DEBUG0",
        number: 160,
    },
    Interrupt {
        name: "DEBUG1",
        number: 161,
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
    DmaChannel {
        name: "XDMA_CH0",
        dma: "XDMA",
        channel: 0,
        dmamux_channel: 32,
    },
    DmaChannel {
        name: "XDMA_CH1",
        dma: "XDMA",
        channel: 1,
        dmamux_channel: 33,
    },
    DmaChannel {
        name: "XDMA_CH2",
        dma: "XDMA",
        channel: 2,
        dmamux_channel: 34,
    },
    DmaChannel {
        name: "XDMA_CH3",
        dma: "XDMA",
        channel: 3,
        dmamux_channel: 35,
    },
    DmaChannel {
        name: "XDMA_CH4",
        dma: "XDMA",
        channel: 4,
        dmamux_channel: 36,
    },
    DmaChannel {
        name: "XDMA_CH5",
        dma: "XDMA",
        channel: 5,
        dmamux_channel: 37,
    },
    DmaChannel {
        name: "XDMA_CH6",
        dma: "XDMA",
        channel: 6,
        dmamux_channel: 38,
    },
    DmaChannel {
        name: "XDMA_CH7",
        dma: "XDMA",
        channel: 7,
        dmamux_channel: 39,
    },
    DmaChannel {
        name: "XDMA_CH8",
        dma: "XDMA",
        channel: 8,
        dmamux_channel: 40,
    },
    DmaChannel {
        name: "XDMA_CH9",
        dma: "XDMA",
        channel: 9,
        dmamux_channel: 41,
    },
    DmaChannel {
        name: "XDMA_CH10",
        dma: "XDMA",
        channel: 10,
        dmamux_channel: 42,
    },
    DmaChannel {
        name: "XDMA_CH11",
        dma: "XDMA",
        channel: 11,
        dmamux_channel: 43,
    },
    DmaChannel {
        name: "XDMA_CH12",
        dma: "XDMA",
        channel: 12,
        dmamux_channel: 44,
    },
    DmaChannel {
        name: "XDMA_CH13",
        dma: "XDMA",
        channel: 13,
        dmamux_channel: 45,
    },
    DmaChannel {
        name: "XDMA_CH14",
        dma: "XDMA",
        channel: 14,
        dmamux_channel: 46,
    },
    DmaChannel {
        name: "XDMA_CH15",
        dma: "XDMA",
        channel: 15,
        dmamux_channel: 47,
    },
    DmaChannel {
        name: "XDMA_CH16",
        dma: "XDMA",
        channel: 16,
        dmamux_channel: 48,
    },
    DmaChannel {
        name: "XDMA_CH17",
        dma: "XDMA",
        channel: 17,
        dmamux_channel: 49,
    },
    DmaChannel {
        name: "XDMA_CH18",
        dma: "XDMA",
        channel: 18,
        dmamux_channel: 50,
    },
    DmaChannel {
        name: "XDMA_CH19",
        dma: "XDMA",
        channel: 19,
        dmamux_channel: 51,
    },
    DmaChannel {
        name: "XDMA_CH20",
        dma: "XDMA",
        channel: 20,
        dmamux_channel: 52,
    },
    DmaChannel {
        name: "XDMA_CH21",
        dma: "XDMA",
        channel: 21,
        dmamux_channel: 53,
    },
    DmaChannel {
        name: "XDMA_CH22",
        dma: "XDMA",
        channel: 22,
        dmamux_channel: 54,
    },
    DmaChannel {
        name: "XDMA_CH23",
        dma: "XDMA",
        channel: 23,
        dmamux_channel: 55,
    },
    DmaChannel {
        name: "XDMA_CH24",
        dma: "XDMA",
        channel: 24,
        dmamux_channel: 56,
    },
    DmaChannel {
        name: "XDMA_CH25",
        dma: "XDMA",
        channel: 25,
        dmamux_channel: 57,
    },
    DmaChannel {
        name: "XDMA_CH26",
        dma: "XDMA",
        channel: 26,
        dmamux_channel: 58,
    },
    DmaChannel {
        name: "XDMA_CH27",
        dma: "XDMA",
        channel: 27,
        dmamux_channel: 59,
    },
    DmaChannel {
        name: "XDMA_CH28",
        dma: "XDMA",
        channel: 28,
        dmamux_channel: 60,
    },
    DmaChannel {
        name: "XDMA_CH29",
        dma: "XDMA",
        channel: 29,
        dmamux_channel: 61,
    },
    DmaChannel {
        name: "XDMA_CH30",
        dma: "XDMA",
        channel: 30,
        dmamux_channel: 62,
    },
    DmaChannel {
        name: "XDMA_CH31",
        dma: "XDMA",
        channel: 31,
        dmamux_channel: 63,
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
        name: "CPU1",
        index: 8,
    },
    Resource {
        name: "CPX1",
        index: 9,
    },
    Resource {
        name: "POW_CPU0",
        index: 21,
    },
    Resource {
        name: "POW_CPU1",
        index: 22,
    },
    Resource {
        name: "POW_OTN",
        index: 23,
    },
    Resource {
        name: "RST_SOC",
        index: 24,
    },
    Resource {
        name: "RST_CPU0",
        index: 25,
    },
    Resource {
        name: "RST_CPU1",
        index: 26,
    },
    Resource {
        name: "RST_OTN",
        index: 27,
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
        name: "CLK_SRC_PLL1",
        index: 36,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL1",
        index: 37,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL1",
        index: 38,
    },
    Resource {
        name: "CLK_SRC_CLK2_PLL1",
        index: 39,
    },
    Resource {
        name: "CLK_SRC_PLL2",
        index: 40,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL2",
        index: 41,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL2",
        index: 42,
    },
    Resource {
        name: "CLK_SRC_PLL0_REF",
        index: 43,
    },
    Resource {
        name: "CLK_SRC_PLL1_REF",
        index: 44,
    },
    Resource {
        name: "CLK_SRC_PLL2_REF",
        index: 45,
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
        name: "CLK_TOP_CPU1",
        index: 66,
    },
    Resource {
        name: "CLK_TOP_MCT1",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_AHB0",
        index: 68,
    },
    Resource {
        name: "CLK_TOP_AXIF",
        index: 69,
    },
    Resource {
        name: "CLK_TOP_AXIS",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_AXIC",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_AXIN",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_TMR4",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_TMR5",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_TMR6",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_TMR7",
        index: 80,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 84,
    },
    Resource {
        name: "CLK_TOP_I2C4",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_I2C5",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_I2C6",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_I2C7",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_SPI4",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_SPI5",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_SPI6",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_SPI7",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 99,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 101,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 104,
    },
    Resource {
        name: "CLK_TOP_URT8",
        index: 105,
    },
    Resource {
        name: "CLK_TOP_URT9",
        index: 106,
    },
    Resource {
        name: "CLK_TOP_URT10",
        index: 107,
    },
    Resource {
        name: "CLK_TOP_URT11",
        index: 108,
    },
    Resource {
        name: "CLK_TOP_URT12",
        index: 109,
    },
    Resource {
        name: "CLK_TOP_URT13",
        index: 110,
    },
    Resource {
        name: "CLK_TOP_URT14",
        index: 111,
    },
    Resource {
        name: "CLK_TOP_URT15",
        index: 112,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 113,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 114,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 115,
    },
    Resource {
        name: "CLK_TOP_ANA3",
        index: 116,
    },
    Resource {
        name: "CLK_TOP_AUD0",
        index: 117,
    },
    Resource {
        name: "CLK_TOP_AUD1",
        index: 118,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 119,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 120,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 121,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 122,
    },
    Resource {
        name: "CLK_TOP_CAN4",
        index: 123,
    },
    Resource {
        name: "CLK_TOP_CAN5",
        index: 124,
    },
    Resource {
        name: "CLK_TOP_CAN6",
        index: 125,
    },
    Resource {
        name: "CLK_TOP_CAN7",
        index: 126,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 127,
    },
    Resource {
        name: "CLK_TOP_FEMC",
        index: 128,
    },
    Resource {
        name: "CLK_TOP_ETH0",
        index: 129,
    },
    Resource {
        name: "CLK_TOP_PTP0",
        index: 130,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 131,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 132,
    },
    Resource {
        name: "CLK_TOP_NTM0",
        index: 133,
    },
    Resource {
        name: "CLK_TOP_TSW1",
        index: 134,
    },
    Resource {
        name: "CLK_TOP_TSW2",
        index: 135,
    },
    Resource {
        name: "CLK_TOP_TSW3",
        index: 136,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 137,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 138,
    },
    Resource {
        name: "CLK_TOP_ADC2",
        index: 139,
    },
    Resource {
        name: "CLK_TOP_ADC3",
        index: 140,
    },
    Resource {
        name: "CLK_TOP_I2S0",
        index: 141,
    },
    Resource {
        name: "CLK_TOP_I2S1",
        index: 142,
    },
    Resource {
        name: "AHBP",
        index: 256,
    },
    Resource {
        name: "AXIS",
        index: 257,
    },
    Resource {
        name: "AXIC",
        index: 258,
    },
    Resource {
        name: "AXIN",
        index: 259,
    },
    Resource {
        name: "ROM0",
        index: 260,
    },
    Resource {
        name: "LMM0",
        index: 261,
    },
    Resource {
        name: "MCT0",
        index: 262,
    },
    Resource {
        name: "LMM1",
        index: 263,
    },
    Resource {
        name: "MCT1",
        index: 264,
    },
    Resource {
        name: "TMR0",
        index: 265,
    },
    Resource {
        name: "TMR1",
        index: 266,
    },
    Resource {
        name: "TMR2",
        index: 267,
    },
    Resource {
        name: "TMR3",
        index: 268,
    },
    Resource {
        name: "TMR4",
        index: 269,
    },
    Resource {
        name: "TMR5",
        index: 270,
    },
    Resource {
        name: "TMR6",
        index: 271,
    },
    Resource {
        name: "TMR7",
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
        name: "I2C4",
        index: 277,
    },
    Resource {
        name: "I2C5",
        index: 278,
    },
    Resource {
        name: "I2C6",
        index: 279,
    },
    Resource {
        name: "I2C7",
        index: 280,
    },
    Resource {
        name: "SPI0",
        index: 281,
    },
    Resource {
        name: "SPI1",
        index: 282,
    },
    Resource {
        name: "SPI2",
        index: 283,
    },
    Resource {
        name: "SPI3",
        index: 284,
    },
    Resource {
        name: "SPI4",
        index: 285,
    },
    Resource {
        name: "SPI5",
        index: 286,
    },
    Resource {
        name: "SPI6",
        index: 287,
    },
    Resource {
        name: "SPI7",
        index: 288,
    },
    Resource {
        name: "URT0",
        index: 289,
    },
    Resource {
        name: "URT1",
        index: 290,
    },
    Resource {
        name: "URT2",
        index: 291,
    },
    Resource {
        name: "URT3",
        index: 292,
    },
    Resource {
        name: "URT4",
        index: 293,
    },
    Resource {
        name: "URT5",
        index: 294,
    },
    Resource {
        name: "URT6",
        index: 295,
    },
    Resource {
        name: "URT7",
        index: 296,
    },
    Resource {
        name: "URT8",
        index: 297,
    },
    Resource {
        name: "URT9",
        index: 298,
    },
    Resource {
        name: "URT10",
        index: 299,
    },
    Resource {
        name: "URT11",
        index: 300,
    },
    Resource {
        name: "URT12",
        index: 301,
    },
    Resource {
        name: "URT13",
        index: 302,
    },
    Resource {
        name: "URT14",
        index: 303,
    },
    Resource {
        name: "URT15",
        index: 304,
    },
    Resource {
        name: "CRC0",
        index: 305,
    },
    Resource {
        name: "TSNS",
        index: 306,
    },
    Resource {
        name: "WDG0",
        index: 307,
    },
    Resource {
        name: "WDG1",
        index: 308,
    },
    Resource {
        name: "WDG2",
        index: 309,
    },
    Resource {
        name: "WDG3",
        index: 310,
    },
    Resource {
        name: "MBX0",
        index: 311,
    },
    Resource {
        name: "MBX1",
        index: 312,
    },
    Resource {
        name: "GPIO",
        index: 313,
    },
    Resource {
        name: "PPI0",
        index: 314,
    },
    Resource {
        name: "HDMA",
        index: 315,
    },
    Resource {
        name: "LOBS",
        index: 316,
    },
    Resource {
        name: "ADC0",
        index: 317,
    },
    Resource {
        name: "ADC1",
        index: 318,
    },
    Resource {
        name: "ADC2",
        index: 319,
    },
    Resource {
        name: "ADC3",
        index: 320,
    },
    Resource {
        name: "CMP0",
        index: 321,
    },
    Resource {
        name: "CMP1",
        index: 322,
    },
    Resource {
        name: "CMP2",
        index: 323,
    },
    Resource {
        name: "CMP3",
        index: 324,
    },
    Resource {
        name: "I2S0",
        index: 325,
    },
    Resource {
        name: "I2S1",
        index: 326,
    },
    Resource {
        name: "PDM0",
        index: 327,
    },
    Resource {
        name: "CLSD",
        index: 328,
    },
    Resource {
        name: "CAN0",
        index: 329,
    },
    Resource {
        name: "CAN1",
        index: 330,
    },
    Resource {
        name: "CAN2",
        index: 331,
    },
    Resource {
        name: "CAN3",
        index: 332,
    },
    Resource {
        name: "CAN4",
        index: 333,
    },
    Resource {
        name: "CAN5",
        index: 334,
    },
    Resource {
        name: "CAN6",
        index: 335,
    },
    Resource {
        name: "CAN7",
        index: 336,
    },
    Resource {
        name: "PTPC",
        index: 337,
    },
    Resource {
        name: "QEI0",
        index: 338,
    },
    Resource {
        name: "QEI1",
        index: 339,
    },
    Resource {
        name: "QEI2",
        index: 340,
    },
    Resource {
        name: "QEI3",
        index: 341,
    },
    Resource {
        name: "QEO0",
        index: 342,
    },
    Resource {
        name: "QEO1",
        index: 343,
    },
    Resource {
        name: "QEO2",
        index: 344,
    },
    Resource {
        name: "QEO3",
        index: 345,
    },
    Resource {
        name: "PWM0",
        index: 346,
    },
    Resource {
        name: "PWM1",
        index: 347,
    },
    Resource {
        name: "PWM2",
        index: 348,
    },
    Resource {
        name: "PWM3",
        index: 349,
    },
    Resource {
        name: "RDC0",
        index: 350,
    },
    Resource {
        name: "RDC1",
        index: 351,
    },
    Resource {
        name: "SDM0",
        index: 352,
    },
    Resource {
        name: "SDM1",
        index: 353,
    },
    Resource {
        name: "PLB0",
        index: 354,
    },
    Resource {
        name: "SEI0",
        index: 355,
    },
    Resource {
        name: "MTG0",
        index: 356,
    },
    Resource {
        name: "MTG1",
        index: 357,
    },
    Resource {
        name: "VSC0",
        index: 358,
    },
    Resource {
        name: "VSC1",
        index: 359,
    },
    Resource {
        name: "CLC0",
        index: 360,
    },
    Resource {
        name: "CLC1",
        index: 361,
    },
    Resource {
        name: "EMDS",
        index: 362,
    },
    Resource {
        name: "RNG0",
        index: 363,
    },
    Resource {
        name: "SDP0",
        index: 364,
    },
    Resource {
        name: "PKA0",
        index: 365,
    },
    Resource {
        name: "KMAN",
        index: 366,
    },
    Resource {
        name: "XPI0",
        index: 367,
    },
    Resource {
        name: "FEMC",
        index: 368,
    },
    Resource {
        name: "RAM0",
        index: 369,
    },
    Resource {
        name: "RAM1",
        index: 370,
    },
    Resource {
        name: "XDMA",
        index: 371,
    },
    Resource {
        name: "FFA0",
        index: 372,
    },
    Resource {
        name: "ETH0",
        index: 373,
    },
    Resource {
        name: "USB0",
        index: 374,
    },
    Resource {
        name: "NTM0",
        index: 375,
    },
    Resource {
        name: "REF0",
        index: 376,
    },
    Resource {
        name: "REF1",
        index: 377,
    },
    Resource {
        name: "TSW0",
        index: 378,
    },
    Resource {
        name: "ESC0",
        index: 379,
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
        name: "CPU1",
        index: 2,
    },
    Clock {
        name: "MCT1",
        index: 3,
    },
    Clock {
        name: "AHB0",
        index: 4,
    },
    Clock {
        name: "AXIF",
        index: 5,
    },
    Clock {
        name: "AXIS",
        index: 6,
    },
    Clock {
        name: "AXIC",
        index: 7,
    },
    Clock {
        name: "AXIN",
        index: 8,
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
        name: "TMR4",
        index: 13,
    },
    Clock {
        name: "TMR5",
        index: 14,
    },
    Clock {
        name: "TMR6",
        index: 15,
    },
    Clock {
        name: "TMR7",
        index: 16,
    },
    Clock {
        name: "I2C0",
        index: 17,
    },
    Clock {
        name: "I2C1",
        index: 18,
    },
    Clock {
        name: "I2C2",
        index: 19,
    },
    Clock {
        name: "I2C3",
        index: 20,
    },
    Clock {
        name: "I2C4",
        index: 21,
    },
    Clock {
        name: "I2C5",
        index: 22,
    },
    Clock {
        name: "I2C6",
        index: 23,
    },
    Clock {
        name: "I2C7",
        index: 24,
    },
    Clock {
        name: "SPI0",
        index: 25,
    },
    Clock {
        name: "SPI1",
        index: 26,
    },
    Clock {
        name: "SPI2",
        index: 27,
    },
    Clock {
        name: "SPI3",
        index: 28,
    },
    Clock {
        name: "SPI4",
        index: 29,
    },
    Clock {
        name: "SPI5",
        index: 30,
    },
    Clock {
        name: "SPI6",
        index: 31,
    },
    Clock {
        name: "SPI7",
        index: 32,
    },
    Clock {
        name: "URT0",
        index: 33,
    },
    Clock {
        name: "URT1",
        index: 34,
    },
    Clock {
        name: "URT2",
        index: 35,
    },
    Clock {
        name: "URT3",
        index: 36,
    },
    Clock {
        name: "URT4",
        index: 37,
    },
    Clock {
        name: "URT5",
        index: 38,
    },
    Clock {
        name: "URT6",
        index: 39,
    },
    Clock {
        name: "URT7",
        index: 40,
    },
    Clock {
        name: "URT8",
        index: 41,
    },
    Clock {
        name: "URT9",
        index: 42,
    },
    Clock {
        name: "URT10",
        index: 43,
    },
    Clock {
        name: "URT11",
        index: 44,
    },
    Clock {
        name: "URT12",
        index: 45,
    },
    Clock {
        name: "URT13",
        index: 46,
    },
    Clock {
        name: "URT14",
        index: 47,
    },
    Clock {
        name: "URT15",
        index: 48,
    },
    Clock {
        name: "ANA0",
        index: 49,
    },
    Clock {
        name: "ANA1",
        index: 50,
    },
    Clock {
        name: "ANA2",
        index: 51,
    },
    Clock {
        name: "ANA3",
        index: 52,
    },
    Clock {
        name: "AUD0",
        index: 53,
    },
    Clock {
        name: "AUD1",
        index: 54,
    },
    Clock {
        name: "CAN0",
        index: 55,
    },
    Clock {
        name: "CAN1",
        index: 56,
    },
    Clock {
        name: "CAN2",
        index: 57,
    },
    Clock {
        name: "CAN3",
        index: 58,
    },
    Clock {
        name: "CAN4",
        index: 59,
    },
    Clock {
        name: "CAN5",
        index: 60,
    },
    Clock {
        name: "CAN6",
        index: 61,
    },
    Clock {
        name: "CAN7",
        index: 62,
    },
    Clock {
        name: "XPI0",
        index: 63,
    },
    Clock {
        name: "FEMC",
        index: 64,
    },
    Clock {
        name: "ETH0",
        index: 65,
    },
    Clock {
        name: "PTP0",
        index: 66,
    },
    Clock {
        name: "NTM0",
        index: 67,
    },
    Clock {
        name: "REF0",
        index: 68,
    },
    Clock {
        name: "REF1",
        index: 69,
    },
    Clock {
        name: "TSW1",
        index: 70,
    },
    Clock {
        name: "TSW2",
        index: 71,
    },
    Clock {
        name: "TSW3",
        index: 72,
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
        name: "PV13",
        index: 365,
    },
    IoPin {
        name: "PV14",
        index: 366,
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
        name: "PW13",
        index: 397,
    },
    IoPin {
        name: "PW14",
        index: 398,
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
        name: "PW18",
        index: 402,
    },
    IoPin {
        name: "PW19",
        index: 403,
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
        name: "PW22",
        index: 406,
    },
    IoPin {
        name: "PW23",
        index: 407,
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

#[path = "../registers/acmp_v6e.rs"]
pub mod acmp;
#[path = "../registers/adc16_v6e.rs"]
pub mod adc16;
#[path = "../registers/bcfg_v68.rs"]
pub mod bcfg;
#[path = "../registers/bkey_common.rs"]
pub mod bkey;
#[path = "../registers/bmon_common.rs"]
pub mod bmon;
#[path = "../registers/bpor_v68.rs"]
pub mod bpor;
#[path = "../registers/bsec_common.rs"]
pub mod bsec;
#[path = "../registers/clc_v6e.rs"]
pub mod clc;
#[path = "../registers/crc_common.rs"]
pub mod crc;
#[path = "../registers/dao_v68.rs"]
pub mod dao;
#[path = "../registers/dma_v6e.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/enet_v68.rs"]
pub mod enet;
#[path = "../registers/esc_v6e.rs"]
pub mod esc;
#[path = "../registers/femc_common.rs"]
pub mod femc;
#[path = "../registers/ffa_v6e.rs"]
pub mod ffa;
#[path = "../registers/gpio_v53.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v67.rs"]
pub mod gpiom;
#[path = "../registers/i2c_v53.rs"]
pub mod i2c;
#[path = "../registers/i2s_common.rs"]
pub mod i2s;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/keym_common.rs"]
pub mod keym;
#[path = "../registers/lobs_v6e.rs"]
pub mod lobs;
#[path = "../registers/mbx_common.rs"]
pub mod mbx;
#[path = "../registers/mcan_v53.rs"]
pub mod mcan;
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/mono_common.rs"]
pub mod mono;
#[path = "../registers/mtg_v6e.rs"]
pub mod mtg;
#[path = "../registers/otp_common.rs"]
pub mod otp;
#[path = "../registers/pcfg_v6e.rs"]
pub mod pcfg;
#[path = "../registers/pdm_common.rs"]
pub mod pdm;
#[path = "../registers/plb_v6e.rs"]
pub mod plb;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../registers/pmon_common.rs"]
pub mod pmon;
#[path = "../registers/ppi_v6e.rs"]
pub mod ppi;
#[path = "../registers/ppor_v53.rs"]
pub mod ppor;
#[path = "../registers/psec_common.rs"]
pub mod psec;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwm_v6e.rs"]
pub mod pwm;
#[path = "../registers/qei_v6e.rs"]
pub mod qei;
#[path = "../registers/qeo_v6e.rs"]
pub mod qeo;
#[path = "../registers/rdc_v6e.rs"]
pub mod rdc;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdm_v6e.rs"]
pub mod sdm;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/sei_v6e.rs"]
pub mod sei;
#[path = "../registers/spi_v53.rs"]
pub mod spi;
#[path = "../registers/synt_v53.rs"]
pub mod synt;
#[path = "../registers/sysctl_v6e.rs"]
pub mod sysctl;
#[path = "../registers/tamp_v62.rs"]
pub mod tamp;
#[path = "../registers/tmr_v6e.rs"]
pub mod tmr;
#[path = "../registers/trgm_v53.rs"]
pub mod trgm;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/uart_v53.rs"]
pub mod uart;
#[path = "../registers/usb_v53.rs"]
pub mod usb;
#[path = "../registers/vsc_v6e.rs"]
pub mod vsc;
#[path = "../registers/wdg_v53.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
