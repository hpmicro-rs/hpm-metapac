
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
    Peripheral {
        name: "SYSCTL",
        address: 0xf4000000,
        registers: Some(PeripheralRegisters {
            kind: "sysctl",
            version: "v67",
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
        name: "XPI0",
        address: 0xf3040000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 12,
            resource_clock_top: Some(73),
            resource: 268,
            clock_node: Some(9),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(108),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(109),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI0",
        }],
    },
    Peripheral {
        name: "XPI1",
        address: 0xf3044000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 13,
            resource_clock_top: Some(74),
            resource: 269,
            clock_node: Some(10),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(110),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(111),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI1",
        }],
    },
    Peripheral {
        name: "PLLCTL",
        address: 0xf4100000,
        registers: Some(PeripheralRegisters {
            kind: "pllctl",
            version: "v67",
            block: "PLLCTL",
            ir: &pllctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BCFG",
        address: 0xf5008000,
        registers: Some(PeripheralRegisters {
            kind: "bcfg",
            version: "v67",
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
        address: 0xf5004000,
        registers: Some(PeripheralRegisters {
            kind: "bpor",
            version: "v67",
            block: "BPOR",
            ir: &bpor::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BUTN",
        address: 0xf500c000,
        registers: Some(PeripheralRegisters {
            kind: "butn",
            version: "common",
            block: "BUTN",
            ir: &butn::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "BUTN",
        }],
    },
    Peripheral {
        name: "PCFG",
        address: 0xf40c4000,
        registers: Some(PeripheralRegisters {
            kind: "pcfg",
            version: "v67",
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
        address: 0xf40c0000,
        registers: Some(PeripheralRegisters {
            kind: "ppor",
            version: "v67",
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
            version: "v67",
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
        address: 0xf40d8000,
        registers: Some(PeripheralRegisters {
            kind: "ioc",
            version: "v67",
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
        address: 0xf5010000,
        registers: Some(PeripheralRegisters {
            kind: "ioc",
            version: "v67",
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
        address: 0xc0000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "common",
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
        address: 0xf0000000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "common",
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
        address: 0xf0004000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "common",
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
        address: 0xf40dc000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "common",
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
        address: 0xf5014000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "common",
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
        address: 0xf0008000,
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
        name: "OTPSHW",
        address: 0xf4080000,
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
        name: "OTP",
        address: 0xf40c8000,
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
        name: "FEMC",
        address: 0xf3050000,
        registers: Some(PeripheralRegisters {
            kind: "femc",
            version: "common",
            block: "FEMC",
            ir: &femc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 4,
            resource_clock_top: Some(72),
            resource: 260,
            clock_node: Some(8),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC08",
                signal: "A00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "A01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "A02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "A03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "A04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "A05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "A06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "A07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "A08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "A09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "A10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "A11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "A12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "BA0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "BA1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "CAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CKE",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "CLK",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CS0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CS1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "DM0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "DM1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "DM2",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "DM3",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "DQ00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "DQ01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "DQ02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "DQ03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "DQ04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "DQ05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "DQ06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "DQ07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "DQ08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "DQ09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "DQ10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "DQ11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DQ12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "DQ13",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DQ14",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "DQ15",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "DQ16",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "DQ17",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "DQ18",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "DQ19",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "DQ20",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "DQ21",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "DQ22",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "DQ23",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "DQ24",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "DQ25",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "DQ26",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "DQ27",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "DQ28",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "DQ29",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "DQ30",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "DQ31",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "DQS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "WE",
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
        name: "SDXC0",
        address: 0xf2030000,
        registers: Some(PeripheralRegisters {
            kind: "sdxc",
            version: "v67",
            block: "SDXC",
            ir: &sdxc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 24,
            resource_clock_top: None,
            resource: 344,
            clock_node: Some(65),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDXC0",
        }],
    },
    Peripheral {
        name: "SDXC1",
        address: 0xf2034000,
        registers: Some(PeripheralRegisters {
            kind: "sdxc",
            version: "v67",
            block: "SDXC",
            ir: &sdxc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 25,
            resource_clock_top: None,
            resource: 345,
            clock_node: Some(66),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDXC1",
        }],
    },
    Peripheral {
        name: "HDMA",
        address: 0xf00c4000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v67",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 273,
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
        address: 0xf3048000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v67",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 18,
            resource_clock_top: None,
            resource: 274,
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
        address: 0xf00c0000,
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
        sysctl: None,
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
        sysctl: None,
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
        name: "NTMR0",
        address: 0xf2010000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 342,
            clock_node: Some(63),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(64),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "NTMR0",
        }],
    },
    Peripheral {
        name: "NTMR1",
        address: 0xf2014000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 23,
            resource_clock_top: None,
            resource: 343,
            clock_node: Some(64),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(70),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "NTMR1",
        }],
    },
    Peripheral {
        name: "GPTMR0",
        address: 0xf3000000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: None,
            resource: 282,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(75),
            },
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
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(72),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR0",
        }],
    },
    Peripheral {
        name: "GPTMR1",
        address: 0xf3004000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 27,
            resource_clock_top: None,
            resource: 283,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(77),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR1",
        }],
    },
    Peripheral {
        name: "GPTMR2",
        address: 0xf3008000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 28,
            resource_clock_top: None,
            resource: 284,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE23",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(82),
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
            interrupt: "GPTMR2",
        }],
    },
    Peripheral {
        name: "GPTMR3",
        address: 0xf300c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: None,
            resource: 285,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE14",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "COMP1",
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
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(84),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(86),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR3",
        }],
    },
    Peripheral {
        name: "GPTMR4",
        address: 0xf3010000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: None,
            resource: 286,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE21",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(89),
            },
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
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(91),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR4",
        }],
    },
    Peripheral {
        name: "GPTMR5",
        address: 0xf3014000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: None,
            resource: 287,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PF08",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(93),
            },
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
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR5",
        }],
    },
    Peripheral {
        name: "GPTMR6",
        address: 0xf3018000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 0,
            resource_clock_top: None,
            resource: 288,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PY09",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(98),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(96),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR6",
        }],
    },
    Peripheral {
        name: "GPTMR7",
        address: 0xf301c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 1,
            resource_clock_top: None,
            resource: 289,
            clock_node: Some(18),
        }),
        pins: &[
            PeripheralPin {
                pin: "PZ04",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ10",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(100),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(103),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(102),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR7",
        }],
    },
    Peripheral {
        name: "PTMR",
        address: 0xf40e0000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[
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
                pin: "PY09",
                signal: "CAPT2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY11",
                signal: "CAPT3",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "COMP3",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY10",
                signal: "COMP3",
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
        address: 0xf0090000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v67",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 278,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WDG0",
        }],
    },
    Peripheral {
        name: "WDG1",
        address: 0xf0094000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v67",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 23,
            resource_clock_top: None,
            resource: 279,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WDG1",
        }],
    },
    Peripheral {
        name: "WDG2",
        address: 0xf0098000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v67",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 24,
            resource_clock_top: None,
            resource: 280,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WDG2",
        }],
    },
    Peripheral {
        name: "WDG3",
        address: 0xf009c000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v67",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: None,
            resource: 281,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WDG3",
        }],
    },
    Peripheral {
        name: "PWDG",
        address: 0xf40e8000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v67",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWDG",
        }],
    },
    Peripheral {
        name: "RTCSHW",
        address: 0xf501c000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "common",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0xf5044000,
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 2,
            resource_clock_top: None,
            resource: 290,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB17",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "TXD",
                alt: Some(2),
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 3,
            resource_clock_top: None,
            resource: 291,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB18",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "TXD",
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 4,
            resource_clock_top: None,
            resource: 292,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB08",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(12),
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 5,
            resource_clock_top: None,
            resource: 293,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(15),
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 6,
            resource_clock_top: None,
            resource: 294,
            clock_node: Some(23),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD00",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "TXD",
                alt: Some(2),
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 7,
            resource_clock_top: None,
            resource: 295,
            clock_node: Some(24),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD08",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "TXD",
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 8,
            resource_clock_top: None,
            resource: 296,
            clock_node: Some(25),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD09",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(21),
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
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 9,
            resource_clock_top: None,
            resource: 297,
            clock_node: Some(26),
        }),
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "TXD",
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
        name: "UART8",
        address: 0xf0060000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 10,
            resource_clock_top: None,
            resource: 298,
            clock_node: Some(27),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "TXD",
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
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(24),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
    },
    Peripheral {
        name: "UART9",
        address: 0xf0064000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 11,
            resource_clock_top: None,
            resource: 299,
            clock_node: Some(28),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(26),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART9",
        }],
    },
    Peripheral {
        name: "UART10",
        address: 0xf0068000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 12,
            resource_clock_top: None,
            resource: 300,
            clock_node: Some(29),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA16",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "TXD",
                alt: Some(2),
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
            interrupt: "UART10",
        }],
    },
    Peripheral {
        name: "UART11",
        address: 0xf006c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 13,
            resource_clock_top: None,
            resource: 301,
            clock_node: Some(30),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(31),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART11",
        }],
    },
    Peripheral {
        name: "UART12",
        address: 0xf0070000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 302,
            clock_node: Some(31),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "TXD",
                alt: Some(2),
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
            interrupt: "UART12",
        }],
    },
    Peripheral {
        name: "UART13",
        address: 0xf0074000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 303,
            clock_node: Some(32),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA31",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ08",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ09",
                signal: "TXD",
                alt: Some(2),
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
            interrupt: "UART13",
        }],
    },
    Peripheral {
        name: "UART14",
        address: 0xf0078000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: None,
            resource: 304,
            clock_node: Some(33),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ10",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ11",
                signal: "TXD",
                alt: Some(2),
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
            interrupt: "UART14",
        }],
    },
    Peripheral {
        name: "UART15",
        address: 0xf007c000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 305,
            clock_node: Some(34),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ00",
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
            interrupt: "UART15",
        }],
    },
    Peripheral {
        name: "PUART",
        address: 0xf40e4000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v67",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PY09",
                signal: "CTS",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "RTS",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "RXD",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "TXD",
                alt: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PUART",
        }],
    },
    Peripheral {
        name: "SPI0",
        address: 0xf0030000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v67",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 22,
            resource_clock_top: Some(103),
            resource: 310,
            clock_node: Some(39),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA05",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "SCLK",
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
        address: 0xf0034000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v67",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 23,
            resource_clock_top: Some(104),
            resource: 311,
            clock_node: Some(40),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA18",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY09",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 0xf0038000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v67",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 24,
            resource_clock_top: Some(105),
            resource: 312,
            clock_node: Some(41),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(5),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0xf003c000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v67",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 25,
            resource_clock_top: Some(106),
            resource: 313,
            clock_node: Some(42),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB29",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "I2C0",
        address: 0xf3020000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v67",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 18,
            resource_clock_top: Some(99),
            resource: 306,
            clock_node: Some(35),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ11",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ10",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(104),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C0",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0xf3024000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v67",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 19,
            resource_clock_top: Some(100),
            resource: 307,
            clock_node: Some(36),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(105),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C1",
        }],
    },
    Peripheral {
        name: "I2C2",
        address: 0xf3028000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v67",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 20,
            resource_clock_top: Some(101),
            resource: 308,
            clock_node: Some(37),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(106),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C2",
        }],
    },
    Peripheral {
        name: "I2C3",
        address: 0xf302c000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v67",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 21,
            resource_clock_top: Some(102),
            resource: 309,
            clock_node: Some(38),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(107),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C3",
        }],
    },
    Peripheral {
        name: "CAN0",
        address: 0xf0080000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "v67",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 26,
            resource_clock_top: Some(107),
            resource: 314,
            clock_node: Some(43),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CAN0",
        }],
    },
    Peripheral {
        name: "CAN1",
        address: 0xf0084000,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "v67",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 27,
            resource_clock_top: Some(108),
            resource: 315,
            clock_node: Some(44),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CAN1",
        }],
    },
    Peripheral {
        name: "PTPC",
        address: 0xf00b0000,
        registers: Some(PeripheralRegisters {
            kind: "ptpc",
            version: "common",
            block: "PTPC",
            ir: &ptpc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 30,
            resource_clock_top: Some(111),
            resource: 318,
            clock_node: Some(47),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTPC",
        }],
    },
    Peripheral {
        name: "ENET0",
        address: 0xf2000000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v67",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 20,
            resource_clock_top: Some(121),
            resource: 340,
            clock_node: Some(57),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET0",
        }],
    },
    Peripheral {
        name: "ENET1",
        address: 0xf2004000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v67",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 21,
            resource_clock_top: Some(122),
            resource: 341,
            clock_node: Some(58),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET1",
        }],
    },
    Peripheral {
        name: "USB0",
        address: 0xf2020000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v67",
            block: "USB",
            ir: &usb::REGISTERS,
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
                pin: "PF10",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "PWR",
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
        name: "USB1",
        address: 0xf2024000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v67",
            block: "USB",
            ir: &usb::REGISTERS,
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
                pin: "PF07",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "PWR",
                alt: Some(24),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB1",
        }],
    },
    Peripheral {
        name: "CONCTL",
        address: 0xf2040000,
        registers: Some(PeripheralRegisters {
            kind: "conctl",
            version: "v67",
            block: "CONCTL",
            ir: &conctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC0",
        address: 0xf0010000,
        registers: Some(PeripheralRegisters {
            kind: "adc12",
            version: "v67",
            block: "ADC",
            ir: &adc12::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 31,
            resource_clock_top: Some(192),
            resource: 319,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE31",
                signal: "INN16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "INN17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "INP0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "INP10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "INP11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "INP12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "INP13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "INP14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "INP15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "INP16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "INP17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "INP8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "INP9",
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
        address: 0xf0014000,
        registers: Some(PeripheralRegisters {
            kind: "adc12",
            version: "v67",
            block: "ADC",
            ir: &adc12::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 0,
            resource_clock_top: Some(193),
            resource: 320,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF03",
                signal: "INN16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "INN17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "INP0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "INP10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "INP11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "INP12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "INP13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "INP14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "INP15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "INP16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "INP17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "INP8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "INP9",
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
        address: 0xf0018000,
        registers: Some(PeripheralRegisters {
            kind: "adc12",
            version: "v67",
            block: "ADC",
            ir: &adc12::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 1,
            resource_clock_top: Some(194),
            resource: 321,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF07",
                signal: "INN16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "INN17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "INP0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "INP10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "INP11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "INP12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "INP13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "INP14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "INP15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "INP16",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "INP17",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "INP8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "INP9",
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
        name: "ADC3",
        address: 0xf001c000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v67",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 2,
            resource_clock_top: Some(195),
            resource: 322,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE19",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "IN7",
                alt: Some(0),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "ACMP",
        address: 0xf0020000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "common",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 3,
            resource_clock_top: None,
            resource: 323,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF09",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "CMP1_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "CMP1_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CMP1_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "CMP1_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP2_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP2_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP2_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CMP2_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "CMP2_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "CMP2_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "CMP2_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP2_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP2_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "CMP2_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP2_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "CMP2_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CMP2_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "CMP2_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CMP3_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "CMP3_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "CMP3_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "CMP3_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "CMP3_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "CMP3_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "CMP3_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "CMP3_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CMP3_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "CMP3_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CMP3_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "CMP3_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CMP3_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CMP3_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PZ08",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PZ09",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "COMP2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY08",
                signal: "COMP2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PZ11",
                signal: "COMP2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "COMP3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY09",
                signal: "COMP3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PZ10",
                signal: "COMP3",
                alt: Some(16),
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
            PeripheralDmaChannel {
                signal: "3",
                dmamux: Some("DMAMUX"),
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "2",
                dmamux: Some("DMAMUX"),
                request: Some(114),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "ACMP_0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "ACMP_1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "ACMP_2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "ACMP_3",
            },
        ],
    },
    Peripheral {
        name: "LCDC",
        address: 0xf1000000,
        registers: Some(PeripheralRegisters {
            kind: "lcdc",
            version: "v67",
            block: "LCDC",
            ir: &lcdc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 15,
            resource_clock_top: Some(118),
            resource: 335,
            clock_node: Some(54),
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "D0",
                interrupt: "LCDC_D0",
            },
            PeripheralInterrupt {
                signal: "D1",
                interrupt: "LCDC_D1",
            },
        ],
    },
    Peripheral {
        name: "CAM0",
        address: 0xf1008000,
        registers: Some(PeripheralRegisters {
            kind: "cam",
            version: "v67",
            block: "CAM",
            ir: &cam::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 16,
            resource_clock_top: Some(119),
            resource: 336,
            clock_node: Some(55),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA07",
                signal: "D2",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "D2",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "D3",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "D3",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "D4",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "D4",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "D5",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "D5",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "D6",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "D6",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "D7",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "D7",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "D8",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "D8",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "D9",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "D9",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "HSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "HSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "PIXCLK",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "PIXCLK",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "VSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "VSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "XCLK",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "XCLK",
                alt: Some(22),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CAM0",
        }],
    },
    Peripheral {
        name: "CAM1",
        address: 0xf100c000,
        registers: Some(PeripheralRegisters {
            kind: "cam",
            version: "v67",
            block: "CAM",
            ir: &cam::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 17,
            resource_clock_top: Some(120),
            resource: 337,
            clock_node: Some(56),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB03",
                signal: "D2",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "D3",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "D4",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "D5",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "D6",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "D7",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "D8",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "D9",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "HSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "PIXCLK",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "VSYNC",
                alt: Some(22),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "XCLK",
                alt: Some(22),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CAM1",
        }],
    },
    Peripheral {
        name: "PDMA",
        address: 0xf1010000,
        registers: Some(PeripheralRegisters {
            kind: "pdma",
            version: "v67",
            block: "PDMA",
            ir: &pdma::REGISTERS,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "D0",
                interrupt: "PDMA_D0",
            },
            PeripheralInterrupt {
                signal: "D1",
                interrupt: "PDMA_D1",
            },
        ],
    },
    Peripheral {
        name: "JPEG",
        address: 0xf1014000,
        registers: Some(PeripheralRegisters {
            kind: "jpeg",
            version: "common",
            block: "JPEG",
            ir: &jpeg::REGISTERS,
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
            interrupt: "JPEG",
        }],
    },
    Peripheral {
        name: "I2S0",
        address: 0xf0100000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 4,
            resource_clock_top: Some(196),
            resource: 324,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE20",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "FCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "MCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PZ09",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "RXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PZ08",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "RXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "RXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "RXD3",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "TXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "TXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "TXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "TXD3",
                alt: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(41),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S0",
        }],
    },
    Peripheral {
        name: "I2S1",
        address: 0xf0104000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 5,
            resource_clock_top: Some(197),
            resource: 325,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA05",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "FCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "MCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY11",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "RXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY10",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RXD3",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "TXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "TXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TXD3",
                alt: Some(9),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(42),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S1",
        }],
    },
    Peripheral {
        name: "I2S2",
        address: 0xf0108000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 6,
            resource_clock_top: Some(198),
            resource: 326,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA03",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "FCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "FCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "MCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "MCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(44),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S2",
        }],
    },
    Peripheral {
        name: "I2S3",
        address: 0xf010c000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 7,
            resource_clock_top: Some(199),
            resource: 327,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "BCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "MCLK",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD3",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "TXD0",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "TXD1",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD2",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD3",
                alt: Some(9),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(47),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S3",
        }],
    },
    Peripheral {
        name: "PDM",
        address: 0xf0114000,
        registers: Some(PeripheralRegisters {
            kind: "pdm",
            version: "common",
            block: "PDM",
            ir: &pdm::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 8,
            resource_clock_top: None,
            resource: 328,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "PDM",
            },
            PeripheralInterrupt {
                signal: "D0",
                interrupt: "PDMA_D0",
            },
            PeripheralInterrupt {
                signal: "D1",
                interrupt: "PDMA_D1",
            },
        ],
    },
    Peripheral {
        name: "DAO",
        address: 0xf0110000,
        registers: Some(PeripheralRegisters {
            kind: "dao",
            version: "v67",
            block: "DAO",
            ir: &dao::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 9,
            resource_clock_top: None,
            resource: 329,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAO",
        }],
    },
    Peripheral {
        name: "VAD",
        address: 0xf40ec000,
        registers: Some(PeripheralRegisters {
            kind: "vad",
            version: "common",
            block: "VAD",
            ir: &vad::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "VAD",
        }],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0200000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v67",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "P7",
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
        address: 0xf0210000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v67",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC03",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "P7",
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
        address: 0xf0220000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v67",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC31",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "P7",
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
        address: 0xf0230000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v67",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "P7",
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
        name: "QEI0",
        address: 0xf0208000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v67",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI0",
        }],
    },
    Peripheral {
        name: "QEI1",
        address: 0xf0218000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v67",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI1",
        }],
    },
    Peripheral {
        name: "QEI2",
        address: 0xf0228000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v67",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI2",
        }],
    },
    Peripheral {
        name: "QEI3",
        address: 0xf0238000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v67",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI3",
        }],
    },
    Peripheral {
        name: "HALL0",
        address: 0xf0204000,
        registers: Some(PeripheralRegisters {
            kind: "hall",
            version: "common",
            block: "HALL",
            ir: &hall::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL0",
        }],
    },
    Peripheral {
        name: "HALL1",
        address: 0xf0214000,
        registers: Some(PeripheralRegisters {
            kind: "hall",
            version: "common",
            block: "HALL",
            ir: &hall::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL1",
        }],
    },
    Peripheral {
        name: "HALL2",
        address: 0xf0224000,
        registers: Some(PeripheralRegisters {
            kind: "hall",
            version: "common",
            block: "HALL",
            ir: &hall::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL2",
        }],
    },
    Peripheral {
        name: "HALL3",
        address: 0xf0234000,
        registers: Some(PeripheralRegisters {
            kind: "hall",
            version: "common",
            block: "HALL",
            ir: &hall::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL3",
        }],
    },
    Peripheral {
        name: "TRGM0",
        address: 0xf020c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v67",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC17",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P11",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM1",
        address: 0xf021c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v67",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "P11",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM2",
        address: 0xf022c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v67",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC22",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "P11",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM3",
        address: 0xf023c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v67",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PD18",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "P11",
                alt: Some(16),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYNT",
        address: 0xf0240000,
        registers: Some(PeripheralRegisters {
            kind: "synt",
            version: "v67",
            block: "SYNT",
            ir: &synt::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 10,
            resource_clock_top: None,
            resource: 330,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SDP",
        address: 0xf304c000,
        registers: Some(PeripheralRegisters {
            kind: "sdp",
            version: "v67",
            block: "SDP",
            ir: &sdp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 270,
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
        address: 0xf00c8000,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "common",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 271,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
    },
    Peripheral {
        name: "KEYM",
        address: 0xf00cc000,
        registers: Some(PeripheralRegisters {
            kind: "keym",
            version: "common",
            block: "KEYM",
            ir: &keym::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 16,
            resource_clock_top: None,
            resource: 272,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BKEY",
        address: 0xf5048000,
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
        address: 0xf40cc000,
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
        address: 0xf40d0000,
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
        address: 0xf5040000,
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
        address: 0xf504c000,
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
        address: 0xf5050000,
        registers: Some(PeripheralRegisters {
            kind: "tamp",
            version: "v67",
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
        address: 0xf5054000,
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
        name: "GPIO1_A",
        number: 10,
    },
    Interrupt {
        name: "GPIO1_B",
        number: 11,
    },
    Interrupt {
        name: "GPIO1_C",
        number: 12,
    },
    Interrupt {
        name: "GPIO1_D",
        number: 13,
    },
    Interrupt {
        name: "GPIO1_E",
        number: 14,
    },
    Interrupt {
        name: "GPIO1_F",
        number: 15,
    },
    Interrupt {
        name: "GPIO1_X",
        number: 16,
    },
    Interrupt {
        name: "GPIO1_Y",
        number: 17,
    },
    Interrupt {
        name: "GPIO1_Z",
        number: 18,
    },
    Interrupt {
        name: "ADC0",
        number: 19,
    },
    Interrupt {
        name: "ADC1",
        number: 20,
    },
    Interrupt {
        name: "ADC2",
        number: 21,
    },
    Interrupt {
        name: "ADC3",
        number: 22,
    },
    Interrupt {
        name: "ACMP_0",
        number: 23,
    },
    Interrupt {
        name: "ACMP_1",
        number: 24,
    },
    Interrupt {
        name: "ACMP_2",
        number: 25,
    },
    Interrupt {
        name: "ACMP_3",
        number: 26,
    },
    Interrupt {
        name: "SPI0",
        number: 27,
    },
    Interrupt {
        name: "SPI1",
        number: 28,
    },
    Interrupt {
        name: "SPI2",
        number: 29,
    },
    Interrupt {
        name: "SPI3",
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
        name: "UART8",
        number: 39,
    },
    Interrupt {
        name: "UART9",
        number: 40,
    },
    Interrupt {
        name: "UART10",
        number: 41,
    },
    Interrupt {
        name: "UART11",
        number: 42,
    },
    Interrupt {
        name: "UART12",
        number: 43,
    },
    Interrupt {
        name: "UART13",
        number: 44,
    },
    Interrupt {
        name: "UART14",
        number: 45,
    },
    Interrupt {
        name: "UART15",
        number: 46,
    },
    Interrupt {
        name: "CAN0",
        number: 47,
    },
    Interrupt {
        name: "CAN1",
        number: 48,
    },
    Interrupt {
        name: "CAN2",
        number: 49,
    },
    Interrupt {
        name: "CAN3",
        number: 50,
    },
    Interrupt {
        name: "PTPC",
        number: 51,
    },
    Interrupt {
        name: "WDG0",
        number: 52,
    },
    Interrupt {
        name: "WDG1",
        number: 53,
    },
    Interrupt {
        name: "WDG2",
        number: 54,
    },
    Interrupt {
        name: "WDG3",
        number: 55,
    },
    Interrupt {
        name: "MBX0A",
        number: 56,
    },
    Interrupt {
        name: "MBX0B",
        number: 57,
    },
    Interrupt {
        name: "MBX1A",
        number: 58,
    },
    Interrupt {
        name: "MBX1B",
        number: 59,
    },
    Interrupt {
        name: "GPTMR0",
        number: 60,
    },
    Interrupt {
        name: "GPTMR1",
        number: 61,
    },
    Interrupt {
        name: "GPTMR2",
        number: 62,
    },
    Interrupt {
        name: "GPTMR3",
        number: 63,
    },
    Interrupt {
        name: "GPTMR4",
        number: 64,
    },
    Interrupt {
        name: "GPTMR5",
        number: 65,
    },
    Interrupt {
        name: "GPTMR6",
        number: 66,
    },
    Interrupt {
        name: "GPTMR7",
        number: 67,
    },
    Interrupt {
        name: "I2C0",
        number: 68,
    },
    Interrupt {
        name: "I2C1",
        number: 69,
    },
    Interrupt {
        name: "I2C2",
        number: 70,
    },
    Interrupt {
        name: "I2C3",
        number: 71,
    },
    Interrupt {
        name: "PWM0",
        number: 72,
    },
    Interrupt {
        name: "HALL0",
        number: 73,
    },
    Interrupt {
        name: "QEI0",
        number: 74,
    },
    Interrupt {
        name: "PWM1",
        number: 75,
    },
    Interrupt {
        name: "HALL1",
        number: 76,
    },
    Interrupt {
        name: "QEI1",
        number: 77,
    },
    Interrupt {
        name: "PWM2",
        number: 78,
    },
    Interrupt {
        name: "HALL2",
        number: 79,
    },
    Interrupt {
        name: "QEI2",
        number: 80,
    },
    Interrupt {
        name: "PWM3",
        number: 81,
    },
    Interrupt {
        name: "HALL3",
        number: 82,
    },
    Interrupt {
        name: "QEI3",
        number: 83,
    },
    Interrupt {
        name: "SDP",
        number: 84,
    },
    Interrupt {
        name: "XPI0",
        number: 85,
    },
    Interrupt {
        name: "XPI1",
        number: 86,
    },
    Interrupt {
        name: "XDMA",
        number: 87,
    },
    Interrupt {
        name: "HDMA",
        number: 88,
    },
    Interrupt {
        name: "FEMC",
        number: 89,
    },
    Interrupt {
        name: "RNG",
        number: 90,
    },
    Interrupt {
        name: "I2S0",
        number: 91,
    },
    Interrupt {
        name: "I2S1",
        number: 92,
    },
    Interrupt {
        name: "I2S2",
        number: 93,
    },
    Interrupt {
        name: "I2S3",
        number: 94,
    },
    Interrupt {
        name: "DAO",
        number: 95,
    },
    Interrupt {
        name: "PDM",
        number: 96,
    },
    Interrupt {
        name: "CAM0",
        number: 97,
    },
    Interrupt {
        name: "CAM1",
        number: 98,
    },
    Interrupt {
        name: "LCDC_D0",
        number: 99,
    },
    Interrupt {
        name: "LCDC_D1",
        number: 100,
    },
    Interrupt {
        name: "PDMA_D0",
        number: 101,
    },
    Interrupt {
        name: "PDMA_D1",
        number: 102,
    },
    Interrupt {
        name: "JPEG",
        number: 103,
    },
    Interrupt {
        name: "NTMR0",
        number: 104,
    },
    Interrupt {
        name: "NTMR1",
        number: 105,
    },
    Interrupt {
        name: "USB0",
        number: 106,
    },
    Interrupt {
        name: "USB1",
        number: 107,
    },
    Interrupt {
        name: "ENET0",
        number: 108,
    },
    Interrupt {
        name: "ENET1",
        number: 109,
    },
    Interrupt {
        name: "SDXC0",
        number: 110,
    },
    Interrupt {
        name: "SDXC1",
        number: 111,
    },
    Interrupt {
        name: "PSEC",
        number: 112,
    },
    Interrupt {
        name: "PGPIO",
        number: 113,
    },
    Interrupt {
        name: "PWDG",
        number: 114,
    },
    Interrupt {
        name: "PTMR",
        number: 115,
    },
    Interrupt {
        name: "PUART",
        number: 116,
    },
    Interrupt {
        name: "VAD",
        number: 117,
    },
    Interrupt {
        name: "FUSE",
        number: 118,
    },
    Interrupt {
        name: "SECMON",
        number: 119,
    },
    Interrupt {
        name: "RTC",
        number: 120,
    },
    Interrupt {
        name: "BUTN",
        number: 121,
    },
    Interrupt {
        name: "BGPIO",
        number: 122,
    },
    Interrupt {
        name: "BVIO",
        number: 123,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 124,
    },
    Interrupt {
        name: "SYSCTL",
        number: 125,
    },
    Interrupt {
        name: "DEBUG_0",
        number: 126,
    },
    Interrupt {
        name: "DEBUG_1",
        number: 127,
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
        name: "XDMA_CH0",
        dma: "XDMA",
        channel: 0,
        dmamux_channel: 8,
    },
    DmaChannel {
        name: "XDMA_CH1",
        dma: "XDMA",
        channel: 1,
        dmamux_channel: 9,
    },
    DmaChannel {
        name: "XDMA_CH2",
        dma: "XDMA",
        channel: 2,
        dmamux_channel: 10,
    },
    DmaChannel {
        name: "XDMA_CH3",
        dma: "XDMA",
        channel: 3,
        dmamux_channel: 11,
    },
    DmaChannel {
        name: "XDMA_CH4",
        dma: "XDMA",
        channel: 4,
        dmamux_channel: 12,
    },
    DmaChannel {
        name: "XDMA_CH5",
        dma: "XDMA",
        channel: 5,
        dmamux_channel: 13,
    },
    DmaChannel {
        name: "XDMA_CH6",
        dma: "XDMA",
        channel: 6,
        dmamux_channel: 14,
    },
    DmaChannel {
        name: "XDMA_CH7",
        dma: "XDMA",
        channel: 7,
        dmamux_channel: 15,
    },
];
pub(crate) static RESOURCES: &[Resource] = &[
    Resource {
        name: "CPU0_CORE",
        index: 0,
    },
    Resource {
        name: "CPU0_SUBSYS",
        index: 1,
    },
    Resource {
        name: "CPU1_CORE",
        index: 8,
    },
    Resource {
        name: "CPX1_SUBSYS",
        index: 9,
    },
    Resource {
        name: "POW_CON",
        index: 21,
    },
    Resource {
        name: "POW_VIS",
        index: 22,
    },
    Resource {
        name: "POW_CPU0",
        index: 23,
    },
    Resource {
        name: "POW_CPU1",
        index: 24,
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
        name: "RST_CPU1",
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
        name: "CLK_SRC_PLL0CLK0",
        index: 34,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 35,
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
        name: "CLK_SRC_PLL2",
        index: 38,
    },
    Resource {
        name: "CLK_SRC_PLL2CLK0",
        index: 39,
    },
    Resource {
        name: "CLK_SRC_PLL2CLK1",
        index: 40,
    },
    Resource {
        name: "CLK_SRC_PLL3",
        index: 41,
    },
    Resource {
        name: "CLK_SRC_PLL3CLK0",
        index: 42,
    },
    Resource {
        name: "CLK_SRC_PLL4",
        index: 43,
    },
    Resource {
        name: "CLK_SRC_PLL4CLK0",
        index: 44,
    },
    Resource {
        name: "CLK_TOP_CPU0",
        index: 64,
    },
    Resource {
        name: "CLK_TOP_MCHTMR0",
        index: 65,
    },
    Resource {
        name: "CLK_TOP_CPU1",
        index: 66,
    },
    Resource {
        name: "CLK_TOP_MCHTMR1",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_AXI",
        index: 68,
    },
    Resource {
        name: "CLK_TOP_CONN",
        index: 69,
    },
    Resource {
        name: "CLK_TOP_VIS",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_AHB",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_FEMC",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_XPI1",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_GPTMR0",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_GPTMR1",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_GPTMR2",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_GPTMR3",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_GPTMR4",
        index: 79,
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
        name: "CLK_TOP_GPTMR7",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_UART0",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_UART1",
        index: 84,
    },
    Resource {
        name: "CLK_TOP_UART2",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_UART3",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_UART4",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_UART5",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_UART6",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_UART7",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_UART8",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_UART9",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_UART10",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_UART11",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_UART12",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_UART13",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_UART14",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_UART15",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 99,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 101,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 104,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 105,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 106,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 107,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 108,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 109,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 110,
    },
    Resource {
        name: "CLK_TOP_PTPC",
        index: 111,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 112,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 113,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 114,
    },
    Resource {
        name: "CLK_TOP_AUD0",
        index: 115,
    },
    Resource {
        name: "CLK_TOP_AUD1",
        index: 116,
    },
    Resource {
        name: "CLK_TOP_AUD2",
        index: 117,
    },
    Resource {
        name: "CLK_TOP_LCDC",
        index: 118,
    },
    Resource {
        name: "CLK_TOP_CAM0",
        index: 119,
    },
    Resource {
        name: "CLK_TOP_CAM1",
        index: 120,
    },
    Resource {
        name: "CLK_TOP_ENET0",
        index: 121,
    },
    Resource {
        name: "CLK_TOP_ENET1",
        index: 122,
    },
    Resource {
        name: "CLK_TOP_PTP0",
        index: 123,
    },
    Resource {
        name: "CLK_TOP_PTP1",
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
        name: "CLK_TOP_NTMR0",
        index: 127,
    },
    Resource {
        name: "CLK_TOP_NTMR1",
        index: 128,
    },
    Resource {
        name: "CLK_TOP_SDXC0",
        index: 129,
    },
    Resource {
        name: "CLK_TOP_SDXC1",
        index: 130,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 192,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 193,
    },
    Resource {
        name: "CLK_TOP_ADC2",
        index: 194,
    },
    Resource {
        name: "CLK_TOP_ADC3",
        index: 195,
    },
    Resource {
        name: "CLK_TOP_I2S0",
        index: 196,
    },
    Resource {
        name: "CLK_TOP_I2S1",
        index: 197,
    },
    Resource {
        name: "CLK_TOP_I2S2",
        index: 198,
    },
    Resource {
        name: "CLK_TOP_I2S3",
        index: 199,
    },
    Resource {
        name: "AHBAPB_BUS",
        index: 256,
    },
    Resource {
        name: "AXI_BUS",
        index: 257,
    },
    Resource {
        name: "CONN_BUS",
        index: 258,
    },
    Resource {
        name: "VIS_BUS",
        index: 259,
    },
    Resource {
        name: "FEMC",
        index: 260,
    },
    Resource {
        name: "ROM",
        index: 261,
    },
    Resource {
        name: "LMM0",
        index: 262,
    },
    Resource {
        name: "LMM1",
        index: 263,
    },
    Resource {
        name: "MCHTMR0",
        index: 264,
    },
    Resource {
        name: "MCHTMR1",
        index: 265,
    },
    Resource {
        name: "AXI_SRAM0",
        index: 266,
    },
    Resource {
        name: "AXI_SRAM1",
        index: 267,
    },
    Resource {
        name: "XPI0",
        index: 268,
    },
    Resource {
        name: "XPI1",
        index: 269,
    },
    Resource {
        name: "SDP",
        index: 270,
    },
    Resource {
        name: "RNG",
        index: 271,
    },
    Resource {
        name: "KEYM",
        index: 272,
    },
    Resource {
        name: "HDMA",
        index: 273,
    },
    Resource {
        name: "XDMA",
        index: 274,
    },
    Resource {
        name: "GPIO",
        index: 275,
    },
    Resource {
        name: "MBX0",
        index: 276,
    },
    Resource {
        name: "MBX1",
        index: 277,
    },
    Resource {
        name: "WDG0",
        index: 278,
    },
    Resource {
        name: "WDG1",
        index: 279,
    },
    Resource {
        name: "WDG2",
        index: 280,
    },
    Resource {
        name: "WDG3",
        index: 281,
    },
    Resource {
        name: "GPTMR0",
        index: 282,
    },
    Resource {
        name: "GPTMR1",
        index: 283,
    },
    Resource {
        name: "GPTMR2",
        index: 284,
    },
    Resource {
        name: "GPTMR3",
        index: 285,
    },
    Resource {
        name: "GPTMR4",
        index: 286,
    },
    Resource {
        name: "GPTMR5",
        index: 287,
    },
    Resource {
        name: "GPTMR6",
        index: 288,
    },
    Resource {
        name: "GPTMR7",
        index: 289,
    },
    Resource {
        name: "UART0",
        index: 290,
    },
    Resource {
        name: "UART1",
        index: 291,
    },
    Resource {
        name: "UART2",
        index: 292,
    },
    Resource {
        name: "UART3",
        index: 293,
    },
    Resource {
        name: "UART4",
        index: 294,
    },
    Resource {
        name: "UART5",
        index: 295,
    },
    Resource {
        name: "UART6",
        index: 296,
    },
    Resource {
        name: "UART7",
        index: 297,
    },
    Resource {
        name: "UART8",
        index: 298,
    },
    Resource {
        name: "UART9",
        index: 299,
    },
    Resource {
        name: "UART10",
        index: 300,
    },
    Resource {
        name: "UART11",
        index: 301,
    },
    Resource {
        name: "UART12",
        index: 302,
    },
    Resource {
        name: "UART13",
        index: 303,
    },
    Resource {
        name: "UART14",
        index: 304,
    },
    Resource {
        name: "UART15",
        index: 305,
    },
    Resource {
        name: "I2C0",
        index: 306,
    },
    Resource {
        name: "I2C1",
        index: 307,
    },
    Resource {
        name: "I2C2",
        index: 308,
    },
    Resource {
        name: "I2C3",
        index: 309,
    },
    Resource {
        name: "SPI0",
        index: 310,
    },
    Resource {
        name: "SPI1",
        index: 311,
    },
    Resource {
        name: "SPI2",
        index: 312,
    },
    Resource {
        name: "SPI3",
        index: 313,
    },
    Resource {
        name: "CAN0",
        index: 314,
    },
    Resource {
        name: "CAN1",
        index: 315,
    },
    Resource {
        name: "CAN2",
        index: 316,
    },
    Resource {
        name: "CAN3",
        index: 317,
    },
    Resource {
        name: "PTPC",
        index: 318,
    },
    Resource {
        name: "ADC0",
        index: 319,
    },
    Resource {
        name: "ADC1",
        index: 320,
    },
    Resource {
        name: "ADC2",
        index: 321,
    },
    Resource {
        name: "ADC3",
        index: 322,
    },
    Resource {
        name: "ACMP",
        index: 323,
    },
    Resource {
        name: "I2S0",
        index: 324,
    },
    Resource {
        name: "I2S1",
        index: 325,
    },
    Resource {
        name: "I2S2",
        index: 326,
    },
    Resource {
        name: "I2S3",
        index: 327,
    },
    Resource {
        name: "PDM",
        index: 328,
    },
    Resource {
        name: "DAO",
        index: 329,
    },
    Resource {
        name: "SYNT",
        index: 330,
    },
    Resource {
        name: "MOT0",
        index: 331,
    },
    Resource {
        name: "MOT1",
        index: 332,
    },
    Resource {
        name: "MOT2",
        index: 333,
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
        name: "CAM0",
        index: 336,
    },
    Resource {
        name: "CAM1",
        index: 337,
    },
    Resource {
        name: "JPEG",
        index: 338,
    },
    Resource {
        name: "PDMA",
        index: 339,
    },
    Resource {
        name: "ENET0",
        index: 340,
    },
    Resource {
        name: "ENET1",
        index: 341,
    },
    Resource {
        name: "NTMR0",
        index: 342,
    },
    Resource {
        name: "NTMR1",
        index: 343,
    },
    Resource {
        name: "SDXC0",
        index: 344,
    },
    Resource {
        name: "SDXC1",
        index: 345,
    },
    Resource {
        name: "USB0",
        index: 346,
    },
    Resource {
        name: "USB1",
        index: 347,
    },
    Resource {
        name: "REF0",
        index: 348,
    },
    Resource {
        name: "REF1",
        index: 349,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "CPU0",
        index: 0,
    },
    Clock {
        name: "MCHTMR0",
        index: 1,
    },
    Clock {
        name: "CPU1",
        index: 2,
    },
    Clock {
        name: "MCHTMR1",
        index: 3,
    },
    Clock {
        name: "AXI",
        index: 4,
    },
    Clock {
        name: "CONN",
        index: 5,
    },
    Clock {
        name: "VIS",
        index: 6,
    },
    Clock {
        name: "AHB",
        index: 7,
    },
    Clock {
        name: "FEMC",
        index: 8,
    },
    Clock {
        name: "XPI0",
        index: 9,
    },
    Clock {
        name: "XPI1",
        index: 10,
    },
    Clock {
        name: "GPTMR0",
        index: 11,
    },
    Clock {
        name: "GPTMR1",
        index: 12,
    },
    Clock {
        name: "GPTMR2",
        index: 13,
    },
    Clock {
        name: "GPTMR3",
        index: 14,
    },
    Clock {
        name: "GPTMR4",
        index: 15,
    },
    Clock {
        name: "GPTMR5",
        index: 16,
    },
    Clock {
        name: "GPTMR6",
        index: 17,
    },
    Clock {
        name: "GPTMR7",
        index: 18,
    },
    Clock {
        name: "UART0",
        index: 19,
    },
    Clock {
        name: "UART1",
        index: 20,
    },
    Clock {
        name: "UART2",
        index: 21,
    },
    Clock {
        name: "UART3",
        index: 22,
    },
    Clock {
        name: "UART4",
        index: 23,
    },
    Clock {
        name: "UART5",
        index: 24,
    },
    Clock {
        name: "UART6",
        index: 25,
    },
    Clock {
        name: "UART7",
        index: 26,
    },
    Clock {
        name: "UART8",
        index: 27,
    },
    Clock {
        name: "UART9",
        index: 28,
    },
    Clock {
        name: "UART10",
        index: 29,
    },
    Clock {
        name: "UART11",
        index: 30,
    },
    Clock {
        name: "UART12",
        index: 31,
    },
    Clock {
        name: "UART13",
        index: 32,
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
        name: "I2C0",
        index: 35,
    },
    Clock {
        name: "I2C1",
        index: 36,
    },
    Clock {
        name: "I2C2",
        index: 37,
    },
    Clock {
        name: "I2C3",
        index: 38,
    },
    Clock {
        name: "SPI0",
        index: 39,
    },
    Clock {
        name: "SPI1",
        index: 40,
    },
    Clock {
        name: "SPI2",
        index: 41,
    },
    Clock {
        name: "SPI3",
        index: 42,
    },
    Clock {
        name: "CAN0",
        index: 43,
    },
    Clock {
        name: "CAN1",
        index: 44,
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
        name: "PTPC",
        index: 47,
    },
    Clock {
        name: "ANA0",
        index: 48,
    },
    Clock {
        name: "ANA1",
        index: 49,
    },
    Clock {
        name: "ANA2",
        index: 50,
    },
    Clock {
        name: "AUD0",
        index: 51,
    },
    Clock {
        name: "AUD1",
        index: 52,
    },
    Clock {
        name: "AUD2",
        index: 53,
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
        name: "CAM1",
        index: 56,
    },
    Clock {
        name: "ENET0",
        index: 57,
    },
    Clock {
        name: "ENET1",
        index: 58,
    },
    Clock {
        name: "PTP0",
        index: 59,
    },
    Clock {
        name: "PTP1",
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
        name: "NTMR0",
        index: 63,
    },
    Clock {
        name: "NTMR1",
        index: 64,
    },
    Clock {
        name: "SDXC0",
        index: 65,
    },
    Clock {
        name: "SDXC1",
        index: 66,
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
pub(crate) static TRGMMUX: &[TrgmMux] = &[
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP12",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP13",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP14",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP15",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP16",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP17",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP18",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP19",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP20",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP21",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP22",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_CMP23",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_RLD",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_HALFRLD",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_XRLD",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_QEI0",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_HALL0",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_IN7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN8",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN9",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN10",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_IN11",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_VSS",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_VDD",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P0",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P1",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P2",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P3",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P4",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P5",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P6",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P7",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P8",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P9",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P10",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P11",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM3_OUTX0",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM3_OUTX1",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM2_OUTX0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM2_OUTX1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM1_OUTX0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM1_OUTX1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH8REF",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH9REF",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH10REF",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH11REF",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH12REF",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH13REF",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH14REF",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH15REF",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH16REF",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH17REF",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH18REF",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH19REF",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH20REF",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH21REF",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH22REF",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH23REF",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI0_TRGO",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_HALL0_TRGO",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_USB0_SOF",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_USB1_SOF",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ENET1_PTP_OUT3",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PTPC_CMP0",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PTPC_CMP1",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH0",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH1",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH2",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH3",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT2",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT3",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT2",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT3",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP0_OUT",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP1_OUT",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP2_OUT",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP3_OUT",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_DEBUG_FLAG",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_OUTX0",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_OUTX1",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_SYNCI",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FRCI",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FRCSYNCI",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_SHRLDSYNCI",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI2",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI3",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN8",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN9",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN10",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN11",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN12",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN13",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN14",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN15",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN16",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN17",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN18",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN19",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN20",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN21",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN22",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_IN23",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_A",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_B",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_Z",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_H",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_PAUSE",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_SNAPI",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_HALL0_U",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_HALL0_V",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_HALL0_W",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_HALL0_SNAPI",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC0_STRGI",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC1_STRGI",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC2_STRGI",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC3_STRGI",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0A",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0B",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0C",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_SYNCI",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_IN2",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_IN3",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_SYNCI",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_IN2",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_IN3",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ACMP0_WIN",
        value: 61,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PTPC_CAP0",
        value: 62,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PTPC_CAP1",
        value: 63,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP12",
        value: 12,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP13",
        value: 13,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP14",
        value: 14,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP15",
        value: 15,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP16",
        value: 16,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP17",
        value: 17,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP18",
        value: 18,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP19",
        value: 19,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP20",
        value: 20,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP21",
        value: 21,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP22",
        value: 22,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_CMP23",
        value: 23,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_RLD",
        value: 24,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_HALFRLD",
        value: 25,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_PWM1_XRLD",
        value: 26,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_QEI1",
        value: 27,
    },
    TrgmMux {
        name: "TRGM1_DMA_SRC_HALL1",
        value: 28,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_PWM1_IN7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN8",
        value: 16,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN9",
        value: 17,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN10",
        value: 18,
    },
    TrgmMux {
        name: "TRGM1_FILTER_SRC_TRGM1_IN11",
        value: 19,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_VSS",
        value: 0,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_VDD",
        value: 1,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P0",
        value: 2,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P1",
        value: 3,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P2",
        value: 4,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P3",
        value: 5,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P4",
        value: 6,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P5",
        value: 7,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P6",
        value: 8,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P7",
        value: 9,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P8",
        value: 10,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P9",
        value: 11,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P10",
        value: 12,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM1_P11",
        value: 13,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM3_OUTX0",
        value: 14,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM3_OUTX1",
        value: 15,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM2_OUTX0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM2_OUTX1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM0_OUTX0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_TRGM0_OUTX1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH8REF",
        value: 20,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH9REF",
        value: 21,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH10REF",
        value: 22,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH11REF",
        value: 23,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH12REF",
        value: 24,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH13REF",
        value: 25,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH14REF",
        value: 26,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH15REF",
        value: 27,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH16REF",
        value: 28,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH17REF",
        value: 29,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH18REF",
        value: 30,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH19REF",
        value: 31,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH20REF",
        value: 32,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH21REF",
        value: 33,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH22REF",
        value: 34,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PWM1_CH23REF",
        value: 35,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_QEI1_TRGO",
        value: 36,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_HALL1_TRGO",
        value: 37,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_USB0_SOF",
        value: 38,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_USB1_SOF",
        value: 39,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_ENET1_PTP_OUT3",
        value: 41,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PTPC_CMP0",
        value: 42,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_PTPC_CMP1",
        value: 43,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT_CH0",
        value: 44,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT_CH1",
        value: 45,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT_CH2",
        value: 46,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT_CH3",
        value: 47,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_GPTMR2_OUT2",
        value: 48,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_GPTMR2_OUT3",
        value: 49,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_GPTMR3_OUT2",
        value: 50,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_GPTMR3_OUT3",
        value: 51,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_CMP0_OUT",
        value: 52,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_CMP1_OUT",
        value: 53,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_CMP2_OUT",
        value: 54,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_CMP3_OUT",
        value: 55,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_DEBUG_FLAG",
        value: 56,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_P11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_OUTX0",
        value: 12,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_TRGM1_OUTX1",
        value: 13,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_SYNCI",
        value: 14,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FRCI",
        value: 15,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FRCSYNCI",
        value: 16,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_SHRLDSYNCI",
        value: 17,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FAULTI0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FAULTI1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FAULTI2",
        value: 20,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_FAULTI3",
        value: 21,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN8",
        value: 22,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN9",
        value: 23,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN10",
        value: 24,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN11",
        value: 25,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN12",
        value: 26,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN13",
        value: 27,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN14",
        value: 28,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN15",
        value: 29,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN16",
        value: 30,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN17",
        value: 31,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN18",
        value: 32,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN19",
        value: 33,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN20",
        value: 34,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN21",
        value: 35,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN22",
        value: 36,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PWM1_IN23",
        value: 37,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_A",
        value: 38,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_B",
        value: 39,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_Z",
        value: 40,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_H",
        value: 41,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_PAUSE",
        value: 42,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_QEI1_SNAPI",
        value: 43,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_HALL1_U",
        value: 44,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_HALL1_V",
        value: 45,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_HALL1_W",
        value: 46,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_HALL1_SNAPI",
        value: 47,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC0_STRGI",
        value: 48,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC1_STRGI",
        value: 49,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC2_STRGI",
        value: 50,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC3_STRGI",
        value: 51,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADCX_PTRGI1A",
        value: 52,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADCX_PTRGI1B",
        value: 53,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADCX_PTRGI1C",
        value: 54,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR2_SYNCI",
        value: 55,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR2_IN2",
        value: 56,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR2_IN3",
        value: 57,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR3_SYNCI",
        value: 58,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR3_IN2",
        value: 59,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_GPTMR3_IN3",
        value: 60,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ACMP1_WIN",
        value: 61,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PTPC_CAP0",
        value: 62,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_PTPC_CAP1",
        value: 63,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP12",
        value: 12,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP13",
        value: 13,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP14",
        value: 14,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP15",
        value: 15,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP16",
        value: 16,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP17",
        value: 17,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP18",
        value: 18,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP19",
        value: 19,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP20",
        value: 20,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP21",
        value: 21,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP22",
        value: 22,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_CMP23",
        value: 23,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_RLD",
        value: 24,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_HALFRLD",
        value: 25,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_PWM2_XRLD",
        value: 26,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_QEI2",
        value: 27,
    },
    TrgmMux {
        name: "TRGM2_DMA_SRC_HALL2",
        value: 28,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_PWM2_IN7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN8",
        value: 16,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN9",
        value: 17,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN10",
        value: 18,
    },
    TrgmMux {
        name: "TRGM2_FILTER_SRC_TRGM2_IN11",
        value: 19,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_VSS",
        value: 0,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_VDD",
        value: 1,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P0",
        value: 2,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P1",
        value: 3,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P2",
        value: 4,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P3",
        value: 5,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P4",
        value: 6,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P5",
        value: 7,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P6",
        value: 8,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P7",
        value: 9,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P8",
        value: 10,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P9",
        value: 11,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P10",
        value: 12,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM2_P11",
        value: 13,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM3_OUTX0",
        value: 14,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM3_OUTX1",
        value: 15,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM1_OUTX0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM1_OUTX1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM0_OUTX0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_TRGM0_OUTX1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH8REF",
        value: 20,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH9REF",
        value: 21,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH10REF",
        value: 22,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH11REF",
        value: 23,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH12REF",
        value: 24,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH13REF",
        value: 25,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH14REF",
        value: 26,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH15REF",
        value: 27,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH16REF",
        value: 28,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH17REF",
        value: 29,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH18REF",
        value: 30,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH19REF",
        value: 31,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH20REF",
        value: 32,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH21REF",
        value: 33,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH22REF",
        value: 34,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PWM2_CH23REF",
        value: 35,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_QEI2_TRGO",
        value: 36,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_HALL2_TRGO",
        value: 37,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_USB0_SOF",
        value: 38,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_USB1_SOF",
        value: 39,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_ENET1_PTP_OUT3",
        value: 41,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PTPC_CMP0",
        value: 42,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_PTPC_CMP1",
        value: 43,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_SYNT_CH0",
        value: 44,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_SYNT_CH1",
        value: 45,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_SYNT_CH2",
        value: 46,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_SYNT_CH3",
        value: 47,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_GPTMR4_OUT2",
        value: 48,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_GPTMR4_OUT3",
        value: 49,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_GPTMR5_OUT2",
        value: 50,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_GPTMR5_OUT3",
        value: 51,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_CMP0_OUT",
        value: 52,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_CMP1_OUT",
        value: 53,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_CMP2_OUT",
        value: 54,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_CMP3_OUT",
        value: 55,
    },
    TrgmMux {
        name: "TRGM2_INPUT_SRC_DEBUG_FLAG",
        value: 56,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_P11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_OUTX0",
        value: 12,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_TRGM2_OUTX1",
        value: 13,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_SYNCI",
        value: 14,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FRCI",
        value: 15,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FRCSYNCI",
        value: 16,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_SHRLDSYNCI",
        value: 17,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FAULTI0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FAULTI1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FAULTI2",
        value: 20,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_FAULTI3",
        value: 21,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN8",
        value: 22,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN9",
        value: 23,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN10",
        value: 24,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN11",
        value: 25,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN12",
        value: 26,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN13",
        value: 27,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN14",
        value: 28,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN15",
        value: 29,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN16",
        value: 30,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN17",
        value: 31,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN18",
        value: 32,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN19",
        value: 33,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN20",
        value: 34,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN21",
        value: 35,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN22",
        value: 36,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PWM2_IN23",
        value: 37,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_A",
        value: 38,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_B",
        value: 39,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_Z",
        value: 40,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_H",
        value: 41,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_PAUSE",
        value: 42,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_QEI2_SNAPI",
        value: 43,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_HALL2_U",
        value: 44,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_HALL2_V",
        value: 45,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_HALL2_W",
        value: 46,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_HALL2_SNAPI",
        value: 47,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADC0_STRGI",
        value: 48,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADC1_STRGI",
        value: 49,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADC2_STRGI",
        value: 50,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADC3_STRGI",
        value: 51,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADCX_PTRGI2A",
        value: 52,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADCX_PTRGI2B",
        value: 53,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ADCX_PTRGI2C",
        value: 54,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR4_SYNCI",
        value: 55,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR4_IN2",
        value: 56,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR4_IN3",
        value: 57,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR5_SYNCI",
        value: 58,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR5_IN2",
        value: 59,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_GPTMR5_IN3",
        value: 60,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_ACMP2_WIN",
        value: 61,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PTPC_CAP0",
        value: 62,
    },
    TrgmMux {
        name: "TRGM2_OUTPUT_SRC_PTPC_CAP1",
        value: 63,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP12",
        value: 12,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP13",
        value: 13,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP14",
        value: 14,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP15",
        value: 15,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP16",
        value: 16,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP17",
        value: 17,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP18",
        value: 18,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP19",
        value: 19,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP20",
        value: 20,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP21",
        value: 21,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP22",
        value: 22,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_CMP23",
        value: 23,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_RLD",
        value: 24,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_HALFRLD",
        value: 25,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_PWM3_XRLD",
        value: 26,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_QEI3",
        value: 27,
    },
    TrgmMux {
        name: "TRGM3_DMA_SRC_HALL3",
        value: 28,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_PWM3_IN7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN8",
        value: 16,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN9",
        value: 17,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN10",
        value: 18,
    },
    TrgmMux {
        name: "TRGM3_FILTER_SRC_TRGM3_IN11",
        value: 19,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_VSS",
        value: 0,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_VDD",
        value: 1,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P0",
        value: 2,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P1",
        value: 3,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P2",
        value: 4,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P3",
        value: 5,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P4",
        value: 6,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P5",
        value: 7,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P6",
        value: 8,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P7",
        value: 9,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P8",
        value: 10,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P9",
        value: 11,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P10",
        value: 12,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM3_P11",
        value: 13,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM2_OUTX0",
        value: 14,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM2_OUTX1",
        value: 15,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM1_OUTX0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM1_OUTX1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM0_OUTX0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_TRGM0_OUTX1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH8REF",
        value: 20,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH9REF",
        value: 21,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH10REF",
        value: 22,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH11REF",
        value: 23,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH12REF",
        value: 24,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH13REF",
        value: 25,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH14REF",
        value: 26,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH15REF",
        value: 27,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH16REF",
        value: 28,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH17REF",
        value: 29,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH18REF",
        value: 30,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH19REF",
        value: 31,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH20REF",
        value: 32,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH21REF",
        value: 33,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH22REF",
        value: 34,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PWM3_CH23REF",
        value: 35,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_QEI3_TRGO",
        value: 36,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_HALL3_TRGO",
        value: 37,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_USB0_SOF",
        value: 38,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_USB1_SOF",
        value: 39,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_ENET1_PTP_OUT3",
        value: 41,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PTPC_CMP0",
        value: 42,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_PTPC_CMP1",
        value: 43,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_SYNT_CH0",
        value: 44,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_SYNT_CH1",
        value: 45,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_SYNT_CH2",
        value: 46,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_SYNT_CH3",
        value: 47,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_GPTMR6_OUT2",
        value: 48,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_GPTMR6_OUT3",
        value: 49,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_GPTMR7_OUT2",
        value: 50,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_GPTMR7_OUT3",
        value: 51,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_CMP0_OUT",
        value: 52,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_CMP1_OUT",
        value: 53,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_CMP2_OUT",
        value: 54,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_CMP3_OUT",
        value: 55,
    },
    TrgmMux {
        name: "TRGM3_INPUT_SRC_DEBUG_FLAG",
        value: 56,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P8",
        value: 8,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P9",
        value: 9,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P10",
        value: 10,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_P11",
        value: 11,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_OUTX0",
        value: 12,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_TRGM3_OUTX1",
        value: 13,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_SYNCI",
        value: 14,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FRCI",
        value: 15,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FRCSYNCI",
        value: 16,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_SHRLDSYNCI",
        value: 17,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FAULTI0",
        value: 18,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FAULTI1",
        value: 19,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FAULTI2",
        value: 20,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_FAULTI3",
        value: 21,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN8",
        value: 22,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN9",
        value: 23,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN10",
        value: 24,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN11",
        value: 25,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN12",
        value: 26,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN13",
        value: 27,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN14",
        value: 28,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN15",
        value: 29,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN16",
        value: 30,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN17",
        value: 31,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN18",
        value: 32,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN19",
        value: 33,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN20",
        value: 34,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN21",
        value: 35,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN22",
        value: 36,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PWM3_IN23",
        value: 37,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_A",
        value: 38,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_B",
        value: 39,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_Z",
        value: 40,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_H",
        value: 41,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_PAUSE",
        value: 42,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_QEI3_SNAPI",
        value: 43,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_HALL3_U",
        value: 44,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_HALL3_V",
        value: 45,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_HALL3_W",
        value: 46,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_HALL3_SNAPI",
        value: 47,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADC0_STRGI",
        value: 48,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADC1_STRGI",
        value: 49,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADC2_STRGI",
        value: 50,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADC3_STRGI",
        value: 51,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADCX_PTRGI3A",
        value: 52,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADCX_PTRGI3B",
        value: 53,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ADCX_PTRGI3C",
        value: 54,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR6_SYNCI",
        value: 55,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR6_IN2",
        value: 56,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR6_IN3",
        value: 57,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR7_SYNCI",
        value: 58,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR7_IN2",
        value: 59,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_GPTMR7_IN3",
        value: 60,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_ACMP3_WIN",
        value: 61,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PTPC_CAP0",
        value: 62,
    },
    TrgmMux {
        name: "TRGM3_OUTPUT_SRC_PTPC_CAP1",
        value: 63,
    },
];
#[path = "../registers/acmp_common.rs"]
pub mod acmp;
#[path = "../registers/adc12_v67.rs"]
pub mod adc12;
#[path = "../registers/adc16_v67.rs"]
pub mod adc16;
#[path = "../registers/bcfg_v67.rs"]
pub mod bcfg;
#[path = "../registers/bkey_common.rs"]
pub mod bkey;
#[path = "../registers/bmon_common.rs"]
pub mod bmon;
#[path = "../registers/bpor_v67.rs"]
pub mod bpor;
#[path = "../registers/bsec_common.rs"]
pub mod bsec;
#[path = "../registers/butn_common.rs"]
pub mod butn;
#[path = "../registers/cam_v67.rs"]
pub mod cam;
#[path = "../registers/can_v67.rs"]
pub mod can;
#[path = "../registers/conctl_v67.rs"]
pub mod conctl;
#[path = "../registers/dao_v67.rs"]
pub mod dao;
#[path = "../registers/dma_v67.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/enet_v67.rs"]
pub mod enet;
#[path = "../registers/femc_common.rs"]
pub mod femc;
#[path = "../registers/gpio_common.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v67.rs"]
pub mod gpiom;
#[path = "../registers/hall_common.rs"]
pub mod hall;
#[path = "../registers/i2c_v67.rs"]
pub mod i2c;
#[path = "../registers/i2s_common.rs"]
pub mod i2s;
#[path = "../registers/ioc_v67.rs"]
pub mod ioc;
#[path = "../registers/jpeg_common.rs"]
pub mod jpeg;
#[path = "../registers/keym_common.rs"]
pub mod keym;
#[path = "../registers/lcdc_v67.rs"]
pub mod lcdc;
#[path = "../registers/mbx_common.rs"]
pub mod mbx;
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/mono_common.rs"]
pub mod mono;
#[path = "../registers/otp_common.rs"]
pub mod otp;
#[path = "../registers/pcfg_v67.rs"]
pub mod pcfg;
#[path = "../registers/pdm_common.rs"]
pub mod pdm;
#[path = "../registers/pdma_v67.rs"]
pub mod pdma;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v67.rs"]
pub mod pllctl;
#[path = "../registers/pmon_common.rs"]
pub mod pmon;
#[path = "../registers/ppor_v67.rs"]
pub mod ppor;
#[path = "../registers/psec_common.rs"]
pub mod psec;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwm_v67.rs"]
pub mod pwm;
#[path = "../registers/qei_v67.rs"]
pub mod qei;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdp_v67.rs"]
pub mod sdp;
#[path = "../registers/sdxc_v67.rs"]
pub mod sdxc;
#[path = "../registers/spi_v67.rs"]
pub mod spi;
#[path = "../registers/synt_v67.rs"]
pub mod synt;
#[path = "../registers/sysctl_v67.rs"]
pub mod sysctl;
#[path = "../registers/tamp_v67.rs"]
pub mod tamp;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/trgm_v67.rs"]
pub mod trgm;
#[path = "../registers/uart_v67.rs"]
pub mod uart;
#[path = "../registers/usb_v67.rs"]
pub mod usb;
#[path = "../registers/vad_common.rs"]
pub mod vad;
#[path = "../registers/wdg_v67.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
