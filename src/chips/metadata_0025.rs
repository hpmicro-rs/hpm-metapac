
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
            group_bit_offset: 5,
            resource_clock_top: Some(65),
            resource: 261,
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
            version: "v68",
            block: "SYSCTL",
            ir: &sysctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SYSCTL",
            dmamux: Some("DMAMUX"),
            request: Some(115),
        }],
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
            version: "v68",
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
            version: "v68",
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
        address: 0xf00d0000,
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
                signal: "F",
                dmamux: Some("DMAMUX"),
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "D",
                dmamux: Some("DMAMUX"),
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "C",
                dmamux: Some("DMAMUX"),
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "Y",
                dmamux: Some("DMAMUX"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "B",
                dmamux: Some("DMAMUX"),
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "X",
                dmamux: Some("DMAMUX"),
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "Z",
                dmamux: Some("DMAMUX"),
                request: Some(9),
            },
            PeripheralDmaChannel {
                signal: "A",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "E",
                dmamux: Some("DMAMUX"),
                request: Some(5),
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
        name: "PGPIO",
        address: 0xf411c000,
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
            request: Some(105),
        }],
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
            version: "common",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "BGPIO",
            dmamux: Some("DMAMUX"),
            request: Some(112),
        }],
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
            version: "v68",
            block: "GPIOM",
            ir: &gpiom::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DDRCTL",
        address: 0xf3010000,
        registers: Some(PeripheralRegisters {
            kind: "ddrctl",
            version: "v68",
            block: "DDRCTL",
            ir: &ddrctl::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DDRPHY",
        address: 0xf4150000,
        registers: Some(PeripheralRegisters {
            kind: "ddrphy",
            version: "v68",
            block: "DDRPHY",
            ir: &ddrphy::REGISTERS,
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
        name: "XPI0",
        address: 0xf3000000,
        registers: Some(PeripheralRegisters {
            kind: "xpi",
            version: "dummy",
            block: "XPI",
            ir: &xpi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 8,
            resource_clock_top: Some(112),
            resource: 328,
            clock_node: Some(48),
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XPI0",
            dmamux: Some("DMAMUX"),
            request: Some(98),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "XPI0",
        }],
    },
    Peripheral {
        name: "SDXC0",
        address: 0xf1130000,
        registers: Some(PeripheralRegisters {
            kind: "sdxc",
            version: "v68",
            block: "SDXC",
            ir: &sdxc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDXC0",
            dmamux: Some("DMAMUX"),
            request: Some(95),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDXC0",
        }],
    },
    Peripheral {
        name: "SDXC1",
        address: 0xf1134000,
        registers: Some(PeripheralRegisters {
            kind: "sdxc",
            version: "v68",
            block: "SDXC",
            ir: &sdxc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDXC1",
            dmamux: Some("DMAMUX"),
            request: Some(96),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDXC1",
        }],
    },
    Peripheral {
        name: "FFA",
        address: 0xf3018000,
        registers: Some(PeripheralRegisters {
            kind: "ffa",
            version: "common",
            block: "FFA",
            ir: &ffa::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 9,
            resource_clock_top: None,
            resource: 329,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "FFA",
            dmamux: Some("DMAMUX"),
            request: Some(101),
        }],
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
            version: "v53",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 6,
            resource_clock_top: None,
            resource: 326,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "HDMA",
            dmamux: Some("DMAMUX"),
            request: Some(58),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HDMA",
        }],
    },
    Peripheral {
        name: "XDMA",
        address: 0xf3008000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v53",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 7,
            resource_clock_top: None,
            resource: 327,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "XDMA",
            dmamux: Some("DMAMUX"),
            request: Some(99),
        }],
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
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 302,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX0A",
            dmamux: Some("DMAMUX"),
            request: Some(53),
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
            request: Some(54),
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
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 303,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MBX1A",
            dmamux: Some("DMAMUX"),
            request: Some(55),
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
            request: Some(56),
        }],
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
            group_link: 0,
            group_bit_offset: 18,
            resource_clock_top: None,
            resource: 274,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2S0",
        address: 0xf0200000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 24,
            resource_clock_top: Some(135),
            resource: 312,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "I2S0",
            dmamux: Some("DMAMUX"),
            request: Some(63),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S0",
        }],
    },
    Peripheral {
        name: "I2S1",
        address: 0xf0204000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 25,
            resource_clock_top: Some(136),
            resource: 313,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB25",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "I2S1",
            dmamux: Some("DMAMUX"),
            request: Some(64),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S1",
        }],
    },
    Peripheral {
        name: "I2S2",
        address: 0xf0208000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 26,
            resource_clock_top: Some(137),
            resource: 314,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PC05",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "I2S2",
            dmamux: Some("DMAMUX"),
            request: Some(65),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S2",
        }],
    },
    Peripheral {
        name: "I2S3",
        address: 0xf020c000,
        registers: Some(PeripheralRegisters {
            kind: "i2s",
            version: "common",
            block: "I2S",
            ir: &i2s::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 27,
            resource_clock_top: Some(138),
            resource: 315,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PD24",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "BCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "FCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "MCLK",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "RXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "RXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "RXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "RXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "TXD0",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "TXD1",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "TXD2",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "TXD3",
                alt: Some(8),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "TXD3",
                alt: Some(8),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "I2S3",
            dmamux: Some("DMAMUX"),
            request: Some(66),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2S3",
        }],
    },
    Peripheral {
        name: "PDM",
        address: 0xf0214000,
        registers: Some(PeripheralRegisters {
            kind: "pdm",
            version: "common",
            block: "PDM",
            ir: &pdm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PDM",
            dmamux: Some("DMAMUX"),
            request: Some(68),
        }],
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAO",
            dmamux: Some("DMAMUX"),
            request: Some(67),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DAO",
        }],
    },
    Peripheral {
        name: "VAD",
        address: 0xf412c000,
        registers: Some(PeripheralRegisters {
            kind: "vad",
            version: "common",
            block: "VAD",
            ir: &vad::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "VAD",
            dmamux: Some("DMAMUX"),
            request: Some(104),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "VAD",
        }],
    },
    Peripheral {
        name: "SMIX",
        address: 0xf0218000,
        registers: Some(PeripheralRegisters {
            kind: "smix",
            version: "v68",
            block: "SMIX",
            ir: &smix::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 30,
            resource_clock_top: None,
            resource: 318,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ASRC",
                dmamux: Some("DMAMUX"),
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "DMA",
                dmamux: Some("DMAMUX"),
                request: Some(69),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "DMA",
                interrupt: "SMIX_DMA",
            },
            PeripheralInterrupt {
                signal: "ASRC",
                interrupt: "SMIX_ASRC",
            },
        ],
    },
    Peripheral {
        name: "GPTMR0",
        address: 0xf0080000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 16,
            resource_clock_top: Some(104),
            resource: 304,
            clock_node: Some(40),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "COMP0",
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
                pin: "PC08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR0",
            dmamux: Some("DMAMUX"),
            request: Some(43),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR0",
        }],
    },
    Peripheral {
        name: "GPTMR1",
        address: 0xf0084000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 17,
            resource_clock_top: Some(105),
            resource: 305,
            clock_node: Some(41),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "COMP2",
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
                pin: "PY05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR1",
            dmamux: Some("DMAMUX"),
            request: Some(44),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR1",
        }],
    },
    Peripheral {
        name: "GPTMR2",
        address: 0xf0088000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 18,
            resource_clock_top: Some(106),
            resource: 306,
            clock_node: Some(42),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "CAPT0",
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
                pin: "PE25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR2",
            dmamux: Some("DMAMUX"),
            request: Some(45),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR2",
        }],
    },
    Peripheral {
        name: "GPTMR3",
        address: 0xf008c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 19,
            resource_clock_top: Some(107),
            resource: 307,
            clock_node: Some(43),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "COMP1",
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
                pin: "PC21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR3",
            dmamux: Some("DMAMUX"),
            request: Some(46),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR3",
        }],
    },
    Peripheral {
        name: "GPTMR4",
        address: 0xf0090000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 20,
            resource_clock_top: Some(108),
            resource: 308,
            clock_node: Some(44),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR4",
            dmamux: Some("DMAMUX"),
            request: Some(47),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR4",
        }],
    },
    Peripheral {
        name: "GPTMR5",
        address: 0xf0094000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 21,
            resource_clock_top: Some(109),
            resource: 309,
            clock_node: Some(45),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR5",
            dmamux: Some("DMAMUX"),
            request: Some(48),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR5",
        }],
    },
    Peripheral {
        name: "GPTMR6",
        address: 0xf0098000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 22,
            resource_clock_top: Some(110),
            resource: 310,
            clock_node: Some(46),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD26",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR6",
            dmamux: Some("DMAMUX"),
            request: Some(49),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR6",
        }],
    },
    Peripheral {
        name: "GPTMR7",
        address: 0xf009c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 23,
            resource_clock_top: Some(111),
            resource: 311,
            clock_node: Some(47),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "CAPT0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "CAPT1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "CAPT2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "CAPT3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "COMP0",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "COMP1",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "COMP2",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GPTMR7",
            dmamux: Some("DMAMUX"),
            request: Some(50),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "GPTMR7",
        }],
    },
    Peripheral {
        name: "NTMR0",
        address: 0xf1110000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "NTMR0",
            dmamux: Some("DMAMUX"),
            request: Some(93),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "NTMR0",
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTMR",
            dmamux: Some("DMAMUX"),
            request: Some(107),
        }],
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
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
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
        name: "WDG1",
        address: 0xf00b4000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 13,
            resource_clock_top: None,
            resource: 301,
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
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PWDG",
            dmamux: Some("DMAMUX"),
            request: Some(106),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWDG",
        }],
    },
    Peripheral {
        name: "RTCSHW",
        address: 0xf421c000,
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
        address: 0xf4244000,
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
            request: Some(111),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 4,
            resource_clock_top: Some(96),
            resource: 292,
            clock_node: Some(32),
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
                pin: "PC03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF03",
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
                pin: "PC02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF02",
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
                pin: "PC02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF02",
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
                pin: "PC01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF01",
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
                pin: "PC00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY00",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART0",
            dmamux: Some("DMAMUX"),
            request: Some(27),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 5,
            resource_clock_top: Some(97),
            resource: 293,
            clock_node: Some(33),
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
                pin: "PC04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF04",
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
                pin: "PC05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF05",
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
                pin: "PC05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PF05",
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
                pin: "PC06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF06",
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
                pin: "PC07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART1",
            dmamux: Some("DMAMUX"),
            request: Some(28),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 6,
            resource_clock_top: Some(98),
            resource: 294,
            clock_node: Some(34),
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
                pin: "PC11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE11",
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
                pin: "PC10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE10",
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
                pin: "PC10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
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
                pin: "PC09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF09",
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
            PeripheralPin {
                pin: "PC08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART2",
            dmamux: Some("DMAMUX"),
            request: Some(29),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 7,
            resource_clock_top: Some(99),
            resource: 295,
            clock_node: Some(35),
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
                pin: "PC12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE12",
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
                pin: "PC13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
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
                pin: "PC13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "RTS",
                alt: Some(3),
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
                pin: "PB14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RXD",
                alt: Some(2),
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
                pin: "PB15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART3",
            dmamux: Some("DMAMUX"),
            request: Some(30),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 8,
            resource_clock_top: Some(100),
            resource: 296,
            clock_node: Some(36),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
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
                pin: "PD19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA18",
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
                pin: "PD18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
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
                pin: "PD18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE16",
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
            signal: "UART4",
            dmamux: Some("DMAMUX"),
            request: Some(31),
        }],
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 9,
            resource_clock_top: Some(101),
            resource: 297,
            clock_node: Some(37),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART5",
            dmamux: Some("DMAMUX"),
            request: Some(32),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 10,
            resource_clock_top: Some(102),
            resource: 298,
            clock_node: Some(38),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA25",
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
                pin: "PD25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
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
            PeripheralPin {
                pin: "PD24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART6",
            dmamux: Some("DMAMUX"),
            request: Some(33),
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 11,
            resource_clock_top: Some(103),
            resource: 299,
            clock_node: Some(39),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "RXD",
                alt: Some(2),
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
                pin: "PB31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UART7",
            dmamux: Some("DMAMUX"),
            request: Some(34),
        }],
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
            version: "v68",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PUART",
            dmamux: Some("DMAMUX"),
            request: Some(108),
        }],
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
            group_link: 1,
            group_bit_offset: 0,
            resource_clock_top: Some(92),
            resource: 288,
            clock_node: Some(28),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI0",
            dmamux: Some("DMAMUX"),
            request: Some(39),
        }],
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
            group_link: 1,
            group_bit_offset: 1,
            resource_clock_top: Some(93),
            resource: 289,
            clock_node: Some(29),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI1",
            dmamux: Some("DMAMUX"),
            request: Some(40),
        }],
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
            group_link: 1,
            group_bit_offset: 2,
            resource_clock_top: Some(94),
            resource: 290,
            clock_node: Some(30),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PY05",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI2",
            dmamux: Some("DMAMUX"),
            request: Some(41),
        }],
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
            group_link: 1,
            group_bit_offset: 3,
            resource_clock_top: Some(95),
            resource: 291,
            clock_node: Some(31),
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PF03",
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
                pin: "PB12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SPI3",
            dmamux: Some("DMAMUX"),
            request: Some(42),
        }],
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
            group_bit_offset: 28,
            resource_clock_top: Some(88),
            resource: 284,
            clock_node: Some(24),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE08",
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
                pin: "PB09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "SDA",
                alt: Some(4),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(35),
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
            group_bit_offset: 29,
            resource_clock_top: Some(89),
            resource: 285,
            clock_node: Some(25),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC12",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "GLOBAL",
            dmamux: Some("DMAMUX"),
            request: Some(36),
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
            group_bit_offset: 30,
            resource_clock_top: Some(90),
            resource: 286,
            clock_node: Some(26),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA24",
                signal: "SCL",
                alt: Some(4),
            },
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
                pin: "PD24",
                signal: "SCL",
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
                pin: "PB25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD25",
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
            request: Some(37),
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
            group_bit_offset: 31,
            resource_clock_top: Some(91),
            resource: 287,
            clock_node: Some(27),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE28",
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
            interrupt: "I2C3",
        }],
    },
    Peripheral {
        name: "MCAN0",
        address: 0xf0280000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 9,
            resource_clock_top: Some(72),
            resource: 265,
            clock_node: Some(8),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "RXD",
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
                pin: "PA02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PY02",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF00",
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
            signal: "MCAN0",
            dmamux: Some("DMAMUX"),
            request: Some(10),
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
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 10,
            resource_clock_top: Some(73),
            resource: 266,
            clock_node: Some(9),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "RXD",
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
                pin: "PA03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD03",
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
            PeripheralPin {
                pin: "PA05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF05",
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
            signal: "MCAN1",
            dmamux: Some("DMAMUX"),
            request: Some(11),
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
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 11,
            resource_clock_top: Some(74),
            resource: 267,
            clock_node: Some(10),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "STBY",
                alt: Some(7),
            },
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
                pin: "PE10",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE08",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN2",
            dmamux: Some("DMAMUX"),
            request: Some(12),
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
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 12,
            resource_clock_top: Some(75),
            resource: 268,
            clock_node: Some(11),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "STBY",
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
                pin: "PE13",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN3",
            dmamux: Some("DMAMUX"),
            request: Some(13),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN3",
        }],
    },
    Peripheral {
        name: "MCAN4",
        address: 0xf0290000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 13,
            resource_clock_top: Some(76),
            resource: 269,
            clock_node: Some(12),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ01",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ02",
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
                pin: "PC16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ00",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN4",
            dmamux: Some("DMAMUX"),
            request: Some(14),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN4",
        }],
    },
    Peripheral {
        name: "MCAN5",
        address: 0xf0294000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 14,
            resource_clock_top: Some(77),
            resource: 270,
            clock_node: Some(13),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE20",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ04",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ03",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PZ05",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN5",
            dmamux: Some("DMAMUX"),
            request: Some(15),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN5",
        }],
    },
    Peripheral {
        name: "MCAN6",
        address: 0xf0298000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 15,
            resource_clock_top: Some(78),
            resource: 271,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE25",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD24",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN6",
            dmamux: Some("DMAMUX"),
            request: Some(16),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN6",
        }],
    },
    Peripheral {
        name: "MCAN7",
        address: 0xf029c000,
        registers: Some(PeripheralRegisters {
            kind: "mcan",
            version: "v68",
            block: "MCAN",
            ir: &mcan::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 16,
            resource_clock_top: Some(79),
            resource: 272,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "RXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
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
                pin: "PA29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD29",
                signal: "STBY",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE29",
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
                pin: "PC31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN7",
            dmamux: Some("DMAMUX"),
            request: Some(17),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "MCAN7",
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
            group_bit_offset: 17,
            resource_clock_top: None,
            resource: 273,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "PTPC",
            dmamux: Some("DMAMUX"),
            request: Some(18),
        }],
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
            group_link: 2,
            group_bit_offset: 12,
            resource_clock_top: None,
            resource: 332,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE20",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "PWR",
                alt: Some(24),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "USB0",
            dmamux: Some("DMAMUX"),
            request: Some(94),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB0",
        }],
    },
    Peripheral {
        name: "ENET0",
        address: 0xf1100000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v68",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ENET0",
            dmamux: Some("DMAMUX"),
            request: Some(92),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET0",
        }],
    },
    Peripheral {
        name: "ADC0",
        address: 0xf00e0000,
        registers: Some(PeripheralRegisters {
            kind: "adc16",
            version: "v68",
            block: "ADC",
            ir: &adc16::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 3,
            resource_clock_top: Some(133),
            resource: 323,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE25",
                signal: "IN0",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE28",
                signal: "IN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE19",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE27",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE24",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE17",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE26",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE21",
                signal: "IN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE29",
                signal: "IN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE22",
                signal: "IN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE30",
                signal: "IN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE23",
                signal: "IN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE31",
                signal: "IN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE16",
                signal: "IN8",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PE18",
                signal: "IN9",
                alt: Some(0),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC0",
            dmamux: Some("DMAMUX"),
            request: Some(59),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC0",
        }],
    },
    Peripheral {
        name: "TSNS",
        address: 0xf4154000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 2,
            group_bit_offset: 10,
            resource_clock_top: None,
            resource: 330,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "TSNS",
            dmamux: Some("DMAMUX"),
            request: Some(103),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TSNS",
        }],
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
            group_link: 2,
            group_bit_offset: 0,
            resource_clock_top: None,
            resource: 320,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDP",
            dmamux: Some("DMAMUX"),
            request: Some(97),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDP",
        }],
    },
    Peripheral {
        name: "RNG",
        address: 0xf304c000,
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
            request: Some(57),
        }],
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
            group_link: 2,
            group_bit_offset: 1,
            resource_clock_top: None,
            resource: 321,
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
        address: 0xf3044000,
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
            request: Some(102),
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PSEC",
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
#[path = "../registers/adc16_v68.rs"]
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
#[path = "../registers/crc_common.rs"]
pub mod crc;
#[path = "../registers/dao_v68.rs"]
pub mod dao;
#[path = "../registers/ddrctl_v68.rs"]
pub mod ddrctl;
#[path = "../registers/ddrphy_v68.rs"]
pub mod ddrphy;
#[path = "../registers/dma_v53.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/enet_v68.rs"]
pub mod enet;
#[path = "../registers/ffa_common.rs"]
pub mod ffa;
#[path = "../registers/gpio_common.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v68.rs"]
pub mod gpiom;
#[path = "../registers/i2c_v53.rs"]
pub mod i2c;
#[path = "../registers/i2s_common.rs"]
pub mod i2s;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/keym_common.rs"]
pub mod keym;
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
#[path = "../registers/pcfg_v68.rs"]
pub mod pcfg;
#[path = "../registers/pdm_common.rs"]
pub mod pdm;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../registers/pmon_common.rs"]
pub mod pmon;
#[path = "../registers/ppor_v68.rs"]
pub mod ppor;
#[path = "../registers/psec_common.rs"]
pub mod psec;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/rng_common.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdp_v53.rs"]
pub mod sdp;
#[path = "../registers/sdxc_v68.rs"]
pub mod sdxc;
#[path = "../registers/smix_v68.rs"]
pub mod smix;
#[path = "../registers/spi_v53.rs"]
pub mod spi;
#[path = "../registers/sysctl_v68.rs"]
pub mod sysctl;
#[path = "../registers/tamp_v62.rs"]
pub mod tamp;
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/uart_v68.rs"]
pub mod uart;
#[path = "../registers/usb_v53.rs"]
pub mod usb;
#[path = "../registers/vad_common.rs"]
pub mod vad;
#[path = "../registers/wdg_v68.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
