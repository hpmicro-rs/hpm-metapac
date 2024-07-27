
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
                pin: "PB08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
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
                pin: "PA08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "COMP0",
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
            PeripheralPin {
                pin: "PB09",
                signal: "CAPT1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(1),
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
                pin: "PB00",
                signal: "COMP0",
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
                pin: "PB01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CAPT1",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(5),
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
                pin: "PY03",
                signal: "COMP3",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "CAPT0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP2",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "COMP1",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "CAPT1",
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
        interrupts: &[],
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
                pin: "PB00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA03",
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
                pin: "PA02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
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
                pin: "PB04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA04",
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
                pin: "PA07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RXD",
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
                pin: "PB11",
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
                pin: "PA08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PA15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CTS",
                alt: Some(3),
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
            PeripheralPin {
                pin: "PB15",
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
                pin: "PY00",
                signal: "TXD",
                alt: Some(1),
            },
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
                pin: "PA07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "MISO",
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
                pin: "PA29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "MISO",
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
                pin: "PA31",
                signal: "DAT3",
                alt: Some(5),
            },
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
                pin: "PB09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "MOSI",
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
                pin: "PA23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MOSI",
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
                pin: "PA15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DAT2",
                alt: Some(5),
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
                pin: "PA03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "SCL",
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
                pin: "PB07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "SCL",
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
                pin: "PA25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "SDA",
                alt: Some(4),
            },
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
                pin: "PB13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                alt: Some(4),
            },
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
                pin: "PB12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "SCL",
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
                pin: "PY02",
                signal: "PWR",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "OC",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "OC",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "PWR",
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
                pin: "PA30",
                signal: "PWR",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "ID",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "OC",
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
                pin: "PB14",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "IN9",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "IN0",
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
                pin: "PB01",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
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
                pin: "PB10",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "COMP1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "CMP1_INN3",
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
                pin: "PB11",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "COMP1",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "COMP0",
                alt: Some(19),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "COMP1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(17),
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
                pin: "PB12",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "COMP0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "COMP1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "COMP1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "COMP0",
                alt: Some(17),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "COMP1",
                alt: Some(16),
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
                pin: "PA23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "COMP1",
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
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(11),
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
                pin: "PY02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "COMP2",
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
                pin: "PA19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "COMP2",
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
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(13),
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
                pin: "PA18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
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
                pin: "PA17",
                signal: "RXD",
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
                pin: "PA23",
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
                pin: "PA22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(32),
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
                pin: "PA31",
                signal: "TXD",
                alt: Some(2),
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
                pin: "PA28",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PB06",
                signal: "IN9",
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
                pin: "PB15",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "IN15",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "IN0",
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
                pin: "PB00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "STBY",
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
                pin: "PA19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "RXD",
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
                pin: "PB04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA21",
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
                pin: "PY02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
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
                pin: "PA29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
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
        name: "TRGMUX0",
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
        name: "TRGMUX1",
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
        name: "PEWDG",
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

#[path = "../registers/acmp_common.rs"]
pub mod acmp;
#[path = "../registers/adc16_v53.rs"]
pub mod adc16;
#[path = "../registers/crc_common.rs"]
pub mod crc;
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
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/sec_common.rs"]
pub mod sec;
#[path = "../registers/spi_v53.rs"]
pub mod spi;
#[path = "../registers/sysctl_v53.rs"]
pub mod sysctl;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
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
