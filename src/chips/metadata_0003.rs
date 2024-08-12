
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
            group_bit_offset: 2,
            resource_clock_top: Some(65),
            resource: 258,
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
            version: "v53",
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
        address: 0xf3000000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 19,
            resource_clock_top: Some(94),
            resource: 307,
            clock_node: Some(29),
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(61),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI0",
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
        name: "PDGO",
        address: 0xf4134000,
        registers: Some(PeripheralRegisters {
            kind: "pdgo",
            version: "v53",
            block: "PDGO",
            ir: &pdgo::REGISTERS,
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
            version: "v53",
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
        name: "FGPIO",
        address: 0xc0000,
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
                signal: "PX",
                interrupt: "GPIO0_X",
            },
            PeripheralInterrupt {
                signal: "PY",
                interrupt: "GPIO0_Y",
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
        name: "GPIOM",
        address: 0xf00d8000,
        registers: Some(PeripheralRegisters {
            kind: "gpiom",
            version: "v53",
            block: "GPIOM",
            ir: &gpiom::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OTP",
        address: 0xf3050000,
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
        name: "HDMA",
        address: 0xf00c8000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v53",
            block: "DMA",
            ir: &dma::REGISTERS,
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
            interrupt: "HDMA",
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
            group_bit_offset: 3,
            resource_clock_top: None,
            resource: 291,
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
        name: "CRC",
        address: 0xf0080000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "common",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 5,
            resource_clock_top: None,
            resource: 293,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPTMR0",
        address: 0xf0000000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 13,
            resource_clock_top: Some(74),
            resource: 269,
            clock_node: Some(9),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(2),
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 14,
            resource_clock_top: Some(75),
            resource: 270,
            clock_node: Some(10),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR1",
        }],
    },
    Peripheral {
        name: "PTMR",
        address: 0xf4120000,
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
                pin: "PY03",
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
        address: 0xf00b0000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v53",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 1,
            resource_clock_top: None,
            resource: 289,
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
            group_bit_offset: 2,
            resource_clock_top: None,
            resource: 290,
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWDG",
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
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: Some(86),
            resource: 281,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY02",
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
                pin: "PY01",
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
                pin: "PY00",
                signal: "TXD",
                alt: Some(2),
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
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: Some(87),
            resource: 282,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
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
            group_link: 0,
            group_bit_offset: 27,
            resource_clock_top: Some(88),
            resource: 283,
            clock_node: Some(23),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB08",
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
            group_link: 0,
            group_bit_offset: 28,
            resource_clock_top: Some(89),
            resource: 284,
            clock_node: Some(24),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(27),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART3",
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
        pins: &[
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "RTS",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY00",
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
        address: 0xf0070000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 21,
            resource_clock_top: Some(82),
            resource: 277,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "SCLK",
                alt: Some(5),
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
            group_bit_offset: 22,
            resource_clock_top: Some(83),
            resource: 278,
            clock_node: Some(18),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "SCLK",
                alt: Some(5),
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
            group_bit_offset: 23,
            resource_clock_top: Some(84),
            resource: 279,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "SCLK",
                alt: Some(5),
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
            group_bit_offset: 24,
            resource_clock_top: Some(85),
            resource: 280,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "SCLK",
                alt: Some(5),
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
            resource_clock_top: Some(78),
            resource: 273,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(36),
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
            resource_clock_top: Some(79),
            resource: 274,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(37),
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
            resource_clock_top: Some(80),
            resource: 275,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY03",
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
            resource_clock_top: Some(81),
            resource: 276,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY06",
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
            interrupt: "I2C3",
        }],
    },
    Peripheral {
        name: "PTPC",
        address: 0xf02fc000,
        registers: Some(PeripheralRegisters {
            kind: "ptpc",
            version: "common",
            block: "PTPC",
            ir: &ptpc::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 8,
            resource_clock_top: None,
            resource: 264,
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
        address: 0xf300c000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v53",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 20,
            resource_clock_top: None,
            resource: 308,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA31",
                signal: "ID",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "ID",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "ID",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "OC",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "OC",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "OC",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "PWR",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "PWR",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "PWR",
                alt: Some(25),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB0",
        }],
    },
    Peripheral {
        name: "ADC0",
        address: 0xf3080000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v53",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 6,
            resource_clock_top: Some(101),
            resource: 294,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB04",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "IN9",
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
        name: "ACMP",
        address: 0xf30b0000,
        registers: Some(PeripheralRegisters {
            kind: "acmp",
            version: "common",
            block: "ACMP",
            ir: &acmp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 10,
            resource_clock_top: None,
            resource: 298,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "CMP1_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CMP1_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CMP1_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CMP1_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "COMP0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "COMP0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "COMP0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "COMP1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "COMP1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "COMP1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "COMP1",
                alt: Some(18),
            },
        ],
        dma_channels: &[],
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
        address: 0xf0090000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 4,
            resource_clock_top: None,
            resource: 292,
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
        name: "GPTMR2",
        address: 0xf0008000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 15,
            resource_clock_top: Some(76),
            resource: 271,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(8),
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 16,
            resource_clock_top: Some(77),
            resource: 272,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(12),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR3",
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
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: Some(90),
            resource: 285,
            clock_node: Some(25),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(28),
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
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: Some(91),
            resource: 286,
            clock_node: Some(26),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TXD",
                alt: Some(2),
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
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: Some(92),
            resource: 287,
            clock_node: Some(27),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
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
            group_bit_offset: 0,
            resource_clock_top: Some(93),
            resource: 288,
            clock_node: Some(28),
        }),
        pins: &[
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
                pin: "PA31",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(35),
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0xf3084000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v53",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 7,
            resource_clock_top: Some(102),
            resource: 295,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB04",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "IN9",
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
        name: "DAC0",
        address: 0xf3090000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v53",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 8,
            resource_clock_top: Some(103),
            resource: 296,
            clock_node: None,
        }),
        pins: &[PeripheralPin {
            pin: "PB08",
            signal: "OUT",
            alt: Some(0),
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC0",
            dmamux: Some("DMAMUX"),
            request: Some(62),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC0",
        }],
    },
    Peripheral {
        name: "DAC1",
        address: 0xf3094000,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v53",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 9,
            resource_clock_top: Some(104),
            resource: 297,
            clock_node: None,
        }),
        pins: &[PeripheralPin {
            pin: "PB09",
            signal: "OUT",
            alt: Some(0),
        }],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC1",
            dmamux: Some("DMAMUX"),
            request: Some(63),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAC1",
        }],
    },
    Peripheral {
        name: "OPAMP0",
        address: 0xf30a0000,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v53",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 11,
            resource_clock_top: None,
            resource: 299,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "OPAMP1",
        address: 0xf30a4000,
        registers: Some(PeripheralRegisters {
            kind: "opamp",
            version: "v53",
            block: "OPAMP",
            ir: &opamp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 12,
            resource_clock_top: None,
            resource: 300,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MCAN0",
        address: 0xf0280000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 4,
            resource_clock_top: Some(66),
            resource: 260,
            clock_node: Some(1),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN0",
            dmamux: Some("DMAMUX"),
            request: Some(48),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN0",
        }],
    },
    Peripheral {
        name: "MCAN1",
        address: 0xf0284000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 5,
            resource_clock_top: Some(67),
            resource: 261,
            clock_node: Some(2),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN1",
            dmamux: Some("DMAMUX"),
            request: Some(49),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN1",
        }],
    },
    Peripheral {
        name: "MCAN2",
        address: 0xf0288000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 6,
            resource_clock_top: Some(68),
            resource: 262,
            clock_node: Some(3),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN2",
            dmamux: Some("DMAMUX"),
            request: Some(50),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN2",
        }],
    },
    Peripheral {
        name: "MCAN3",
        address: 0xf028c000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v53",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 7,
            resource_clock_top: Some(69),
            resource: 263,
            clock_node: Some(4),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN3",
            dmamux: Some("DMAMUX"),
            request: Some(51),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN3",
        }],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0318000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "FAULT0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FAULT0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "FAULT0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "FAULT0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "FAULT1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "FAULT1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "FAULT1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "FAULT1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY07",
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
        address: 0xf031c000,
        registers: Some(PeripheralRegisters {
            kind: "pwm",
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "FAULT0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "FAULT0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "FAULT0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "FAULT0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "FAULT0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "FAULT0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "FAULT0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "FAULT1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "FAULT1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "FAULT1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "FAULT1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "FAULT1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "FAULT1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "FAULT1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "P0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "P1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "P1",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "P2",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P2",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P2",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P2",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "P2",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "P3",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P3",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P3",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P3",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "P3",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "P4",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "P4",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P4",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "P4",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "P4",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "P5",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P5",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P5",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "P5",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "P5",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "P6",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "P6",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P6",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P6",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "P6",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "P7",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P7",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "P7",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "P7",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "P7",
                alt: Some(17),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWM1",
        }],
    },
    Peripheral {
        name: "TRGM0",
        address: 0xf033c000,
        registers: Some(PeripheralRegisters {
            kind: "trgm",
            version: "v53",
            block: "TRGM",
            ir: &trgm::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "P00",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "P00",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "P00",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P00",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "P00",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "P01",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "P01",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "P01",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P01",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "P01",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "P02",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "P02",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "P02",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P02",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "P02",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "P03",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "P03",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "P03",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P03",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "P03",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "P04",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "P04",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "P04",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P04",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "P04",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "P05",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "P05",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "P05",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P05",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "P05",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "P06",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "P06",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "P06",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P06",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "P06",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "P07",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "P07",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "P07",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P07",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "P07",
                alt: Some(18),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TRGM0",
        }],
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
        address: 0xf0300000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v53",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "Z",
                alt: Some(20),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI0",
        }],
    },
    Peripheral {
        name: "QEI1",
        address: 0xf0304000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v53",
            block: "QEI",
            ir: &qei::REGISTERS,
        }),
        sysctl: None,
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "Z",
                alt: Some(20),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QEI1",
        }],
    },
    Peripheral {
        name: "QEO0",
        address: 0xf0308000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v53",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "QEO1",
        address: 0xf030c000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v53",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "MMC0",
        address: 0xf0310000,
        registers: Some(PeripheralRegisters {
            kind: "mmc",
            version: "v53",
            block: "MMC",
            ir: &mmc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MMC0",
        }],
    },
    Peripheral {
        name: "MMC1",
        address: 0xf0314000,
        registers: Some(PeripheralRegisters {
            kind: "mmc",
            version: "v53",
            block: "MMC",
            ir: &mmc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MMC1",
        }],
    },
    Peripheral {
        name: "RDC0",
        address: 0xf0320000,
        registers: Some(PeripheralRegisters {
            kind: "rdc",
            version: "v53",
            block: "RDC",
            ir: &rdc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SEI",
        address: 0xf032c000,
        registers: Some(PeripheralRegisters {
            kind: "sei",
            version: "v53",
            block: "SEI",
            ir: &sei::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SEI0",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SEI1",
            },
        ],
    },
    Peripheral {
        name: "PLB",
        address: 0xf0324000,
        registers: Some(PeripheralRegisters {
            kind: "plb",
            version: "v53",
            block: "PLB",
            ir: &plb::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SDP",
        address: 0xf3040000,
        registers: Some(PeripheralRegisters {
            kind: "sdp",
            version: "v53",
            block: "SDP",
            ir: &sdp::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 303,
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
        address: 0xf3054000,
        registers: Some(PeripheralRegisters {
            kind: "keym",
            version: "common",
            block: "KEYM",
            ir: &keym::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: None,
            resource: 304,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SEC",
        address: 0xf3044000,
        registers: Some(PeripheralRegisters {
            kind: "sec",
            version: "common",
            block: "SEC",
            ir: &sec::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SECMON",
        }],
    },
    Peripheral {
        name: "PMON",
        address: 0xf3048000,
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
        name: "GPIO0_X",
        number: 3,
    },
    Interrupt {
        name: "GPIO0_Y",
        number: 4,
    },
    Interrupt {
        name: "GPTMR0",
        number: 5,
    },
    Interrupt {
        name: "GPTMR1",
        number: 6,
    },
    Interrupt {
        name: "GPTMR2",
        number: 7,
    },
    Interrupt {
        name: "GPTMR3",
        number: 8,
    },
    Interrupt {
        name: "UART0",
        number: 13,
    },
    Interrupt {
        name: "UART1",
        number: 14,
    },
    Interrupt {
        name: "UART2",
        number: 15,
    },
    Interrupt {
        name: "UART3",
        number: 16,
    },
    Interrupt {
        name: "UART4",
        number: 17,
    },
    Interrupt {
        name: "UART5",
        number: 18,
    },
    Interrupt {
        name: "UART6",
        number: 19,
    },
    Interrupt {
        name: "UART7",
        number: 20,
    },
    Interrupt {
        name: "I2C0",
        number: 21,
    },
    Interrupt {
        name: "I2C1",
        number: 22,
    },
    Interrupt {
        name: "I2C2",
        number: 23,
    },
    Interrupt {
        name: "I2C3",
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
        name: "TSNS",
        number: 29,
    },
    Interrupt {
        name: "MBX0A",
        number: 30,
    },
    Interrupt {
        name: "MBX0B",
        number: 31,
    },
    Interrupt {
        name: "EWDG0",
        number: 32,
    },
    Interrupt {
        name: "EWDG1",
        number: 33,
    },
    Interrupt {
        name: "HDMA",
        number: 34,
    },
    Interrupt {
        name: "MCAN0",
        number: 35,
    },
    Interrupt {
        name: "MCAN1",
        number: 36,
    },
    Interrupt {
        name: "MCAN2",
        number: 37,
    },
    Interrupt {
        name: "MCAN3",
        number: 38,
    },
    Interrupt {
        name: "PTPC",
        number: 39,
    },
    Interrupt {
        name: "PWM0",
        number: 40,
    },
    Interrupt {
        name: "QEI0",
        number: 41,
    },
    Interrupt {
        name: "SEI0",
        number: 42,
    },
    Interrupt {
        name: "MMC0",
        number: 43,
    },
    Interrupt {
        name: "TRGM0",
        number: 44,
    },
    Interrupt {
        name: "PWM1",
        number: 45,
    },
    Interrupt {
        name: "QEI1",
        number: 46,
    },
    Interrupt {
        name: "SEI1",
        number: 47,
    },
    Interrupt {
        name: "MMC1",
        number: 48,
    },
    Interrupt {
        name: "TRGM1",
        number: 49,
    },
    Interrupt {
        name: "RDC",
        number: 50,
    },
    Interrupt {
        name: "USB0",
        number: 51,
    },
    Interrupt {
        name: "XPI0",
        number: 52,
    },
    Interrupt {
        name: "SDP",
        number: 53,
    },
    Interrupt {
        name: "PSEC",
        number: 54,
    },
    Interrupt {
        name: "SECMON",
        number: 55,
    },
    Interrupt {
        name: "RNG",
        number: 56,
    },
    Interrupt {
        name: "FUSE",
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
        name: "DAC0",
        number: 60,
    },
    Interrupt {
        name: "DAC1",
        number: 61,
    },
    Interrupt {
        name: "ACMP_0",
        number: 62,
    },
    Interrupt {
        name: "ACMP_1",
        number: 63,
    },
    Interrupt {
        name: "SYSCTL",
        number: 64,
    },
    Interrupt {
        name: "PGPIO",
        number: 65,
    },
    Interrupt {
        name: "PTMR",
        number: 66,
    },
    Interrupt {
        name: "PUART",
        number: 67,
    },
    Interrupt {
        name: "PWDG",
        number: 68,
    },
    Interrupt {
        name: "BROWNOUT",
        number: 69,
    },
    Interrupt {
        name: "PAD_WAKEUP",
        number: 70,
    },
    Interrupt {
        name: "DEBUG0",
        number: 71,
    },
    Interrupt {
        name: "DEBUG1",
        number: 72,
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
        name: "TRGM0_DMA_SRC_PWM1_CMP0",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP1",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP2",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP3",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP4",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP5",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP6",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP7",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP8",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP9",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP10",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP11",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP12",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP13",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP14",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP15",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP16",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP17",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP18",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP19",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP20",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP21",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP22",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_CMP23",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_RLD",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_HALFRLD",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_XRLD",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_QEI0",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_QEI1",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_MMC0",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_MMC1",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_SEI0",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_SEI1",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_TRGM0",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_TRGM1",
        value: 61,
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
        name: "TRGM0_FILTER_SRC_PWM1_IN0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_IN7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN2",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN3",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN4",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN5",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN6",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM_IN7",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_FAULT0",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_FAULT1",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_FAULT0",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_FAULT1",
        value: 27,
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
        name: "TRGM0_INPUT_SRC_DEBUG_FLAG",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_USB0_SOF",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PTPC_CMP0",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PTPC_CMP1",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP0_OUT",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_CMP1_OUT",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT2",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT3",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR2_OUT2",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR2_OUT3",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR3_OUT2",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR3_OUT3",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P2",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P3",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P4",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P5",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P6",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P7",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH0",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH1",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH2",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT0_CH3",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_MMC0_TRGO_0",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_MMC0_TRGO_1",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_MMC1_TRGO_0",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_MMC1_TRGO_1",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_0",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_1",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_2",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_3",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_4",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_5",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_6",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_TRGO_7",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_0",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_1",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_2",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_3",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_4",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_5",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_6",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_TRGO_7",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH8REF",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH9REF",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH10REF",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH11REF",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH12REF",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH13REF",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH14REF",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CH15REF",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH8REF",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH9REF",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH10REF",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH11REF",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH12REF",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH13REF",
        value: 61,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH14REF",
        value: 62,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CH15REF",
        value: 63,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT00",
        value: 64,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT01",
        value: 65,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT02",
        value: 66,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT03",
        value: 67,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT04",
        value: 68,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT05",
        value: 69,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT06",
        value: 70,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT07",
        value: 71,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT08",
        value: 72,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT09",
        value: 73,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT10",
        value: 74,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT11",
        value: 75,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT12",
        value: 76,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT13",
        value: 77,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT14",
        value: 78,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT15",
        value: 79,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT16",
        value: 80,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT17",
        value: 81,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT18",
        value: 82,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT19",
        value: 83,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT20",
        value: 84,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT21",
        value: 85,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT22",
        value: 86,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT23",
        value: 87,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT24",
        value: 88,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT25",
        value: 89,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT26",
        value: 90,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT27",
        value: 91,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT28",
        value: 92,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT29",
        value: 93,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT30",
        value: 94,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT31",
        value: 95,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_RDC_TRGO_0",
        value: 96,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_RDC_TRGO_1",
        value: 97,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI1_TRGO",
        value: 98,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI0_TRGO",
        value: 99,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_0",
        value: 100,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_1",
        value: 101,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_2",
        value: 102,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_3",
        value: 103,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_4",
        value: 104,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_5",
        value: 105,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_6",
        value: 106,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SEI_TRGO_7",
        value: 107,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_FAULT0",
        value: 108,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_FAULT1",
        value: 109,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_FAULT0",
        value: 110,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_FAULT1",
        value: 111,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN0",
        value: 112,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN1",
        value: 113,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN2",
        value: 114,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN3",
        value: 115,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN4",
        value: 116,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN5",
        value: 117,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN6",
        value: 118,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_CAPIN7",
        value: 119,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN0",
        value: 120,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN1",
        value: 121,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN2",
        value: 122,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN3",
        value: 123,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN4",
        value: 124,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN5",
        value: 125,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN6",
        value: 126,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_CAPIN7",
        value: 127,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP0_7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT2OPAMP1_7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_IN2",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_IN3",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_SYNCI",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_IN2",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_IN3",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_SYNCI",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_IN2",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_IN3",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_SYNCI",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_IN2",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_IN3",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_SYNCI",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_CMP0_WIN",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_CMP1_WIN",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC0_BUFTRG",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DAC1_BUFTRG",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC0_STRGI",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADC1_STRGI",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0A",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0B",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI0C",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI1A",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI1B",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI1C",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI2A",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI2B",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI2C",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI3A",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI3B",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ADCX_PTRGI3C",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MCAN_PTPC0_CAP",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MCAN_PTPC1_CAP",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO0_TRIG_IN0",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO0_TRIG_IN1",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO1_TRIG_IN0",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO1_TRIG_IN1",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN0",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN1",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN2",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN3",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN4",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN5",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN6",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SEI_TRIG_IN7",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MMC0_TRIG_IN0",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MMC0_TRIG_IN1",
        value: 61,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MMC1_TRIG_IN0",
        value: 62,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MMC1_TRIG_IN1",
        value: 63,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_00",
        value: 64,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_01",
        value: 65,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_02",
        value: 66,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_03",
        value: 67,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_04",
        value: 68,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_05",
        value: 69,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_06",
        value: 70,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_07",
        value: 71,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_08",
        value: 72,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_09",
        value: 73,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_10",
        value: 74,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_11",
        value: 75,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_12",
        value: 76,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_13",
        value: 77,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_14",
        value: 78,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_15",
        value: 79,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_16",
        value: 80,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_17",
        value: 81,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_18",
        value: 82,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_19",
        value: 83,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_20",
        value: 84,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_21",
        value: 85,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_22",
        value: 86,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_23",
        value: 87,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_24",
        value: 88,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_25",
        value: 89,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_26",
        value: 90,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_27",
        value: 91,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_28",
        value: 92,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_29",
        value: 93,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_30",
        value: 94,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_31",
        value: 95,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO0",
        value: 96,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO1",
        value: 97,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO2",
        value: 98,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO3",
        value: 99,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO4",
        value: 100,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO5",
        value: 101,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO6",
        value: 102,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_MOT_GPIO7",
        value: 103,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN8",
        value: 104,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN9",
        value: 105,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN10",
        value: 106,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN11",
        value: 107,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN12",
        value: 108,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN13",
        value: 109,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN14",
        value: 110,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM_IN15",
        value: 111,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FRCI",
        value: 112,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FRCSYNCI",
        value: 113,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_SYNCI",
        value: 114,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_SHRLDSYNCI",
        value: 115,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI0",
        value: 116,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_FAULTI1",
        value: 117,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_FRCI",
        value: 118,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_FRCSYNCI",
        value: 119,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_SYNCI",
        value: 120,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_SHRLDSYNCI",
        value: 121,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_FAULTI0",
        value: 122,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_FAULTI1",
        value: 123,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_RDC_TRIG_IN0",
        value: 124,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_RDC_TRIG_IN1",
        value: 125,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SYNCTIMER_TRIG",
        value: 126,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_TRIG_IN",
        value: 127,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI1_TRIG_IN",
        value: 128,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_PAUSE",
        value: 129,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI1_PAUSE",
        value: 130,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_UART_TRIG0",
        value: 131,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_UART_TRIG1",
        value: 132,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM_IRQ0",
        value: 133,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM_IRQ1",
        value: 134,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM_DMA0",
        value: 135,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM_DMA1",
        value: 136,
    },
];
#[path = "../registers/acmp_common.rs"]
pub mod acmp;
#[path = "../registers/adc16_v53.rs"]
pub mod adc16;
#[path = "../registers/crc_common.rs"]
pub mod crc;
#[path = "../registers/dac_v53.rs"]
pub mod dac;
#[path = "../registers/dma_v53.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/gpio_v53.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v53.rs"]
pub mod gpiom;
#[path = "../registers/i2c_v53.rs"]
pub mod i2c;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/keym_common.rs"]
pub mod keym;
#[path = "../registers/mbx_common.rs"]
pub mod mbx;
#[path = "../registers/mcan_v53.rs"]
pub mod mcan;
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/mmc_v53.rs"]
pub mod mmc;
#[path = "../registers/opamp_v53.rs"]
pub mod opamp;
#[path = "../registers/otp_common.rs"]
pub mod otp;
#[path = "../registers/pcfg_v53.rs"]
pub mod pcfg;
#[path = "../registers/pdgo_v53.rs"]
pub mod pdgo;
#[path = "../registers/plb_v53.rs"]
pub mod plb;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../registers/pmon_common.rs"]
pub mod pmon;
#[path = "../registers/ppor_v53.rs"]
pub mod ppor;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwm_v53.rs"]
pub mod pwm;
#[path = "../registers/qei_v53.rs"]
pub mod qei;
#[path = "../registers/qeo_v53.rs"]
pub mod qeo;
#[path = "../registers/rdc_v53.rs"]
pub mod rdc;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/sec_common.rs"]
pub mod sec;
#[path = "../registers/sei_v53.rs"]
pub mod sei;
#[path = "../registers/spi_v53.rs"]
pub mod spi;
#[path = "../registers/synt_v53.rs"]
pub mod synt;
#[path = "../registers/sysctl_v53.rs"]
pub mod sysctl;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/trgm_v53.rs"]
pub mod trgm;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/uart_v53.rs"]
pub mod uart;
#[path = "../registers/usb_v53.rs"]
pub mod usb;
#[path = "../registers/wdg_v53.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
