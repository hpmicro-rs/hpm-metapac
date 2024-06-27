
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
            version: "v6e",
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
        sysctl: None,
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
        sysctl: None,
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
            version: "v6e",
            block: "FEMC",
            ir: &femc::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
        sysctl: None,
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
        name: "CRC",
        address: 0xf00c0000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "common",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(67),
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
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
            version: "v53",
            block: "PWM",
            ir: &pwm::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
        address: 0xf0404000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
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
        name: "QEO0",
        address: 0xf0410000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
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
        address: 0xf0414000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
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
        name: "RDC0",
        address: 0xf0440000,
        registers: Some(PeripheralRegisters {
            kind: "rdc",
            version: "v6e",
            block: "RDC",
            ir: &rdc::REGISTERS,
        }),
        sysctl: None,
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
                signal: "GLOBAL",
                interrupt: "SEI0",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SEI1",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SEI2",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "SEI3",
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
        sysctl: None,
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
        sysctl: None,
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
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
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(74),
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[
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
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(77),
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(83),
            },
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
            version: "common",
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
            version: "common",
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
            version: "common",
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
        name: "WDG0",
        address: 0xf00b0000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(8),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(9),
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(47),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(48),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(40),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(49),
            },
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
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(46),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(42),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(43),
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART1",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART10",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART11",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART12",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART13",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART14",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "UART15",
            },
        ],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(19),
            },
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(18),
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(23),
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        pins: &[],
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
        pins: &[],
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
        pins: &[],
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
        pins: &[],
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
        sysctl: None,
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
        sysctl: None,
        pins: &[],
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
                signal: "SYNC1",
                dmamux: Some("DMAMUX"),
                request: Some(119),
            },
            PeripheralDmaChannel {
                signal: "SYNC0",
                dmamux: Some("DMAMUX"),
                request: Some(118),
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(108),
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
        pins: &[],
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
        pins: &[],
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
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "0",
                dmamux: Some("DMAMUX"),
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "1",
                dmamux: Some("DMAMUX"),
                request: Some(115),
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
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
        pins: &[],
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
        pins: &[],
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
        pins: &[],
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
        pins: &[],
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(37),
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                dmamux: Some("DMAMUX"),
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "TX",
                dmamux: Some("DMAMUX"),
                request: Some(51),
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        sysctl: None,
        pins: &[],
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
        address: 0xf040c000,
        registers: Some(PeripheralRegisters {
            kind: "qei",
            version: "v6e",
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
        name: "QEO2",
        address: 0xf0418000,
        registers: Some(PeripheralRegisters {
            kind: "qeo",
            version: "v6e",
            block: "QEO",
            ir: &qeo::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(86),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                dmamux: Some("DMAMUX"),
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                dmamux: Some("DMAMUX"),
                request: Some(85),
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
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
            interrupt: "GPTMR6",
        }],
    },
    Peripheral {
        name: "GPTMR7",
        address: 0xf001c000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "common",
            block: "TMR",
            ir: &tmr::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                dmamux: Some("DMAMUX"),
                request: Some(96),
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
            PeripheralDmaChannel {
                signal: "CH2",
                dmamux: Some("DMAMUX"),
                request: Some(98),
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
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "WDG3",
        address: 0xf00bc000,
        registers: Some(PeripheralRegisters {
            kind: "wdg",
            version: "v68",
            block: "WDG",
            ir: &wdg::REGISTERS,
        }),
        sysctl: None,
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
        sysctl: None,
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
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC3",
        }],
    },
    Peripheral {
        name: "TSW",
        address: 0xf140c000,
        registers: Some(PeripheralRegisters {
            kind: "tsw",
            version: "v6e",
            block: "TSW",
            ir: &tsw::REGISTERS,
        }),
        sysctl: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSW0",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSW1",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSW2",
            },
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "TSW3",
            },
            PeripheralInterrupt {
                signal: "EVT",
                interrupt: "TSW_PTP_EVT",
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
        name: "SEI0",
        number: 111,
    },
    Interrupt {
        name: "SEI1",
        number: 112,
    },
    Interrupt {
        name: "SEI2",
        number: 113,
    },
    Interrupt {
        name: "SEI3",
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
        name: "TSW0",
        number: 128,
    },
    Interrupt {
        name: "TSW1",
        number: 129,
    },
    Interrupt {
        name: "TSW2",
        number: 130,
    },
    Interrupt {
        name: "TSW3",
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
pub(crate) static RESOURCES: &[Resource] = &[];
pub(crate) static CLOCKS: &[Clock] = &[];
pub(crate) static PINS: &[IoPin] = &[];

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
#[path = "../registers/femc_v6e.rs"]
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
#[path = "../registers/ppi_v6e.rs"]
pub mod ppi;
#[path = "../registers/ppor_v6e.rs"]
pub mod ppor;
#[path = "../registers/psec_common.rs"]
pub mod psec;
#[path = "../registers/ptpc_common.rs"]
pub mod ptpc;
#[path = "../registers/pwm_v53.rs"]
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
#[path = "../registers/tmr_common.rs"]
pub mod tmr;
#[path = "../registers/trgm_v53.rs"]
pub mod trgm;
#[path = "../registers/tsns_common.rs"]
pub mod tsns;
#[path = "../registers/tsw_v6e.rs"]
pub mod tsw;
#[path = "../registers/uart_v53.rs"]
pub mod uart;
#[path = "../registers/usb_v53.rs"]
pub mod usb;
#[path = "../registers/vsc_v6e.rs"]
pub mod vsc;
#[path = "../registers/wdg_v68.rs"]
pub mod wdg;
#[path = "../registers/xpi_dummy.rs"]
pub mod xpi;
