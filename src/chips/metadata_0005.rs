
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
        name: "PPI",
        address: 0xf00cc000,
        registers: Some(PeripheralRegisters {
            kind: "ppi",
            version: "v6e",
            block: "PPI",
            ir: &ppi::REGISTERS,
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
            interrupt: "PPI",
        }],
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
            group_bit_offset: 6,
            resource_clock_top: None,
            resource: 294,
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
        name: "HDMA",
        address: 0xf00c8000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v6e",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
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
            group_bit_offset: 2,
            resource_clock_top: None,
            resource: 290,
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 2,
            resource_clock_top: None,
            resource: 290,
            clock_node: None,
        }),
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
            group_link: 0,
            group_bit_offset: 30,
            resource_clock_top: None,
            resource: 286,
            clock_node: None,
        }),
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWM0",
        address: 0xf0420000,
        registers: Some(PeripheralRegisters {
            kind: "pwmv2",
            version: "v6e",
            block: "PWMV2",
            ir: &pwmv2::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 21,
            resource_clock_top: None,
            resource: 309,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "P2",
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
                pin: "PA03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF07",
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
        address: 0xf0424000,
        registers: Some(PeripheralRegisters {
            kind: "pwmv2",
            version: "v6e",
            block: "PWMV2",
            ir: &pwmv2::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 22,
            resource_clock_top: None,
            resource: 310,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "P0",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "P1",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "P2",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "P3",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "P4",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "P5",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "P6",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "P7",
                alt: Some(16),
            },
            PeripheralPin {
                pin: "PF15",
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
        address: 0xf0464000,
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
            group_link: 1,
            group_bit_offset: 15,
            resource_clock_top: None,
            resource: 303,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB07",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD04",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD05",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PF05",
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
        address: 0xf0404000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
            block: "QEI",
            ir: &qei::REGISTERS,
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
                pin: "PA30",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB30",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD30",
                signal: "A",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB31",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD31",
                signal: "B",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA27",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB27",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD27",
                signal: "F",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA28",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB28",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD28",
                signal: "H0",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD26",
                signal: "H1",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "Z",
                alt: Some(20),
            },
            PeripheralPin {
                pin: "PD29",
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
        address: 0xf0410000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
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
        name: "QEO1",
        address: 0xf0414000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
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
        name: "PLB",
        address: 0xf0460000,
        registers: Some(PeripheralRegisters {
            kind: "plb",
            version: "v6e",
            block: "PLB",
            ir: &plb::REGISTERS,
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
            group_bit_offset: 6,
            resource_clock_top: Some(70),
            resource: 262,
            clock_node: Some(6),
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
                pin: "PC10",
                signal: "COMP2",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(35),
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
            group_bit_offset: 7,
            resource_clock_top: Some(71),
            resource: 263,
            clock_node: Some(7),
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
                pin: "PC13",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(38),
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
            group_bit_offset: 8,
            resource_clock_top: Some(72),
            resource: 264,
            clock_node: Some(8),
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
                pin: "PA31",
                signal: "COMP3",
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
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(41),
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
            group_bit_offset: 9,
            resource_clock_top: Some(73),
            resource: 265,
            clock_node: Some(9),
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
                pin: "PA29",
                signal: "COMP3",
                alt: Some(1),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "COMP3",
                alt: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(44),
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 30,
            resource_clock_top: Some(102),
            resource: 318,
            clock_node: Some(38),
        }),
        pins: &[],
        dma_channels: &[],
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
            version: "v6e",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PTMR",
        }],
    },
    Peripheral {
        name: "EWDG0",
        address: 0xf00b0000,
        registers: Some(PeripheralRegisters {
            kind: "ewdg",
            version: "v53",
            block: "EWDG",
            ir: &ewdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "EWDG0",
        }],
    },
    Peripheral {
        name: "EWDG1",
        address: 0xf00b4000,
        registers: Some(PeripheralRegisters {
            kind: "ewdg",
            version: "v53",
            block: "EWDG",
            ir: &ewdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "EWDG1",
        }],
    },
    Peripheral {
        name: "PEWDG",
        address: 0xf4128000,
        registers: Some(PeripheralRegisters {
            kind: "ewdg",
            version: "v53",
            block: "EWDG",
            ir: &ewdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PEWDG",
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
            group_bit_offset: 22,
            resource_clock_top: Some(84),
            resource: 278,
            clock_node: Some(20),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA03",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CTS",
                alt: Some(3),
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
                pin: "PC02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE02",
                signal: "RTS",
                alt: Some(3),
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
                pin: "PC00",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE00",
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
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 23,
            resource_clock_top: Some(85),
            resource: 279,
            clock_node: Some(21),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(10),
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
            group_bit_offset: 24,
            resource_clock_top: Some(86),
            resource: 280,
            clock_node: Some(22),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC11",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA08",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC08",
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
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 25,
            resource_clock_top: Some(87),
            resource: 281,
            clock_node: Some(23),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC15",
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
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 26,
            resource_clock_top: Some(88),
            resource: 282,
            clock_node: Some(24),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "TXD",
                alt: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(17),
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
            group_bit_offset: 27,
            resource_clock_top: Some(89),
            resource: 283,
            clock_node: Some(25),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC20",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC21",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC22",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC23",
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
            version: "v53",
            block: "UART",
            ir: &uart::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 28,
            resource_clock_top: Some(90),
            resource: 284,
            clock_node: Some(26),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC26",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA24",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC24",
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
            group_link: 0,
            group_bit_offset: 29,
            resource_clock_top: Some(91),
            resource: 285,
            clock_node: Some(27),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "CTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "DE",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "RTS",
                alt: Some(3),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "RXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "TXD",
                alt: Some(2),
            },
            PeripheralPin {
                pin: "PC31",
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
            group_bit_offset: 18,
            resource_clock_top: Some(80),
            resource: 274,
            clock_node: Some(16),
        }),
        pins: &[
            PeripheralPin {
                pin: "PC05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC04",
                signal: "SCLK",
                alt: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(0),
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
            group_bit_offset: 19,
            resource_clock_top: Some(81),
            resource: 275,
            clock_node: Some(17),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA05",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC09",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC08",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA06",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA07",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA04",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC10",
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
        address: 0xf0078000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 20,
            resource_clock_top: Some(82),
            resource: 276,
            clock_node: Some(18),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA21",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC27",
                signal: "CS0",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC25",
                signal: "CS1",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC24",
                signal: "CS2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CS3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC30",
                signal: "DAT2",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC31",
                signal: "DAT3",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC28",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC26",
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
        address: 0xf007c000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v53",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 21,
            resource_clock_top: Some(83),
            resource: 277,
            clock_node: Some(19),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA27",
                signal: "CS0",
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
                pin: "PC22",
                signal: "MISO",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA29",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC23",
                signal: "MOSI",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PA26",
                signal: "SCLK",
                alt: Some(5),
            },
            PeripheralPin {
                pin: "PC20",
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
        address: 0xf0060000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v53",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 14,
            resource_clock_top: Some(76),
            resource: 270,
            clock_node: Some(12),
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
                pin: "PF09",
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
            group_bit_offset: 15,
            resource_clock_top: Some(77),
            resource: 271,
            clock_node: Some(13),
        }),
        pins: &[
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
                pin: "PF13",
                signal: "SCL",
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
                pin: "PF12",
                signal: "SDA",
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
            group_bit_offset: 16,
            resource_clock_top: Some(78),
            resource: 272,
            clock_node: Some(14),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD02",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE02",
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
                pin: "PB03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF03",
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
            group_bit_offset: 17,
            resource_clock_top: Some(79),
            resource: 273,
            clock_node: Some(15),
        }),
        pins: &[
            PeripheralPin {
                pin: "PA07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PC07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD07",
                signal: "SCL",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF07",
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
                pin: "PC06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PD06",
                signal: "SDA",
                alt: Some(4),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "SDA",
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 10,
            resource_clock_top: Some(92),
            resource: 298,
            clock_node: Some(28),
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN0",
            dmamux: Some("DMAMUX"),
            request: Some(28),
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 11,
            resource_clock_top: Some(93),
            resource: 299,
            clock_node: Some(29),
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN1",
            dmamux: Some("DMAMUX"),
            request: Some(29),
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 12,
            resource_clock_top: Some(94),
            resource: 300,
            clock_node: Some(30),
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
                pin: "PF09",
                signal: "RXD",
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
                pin: "PF10",
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
                pin: "PF08",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN2",
            dmamux: Some("DMAMUX"),
            request: Some(30),
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
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 13,
            resource_clock_top: Some(95),
            resource: 301,
            clock_node: Some(31),
        }),
        pins: &[
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
                pin: "PF14",
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
                pin: "PD15",
                signal: "TXD",
                alt: Some(7),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "TXD",
                alt: Some(7),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "MCAN3",
            dmamux: Some("DMAMUX"),
            request: Some(31),
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
            group_link: 1,
            group_bit_offset: 14,
            resource_clock_top: None,
            resource: 302,
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
        address: 0xf1420000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v53",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 29,
            resource_clock_top: None,
            resource: 317,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PE00",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "ID",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE03",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "OC",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PE01",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "PWR",
                alt: Some(24),
            },
            PeripheralPin {
                pin: "PF19",
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
            group_bit_offset: 7,
            resource_clock_top: Some(105),
            resource: 295,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF27",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "IN02",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "IN06",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "IN15",
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
            group_bit_offset: 8,
            resource_clock_top: Some(106),
            resource: 296,
            clock_node: None,
        }),
        pins: &[
            PeripheralPin {
                pin: "PF27",
                signal: "IN00",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "IN01",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "IN02",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "IN03",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "IN04",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "IN05",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "IN06",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "IN07",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "IN08",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "IN09",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "IN10",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "IN11",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "IN12",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "IN13",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "IN14",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "IN15",
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
                pin: "PF30",
                signal: "CMP0_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "CMP0_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "CMP0_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "CMP0_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "CMP0_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "CMP0_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF16",
                signal: "CMP0_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "CMP0_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "CMP0_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "CMP0_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "CMP0_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "CMP0_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "CMP0_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF19",
                signal: "CMP0_INP7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF30",
                signal: "CMP1_INN1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF28",
                signal: "CMP1_INN2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF23",
                signal: "CMP1_INN3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "CMP1_INN4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "CMP1_INN5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF26",
                signal: "CMP1_INN6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF17",
                signal: "CMP1_INN7",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF31",
                signal: "CMP1_INP1",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF29",
                signal: "CMP1_INP2",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF25",
                signal: "CMP1_INP3",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "CMP1_INP4",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF24",
                signal: "CMP1_INP5",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF27",
                signal: "CMP1_INP6",
                alt: Some(0),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "CMP1_INP7",
                alt: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(60),
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
        name: "TSNS",
        address: 0xf0090000,
        registers: Some(PeripheralRegisters {
            kind: "tsns",
            version: "common",
            block: "TSNS",
            ir: &tsns::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 0,
            group_bit_offset: 31,
            resource_clock_top: None,
            resource: 287,
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
            group_link: 1,
            group_bit_offset: 19,
            resource_clock_top: None,
            resource: 307,
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
        name: "ENET0",
        address: 0xf1400000,
        registers: Some(PeripheralRegisters {
            kind: "enet",
            version: "v68",
            block: "ENET",
            ir: &enet::REGISTERS,
        }),
        sysctl: Some(PeripheralSysctl {
            group_link: 1,
            group_bit_offset: 28,
            resource_clock_top: Some(98),
            resource: 316,
            clock_node: Some(34),
        }),
        pins: &[
            PeripheralPin {
                pin: "PE04",
                signal: "COL",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PE05",
                signal: "CRS",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB24",
                signal: "EVTI0",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PF21",
                signal: "EVTI0",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB26",
                signal: "EVTI1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PE04",
                signal: "EVTI1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PF22",
                signal: "EVTI1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB25",
                signal: "EVTO0",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PF18",
                signal: "EVTO0",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PB29",
                signal: "EVTO1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PE00",
                signal: "EVTO1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PF20",
                signal: "EVTO1",
                alt: Some(25),
            },
            PeripheralPin {
                pin: "PA31",
                signal: "MDC",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF00",
                signal: "MDC",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA30",
                signal: "MDIO",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF01",
                signal: "MDIO",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB05",
                signal: "RXCK",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "RXCK",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB01",
                signal: "RXD0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF09",
                signal: "RXD0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB02",
                signal: "RXD1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "RXD1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB03",
                signal: "RXD2",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "RXD2",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB04",
                signal: "RXD3",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "RXD3",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB00",
                signal: "RXDV",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF08",
                signal: "RXDV",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "RXER",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF14",
                signal: "RXER",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB06",
                signal: "TXCK",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF02",
                signal: "TXCK",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB07",
                signal: "TXD0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF03",
                signal: "TXD0",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB08",
                signal: "TXD1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF04",
                signal: "TXD1",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB09",
                signal: "TXD2",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF05",
                signal: "TXD2",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TXD3",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF06",
                signal: "TXD3",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TXEN",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF07",
                signal: "TXEN",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "TXER",
                alt: Some(18),
            },
            PeripheralPin {
                pin: "PF15",
                signal: "TXER",
                alt: Some(18),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ENET0",
        }],
    },
    Peripheral {
        name: "ESC",
        address: 0xf1700000,
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
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "SYNC1",
                dmamux: Some("DMAMUX"),
                request: Some(65),
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
        name: "PMIC",
        index: 20,
    },
    Resource {
        name: "POW_CPU0",
        index: 21,
    },
    Resource {
        name: "RST_SOC",
        index: 23,
    },
    Resource {
        name: "RST_CPU0",
        index: 24,
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
        name: "CLK_TOP_AHB0",
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
        name: "CLK_TOP_TMR0",
        index: 70,
    },
    Resource {
        name: "CLK_TOP_TMR1",
        index: 71,
    },
    Resource {
        name: "CLK_TOP_TMR2",
        index: 72,
    },
    Resource {
        name: "CLK_TOP_TMR3",
        index: 73,
    },
    Resource {
        name: "CLK_TOP_OWR0",
        index: 74,
    },
    Resource {
        name: "CLK_TOP_OWR1",
        index: 75,
    },
    Resource {
        name: "CLK_TOP_I2C0",
        index: 76,
    },
    Resource {
        name: "CLK_TOP_I2C1",
        index: 77,
    },
    Resource {
        name: "CLK_TOP_I2C2",
        index: 78,
    },
    Resource {
        name: "CLK_TOP_I2C3",
        index: 79,
    },
    Resource {
        name: "CLK_TOP_SPI0",
        index: 80,
    },
    Resource {
        name: "CLK_TOP_SPI1",
        index: 81,
    },
    Resource {
        name: "CLK_TOP_SPI2",
        index: 82,
    },
    Resource {
        name: "CLK_TOP_SPI3",
        index: 83,
    },
    Resource {
        name: "CLK_TOP_URT0",
        index: 84,
    },
    Resource {
        name: "CLK_TOP_URT1",
        index: 85,
    },
    Resource {
        name: "CLK_TOP_URT2",
        index: 86,
    },
    Resource {
        name: "CLK_TOP_URT3",
        index: 87,
    },
    Resource {
        name: "CLK_TOP_URT4",
        index: 88,
    },
    Resource {
        name: "CLK_TOP_URT5",
        index: 89,
    },
    Resource {
        name: "CLK_TOP_URT6",
        index: 90,
    },
    Resource {
        name: "CLK_TOP_URT7",
        index: 91,
    },
    Resource {
        name: "CLK_TOP_CAN0",
        index: 92,
    },
    Resource {
        name: "CLK_TOP_CAN1",
        index: 93,
    },
    Resource {
        name: "CLK_TOP_CAN2",
        index: 94,
    },
    Resource {
        name: "CLK_TOP_CAN3",
        index: 95,
    },
    Resource {
        name: "CLK_TOP_XPI0",
        index: 96,
    },
    Resource {
        name: "CLK_TOP_ESC0",
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
        name: "CLK_TOP_ANA0",
        index: 103,
    },
    Resource {
        name: "CLK_TOP_ANA1",
        index: 104,
    },
    Resource {
        name: "CLK_TOP_ADC0",
        index: 105,
    },
    Resource {
        name: "CLK_TOP_ADC1",
        index: 106,
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
        name: "ROM0",
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
        name: "TMR0",
        index: 262,
    },
    Resource {
        name: "TMR1",
        index: 263,
    },
    Resource {
        name: "TMR2",
        index: 264,
    },
    Resource {
        name: "TMR3",
        index: 265,
    },
    Resource {
        name: "OWR0",
        index: 266,
    },
    Resource {
        name: "OWR1",
        index: 267,
    },
    Resource {
        name: "EUI0",
        index: 268,
    },
    Resource {
        name: "EUI1",
        index: 269,
    },
    Resource {
        name: "I2C0",
        index: 270,
    },
    Resource {
        name: "I2C1",
        index: 271,
    },
    Resource {
        name: "I2C2",
        index: 272,
    },
    Resource {
        name: "I2C3",
        index: 273,
    },
    Resource {
        name: "SPI0",
        index: 274,
    },
    Resource {
        name: "SPI1",
        index: 275,
    },
    Resource {
        name: "SPI2",
        index: 276,
    },
    Resource {
        name: "SPI3",
        index: 277,
    },
    Resource {
        name: "URT0",
        index: 278,
    },
    Resource {
        name: "URT1",
        index: 279,
    },
    Resource {
        name: "URT2",
        index: 280,
    },
    Resource {
        name: "URT3",
        index: 281,
    },
    Resource {
        name: "URT4",
        index: 282,
    },
    Resource {
        name: "URT5",
        index: 283,
    },
    Resource {
        name: "URT6",
        index: 284,
    },
    Resource {
        name: "URT7",
        index: 285,
    },
    Resource {
        name: "CRC0",
        index: 286,
    },
    Resource {
        name: "TSNS",
        index: 287,
    },
    Resource {
        name: "WDG0",
        index: 288,
    },
    Resource {
        name: "WDG1",
        index: 289,
    },
    Resource {
        name: "MBX0",
        index: 290,
    },
    Resource {
        name: "GPIO",
        index: 291,
    },
    Resource {
        name: "PPI0",
        index: 292,
    },
    Resource {
        name: "HDMA",
        index: 293,
    },
    Resource {
        name: "LOBS",
        index: 294,
    },
    Resource {
        name: "ADC0",
        index: 295,
    },
    Resource {
        name: "ADC1",
        index: 296,
    },
    Resource {
        name: "CMP0",
        index: 297,
    },
    Resource {
        name: "CAN0",
        index: 298,
    },
    Resource {
        name: "CAN1",
        index: 299,
    },
    Resource {
        name: "CAN2",
        index: 300,
    },
    Resource {
        name: "CAN3",
        index: 301,
    },
    Resource {
        name: "PTPC",
        index: 302,
    },
    Resource {
        name: "QEI0",
        index: 303,
    },
    Resource {
        name: "QEI1",
        index: 304,
    },
    Resource {
        name: "QEO0",
        index: 305,
    },
    Resource {
        name: "QEO1",
        index: 306,
    },
    Resource {
        name: "SDM0",
        index: 307,
    },
    Resource {
        name: "PLB0",
        index: 308,
    },
    Resource {
        name: "PWM0",
        index: 309,
    },
    Resource {
        name: "PWM1",
        index: 310,
    },
    Resource {
        name: "EMDS",
        index: 311,
    },
    Resource {
        name: "XPI0",
        index: 312,
    },
    Resource {
        name: "RAM0",
        index: 313,
    },
    Resource {
        name: "XDMA",
        index: 314,
    },
    Resource {
        name: "ESC0",
        index: 315,
    },
    Resource {
        name: "ETH0",
        index: 316,
    },
    Resource {
        name: "USB0",
        index: 317,
    },
    Resource {
        name: "NTM0",
        index: 318,
    },
    Resource {
        name: "REF0",
        index: 319,
    },
    Resource {
        name: "REF1",
        index: 320,
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
        name: "AHB0",
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
        name: "TMR0",
        index: 6,
    },
    Clock {
        name: "TMR1",
        index: 7,
    },
    Clock {
        name: "TMR2",
        index: 8,
    },
    Clock {
        name: "TMR3",
        index: 9,
    },
    Clock {
        name: "OWR0",
        index: 10,
    },
    Clock {
        name: "OWR1",
        index: 11,
    },
    Clock {
        name: "I2C0",
        index: 12,
    },
    Clock {
        name: "I2C1",
        index: 13,
    },
    Clock {
        name: "I2C2",
        index: 14,
    },
    Clock {
        name: "I2C3",
        index: 15,
    },
    Clock {
        name: "SPI0",
        index: 16,
    },
    Clock {
        name: "SPI1",
        index: 17,
    },
    Clock {
        name: "SPI2",
        index: 18,
    },
    Clock {
        name: "SPI3",
        index: 19,
    },
    Clock {
        name: "URT0",
        index: 20,
    },
    Clock {
        name: "URT1",
        index: 21,
    },
    Clock {
        name: "URT2",
        index: 22,
    },
    Clock {
        name: "URT3",
        index: 23,
    },
    Clock {
        name: "URT4",
        index: 24,
    },
    Clock {
        name: "URT5",
        index: 25,
    },
    Clock {
        name: "URT6",
        index: 26,
    },
    Clock {
        name: "URT7",
        index: 27,
    },
    Clock {
        name: "CAN0",
        index: 28,
    },
    Clock {
        name: "CAN1",
        index: 29,
    },
    Clock {
        name: "CAN2",
        index: 30,
    },
    Clock {
        name: "CAN3",
        index: 31,
    },
    Clock {
        name: "XPI0",
        index: 32,
    },
    Clock {
        name: "ESC0",
        index: 33,
    },
    Clock {
        name: "ETH0",
        index: 34,
    },
    Clock {
        name: "PTP0",
        index: 35,
    },
    Clock {
        name: "REF0",
        index: 36,
    },
    Clock {
        name: "REF1",
        index: 37,
    },
    Clock {
        name: "NTM0",
        index: 38,
    },
    Clock {
        name: "ANA0",
        index: 39,
    },
    Clock {
        name: "ANA1",
        index: 40,
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
pub(crate) static TRGMMUX: &[TrgmMux] = &[
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM0_3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_0",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_1",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_2",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_PWM1_3",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_QEI0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_QEI1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_TRGM_0",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_DMA_SRC_TRGM_1",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_0",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_1",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_2",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_3",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_4",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_5",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_6",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM0_P_7",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_0",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_1",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_2",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_3",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_4",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_5",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_6",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_PWM1_P_7",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P00",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P01",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P02",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P03",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P04",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P05",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P06",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P07",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P08",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P09",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P10",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P11",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P12",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P13",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P14",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_FILTER_SRC_TRGM0_P15",
        value: 31,
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
        name: "TRGM0_INPUT_SRC_PTPC_CMP0",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PTPC_CMP1",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_USB0_SOF",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_DEBUG_FLAG",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ESC_SYNC1",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ESC_SYNC0",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH00",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH01",
        value: 9,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH02",
        value: 10,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH03",
        value: 11,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH04",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH05",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH06",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SYNT_CH07",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT2",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR0_OUT3",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT2",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR1_OUT3",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR2_OUT2",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR2_OUT3",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR3_OUT2",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_GPTMR3_OUT3",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_0",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_1",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_2",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_3",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_4",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_5",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_6",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM0_TRGO_7",
        value: 31,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_0",
        value: 32,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_1",
        value: 33,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_2",
        value: 34,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_3",
        value: 35,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_4",
        value: 36,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_5",
        value: 37,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_6",
        value: 38,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PWM1_TRGO_7",
        value: 39,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P00",
        value: 40,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P01",
        value: 41,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P02",
        value: 42,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P03",
        value: 43,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P04",
        value: 44,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P05",
        value: 45,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P06",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P07",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P08",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P09",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P10",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P11",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P12",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P13",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P14",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_TRGM0_P15",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHZ0",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHZ1",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHZ2",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHZ3",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPL0",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPL1",
        value: 61,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPL2",
        value: 62,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPL3",
        value: 63,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHA0",
        value: 64,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHA1",
        value: 65,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHA2",
        value: 66,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_COMPHA3",
        value: 67,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT00",
        value: 68,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT01",
        value: 69,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT02",
        value: 70,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT03",
        value: 71,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT04",
        value: 72,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT05",
        value: 73,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT06",
        value: 74,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT07",
        value: 75,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT08",
        value: 76,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT09",
        value: 77,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT10",
        value: 78,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT11",
        value: 79,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT12",
        value: 80,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT13",
        value: 81,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT14",
        value: 82,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT15",
        value: 83,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT16",
        value: 84,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT17",
        value: 85,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT18",
        value: 86,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT19",
        value: 87,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT20",
        value: 88,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT21",
        value: 89,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT22",
        value: 90,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT23",
        value: 91,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT24",
        value: 92,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT25",
        value: 93,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT26",
        value: 94,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT27",
        value: 95,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT28",
        value: 96,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT29",
        value: 97,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT30",
        value: 98,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT31",
        value: 99,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT32",
        value: 100,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT33",
        value: 101,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT34",
        value: 102,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT35",
        value: 103,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT36",
        value: 104,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT37",
        value: 105,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT38",
        value: 106,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT39",
        value: 107,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT40",
        value: 108,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT41",
        value: 109,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT42",
        value: 110,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT43",
        value: 111,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT44",
        value: 112,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT45",
        value: 113,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT46",
        value: 114,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT47",
        value: 115,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT48",
        value: 116,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT49",
        value: 117,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT50",
        value: 118,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT51",
        value: 119,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT52",
        value: 120,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT53",
        value: 121,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT54",
        value: 122,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT55",
        value: 123,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT56",
        value: 124,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT57",
        value: 125,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT58",
        value: 126,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT59",
        value: 127,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT60",
        value: 128,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT61",
        value: 129,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT62",
        value: 130,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_PLB_OUT63",
        value: 131,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI0_TRIG_OUT",
        value: 132,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI1_TRIG_OUT",
        value: 133,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ADC0_TRIG_OUT",
        value: 134,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ADC1_TRIG_OUT",
        value: 135,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ACMP0_OUT_CH0",
        value: 136,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ACMP0_OUT_CH1",
        value: 137,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ENET0_PTP_3",
        value: 138,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI0_POS_VALID",
        value: 139,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEI1_POS_VALID",
        value: 140,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ADC0_ADC_VALID",
        value: 141,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_ADC1_ADC_VALID",
        value: 142,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_ADC0_VALID",
        value: 143,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_ADC1_VALID",
        value: 144,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_ADC2_VALID",
        value: 145,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_SDM0_ADC3_VALID",
        value: 146,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO0_DAC0_VALID",
        value: 147,
    },
    TrgmMux {
        name: "TRGM0_INPUT_SRC_QEO1_DAC0_VALID",
        value: 148,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P00",
        value: 0,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P01",
        value: 1,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P02",
        value: 2,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P03",
        value: 3,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P04",
        value: 4,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P05",
        value: 5,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P06",
        value: 6,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P07",
        value: 7,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P08",
        value: 8,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P09",
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
        name: "TRGM0_OUTPUT_SRC_TRGM0_P12",
        value: 12,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P13",
        value: 13,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P14",
        value: 14,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_TRGM0_P15",
        value: 15,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_0",
        value: 16,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_1",
        value: 17,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_2",
        value: 18,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_3",
        value: 19,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_4",
        value: 20,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_5",
        value: 21,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_6",
        value: 22,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM0_TRIG_IN_7",
        value: 23,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_0",
        value: 24,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_1",
        value: 25,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_2",
        value: 26,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_3",
        value: 27,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_4",
        value: 28,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_5",
        value: 29,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_6",
        value: 30,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PWM1_TRIG_IN_7",
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
        name: "TRGM0_OUTPUT_SRC_QEI0_TRIG_IN",
        value: 46,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI1_TRIG_IN",
        value: 47,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI0_PAUSE",
        value: 48,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEI1_PAUSE",
        value: 49,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO0_TRIG_IN_0",
        value: 50,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO0_TRIG_IN_1",
        value: 51,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO1_TRIG_IN_0",
        value: 52,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_QEO1_TRIG_IN_1",
        value: 53,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ACMP0_WIN_CH_0",
        value: 54,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ACMP0_WIN_CH_1",
        value: 55,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_CAPT_2",
        value: 56,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_CAPT_3",
        value: 57,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_CAPT_2",
        value: 58,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_CAPT_3",
        value: 59,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_CAPT_2",
        value: 60,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_CAPT_3",
        value: 61,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_CAPT_2",
        value: 62,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_CAPT_3",
        value: 63,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR0_SYNC",
        value: 64,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR1_SYNC",
        value: 65,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR2_SYNC",
        value: 66,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_GPTMR3_SYNC",
        value: 67,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PTPC_CAPT_0",
        value: 68,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PTPC_CAPT_1",
        value: 69,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_UART_TRIG0",
        value: 70,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_UART_TRIG1",
        value: 71,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC00",
        value: 72,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC01",
        value: 73,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC02",
        value: 74,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC03",
        value: 75,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC04",
        value: 76,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC05",
        value: 77,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC06",
        value: 78,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC07",
        value: 79,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC08",
        value: 80,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC09",
        value: 81,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC10",
        value: 82,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC11",
        value: 83,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC12",
        value: 84,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC13",
        value: 85,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC14",
        value: 86,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SDM0_PWM_SOC15",
        value: 87,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_00",
        value: 88,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_01",
        value: 89,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_02",
        value: 90,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_03",
        value: 91,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_04",
        value: 92,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_05",
        value: 93,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_06",
        value: 94,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_07",
        value: 95,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_08",
        value: 96,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_09",
        value: 97,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_10",
        value: 98,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_11",
        value: 99,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_12",
        value: 100,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_13",
        value: 101,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_14",
        value: 102,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_15",
        value: 103,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_16",
        value: 104,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_17",
        value: 105,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_18",
        value: 106,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_19",
        value: 107,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_20",
        value: 108,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_21",
        value: 109,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_22",
        value: 110,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_23",
        value: 111,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_24",
        value: 112,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_25",
        value: 113,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_26",
        value: 114,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_27",
        value: 115,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_28",
        value: 116,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_29",
        value: 117,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_30",
        value: 118,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_31",
        value: 119,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_32",
        value: 120,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_33",
        value: 121,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_34",
        value: 122,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_35",
        value: 123,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_36",
        value: 124,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_37",
        value: 125,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_38",
        value: 126,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_39",
        value: 127,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_40",
        value: 128,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_41",
        value: 129,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_42",
        value: 130,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_43",
        value: 131,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_44",
        value: 132,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_45",
        value: 133,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_46",
        value: 134,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_47",
        value: 135,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_48",
        value: 136,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_49",
        value: 137,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_50",
        value: 138,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_51",
        value: 139,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_52",
        value: 140,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_53",
        value: 141,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_54",
        value: 142,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_55",
        value: 143,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_56",
        value: 144,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_57",
        value: 145,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_58",
        value: 146,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_59",
        value: 147,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_60",
        value: 148,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_61",
        value: 149,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_62",
        value: 150,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_PLB_IN_63",
        value: 151,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_SYNT_SYNC",
        value: 152,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DCCS_TRIG",
        value: 153,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_IRQ_TRGM_0",
        value: 154,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_IRQ_TRGM_1",
        value: 155,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DMA_TRGM_0",
        value: 156,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_DMA_TRGM_1",
        value: 157,
    },
    TrgmMux {
        name: "TRGM0_OUTPUT_SRC_ESC_LATCH0",
        value: 158,
    },
];
#[path = "../registers/acmp_v6e.rs"]
pub mod acmp;
#[path = "../registers/adc16_v6e.rs"]
pub mod adc16;
#[path = "../registers/crc_common.rs"]
pub mod crc;
#[path = "../registers/dma_v6e.rs"]
pub mod dma;
#[path = "../registers/dmamux_common.rs"]
pub mod dmamux;
#[path = "../registers/enet_v68.rs"]
pub mod enet;
#[path = "../registers/esc_v6e.rs"]
pub mod esc;
#[path = "../registers/ewdg_v53.rs"]
pub mod ewdg;
#[path = "../registers/gpio_v53.rs"]
pub mod gpio;
#[path = "../registers/gpiom_v53.rs"]
pub mod gpiom;
#[path = "../registers/i2c_v53.rs"]
pub mod i2c;
#[path = "../registers/ioc_common.rs"]
pub mod ioc;
#[path = "../registers/lobs_v6e.rs"]
pub mod lobs;
#[path = "../registers/mbx_common.rs"]
pub mod mbx;
#[path = "../registers/mcan_v53.rs"]
pub mod mcan;
#[path = "../registers/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../registers/otp_common.rs"]
pub mod otp;
#[path = "../registers/pcfg_v6e.rs"]
pub mod pcfg;
#[path = "../registers/pdgo_v53.rs"]
pub mod pdgo;
#[path = "../registers/plb_v6e.rs"]
pub mod plb;
#[path = "../registers/plic_common.rs"]
pub mod plic;
#[path = "../registers/plicsw_common.rs"]
pub mod plicsw;
#[path = "../registers/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../registers/ppi_v6e.rs"]
pub mod ppi;
#[path = "../registers/ppor_v53.rs"]
pub mod ppor;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwmv2_v6e.rs"]
pub mod pwmv2;
#[path = "../registers/qei_v6e.rs"]
pub mod qei;
#[path = "../registers/qeo_v6e.rs"]
pub mod qeo;
#[path = "../registers/sdm_v6e.rs"]
pub mod sdm;
#[path = "../registers/spi_v53.rs"]
pub mod spi;
#[path = "../registers/synt_v53.rs"]
pub mod synt;
#[path = "../registers/sysctl_v6e.rs"]
pub mod sysctl;
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
