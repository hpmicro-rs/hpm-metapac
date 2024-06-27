
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
            request: Some(83),
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
                signal: "B",
                dmamux: Some("DMAMUX"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "Z",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "D",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "Y",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "A",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "X",
                dmamux: Some("DMAMUX"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "C",
                dmamux: Some("DMAMUX"),
                request: Some(3),
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "D",
                dmamux: Some("DMAMUX"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "C",
                dmamux: Some("DMAMUX"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "Y",
                dmamux: Some("DMAMUX"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "A",
                dmamux: Some("DMAMUX"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "X",
                dmamux: Some("DMAMUX"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "Z",
                dmamux: Some("DMAMUX"),
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "B",
                dmamux: Some("DMAMUX"),
                request: Some(9),
            },
        ],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PGPIO",
            dmamux: Some("DMAMUX"),
            request: Some(76),
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
            request: Some(84),
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
    Peripheral {
        name: "HDMA",
        address: 0xf00c4000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v62",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 25,
            resource_clock_top: None,
            resource: 313,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "HDMA",
            dmamux: Some("DMAMUX"),
            request: Some(72),
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
            version: "v62",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 26,
            resource_clock_top: None,
            resource: 314,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XDMA",
            dmamux: Some("DMAMUX"),
            request: Some(71),
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
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: None,
            resource: 304,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX0A",
            dmamux: Some("DMAMUX"),
            request: Some(45),
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
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX0B",
            dmamux: Some("DMAMUX"),
            request: Some(46),
        }],
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
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 305,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX1A",
            dmamux: Some("DMAMUX"),
            request: Some(47),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX1B",
            dmamux: Some("DMAMUX"),
            request: Some(48),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MBX1B",
        }],
    },
    Peripheral {
        name: "CRC",
        address: 0xf00b8000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "common",
            block: "CRC",
            ir: &crc::REGISTERS,
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
        interrupts: &[],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0200000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v62",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "P1",
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
            PeripheralPin {
                pin: "PC01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P0",
                alt: Some(16),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM0",
            dmamux: Some("DMAMUX"),
            request: Some(57),
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
            version: "v62",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P2",
                alt: Some(16),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM1",
            dmamux: Some("DMAMUX"),
            request: Some(60),
        }],
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
            version: "v62",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB25",
                signal: "P01",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "P04",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "P07",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "P00",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "P05",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "P03",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "FAULT1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "FAULT0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "P06",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "P02",
                alt: Some(17),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM2",
            dmamux: Some("DMAMUX"),
            request: Some(63),
        }],
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
            version: "v62",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA22",
                signal: "P02",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P03",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P06",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FAULT0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FAULT1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P01",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P04",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "P00",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P07",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P05",
                alt: Some(17),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWM3",
            dmamux: Some("DMAMUX"),
            request: Some(66),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM3",
        }],
    },
    Peripheral {
        name: "PLA0",
        address: 0xf020e000,
        registers: Some(PeripheralRegisters {
            kind: "pla",
            version: "v62",
            block: "PLA",
            ir: &pla::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PLA1",
        address: 0xf021e000,
        registers: Some(PeripheralRegisters {
            kind: "pla",
            version: "v62",
            block: "PLA",
            ir: &pla::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM0",
        address: 0xf020c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v62",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM1",
        address: 0xf021c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v62",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM2",
        address: 0xf022c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v62",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TRGM3",
        address: 0xf023c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v62",
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
        address: 0xf0240000,
        registers: Some(PeripheralRegisters {
            kind: "synt",
            version: "v67",
            block: "SYNT",
            ir: &synt::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
            group_bit_offset: 13,
            resource_clock_top: Some(68),
            resource: 269,
            clock_node: Some(3),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA22",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR0",
            dmamux: Some("DMAMUX"),
            request: Some(49),
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
            group_bit_offset: 14,
            resource_clock_top: Some(69),
            resource: 270,
            clock_node: Some(4),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "COMP1",
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
                pin: "PA23",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "COMP0",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR1",
            dmamux: Some("DMAMUX"),
            request: Some(50),
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
            group_bit_offset: 15,
            resource_clock_top: Some(70),
            resource: 271,
            clock_node: Some(5),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC08",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "COMP1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR2",
            dmamux: Some("DMAMUX"),
            request: Some(51),
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
            group_bit_offset: 16,
            resource_clock_top: Some(71),
            resource: 272,
            clock_node: Some(6),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB24",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "CAPT1",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR3",
            dmamux: Some("DMAMUX"),
            request: Some(52),
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
                pin: "PY04",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CAPT1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP1",
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
                pin: "PY06",
                signal: "COMP3",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTMR",
            dmamux: Some("DMAMUX"),
            request: Some(78),
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
            group_link: 1,
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 302,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "WDG0",
            dmamux: Some("DMAMUX"),
            request: Some(42),
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
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 303,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "WDG1",
            dmamux: Some("DMAMUX"),
            request: Some(43),
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
            request: Some(77),
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
            request: Some(82),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: Some(72),
            resource: 285,
            clock_node: Some(7),
        }),
        pins: &[
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "RXD",
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
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART0",
            dmamux: Some("DMAMUX"),
            request: Some(29),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: Some(73),
            resource: 286,
            clock_node: Some(8),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC20",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB21",
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
                pin: "PC21",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RTS",
                alt: Some(3),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART1",
            dmamux: Some("DMAMUX"),
            request: Some(30),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: Some(74),
            resource: 287,
            clock_node: Some(9),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RTS",
                alt: Some(3),
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
            request: Some(31),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 0,
            resource_clock_top: Some(75),
            resource: 288,
            clock_node: Some(10),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART3",
            dmamux: Some("DMAMUX"),
            request: Some(32),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 1,
            resource_clock_top: Some(76),
            resource: 289,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DE",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART4",
            dmamux: Some("DMAMUX"),
            request: Some(33),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PUART",
            dmamux: Some("DMAMUX"),
            request: Some(79),
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
            group_link: 0,
            group_bit_offset: 24,
            resource_clock_top: Some(84),
            resource: 280,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA30",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
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
                pin: "PA31",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DAT3",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI0",
            dmamux: Some("DMAMUX"),
            request: Some(25),
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
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: Some(85),
            resource: 281,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB02",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI1",
            dmamux: Some("DMAMUX"),
            request: Some(26),
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
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: Some(86),
            resource: 282,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC01",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "DAT3",
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
            request: Some(27),
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
            group_link: 0,
            group_bit_offset: 9,
            resource_clock_top: Some(80),
            resource: 265,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB22",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY04",
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
            group_link: 0,
            group_bit_offset: 10,
            resource_clock_top: Some(81),
            resource: 266,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "SCL",
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
            PeripheralPin {
                pin: "PA08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "SCL",
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
            group_link: 0,
            group_bit_offset: 11,
            resource_clock_top: Some(82),
            resource: 267,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "SCL",
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
                pin: "PB18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ02",
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
            interrupt: "I2C2",
        }],
    },
    Peripheral {
        name: "LIN0",
        address: 0xf3030000,
        registers: Some(PeripheralRegisters {
            kind: "lin",
            version: "v62",
            block: "LIN",
            ir: &lin::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 5,
            resource_clock_top: Some(100),
            resource: 293,
            clock_node: Some(35),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "LIN0",
            dmamux: Some("DMAMUX"),
            request: Some(90),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LIN0",
        }],
    },
    Peripheral {
        name: "LIN1",
        address: 0xf3034000,
        registers: Some(PeripheralRegisters {
            kind: "lin",
            version: "v62",
            block: "LIN",
            ir: &lin::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 6,
            resource_clock_top: Some(101),
            resource: 294,
            clock_node: Some(36),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "LIN1",
            dmamux: Some("DMAMUX"),
            request: Some(91),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LIN1",
        }],
    },
    Peripheral {
        name: "LIN2",
        address: 0xf3038000,
        registers: Some(PeripheralRegisters {
            kind: "lin",
            version: "v62",
            block: "LIN",
            ir: &lin::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 7,
            resource_clock_top: Some(102),
            resource: 295,
            clock_node: Some(37),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "LIN2",
            dmamux: Some("DMAMUX"),
            request: Some(92),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LIN2",
        }],
    },
    Peripheral {
        name: "LIN3",
        address: 0xf303c000,
        registers: Some(PeripheralRegisters {
            kind: "lin",
            version: "v62",
            block: "LIN",
            ir: &lin::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 8,
            resource_clock_top: Some(103),
            resource: 296,
            clock_node: Some(38),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "LIN3",
            dmamux: Some("DMAMUX"),
            request: Some(93),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LIN3",
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
            group_bit_offset: 9,
            resource_clock_top: Some(92),
            resource: 297,
            clock_node: Some(27),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTPC",
            dmamux: Some("DMAMUX"),
            request: Some(41),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTPC",
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
            group_bit_offset: 31,
            resource_clock_top: None,
            resource: 319,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC07",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "ID",
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
            request: Some(74),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB0",
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
            group_link: 0,
            group_bit_offset: 18,
            resource_clock_top: Some(128),
            resource: 274,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC0",
            dmamux: Some("DMAMUX"),
            request: Some(15),
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
            group_link: 0,
            group_bit_offset: 23,
            resource_clock_top: None,
            resource: 279,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC14",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "COMP3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "COMP3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "COMP2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA05",
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
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "3",
                dmamux: Some("DMAMUX"),
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "2",
                dmamux: Some("DMAMUX"),
                request: Some(23),
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
        name: "TSNS",
        address: 0xf4104000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 30,
            resource_clock_top: None,
            resource: 318,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "TSNS",
            dmamux: Some("DMAMUX"),
            request: Some(44),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSNS",
        }],
    },
    Peripheral {
        name: "SDM",
        address: 0xf001c000,
        registers: Some(PeripheralRegisters {
            kind: "sdm",
            version: "v62",
            block: "SDM",
            ir: &sdm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
            group_link: 1,
            group_bit_offset: 28,
            resource_clock_top: None,
            resource: 316,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDP",
            dmamux: Some("DMAMUX"),
            request: Some(69),
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
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "RNG",
            dmamux: Some("DMAMUX"),
            request: Some(73),
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
            group_link: 1,
            group_bit_offset: 27,
            resource_clock_top: None,
            resource: 315,
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
            request: Some(75),
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
        name: "MCAN0",
        address: 0xf0080000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "STBY",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN0",
            dmamux: Some("DMAMUX"),
            request: Some(37),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN0",
        }],
    },
    Peripheral {
        name: "MCAN1",
        address: 0xf0084000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA25",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN1",
            dmamux: Some("DMAMUX"),
            request: Some(38),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN1",
        }],
    },
    Peripheral {
        name: "MCAN2",
        address: 0xf0088000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PC11",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN2",
            dmamux: Some("DMAMUX"),
            request: Some(39),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN2",
        }],
    },
    Peripheral {
        name: "MCAN3",
        address: 0xf008c000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA05",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "STBY",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN3",
            dmamux: Some("DMAMUX"),
            request: Some(40),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN3",
        }],
    },
    Peripheral {
        name: "UART5",
        address: 0xf0054000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 2,
            resource_clock_top: Some(77),
            resource: 290,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                alt: Some(3),
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
            PeripheralPin {
                pin: "PC05",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
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
                pin: "PB09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "RXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART5",
            dmamux: Some("DMAMUX"),
            request: Some(34),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 3,
            resource_clock_top: Some(78),
            resource: 291,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA22",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PA22",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DE",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART6",
            dmamux: Some("DMAMUX"),
            request: Some(35),
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
            version: "v62",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 4,
            resource_clock_top: Some(79),
            resource: 292,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PY04",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PC13",
                signal: "RXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART7",
            dmamux: Some("DMAMUX"),
            request: Some(36),
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
            group_link: 0,
            group_bit_offset: 27,
            resource_clock_top: Some(87),
            resource: 283,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "MOSI",
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
                pin: "PY03",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "CSN",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "CSN",
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
                pin: "PY02",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI3",
            dmamux: Some("DMAMUX"),
            request: Some(28),
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
            group_link: 0,
            group_bit_offset: 12,
            resource_clock_top: Some(83),
            resource: 268,
            clock_node: Some(18),
        }),
        pins: &[
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
                pin: "PA22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "SCL",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(56),
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
            request: Some(59),
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
            request: Some(62),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "QEI2",
            dmamux: Some("DMAMUX"),
            request: Some(65),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "QEI3",
            dmamux: Some("DMAMUX"),
            request: Some(68),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "HALL0",
            dmamux: Some("DMAMUX"),
            request: Some(58),
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
            request: Some(61),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "HALL2",
            dmamux: Some("DMAMUX"),
            request: Some(64),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "HALL3",
            dmamux: Some("DMAMUX"),
            request: Some(67),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HALL3",
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
            group_link: 0,
            group_bit_offset: 19,
            resource_clock_top: Some(129),
            resource: 275,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            dmamux: Some("DMAMUX"),
            request: Some(16),
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
            group_link: 0,
            group_bit_offset: 20,
            resource_clock_top: Some(130),
            resource: 276,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            dmamux: Some("DMAMUX"),
            request: Some(17),
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
            version: "v53",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 21,
            resource_clock_top: Some(131),
            resource: 277,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC0",
            dmamux: Some("DMAMUX"),
            request: Some(19),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC0",
        }],
    },
    Peripheral {
        name: "DAC1",
        address: 0xf0028000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v53",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 22,
            resource_clock_top: Some(132),
            resource: 278,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC1",
            dmamux: Some("DMAMUX"),
            request: Some(20),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC1",
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
        name: "CPU1",
        index: 8,
    },
    Resource {
        name: "XDMA",
        index: 314,
    },
    Resource {
        name: "POW_CPU0",
        index: 21,
    },
    Resource {
        name: "DAC1",
        index: 278,
    },
    Resource {
        name: "CLK_SRC_XTAL",
        index: 32,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 79,
    },
    Resource {
        name: "KMAN",
        index: 315,
    },
    Resource {
        name: "I2C0",
        index: 265,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 85,
    },
    Resource {
        name: "URT3",
        index: 288,
    },
    Resource {
        name: "CLK_SRC_PLL1",
        index: 37,
    },
    Resource {
        name: "RAM0",
        index: 264,
    },
    Resource {
        name: "LMM0",
        index: 259,
    },
    Resource {
        name: "URT5",
        index: 290,
    },
    Resource {
        name: "CAN2",
        index: 300,
    },
    Resource {
        name: "I2C3",
        index: 268,
    },
    Resource {
        name: "CRC0",
        index: 306,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_ANA0",
        index: 93,
    },
    Resource {
        name: "XPI0",
        index: 312,
    },
    Resource {
        name: "CLK_TOP_TMR0",
        index: 68,
    },
    Resource {
        name: "TMR0",
        index: 269,
    },
    Resource {
        name: "SDM0",
        index: 284,
    },
    Resource {
        name: "URT2",
        index: 287,
    },
    Resource {
        name: "CLK_TOP_DAC1",
        index: 132,
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
        name: "CLK_TOP_ADC2",
        index: 130,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 83,
    },
    Resource {
        name: "WDG0",
        index: 302,
    },
    Resource {
        name: "RNG0",
        index: 317,
    },
    Resource {
        name: "MBX1",
        index: 305,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 80,
    },
    Resource {
        name: "SPI0",
        index: 280,
    },
    Resource {
        name: "SPI1",
        index: 281,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 67,
    },
    Resource {
        name: "AHBP",
        index: 256,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 90,
    },
    Resource {
        name: "ADC2",
        index: 276,
    },
    Resource {
        name: "HDMA",
        index: 313,
    },
    Resource {
        name: "CLK_SRC_PLL0",
        index: 33,
    },
    Resource {
        name: "CLK_TOP_DAC0",
        index: 131,
    },
    Resource {
        name: "CLK_SRC_CLK2_PLL0",
        index: 36,
    },
    Resource {
        name: "URT1",
        index: 286,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL2",
        index: 41,
    },
    Resource {
        name: "LIN0",
        index: 293,
    },
    Resource {
        name: "CAN0",
        index: 298,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 84,
    },
    Resource {
        name: "TMR1",
        index: 270,
    },
    Resource {
        name: "CLK_TOP_ANA3",
        index: 96,
    },
    Resource {
        name: "MSYN",
        index: 311,
    },
    Resource {
        name: "CLK_TOP_MCT1",
        index: 66,
    },
    Resource {
        name: "SDP0",
        index: 316,
    },
    Resource {
        name: "CLK_TOP_REF1",
        index: 99,
    },
    Resource {
        name: "CPU0",
        index: 0,
    },
    Resource {
        name: "ROM0",
        index: 263,
    },
    Resource {
        name: "RST_CPU0",
        index: 24,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 129,
    },
    Resource {
        name: "REF0",
        index: 320,
    },
    Resource {
        name: "CAN1",
        index: 299,
    },
    Resource {
        name: "GPIO",
        index: 273,
    },
    Resource {
        name: "CLK_SRC_PLL1_REF",
        index: 44,
    },
    Resource {
        name: "CLK_TOP_ANA4",
        index: 97,
    },
    Resource {
        name: "CLK_TOP_LIN3",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_ANA2",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_MCT0",
        index: 65,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_CPU0",
        index: 64,
    },
    Resource {
        name: "TMR3",
        index: 272,
    },
    Resource {
        name: "CLK_SRC_PLL2_REF",
        index: 45,
    },
    Resource {
        name: "URT0",
        index: 285,
    },
    Resource {
        name: "POW_CPU1",
        index: 22,
    },
    Resource {
        name: "CLK_TOP_LIN2",
        index: 102,
    },
    Resource {
        name: "I2C1",
        index: 266,
    },
    Resource {
        name: "MCT1",
        index: 262,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL0",
        index: 34,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 128,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 70,
    },
    Resource {
        name: "CLK_SRC_CLK0_PLL1",
        index: 38,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_PTPC",
        index: 92,
    },
    Resource {
        name: "URT6",
        index: 291,
    },
    Resource {
        name: "TMR2",
        index: 271,
    },
    Resource {
        name: "URT7",
        index: 292,
    },
    Resource {
        name: "MOT1",
        index: 308,
    },
    Resource {
        name: "REF1",
        index: 321,
    },
    Resource {
        name: "RST_SOC",
        index: 23,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_LIN0",
        index: 100,
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
        name: "CLK_TOP_URT2",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 91,
    },
    Resource {
        name: "I2C2",
        index: 267,
    },
    Resource {
        name: "CLK_TOP_REF0",
        index: 98,
    },
    Resource {
        name: "MOT0",
        index: 307,
    },
    Resource {
        name: "SPI2",
        index: 282,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 73,
    },
    Resource {
        name: "CPX1",
        index: 9,
    },
    Resource {
        name: "AXIS",
        index: 257,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL2",
        index: 42,
    },
    Resource {
        name: "RST_CPU1",
        index: 25,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 71,
    },
    Resource {
        name: "LIN2",
        index: 295,
    },
    Resource {
        name: "ADC1",
        index: 275,
    },
    Resource {
        name: "AXIC",
        index: 258,
    },
    Resource {
        name: "LIN1",
        index: 294,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 76,
    },
    Resource {
        name: "MCT0",
        index: 260,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 77,
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
        name: "LMM1",
        index: 261,
    },
    Resource {
        name: "CLK_SRC_PLL0_REF",
        index: 43,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL1",
        index: 39,
    },
    Resource {
        name: "MBX0",
        index: 304,
    },
    Resource {
        name: "ADC0",
        index: 274,
    },
    Resource {
        name: "PTPC",
        index: 297,
    },
    Resource {
        name: "URT4",
        index: 289,
    },
    Resource {
        name: "SPI3",
        index: 283,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 69,
    },
    Resource {
        name: "CLK_SRC_CLK1_PLL0",
        index: 35,
    },
    Resource {
        name: "DAC0",
        index: 277,
    },
    Resource {
        name: "CAN3",
        index: 301,
    },
    Resource {
        name: "CLK_SRC_PLL2",
        index: 40,
    },
    Resource {
        name: "WDG1",
        index: 303,
    },
    Resource {
        name: "ACMP",
        index: 279,
    },
    Resource {
        name: "LIN3",
        index: 296,
    },
];
pub(crate) static CLOCKS: &[Clock] = &[
    Clock {
        name: "I2C1",
        index: 16,
    },
    Clock {
        name: "TMR2",
        index: 5,
    },
    Clock {
        name: "TMR1",
        index: 4,
    },
    Clock {
        name: "URT2",
        index: 9,
    },
    Clock {
        name: "CAN1",
        index: 24,
    },
    Clock {
        name: "URT5",
        index: 12,
    },
    Clock {
        name: "CAN2",
        index: 25,
    },
    Clock {
        name: "ANA1",
        index: 29,
    },
    Clock {
        name: "SPI1",
        index: 20,
    },
    Clock {
        name: "URT7",
        index: 14,
    },
    Clock {
        name: "I2C2",
        index: 17,
    },
    Clock {
        name: "LIN0",
        index: 35,
    },
    Clock {
        name: "REF1",
        index: 34,
    },
    Clock {
        name: "XPI0",
        index: 2,
    },
    Clock {
        name: "ANA2",
        index: 30,
    },
    Clock {
        name: "TMR0",
        index: 3,
    },
    Clock {
        name: "LIN2",
        index: 37,
    },
    Clock {
        name: "PTPC",
        index: 27,
    },
    Clock {
        name: "ANA3",
        index: 31,
    },
    Clock {
        name: "REF0",
        index: 33,
    },
    Clock {
        name: "SPI2",
        index: 21,
    },
    Clock {
        name: "TMR3",
        index: 6,
    },
    Clock {
        name: "SPI0",
        index: 19,
    },
    Clock {
        name: "ANA0",
        index: 28,
    },
    Clock {
        name: "URT3",
        index: 10,
    },
    Clock {
        name: "SPI3",
        index: 22,
    },
    Clock {
        name: "LIN3",
        index: 38,
    },
    Clock {
        name: "LIN1",
        index: 36,
    },
    Clock {
        name: "URT1",
        index: 8,
    },
    Clock {
        name: "URT0",
        index: 7,
    },
    Clock {
        name: "URT4",
        index: 11,
    },
    Clock {
        name: "ANA4",
        index: 32,
    },
    Clock {
        name: "CAN3",
        index: 26,
    },
    Clock {
        name: "MCT0",
        index: 0,
    },
    Clock {
        name: "URT6",
        index: 13,
    },
    Clock {
        name: "CAN0",
        index: 23,
    },
    Clock {
        name: "I2C0",
        index: 15,
    },
    Clock {
        name: "I2C3",
        index: 18,
    },
    Clock {
        name: "MCT1",
        index: 1,
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
#[path = "../registers/crc_common.rs"]
pub mod crc;
#[path = "../registers/dac_v53.rs"]
pub mod dac;
#[path = "../registers/dma_v62.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/gpio_common.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v67.rs"]
pub mod gpiom;
#[path = "../registers/hall_common.rs"]
pub mod hall;
#[path = "../registers/i2c_v67.rs"]
pub mod i2c;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/keym_common.rs"]
pub mod keym;
#[path = "../registers/lin_v62.rs"]
pub mod lin;
#[path = "../registers/mbx_common.rs"]
pub mod mbx;
#[path = "../registers/mcan_v68.rs"]
pub mod mcan;
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/mono_common.rs"]
pub mod mono;
#[path = "../registers/otp_common.rs"]
pub mod otp;
#[path = "../registers/pcfg_v67.rs"]
pub mod pcfg;
#[path = "../registers/pla_v62.rs"]
pub mod pla;
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
#[path = "../registers/pwm_v62.rs"]
pub mod pwm;
#[path = "../registers/qei_v67.rs"]
pub mod qei;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdm_v62.rs"]
pub mod sdm;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/spi_v67.rs"]
pub mod spi;
#[path = "../registers/synt_v67.rs"]
pub mod synt;
#[path = "../registers/sysctl_v62.rs"]
pub mod sysctl;
#[path = "../registers/tamp_v62.rs"]
pub mod tamp;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/trgm_v62.rs"]
pub mod trgm;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/uart_v62.rs"]
pub mod uart;
#[path = "../registers/usb_v67.rs"]
pub mod usb;
#[path = "../registers/wdg_v67.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
