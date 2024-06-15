use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ddrphy",
            extends: None,
            description: Some(
                "DDRPHY.",
            ),
            items: &[
                BlockItem {
                    name: "ridr",
                    description: Some(
                        "Revision Identification Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ridr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pir",
                    description: Some(
                        "PHY Initialization Register (PIR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pgcr0",
                    description: Some(
                        "PHY General Configuration Registers 0-1 (PGCR0- 1).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pgcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pgcr1",
                    description: Some(
                        "PHY General Configuration Registers 0-1 (PGCR0- 1).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pgcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pgsr0",
                    description: Some(
                        "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pgsr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pgsr1",
                    description: Some(
                        "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pgsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcr",
                    description: Some(
                        "“PLL Control Register (PLLCR)” on page 91.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptr0",
                    description: Some(
                        "PHY Timing Registers 0-4 (PTR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptr1",
                    description: Some(
                        "PHY Timing Registers 0-4 (PTR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptr2",
                    description: Some(
                        "PHY Timing Registers 0-4 (PTR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptr3",
                    description: Some(
                        "PHY Timing Registers 0-4 (PTR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptr4",
                    description: Some(
                        "PHY Timing Registers 0-4 (PTR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ptr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acmdlr",
                    description: Some(
                        "“AC Master Delay Line Register (ACMDLR)” on page 96.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acmdlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acbdlr",
                    description: Some(
                        "“AC Bit Delay Line Register (ACBDLR)” on page 96.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acbdlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aciocr",
                    description: Some(
                        "“AC I/O Configuration Register (ACIOCR)” on page 97.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aciocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dxccr",
                    description: Some(
                        "“DATX8 Common Configuration Register (DXCCR)” on page 99.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dxccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dsgcr",
                    description: Some(
                        "“DDR System General Configuration Register (DSGCR)” on page 101.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dsgcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcr",
                    description: Some(
                        "“DRAM Configuration Register (DCR)” on page 103.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtpr0",
                    description: Some(
                        "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtpr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtpr1",
                    description: Some(
                        "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtpr2",
                    description: Some(
                        "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtpr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mr0",
                    description: Some(
                        "“Mode Register 0 (MR0)” on page 108.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "emr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Emr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mr1",
                    description: Some(
                        "“Mode Register 1 (MR1)” on page 111.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "emr2",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Emr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mr2",
                    description: Some(
                        "“Mode Register 2/Extended Mode Register 2 (MR2/EMR2)” on page 114.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "emr3",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Emr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mr3",
                    description: Some(
                        "“Mode Register 3 (MR3)” on page 116.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odtcr",
                    description: Some(
                        "“ODT Configuration Register (ODTCR)” on page 117.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odtcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtcr",
                    description: Some(
                        "“Data Training Configuration Register (DTCR)” on page 118.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtar0",
                    description: Some(
                        "Data Training Address Register 0-3 (DTAR0-3).",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtar0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtar1",
                    description: Some(
                        "Data Training Address Register 0-3 (DTAR0-3).",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtar2",
                    description: Some(
                        "Data Training Address Register 0-3 (DTAR0-3).",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtar3",
                    description: Some(
                        "Data Training Address Register 0-3 (DTAR0-3).",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtar3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtdr0",
                    description: Some(
                        "Data Training Eye Data Register 0-1 (DTEDR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtdr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtdr1",
                    description: Some(
                        "Data Training Eye Data Register 0-1 (DTEDR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtdr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtedr0",
                    description: Some(
                        "Data Training Eye Data Register 0-1 (DTEDR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtedr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dtedr1",
                    description: Some(
                        "Data Training Eye Data Register 0-1 (DTEDR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dtedr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pgcr2",
                    description: Some(
                        "“PHY General Configuration Register 2 (PGCR2)” on page 87.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pgcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdimmgcr0",
                    description: Some(
                        "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdimmgcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdimmgcr1",
                    description: Some(
                        "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdimmgcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdimmcr0",
                    description: Some(
                        "RDIMM Control Register 0-1 (RDIMMCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdimmcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdimmcr1",
                    description: Some(
                        "RDIMM Control Register 0-1 (RDIMMCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdimmcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcuar",
                    description: Some(
                        "“DCU Address Register (DCUAR)” on page 129.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcuar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcudr",
                    description: Some(
                        "“DCU Data Register (DCUDR)” on page 130.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcudr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcurr",
                    description: Some(
                        "“DCU Run Register (DCURR)” on page 130.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcurr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dculr",
                    description: Some(
                        "“DCU Loop Register (DCULR)” on page 131.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dculr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcugcr",
                    description: Some(
                        "“DCU General Configuration Register (DCUGCR)” on page 132.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcugcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcutpr",
                    description: Some(
                        "“DCU Timing Parameter Register (DCUTPR)” on page 132.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcutpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcusr0",
                    description: Some(
                        "DCU Status Register 0-1 (DCUSR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcusr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcusr1",
                    description: Some(
                        "DCU Status Register 0-1 (DCUSR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcusr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistrr",
                    description: Some(
                        "“BIST Run Register (BISTRR)” on page 133.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistwcr",
                    description: Some(
                        "“BIST Word Count Register (BISTWCR)” on page 136.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistwcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistmskr0",
                    description: Some(
                        "BIST Mask Register 0-2 (BISTMSKR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistmskr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistmskr1",
                    description: Some(
                        "BIST Mask Register 0-2 (BISTMSKR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistmskr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistmskr2",
                    description: Some(
                        "BIST Mask Register 0-2 (BISTMSKR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistmskr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistlsr",
                    description: Some(
                        "“BIST LFSR Seed Register (BISTLSR)” on page 137.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistlsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistar0",
                    description: Some(
                        "BIST Address Register 0-2 (BISTAR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistar0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistar1",
                    description: Some(
                        "BIST Address Register 0-2 (BISTAR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistar1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistar2",
                    description: Some(
                        "BIST Address Register 0-2 (BISTAR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistar2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistudpr",
                    description: Some(
                        "“BIST User Data Pattern Register (BISTUDPR)” on page 138.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistudpr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistgsr",
                    description: Some(
                        "“BIST General Status Register (BISTGSR)” on page 139.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistgsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistwer",
                    description: Some(
                        "“BIST Word Error Register (BISTWER)” on page 139.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistwer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistber0",
                    description: Some(
                        "BIST Bit Error Register 0-3 (BISTBER0-3).",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistber0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistber1",
                    description: Some(
                        "BIST Bit Error Register 0-3 (BISTBER0-3).",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistber1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistber2",
                    description: Some(
                        "BIST Bit Error Register 0-3 (BISTBER0-3).",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistber2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistber3",
                    description: Some(
                        "BIST Bit Error Register 0-3 (BISTBER0-3).",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistber3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistwcsr",
                    description: Some(
                        "“BIST Word Count Status Register (BISTWCSR)” on page 141.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistwcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistfwr0",
                    description: Some(
                        "BIST Fail Word Register 0-2 (BISTFWR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistfwr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistfwr1",
                    description: Some(
                        "BIST Fail Word Register 0-2 (BISTFWR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistfwr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bistfwr2",
                    description: Some(
                        "BIST Fail Word Register 0-2 (BISTFWR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bistfwr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aacr",
                    description: Some(
                        "“Anti-Aging Control Register (AACR)” on page 143.",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr0",
                    description: Some(
                        "General Purpose Register 0-1 (GPR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr1",
                    description: Some(
                        "General Purpose Register 0-1 (GPR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zq",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 16,
                            },
                        ),
                    ),
                    byte_offset: 0x180,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Zq",
                        },
                    ),
                },
                BlockItem {
                    name: "dx",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 9,
                                stride: 64,
                            },
                        ),
                    ),
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Dx",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Dx",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "“DATX8 General Configuration Register (DXnGCR)” on page 148.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gsr0",
                    description: Some(
                        "DATX8 General Status Registers 0-2 (DXnGSR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gsr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gsr1",
                    description: Some(
                        "DATX8 General Status Registers 0-2 (DXnGSR0-2).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdlr0",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdlr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdlr1",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdlr2",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdlr3",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdlr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdlr4",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdlr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcdlr0",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcdlr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcdlr1",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcdlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcdlr2",
                    description: Some(
                        "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcdlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdlr",
                    description: Some(
                        "“DATX8 Master Delay Line Register (DXnMDLR)” on page 157.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gtr",
                    description: Some(
                        "“DATX8 General Timing Register (DXnGTR)” on page 159.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gsr2",
                    description: Some(
                        "“DATX8 General Status Register 2 (DXnGSR2)” on page 152.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gsr2",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Zq",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "cr0",
                    description: Some(
                        "Impedance Control Register 0-1 (ZQnCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Impedance Control Register 0-1 (ZQnCR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr0",
                    description: Some(
                        "Impedance Status Register 0-1 (ZQnSR0-1).",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr1",
                    description: Some(
                        "Impedance Status Register 0-1 (ZQnSR0-1).",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Aacr",
            extends: None,
            description: Some(
                "“Anti-Aging Control Register (AACR)” on page 143.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aatr",
                    description: Some(
                        "Anti-Aging Toggle Rate: Defines the number of controller clock (ctl_clk) cycles after which the PUB will toggle the data going to DATX8 if the data channel between the controller/PUB and DATX8 has been idle for this long. The default value correspond to a toggling count of 4096 ctl_clk cycles. For a ctl_clk running at 533MHz the toggle rate will be approximately 7.68us. The default value may also be overridden by the macro DWC_AACR_AATR_DFLT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aaenc",
                    description: Some(
                        "Anti-Aging Enable Control: Enables, if set, the automatic toggling of the data going to the DATX8 when the data channel from the controller/PUB to DATX8 is idle for programmable number of clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aaoenc",
                    description: Some(
                        "Anti-Aging PAD Output Enable Control: Enables, if set, anti-aging toggling on the pad output enable signal “ctl_oe_n” going into the DATX8s. This will increase power consumption for the anti-aging feature.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Acbdlr",
            extends: None,
            description: Some(
                "“AC Bit Delay Line Register (ACBDLR)” on page 96.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ck0bd",
                    description: Some(
                        "CK0 Bit Delay: Delay select for the BDL on CK0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ck1bd",
                    description: Some(
                        "CK1 Bit Delay: Delay select for the BDL on CK1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ck2bd",
                    description: Some(
                        "CK2 Bit Delay: Delay select for the BDL on CK2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acbd",
                    description: Some(
                        "Address/Command Bit Delay: Delay select for the BDLs on address and command signals.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Aciocr",
            extends: None,
            description: Some(
                "“AC I/O Configuration Register (ACIOCR)” on page 97.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aciom",
                    description: Some(
                        "Address/Command I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for all address and command pins. This bit connects to bit [0] of the IOM pin on the D3F I/Os, and for other I/O libraries, it connects to the IOM pin of the I/O.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acoe",
                    description: Some(
                        "Address/Command Output Enable: Enables, when set, the output driver on the I/O for all address and command pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acodt",
                    description: Some(
                        "Address/Command On-Die Termination: Enables, when set, the on-die termination on the I/O for RAS#, CAS#, WE#, BA[2:0], and A[15:0] pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acpdd1",
                    description: Some(
                        "AC Power Down Driver: Powers down, when set, the output driver on the I/O for RAS#, CAS#, WE#, BA[2:0], and A[15:0] pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acpdr",
                    description: Some(
                        "AC Power Down Receiver: Powers down, when set, the input receiver on the I/O for RAS#, CAS#, WE#, BA[2:0], and A[15:0] pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckodt",
                    description: Some(
                        "CK On-Die Termination: Enables, when set, the on-die termination on the I/O for CK[0], CK[1], and CK[2] pins, respectively.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckpdd1",
                    description: Some(
                        "CK Power Down Driver: Powers down, when set, the output driver on the I/O for CK[0], CK[1], and CK[2] pins, respectively.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckpdr",
                    description: Some(
                        "CK Power Down Receiver: Powers down, when set, the input receiver on the I/O for CK[0], CK[1], and CK[2] pins, respectively.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rankodt",
                    description: Some(
                        "Rank On-Die Termination: Enables, when set, the on-die termination on the I/O for CKE[3:0], ODT[3:0], and CS#[3:0] pins. RANKODT[0] controls the on-die termination for CKE[0], ODT[0], and CS#[0], RANKODT[1] controls the on-die termination for CKE[1], ODT[1], and CS#[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cspdd1",
                    description: Some(
                        "CS# Power Down Driver: Powers down, when set, the output driver on the I/O for CS#[3:0] pins. CSPDD[0] controls the power down for CS#[0], CSPDD[1] controls the power down for CS#[1], and so on. CKE and ODT driver power down is controlled by DSGCR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rankpdr",
                    description: Some(
                        "Rank Power Down Receiver: Powers down, when set, the input receiver on the I/O CKE[3:0], ODT[3:0], and CS#[3:0] pins. RANKPDR[0] controls the power down for CKE[0], ODT[0], and CS#[0], RANKPDR[1] controls the power down for CKE[1], ODT[1], and CS#[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstodt",
                    description: Some(
                        "SDRAM Reset On-Die Termination: Enables, when set, the on-die termination on the I/O for SDRAM RST# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstpdd1",
                    description: Some(
                        "SDRAM Reset Power Down Driver: Powers down, when set, the output driver on the I/O for SDRAM RST# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstpdr",
                    description: Some(
                        "SDRAM Reset Power Down Receiver: Powers down, when set, the input receiver on the I/O for SDRAM RST# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstiom",
                    description: Some(
                        "SDRAM Reset I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for SDRAM Reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acsr",
                    description: Some(
                        "Address/Command Slew Rate (D3F I/O Only): Selects slew rate of the I/O for all address and command pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Acmdlr",
            extends: None,
            description: Some(
                "“AC Master Delay Line Register (ACMDLR)” on page 96.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprd",
                    description: Some(
                        "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tprd",
                    description: Some(
                        "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdld",
                    description: Some(
                        "MDL Delay: Delay select for the LCDL for the Master Delay Line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdlr0",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dq0wbd",
                    description: Some(
                        "DQ0 Write Bit Delay: Delay select for the BDL on DQ0 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq1wbd",
                    description: Some(
                        "DQ1 Write Bit Delay: Delay select for the BDL on DQ1 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq2wbd",
                    description: Some(
                        "DQ2 Write Bit Delay: Delay select for the BDL on DQ2 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq3wbd",
                    description: Some(
                        "DQ3 Write Bit Delay: Delay select for the BDL on DQ3 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq4wbd",
                    description: Some(
                        "DQ4 Write Bit Delay: Delay select for the BDL on DQ4 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdlr1",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dq5wbd",
                    description: Some(
                        "DQ5 Write Bit Delay: Delay select for the BDL on DQ5 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq6wbd",
                    description: Some(
                        "DQ6 Write Bit Delay: Delay select for the BDL on DQ6 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq7wbd",
                    description: Some(
                        "DQ7 Write Bit Delay: Delay select for the BDL on DQ7 write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmwbd",
                    description: Some(
                        "DM Write Bit Delay: Delay select for the BDL on DM write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dswbd",
                    description: Some(
                        "DQS Write Bit Delay: Delay select for the BDL on DQS write path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdlr2",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsoebd",
                    description: Some(
                        "DQS Output Enable Bit Delay: Delay select for the BDL on DQS output enable path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqoebd",
                    description: Some(
                        "DQ Output Enable Bit Delay: Delay select for the BDL on DQ/DM output enable path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsrbd",
                    description: Some(
                        "DQS Read Bit Delay: Delay select for the BDL on DQS read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsnrbd",
                    description: Some(
                        "DQSN Read Bit Delay (Type B/B1 PHY Only): Delay select for the BDL on DQSN read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdlr3",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dq0rbd",
                    description: Some(
                        "DQ0 Read Bit Delay: Delay select for the BDL on DQ0 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq1rbd",
                    description: Some(
                        "DQ1 Read Bit Delay: Delay select for the BDL on DQ1 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq2rbd",
                    description: Some(
                        "DQ2 Read Bit Delay: Delay select for the BDL on DQ2 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq3rbd",
                    description: Some(
                        "DQ3 Read Bit Delay: Delay select for the BDL on DQ3 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq4rbd",
                    description: Some(
                        "DQ4 Read Bit Delay: Delay select for the BDL on DQ4 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdlr4",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dq5rbd",
                    description: Some(
                        "DQ5 Read Bit Delay: Delay select for the BDL on DQ5 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq6rbd",
                    description: Some(
                        "DQ6 Read Bit Delay: Delay select for the BDL on DQ6 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dq7rbd",
                    description: Some(
                        "DQ7 Read Bit Delay: Delay select for the BDL on DQ7 read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmrbd",
                    description: Some(
                        "DM Read Bit Delay: Delay select for the BDL on DM read path.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistar0",
            extends: None,
            description: Some(
                "BIST Address Register 0-2 (BISTAR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bcol",
                    description: Some(
                        "BIST Column Address: Selects the SDRAM column address to be used during BIST. The lower bits of this address must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brow",
                    description: Some(
                        "BIST Row Address: Selects the SDRAM row address to be used during BIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bbank",
                    description: Some(
                        "BIST Bank Address: Selects the SDRAM bank address to be used during BIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistar1",
            extends: None,
            description: Some(
                "BIST Address Register 0-2 (BISTAR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brank",
                    description: Some(
                        "BIST Rank: Selects the SDRAM rank to be used during BIST. Valid values range from 0 to maximum ranks minus 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmrank",
                    description: Some(
                        "BIST Maximum Rank: Specifies the maximum SDRAM rank to be used during BIST. The default value is set to maximum ranks minus 1. Example default shown here is for a 4-rank system.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bainc",
                    description: Some(
                        "BIST Address Increment: Selects the value by which the SDRAM address is incremented for each write/read access. This value must be at the beginning of a burst boundary, i.e. the lower bits must be “0000” for BL16, “000” for BL8, “00” for BL4 and “0” for BL2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistar2",
            extends: None,
            description: Some(
                "BIST Address Register 0-2 (BISTAR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmcol",
                    description: Some(
                        "BIST Maximum Column Address: Specifies the maximum SDRAM column address to be used during BIST before the address increments to the next row.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmrow",
                    description: Some(
                        "BIST Maximum Row Address: Specifies the maximum SDRAM row address to be used during BIST before the address increments to the next bank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmbank",
                    description: Some(
                        "BIST Maximum Bank Address: Specifies the maximum SDRAM bank address to be used during BIST before the address increments to the next rank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistber0",
            extends: None,
            description: Some(
                "BIST Bit Error Register 0-3 (BISTBER0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aber",
                    description: Some(
                        "Address Bit Error: Each group of two bits indicate the bit error count on each of the.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistber1",
            extends: None,
            description: Some(
                "BIST Bit Error Register 0-3 (BISTBER0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baber",
                    description: Some(
                        "Bank Address Bit Error: Each group of two bits indicate the bit error count on each of the up to 3 bank address bits. [1:0] is the error count for BA[0], [3:2] for BA[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "weber",
                    description: Some(
                        "WE# Bit Error: Indicates the number of bit errors on WE#.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckeber",
                    description: Some(
                        "CKE Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CKE bits. [1:0] is the error count for CKE[0], [3:2] for CKE[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csber",
                    description: Some(
                        "CS# Bit Error: Each group of two bits indicate the bit error count on each of the up to 4 CS# bits. [1:0] is the error count for CS#[0], [3:2] for CS#[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odtber",
                    description: Some(
                        "ODT Bit Error: Each group of two bits indicates the bit error count on each of the up to 4 ODT bits. [1:0] is the error count for ODT[0], [3:2] for ODT[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistber2",
            extends: None,
            description: Some(
                "BIST Bit Error Register 0-3 (BISTBER0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dqber0",
                    description: Some(
                        "Data Bit Error: The error count for even DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ[7:0] on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ[7:0] on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ[0], the second for DQ[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistber3",
            extends: None,
            description: Some(
                "BIST Bit Error Register 0-3 (BISTBER0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dqber1",
                    description: Some(
                        "Data Bit Error: The error count for odd DQS cycles. The first 16 bits indicate the error count for the first data beat (i.e. the data driven out on DQ[7:0] on the rising edge of DQS). The second 16 bits indicate the error on the second data beat (i.e. the error count of the data driven out on DQ[7:0] on the falling edge of DQS). For each of the 16-bit group, the first 2 bits are for DQ[0], the second for DQ[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistfwr0",
            extends: None,
            description: Some(
                "BIST Fail Word Register 0-2 (BISTFWR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awebs",
                    description: Some(
                        "Bit status during a word error for each of the up to 16 address bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bawebs",
                    description: Some(
                        "Bit status during a word error for each of the up to 3 bank address bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wewebs",
                    description: Some(
                        "Bit status during a word error for the WE#.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckewebs",
                    description: Some(
                        "Bit status during a word error for each of the up to 4 CKE bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cswebs",
                    description: Some(
                        "Bit status during a word error for each of the up to 4 CS# bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odtwebs",
                    description: Some(
                        "Bit status during a word error for each of the up to 4 ODT bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistfwr1",
            extends: None,
            description: Some(
                "BIST Fail Word Register 0-2 (BISTFWR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "raswebs",
                    description: Some(
                        "Bit status during a word error for the RAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "caswebs",
                    description: Some(
                        "Bit status during a word error for the CAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parwebs",
                    description: Some(
                        "Bit status during a word error for the PAR_IN. Only for DIMM parity support.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmwebs",
                    description: Some(
                        "Bit status during a word error for the data mask (DM) bit. DMWEBS [0] is for the first DM beat, DMWEBS [1] is for the second DM beat, and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistfwr2",
            extends: None,
            description: Some(
                "BIST Fail Word Register 0-2 (BISTFWR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dqwebs",
                    description: Some(
                        "Bit status during a word error for each of the 8 data (DQ) bits. The first 8 bits indicate the status of the first data beat (i.e. the status of the data driven out on DQ[7:0] on the rising edge of DQS). The second 8 bits indicate the status of the second data beat (i.e. the status of the data driven out on DQ[7:0] on the falling edge of DQS), and so on. For each of the 8-bit group, the first bit is for DQ[0], the second bit is for DQ[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistgsr",
            extends: None,
            description: Some(
                "“BIST General Status Register (BISTGSR)” on page 139.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bdone",
                    description: Some(
                        "BIST Done: Indicates, if set, that the BIST has finished executing. This bit is reset to zero when BIST is triggered.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bacerr",
                    description: Some(
                        "BIST Address/Command Error: indicates, if set, that there is a data comparison error in the address/command lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdxerr",
                    description: Some(
                        "BIST Data Error: indicates, if set, that there is a data comparison error in the byte lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parber",
                    description: Some(
                        "PAR_IN Bit Error (DIMM Only): Indicates the number of bit errors on PAR_IN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmber",
                    description: Some(
                        "DM Bit Error: Indicates the number of bit errors on data mask (DM) bit. DMBER[1:0] are for even DQS cycles first DM beat, and DMBER[3:2] are for even DQS cycles second DM beat. Similarly, DMBER[5:4] are for odd DQS cycles first DM beat, and DMBER[7:6] are for odd DQS cycles second DM beat.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rasber",
                    description: Some(
                        "RAS Bit Error: Indicates the number of bit errors on RAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "casber",
                    description: Some(
                        "CAS Bit Error: Indicates the number of bit errors on CAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistlsr",
            extends: None,
            description: Some(
                "“BIST LFSR Seed Register (BISTLSR)” on page 137.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seed",
                    description: Some(
                        "LFSR seed for pseudo-random BIST patterns.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistmskr0",
            extends: None,
            description: Some(
                "BIST Mask Register 0-2 (BISTMSKR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amsk",
                    description: Some(
                        "Mask bit for each of the up to 16 address bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bamsk",
                    description: Some(
                        "Mask bit for each of the up to 3 bank address bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wemsk",
                    description: Some(
                        "Mask bit for the WE#.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckemsk",
                    description: Some(
                        "Mask bit for each of the up to 4 CKE bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csmsk",
                    description: Some(
                        "Mask bit for each of the up to 4 CS# bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odtmsk",
                    description: Some(
                        "Mask bit for each of the up to 4 ODT bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistmskr1",
            extends: None,
            description: Some(
                "BIST Mask Register 0-2 (BISTMSKR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rasmsk",
                    description: Some(
                        "Mask bit for the RAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "casmsk",
                    description: Some(
                        "Mask bit for the CAS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parmsk",
                    description: Some(
                        "Mask bit for the PAR_IN. Only for DIMM parity support and only if the design is compiled for less than 3 ranks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmmsk",
                    description: Some(
                        "Mask bit for the data mask (DM) bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistmskr2",
            extends: None,
            description: Some(
                "BIST Mask Register 0-2 (BISTMSKR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dqmsk",
                    description: Some(
                        "Mask bit for each of the 8 data (DQ) bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistrr",
            extends: None,
            description: Some(
                "“BIST Run Register (BISTRR)” on page 133.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "binst",
                    description: Some(
                        "BIST Instruction: Selects the BIST instruction to be executed: Valid values are: 000 = NOP: No operation 001 = Run: Triggers the running of the BIST. 010 = Stop: Stops the running of the BIST. 011 = Reset: Resets all BIST run-time registers, such as error counters. 100 – 111 Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmode",
                    description: Some(
                        "BIST Mode: Selects the mode in which BIST is run. Valid values are: 0 = Loopback mode: Address, commands and data loop back at the PHY I/Os. 1 = DRAM mode: Address, commands and data go to DRAM for normal memory accesses.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "binf",
                    description: Some(
                        "BIST Infinite Run: Specifies, if set, that the BIST should be run indefinitely until when it is either stopped or a failure has been encountered. Otherwise BIST is run until number of BIST words specified in the BISTWCR register has been generated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nfail",
                    description: Some(
                        "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if BSONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if BSONF is set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bsonf",
                    description: Some(
                        "BIST Stop On Nth Fail: Specifies, if set, that the BIST should stop when an nth data word or address/command comparison error has been encountered.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdxen",
                    description: Some(
                        "BIST DATX8 Enable: Enables the running of BIST on the data byte lane PHYs. This bit is exclusive with BACEN, i.e. both cannot be set to ‘1’ at the same time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bacen",
                    description: Some(
                        "BIST AC Enable: Enables the running of BIST on the address/command lane PHY. This bit is exclusive with BDXEN, i.e. both cannot be set to ‘1’ at the same time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdmen",
                    description: Some(
                        "BIST Data Mask Enable: Enables, if set, that the data mask BIST should be included in the BIST run, i.e. data pattern generated and loopback data compared. This is valid only for loopback mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdpat",
                    description: Some(
                        "BIST Data Pattern: Selects the data pattern used during BIST. Valid values are: 00 = Walking 0 01 = Walking 1 10 = LFSR-based pseudo-random 11 = User programmable (Not valid for AC loopback).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdxsel",
                    description: Some(
                        "BIST DATX8 Select: Select the byte lane for comparison of loopback/read data. Valid values are 0 to 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcksel",
                    description: Some(
                        "BIST CK Select: Selects the CK that should be used to register the AC loopback signals from the I/Os. Valid values are: 00 = CK[0] 01 = CK[1] 10 = CK[2] 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bccsel",
                    description: Some(
                        "BIST Clock Cycle Select: Selects the clock numbers on which the AC loopback data is written into the FIFO. Data is written into the loopback FIFO once every four clock cycles. Valid values are: 00 = Clock cycle 0, 4, 8, 12, etc. 01 = Clock cycle 1, 5, 9, 13, etc. 10 = Clock cycle 2, 6, 10, 14, etc. 11 = Clock cycle 3, 7, 11, 15, etc.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistudpr",
            extends: None,
            description: Some(
                "“BIST User Data Pattern Register (BISTUDPR)” on page 138.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "budp0",
                    description: Some(
                        "BIST User Data Pattern 0: Data to be applied on even DQ pins during BIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "budp1",
                    description: Some(
                        "BIST User Data Pattern 1: Data to be applied on odd DQ pins during BIST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistwcr",
            extends: None,
            description: Some(
                "“BIST Word Count Register (BISTWCR)” on page 136.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bwcnt",
                    description: Some(
                        "BIST Word Count: Indicates the number of words to generate during BIST. This must be a multiple of DRAM burst length (BL) divided by 2, e.g. for BL=8, valid values are 4, 8, 12, 16, and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistwcsr",
            extends: None,
            description: Some(
                "“BIST Word Count Status Register (BISTWCSR)” on page 141.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acwcnt",
                    description: Some(
                        "Address/Command Word Count: Indicates the number of words received from the address/command lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxwcnt",
                    description: Some(
                        "Byte Word Count: Indicates the number of words received from the byte lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bistwer",
            extends: None,
            description: Some(
                "“BIST Word Error Register (BISTWER)” on page 139.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acwer",
                    description: Some(
                        "Address/Command Word Error: Indicates the number of word errors on the address/command lane. An error on any bit of the address/command bus increments the error count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxwer",
                    description: Some(
                        "Byte Word Error: Indicates the number of word errors on the byte lane. An error on any bit of the data bus including the data mask bit increments the error count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr0",
            extends: None,
            description: Some(
                "Impedance Control Register 0-1 (ZQnCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zdata",
                    description: Some(
                        "Impedance Over-Ride Data: Data used to directly drive the impedance control. ZDATA field mapping for D3F I/Os is as follows: ZDATA[27:21] is used to select the pull-up on-die termination impedance ZDATA[20:14] is used to select the pull-down on-die termination impedance ZDATA[13:7] is used to select the pull-up output impedance ZDATA[6:0] is used to select the pull-down output impedance ZDATA field mapping for D3A/B/R I/Os is as follows: ZDATA[27:20] is reserved and returns zeros on reads ZDATA[19:15] is used to select the pull-up on-die termination impedance ZDATA[14:10] is used to select the pull-down on-die termination impedance ZDATA[9:5] is used to select the pull-up output impedance ZDATA[4:0] is used to select the pull-down output impedance The default value is 0x000014A for I/O type D3C/R and 0x0001830 for I/O type D3F.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zden",
                    description: Some(
                        "Impedance Over-ride Enable: When this bit is set, it allows users to directly drive the impedance control using the data programmed in the ZDATA field. Otherwise, the control is generated automatically by the impedance control logic.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcalbyp",
                    description: Some(
                        "Impedance Calibration Bypass: Bypasses, if set, impedance calibration of this ZQ control block when impedance calibration is already in progress. Impedance calibration can be disabled prior to trigger by using the ZCALEN bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcalen",
                    description: Some(
                        "Impedance Calibration Enable: Enables, if set, the impedance calibration of this ZQ control block when impedance calibration is triggered using either the ZCAL bit of PIR register or the DFI update interface.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zqpd",
                    description: Some(
                        "ZQ Power Down: Powers down, if set, the PZQ cell.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Impedance Control Register 0-1 (ZQnCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zprog",
                    description: Some(
                        "Impedance Divide Ratio: Selects the external resistor divide ratio to be used to set the output impedance and the on-die termination as follows: ZPROG[7:4] = On-die termination divide select ZPROG[3:0] = Output impedance divide select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dficu0",
                    description: Some(
                        "DFI Controller Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 0 (channel 0) requests an update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dficu1",
                    description: Some(
                        "DFI Controller Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI controller update interface 1 (channel 1) requests an update. Only valid in shared-AC mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dficcu",
                    description: Some(
                        "DFI Concurrent Controller Update Interface: Sets this impedance controller to be enabled for calibration when both of the DFI controller update interfaces request an update on the same clock. This provides the ability to enable impedance calibration updates for the Address/Command lane. Only valid in shared-AC mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfipu0",
                    description: Some(
                        "DFI Update Interface 0: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 0 (channel 0) requests an update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfipu1",
                    description: Some(
                        "DFI Update Interface 1: Sets this impedance controller to be enabled for calibration when the DFI PHY update interface 1 (channel 1) requests an update. Only valid in shared-AC mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcr",
            extends: None,
            description: Some(
                "“DRAM Configuration Register (DCR)” on page 103.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ddrmd",
                    description: Some(
                        "DDR Mode: SDRAM DDR mode. Valid values are: 000 = Reserved 001 = Reserved 010 = DDR2 011 = DDR3 100 – 111 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddr8bnk",
                    description: Some(
                        "DDR 8-Bank: Indicates, if set, that the SDRAM used has 8 banks. tRPA = tRP+1 and tFAW are used for 8-bank DRAMs, otherwise tRPA = tRP and no tFAW is used. Note that a setting of 1 for DRAMs that have fewer than 8 banks results in correct functionality, but less tight DRAM command spacing for the parameters.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pdq",
                    description: Some(
                        "Primary DQ (DDR3 Only): Specifies the DQ pin in a byte that is designated as a primary pin for Multi-Purpose Register (MPR) reads. Valid values are 0 to 7 for DQ[0] to DQ[7], respectively.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mprdq",
                    description: Some(
                        "Multi-Purpose Register (MPR) DQ (DDR3 Only): Specifies the value that is driven on non-primary DQ pins during MPR reads. Valid values are: 0 = Primary DQ drives out the data from MPR (0-1-0-1); non-primary DQs drive ‘0’ 1 = Primary DQ and non-primary DQs all drive the same data from MPR (0-1-0-1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bytemask",
                    description: Some(
                        "Byte Mask: Mask applied to all beats of read data on all bytes lanes during read DQS gate training. This allows training to be conducted based on selected bit(s) from the byte lanes. Valid values for each bit are: 0 = Disable compare for that bit 1 = Enable compare for that bit Note that this mask applies in DDR3 MPR operation mode as well and must be in keeping with the PDQ field setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nosra",
                    description: Some(
                        "No Simultaneous Rank Access: Specifies, if set, that simultaneous rank access on the same clock cycle is not allowed. This means that multiple chip select signals should not be asserted at the same time. This may be required on some DIMM systems.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddr2t",
                    description: Some(
                        "DDR 2T Timing: Indicates, if set, that 2T timing should be used by PUB internally generated SDRAM transactions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udimm",
                    description: Some(
                        "Un-buffered DIMM Address Mirroring: Indicates, if set, that there is address mirroring on the second rank of an un-buffered DIMM (the rank connected to CS#[1]). In this case, the PUB re-scrambles the bank and address when sending mode register commands to the second rank. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments when using an un-buffered DIMM. DCR[NOSRA] must be set if address mirroring is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcuar",
            extends: None,
            description: Some(
                "“DCU Address Register (DCUAR)” on page 129.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwaddr",
                    description: Some(
                        "Cache Word Address: Address of the cache word to be accessed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csaddr",
                    description: Some(
                        "Cache Slice Address: Address of the cache slice to be accessed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csel",
                    description: Some(
                        "Cache Select: Selects the cache to be accessed. Valid values are: 00 = Command cache 01 = Expected data cache 10 = Read data cache 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inca",
                    description: Some(
                        "Increment Address: Specifies, if set, that the cache address specified in WADDR and SADDR should be automatically incremented after each access of the cache. The increment happens in such a way that all the slices of a selected word are first accessed before going to the next word.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atype",
                    description: Some(
                        "Access Type: Specifies the type of access to be performed using this address. Valid values are: 0 = Write access 1 = Read access.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcudr",
            extends: None,
            description: Some(
                "“DCU Data Register (DCUDR)” on page 130.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cdata",
                    description: Some(
                        "Cache Data: Data to be written to or read from a cache. This data corresponds to the cache word slice specified by the DCU Address Register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcugcr",
            extends: None,
            description: Some(
                "“DCU General Configuration Register (DCUGCR)” on page 132.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcsw",
                    description: Some(
                        "Read Capture Start Word: The capture and compare of read data should start after Nth word. For example setting this value to 12 will skip the first 12 read data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dculr",
            extends: None,
            description: Some(
                "“DCU Loop Register (DCULR)” on page 131.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsaddr",
                    description: Some(
                        "Loop Start Address: Command cache word address where the loop should start.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "leaddr",
                    description: Some(
                        "Loop End Address: Command cache word address where the loop should end.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lcnt",
                    description: Some(
                        "Loop Count: The number of times that the loop should be executed if LINF is not set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linf",
                    description: Some(
                        "Loop Infinite: Indicates, if set, that the loop should be executed indefinitely until stopped by the STOP command. Otherwise the loop is execute LCNT times.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ida",
                    description: Some(
                        "Increment DRAM Address: Indicates, if set, that DRAM addresses should be incremented every time a DRAM read/write command inside the loop is executed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xleaddr",
                    description: Some(
                        "Expected Data Loop End Address: The last expected data cache word address that contains valid expected data. Expected data should looped between 0 and this address. XLEADDR field uses only the following bits based on the cache depth: DCU expected data cache = 4, XLEADDR[1:0] DCU expected data cache = 8, XLEADDR[2:0] DCU expected data cache = 16, XLEADDR[3:0].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcurr",
            extends: None,
            description: Some(
                "“DCU Run Register (DCURR)” on page 130.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dinst",
                    description: Some(
                        "DCU Instruction: Selects the DCU command to be executed: Valid values are: 0000 = NOP: No operation 0001 = Run: Triggers the execution of commands in the command cache. 0010 = Stop: Stops the execution of commands in the command cache. 0011 = Stop Loop: Stops the execution of an infinite loop in the command cache. 0100 = Reset: Resets all DCU run time registers. See “DCU Status” on page 255 for details. 0101 – 1111 Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddr",
                    description: Some(
                        "Start Address: Cache word address where the execution of commands should begin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eaddr",
                    description: Some(
                        "End Address: Cache word address where the execution of command should end.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nfail",
                    description: Some(
                        "Number of Failures: Specifies the number of failures after which the execution of commands and the capture of read data should stop if SONF bit of this register is set. Execution of commands and the capture of read data will stop after (NFAIL+1) failures if SONF is set. Valid values are from 0 to 254.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sonf",
                    description: Some(
                        "Stop On Nth Fail: Specifies, if set, that the execution of commands and the capture of read data should stop when there are N read data failures. The number of failures is specified by NFAIL. Otherwise commands execute until the end of the program or until manually stopped using a STOP command.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scof",
                    description: Some(
                        "Stop Capture On Full: Specifies, if set, that the capture of read data should stop when the capture cache is full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcen",
                    description: Some(
                        "Read Capture Enable: Indicates, if set, that read data coming back from the SDRAM should be captured into the read data cache.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xcen",
                    description: Some(
                        "Expected Compare Enable: Indicates, if set, that read data coming back from the SDRAM should be should be compared with the expected data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcusr0",
            extends: None,
            description: Some(
                "DCU Status Register 0-1 (DCUSR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdone",
                    description: Some(
                        "Run Done: Indicates, if set, that the DCU has finished executing the commands in the command cache. This bit is also set to indicate that a STOP command has successfully been executed and command execution has stopped.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfail",
                    description: Some(
                        "Capture Fail: Indicates, if set, that at least one read data word has failed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfull",
                    description: Some(
                        "Capture Full: Indicates, if set, that the capture cache is full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcusr1",
            extends: None,
            description: Some(
                "DCU Status Register 0-1 (DCUSR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdcnt",
                    description: Some(
                        "Read Count: Number of read words returned from the SDRAM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flcnt",
                    description: Some(
                        "Fail Count: Number of read words that have failed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpcnt",
                    description: Some(
                        "Loop Count: Indicates the value of the loop count. This is useful when the program has stopped because of failures to assess how many reads were executed before first fail.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcutpr",
            extends: None,
            description: Some(
                "“DCU Timing Parameter Register (DCUTPR)” on page 132.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdcut0",
                    description: Some(
                        "DCU Generic Timing Parameter 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdcut1",
                    description: Some(
                        "DCU Generic Timing Parameter 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdcut2",
                    description: Some(
                        "DCU Generic Timing Parameter 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdcut3",
                    description: Some(
                        "DCU Generic Timing Parameter 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dsgcr",
            extends: None,
            description: Some(
                "“DDR System General Configuration Register (DSGCR)” on page 101.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "puren",
                    description: Some(
                        "PHY Update Request Enable: Specifies if set, that the PHY should issue PHY- initiated update request when there is DDL VT drift.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdisen",
                    description: Some(
                        "Byte Disable Enable: Specifies, if set, that the PHY should respond to DFI byte disable request. Otherwise the byte disable from the DFI is ignored in which case bytes can only be disabled using the DXnGCR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zuen",
                    description: Some(
                        "Impedance Update Enable: Specifies, if set, that in addition to DDL VT update, the PHY could also perform impedance calibration (update). Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFICU0, DFICU1 and DFICCU bits to control if an impedance calibration is performed (update) with a DFI controller update request. Refer to the “Impedance Control Register 0-1 (ZQnCR0-1)” on page 145 bit fields DFIPU0 and DFIPU1 bits to control if an impedance calibration is performed (update) with a DFI PHY update request.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpiopd",
                    description: Some(
                        "Low Power I/O Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the I/Os of the byte.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lppllpd",
                    description: Some(
                        "Low Power PLL Power Down: Specifies, if set, that the PHY should respond to the DFI low power opportunity request and power down the PLL of the byte if the wakeup time request satisfies the PLL lock time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cuaen",
                    description: Some(
                        "Controller Update Acknowledge Enable: Specifies, if set, that the PHY should issue controller update acknowledge when the DFI controller update request is asserted. By default the PHY does not acknowledge controller initiated update requests but simply does an update whenever there is a controller update request. This speeds up the update.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsgx",
                    description: Some(
                        "DQS Gate Extension: Specifies, if set, that the DQS gating must be extended by two DRAM clock cycles and then re-centered, i.e. one clock cycle extension on either side.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brrmode",
                    description: Some(
                        "Bypass Rise-to-Rise Mode: Indicates, if set, that the PHY bypass mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY bypass mode is running in rise-to-fall mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "puad",
                    description: Some(
                        "PHY Update Acknowledge Delay: Specifies the number of clock cycles that the indication for the completion of PHY update from the PHY to the controller should be delayed. This essentially delays, by this many clock cycles, the de-assertion of dfi_ctrlup_ack and dfi_phyupd_req signals relative to the time when the delay lines or I/Os are updated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtoodt",
                    description: Some(
                        "DTO On-Die Termination: Enables, when set, the on-die termination on the I/O for DTO pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtopdd1",
                    description: Some(
                        "DTO Power Down Driver: Powers down, when set, the output driver on the I/O for DTO pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtopdr",
                    description: Some(
                        "DTO Power Down Receiver: Powers down, when set, the input receiver on the I/O for DTO pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtoiom",
                    description: Some(
                        "DTO I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DTO pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtooe",
                    description: Some(
                        "DTO Output Enable: Enables, when set, the output driver on the I/O for DTO pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atoae",
                    description: Some(
                        "ATO Analog Test Enable: Enables, if set, the analog test output (ATO) I/O.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrmode",
                    description: Some(
                        "Rise-to-Rise Mode: Indicates, if set, that the PHY mission mode is configured to run in rise-to-rise mode. Otherwise if not set the PHY mission mode is running in rise-to- fall mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdrmode",
                    description: Some(
                        "Single Data Rate Mode: Indicates, if set, that the external controller is configured to run in single data rate (SDR) mode. Otherwise if not set the controller is running in half data rate (HDR) mode. This bit not supported in the current version of the PUB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckepdd1",
                    description: Some(
                        "CKE Power Down Driver: Powers down, when set, the output driver on the I/O for CKE[3:0] pins. CKEPDD[0] controls the power down for CKE[0], CKEPDD[1] controls the power down for CKE[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odtpdd1",
                    description: Some(
                        "ODT Power Down Driver: Powers down, when set, the output driver on the I/O for ODT[3:0] pins. ODTPDD[0] controls the power down for ODT[0], ODTPDD[1] controls the power down for ODT[1], and so on.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoe",
                    description: Some(
                        "SDRAM CK Output Enable: Enables, when set, the output driver on the I/O for SDRAM CK/CK# pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odtoe",
                    description: Some(
                        "SDRAM ODT Output Enable: Enables, when set, the output driver on the I/O for SDRAM ODT pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstoe",
                    description: Some(
                        "SDRAM Reset Output Enable: Enables, when set, the output driver on the I/O for SDRAM RST# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckeoe",
                    description: Some(
                        "SDRAM CKE Output Enable: Enables, when set, the output driver on the I/O for SDRAM CKE pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtar0",
            extends: None,
            description: Some(
                "Data Training Address Register 0-3 (DTAR0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtcol",
                    description: Some(
                        "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrow",
                    description: Some(
                        "Data Training Row Address: Selects the SDRAM row address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbank",
                    description: Some(
                        "Data Training Bank Address: Selects the SDRAM bank address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtar1",
            extends: None,
            description: Some(
                "Data Training Address Register 0-3 (DTAR0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtcol",
                    description: Some(
                        "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrow",
                    description: Some(
                        "Data Training Row Address: Selects the SDRAM row address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbank",
                    description: Some(
                        "Data Training Bank Address: Selects the SDRAM bank address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtar2",
            extends: None,
            description: Some(
                "Data Training Address Register 0-3 (DTAR0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtcol",
                    description: Some(
                        "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrow",
                    description: Some(
                        "Data Training Row Address: Selects the SDRAM row address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbank",
                    description: Some(
                        "Data Training Bank Address: Selects the SDRAM bank address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtar3",
            extends: None,
            description: Some(
                "Data Training Address Register 0-3 (DTAR0-3).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtcol",
                    description: Some(
                        "Data Training Column Address: Selects the SDRAM column address to be used during data training. The lower four bits of this address must always be “000”.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrow",
                    description: Some(
                        "Data Training Row Address: Selects the SDRAM row address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbank",
                    description: Some(
                        "Data Training Bank Address: Selects the SDRAM bank address to be used during data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtcr",
            extends: None,
            description: Some(
                "“Data Training Configuration Register (DTCR)” on page 118.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrptn",
                    description: Some(
                        "Data Training Repeat Number: Repeat number used to confirm stability of DDR write or read. Note: The minimum value should be 0x4 and the maximum value should be 0x14.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrank",
                    description: Some(
                        "Data Training Rank: Select the SDRAM rank to be used during Read DQS gate training, Read/Write Data Bit Deskew, Read/Write Eye Training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtmpr",
                    description: Some(
                        "Read Data Training Using MPR (DDR3 Only): Specifies, if set, that DQS gate training should use the SDRAM Multi-Purpose Register (MPR) register. Otherwise data-training is performed by first writing to some locations in the SDRAM and then reading them back.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcmpd",
                    description: Some(
                        "Read Data Training Compare Data: Specifies, if set, that DQS gate training should also check if the returning read data is correct. Otherwise data-training only checks if the correct number of DQS edges were returned.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwdqm",
                    description: Some(
                        "Training WDQ Margin: Defines how close to 0 or how close to 2*(wdq calibration_value) the WDQ LCDL can be moved during training. Basically defines how much timing margin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwbddm",
                    description: Some(
                        "Data Training Write Bit Deskew Data Mask, if set, it enables write bit deskew of the data mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbdc",
                    description: Some(
                        "Data Training Bit Deskew Centering: Enables, if set, eye centering capability during write and read bit deskew training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwdqmo",
                    description: Some(
                        "Data Training WDQ Margin Override: If set, the Training WDQ Margin value specified in DTCR[11:8] (DTWDQM) is used during data training. Otherwise the value is computed as ¼ of the ddr_clk period measurement found during calibration of the WDQ LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtdbs",
                    description: Some(
                        "Data Training Debug Byte Select: Selects the byte during data training single step debug mode. Note: DTDEN is not used to enable this feature.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtden",
                    description: Some(
                        "Data Training Debug Enable: Enables, if set, the data training single step debug mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtdstp",
                    description: Some(
                        "Data Training Debug Step: A write of 1 to this bit steps the data training algorithm through a single step. This bit is used to initiate one step of the data training algorithm in question. This bit is self-clearing. To trigger the next step, this bit must be written to again. Note: The training steps must be repeated in order to get new data in the “Data Training Eye Data Register 0-1 (DTEDR0-1)” on page 122. For example, to see the training results for a different lane, select that lane and repeat the training steps to populate DTEDR0 and DTEDR1 with the correct data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtexd",
                    description: Some(
                        "Data Training Extended Write DQS: Enables, if set, an extended write DQS whereby two additional pulses of DQS are added as post-amble to a burst of writes. Generally this should only be enabled when running read bit deskew with the intention of performing read eye deskew prior to running write leveling adjustment.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ranken",
                    description: Some(
                        "Rank Enable: Specifies the ranks that are enabled for data-training. Bit 0 controls rank 0, bit 1 controls rank 1, bit 2 controls rank 2, and bit 3 controls rank 3. Setting the bit to ‘1’ enables the rank, and setting it to ‘0’ disables the rank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rfshdt",
                    description: Some(
                        "Refresh During Training: A non-zero value specifies that a burst of refreshes equal to the number specified in this field should be sent to the SDRAM after training each rank except the last rank.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtdr0",
            extends: None,
            description: Some(
                "Data Training Eye Data Register 0-1 (DTEDR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtbyte0",
                    description: Some(
                        "Data Training Data: The first 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte1",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte2",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte3",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtdr1",
            extends: None,
            description: Some(
                "Data Training Eye Data Register 0-1 (DTEDR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtbyte4",
                    description: Some(
                        "Data Training Data: The second 4 bytes of data used during data training. This same data byte is used for each Byte Lane. Default sequence is a walking 1 while toggling data every data cycle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte5",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte6",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtbyte7",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtedr0",
            extends: None,
            description: Some(
                "Data Training Eye Data Register 0-1 (DTEDR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtwlmn",
                    description: Some(
                        "Data Training WDQ LCDL Minimum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwlmx",
                    description: Some(
                        "Data Training WDQ LCDL Maximum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwbmn",
                    description: Some(
                        "Data Training Write BDL Shift Minimum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtwbmx",
                    description: Some(
                        "Data Training Write BDL Shift Maximum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtedr1",
            extends: None,
            description: Some(
                "Data Training Eye Data Register 0-1 (DTEDR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrlmn",
                    description: Some(
                        "Data Training RDQS LCDL Minimum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrlmx",
                    description: Some(
                        "Data Training RDQS LCDL Maximum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrbmn",
                    description: Some(
                        "Data Training Read BDL Shift Minimum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtrbmx",
                    description: Some(
                        "Data Training Read BDL Shift Maximum.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtpr0",
            extends: None,
            description: Some(
                "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trtp",
                    description: Some(
                        "Internal read to precharge command delay. Valid values are 2 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twtr",
                    description: Some(
                        "Internal write to read command delay. Valid values are 1 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trp",
                    description: Some(
                        "Precharge command period: The minimum time between a precharge command and any other command. Note that the Controller automatically derives tRPA for 8- bank DDR2 devices by adding 1 to tRP. Valid values are 2 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trcd",
                    description: Some(
                        "Activate to read or write delay. Minimum time from when an activate command is issued to when a read or write to the activated row can be issued. Valid values are 2 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tras",
                    description: Some(
                        "Activate to precharge command delay. Valid values are 2 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trrd",
                    description: Some(
                        "Activate to activate command delay (different banks). Valid values are 1 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trc",
                    description: Some(
                        "Activate to activate command delay (same bank). Valid values are 2 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtpr1",
            extends: None,
            description: Some(
                "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmrd",
                    description: Some(
                        "Load mode cycle time: The minimum time between a load mode register command and any other command. For DDR3 this is the minimum time between two load mode register commands. Valid values for DDR2 are 2 to 3. For DDR3, the value used for tMRD is 4 plus the value programmed in these bits, i.e. tMRD value for DDR3 ranges from 4 to 7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmod",
                    description: Some(
                        "Load mode update delay (DDR3 only). The minimum time between a load mode register command and a non-load mode register command. Valid values are: 000 = 12 001 = 13 010 = 14 011 = 15 100 = 16 101 = 17 110 – 111 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tfaw",
                    description: Some(
                        "4-bank activate period. No more than 4-bank activate commands may be issued in a given tFAW period. Only applies to 8-bank devices. Valid values are 2 to 63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trfc",
                    description: Some(
                        "Refresh-to-Refresh: Indicates the minimum time between two refresh commands or between a refresh and an active command. This is derived from the minimum refresh interval from the datasheet, tRFC(min), divided by the clock cycle time. The default number of clock cycles is for the largest JEDEC tRFC(min parameter value supported.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twlmrd",
                    description: Some(
                        "Minimum delay from when write leveling mode is programmed to the first DQS/DQS# rising edge.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twlo",
                    description: Some(
                        "Write leveling output delay: Number of clock cycles from when write leveling DQS is driven high by the control block to when the results from the SDRAM on DQ is sampled by the control block. This must include the SDRAM tWLO timing parameter plus the round trip delay from control block to SDRAM back to control block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "taond_taofd",
                    description: Some(
                        "ODT turn-on/turn-off delays (DDR2 only). Valid values are: 00 = 2/2.5 01 = 3/3.5 10 = 4/4.5 11 = 5/5.5 Most DDR2 devices utilize a fixed value of 2/2.5. For non-standard SDRAMs, the user must ensure that the operational Write Latency is always greater than or equal to the ODT turn-on delay. For example, a DDR2 SDRAM with CAS latency set to 3 and CAS additive latency set to 0 has a Write Latency of 2. Thus 2/2.5 can be used, but not 3/3.5 or higher.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dtpr2",
            extends: None,
            description: Some(
                "DRAM Timing Parameters Register 0-2 (DTPR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txs",
                    description: Some(
                        "Self refresh exit delay. The minimum time between a self refresh exit command and any other command. This parameter must be set to the maximum of the various minimum self refresh exit delay parameters specified in the SDRAM datasheet, i.e. max(tXSNR, tXSRD) for DDR2 and max(tXS, tXSDLL) for DDR3. Valid values are 2 to 1023.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txp",
                    description: Some(
                        "Power down exit delay. The minimum time between a power down exit command and any other command. This parameter must be set to the maximum of the various minimum power down exit delay parameters specified in the SDRAM datasheet, i.e. max(tXP, tXARD, tXARDS) for DDR2 and max(tXP, tXPDLL) for DDR3. Valid values are 2 to 31.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcke",
                    description: Some(
                        "CKE minimum pulse width. Also specifies the minimum time that the SDRAM must remain in power down or self refresh mode. For DDR3 this parameter must be set to the value of tCKESR which is usually bigger than the value of tCKE. Valid values are 2 to 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdllk",
                    description: Some(
                        "DLL locking time. Valid values are 2 to 1023.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trtodt",
                    description: Some(
                        "Read to ODT delay (DDR3 only). Specifies whether ODT can be enabled immediately after the read post-amble or one clock delay has to be added. Valid values are: 0 = ODT may be turned on immediately after read post-amble 1 = ODT may not be turned on until one clock after the read post-amble If tRTODT is set to 1, then the read-to-write latency is increased by 1 if ODT is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trtw",
                    description: Some(
                        "Read to Write command delay. Valid values are: 0 = standard bus turn around delay 1 = add 1 clock to standard bus turn around delay This parameter allows the user to increase the delay between issuing Write commands to the SDRAM when preceded by Read commands. This provides an option to increase bus turn-around margin for high frequency systems.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tccd",
                    description: Some(
                        "Read to read and write to write command delay. Valid values are: 0 = BL/2 for DDR2 and 4 for DDR3 1 = BL/2 + 1 for DDR2 and 5 for DDR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dxccr",
            extends: None,
            description: Some(
                "“DATX8 Common Configuration Register (DXCCR)” on page 99.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dxodt",
                    description: Some(
                        "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the ODT configuration bit of the individual DATX8 (“DATX8 General Configuration Register (DXnGCR)” on page 148).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxiom",
                    description: Some(
                        "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the IOM configuration bit of the individual DATX8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdlen",
                    description: Some(
                        "Master Delay Line Enable: Enables, if set, all DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the MDLEN bit in the individual DATX8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxpdd1",
                    description: Some(
                        "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDD configuration bit of the individual DATX8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxpdr",
                    description: Some(
                        "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros. This bit is ORed with the PDR configuration bit of the individual DATX8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsres",
                    description: Some(
                        "DQS Resistor: Selects the on-die pull-down/pull-up resistor for DQS pins. DQSRES[3] selects pull-down (when set to 0) or pull-up (when set to 1). DQSRES[2:0] selects the resistor value. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsnres",
                    description: Some(
                        "DQS# Resistor: Selects the on-die pull-up/pull-down resistor for DQS# pins. Same encoding as DQSRES. Refer PHY databook for pull-down/pull-up resistor values (RA_SEL/RB_SEL) for DQS/DQS_b.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxsr",
                    description: Some(
                        "Data Slew Rate (D3F I/O Only): Selects slew rate of the I/O for DQ, DM, and DQS/DQS# pins of all DATX8 macros.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msbudq",
                    description: Some(
                        "Most Significant Byte Unused DQs: Specifies the number of DQ bits that are not used in the most significant byte. The used (valid) bits for this byte are [8-MSBDQ- 1:0]. To disable the whole byte, use the DXnGCR.DXEN register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udqodt",
                    description: Some(
                        "Unused DQ On-Die Termination: Enables, when set, the on-die termination on the I/O for unused DQ pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udqpdd1",
                    description: Some(
                        "Unused DQ Power Down Driver: Powers down, when set, the output driver on the I/O for unused DQ pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udqpdr",
                    description: Some(
                        "Unused DQ Power Down Receiver: Powers down, when set, the input receiver on the I/O for unused DQ pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udqiom",
                    description: Some(
                        "Unused DQ I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for unused DQ pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dyndxpdd1",
                    description: Some(
                        "Dynamic Data Power Down Driver: Dynamically powers down, when set, the output driver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDD and DXnGCR.DXPDD are not set to 1. Driver is powered-up on a DFI WRITE command and powered-down (twrlat + WL/2 + n) HDR cycles after the last DFI WRITE command. Note that n is defined by the register bit field DXCCR[27:24] (DDPDDCDO).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dyndxpdr",
                    description: Some(
                        "Data Power Down Receiver: Dynamically powers down, when set, the input receiver on I/O for the DQ pins of the active DATX8 macros. Applies only when DXPDR and DXnGCR.DXPDR are not set to 1. Receiver is powered-up on a DFI READ command and powered-down (trddata_en + fixed_read_latency + n) HDR cycles after the last DFI READ command. Note that n is defined by the register bit field DXCCR[31:28] (DDPDRCDO).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddpddcdo",
                    description: Some(
                        "Dynamic Data Power Down Driver Count Down Offset: Offset applied in calculating window of time where driver is powered up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddpdrcdo",
                    description: Some(
                        "Dynamic Data Power Down Receiver Count Down Offset: Offset applied in calculating window of time where receiver is powered up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Emr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "de",
                    description: Some(
                        "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dic",
                    description: Some(
                        "Output Driver Impedance Control: Controls the output drive strength. Valid values are: 0 = Full strength 1 = Reduced strength.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttl",
                    description: Some(
                        "On Die Termination low bit:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "al",
                    description: Some(
                        "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 000 = 0 001 = 1 010 = 2 011 = 3 100 = 4 101 = 5 All other settings are reserved and should not be used. The maximum allowed value of AL is tRCD-1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtth",
                    description: Some(
                        "On Die Termination high bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 00 = ODT disabled 01 = 75\u{f057} 10 = 150\u{f057} 11 = 50\u{f057} (some vendors).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ocd",
                    description: Some(
                        "Off-Chip Driver (OCD) Impedance Calibration: Used to calibrate and match pull-up to pull- down impedance to 18 \u{f057} nominal (refer to the SDRAM datasheet for details). Valid values are: 000 = OCD calibration mode exit 001 = Drive (1) pull-up 010 = Drive (0) pull-down 100 = OCD enter adjust mode 111 = OCD calibration default All other settings are reserved and should not be used. Note that OCD is not supported by all vendors. Refer to the SDRAM datasheet for details on the recommended OCD settings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqs",
                    description: Some(
                        "DQS_b Enable/Disable: When ‘0’, DQS_b is the complement of the differential data strobe pair DQS/DQS_b. When ‘1’, DQS is used in a single-ended mode and the DQS_b pin is disabled. Also used to similarly enable/disable RDQS_b if RDQS is enabled. The Controller does not allow the user to change this bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdqs",
                    description: Some(
                        "RDQS Enable/Disable: When enabled (‘1’), RDQS is identical in function and timing to data strobe DQS during a read, and ignored during a write. A ‘0’ disables the SDRAM from driving RDQS. The Controller does not allow the user to change this bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qoff",
                    description: Some(
                        "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Emr2",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pasr",
                    description: Some(
                        "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA[1:0] = 00 & 01) 010 = Quarter Array (BA[1:0] = 00) 011 = Not defined 100 = 3/4 Array (BA[1:0] = 01, 10, & 11) 101 = Half Array (BA[1:0] = 10 & 11) 110 = Quarter Array (BA[1:0] = 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA[2:0] = 000, 001, 010 & 011) 010 = Quarter Array (BA[2:0] = 000, 001) 011 = 1/8 Array (BA[2:0] = 000) 100 = 3/4 Array (BA[2:0] = 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA[2:0] = 100, 101, 110 & 111) 110 = Quarter Array (BA[2:0] = 110 & 111) 111 = 1/8 Array (BA[2:0] 111).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcc",
                    description: Some(
                        "Duty Cycle Corrector: Enables, if set, duty cycle correction within SDRAM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srf",
                    description: Some(
                        "Self Refresh Rate: Enables, if set, high temperature self refresh rate.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Emr3",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some(
                "“DATX8 General Configuration Register (DXnGCR)” on page 148.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dxen",
                    description: Some(
                        "Data Byte Enable: Enables, if set, the data byte. Setting this bit to '0' disables the byte, i.e. the byte is not used in PHY initialization or training and is ignored during SDRAM read/write operations.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsodt",
                    description: Some(
                        "DQS On-Die Termination: Enables, when set, the on-die termination on the I/O for DQS/DQS# pin of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0[9] is '0'.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqodt",
                    description: Some(
                        "Data On-Die Termination: Enables, when set, the on-die termination on the I/O for DQ and DM pins of the byte. This bit is ORed with the common DATX8 ODT configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99). Note: This bit is only valid when DXnGCR0[10] is '0'.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxiom",
                    description: Some(
                        "Data I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the IOM configuration bit of the individual DATX8(see “DATX8 Common Configuration Register (DXCCR)” on page 99).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxpdd1",
                    description: Some(
                        "Data Power Down Driver: Powers down, when set, the output driver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDD configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxpdr",
                    description: Some(
                        "Data Power Down Receiver: Powers down, when set, the input receiver on I/O for DQ, DM, and DQS/DQS# pins of the byte. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsrpd",
                    description: Some(
                        "DQSR Power Down: Powers down, if set, the PDQSR cell. This bit is ORed with the common PDR configuration bit (see “DATX8 Common Configuration Register (DXCCR)” on page 99).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsen",
                    description: Some(
                        "Write DQS Enable: Controls whether the write DQS going to the SDRAM is enabled (toggling) or disabled (static value) and whether the DQS is inverted. DQS# is always the inversion of DQS. These values are valid only when DQS/DQS# output enable is on, otherwise the DQS/DQS# is tristated. Valid settings are: 00 = Reserved 01 = DQS toggling with normal polarity (This should be the default setting) 10 = Reserved 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqsrtt",
                    description: Some(
                        "DQS Dynamic RTT Control: If set, the on die termination (ODT) control of the DQS/DQS# SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0[1] (DQSODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0').",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dqrtt",
                    description: Some(
                        "DQ Dynamic RTT Control: If set, the on die termination (ODT) control of the DQ/DM SSTL I/O is dynamically generated to enable the ODT during read operation and disabled otherwise. By setting this bit to '0' the dynamic ODT feature is disabled. To control ODT statically this bit must be set to '0' and DXnGCR0[2] (DQODT) is used to enable ODT (when set to '1') or disable ODT(when set to '0').",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttoh",
                    description: Some(
                        "RTT Output Hold: Indicates the number of clock cycles (from 0 to 3) after the read data postamble for which ODT control should remain set to DQSODT for DQS or DQODT for DQ/DM before disabling it (setting it to ‘0’) when using dynamic ODT control. ODT is disabled almost RTTOH clock cycles after the read postamble.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttoal",
                    description: Some(
                        "RTT On Additive Latency: Indicates when the ODT control of DQ/DQS SSTL I/Os is set to the value in DQODT/DQSODT during read cycles. Valid values are: 0 = ODT control is set to DQSODT/DQODT almost two cycles before read data preamble 1 = ODT control is set to DQSODT/DQODT almost one cycle before read data preamble.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxoeo",
                    description: Some(
                        "Data Byte Output Enable Override: Specifies whether the output I/O output enable for the byte lane should be set to a fixed value. Valid values are: 00 = No override. Output enable is controlled by DFI transactions 01 = output enable is asserted (I/O is forced to output mode). 10 = Output enable is de-asserted (I/O is forced to input mode) 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrst",
                    description: Some(
                        "PLL Rest: Resets the byte PLL by driving the PLL reset pin. This bit is not self- clearing and a '0' must be written to de-assert the reset. This bit is ORed with the global PLLRST configuration bit (see Table 3-10 on page 91).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllpd",
                    description: Some(
                        "PLL Power Down: Puts the byte PLL in power down mode by driving the PLL power down pin. This bit is not self-clearing and a '0' must be written to de-assert the power-down. This bit is ORed with the global PLLPD configuration bit (see Table 3-10 on page 91).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gshift",
                    description: Some(
                        "Gear Shift: Enables, if set, rapid locking mode on the byte PLL. This bit is ORed with the global GSHIFT configuration bit (see Table 3-10 on page 91).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllbyp",
                    description: Some(
                        "PLL Bypass: Puts the byte PLL in bypass mode by driving the PLL bypass pin. This bit is not self-clearing and a '0' must be written to de-assert the bypass. This bit is ORed with the global BYP configuration bit (see Table 3-10 on page 91).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlrken",
                    description: Some(
                        "Write Level Rank Enable: Specifies the ranks that should be write leveled for this byte. Write leveling responses from ranks that are not enabled for write leveling for a particular byte are ignored and write leveling is flagged as done for these ranks. WLRKEN[0] enables rank 0, [1] enables rank 1, [2] enables rank 2, and [3] enables rank 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdlen",
                    description: Some(
                        "Master Delay Line Enable: Enables, if set, the DATX8 master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high. This bit is ANDed with the common DATX8 MDL enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "calbyp",
                    description: Some(
                        "Calibration Bypass: Prevents, if set, period measurement calibration from automatically triggering after PHY initialization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gpr0",
            extends: None,
            description: Some(
                "General Purpose Register 0-1 (GPR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr0",
                    description: Some(
                        "General Purpose Register 0: General purpose register bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gpr1",
            extends: None,
            description: Some(
                "General Purpose Register 0-1 (GPR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpr1",
                    description: Some(
                        "General Purpose Register 1: General purpose register bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gsr0",
            extends: None,
            description: Some(
                "DATX8 General Status Registers 0-2 (DXnGSR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdqcal",
                    description: Some(
                        "Write DQ Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write DQ LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdqscal",
                    description: Some(
                        "Read DQS Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdqsncal",
                    description: Some(
                        "Read DQS# Calibration (Type B/B1 PHY Only): Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS# LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gdqscal",
                    description: Some(
                        "Read DQS gating Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the read DQS gating LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlcal",
                    description: Some(
                        "Write Leveling Calibration: Indicates, if set, that the DATX8 has finished doing period measurement calibration for the write leveling slave delay line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wldone",
                    description: Some(
                        "Write Leveling Done: Indicates, if set, that the DATX8 has completed write leveling.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlerr",
                    description: Some(
                        "Write Leveling Error: Indicates, if set, that there is a write leveling error in the DATX8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlprd",
                    description: Some(
                        "Write Leveling Period: Returns the DDR clock period measured by the write leveling LCDL during calibration. The measured period is used to generate the control of the write leveling pipeline which is a function of the write-leveling delay and the clock period. This value is PVT compensated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dplock",
                    description: Some(
                        "DATX8 PLL Lock: Indicates, if set, that the DATX8 PLL has locked. This is a direct status of the DATX8 PLL lock pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gdqsprd",
                    description: Some(
                        "Read DQS gating Period: Returns the DDR clock period measured by the read DQS gating LCDL during calibration. This value is PVT compensated.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qsgerr",
                    description: Some(
                        "DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training. One bit for each of the up to 4 ranks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wldq",
                    description: Some(
                        "Write Leveling DQ Status: Captures the write leveling DQ status from the DRAM during software write leveling.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gsr1",
            extends: None,
            description: Some(
                "DATX8 General Status Registers 0-2 (DXnGSR0-2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dltdone",
                    description: Some(
                        "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the DATX8 delay line digital test output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dltcode",
                    description: Some(
                        "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the DATX8 delay line digital test output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gsr2",
            extends: None,
            description: Some(
                "“DATX8 General Status Register 2 (DXnGSR2)” on page 152.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rderr",
                    description: Some(
                        "Read Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read bit deskew training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdwn",
                    description: Some(
                        "Read Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read bit deskew training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wderr",
                    description: Some(
                        "Write Bit Deskew Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write bit deskew training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdwn",
                    description: Some(
                        "Write Bit Deskew Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write bit deskew training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reerr",
                    description: Some(
                        "Read Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the read data eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rewn",
                    description: Some(
                        "Read Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the read data eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "weerr",
                    description: Some(
                        "Write Data Eye Training Error: Indicates, if set, that the DATX8 has encountered an error during execution of the write data eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wewn",
                    description: Some(
                        "Write Data Eye Training Warning: Indicates, if set, that the DATX8 has encountered a warning during execution of the write data eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "estat",
                    description: Some(
                        "Error Status: If an error occurred for this lane as indicated by RDERR, WDERR, REERR or WEERR the error status code can provide additional information regard when the error occurred during the algorithm execution.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gtr",
            extends: None,
            description: Some(
                "“DATX8 General Timing Register (DXnGTR)” on page 159.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "r0dgsl",
                    description: Some(
                        "Rank n DQS Gating System Latency: This is used to increase the number of clock cycles needed to expect valid DDR read data by up to seven extra clock cycles. This is used to compensate for board delays and other system delays. Power-up default is 000 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic DQS data training but these values can be overwritten by a direct write to this register. Every three bits of this register control the latency of each of the (up to) four ranks. R0DGSL controls the latency of rank 0, R1DGSL controls rank 1, and so on. Valid values are 0 to 7:.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r1dgsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r2dgsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r3dgsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r0wlsl",
                    description: Some(
                        "Rank n Write Leveling System Latency: This is used to adjust the write latency after write leveling. Power-up default is 01 (i.e. no extra clock cycles required). The SL fields are initially set by the PUB during automatic write leveling but these values can be overwritten by a direct write to this register. Every two bits of this register control the latency of each of the (up to) four ranks. R0WLSL controls the latency of rank 0, R1WLSL controls rank 1, and so on. Valid values: 00 = Write latency = WL - 1 01 = Write latency = WL 10 = Write latency = WL + 1 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r1wlsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r2wlsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r3wlsl",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcdlr0",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "r0wld",
                    description: Some(
                        "Rank 0 Write Leveling Delay: Rank 0 delay select for the write leveling (WL) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r1wld",
                    description: Some(
                        "Rank 1 Write Leveling Delay: Rank 1 delay select for the write leveling (WL) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r2wld",
                    description: Some(
                        "Rank 2 Write Leveling Delay: Rank 2 delay select for the write leveling (WL) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r3wld",
                    description: Some(
                        "Rank 3 Write Leveling Delay: Rank 3 delay select for the write leveling (WL) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcdlr1",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdqd",
                    description: Some(
                        "Write Data Delay: Delay select for the write data (WDQ) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdqsd",
                    description: Some(
                        "Read DQS Delay: Delay select for the read DQS (RDQS) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdqsnd",
                    description: Some(
                        "Read DQSN Delay (Type B/B1 PHY Only): Delay select for the read DQSN (RDQS) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcdlr2",
            extends: None,
            description: Some(
                "DATX8 Bit Delay Line Register 0-4 (DXnBDLR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "r0dqsgd",
                    description: Some(
                        "Rank 0 Read DQS Gating Delay: Rank 0 delay select for the read DQS gating (DQSG) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r1dqsgd",
                    description: Some(
                        "Rank 1 Read DQS Gating Delay: Rank 1 delay select for the read DQS gating (DQSG) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r2dqsgd",
                    description: Some(
                        "Rank 2 Read DQS Gating Delay: Rank 2 delay select for the read DQS gating (DQSG) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "r3dqsgd",
                    description: Some(
                        "Rank 3 Read DQS Gating Delay: Rank 3 delay select for the read DQS gating (DQSG) LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mdlr",
            extends: None,
            description: Some(
                "“DATX8 Master Delay Line Register (DXnMDLR)” on page 157.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprd",
                    description: Some(
                        "Initial Period: Initial period measured by the master delay line calibration for VT drift compensation. This value is used as the denominator when calculating the ratios of updates during VT compensation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tprd",
                    description: Some(
                        "Target Period: Target period measured by the master delay line calibration for VT drift compensation. This is the current measured value of the period and is continuously updated if the MDL is enabled to do so.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdld",
                    description: Some(
                        "MDL Delay: Delay select for the LCDL for the Master Delay Line.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bl",
                    description: Some(
                        "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: 010 = 4 011 = 8 All other settings are reserved and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bt",
                    description: Some(
                        "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cl",
                    description: Some(
                        "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 010 = 2 011 = 3 100 = 4 101 = 5 110 = 6 111 = 7 All other settings are reserved and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tm",
                    description: Some(
                        "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dr",
                    description: Some(
                        "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wr",
                    description: Some(
                        "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 001 = 2 010 = 3 011 = 4 100 = 5 101 = 6 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd",
                    description: Some(
                        "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Fast exit 1 = Slow exit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr0",
            extends: None,
            description: Some(
                "“Mode Register 0 (MR0)” on page 108.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bl",
                    description: Some(
                        "Burst Length: Determines the maximum number of column locations that can be accessed during a given read or write command. Valid values are: Valid values for DDR3 are: 00 = 8 (Fixed) 01 = 4 or 8 (On the fly) 10 = 4 (Fixed) 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cll",
                    description: Some(
                        "CAS Latency low bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bt",
                    description: Some(
                        "Burst Type: Indicates whether a burst is sequential (0) or interleaved (1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clh",
                    description: Some(
                        "CAS Latency: The delay between when the SDRAM registers a read command to when data is available. Valid values are: 0010 = 5 0100 = 6 0110 = 7 1000 = 8 1010 = 9 1100 = 10 1110 = 11 0001 = 12 0011 = 13 0101 = 14 All other settings are reserved and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tm",
                    description: Some(
                        "Operating Mode: Selects either normal operating mode (0) or test mode (1). Test mode is reserved for the manufacturer and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dr",
                    description: Some(
                        "DLL Reset: Writing a ‘1’ to this bit will reset the SDRAM DLL. This bit is self- clearing, i.e. it returns back to ‘0’ after the DLL reset has been issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wr",
                    description: Some(
                        "Write Recovery: This is the value of the write recovery. It is calculated by dividing the datasheet write recovery time, tWR (ns) by the datasheet clock cycle time, tCK (ns) and rounding up a non-integer value to the next integer. Valid values are: 000 = 16 001 = 5 010 = 6 011 = 7 100 = 8 101 = 10 110 = 12 111 = 14 All other settings are reserved and should not be used. NOTE: tWR (ns) is the time from the first SDRAM positive clock edge after the last data-in pair of a write command, to when a precharge of the same bank can be issued.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pd",
                    description: Some(
                        "Power-Down Control: Controls the exit time for power-down modes. Refer to the SDRAM datasheet for details on power-down modes. Valid values are: 0 = Slow exit (DLL off) 1 = Fast exit (DLL on).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr1",
            extends: None,
            description: Some(
                "“Mode Register 1 (MR1)” on page 111.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "de",
                    description: Some(
                        "DLL Enable/Disable: Enable (0) or disable (1) the DLL. DLL must be enabled for normal operation. Note: SDRAM DLL off mode is not supported.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dicl",
                    description: Some(
                        "Output Driver Impedance Control low bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttl",
                    description: Some(
                        "On Die Termination low bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "al",
                    description: Some(
                        "Posted CAS Additive Latency: Setting additive latency that allows read and write commands to be issued to the SDRAM earlier than normal (refer to the SDRAM datasheet for details). Valid values are: 00 = 0 (AL disabled) 01 = CL - 1 10 = CL - 2 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dich",
                    description: Some(
                        "Output Driver Impedance Control high bit: Controls the output drive strength. Valid values are: 00 = RZQ/6 01 = RZQ7 10 = Reserved 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttm",
                    description: Some(
                        "On Die Termination mid bit: Selects the effective resistance for SDRAM on die termination. Valid values are: 000 = ODT disabled 001 = RZQ/4 010 = RZQ/2 011 = RZQ/6 100 = RZQ/12 101 = RZQ/8 All other settings are reserved and should not be used. Bit on [9, 6,2].",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "level",
                    description: Some(
                        "Write Leveling Enable: Enables write-leveling when set.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtth",
                    description: Some(
                        "On Die Termination high bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdqs",
                    description: Some(
                        "Termination Data Strobe: When enabled (‘1’) TDQS provides additional termination resistance outputs that may be useful in some system configurations. Refer to the SDRAM datasheet for details.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qoff",
                    description: Some(
                        "Output Enable/Disable: When ‘0’, all outputs function normal; when ‘1’ all SDRAM outputs are disabled removing output buffer current. This feature is intended to be used for IDD characterization of read current and should not be used in normal operation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr2",
            extends: None,
            description: Some(
                "“Mode Register 2/Extended Mode Register 2 (MR2/EMR2)” on page 114.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pasr",
                    description: Some(
                        "Partial Array Self Refresh: Specifies that data located in areas of the array beyond the specified location will be lost if self refresh is entered. Valid settings for 4 banks are: 000 = Full Array 001 = Half Array (BA[1:0] = 00 & 01) 010 = Quarter Array (BA[1:0] = 00) 011 = Not defined 100 = 3/4 Array (BA[1:0] = 01, 10, & 11) 101 = Half Array (BA[1:0] = 10 & 11) 110 = Quarter Array (BA[1:0] = 11) 111 = Not defined Valid settings for 8 banks are: 000 = Full Array 001 = Half Array (BA[2:0] = 000, 001, 010 & 011) 010 = Quarter Array (BA[2:0] = 000, 001) 011 = 1/8 Array (BA[2:0] = 000) 100 = 3/4 Array (BA[2:0] = 010, 011, 100, 101, 110 & 111) 101 = Half Array (BA[2:0] = 100, 101, 110 & 111) 110 = Quarter Array (BA[2:0] = 110 & 111) 111 = 1/8 Array (BA[2:0] 111).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cwl",
                    description: Some(
                        "CAS Write Latency: The delay between when the SDRAM registers a write command to when write data is available. Valid values are: 000 = 5 (tCK > 2.5ns) 001 = 6 (2.5ns > tCK > 1.875ns) 010 = 7 (1.875ns > tCK> 1.5ns) 011 = 8 (1.5ns > tCK > 1.25ns) 100 = 9 (1.25ns > tCK > 1.07ns) 101 = 10 (1.07ns > tCK > 0.935ns) 110 = 11 (0.935ns > tCK > 0.833ns) 111 = 12 (0.833ns > tCK > 0.75ns) All other settings are reserved and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asr",
                    description: Some(
                        "Auto Self-Refresh: When enabled (‘1’), SDRAM automatically provides self-refresh power management functions for all supported operating temperature values. Otherwise the SRT bit must be programmed to indicate the temperature range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srt",
                    description: Some(
                        "Self-Refresh Temperature Range: Selects either normal (‘0’) or extended (‘1’) operating temperature range during self-refresh.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rttwr",
                    description: Some(
                        "Dynamic ODT: Selects RTT for dynamic ODT. Valid values are: 00 = Dynamic ODT off 01 = RZQ/4 10 = RZQ/2 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mr3",
            extends: None,
            description: Some(
                "“Mode Register 3 (MR3)” on page 116.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mprloc",
                    description: Some(
                        "Multi-Purpose Register (MPR) Location: Selects MPR data location: Valid value are: 00 = Predefined pattern for system calibration All other settings are reserved and should not be used.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mpr",
                    description: Some(
                        "Multi-Purpose Register Enable: Enables, if set, that read data should come from the Multi-Purpose Register. Otherwise read data come from the DRAM array.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Odtcr",
            extends: None,
            description: Some(
                "“ODT Configuration Register (ODTCR)” on page 117.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdodt0",
                    description: Some(
                        "Read ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a read command is sent to rank n. RDODT0, RDODT1, RDODT2, and RDODT3 specify ODT settings when a read is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to disable ODT during reads.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdodt1",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdodt2",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdodt3",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrodt0",
                    description: Some(
                        "Write ODT: Specifies whether ODT should be enabled (‘1’) or disabled (‘0’) on each of the up to four ranks when a write command is sent to rank n. WRODT0, WRODT1, WRODT2, and WRODT3 specify ODT settings when a write is to rank 0, rank 1, rank 2, and rank 3, respectively. The four bits of each field each represent a rank, the LSB being rank 0 and the MSB being rank 3. Default is to enable ODT only on rank being written to.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrodt1",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrodt2",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrodt3",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pgcr0",
            extends: None,
            description: Some(
                "PHY General Configuration Registers 0-1 (PGCR0- 1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wllvt",
                    description: Some(
                        "Write Leveling LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write leveling LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdlvt",
                    description: Some(
                        "Write DQ LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the write DQ LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdlvt",
                    description: Some(
                        "Read DQS LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rglvt",
                    description: Some(
                        "Read DQS Gating LCDL Delay VT Compensation: Enables, if set, the VT drift compensation of the read DQS gating LCDL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdbvt",
                    description: Some(
                        "Write Data BDL VT Compensation: Enables, if set, the VT drift compensation of the write data bit delay lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdbvt",
                    description: Some(
                        "Read Data BDL VT Compensation: Enables, if set, the VT drift compensation of the read data bit delay lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dltmode",
                    description: Some(
                        "Delay Line Test Mode: Selects, if set, the delay line oscillator test mode. Setting this bit also clears all delay line register values. For DL oscillator testing, first set this bit, then apply desired non-zero LCDL and BDL register programmings.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dltst",
                    description: Some(
                        "Delay Line Test Start: A write of '1' to this bit will trigger delay line oscillator mode period measurement. This bit is not self clearing and needs to be reset to '0' before the measurement can be re-triggered.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oscen",
                    description: Some(
                        "Oscillator Enable: Enables, if set, the delay line oscillation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oscdiv",
                    description: Some(
                        "Oscillator Mode Division: Specifies the factor by which the delay line oscillator mode output is divided down before it is output on the delay line digital test output pin dl_dto. Valid values are: 000 = Divide by 1 001 = Divide by 256 010 = Divide by 512 011 = Divide by 1024 100 = Divide by 2048 101 = Divide by 4096 110 = Divide by 8192 111 = Divide by 65536.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oscwdl",
                    description: Some(
                        "Oscillator Mode Write-Leveling Delay Line Select: Selects which of the two write leveling LCDLs is active. The delay select value of the inactive LCDL is set to zero while the delay select value of the active LCDL can be varied by the input write leveling delay select pin. Valid values are: 00 = No WL LCDL is active 01 = DDR WL LCDL is active 10 = SDR WL LCDL is active 11 = Both LCDLs are active.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtosel",
                    description: Some(
                        "Digital Test Output Select: Selects the PHY digital test output that is driven onto PHY digital test output (phy_dto) pin: Valid values are: 00000 = DATX8 0 PLL digital test output 00001 = DATX8 1 PLL digital test output 00010 = DATX8 2 PLL digital test output 00011 = DATX8 3 PLL digital test output 00100 = DATX8 4 PLL digital test output 00101 = DATX8 5 PLL digital test output 00110 = DATX8 6 PLL digital test output 00111 = DATX8 7 PLL digital test output 01000 = DATX8 8 PLL digital test output 01001 = AC PLL digital test output 01010 – 01111 = Reserved 10000 = DATX8 0 delay line digital test output 10001 = DATX8 1 delay line digital test output 10010 = DATX8 2 delay line digital test output 10011 = DATX8 3 delay line digital test output 10100 = DATX8 4 delay line digital test output 10101 = DATX8 5 delay line digital test output 10110 = DATX8 6 delay line digital test output 10111 = DATX8 7 delay line digital test output 11000 = DATX8 8 delay line digital test output 11001 = AC delay line digital test output 11010 – 11111 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pubmode",
                    description: Some(
                        "Enables, if set, the PUB to control the interface to the PHY and SDRAM. In this mode the DFI commands from the controller are ignored. The bit must be set to 0 after the system determines it is convenient to pass control of the DFI bus to the controller. When set to 0 the DFI interface has control of the PHY and SDRAM interface except when triggering pub operations such as BIST, DCU or data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cken",
                    description: Some(
                        "CK Enable: Controls whether the CK going to the SDRAM is enabled (toggling) or disabled (static value) and whether the CK is inverted. Two bits for each of the up to three CK pairs. Valid values for the two bits are: 00 = CK disabled (Driven to constant 0) 01 = CK toggling with inverted polarity 10 = CK toggling with normal polarity (This should be the default setting) 11 = CK disabled (Driven to constant 1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pgcr1",
            extends: None,
            description: Some(
                "PHY General Configuration Registers 0-1 (PGCR0- 1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pddisdx",
                    description: Some(
                        "Power Down Disabled Byte: Indicates, if set, that the PLL and I/Os of a disabled byte should be powered down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlmode",
                    description: Some(
                        "Write Leveling (Software) Mode: Indicates, if set, that the PUB is in software write leveling mode in which software executes single steps of DQS pulsing by writing '1' to PIR.WL. The write leveling DQ status from the DRAM is captured in DXnGSR0.WLDQ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlstep",
                    description: Some(
                        "Write Leveling Step: Specifies the number of delay step-size increments during each step of write leveling. Valid values are: 0 = computed to be 1/2 of the associated lane's DXnGSR0.WLPRD value 1 = 1 step size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wslopt",
                    description: Some(
                        "Write System Latency Optimization: controls the insertion of a pipeline stage on the AC signals from the DFI interface to the PHY to cater for a negative write system latency (WSL) value (only -1 possible). 0x0 = A pipeline stage is inserted only if WL2 training results in a WSL of -1 for any rank 0x1 = Inserts a pipeline stage.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "achrst",
                    description: Some(
                        "AC PHY High-Speed Reset: a Write of '0' to this bit resets the AC macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlselt",
                    description: Some(
                        "Write Leveling Select Type: Selects the encoding type for the write leveling select signal depending on the desired setup/hold margins for the internal pipelines. Refer to the DDR PHY Databook for details of how the select type is used. Valid values are: 0 = Type 1: Setup margin of 90 degrees and hold margin of 90 degrees 1 = Type 2: Setup margin of 135 degrees and hold margin of 45 degrees.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ioddrm",
                    description: Some(
                        "I/O DDR Mode (D3F I/O Only): Selects the DDR mode for the I/Os. These bits connect to bits [2:1] of the IOM pin of the SSTL I/O. For more information, refer to the SSTL I/O chapter in the DWC DDR PHY Databook.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdlen",
                    description: Some(
                        "Master Delay Line Enable: Enables, if set, the AC master delay line calibration to perform subsequent period measurements following the initial period measurements that are performed after reset or on when calibration is manually triggered. These additional measurements are accumulated and filtered as long as this bit remains high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpfen",
                    description: Some(
                        "Low-Pass Filter Enable: Enables, if set, the low pass filtering of MDL period measurements.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpfdepth",
                    description: Some(
                        "Low-Pass Filter Depth: Specifies the number of measurements over which MDL period measurements are filtered. This determines the time constant of the low pass filter. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdepth",
                    description: Some(
                        "Filter Depth: Specifies the number of measurements over which all AC and DATX8 initial period measurements, that happen after reset or when calibration is manually triggered, are averaged. Valid values are: 00 = 2 01 = 4 10 = 8 11 = 16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dldlmt",
                    description: Some(
                        "Delay Line VT Drift Limit: Specifies the minimum change in the delay line VT drift in one direction which should result in the assertion of the delay line VT drift status signal (vt_drift). The limit is specified in terms of delay select values. A value of 0 disables the assertion of delay line VT drift status signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcksel",
                    description: Some(
                        "Impedance Clock Divider Select: Selects the divide ratio for the clock used by the impedance control logic relative to the clock used by the memory controller and SDRAM. Valid values are: 00 = Divide by 2 01 = Divide by 8 10 = Divide by 32 11 = Divide by 64 For more information, refer to “Impedance Calibration” on page 174.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dxhrst",
                    description: Some(
                        "DX PHY High-Speed Reset: a Write of '0' to this bit resets the DX macro without resetting the PUB RTL logic. This bit is not self-clearing and a '1' must be written to de-assert the reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inhvt",
                    description: Some(
                        "VT Calculation Inhibit: Inhibits calculation of the next VT compensated delay line values. A value of 1 will inhibit the VT calculation. This bit should be set to 1 during writes to the delay line registers.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iolb",
                    description: Some(
                        "I/O Loop-Back Select: Selects where inside the I/O the loop-back of signals happens. Valid values are: 0 = Loopback is after output buffer; output enable must be asserted 1 = Loopback is before output buffer; output enable is don’t care.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbdqss",
                    description: Some(
                        "Loopback DQS Shift: Selects how the read DQS is shifted during loopback to ensure that the read DQS is centered into the read data eye. Valid values are: 1b0 = PUB sets the read DQS LCDL to 0 (internally). DQS is already shifted 90 degrees by write path 1b1 = The read DQS shift is set manually through software.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbgdqs",
                    description: Some(
                        "Loopback DQS Gating: Selects the DQS gating mode that should be used when the PHY is in loopback mode, including BIST loopback mode. Valid values are: 00 = DQS gate is always on 01 = DQS gate training will be triggered on the PUB 10 = DQS gate is set manually using software 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbmode",
                    description: Some(
                        "Loopback Mode: Indicates, if set, that the PHY/PUB is in loopback mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pgcr2",
            extends: None,
            description: Some(
                "“PHY General Configuration Register 2 (PGCR2)” on page 87.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trefprd",
                    description: Some(
                        "Refresh Period: Indicates the period, after which the PUB has to issue a refresh command to the SDRAM. This is derived from the maximum refresh interval from the datasheet, tRFC(max) or REFI, divided by the clock cycle time. A further 400 clocks must be subtracted from the derived number to account for command flow and missed slots of refreshes in the internal PUB blocks. The default corresponds to DDR3 9*7.8us at 1066MHz when a burst of 9 refreshes are issued at every refresh interval.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nobub",
                    description: Some(
                        "No Bubbles: Specified whether reads should be returned to the controller with no bubbles. Enabling no-bubble reads increases the read latency. Valid values are: 0 = Bubbles are allowed during reads 1 = Bubbles are not allowed during reads.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fxdlat",
                    description: Some(
                        "Fixed Latency: Specified whether all reads should be returned to the controller with a fixed read latency. Enabling fixed read latency increases the read latency. Valid values are: 0 = Disable fixed read latency 1 = Enable fixed read latency Fixed read latency is calculated as (12 + (maximum DXnGTR.RxDGSL)/2) HDR clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtpmxtmr",
                    description: Some(
                        "Data Training PUB Mode Timer Exit: Specifies the number of controller clocks to wait when entering and exiting pub mode data training. The default value ensures controller refreshes do not cause memory model errors when entering and exiting data training. The value should be increased if controller initiated SDRAM ZQ short or long operation may occur just before or just after the execution of data training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shrac",
                    description: Some(
                        "Shared-AC mode: set to 1 to enable shared address/command mode with two independent data channels – available only if shared address/command mode support is compiled in.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acpddc",
                    description: Some(
                        "AC Power-Down with Dual Channels: Set to 1 to power-down address/command lane when both data channels are powered-down. Only valid in shared-AC mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpmstrc0",
                    description: Some(
                        "Low-Power Master Channel 0: set to 1 to have channel 0 act as master to drive channel 1 low-power functions simultaneously. Only valid in shared-AC mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dynacpdd1",
                    description: Some(
                        "Dynamic AC Power Down Driver: Powers down, when set, the output driver on I/O for ADDR and BA. This bit is ORed with bit ACIOCR[3] (ACPDD).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pgsr0",
            extends: None,
            description: Some(
                "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idone",
                    description: Some(
                        "Initialization Done: Indicates, if set, that the DDR system initialization has completed. This bit is set after all the selected initialization routines in PIR register have completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pldone",
                    description: Some(
                        "PLL Lock Done: Indicates, if set, that PLL locking has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcdone",
                    description: Some(
                        "Digital Delay Line (DDL) Calibration Done: Indicates, if set, that DDL calibration has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcdone",
                    description: Some(
                        "Impedance Calibration Done: Indicates, if set, that impedance calibration has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "didone",
                    description: Some(
                        "DRAM Initialization Done: Indicates, if set, that DRAM initialization has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wldone",
                    description: Some(
                        "Write Leveling Done: Indicates, if set, that write leveling has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qsgdone",
                    description: Some(
                        "Read DQS Gate Training Done: Indicates, if set, that DQS gate training has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wladone",
                    description: Some(
                        "Write Leveling Adjustment Done: Indicates, if set, that write leveling adjustment has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rddone",
                    description: Some(
                        "Read Data Bit Deskew Done: Indicates, if set, that read bit deskew has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wddone",
                    description: Some(
                        "Write Data Bit Deskew Done: Indicates, if set, that write bit deskew has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "redone",
                    description: Some(
                        "Read Data Eye Training Done: Indicates, if set, that read eye training has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wedone",
                    description: Some(
                        "Write Data Eye Training Done: Indicates, if set, that write eye training has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcerr",
                    description: Some(
                        "Impedance Calibration Error: Indicates, if set, that there is an error in impedance calibration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlerr",
                    description: Some(
                        "Write Leveling Error: Indicates, if set, that there is an error in write leveling.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qsgerr",
                    description: Some(
                        "Read DQS Gate Training Error: Indicates, if set, that there is an error in DQS gate training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wlaerr",
                    description: Some(
                        "Write Data Leveling Adjustment Error: Indicates, if set, that there is an error in write leveling adjustment.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rderr",
                    description: Some(
                        "Read Data Bit Deskew Error: Indicates, if set, that there is an error in read bit deskew.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wderr",
                    description: Some(
                        "Write Data Bit Deskew Error: Indicates, if set, that there is an error in write bit deskew.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reerr",
                    description: Some(
                        "Read Data Eye Training Error: Indicates, if set, that there is an error in read eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "weerr",
                    description: Some(
                        "Write Eye Training Error: Indicates, if set, that there is an error in write eye training.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pldone_chn",
                    description: Some(
                        "PLL Lock Done per Channel: Indicates PLL locking has completed for each underlying channel. Bit 28 represents channel 0 while bit 29 represents channel 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aplock",
                    description: Some(
                        "AC PLL Lock: Indicates, if set, that AC PLL has locked. This is a direct status of the AC PLL lock pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pgsr1",
            extends: None,
            description: Some(
                "“PHY General Status Registers 0-1 (PGSR0-1)” on page 89.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dltdone",
                    description: Some(
                        "Delay Line Test Done: Indicates, if set, that the PHY control block has finished doing period measurement of the AC delay line digital test output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dltcode",
                    description: Some(
                        "Delay Line Test Code: Returns the code measured by the PHY control block that corresponds to the period of the AC delay line digital test output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vtstop",
                    description: Some(
                        "VT Stop: Indicates, if set, that the VT calculation logic has stopped computing the next values for the VT compensated delay line values. After assertion of the PGCR.INHVT, the VTSTOP bit should be read to ensure all VT compensation logic has stopped computations before writing to the delay line registers.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parerr",
                    description: Some(
                        "RDIMM Parity Error: Indicates, if set, that there was a parity error (i.e. err_out_n was sampled low) during one of the transactions to the RDIMM buffer chip. This bit remains asserted until cleared by the PIR.CLRSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pir",
            extends: None,
            description: Some(
                "PHY Initialization Register (PIR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "init",
                    description: Some(
                        "Initialization Trigger: A write of '1' to this bit triggers the DDR system initialization, including PHY initialization, DRAM initialization, and PHY training. The exact initialization steps to be executed are specified in bits 1 to 15 of this register. A bit setting of 1 means the step will be executed as part of the initialization sequence, while a setting of ‘0’ means the step will be bypassed. The initialization trigger bit is self-clearing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcal",
                    description: Some(
                        "Impedance Calibration: Performs PHY impedance calibration. When set the impedance calibration will be performed in parallel with PHY initialization (PLL initialization + DDL calibration + PHY reset).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllinit",
                    description: Some(
                        "PLL Initialization: Executes the PLL initialization sequence which includes correct driving of PLL power-down, reset and gear shift pins, and then waiting for the PHY PLLs to lock.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcal",
                    description: Some(
                        "Digital Delay Line (DDL) Calibration: Performs PHY delay line calibration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phyrst",
                    description: Some(
                        "PHY Reset: Resets the AC and DATX8 modules by asserting the AC/DATX8 reset pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dramrst",
                    description: Some(
                        "DRAM Reset (DDR3 Only): Issues a reset to the DRAM (by driving the DRAM reset pin low) and wait 200us. This can be triggered in isolation or with the full DRAM initialization (DRAMINIT). For the later case, the reset is issued and 200us is waited before starting the full initialization sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "draminit",
                    description: Some(
                        "DRAM Initialization: Executes the DRAM initialization sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wl",
                    description: Some(
                        "Write Leveling (DDR3 Only): Executes a PUB write leveling routine.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qsgate",
                    description: Some(
                        "Read DQS Gate Training: Executes a PUB training routine to determine the optimum position of the read data DQS strobe for maximum system timing margins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wladj",
                    description: Some(
                        "Write Leveling Adjust (DDR3 Only): Executes a PUB training routine that re- adjusts the write latency used during write in case the write leveling routine changed the expected latency. Note: Ensure that the DCU command cache is cleared prior to running WLADJ.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rddskw",
                    description: Some(
                        "Read Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during read.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrdskw",
                    description: Some(
                        "Write Data Bit Deskew: Executes a PUB training routine to deskew the DQ bits during write.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdeye",
                    description: Some(
                        "Read Data Eye Training: Executes a PUB training routine to maximize the read data eye.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wreye",
                    description: Some(
                        "Write Data Eye Training: Executes a PUB training routine to maximize the write data eye.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icpc",
                    description: Some(
                        "Initialization Complete Pin Configuration: Specifies how the DFI initialization complete output pin (dfi_init_complete) should be used to indicate the status of initialization. Valid value are: 0 = Asserted after PHY initialization (DLL locking and impedance calibration) is complete. 1 = Asserted after PHY initialization is complete and the triggered the PUB initialization (DRAM initialization, data training, or initialization trigger with no selected initialization) is complete.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllbyp",
                    description: Some(
                        "PLL Bypass: A setting of 1 on this bit will put all PHY PLLs in bypass mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctldinit",
                    description: Some(
                        "Controller DRAM Initialization: Indicates, if set, that DRAM initialization will be performed by the controller. Otherwise if not set it indicates that DRAM initialization will be performed using the built-in initialization sequence or using software through the configuration port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdimminit",
                    description: Some(
                        "RDIMM Initialization: Executes the RDIMM buffer chip initialization before executing DRAM initialization. The RDIMM buffer chip initialization is run after the DRAM is reset and CKE have been driven high by the DRAM initialization sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clrsr",
                    description: Some(
                        "Clear Status Registers: Writing 1 to this bit clears (reset to 0) select status bits in register PGSR0. This bit is primarily for debug purposes and is typically not needed during normal functional operation. It can be used when PGSR.IDONE=1, to manually clear a selection of the PGSR status bits, although starting a new initialization process (PIR[0].INIT = 1’b1) automatically clears the PGSR status bits associated with the initialization steps enabled. The following list describes which bits within the PGSR0 register are cleared when CLRSR is set to 1’b1 and which bits are not cleared: The following bits are not cleared by PIR[27] (CLRSR): PGSR0[31] (APLOCK) PGSR0[29:28] (PLDONE_CHN) PGSR0[23] (WLAERR) PGSR0[21] (WLERR) PGSR0[4] (DIDONE) PGSR0[2] (DCDONE) PGSR0[1] (PLDONE) PGSR0[0] (IDONE) The following bits are always zero: PGSR0[30] (reserved) PGSR0[19:12] (reserved) The following bits are cleared unconditionally by PIR[27] (CLRSR): PGSR0[27] (WEERR) PGSR0[26] (REERR) PGSR0[25] (WDERR) PGSR0[24] (RDERR) - PGSR0[22] (QSGERR) - PGSR0[20] (ZCERR) - PGSR0[11] (WEDONE) - PGSR0[10] (REDONE) - PGSR0[9] (WDDONE) - PGSR0[8] (RDDONE) - PGSR0[7] (WLADONE) - PGSR0[6] (QSGDONE) - PGSR0[5] (WLDONE) - PGSR0[3] (ZCDONE).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lockbyp",
                    description: Some(
                        "PLL Lock Bypass: Bypasses or stops, if set, the waiting of PLLs to lock. PLL lock wait is automatically triggered after reset. PLL lock wait may be triggered manually using INIT and PLLINIT bits of the PIR register. This bit is self-clearing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcalbyp",
                    description: Some(
                        "Digital Delay Line (DDL) Calibration Bypass: Bypasses or stops, if set, DDL calibration that automatically triggers after reset. DDL calibration may be triggered manually using INIT and DCAL bits of the PIR register. This bit is self- clearing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zcalbyp",
                    description: Some(
                        "Impedance Calibration Bypass: Bypasses or stops, if set, impedance calibration of all ZQ control blocks that automatically triggers after reset. Impedance calibration may be triggered manually using INIT and ZCAL bits of the PIR register. This bit is self-clearing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "initbyp",
                    description: Some(
                        "Initialization Bypass: Bypasses or stops, if set, all initialization routines currently running, including PHY initialization, DRAM initialization, and PHY training. Initialization may be triggered manually using INIT and the other relevant bits of the PIR register. This bit is self-clearing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcr",
            extends: None,
            description: Some(
                "“PLL Control Register (PLLCR)” on page 91.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtc",
                    description: Some(
                        "Digital Test Control: Selects various PLL digital test signals and other test mode signals to be brought out via bit [1] of the PLL digital test output (pll_dto[1]). Valid values are: 00 = ‘0’ (Test output is disabled) 01 = PLL x1 clock (X1) 10 = PLL reference (input) clock (REF_CLK) 11 = PLL feedback clock (FB_X1).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atc",
                    description: Some(
                        "Analog Test Control: Selects various PLL analog test signals to be brought out via PLL analog test output pin (pll_ato). Valid values are: 0000 = Reserved 0001 = vdd_ckin 0010 = vrfbf 0011 = vdd_cko 0100 = vp_cp 0101 = vpfil(vp) 0110 = Reserved 0111 = gd 1000 = vcntrl_atb 1001 = vref_atb 1010 = vpsf_atb 1011 – 1111 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "atoen",
                    description: Some(
                        "Analog Test Enable (ATOEN): Selects the analog test signal that is driven on the analog test output pin. Otherwise the analog test output is tri-stated. This allows analog test output pins from multiple PLLs to be connected together. Valid values are: 0000 = All PLL analog test signals are tri-stated 0001 = AC PLL analog test signal is driven out 0010 = DATX8 0 PLL analog test signal is driven out 0011 = DATX8 1 PLL analog test signal is driven out 0100 = DATX8 2 PLL analog test signal is driven out 0101 = DATX8 3 PLL analog test signal is driven out 0110 = DATX8 4 PLL analog test signal is driven out 0111 = DATX8 5 PLL analog test signal is driven out 1000 = DATX8 6 PLL analog test signal is driven out 1001 = DATX8 7 PLL analog test signal is driven out 1010 = DATX8 8 PLL analog test signal is driven out 1011 – 1111 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gshift",
                    description: Some(
                        "Gear Shift: Enables, if set, rapid locking mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpic",
                    description: Some(
                        "Charge Pump Integrating Current Control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cppc",
                    description: Some(
                        "Charge Pump Proportional Current Control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qpmode",
                    description: Some(
                        "PLL Quadrature Phase Mode: Enables, if set, the quadrature phase clock outputs. This mode is not used in this version of the PHY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "frqsel",
                    description: Some(
                        "PLL Frequency Select: Selects the operating range of the PLL. Valid values for PHYs that go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 335MHz to 533MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 225MHz to 385MHz 10 = Reserved 11 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 275MHz Valid values for PHYs that don’t go up to 2133 Mbps are: 00 = PLL reference clock (ctl_clk/REF_CLK) ranges from 250MHz to 400MHz 01 = PLL reference clock (ctl_clk/REF_CLK) ranges from 166MHz to 300MHz 10 = Reserved 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllpd",
                    description: Some(
                        "PLL Power Down: Puts the PLLs in power down mode by driving the PLL power down pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the power-down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrst",
                    description: Some(
                        "PLL Rest: Resets the PLLs by driving the PLL reset pin. This bit is not self-clearing and a ‘0’ must be written to de-assert the reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byp",
                    description: Some(
                        "PLL Bypass: Bypasses the PLL, if set, to 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptr0",
            extends: None,
            description: Some(
                "PHY Timing Registers 0-4 (PTR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tphyrst",
                    description: Some(
                        "PHY Reset Time: Number of configuration or APB clock cycles that the PHY reset must remain asserted after PHY calibration is done before the reset to the PHY is de-asserted. This is used to extend the reset to the PHY so that the reset is asserted for some clock cycles after the clocks are stable. Valid values are from 1 to 63 (the value must be non-zero).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpllgs",
                    description: Some(
                        "PLL Gear Shift Time: Number of configuration or APB clock cycles from when the PLL reset pin is de-asserted to when the PLL gear shift pin is de-asserted. This must correspond to a value that is equal to or more than 4us. Default value corresponds to 4us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpllpd",
                    description: Some(
                        "PLL Power-Down Time: Number of configuration or APB clock cycles that the PLL must remain in power-down mode, i.e. number of clock cycles from when PLL power-down pin is asserted to when PLL power-down pin is de-asserted. This must correspond to a value that is equal to or more than 1us. Default value corresponds to 1us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptr1",
            extends: None,
            description: Some(
                "PHY Timing Registers 0-4 (PTR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpllrst",
                    description: Some(
                        "PLL Reset Time: Number of configuration or APB clock cycles that the PLL must remain in reset mode, i.e. number of clock cycles from when PLL power-down pin is de-asserted and PLL reset pin is asserted to when PLL reset pin is de-asserted. The setting must correspond to a value that is equal to, or greater than, 3us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tplllock",
                    description: Some(
                        "PLL Lock Time: Number of configuration or APB clock cycles for the PLL to stabilize and lock, i.e. number of clock cycles from when the PLL reset pin is de-asserted to when the PLL has lock and is ready for use. This must correspond to a value that is equal to or more than 100us. Default value corresponds to 100us.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptr2",
            extends: None,
            description: Some(
                "PHY Timing Registers 0-4 (PTR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcalon",
                    description: Some(
                        "Calibration On Time: Number of clock cycles that the calibration clock is enabled (cal_clk_en asserted).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcals",
                    description: Some(
                        "Calibration Setup Time: Number of controller clock cycles from when calibration is enabled (cal_en asserted) to when the calibration clock is asserted again (cal_clk_en asserted).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcalh",
                    description: Some(
                        "Calibration Hold Time: Number of controller clock cycles from when the clock was disabled (cal_clk_en deasserted) to when calibration is enable (cal_en asserted).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "twldlys",
                    description: Some(
                        "Write Leveling Delay Settling Time: Number of controller clock cycles from when a new value of the write leveling delay is applies to the LCDL to when to DQS high is driven high. This allows the delay to settle.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptr3",
            extends: None,
            description: Some(
                "PHY Timing Registers 0-4 (PTR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdinit0",
                    description: Some(
                        "DRAM Initialization Time 0: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE low time with power and clock stable (500 us) DDR2 = CKE low time with power and clock stable (200 us) Default value corresponds to DDR3 500 us at 1066 MHz. During Verilog simulations, it is recommended that this value is changed to a much smaller value in order to avoid long simulation times. However, this may cause a memory model error, due to a violation of the CKE setup sequence. This violation is expected if this value is not programmed to the required SDRAM CKE low time, but memory models should be able to tolerate this violation without malfunction of the model.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdinit1",
                    description: Some(
                        "DRAM Initialization Time 1: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = CKE high time to first command (tRFC + 10 ns or 5 tCK, whichever is bigger) DDR2 = CKE high time to first command (400 ns) Default value corresponds to DDR3 tRFC of 360ns at 1066 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ptr4",
            extends: None,
            description: Some(
                "PHY Timing Registers 0-4 (PTR0-4).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdinit2",
                    description: Some(
                        "DRAM Initialization Time 2: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Reset low time (200 us on power-up or 100 ns after power-up) Default value corresponds to DDR3 200 us at 1066 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 18,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdinit3",
                    description: Some(
                        "DRAM Initialization Time 3: DRAM initialization time in DRAM clock cycles corresponding to the following: DDR3 = Time from ZQ initialization command to first command (1 us) Default value corresponds to the DDR3 640ns at 1066 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdimmcr0",
            extends: None,
            description: Some(
                "RDIMM Control Register 0-1 (RDIMMCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rc0",
                    description: Some(
                        "Control Word 0 (Global Features Control Word): Bit definitions are: RC0[0]: 0 = Output inversion enabled, 1 = Output inversion disabled. RC0[1]: 0 = Floating outputs disabled, 1 = Floating outputs enabled. RC0[2]: 0 = A outputs enabled, 1 = A outputs disabled. RC0[3]: 0 = B outputs enabled, 1 = B outputs disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc1",
                    description: Some(
                        "Control Word 1 (Clock Driver Enable Control Word): Bit definitions are: RC1[0]: 0 = Y0/Y0# clock enabled, 1 = Y0/Y0# clock disabled. RC1[1]: 0 = Y1/Y1# clock enabled, 1 = Y1/Y1# clock disabled. RC1[2]: 0 = Y2/Y2# clock enabled, 1 = Y2/Y2# clock disabled. RC1[3]: 0 = Y3/Y3# clock enabled, 1 = Y3/Y3# clock disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc2",
                    description: Some(
                        "Control Word 2 (Timing Control Word): Bit definitions are: RC2[0]: 0 = Standard (1/2 clock) pre-launch, 1 = Prelaunch controlled by RC12. RC2[1]: 0 = Reserved. RC2[2]: 0 = 100 Ohm input bus termination, 1 = 150 Ohm input bus termination. RC2[3]: 0 = Operation frequency band 1, 1 = Test mode frequency band 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc3",
                    description: Some(
                        "Control Word 3 (Command/Address Signals Driver Characteristics Control Word): RC3[1:0] is driver settings for command/address A outputs, and RC3[3:2] is driver settings for command/address B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc4",
                    description: Some(
                        "Control Word 4 (Control Signals Driver Characteristics Control Word): RC4[1:0] is driver settings for control A outputs, and RC4[3:2] is driver settings for control B outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Reserved 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc5",
                    description: Some(
                        "Control Word 5 (CK Driver Characteristics Control Word): RC5[1:0] is driver settings for clock Y1, Y1#, Y3, and Y3# outputs, and RC5[3:2] is driver settings for clock Y0, Y0#, Y2, and Y2# outputs. Bit definitions are: 00 = Light drive (4 or 5 DRAM loads) 01 = Moderate drive (8 or 10 DRAM loads) 10 = Strong drive (16 or 20 DRAM loads) 11 = Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc6",
                    description: Some(
                        "Control Word 6: Reserved, free to use by vendor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc7",
                    description: Some(
                        "Control Word 7: Reserved, free to use by vendor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdimmcr1",
            extends: None,
            description: Some(
                "RDIMM Control Register 0-1 (RDIMMCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rc8",
                    description: Some(
                        "Control Word 8 (Additional Input Bus Termination Setting Control Word): RC8[2:0] is Input Bus Termination (IBT) setting as follows: 000 = IBT as defined in RC2. 001 = Reserved 010 = 200 Ohm 011 = Reserved 100 = 300 Ohm 101 = Reserved 110 = Reserved 111 = Off RC8[3]: 0 = IBT off when MIRROR is HIGH, 1 = IBT on when MIRROR is high.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc9",
                    description: Some(
                        "Control Word 9 (Power Saving Settings Control Word): Bit definitions are: RC9[0]: 0 = Floating outputs as defined in RC0, 1 = Weak drive enabled. RC9[1]: 0 = Reserved. RC9[2]: 0 = CKE power down with IBT ON, QxODT is a function of DxODT, 1 = CKE power down with IBT off, QxODT held LOW. RC9[2] is valid only when RC9[3] is 1. RC9[3]: 0 = CKE power down mode disabled, 1 = CKE power down mode enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc10",
                    description: Some(
                        "Control Word 10 (RDIMM Operating Speed Control Word): RC10[2:0] is RDIMM operating speed setting as follows: 000 = DDR3/DDR3L-800 001 = DDR3/DDR3L-1066 010 = DDR3/DDR3L-1333 011 = DDR3/DDR3L-1600 100 = Reserved 101 = Reserved 110 = Reserved 111 = Reserved RC10[3]: Don’t care.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc11",
                    description: Some(
                        "Control Word 11 (Operating Voltage VDD Control Word): RC10[1:0] is VDD operating voltage setting as follows: 00 = DDR3 1.5V mode 01 = DDR3L 1.35V mode 10 = Reserved 11 = Reserved RC10[3:2]: Reserved.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc12",
                    description: Some(
                        "Control Word 12: Reserved for future use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc13",
                    description: Some(
                        "Control Word 13: Reserved for future use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc14",
                    description: Some(
                        "Control Word 14: Reserved for future use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rc15",
                    description: Some(
                        "Control Word 15: Reserved for future use.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdimmgcr0",
            extends: None,
            description: Some(
                "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdimm",
                    description: Some(
                        "Registered DIMM: Indicates, if set, that a registered DIMM is used. In this case, the PUB increases the SDRAM write and read latencies (WL/RL) by 1 and also enforces that accesses adhere to RDIMM buffer chip. This only applies to PUB internal SDRAM transactions. Transactions generated by the controller must make its own adjustments to WL/RL when using a registered DIMM. The DCR.NOSRA register bit must be set to ‘1’ if using the standard RDIMM buffer chip so that normal DRAM accesses do not assert multiple chip select bits at the same time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "errnoreg",
                    description: Some(
                        "Parity Error No Registering: Indicates, if set, that parity error signal from the RDIMM should be passed to the DFI controller without any synchronization or registering. Otherwise, the error signal is synchronized as shown in Figure 4-30 on page 262.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "soperr",
                    description: Some(
                        "Stop On Parity Error: Indicates, if set, that the PUB is to stop driving commands to the DRAM upon encountering a parity error. Transactions can resume only after status is cleared via PIR.CLRSR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parinodt",
                    description: Some(
                        "PAR_IN On-Die Termination: Enables, when set, the on-die termination on the I/O for PAR_IN pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parinpdd",
                    description: Some(
                        "PAR_IN Power Down Driver: Powers down, when set, the output driver on the I/O for PAR_IN pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parinpdr",
                    description: Some(
                        "PAR_IN Power Down Receiver: Powers down, when set, the input receiver on the I/O for PAR_IN pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pariniom",
                    description: Some(
                        "PAR_IN I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for PAR_IN pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "parinoe",
                    description: Some(
                        "PAR_IN Output Enable: Enables, when set, the output driver on the I/O for PAR_IN pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erroutodt",
                    description: Some(
                        "ERROUT# On-Die Termination: Enables, when set, the on-die termination on the I/O for ERROUT# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erroutpdd",
                    description: Some(
                        "ERROUT# Power Down Driver: Powers down, when set, the output driver on the I/O for ERROUT# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erroutpdr",
                    description: Some(
                        "ERROUT# Power Down Receiver: Powers down, when set, the input receiver on the I/O for ERROUT# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erroutiom",
                    description: Some(
                        "ERROUT# I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for ERROUT# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erroutoe",
                    description: Some(
                        "ERROUT# Output Enable: Enables, when set, the output driver on the I/O for ERROUT# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdimmodt",
                    description: Some(
                        "RDIMM Outputs On-Die Termination: Enables, when set, the on-die termination on the I/O for QCSEN# and MIRROR pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdimmpdd",
                    description: Some(
                        "RDIMM Outputs Power Down Driver: Powers down, when set, the output driver on the I/O for QCSEN# and MIRROR pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdimmpdr",
                    description: Some(
                        "RDIMM Outputs Power Down Receiver: Powers down, when set, the input receiver on the I/O for QCSEN# and MIRROR pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdimmiom",
                    description: Some(
                        "RDIMM Outputs I/O Mode: Selects SSTL mode (when set to 0) or CMOS mode (when set to 1) of the I/O for QCSEN# and MIRROR pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qcsenoe",
                    description: Some(
                        "QCSEN# Output Enable: Enables, when set, the output driver on the I/O for QCSEN# pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mirroroe",
                    description: Some(
                        "MIRROR Output Enable: Enables, when set, the output driver on the I/O for MIRROR pin.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qcsen",
                    description: Some(
                        "RDMIMM Quad CS Enable: Enables, if set, the Quad CS mode for the RDIMM registering buffer chip. This register bit controls the buffer chip QCSEN# signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mirror",
                    description: Some(
                        "RDIMM Mirror: Selects between two different ballouts of the RDIMM buffer chip for front or back operation. This register bit controls the buffer chip MIRROR signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdimmgcr1",
            extends: None,
            description: Some(
                "RDIMM General Configuration Register 0-1 (RDIMMGCR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbcstab",
                    description: Some(
                        "Stabilization time: Number of DRAM clock cycles for the RDIMM buffer chip to stabilize. This parameter corresponds to the buffer chip tSTAB parameter. Default value is in decimal format and corresponds to 6us at 533MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tbcmrd",
                    description: Some(
                        "Command word to command word programming delay: Number of DRAM clock cycles between two RDIMM buffer chip command programming accesses. The value used for tBCMRD is 8 plus the value programmed in these bits, i.e. tBCMRD value ranges from 8 to 15. This parameter corresponds to the buffer chip tMRD parameter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crinit",
                    description: Some(
                        "Control Registers Initialization Enable: Indicates which RDIMM buffer chip control registers (RC0 to RC15) should be initialized (written) when the PUB is triggered to initialize the buffer chip. A setting of ‘1’ on CRINIT[n] bit means that CRn should be written during initialization.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ridr",
            extends: None,
            description: Some(
                "Revision Identification Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pubmnr",
                    description: Some(
                        "PUB Minor Revision: Indicates minor update of the PUB such as bug fixes. Normally no new features are included.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pubmdr",
                    description: Some(
                        "PUB Moderate Revision: Indicates moderate revision of the PUB such as addition of new features. Normally the new version is still compatible with previous versions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pubmjr",
                    description: Some(
                        "PUB Major Revision: Indicates major revision of the PUB such addition of the features that make the new version not compatible with previous versions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phymnr",
                    description: Some(
                        "PHY Minor Revision: Indicates minor update of the PHY such as bug fixes. Normally no new features are included.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phymdr",
                    description: Some(
                        "PHY Moderate Revision: Indicates moderate revision of the PHY such as addition of new features. Normally the new version is still compatible with previous versions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phymjr",
                    description: Some(
                        "PHY Major Revision: Indicates major revision of the PHY such addition of the features that make the new version not compatible with previous versions.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "udrid",
                    description: Some(
                        "User-Defined Revision ID: General purpose revision identification set by the user.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr0",
            extends: None,
            description: Some(
                "Impedance Status Register 0-1 (ZQnSR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zctrl",
                    description: Some(
                        "Impedance Control: Current value of impedance control. ZCTRL field mapping for D3F I/Os is as follows: ZCTRL[27:21] is used to select the pull-up on-die termination impedance ZCTRL[20:14] is used to select the pull-down on-die termination impedance ZCTRL[13:7] is used to select the pull-up output impedance ZCTRL[6:0] is used to select the pull-down output impedance ZCTRL field mapping for D3A/B/R I/Os is as follows: ZCTRL[27:20] is reserved and returns zeros on reads ZCTRL[19:15] is used to select the pull-up on-die termination impedance ZCTRL[14:10] is used to select the pull-down on-die termination impedance ZCTRL[9:5] is used to select the pull-up output impedance ZCTRL[4:0] is used to select the pull-down output impedance Note: The default value is 0x000014A for I/O type D3C/D3R and 0x0001839 for I/O type D3F.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zerr",
                    description: Some(
                        "Impedance Calibration Error: If set, indicates that there was an error during impedance calibration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zdone",
                    description: Some(
                        "Impedance Calibration Done: Indicates that impedance calibration has completed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr1",
            extends: None,
            description: Some(
                "Impedance Status Register 0-1 (ZQnSR0-1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zpd",
                    description: Some(
                        "Output impedance pull-down calibration status. Valid status encodings are: 00 = Completed with no errors 01 = Overflow error 10 = Underflow error 11 = Calibration in progress.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zpu",
                    description: Some(
                        "Output impedance pull-up calibration status. Similar status encodings as ZPD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opd",
                    description: Some(
                        "On-die termination (ODT) pull-down calibration status. Similar status encodings as ZPD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opu",
                    description: Some(
                        "On-die termination (ODT) pull-up calibration status. Similar status encodings as ZPD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
