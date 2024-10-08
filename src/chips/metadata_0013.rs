
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
            version: "v63",
            block: "SYSCTL",
            ir: &sysctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SYSCTL",
            dmamux: Some("DMAMUX"),
            request: Some(76),
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
            group_link: 0,
            group_bit_offset: 8,
            resource_clock_top: Some(67),
            resource: 264,
            clock_node: Some(2),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XPI0",
            dmamux: Some("DMAMUX"),
            request: Some(49),
        }],
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
            group_bit_offset: 9,
            resource_clock_top: Some(68),
            resource: 265,
            clock_node: Some(3),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XPI1",
            dmamux: Some("DMAMUX"),
            request: Some(50),
        }],
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
        address: 0xf5008000,
        registers: Some(PeripheralRegisters {
            kind: "bcfg",
            version: "v62",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "BUTN",
            dmamux: Some("DMAMUX"),
            request: Some(72),
        }],
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
        address: 0xf40d8000,
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
        address: 0xf5010000,
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "Y",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "D",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "X",
                dmamux: Some("DMAMUX"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "A",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "Z",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "C",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "B",
                dmamux: Some("DMAMUX"),
                request: Some(2),
            },
        ],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PGPIO",
            dmamux: Some("DMAMUX"),
            request: Some(65),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "BGPIO",
            dmamux: Some("DMAMUX"),
            request: Some(73),
        }],
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
            version: "v63",
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
            group_bit_offset: 3,
            resource_clock_top: Some(66),
            resource: 259,
            clock_node: Some(1),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB18",
                signal: "A00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "A01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "A02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "A03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "A04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "A05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "A06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "A07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "A08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "A09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "A10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "A11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "A12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "BA0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "BA1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "CKE",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "CLK",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CS0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CS1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "DM0",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DM1",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "DQ00",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "DQ01",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "DQ02",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "DQ03",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "DQ04",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "DQ05",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DQ06",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "DQ07",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "DQ08",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "DQ09",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "DQ10",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "DQ11",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "DQ12",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "DQ13",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "DQ14",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "DQ15",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "DQS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RAS",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SCLK",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "SRDY",
                alt: Some(12),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "WE",
                alt: Some(12),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "FEMC",
            dmamux: Some("DMAMUX"),
            request: Some(53),
        }],
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
            version: "v63",
            block: "SDXC",
            ir: &sdxc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 26,
            resource_clock_top: Some(103),
            resource: 314,
            clock_node: Some(38),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDXC0",
            dmamux: Some("DMAMUX"),
            request: Some(63),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDXC0",
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
            group_bit_offset: 13,
            resource_clock_top: None,
            resource: 269,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "HDMA",
            dmamux: Some("DMAMUX"),
            request: Some(52),
        }],
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
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 270,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XDMA",
            dmamux: Some("DMAMUX"),
            request: Some(51),
        }],
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
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 273,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX0A",
            dmamux: Some("DMAMUX"),
            request: Some(32),
        }],
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
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 273,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX0B",
            dmamux: Some("DMAMUX"),
            request: Some(33),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX0B",
        }],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0200000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB11",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "P7",
                alt: Some(16),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM0",
            dmamux: Some("DMAMUX"),
            request: Some(42),
        }],
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
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB08",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P7",
                alt: Some(16),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM1",
            dmamux: Some("DMAMUX"),
            request: Some(45),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM1",
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
                pin: "PB20",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB31",
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
                pin: "PA20",
                signal: "P00",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P01",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "P02",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P03",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P04",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P05",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P06",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P07",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P08",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P09",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P10",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA31",
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
        name: "NTMR0",
        address: 0xf2010000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 25,
            resource_clock_top: Some(102),
            resource: 313,
            clock_node: Some(37),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "NTMR0",
            dmamux: Some("DMAMUX"),
            request: Some(60),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "NTMR0",
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
            group_bit_offset: 21,
            resource_clock_top: Some(69),
            resource: 277,
            clock_node: Some(4),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR0",
            dmamux: Some("DMAMUX"),
            request: Some(34),
        }],
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
            group_bit_offset: 22,
            resource_clock_top: Some(70),
            resource: 278,
            clock_node: Some(5),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR1",
            dmamux: Some("DMAMUX"),
            request: Some(35),
        }],
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
            group_bit_offset: 23,
            resource_clock_top: Some(71),
            resource: 279,
            clock_node: Some(6),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB18",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR2",
            dmamux: Some("DMAMUX"),
            request: Some(36),
        }],
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
            group_bit_offset: 24,
            resource_clock_top: Some(72),
            resource: 280,
            clock_node: Some(7),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR3",
            dmamux: Some("DMAMUX"),
            request: Some(37),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR3",
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
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CAPT1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CAPT2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "CAPT3",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "COMP3",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTMR",
            dmamux: Some("DMAMUX"),
            request: Some(67),
        }],
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
            group_bit_offset: 18,
            resource_clock_top: None,
            resource: 274,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "WDG0",
            dmamux: Some("DMAMUX"),
            request: Some(29),
        }],
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
            group_bit_offset: 19,
            resource_clock_top: None,
            resource: 275,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "WDG1",
            dmamux: Some("DMAMUX"),
            request: Some(30),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WDG1",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWDG",
            dmamux: Some("DMAMUX"),
            request: Some(66),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWDG",
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "RTC",
            dmamux: Some("DMAMUX"),
            request: Some(71),
        }],
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
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: Some(73),
            resource: 281,
            clock_node: Some(8),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART0",
            dmamux: Some("DMAMUX"),
            request: Some(18),
        }],
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
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: Some(74),
            resource: 282,
            clock_node: Some(9),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA29",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART1",
            dmamux: Some("DMAMUX"),
            request: Some(19),
        }],
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
            group_link: 0,
            group_bit_offset: 27,
            resource_clock_top: Some(75),
            resource: 283,
            clock_node: Some(10),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA07",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART2",
            dmamux: Some("DMAMUX"),
            request: Some(20),
        }],
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
            group_link: 0,
            group_bit_offset: 28,
            resource_clock_top: Some(76),
            resource: 284,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA09",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART3",
            dmamux: Some("DMAMUX"),
            request: Some(21),
        }],
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
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: Some(77),
            resource: 285,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART4",
            dmamux: Some("DMAMUX"),
            request: Some(22),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PUART",
            dmamux: Some("DMAMUX"),
            request: Some(68),
        }],
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
            group_bit_offset: 5,
            resource_clock_top: Some(85),
            resource: 293,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI0",
            dmamux: Some("DMAMUX"),
            request: Some(14),
        }],
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
            group_bit_offset: 6,
            resource_clock_top: Some(86),
            resource: 294,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA16",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI1",
            dmamux: Some("DMAMUX"),
            request: Some(15),
        }],
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
            group_bit_offset: 7,
            resource_clock_top: Some(87),
            resource: 295,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI2",
            dmamux: Some("DMAMUX"),
            request: Some(16),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
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
            group_bit_offset: 1,
            resource_clock_top: Some(81),
            resource: 289,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC14",
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
            request: Some(38),
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
            group_bit_offset: 2,
            resource_clock_top: Some(82),
            resource: 290,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(39),
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
            group_bit_offset: 3,
            resource_clock_top: Some(83),
            resource: 291,
            clock_node: Some(18),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SDA",
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
            request: Some(40),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C2",
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
            group_bit_offset: 11,
            resource_clock_top: Some(91),
            resource: 299,
            clock_node: Some(26),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTPC",
            dmamux: Some("DMAMUX"),
            request: Some(28),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTPC",
        }],
    },
    Peripheral {
        name: "ADC0",
        address: 0xf0010000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v63",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 12,
            resource_clock_top: Some(128),
            resource: 300,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC04",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN9",
                alt: Some(0),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC0",
            dmamux: Some("DMAMUX"),
            request: Some(8),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC0",
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
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: None,
            resource: 304,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC26",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "CMP1_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "CMP1_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CMP1_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CMP1_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "COMP1",
                alt: Some(16),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(12),
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
        ],
    },
    Peripheral {
        name: "TSNS",
        address: 0xf4104000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 20,
            resource_clock_top: None,
            resource: 276,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "TSNS",
            dmamux: Some("DMAMUX"),
            request: Some(31),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSNS",
        }],
    },
    Peripheral {
        name: "SDP",
        address: 0xf304c000,
        registers: Some(PeripheralRegisters {
            kind: "sdp",
            version: "v53",
            block: "SDP",
            ir: &sdp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 10,
            resource_clock_top: None,
            resource: 266,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDP",
            dmamux: Some("DMAMUX"),
            request: Some(48),
        }],
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
            group_bit_offset: 11,
            resource_clock_top: None,
            resource: 267,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "RNG",
            dmamux: Some("DMAMUX"),
            request: Some(54),
        }],
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
            group_bit_offset: 12,
            resource_clock_top: None,
            resource: 268,
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PSEC",
            dmamux: Some("DMAMUX"),
            request: Some(64),
        }],
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
    Peripheral {
        name: "FFA",
        address: 0xf3058000,
        registers: Some(PeripheralRegisters {
            kind: "ffa",
            version: "common",
            block: "FFA",
            ir: &ffa::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 271,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "FFA",
            dmamux: Some("DMAMUX"),
            request: Some(59),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FFA",
        }],
    },
    Peripheral {
        name: "ENET0",
        address: 0xf2000000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v63",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ENET0",
            dmamux: Some("DMAMUX"),
            request: Some(62),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET0",
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
            group_link: 1,
            group_bit_offset: 27,
            resource_clock_top: None,
            resource: 315,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC00",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "PWR",
                alt: Some(24),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "USB0",
            dmamux: Some("DMAMUX"),
            request: Some(61),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB0",
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
            group_bit_offset: 9,
            resource_clock_top: Some(89),
            resource: 297,
            clock_node: Some(24),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CAN0",
            dmamux: Some("DMAMUX"),
            request: Some(26),
        }],
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
            group_bit_offset: 10,
            resource_clock_top: Some(90),
            resource: 298,
            clock_node: Some(25),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CAN1",
            dmamux: Some("DMAMUX"),
            request: Some(27),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "CAN1",
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
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: Some(78),
            resource: 286,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART5",
            dmamux: Some("DMAMUX"),
            request: Some(23),
        }],
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
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: Some(79),
            resource: 287,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART6",
            dmamux: Some("DMAMUX"),
            request: Some(24),
        }],
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
            group_bit_offset: 0,
            resource_clock_top: Some(80),
            resource: 288,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA25",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART7",
            dmamux: Some("DMAMUX"),
            request: Some(25),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
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
            group_bit_offset: 8,
            resource_clock_top: Some(88),
            resource: 296,
            clock_node: Some(23),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI3",
            dmamux: Some("DMAMUX"),
            request: Some(17),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
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
            group_bit_offset: 4,
            resource_clock_top: Some(84),
            resource: 292,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(41),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C3",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "QEI0",
            dmamux: Some("DMAMUX"),
            request: Some(44),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "QEI1",
            dmamux: Some("DMAMUX"),
            request: Some(47),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI1",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "HALL0",
            dmamux: Some("DMAMUX"),
            request: Some(43),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "HALL1",
            dmamux: Some("DMAMUX"),
            request: Some(46),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL1",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0xf0014000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v63",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 13,
            resource_clock_top: Some(129),
            resource: 301,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC08",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "IN9",
                alt: Some(0),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            dmamux: Some("DMAMUX"),
            request: Some(9),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 0xf0018000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v63",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 14,
            resource_clock_top: Some(130),
            resource: 302,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "IN9",
                alt: Some(0),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            dmamux: Some("DMAMUX"),
            request: Some(10),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC2",
        }],
    },
    Peripheral {
        name: "DAC0",
        address: 0xf0024000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v63",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: Some(131),
            resource: 303,
            clock_node: None,
        }),
        pins: &[PeripheralPin {
            pin: "PC03",
            signal: "OUT",
            alt: Some(0),
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC0",
            dmamux: Some("DMAMUX"),
            request: Some(11),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC0",
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
        name: "ADC0",
        number: 8,
    },
    Interrupt {
        name: "ADC1",
        number: 9,
    },
    Interrupt {
        name: "ADC2",
        number: 10,
    },
    Interrupt {
        name: "DAC0",
        number: 11,
    },
    Interrupt {
        name: "ACMP_0",
        number: 12,
    },
    Interrupt {
        name: "ACMP_1",
        number: 13,
    },
    Interrupt {
        name: "SPI0",
        number: 14,
    },
    Interrupt {
        name: "SPI1",
        number: 15,
    },
    Interrupt {
        name: "SPI2",
        number: 16,
    },
    Interrupt {
        name: "SPI3",
        number: 17,
    },
    Interrupt {
        name: "UART0",
        number: 18,
    },
    Interrupt {
        name: "UART1",
        number: 19,
    },
    Interrupt {
        name: "UART2",
        number: 20,
    },
    Interrupt {
        name: "UART3",
        number: 21,
    },
    Interrupt {
        name: "UART4",
        number: 22,
    },
    Interrupt {
        name: "UART5",
        number: 23,
    },
    Interrupt {
        name: "UART6",
        number: 24,
    },
    Interrupt {
        name: "UART7",
        number: 25,
    },
    Interrupt {
        name: "CAN0",
        number: 26,
    },
    Interrupt {
        name: "CAN1",
        number: 27,
    },
    Interrupt {
        name: "PTPC",
        number: 28,
    },
    Interrupt {
        name: "WDG0",
        number: 29,
    },
    Interrupt {
        name: "WDG1",
        number: 30,
    },
    Interrupt {
        name: "TSNS",
        number: 31,
    },
    Interrupt {
        name: "MBX0A",
        number: 32,
    },
    Interrupt {
        name: "MBX0B",
        number: 33,
    },
    Interrupt {
        name: "GPTMR0",
        number: 34,
    },
    Interrupt {
        name: "GPTMR1",
        number: 35,
    },
    Interrupt {
        name: "GPTMR2",
        number: 36,
    },
    Interrupt {
        name: "GPTMR3",
        number: 37,
    },
    Interrupt {
        name: "I2C0",
        number: 38,
    },
    Interrupt {
        name: "I2C1",
        number: 39,
    },
    Interrupt {
        name: "I2C2",
        number: 40,
    },
    Interrupt {
        name: "I2C3",
        number: 41,
    },
    Interrupt {
        name: "PWM0",
        number: 42,
    },
    Interrupt {
        name: "HALL0",
        number: 43,
    },
    Interrupt {
        name: "QEI0",
        number: 44,
    },
    Interrupt {
        name: "PWM1",
        number: 45,
    },
    Interrupt {
        name: "HALL1",
        number: 46,
    },
    Interrupt {
        name: "QEI1",
        number: 47,
    },
    Interrupt {
        name: "SDP",
        number: 48,
    },
    Interrupt {
        name: "XPI0",
        number: 49,
    },
    Interrupt {
        name: "XPI1",
        number: 50,
    },
    Interrupt {
        name: "XDMA",
        number: 51,
    },
    Interrupt {
        name: "HDMA",
        number: 52,
    },
    Interrupt {
        name: "FEMC",
        number: 53,
    },
    Interrupt {
        name: "RNG",
        number: 54,
    },
    Interrupt {
        name: "I2S0",
        number: 55,
    },
    Interrupt {
        name: "I2S1",
        number: 56,
    },
    Interrupt {
        name: "DAO",
        number: 57,
    },
    Interrupt {
        name: "PDM",
        number: 58,
    },
    Interrupt {
        name: "FFA",
        number: 59,
    },
    Interrupt {
        name: "NTMR0",
        number: 60,
    },
    Interrupt {
        name: "USB0",
        number: 61,
    },
    Interrupt {
        name: "ENET0",
        number: 62,
    },
    Interrupt {
        name: "SDXC0",
        number: 63,
    },
    Interrupt {
        name: "PSEC",
        number: 64,
    },
    Interrupt {
        name: "PGPIO",
        number: 65,
    },
    Interrupt {
        name: "PWDG",
        number: 66,
    },
    Interrupt {
        name: "PTMR",
        number: 67,
    },
    Interrupt {
        name: "PUART",
        number: 68,
    },
    Interrupt {
        name: "FUSE",
        number: 69,
    },
    Interrupt {
        name: "SECMON",
        number: 70,
    },
    Interrupt {
        name: "RTC",
        number: 71,
    },
    Interrupt {
        name: "BUTN",
        number: 72,
    },
    Interrupt {
        name: "BGPIO",
        number: 73,
    },
    Interrupt {
        name: "BVIO",
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
        name: "DEBUG_0",
        number: 77,
    },
    Interrupt {
        name: "DEBUG_1",
        number: 78,
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
        name: "CLK_TOP_FEMC",
        index: 66,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 67,
    },
    Resource {
        name: "CLK_TOP_XPI1",
        index: 68,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 69,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_URT7",
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
        name: "CLK_TOP_SPI0",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_PTPC",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_ANA3",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_AUD0",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_AUD1",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_ETH0",
        index: 98,
    },
    Resource {
        name: "CLK_TOP_PTP0",
        index: 99,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 100,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 101,
    },
    Resource {
        name: "CLK_TOP_NTM0",
        index: 102,
    },
    Resource {
        name: "CLK_TOP_SDC0",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 128,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 129,
    },
    Resource {
        name: "CLK_TOP_ADC2",
        index: 130,
    },
    Resource {
        name: "CLK_TOP_DAC0",
        index: 131,
    },
    Resource {
        name: "CLK_TOP_I2S0",
        index: 132,
    },
    Resource {
        name: "CLK_TOP_I2S1",
        index: 133,
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
        name: "FEMC",
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
        name: "RAM0",
        index: 262,
    },
    Resource {
        name: "MCT0",
        index: 263,
    },
    Resource {
        name: "XPI0",
        index: 264,
    },
    Resource {
        name: "XPI1",
        index: 265,
    },
    Resource {
        name: "SDP0",
        index: 266,
    },
    Resource {
        name: "RNG0",
        index: 267,
    },
    Resource {
        name: "KMAN",
        index: 268,
    },
    Resource {
        name: "DMA0",
        index: 269,
    },
    Resource {
        name: "DMA1",
        index: 270,
    },
    Resource {
        name: "FFA0",
        index: 271,
    },
    Resource {
        name: "GPIO",
        index: 272,
    },
    Resource {
        name: "MBX0",
        index: 273,
    },
    Resource {
        name: "WDG0",
        index: 274,
    },
    Resource {
        name: "WDG1",
        index: 275,
    },
    Resource {
        name: "TSNS",
        index: 276,
    },
    Resource {
        name: "TMR0",
        index: 277,
    },
    Resource {
        name: "TMR1",
        index: 278,
    },
    Resource {
        name: "TMR2",
        index: 279,
    },
    Resource {
        name: "TMR3",
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
        name: "I2C0",
        index: 289,
    },
    Resource {
        name: "I2C1",
        index: 290,
    },
    Resource {
        name: "I2C2",
        index: 291,
    },
    Resource {
        name: "I2C3",
        index: 292,
    },
    Resource {
        name: "SPI0",
        index: 293,
    },
    Resource {
        name: "SPI1",
        index: 294,
    },
    Resource {
        name: "SPI2",
        index: 295,
    },
    Resource {
        name: "SPI3",
        index: 296,
    },
    Resource {
        name: "CAN0",
        index: 297,
    },
    Resource {
        name: "CAN1",
        index: 298,
    },
    Resource {
        name: "PTPC",
        index: 299,
    },
    Resource {
        name: "ADC0",
        index: 300,
    },
    Resource {
        name: "ADC1",
        index: 301,
    },
    Resource {
        name: "ADC2",
        index: 302,
    },
    Resource {
        name: "DAC0",
        index: 303,
    },
    Resource {
        name: "ACMP",
        index: 304,
    },
    Resource {
        name: "I2S0",
        index: 305,
    },
    Resource {
        name: "I2S1",
        index: 306,
    },
    Resource {
        name: "PDM0",
        index: 307,
    },
    Resource {
        name: "DAO",
        index: 308,
    },
    Resource {
        name: "SYNT",
        index: 309,
    },
    Resource {
        name: "MOT0",
        index: 310,
    },
    Resource {
        name: "MOT1",
        index: 311,
    },
    Resource {
        name: "ETH0",
        index: 312,
    },
    Resource {
        name: "NTM0",
        index: 313,
    },
    Resource {
        name: "SDC0",
        index: 314,
    },
    Resource {
        name: "USB0",
        index: 315,
    },
    Resource {
        name: "REF0",
        index: 316,
    },
    Resource {
        name: "REF1",
        index: 317,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "MCT0",
        index: 0,
    },
    Clock {
        name: "FEMC",
        index: 1,
    },
    Clock {
        name: "XPI0",
        index: 2,
    },
    Clock {
        name: "XPI1",
        index: 3,
    },
    Clock {
        name: "TMR0",
        index: 4,
    },
    Clock {
        name: "TMR1",
        index: 5,
    },
    Clock {
        name: "TMR2",
        index: 6,
    },
    Clock {
        name: "TMR3",
        index: 7,
    },
    Clock {
        name: "URT0",
        index: 8,
    },
    Clock {
        name: "URT1",
        index: 9,
    },
    Clock {
        name: "URT2",
        index: 10,
    },
    Clock {
        name: "URT3",
        index: 11,
    },
    Clock {
        name: "URT4",
        index: 12,
    },
    Clock {
        name: "URT5",
        index: 13,
    },
    Clock {
        name: "URT6",
        index: 14,
    },
    Clock {
        name: "URT7",
        index: 15,
    },
    Clock {
        name: "I2C0",
        index: 16,
    },
    Clock {
        name: "I2C1",
        index: 17,
    },
    Clock {
        name: "I2C2",
        index: 18,
    },
    Clock {
        name: "I2C3",
        index: 19,
    },
    Clock {
        name: "SPI0",
        index: 20,
    },
    Clock {
        name: "SPI1",
        index: 21,
    },
    Clock {
        name: "SPI2",
        index: 22,
    },
    Clock {
        name: "SPI3",
        index: 23,
    },
    Clock {
        name: "CAN0",
        index: 24,
    },
    Clock {
        name: "CAN1",
        index: 25,
    },
    Clock {
        name: "PTPC",
        index: 26,
    },
    Clock {
        name: "ANA0",
        index: 27,
    },
    Clock {
        name: "ANA1",
        index: 28,
    },
    Clock {
        name: "ANA2",
        index: 29,
    },
    Clock {
        name: "ANA3",
        index: 30,
    },
    Clock {
        name: "AUD0",
        index: 31,
    },
    Clock {
        name: "AUD1",
        index: 32,
    },
    Clock {
        name: "ETH0",
        index: 33,
    },
    Clock {
        name: "PTP0",
        index: 34,
    },
    Clock {
        name: "REF0",
        index: 35,
    },
    Clock {
        name: "REF1",
        index: 36,
    },
    Clock {
        name: "NTM0",
        index: 37,
    },
    Clock {
        name: "SDC0",
        index: 38,
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
        name: "TRGM0_INPUT_SRC_NTMR0_CH1_OUT",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_NTMR0_CH0_OUT",
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
        name: "TRGM0_OUTPUT_SRC_ADC0_STRGI_ADCX_PTRGI2A",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC1_STRGI_ADCX_PTRGI2B",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC2_STRGI_ADCX_PTRGI2C",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC_BUFF_TRIGGER",
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
        name: "TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN0",
        value: 64,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN1",
        value: 65,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN2",
        value: 66,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN3",
        value: 67,
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
        name: "TRGM1_INPUT_SRC_NTMR0_CH1_OUT",
        value: 39,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_ENET0_PTP_OUT3",
        value: 40,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_NTMR0_CH0_OUT",
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
        name: "TRGM1_INPUT_SRC_SYNT0_CH0",
        value: 44,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT0_CH1",
        value: 45,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT0_CH2",
        value: 46,
    },
    TrgmMux {
        name: "TRGM1_INPUT_SRC_SYNT0_CH3",
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
        name: "TRGM1_OUTPUT_SRC_ADC0_STRGI_ADCX_PTRGI3A",
        value: 48,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC1_STRGI_ADCX_PTRGI3B",
        value: 49,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_ADC2_STRGI_ADCX_PTRGI3C",
        value: 50,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_DAC_BUFF_TRIGGER",
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
        name: "TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN0",
        value: 64,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN1",
        value: 65,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN2",
        value: 66,
    },
    TrgmMux {
        name: "TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN3",
        value: 67,
    },
];
#[path = "../registers/acmp_common.rs"]
pub mod acmp;
#[path = "../registers/adc16_v63.rs"]
pub mod adc16;
#[path = "../registers/bcfg_v62.rs"]
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
#[path = "../registers/can_v67.rs"]
pub mod can;
#[path = "../registers/dac_v63.rs"]
pub mod dac;
#[path = "../registers/dma_v67.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/enet_v63.rs"]
pub mod enet;
#[path = "../registers/femc_common.rs"]
pub mod femc;
#[path = "../registers/ffa_common.rs"]
pub mod ffa;
#[path = "../registers/gpio_common.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v63.rs"]
pub mod gpiom;
#[path = "../registers/hall_common.rs"]
pub mod hall;
#[path = "../registers/i2c_v67.rs"]
pub mod i2c;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/keym_common.rs"]
pub mod keym;
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
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../registers/pmon_common.rs"]
pub mod pmon;
#[path = "../registers/ppor_v67.rs"]
pub mod ppor;
#[path = "../registers/psec_common.rs"]
pub mod psec;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwm_v53.rs"]
pub mod pwm;
#[path = "../registers/qei_v67.rs"]
pub mod qei;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/sdxc_v63.rs"]
pub mod sdxc;
#[path = "../registers/spi_v67.rs"]
pub mod spi;
#[path = "../registers/synt_v67.rs"]
pub mod synt;
#[path = "../registers/sysctl_v63.rs"]
pub mod sysctl;
#[path = "../registers/tamp_v62.rs"]
pub mod tamp;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/trgm_v67.rs"]
pub mod trgm;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/uart_v67.rs"]
pub mod uart;
#[path = "../registers/usb_v67.rs"]
pub mod usb;
#[path = "../registers/wdg_v67.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
