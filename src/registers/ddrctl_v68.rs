use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ddrctl",
            extends: None,
            description: Some(
                "DDRCTL.",
            ),
            items: &[
                BlockItem {
                    name: "mstr",
                    description: Some(
                        "Description: Master Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Description: Operating Mode Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mrctrl0",
                    description: Some(
                        "Description: Mode Register Read/Write Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mrctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mrctrl1",
                    description: Some(
                        "Description: Mode Register Read/Write Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mrctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mrstat",
                    description: Some(
                        "Description: Mode Register Read/Write Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mrstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwrctl",
                    description: Some(
                        "Description: Low Power Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwrctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwrtmg",
                    description: Some(
                        "Description: Low Power Timing Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwrtmg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hwlpctl",
                    description: Some(
                        "Description: Hardware Low Power Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hwlpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfshctl0",
                    description: Some(
                        "Description: Refresh Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfshctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfshctl1",
                    description: Some(
                        "Description: Refresh Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfshctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfshctl3",
                    description: Some(
                        "Description: Refresh Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfshctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfshtmg",
                    description: Some(
                        "Description: Refresh Timing Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfshtmg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "eccuaddr0",
                    description: Some(
                        "Description: ECC Uncorrected Error Address Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eccuaddr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcparctl0",
                    description: Some(
                        "Description: CRC Parity Control Register0.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcparctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcparstat",
                    description: Some(
                        "Description: CRC Parity Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcparstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "init0",
                    description: Some(
                        "Description: SDRAM Initialization Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "init1",
                    description: Some(
                        "Description: SDRAM Initialization Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "init3",
                    description: Some(
                        "Description: SDRAM Initialization Register 3.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "init4",
                    description: Some(
                        "Description: SDRAM Initialization Register 4.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "init5",
                    description: Some(
                        "Description: SDRAM Initialization Register 5.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dimmctl",
                    description: Some(
                        "Description: DIMM Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dimmctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rankctl",
                    description: Some(
                        "Description: Rank Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rankctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg0",
                    description: Some(
                        "Description: SDRAM Timing Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg1",
                    description: Some(
                        "Description: SDRAM Timing Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg2",
                    description: Some(
                        "Description: SDRAM Timing Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg3",
                    description: Some(
                        "Description: SDRAM Timing Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg4",
                    description: Some(
                        "Description: SDRAM Timing Register 4.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg5",
                    description: Some(
                        "Description: SDRAM Timing Register 5.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dramtmg8",
                    description: Some(
                        "Description: SDRAM Timing Register 8.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dramtmg8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zqctl0",
                    description: Some(
                        "Description: ZQ Control Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zqctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zqctl1",
                    description: Some(
                        "Description: ZQ Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zqctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zqstat",
                    description: Some(
                        "Description: ZQ Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zqstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfitmg0",
                    description: Some(
                        "Description: DFI Timing Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfitmg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfitmg1",
                    description: Some(
                        "Description: DFI Timing Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfitmg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfilpcfg0",
                    description: Some(
                        "Description: DFI Low Power Configuration Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfilpcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfiupd0",
                    description: Some(
                        "Description: DFI Update Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfiupd0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfiupd1",
                    description: Some(
                        "Description: DFI Update Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfiupd1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfiupd2",
                    description: Some(
                        "Description: DFI Update Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfiupd2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfiupd3",
                    description: Some(
                        "Description: DFI Update Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfiupd3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfimisc",
                    description: Some(
                        "Description: DFI Miscellaneous Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfimisc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dfitmg2",
                    description: Some(
                        "Description: DFI Timing Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dfitmg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap0",
                    description: Some(
                        "Description: Address Map Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap1",
                    description: Some(
                        "Description: Address Map Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap2",
                    description: Some(
                        "Description: Address Map Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap3",
                    description: Some(
                        "Description: Address Map Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap4",
                    description: Some(
                        "Description: Address Map Register 4.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap5",
                    description: Some(
                        "Description: Address Map Register 5.",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addrmap6",
                    description: Some(
                        "Description: Address Map Register 6.",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addrmap6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odtcfg",
                    description: Some(
                        "Description: ODT Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odtmap",
                    description: Some(
                        "Description: ODT/Rank Map Register.",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odtmap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sched",
                    description: Some(
                        "Description: Scheduler Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sched",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sched1",
                    description: Some(
                        "Description: Scheduler Control Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sched1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "perfhpr1",
                    description: Some(
                        "Description: High Priority Read CAM Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x25c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Perfhpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "perflpr1",
                    description: Some(
                        "Description: Low Priority Read CAM Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Perflpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "perfwr1",
                    description: Some(
                        "Description: Write CAM Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x26c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Perfwr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "perfvpr1",
                    description: Some(
                        "Description: Variable Priority Read CAM Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x274,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Perfvpr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "perfvpw1",
                    description: Some(
                        "Description: Variable Priority Write CAM Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x278,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Perfvpw1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbg0",
                    description: Some(
                        "Description: Debug Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbg1",
                    description: Some(
                        "Description: Debug Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgcam",
                    description: Some(
                        "Description: CAM Debug Register.",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbgcam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgcmd",
                    description: Some(
                        "Description: Command Debug Register.",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbgcmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbgstat",
                    description: Some(
                        "Description: Status Debug Register.",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbgstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pstat",
                    description: Some(
                        "Description: Port Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x3fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pccfg",
                    description: Some(
                        "Description: Port Common Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pccfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 176,
                            },
                        ),
                    ),
                    byte_offset: 0x404,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pcfg",
                        },
                    ),
                },
                BlockItem {
                    name: "sar",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0xf04,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Sar",
                        },
                    ),
                },
                BlockItem {
                    name: "sbrctl",
                    description: Some(
                        "Description: Scrubber Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xf24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbrctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbrstat",
                    description: Some(
                        "Description: Scrubber Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xf28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbrstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbrwdata0",
                    description: Some(
                        "Description: Scrubber Write Data Pattern0.",
                    ),
                    array: None,
                    byte_offset: 0xf2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbrwdata0",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Id",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "maskch",
                    description: Some(
                        "Description: Port n Channel m Configuration ID Mask Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maskch",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "valuech",
                    description: Some(
                        "Description: Port n Channel m Configuration ID Value Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Valuech",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pcfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "r",
                    description: Some(
                        "Description: Port n Configuration Read Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "R",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "w",
                    description: Some(
                        "Description: Port n Configuration Write Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "W",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "c",
                    description: Some(
                        "Description: Port n Common Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "C",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "id",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Id",
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "Description: Port n Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qos0",
                    description: Some(
                        "Description: Port n Read QoS Configuration Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qos0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qos1",
                    description: Some(
                        "Description: Port n Read QoS Configuration Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qos1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wqos0",
                    description: Some(
                        "Description: Port n Write QoS Configuration Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wqos0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wqos1",
                    description: Some(
                        "Description: Port n Write QoS Configuration Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wqos1",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Sar",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "base",
                    description: Some(
                        "Description: SAR Base Address Register n.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Base",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "size",
                    description: Some(
                        "Description: SAR Size Register n.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Size",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addrmap0",
            extends: None,
            description: Some(
                "Description: Address Map Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_cs_bit0",
                    description: Some(
                        "Description: Selects the HIF address bit used as rank address bit 0. Valid Range: 0 to 27, and 31 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, rank address bit 0 is set to 0. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
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
            ],
        },
        FieldSet {
            name: "Addrmap1",
            extends: None,
            description: Some(
                "Description: Address Map Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_bank_b0",
                    description: Some(
                        "Description: Selects the HIF address bits used as bank address bit 0. Valid Range: 0 to 30 Internal Base: 2 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_bank_b1",
                    description: Some(
                        "Description: Selects the HIF address bits used as bank address bit 1. Valid Range: 0 to 30 Internal Base: 3 The selected HIF address bit for each of the bank address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addrmap_bank_b2",
                    description: Some(
                        "Description: Selects the HIF address bit used as bank address bit 2. Valid Range: 0 to 29 and 31 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 31, bank address bit 2 is set to 0. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Addrmap2",
            extends: None,
            description: Some(
                "Description: Address Map Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_col_b2",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 2 (if MEMC_BURST_LENGTH = 4) or 3 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 2 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b3",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 3 (if MEMC_BURST_LENGTH = 4) or 4 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7 Internal Base: 3 The selected HIF address bit is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b4",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 4 (if MEMC_BURST_LENGTH = 4) or 5 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 4 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b5",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 5 (if MEMC_BURST_LENGTH = 4) or 6 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 5 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Addrmap3",
            extends: None,
            description: Some(
                "Description: Address Map Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_col_b6",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 6 (if MEMC_BURST_LENGTH = 4) or 7 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 6 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b7",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 7 (if MEMC_BURST_LENGTH = 4) or 8 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 7 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b8",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 8 (if MEMC_BURST_LENGTH = 4) or 9 (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Quarter bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 8 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b9",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 9 (if MEMC_BURST_LENGTH = 4) or 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8) Half bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). (Column address bit 11 in LPDDR2/LPDDR3 mode) Quarter bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Valid Range: 0 to 7, and 15 Internal Base: 9 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Addrmap4",
            extends: None,
            description: Some(
                "Description: Address Map Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_col_b10",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 11 (10 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 8). Half bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8) Quarter bus width mode: UNUSED. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 10 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_col_b11",
                    description: Some(
                        "Description: Full bus width mode: Selects the HIF address bit used as column address bit 13 (11 in LPDDR2/LPDDR3 mode) (if MEMC_BURST_LENGTH = 4) or UNUSED (if MEMC_BURST_LENGTH = 8). Half bus width mode: Unused. To make it unused, this should be tied to 4'hF. Quarter bus width mode: Unused. To make it unused, this must be tied to 4'hF. Valid Range: 0 to 7, and 15 Internal Base: 11 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, this column address bit is set to 0. Note: Per JEDEC DDR2/3/mDDR specification, column address bit 10 is reserved for indicating auto-precharge, and hence no source address bit can be mapped to column address bit 10. In LPDDR2/LPDDR3, there is a dedicated bit for auto- precharge in the CA bus and hence column bit 10 is used. Value After Reset: 0x0 Exists: Always.",
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
            name: "Addrmap5",
            extends: None,
            description: Some(
                "Description: Address Map Register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_row_b0",
                    description: Some(
                        "Description: Selects the HIF address bits used as row address bit 0. Valid Range: 0 to 11 Internal Base: 6 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b1",
                    description: Some(
                        "Description: Selects the HIF address bits used as row address bit 1. Valid Range: 0 to 11 Internal Base: 7 The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b2_10",
                    description: Some(
                        "Description: Selects the HIF address bits used as row address bits 2 to 10. Valid Range: 0 to 11 Internal Base: 8 (for row address bit 2), 9 (for row address bit 3), 10 (for row address bit 4) etc increasing to 16 (for row address bit 10) The selected HIF address bit for each of the row address bits is determined by adding the internal base to the value of this field. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b11",
                    description: Some(
                        "Description: Selects the HIF address bit used as row address bit 11. Valid Range: 0 to 11, and 15 Internal Base: 17 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 11 is set to 0. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Addrmap6",
            extends: None,
            description: Some(
                "Description: Address Map Register 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrmap_row_b12",
                    description: Some(
                        "Description: Selects the HIF address bit used as row address bit 12. Valid Range: 0 to 11, and 15 Internal Base: 18 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 12 is set to 0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b13",
                    description: Some(
                        "Description: Selects the HIF address bit used as row address bit 13. Valid Range: 0 to 11, and 15 Internal Base: 19 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 13 is set to 0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b14",
                    description: Some(
                        "Description: Selects the HIF address bit used as row address bit 14. Valid Range: 0 to 11, and 15 Internal Base: 20 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 14 is set to 0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "addrmap_row_b15",
                    description: Some(
                        "Description: Selects the HIF address bit used as row address bit 15. Valid Range: 0 to 11, and 15 Internal Base: 21 The selected HIF address bit is determined by adding the internal base to the value of this field. If set to 15, row address bit 15 is set to 0. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Base",
            extends: None,
            description: Some(
                "Description: SAR Base Address Register n.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "base_addr",
                    description: Some(
                        "Description: Base address for address region n specified as awaddr[UMCTL2_A_ADDRW-1:x] and araddr[UMCTL2_A_ADDRW-1:x] where x is determined by the minimum block size parameter UMCTL2_SARMINSIZE: (x=log2(block size)). Value After Reset: 0x0 Exists: Always.",
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
            name: "C",
            extends: None,
            description: Some(
                "Description: Port n Common Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ahb_endianness",
                    description: Some(
                        "Description: If set to 0, enables support for little endian on the AHB port. If set to 1, enables support for big endian (BE- 32) on the AHB port. If set to 2, enables support for big endian (BE-A) on the AHB port. Value After Reset: 0x0 Exists: UMCTL2_A_AHB_n==1.",
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
            ],
        },
        FieldSet {
            name: "Crcparctl0",
            extends: None,
            description: Some(
                "Description: CRC Parity Control Register0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_alert_err_int_en",
                    description: Some(
                        "Description: Interrupt enable bit for DFI alert error. If this bit is set, any parity/CRC error detected on the dfi_alert_n input will result in an interrupt being set on CRCPARSTAT.dfi_alert_err_int. Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_alert_err_int_clr",
                    description: Some(
                        "Description: Interrupt clear bit for DFI alert error. If this bit is set, the alert error interrupt on CRCPARSTAT.dfi_alert_err_int will be cleared. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_alert_err_cnt_clr",
                    description: Some(
                        "Description: DFI alert error count clear. Clear bit for DFI alert error counter. Asserting this bit will clear the DFI alert error counter, CRCPARSTAT.dfi_alert_err_cnt. When the clear operation is complete, the uMCTL2 automatically clears this bit. Value After Reset: 0x0 Exists: Always.",
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
            name: "Crcparstat",
            extends: None,
            description: Some(
                "Description: CRC Parity Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_alert_err_cnt",
                    description: Some(
                        "Description: DFI alert error count. If a parity/CRC error is detected on dfi_alert_n, this counter be incremented. This is independent of the setting of CRCPARCTL0.dfi_alert_err_int_en. It will saturate at 0xFFFF, and can be cleared by asserting CRCPARCTL0.dfi_alert_err_cnt_clr. Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_alert_err_int",
                    description: Some(
                        "Description: DFI alert error interrupt. If a parity/CRC error is detected on dfi_alert_n, and the interrupt is enabled by CRCPARCTL0.dfi_alert_err_int_en, this interrupt bit will be set. It will remain set until cleared by CRCPARCTL0.dfi_alert_err_int_clr Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "Description: Port n Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port_en",
                    description: Some(
                        "Description: Enables port n. Value After Reset: \"UMCTL2_PORT_EN_RESET_VALUE\" Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dbg0",
            extends: None,
            description: Some(
                "Description: Debug Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dis_wc",
                    description: Some(
                        "Description: When 1, disable write combine. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
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
                    name: "dis_rd_bypass",
                    description: Some(
                        "Description: Only present in designs supporting read bypass. When 1, disable bypass path for high priority read page hits FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1.",
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
                    name: "dis_act_bypass",
                    description: Some(
                        "Description: Only present in designs supporting activate bypass. When 1, disable bypass path for high priority read activates FOR DEBUG ONLY. Value After Reset: 0x0 Exists: MEMC_BYPASS==1.",
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
                    name: "dis_collision_page_opt",
                    description: Some(
                        "Description: When this is set to '0', auto-precharge is disabled for the flushed command in a collision case. Collision cases are write followed by read to same address, read followed by write to same address, or write followed by write to same address with DBG0.dis_wc bit = 1 (where same address comparisons exclude the two address bits representing critical word). FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dbg1",
            extends: None,
            description: Some(
                "Description: Debug Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dis_dq",
                    description: Some(
                        "Description: When 1, uMCTL2 will not de-queue any transactions from the CAM. Bypass is also disabled. All transactions are queued in the CAM. No reads or writes are issued to SDRAM as long as this is asserted. This bit may be used to prevent reads or writes being issued by the uMCTL2, which makes it safe to modify certain register fields associated with reads and writes (see User Guide for details). After setting this bit, it is strongly recommended to poll DBGCAM.wr_data_pipeline_empty and DBGCAM.rd_data_pipeline_empty, before making changes to any registers which affect reads and writes. This will ensure that the relevant logic in the DDRC is idle. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always.",
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
                    name: "dis_hif",
                    description: Some(
                        "Description: When 1, uMCTL2 asserts the HIF command ih_co_stall. uMCTL2 will ignore the co_ih_rxcmd_valid and all other associated request signals. This bit is intended to be switched on-the-fly. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dbgcam",
            extends: None,
            description: Some(
                "Description: CAM Debug Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbg_hpr_q_depth",
                    description: Some(
                        "Description: High priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 6 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_lpr_q_depth",
                    description: Some(
                        "Description: Low priority read queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 14 is reserved FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_w_q_depth",
                    description: Some(
                        "Description: Write queue depth Note: The width of this field is dependent on log(MEMC_NO_OF_ENTRY+1). For example, if CAM depth = 32, then register width is 6 bits and bit 22 is reserved. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbg_stall",
                    description: Some(
                        "Description: Stall FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
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
                    name: "dbg_rd_q_empty",
                    description: Some(
                        "Description: When 1, all the Read command queues and Read data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
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
                    name: "dbg_wr_q_empty",
                    description: Some(
                        "Description: When 1, all the Write command queues and Write data buffers inside DDRC are empty. This register is to be used for debug purpose. An example use-case scenario: When Controller enters Self- Refresh using the Low-Power entry sequence, Controller is expected to have executed all the commands in its queues and the write and read data drained. Hence this register should be 1 at that time. FOR DEBUG ONLY Value After Reset: 0x0 Exists: Always.",
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
                    name: "rd_data_pipeline_empty",
                    description: Some(
                        "Description: This bit indicates that the read data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always.",
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
                    name: "wr_data_pipeline_empty",
                    description: Some(
                        "Description: This bit indicates that the write data pipeline on the DFI interface is empty. This register is intended to be polled after setting DBG1.dis_dq, to ensure that all remaining commands/data have completed. Value After Reset: 0x0 Exists: Always.",
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
            name: "Dbgcmd",
            extends: None,
            description: Some(
                "Description: Command Debug Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rank0_refresh",
                    description: Some(
                        "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 0. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: Always.",
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
                    name: "rank1_refresh",
                    description: Some(
                        "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a refresh to rank 1. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when RFSHCTL3.dis_auto_refresh=1. It is recommended NOT to set this register bit if in Init or Deep power-down operating modes or Maximum Power Saving Mode. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
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
                    name: "zq_calib_short",
                    description: Some(
                        "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a ZQCS (ZQ calibration short) command to the SDRAM. When this request is stored in uMCTL2, the bit is automatically cleared. This operation can be performed only when ZQCTL0.dis_auto_zq=1. It is recommended NOT to set this register bit if in Init operating mode. This register bit is ignored when in Self-Refresh and Deep power-down operating modes. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1.",
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
                    name: "ctrlupd",
                    description: Some(
                        "Description: Setting this register bit to 1 indicates to the uMCTL2 to issue a dfi_ctrlupd_req to the PHY. When this request is stored in uMCTL2, the bit is automatically cleared. This operation must only be performed when DFIUPD0.dis_auto_ctrlupd=1. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dbgstat",
            extends: None,
            description: Some(
                "Description: Status Debug Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rank0_refresh_busy",
                    description: Some(
                        "Description: SoC core may initiate a rank0_refresh operation (refresh operation to rank 0) only if this signal is low. This signal goes high in the clock after DBGCMD.rank0_refresh is set to one. It goes low when the rank0_refresh operation is stored in uMCTL2. It is recommended not to perform rank0_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank0_refresh operation 1 - Indicates that rank0_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: Always.",
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
                    name: "rank1_refresh_busy",
                    description: Some(
                        "Description: SoC core may initiate a rank1_refresh operation (refresh operation to rank 1) only if this signal is low. This signal goes high in the clock after DBGCMD.rank1_refresh is set to one. It goes low when the rank1_refresh operation is stored in uMCTL2. It is recommended not to perform rank1_refresh operations when this signal is high. 0 - Indicates that the SoC core can initiate a rank1_refresh operation 1 - Indicates that rank1_refresh operation has not been stored yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
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
                    name: "zq_calib_short_busy",
                    description: Some(
                        "Description: SoC core may initiate a ZQCS (ZQ calibration short) operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQCS request. It goes low when the ZQCS operation is initiated in uMCTL2. It is recommended not to perform ZQCS operations when this signal is high. 0 - Indicates that the SoC core can initiate a ZQCS operation 1 - Indicates that ZQCS operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4_OR_LPDDR2==1.",
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
                    name: "ctrlupd_busy",
                    description: Some(
                        "Description: SoC core may initiate a ctrlupd operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ctrlupd request. It goes low when the ctrlupd operation is initiated in uMCTL2. It is recommended not to perform ctrlupd operations when this signal is high. 0 - Indicates that the SoC core can initiate a ctrlupd operation 1 - Indicates that ctrlupd operation has not been initiated yet in uMCTL2 Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dfilpcfg0",
            extends: None,
            description: Some(
                "Description: DFI Low Power Configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_lp_en_pd",
                    description: Some(
                        "Description: Enables DFI Low Power interface handshaking during Power Down Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_lp_wakeup_pd",
                    description: Some(
                        "Description: Value to drive on dfi_lp_wakeup signal when Power Down mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_lp_en_sr",
                    description: Some(
                        "Description: Enables DFI Low Power interface handshaking during Self Refresh Entry/Exit. 0 - Disabled 1 - Enabled Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_lp_wakeup_sr",
                    description: Some(
                        "Description: Value to drive on dfi_lp_wakeup signal when Self Refresh mode is entered. Determines the DFI's tlp_wakeup time: 0x0 - 16 cycles 0x1 - 32 cycles 0x2 - 64 cycles 0x3 - 128 cycles 0x4 - 256 cycles 0x5 - 512 cycles 0x6 - 1024 cycles 0x7 - 2048 cycles 0x8 - 4096 cycles 0x9 - 8192 cycles 0xA - 16384 cycles 0xB - 32768 cycles 0xC - 65536 cycles 0xD - 131072 cycles 0xE - 262144 cycles 0xF - Unlimited Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_tlp_resp",
                    description: Some(
                        "Description: Setting for DFI's tlp_resp time. Same value is used for both Power Down, Self Refresh, Deep Power Down and Maximum Power Saving modes. DFI 2.1 specification onwards, recommends using a fixed value of 7 always. Value After Reset: 0x7 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dfimisc",
            extends: None,
            description: Some(
                "Description: DFI Miscellaneous Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_init_complete_en",
                    description: Some(
                        "Description: PHY initialization complete enable signal. When asserted the dfi_init_complete signal can be used to trigger SDRAM initialisation Value After Reset: 0x1 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dfitmg0",
            extends: None,
            description: Some(
                "Description: DFI Timing Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_tphy_wrlat",
                    description: Some(
                        "Description: Write latency Number of clocks from the write command to write data enable (dfi_wrdata_en). This corresponds to the DFI timing parameter tphy_wrlat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value.Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of tphy_wrlat. This is to compensate for the extra cycle of latency through the RDIMM. Value After Reset: 0x2 Exists: Always.",
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
                    name: "dfi_tphy_wrdata",
                    description: Some(
                        "Description: Specifies the number of clock cycles between when dfi_wrdata_en is asserted to when the associated write data is driven on the dfi_wrdata signal. This corresponds to the DFI timing parameter tphy_wrdata. Refer to PHY specification for correct value. Note, max supported value is 8. Unit: Clocks Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfi_wrdata_use_sdr",
                    description: Some(
                        "Description: Defines whether dfi_wrdata_en/dfi_wrdata/dfi_wrdata_mask is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_tphy_wrlat is in terms of SDR or HDR clock cycles Selects whether value in DFITMG0.dfi_tphy_wrdata is in terms of SDR or HDR clock cycles 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2.",
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
                    name: "dfi_t_rddata_en",
                    description: Some(
                        "Description: Time from the assertion of a read command on the DFI interface to the assertion of the dfi_rddata_en signal. Refer to PHY specification for correct value. This corresponds to the DFI parameter trddata_en. Note that, depending on the PHY, if using RDIMM, it may be necessary to use the value (CL + 1) in the calculation of trddata_en. This is to compensate for the extra cycle of latency through the RDIMM. Unit: Clocks Value After Reset: 0x2 Exists: Always.",
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
                    name: "dfi_rddata_use_sdr",
                    description: Some(
                        "Description: Defines whether dfi_rddata_en/dfi_rddata/dfi_rddata_valid is generated using HDR or SDR values Selects whether value in DFITMG0.dfi_t_rddata_en is in terms of SDR or HDR clock cycles: 0 in terms of HDR clock cycles 1 in terms of SDR clock cycles Refer to PHY specification for correct value. Value After Reset: 0x0 Exists: MEMC_FREQ_RATIO==2.",
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
                    name: "dfi_t_ctrl_delay",
                    description: Some(
                        "Description: Specifies the number of DFI clock cycles after an assertion or de-assertion of the DFI control signals that the control signals at the PHY-DRAM interface reflect the assertion or de-assertion. If the DFI clock and the memory clock are not phase-aligned, this timing parameter should be rounded up to the next integer value. Note that if using RDIMM, depending on the PHY, it may be necessary to increment this parameter by 1. This is to compensate for the extra cycle of latency through the RDIMM Value After Reset: 0x7 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dfitmg1",
            extends: None,
            description: Some(
                "Description: DFI Timing Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_t_dram_clk_enable",
                    description: Some(
                        "Description: Specifies the number of DFI clock cycles from the de-assertion of the dfi_dram_clk_disable signal on the DFI until the first valid rising edge of the clock to the DRAM memory devices, at the PHY-DRAM boundary. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always.",
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
                    name: "dfi_t_dram_clk_disable",
                    description: Some(
                        "Description: Specifies the number of DFI clock cycles from the assertion of the dfi_dram_clk_disable signal on the DFI until the clock to the DRAM memory devices, at the PHY- DRAM boundary, maintains a low value. If the DFI clock and the memory clock are not phase aligned, this timing parameter should be rounded up to the next integer value. Value After Reset: 0x4 Exists: Always.",
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
                    name: "dfi_t_wrdata_delay",
                    description: Some(
                        "Description: Specifies the number of DFI clocks between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus. This corresponds to the DFI timing parameter twrdata_delay. Refer to PHY specification for correct value. For DFI 3.0 PHY, set to twrdata_delay, a new timing parameter introduced in DFI 3.0. For DFI 2.1 PHY, set to tphy_wrdata + (delay of DFI write data to the DRAM). Value to be programmed is in terms of DFI clocks, not PHY clocks. In FREQ_RATIO=2, divide PHY's value by 2 and round up to next integer. If using DFITMG0.dfi_wrdata_use_sdr=1, add 1 to the value. Unit: Clocks Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dfitmg2",
            extends: None,
            description: Some(
                "Description: DFI Timing Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_tphy_wrcslat",
                    description: Some(
                        "Description: Number of clocks between when a write command is sent on the DFI control interface and when the associated dfi_wrdata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_wrcslat. The minimum supported value is as follows: 0 for configurations with MEMC_WL0 = 1 1 for configurations with MEMC_WL0 = 0 Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always.",
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
                    name: "dfi_tphy_rdcslat",
                    description: Some(
                        "Description: Number of clocks between when a read command is sent on the DFI control interface and when the associated dfi_rddata_cs_n signal is asserted. This corresponds to the DFI timing parameter tphy_rdcslat. Refer to PHY specification for correct value. Value After Reset: 0x2 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dfiupd0",
            extends: None,
            description: Some(
                "Description: DFI Update Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_t_ctrlup_min",
                    description: Some(
                        "Description: Specifies the minimum number of clock cycles that the dfi_ctrlupd_req signal must be asserted. The uMCTL2 expects the PHY to respond within this time. If the PHY does not respond, the uMCTL2 will de-assert dfi_ctrlupd_req after dfi_t_ctrlup_min + 2 cycles. Lowest value to assign to this variable is 0x3. Unit: Clocks Value After Reset: 0x3 Exists: Always.",
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
                    name: "dfi_t_ctrlup_max",
                    description: Some(
                        "Description: Specifies the maximum number of clock cycles that the dfi_ctrlupd_req signal can assert. Lowest value to assign to this variable is 0x40. Unit: Clocks Value After Reset: 0x40 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dis_auto_ctrlupd",
                    description: Some(
                        "Description: When '1', disable the automatic dfi_ctrlupd_req generation by the uMCTL2. The core must issue the dfi_ctrlupd_req signal using register reg_ddrc_ctrlupd. This register field is changeable on the fly. When '0', uMCTL2 issues dfi_ctrlupd_req periodically. Value After Reset: 0x0 Exists: Always.",
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
            name: "Dfiupd1",
            extends: None,
            description: Some(
                "Description: DFI Update Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_t_ctrlupd_interval_max_x1024",
                    description: Some(
                        "Description: This is the maximum amount of time between uMCTL2 initiated DFI update requests. This timer resets with each update request; when the timer expires dfi_ctrlupd_req is sent and traffic is blocked until the dfi_ctrlupd_ackx is received. PHY can use this idle time to recalibrate the delay lines to the DLLs. The DFI controller update is also used to reset PHY FIFO pointers in case of data capture errors. Updates are required to maintain calibration over PVT, but frequent updates may impact performance. Note: Value programmed for DFIUPD1.dfi_t_ctrlupd_interval_max_x1024 must be greater than DFIUPD1.dfi_t_ctrlupd_interval_min_x1024. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always.",
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
                    name: "dfi_t_ctrlupd_interval_min_x1024",
                    description: Some(
                        "Description: This is the minimum amount of time between uMCTL2 initiated DFI update requests (which is executed whenever the uMCTL2 is idle). Set this number higher to reduce the frequency of update requests, which can have a small impact on the latency of the first read request when the uMCTL2 is idle. Unit: 1024 clocks Value After Reset: 0x0 Exists: Always.",
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
            name: "Dfiupd2",
            extends: None,
            description: Some(
                "Description: DFI Update Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_phyupd_type0",
                    description: Some(
                        "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b00. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always.",
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
                    name: "dfi_phyupd_type1",
                    description: Some(
                        "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b01. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfi_phyupd_en",
                    description: Some(
                        "Description: Enables the support for acknowledging PHY- initiated updates: 0 - Disabled 1 - Enabled Value After Reset: 0x1 Exists: Always.",
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
            name: "Dfiupd3",
            extends: None,
            description: Some(
                "Description: DFI Update Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dfi_phyupd_type2",
                    description: Some(
                        "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b10. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always.",
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
                    name: "dfi_phyupd_type3",
                    description: Some(
                        "Description: Specifies the maximum number of DFI clock cycles that the dfi_phyupd_req signal may remain asserted after the assertion of the dfi_phyupd_ack signal for dfi_phyupd_type = 2'b11. The dfi_phyupd_req signal may de-assert at any cycle after the assertion of the dfi_phyupd_ack signal. Value After Reset: 0x10 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dimmctl",
            extends: None,
            description: Some(
                "Description: DIMM Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dimm_stagger_cs_en",
                    description: Some(
                        "Description: Staggering enable for multi-rank accesses (for multi-rank UDIMM and RDIMM implementations only). This is not supported for DDR4, mDDR, LPDDR2 or LPDDR3 SDRAMs. 1 - Stagger accesses to even and odd ranks 0 - Do not stagger accesses Value After Reset: 0x0 Exists: Always.",
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
                    name: "dimm_addr_mirr_en",
                    description: Some(
                        "Description: Address Mirroring Enable (for multi-rank UDIMM implementations and multi-rank DDR4 RDIMM implementations). Some UDIMMs and DDR4 RDIMMs implement address mirroring for odd ranks, which means that the following address, bank address and bank group bits are swapped: (A3, A4), (A5, A6), (A7, A8), (BA0, BA1) and also (A11, A13), (BG0, BG1) for the DDR4. Setting this bit ensures that, for mode register accesses during the automatic initialization routine, these bits are swapped within the uMCTL2 to compensate for this UDIMM/RDIMM swapping. In addition to the automatic initialization routine, in case of DDR4 UDIMM/RDIMM, they are swapped during the automatic MRS access to enable/disable of a particular DDR4 feature. Note: This has no effect on the address of any other memory accesses, or of software-driven mode register accesses. This is not supported for mDDR, LPDDR2 or LPDDR3 SDRAMs. Note: In case of x16 DDR4 DIMMs, BG1 output of MRS for the odd ranks is same as BG0 because BG1 is invalid, hence dimm_dis_bg_mirroring register must be set to 1. 1 - For odd ranks, implement address mirroring for MRS commands to during initialization and for any automatic DDR4 MRS commands (to be used if UDIMM/RDIMM implements address mirroring) 0 - Do not implement address mirroring Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dramtmg0",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_ras_min",
                    description: Some(
                        "Description: tRAS(min): Minimum time between activate and precharge to the same bank. For configurations with MEMC_FREQ_RATIO=2, 1T mode, program this to tRAS(min)/2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, program this to (tRAS(min)/2 + 1). No rounding up of the division operation. Unit: Clocks Value After Reset: 0xf Exists: Always.",
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
                    name: "t_ras_max",
                    description: Some(
                        "Description: tRAS(max): Maximum time between activate and precharge to same bank. This is the maximum time that a page can be kept open Minimum value of this register is 1. Zero is invalid. For configurations with MEMC_FREQ_RATIO=2, program this to (tRAS(max)-1)/2. No rounding up. Unit: Multiples of 1024 clocks. Value After Reset: 0x1b Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_faw",
                    description: Some(
                        "Description: tFAW Valid only when 8 or more banks(or banks x bank groups) are present. In 8-bank design, at most 4 banks must be activated in a rolling window of tFAW cycles. For configurations with MEMC_FREQ_RATIO=2, program this to (tFAW/2) and round up to next integer value. In a 4-bank design, set this register to 0x1 independent of the MEMC_FREQ_RATIO configuration. Unit: Clocks Value After Reset: 0x10 Exists: Always.",
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
                    name: "wr2pre",
                    description: Some(
                        "Description: Minimum time between write and precharge to same bank. Unit: Clocks Specifications: WL + BL/2 + tWR = approximately 8 cycles + 15 ns = 14 clocks @400MHz and less for lower frequencies where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM. BST (burst terminate) is not supported at present. tWR = Write recovery time. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 for this parameter. For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up. Value After Reset: 0xf Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dramtmg1",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rc",
                    description: Some(
                        "Description: tRC: Minimum time between activates to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRC/2) and round up to next integer value. Unit: Clocks. Value After Reset: 0x14 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rd2pre",
                    description: Some(
                        "Description: tRTP: Minimum time from read to precharge of same bank. DDR2: tAL + BL/2 + max(tRTP, 2) - 2 DDR3: tAL + max (tRTP, 4) DDR4: Max of following two equations: tAL + max (tRTP, 4) or, RL + BL/2 - tRP. mDDR: BL/2 LPDDR2: Depends on if it's LPDDR2-S2 or LPDDR2-S4: LPDDR2-S2: BL/2 + tRTP - 1. LPDDR2-S4: BL/2 + max(tRTP,2) - 2. LPDDR3: BL/2 + max(tRTP,4) - 4 For configurations with MEMC_FREQ_RATIO=2, 1T mode, divide the above value by 2. No rounding up. For configurations with MEMC_FREQ_RATIO=2, 2T mode, divide the above value by 2 and add 1. No rounding up of division operation. Unit: Clocks. Value After Reset: 0x4 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_xp",
                    description: Some(
                        "Description: tXP: Minimum time after power-down exit to any operation. For DDR3, this should be programmed to tXPDLL if slow powerdown exit is selected in MR0[12]. If C/A parity for DDR4 is used, set to (tXP+PL) instead. For configurations with MEMC_FREQ_RATIO=2, program this to (tXP/2) and round it up to the next integer value. Units: Clocks Value After Reset: 0x8 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dramtmg2",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr2rd",
                    description: Some(
                        "Description: DDR4: WL + BL/2 + tWTR_L Others: WL + BL/2 + tWTR In DDR4, minimum time from write command to read command for same bank group. In others, minimum time from write command to read command. Includes time for bus turnaround, recovery times, and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM tWTR_L = internal write to read command delay for same bank group. This comes directly from the SDRAM specification. tWTR = internal write to read command delay. This comes directly from the SDRAM specification. Add one extra cycle for LPDDR2/LPDDR3 operation. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0xd Exists: Always.",
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
                    name: "rd2wr",
                    description: Some(
                        "Description: DDR2/3/mDDR: RL + BL/2 + 2 - WL DDR4: RL + BL/2 + 1 + WR_PREAMBLE - WL LPDDR2/LPDDR3: RL + BL/2 + RU(tDQSCKmax/tCK) + 1 - WL. Minimum time from read command to write command. Include time for bus turnaround and all per-bank, per-rank, and global constraints. Unit: Clocks. Where: WL = write latency BL = burst length. This must match the value programmed in the BL bit of the mode register to the SDRAM RL = read latency = CAS latency WR_PREAMBLE = write preamble. This is unique to DDR4. For configurations with MEMC_FREQ_RATIO=2, divide the value calculated using the above equation by 2, and round it up to next integer. Value After Reset: 0x6 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dramtmg3",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_mod",
                    description: Some(
                        "Description: tMOD: Present if MEMC_DDR3_OR_4 = 1. Cycles between load mode command and following non-load mode command. This is required to be programmed even when a design that supports DDR3/4 is running in DDR2 mode. If C/A parity for DDR4 is used, set to tMOD_PAR(tMOD+PL) instead Set to tMOD if MEMC_FREQ_RATIO=1, or tMOD/2 (rounded up to next integer) if MEMC_FREQ_RATIO=2. Note that if using RDIMM, depending on the PHY, it may be necessary to use a value of tMOD + 1 or (tMOD + 1)/2 to compensate for the extra cycle of latency applied to mode register writes by the RDIMM chip Value After Reset: \"(MEMC_DDR3_EN==1 || MEMC_DDR4_EN==1 ) ? 0xc : 0x0\" Exists: MEMC_DDR3==1 || MEMC_DDR4==1.",
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
                    name: "t_mrd",
                    description: Some(
                        "Description: tMRD: Cycles between load mode commands. If MEMC_DDR3_OR_4 = 0, this parameter is also used to define the cycles between load mode command and following non-load mode command. For configurations with MEMC_FREQ_RATIO=2, program this to (tMRD/2) and round it up to the next integer value. If C/A parity for DDR4 is used, set to tMRD_PAR(tMOD+PL) instead Value After Reset: 0x4 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dramtmg4",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rp",
                    description: Some(
                        "Description: tRP: Minimum time from precharge to activate of same bank. For configurations with MEMC_FREQ_RATIO=2, program this to (tRP/2 + 1). No round up of the fraction. Unit: Clocks. Value After Reset: 0x5 Exists: Always.",
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
                    name: "t_rrd",
                    description: Some(
                        "Description: DDR4: tRRD_L: Minimum time between activates from bank \"a\" to bank \"b\" for same bank group. Others: tRRD: Minimum time between activates from bank \"a\" to bank \"b\" For configurations with MEMC_FREQ_RATIO=2, program this to (tRRD_L/2 or tRRD/2) and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x4 Exists: Always.",
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
                    name: "t_ccd",
                    description: Some(
                        "Description: DDR4: tCCD_L: This is the minimum time between two reads or two writes for same bank group. Others: tCCD: This is the minimum time between two reads or two writes. For configurations with MEMC_FREQ_RATIO=2, program this to (tCCD_L/2 or tCCD/2) and round it up to the next integer value. Unit: clocks. Value After Reset: 0x4 Exists: Always.",
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
                    name: "t_rcd",
                    description: Some(
                        "Description: tRCD - tAL: Minimum time from activate to read or write command to same bank. For configurations with MEMC_FREQ_RATIO=2, program this to ((tRCD - tAL)/2) and round it up to the next integer value. Minimum value allowed for this register is 1, which implies minimum (tRCD - tAL) value to be 2 in configurations with MEMC_FREQ_RATIO=2. Unit: Clocks. Value After Reset: 0x5 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dramtmg5",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_cke",
                    description: Some(
                        "Description: Minimum number of cycles of CKE HIGH/LOW during power-down and self refresh. LPDDR2/LPDDR3 mode: Set this to the larger of tCKE or tCKESR Non-LPDDR2/non-LPDDR3 designs: Set this to tCKE value. For configurations with MEMC_FREQ_RATIO=2, program this to (value described above)/2 and round it up to the next integer value. Unit: Clocks. Value After Reset: 0x3 Exists: Always.",
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
                    name: "t_ckesr",
                    description: Some(
                        "Description: Minimum CKE low width for Self refresh entry to exit timing im memory clock cycles. Recommended settings: mDDR: tRFC LPDDR2: tCKESR LPDDR3: tCKESR DDR2: tCKE DDR3: tCKE + 1 DDR4: tCKE + 1 For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x4 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_cksre",
                    description: Some(
                        "Description: This is the time after Self Refresh Down Entry that CK is maintained as a valid clock. Specifies the clock disable delay after SRE. Recommended settings: mDDR: 0 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: max (10 ns, 5 tCK) DDR4: max (10 ns, 5 tCK) For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always.",
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
                    name: "t_cksrx",
                    description: Some(
                        "Description: This is the time before Self Refresh Exit that CK is maintained as a valid clock before issuing SRX. Specifies the clock stable time before SRX. Recommended settings: mDDR: 1 LPDDR2: 2 LPDDR3: 2 DDR2: 1 DDR3: tCKSRX DDR4: tCKSRX For configurations with MEMC_FREQ_RATIO=2, program this to recommended value divided by two and round it up to next integer. Value After Reset: 0x5 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Dramtmg8",
            extends: None,
            description: Some(
                "Description: SDRAM Timing Register 8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_xs_x32",
                    description: Some(
                        "Description: tXS: Exit Self Refresh to commands not requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x5 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_xs_dll_x32",
                    description: Some(
                        "Description: tXSDLL: Exit Self Refresh to commands requiring a locked DLL. For configurations with MEMC_FREQ_RATIO=2, program this to the above value divided by 2 and round up to next integer value. Unit: Multiples of 32 clocks. Note: In LPDDR2/LPDDR3/Mobile DDR mode, t_xs_x32 and t_xs_dll_x32 must be set the same values derived from tXSR. Value After Reset: 0x44 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Eccuaddr0",
            extends: None,
            description: Some(
                "Description: ECC Uncorrected Error Address Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ecc_uncorr_row",
                    description: Some(
                        "Description: Page/row number of a read resulting in an uncorrected ECC error. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always.",
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
                    name: "ecc_uncorr_rank",
                    description: Some(
                        "Description: Rank number of a read resulting in an uncorrected ECC error Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hwlpctl",
            extends: None,
            description: Some(
                "Description: Hardware Low Power Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hw_lp_en",
                    description: Some(
                        "Description: Enable for Hardware Low Power Interface. Value After Reset: 0x1 Exists: Always.",
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
                    name: "hw_lp_exit_idle_en",
                    description: Some(
                        "Description: When this bit is programmed to 1 the cactive_in_ddrc pin of the DDRC can be used to exit from the automatic clock stop, automatic power down or automatic self-refresh modes. Note, it will not cause exit of Self-Refresh that was caused by Hardware Low Power Interface and/or Software (PWRCTL.selfref_sw). Value After Reset: 0x1 Exists: Always.",
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
                    name: "hw_lp_idle_x32",
                    description: Some(
                        "Description: Hardware idle period. The cactive_ddrc output is driven low if the system is idle for hw_lp_idle * 32 cycles if not in INIT or DPD/MPSM operating_mode. The hardware idle function is disabled when hw_lp_idle_x32=0. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Init0",
            extends: None,
            description: Some(
                "Description: SDRAM Initialization Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pre_cke_x1024",
                    description: Some(
                        "Description: Cycles to wait after reset before driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clock cycles. DDR2 specifications typically require this to be programmed for a delay of >= 200 us. LPDDR2/LPDDR3: tINIT1 of 100 ns (min) For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x4e Exists: Always.",
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
                    name: "post_cke_x1024",
                    description: Some(
                        "Description: Cycles to wait after driving CKE high to start the SDRAM initialization sequence. Unit: 1024 clocks. DDR2 typically requires a 400 ns delay, requiring this value to be programmed to 2 at all clock speeds. LPDDR2/LPDDR3 typically requires this to be programmed for a delay of 200 us. For configurations with MEMC_FREQ_RATIO=2, program this to JEDEC spec value divided by 2, and round it up to next integer value. Value After Reset: 0x2 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "skip_dram_init",
                    description: Some(
                        "Description: If lower bit is enabled the SDRAM initialization routine is skipped. The upper bit decides what state the controller starts up in when reset is removed 00 - SDRAM Initialization routine is run after power-up 01 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Normal Mode 11 - SDRAM Initialization routine is skipped after power- up. Controller starts up in Self-refresh Mode 10 - SDRAM Initialization routine is run after power-up. Value After Reset: 0x0 Exists: Always.",
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
            name: "Init1",
            extends: None,
            description: Some(
                "Description: SDRAM Initialization Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pre_ocd_x32",
                    description: Some(
                        "Description: Wait period before driving the OCD complete command to SDRAM. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always.",
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
                    name: "final_wait_x32",
                    description: Some(
                        "Description: Cycles to wait after completing the SDRAM initialization sequence before starting the dynamic scheduler. Unit: Counts of a global timer that pulses every 32 clock cycles. There is no known specific requirement for this; it may be set to zero. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dram_rstn_x1024",
                    description: Some(
                        "Description: Number of cycles to assert SDRAM reset signal during init sequence. This is only present for designs supporting DDR3/DDR4 devices. For use with a Synopsys DDR PHY, this should be set to a minimum of 1 Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1.",
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
            name: "Init3",
            extends: None,
            description: Some(
                "Description: SDRAM Initialization Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "emr",
                    description: Some(
                        "Description: DDR2: Value to write to EMR register. Bits 9:7 are for OCD and the setting in this register is ignored. The uMCTL2 sets those bits appropriately. DDR3/DDR4: Value to write to MR1 register Set bit 7 to 0. If PHY-evaluation mode training is enabled, this bit is set appropriately by the uMCTL2 during write leveling. mDDR: Value to write to EMR register. LPDDR2/LPDDR3 - Value to write to MR2 register Value After Reset: 0x510 Exists: Always.",
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
                    name: "mr",
                    description: Some(
                        "Description: DDR2: Value to write to MR register. Bit 8 is for DLL and the setting here is ignored. The uMCTL2 sets this bit appropriately. DDR3/DDR4: Value loaded into MR0 register. mDDR: Value to write to MR register. LPDDR2/LPDDR3 - Value to write to MR1 register Value After Reset: 0x0 Exists: Always.",
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
            name: "Init4",
            extends: None,
            description: Some(
                "Description: SDRAM Initialization Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "emr3",
                    description: Some(
                        "Description: DDR2: Value to write to EMR3 register. DDR3/DDR4: Value to write to MR3 register mDDR/LPDDR2/LPDDR3: Unused Value After Reset: 0x0 Exists: Always.",
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
                    name: "emr2",
                    description: Some(
                        "Description: DDR2: Value to write to EMR2 register. DDR3/DDR4: Value to write to MR2 register LPDDR2/LPDDR3: Value to write to MR3 register mDDR: Unused Value After Reset: 0x0 Exists: Always.",
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
            name: "Init5",
            extends: None,
            description: Some(
                "Description: SDRAM Initialization Register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dev_zqinit_x32",
                    description: Some(
                        "Description: ZQ initial calibration, tZQINIT. Present only in designs configured to support DDR3 or DDR4 or LPDDR2/LPDDR3. Unit: 32 clock cycles. DDR3 typically requires 512 clocks. DDR4 requires 1024 clocks. LPDDR2/LPDDR3 requires 1 us. Value After Reset: 0x10 Exists: MEMC_DDR3==1 || MEMC_DDR4 == 1 || MEMC_LPDDR2==1.",
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
            name: "Maskch",
            extends: None,
            description: Some(
                "Description: Port n Channel m Configuration ID Mask Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id_mask",
                    description: Some(
                        "Description: Determines the mask used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always.",
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
            name: "Mrctrl0",
            extends: None,
            description: Some(
                "Description: Mode Register Read/Write Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mr_rank",
                    description: Some(
                        "Description: Controls which rank is accessed by MRCTRL0.mr_wr. Normally, it is desired to access all ranks, so all bits should be set to 1. However, for multi-rank UDIMMs/RDIMMs which implement address mirroring, it may be necessary to access ranks individually. Examples (assume uMCTL2 is configured for 4 ranks): 0x1 - select rank 0 only 0x2 - select rank 1 only 0x5 - select ranks 0 and 2 0xA - select ranks 1 and 3 0xF - select ranks 0, 1, 2 and 3 Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: Always.",
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
                    name: "mr_addr",
                    description: Some(
                        "Description: Address of the mode register that is to be written to. 0000 - MR0 0001 - MR1 0010 - MR2 0011 - MR3 0100 - MR4 0101 - MR5 0110 - MR6 0111 - MR7 Don't Care for LPDDR2/LPDDR3 (see MRCTRL1.mr_data for mode register addressing in LPDDR2/LPDDR3) This signal is also used for writing to control words of RDIMMs. In that case, it corresponds to the bank address bits sent to the RDIMM In case of DDR4, the bit[3:2] corresponds to the bank group bits. Therefore, the bit[3] as well as the bit[2:0] must be set to an appropriate value which is considered both the Address Mirroring of UDIMMs/RDIMMs and the Output Inversion of RDIMMs. Value After Reset: 0x0 Exists: Always.",
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
                    name: "mr_wr",
                    description: Some(
                        "Description: Setting this register bit to 1 triggers a mode register read or write operation. When the MR operation is complete, the uMCTL2 automatically clears this bit. The other register fields of this register must be written in a separate APB transaction, before setting this mr_wr bit. It is recommended NOT to set this signal if in Init, Deep power- down or MPSM operating modes. Value After Reset: 0x0 Exists: Always.",
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
            name: "Mrctrl1",
            extends: None,
            description: Some(
                "Description: Mode Register Read/Write Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mr_data",
                    description: Some(
                        "Description: Mode register write data for all non- LPDDR2/non-LPDDR3 modes. For LPDDR2/LPDDR3, MRCTRL1[15:0] are interpreted as [15:8] MR Address and [7:0] MR data for writes, don't care for reads. This is 18-bits wide in configurations with DDR4 support and 16-bits in all other configurations. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Mrstat",
            extends: None,
            description: Some(
                "Description: Mode Register Read/Write Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mr_wr_busy",
                    description: Some(
                        "Description: The SoC core may initiate a MR write operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the MRW/MRR request. It goes low when the MRW/MRR command is issued to the SDRAM. It is recommended not to perform MRW/MRR commands when 'MRSTAT.mr_wr_busy' is high. 0 - Indicates that the SoC core can initiate a mode register write operation 1 - Indicates that mode register write operation is in progress Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Mstr",
            extends: None,
            description: Some(
                "Description: Master Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ddr3",
                    description: Some(
                        "Description: Select DDR3 SDRAM 1 - DDR3 SDRAM device in use 0 - non-DDR3 SDRAM device in use Only present in designs that support DDR3. Value After Reset: \"(MEMC_DDR3_EN==1) ? 0x1 : 0x0\" Exists: MEMC_DDR3==1.",
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
                    name: "burstchop",
                    description: Some(
                        "Description: When set, enable burst-chop in DDR3/DDR4. This is only supported in full bus width mode (MSTR.data_bus_width = 00). If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), burst chop is not supported, and this bit must be set to '0' Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1.",
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
                    name: "en_2t_timing_mode",
                    description: Some(
                        "Description: If 1, then uMCTL2 uses 2T timing. Otherwise, uses 1T timing. In 2T timing, all command signals (except chip select) are held for 2 clocks on the SDRAM bus. Chip select is asserted on the second cycle of the command Note: 2T timing is not supported in LPDDR2/LPDDR3 mode Note: 2T timing is not supported if the configuration parameter MEMC_CMD_RTN2IDLE is set Note: 2T timing is not supported in DDR4 geardown mode. Value After Reset: 0x0 Exists: MEMC_CMD_RTN2IDLE==0.",
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
                    name: "data_bus_width",
                    description: Some(
                        "Description: Selects proportion of DQ bus width that is used by the SDRAM 00 - Full DQ bus width to SDRAM 01 - Half DQ bus width to SDRAM 10 - Quarter DQ bus width to SDRAM 11 - Reserved. Note that half bus width mode is only supported when the SDRAM bus width is a multiple of 16, and quarter bus width mode is only supported when the SDRAM bus width is a multiple of 32 and the configuration parameter MEMC_QBUS_SUPPORT is set. Bus width refers to DQ bus width (excluding any ECC width). Value After Reset: 0x0 Exists: Always.",
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
                    name: "dll_off_mode",
                    description: Some(
                        "Description: Set to 1 when uMCTL2 and DRAM has to be put in DLL-off mode for low frequency operation. Set to 0 to put uMCTL2 and DRAM in DLL-on mode for normal frequency operation. Value After Reset: 0x0 Exists: MEMC_DDR3_OR_4==1.",
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
                    name: "burst_rdwr",
                    description: Some(
                        "Description: SDRAM burst length used: 0001 - Burst length of 2 (only supported for mDDR) 0010 - Burst length of 4 0100 - Burst length of 8 1000 - Burst length of 16 (only supported for mDDR and LPDDR2) All other values are reserved. This controls the burst size used to access the SDRAM. This must match the burst length mode register setting in the SDRAM. Burst length of 2 is not supported with AXI ports when MEMC_BURST_LENGTH is 8. Value After Reset: 0x4 Exists: Always.",
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
                    name: "active_ranks",
                    description: Some(
                        "Description: Only present for multi-rank configurations. Each bit represents one rank. For two-rank configurations, only bits[25:24] are present. 1 - populated 0 - unpopulated LSB is the lowest rank number. For 2 ranks following combinations are legal: 01 - One rank 11 - Two ranks Others - Reserved. For 4 ranks following combinations are legal: 0001 - One rank 0011 - Two ranks 1111 - Four ranks Value After Reset: \"(MEMC_NUM_RANKS==4) ? 0xF :((MEMC_NUM_RANKS==2) ? 0x3 : 0x1)\" Exists: MEMC_NUM_RANKS>1.",
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
            ],
        },
        FieldSet {
            name: "Odtcfg",
            extends: None,
            description: Some(
                "Description: ODT Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_odt_delay",
                    description: Some(
                        "Description: The delay, in clock cycles, from issuing a read command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CL + AL < 4), then 0. If (CL + AL >= 4), then (CL + AL - 4) DDR3 (CL - CWL) DDR4 If CAL mode is enabled, CL - CWL + DFITMG1.dfi_t_cmd_lat If CAL mode is not enabled, CL - CWL -1, or 0 if CL - CWL < 1 LPDDR3, MEMC_FREQ_RATIO=2 CL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rd_odt_hold",
                    description: Some(
                        "Description: Cycles to hold ODT for a read command. The minimum supported value is 2. Recommended values: DDR2/DDR3 BL8 - 0x6 BL4 - 0x4 DDR4 - 0x6, but needs to be reduced to 0x5 in CAL mode to avoid overlap of read and write ODT LPDDR3 - RU(tDQSCKmax/tCK) + 4 + 1 Value After Reset: 0x4 Exists: Always.",
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
                    name: "wr_odt_delay",
                    description: Some(
                        "Description: The delay, in clock cycles, from issuing a write command to setting ODT values associated with that command. ODT setting must remain constant for the entire time that DQS is driven by the uMCTL2. ODT is used only in DDR2, DDR3, DDR4 and LPDDR3 designs. Recommended values: DDR2 If (CWL + AL < 3), then 0. If (CWL + AL >= 3), then (CWL + AL - 3) DDR3 - 0 DDR4 - DFITMG1.dfi_t_cmd_lat (to adjust for CAL mode) LPDDR3 - (CWL - RU(tODToffmax/tCK)) Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wr_odt_hold",
                    description: Some(
                        "Description: Cycles to hold ODT for a write command. The minimum supported value is 2. DDR2/DDR3/DDR4 BL8 - 0x6 BL4 - 0x4 LPDDR3 - RU(tDQSSmax/tCK) + 4 Value After Reset: 0x4 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Odtmap",
            extends: None,
            description: Some(
                "Description: ODT/Rank Map Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rank0_wr_odt",
                    description: Some(
                        "Description: Indicates which remote ODTs must be turned on during a write to rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always.",
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
                    name: "rank0_rd_odt",
                    description: Some(
                        "Description: Indicates which remote ODTs must be turned on during a read from rank 0. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Value After Reset: 0x1 Exists: Always.",
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
                    name: "rank1_wr_odt",
                    description: Some(
                        "Description: Indicates which remote ODTs must be turned on during a write to rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1.",
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
                    name: "rank1_rd_odt",
                    description: Some(
                        "Description: Indicates which remote ODTs must be turned on during a read from rank 1. Each rank has a remote ODT (in the SDRAM) which can be turned on by setting the appropriate bit here. Rank 0 is controlled by the LSB; rank 1 is controlled by bit next to the LSB, etc. For each rank, set its bit to 1 to enable its ODT. Present only in configurations that have 2 or more ranks Value After Reset: \"(MEMC_NUM_RANKS>1) ? 0x2 : 0x0\" Exists: MEMC_NUM_RANKS>1.",
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
            ],
        },
        FieldSet {
            name: "Pccfg",
            extends: None,
            description: Some(
                "Description: Port Common Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "go2critical_en",
                    description: Some(
                        "Description: If set to 1 (enabled), sets co_gs_go2critical_wr and co_gs_go2critical_rd signals going to DDRC based on urgent input (awurgent, arurgent) coming from AXI master. If set to 0 (disabled), co_gs_go2critical_wr and co_gs_go2critical_rd signals at DDRC are driven to 1b'0. Value After Reset: 0x0 Exists: Always.",
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
                    name: "pagematch_limit",
                    description: Some(
                        "Description: Page match four limit. If set to 1, limits the number of consecutive same page DDRC transactions that can be granted by the Port Arbiter to four when Page Match feature is enabled. If set to 0, there is no limit imposed on number of consecutive same page DDRC transactions. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Perfhpr1",
            extends: None,
            description: Some(
                "Description: High Priority Read CAM Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpr_max_starve",
                    description: Some(
                        "Description: Number of clocks that the HPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always.",
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
                    name: "hpr_xact_run_length",
                    description: Some(
                        "Description: Number of transactions that are serviced once the HPR queue goes critical is the smaller of: This number Number of transactions available Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always.",
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
            name: "Perflpr1",
            extends: None,
            description: Some(
                "Description: Low Priority Read CAM Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpr_max_starve",
                    description: Some(
                        "Description: Number of clocks that the LPR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always.",
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
                    name: "lpr_xact_run_length",
                    description: Some(
                        "Description: Number of transactions that are serviced once the LPR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always.",
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
            name: "Perfvpr1",
            extends: None,
            description: Some(
                "Description: Variable Priority Read CAM Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpr_timeout_range",
                    description: Some(
                        "Description: Indicates the range of the timeout value that is used for grouping the expired VPR commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPR commands whose timeout counters are 15 or below will be considered as expired-VPR commands when the timeout value of any of the VPR commands reach 0. The expired-VPR commands, when present, are given higher priority than HPR commands. The VPR commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPR commands that come in to DDRC will time-out right-away and will be considered as expired-VPR. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPR. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPR_EN==1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Perfvpw1",
            extends: None,
            description: Some(
                "Description: Variable Priority Write CAM Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpw_timeout_range",
                    description: Some(
                        "Description: Indicates the range of the timeout value that is used for grouping the expired VPW commands in the CAM in DDRC. For example, if the register value is set to 0xF, then the priorities of all the VPW commands whose timeout counters are 15 or below will be considered as expired-VPW commands when the timeout value of any of the VPW commands reach 0. The expired-VPW commands, when present, are given higher priority than normal Write commands. The VPW commands are expected to consist of largely page hit traffic and by grouping them together the bus utilization is expected to increase. This register applies to transactions inside the DDRC only. The Max value for this register is 0x7FF and the Min value is 0x0. When programmed to the Max value of 0x7FF, all the VPW commands that come in to DDRC will time-out right-away and will be considered as expired-VPW. When programmed to the Min value of 0x0, the timer of each command would have to reach a value of 0 before it will be considered as expired-VPW. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: UMCTL2_VPW_EN==1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Perfwr1",
            extends: None,
            description: Some(
                "Description: Write CAM Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "w_max_starve",
                    description: Some(
                        "Description: Number of clocks that the WR queue can be starved before it goes critical. The minimum valid functional value for this register is 0x1. Programming it to 0x0 will disable the starvation functionality; during normal operation, this function should not be disabled as it will cause excessive latencies. Unit: Clock cycles. FOR PERFORMANCE ONLY. Value After Reset: 0x7f Exists: Always.",
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
                    name: "w_xact_run_length",
                    description: Some(
                        "Description: Number of transactions that are serviced once the WR queue goes critical is the smaller of: This number Number of transactions available. Unit: Transaction. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: Always.",
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
            name: "Pstat",
            extends: None,
            description: Some(
                "Description: Port Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_port_busy_0",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1.",
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
                    name: "rd_port_busy_1",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1.",
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
                    name: "rd_port_busy_2",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1.",
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
                    name: "rd_port_busy_3",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1.",
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
                    name: "rd_port_busy_4",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1.",
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
                    name: "rd_port_busy_5",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1.",
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
                    name: "rd_port_busy_6",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1.",
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
                    name: "rd_port_busy_7",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1.",
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
                    name: "rd_port_busy_8",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1.",
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
                    name: "rd_port_busy_9",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1.",
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
                    name: "rd_port_busy_10",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1.",
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
                    name: "rd_port_busy_11",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1.",
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
                    name: "rd_port_busy_12",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1.",
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
                    name: "rd_port_busy_13",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1.",
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
                    name: "rd_port_busy_14",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1.",
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
                    name: "rd_port_busy_15",
                    description: Some(
                        "Description: Indicates if there are outstanding reads for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1.",
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
                    name: "wr_port_busy_0",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 0. Value After Reset: 0x0 Exists: UMCTL2_PORT_0==1.",
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
                    name: "wr_port_busy_1",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 1. Value After Reset: 0x0 Exists: UMCTL2_PORT_1==1.",
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
                    name: "wr_port_busy_2",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 2. Value After Reset: 0x0 Exists: UMCTL2_PORT_2==1.",
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
                    name: "wr_port_busy_3",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 3. Value After Reset: 0x0 Exists: UMCTL2_PORT_3==1.",
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
                    name: "wr_port_busy_4",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 4. Value After Reset: 0x0 Exists: UMCTL2_PORT_4==1.",
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
                    name: "wr_port_busy_5",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 5. Value After Reset: 0x0 Exists: UMCTL2_PORT_5==1.",
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
                    name: "wr_port_busy_6",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 6. Value After Reset: 0x0 Exists: UMCTL2_PORT_6==1.",
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
                    name: "wr_port_busy_7",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 7. Value After Reset: 0x0 Exists: UMCTL2_PORT_7==1.",
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
                    name: "wr_port_busy_8",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 8. Value After Reset: 0x0 Exists: UMCTL2_PORT_8==1.",
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
                    name: "wr_port_busy_9",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 9. Value After Reset: 0x0 Exists: UMCTL2_PORT_9==1.",
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
                    name: "wr_port_busy_10",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 10. Value After Reset: 0x0 Exists: UMCTL2_PORT_10==1.",
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
                    name: "wr_port_busy_11",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 11. Value After Reset: 0x0 Exists: UMCTL2_PORT_11==1.",
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
                    name: "wr_port_busy_12",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 12. Value After Reset: 0x0 Exists: UMCTL2_PORT_12==1.",
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
                    name: "wr_port_busy_13",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 13. Value After Reset: 0x0 Exists: UMCTL2_PORT_13==1.",
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
                    name: "wr_port_busy_14",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 14. Value After Reset: 0x0 Exists: UMCTL2_PORT_14==1.",
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
                    name: "wr_port_busy_15",
                    description: Some(
                        "Description: Indicates if there are outstanding writes for port 15. Value After Reset: 0x0 Exists: UMCTL2_PORT_15==1.",
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
            name: "Pwrctl",
            extends: None,
            description: Some(
                "Description: Low Power Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "selfref_en",
                    description: Some(
                        "Description: If true then the uMCTL2 puts the SDRAM into Self Refresh after a programmable number of cycles \"maximum idle clocks before Self Refresh (PWRTMG.selfref_to_x32)\". This register bit may be re- programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always.",
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
                    name: "powerdown_en",
                    description: Some(
                        "Description: If true then the uMCTL2 goes into power-down after a programmable number of cycles \"maximum idle clocks before power down\" (PWRTMG.powerdown_to_x32). This register bit may be re-programmed during the course of normal operation. Value After Reset: 0x0 Exists: Always.",
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
                    name: "en_dfi_dram_clk_disable",
                    description: Some(
                        "Description: Enable the assertion of dfi_dram_clk_disable whenever a clock is not required by the SDRAM. If set to 0, dfi_dram_clk_disable is never asserted. Assertion of dfi_dram_clk_disable is as follows: In DDR2/DDR3, can only be asserted in Self Refresh. In DDR4, can be asserted in following: in Self Refresh. in Maximum Power Saving Mode In mDDR/LPDDR2/LPDDR3, can be asserted in following: in Self Refresh in Power Down in Deep Power Down during Normal operation (Clock Stop) Value After Reset: 0x0 Exists: Always.",
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
                    name: "selfref_sw",
                    description: Some(
                        "Description: A value of 1 to this register causes system to move to Self Refresh state immediately, as long as it is not in INIT or DPD/MPSM operating_mode. This is referred to as Software Entry/Exit to Self Refresh. 1 - Software Entry to Self Refresh 0 - Software Exit from Self Refresh Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Pwrtmg",
            extends: None,
            description: Some(
                "Description: Low Power Timing Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "powerdown_to_x32",
                    description: Some(
                        "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into power-down. This must be enabled in the PWRCTL.powerdown_en. Unit: Multiples of 32 clocks FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always.",
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
                    name: "selfref_to_x32",
                    description: Some(
                        "Description: After this many clocks of NOP or deselect the uMCTL2 automatically puts the SDRAM into Self Refresh. This must be enabled in the PWRCTL.selfref_en. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x40 Exists: Always.",
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
            name: "Qos0",
            extends: None,
            description: Some(
                "Description: Port n Read QoS Configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rqos_map_level1",
                    description: Some(
                        "Description: Separation level1 indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 13(for dual RAQ) or 0 to 14(for single RAQ) which corresponds to arqos. Note that for PA, arqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. All of the map_level* registers must be set to distinct values. Value After Reset: 0x0 Exists: Always.",
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
                    name: "rqos_map_region0",
                    description: Some(
                        "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region 0 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region0 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always.",
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
                    name: "rqos_map_region1",
                    description: Some(
                        "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: LPR, 1: VPR, 2: HPR. For dual address queue configurations, region1 maps to the blue address queue. In this case, valid values are 0: LPR and 1: VPR only. When VPR support is disabled (UMCTL2_VPR_EN = 0) and traffic class of region 1 is set to 1 (VPR) then VPR traffic is aliased to LPR traffic. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qos1",
            extends: None,
            description: Some(
                "Description: Port n Read QoS Configuration Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rqos_map_timeoutb",
                    description: Some(
                        "Description: Specifies the timeout value for transactions mapped to the blue address queue. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rqos_map_timeoutr",
                    description: Some(
                        "Description: Specifies the timeout value for transactions mapped to the red address queue. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "R",
            extends: None,
            description: Some(
                "Description: Port n Configuration Read Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_port_priority",
                    description: Some(
                        "Description: Determines the initial load value of read aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the read aging counter sets the priority of the read channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level (timeout condition - Priority0). For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (arqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. In this case, external dynamic priority input, arqos (for reads only) can still be used to set the DDRC read priority (2 priority levels: low priority read - LPR, high priority read - HPR) on a command by command basis. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always.",
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
                    name: "rd_port_aging_en",
                    description: Some(
                        "Description: If set to 1, enables aging function for the read channel of the port. Value After Reset: 0x0 Exists: Always.",
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
                    name: "rd_port_urgent_en",
                    description: Some(
                        "Description: If set to 1, enables the AXI urgent sideband signal (arurgent). When enabled and arurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_rd signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that arurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always.",
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
                    name: "rd_port_pagematch_en",
                    description: Some(
                        "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: \"(MEMC_DDR4_EN==1) ? 0x0 : 0x1\" Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Rankctl",
            extends: None,
            description: Some(
                "Description: Rank Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_rank_rd",
                    description: Some(
                        "Description: Only present for multi-rank configurations. Background: Reads to the same rank can be performed back-to-back. Reads to different ranks require additional gap dictated by the register RANKCTL.diff_rank_rd_gap. This is to avoid possible data bus contention as well as to give PHY enough time to switch the delay when changing ranks. The uMCTL2 arbitrates for bus access on a cycle-by-cycle basis; therefore after a read is scheduled, there are few clock cycles (determined by the value on diff_rank_rd_gap register) in which only reads from the same rank are eligible to be scheduled. This prevents reads from other ranks from having fair access to the data bus. This parameter represents the maximum number of reads that can be scheduled consecutively to the same rank. After this number is reached, a delay equal to RANKCTL.diff_rank_rd_gap is inserted by the scheduler to allow all ranks a fair opportunity to be scheduled. Higher numbers increase bandwidth utilization, lower numbers increase fairness. This feature can be DISABLED by setting this register to 0. When set to 0, the Controller will stay on the same rank as long as commands are available for it. Minimum programmable value is 0 (feature disabled) and maximum programmable value is 0xF. Feature limitation: max_rank_rd feature works as described only in the mode in which one command at the DDRC input results in one DFI command at the output. An example of this mode is: BL8 hardware configuration (MEMC_BURST_LENGTH=8) and Full bus width mode (MSTR.data_bus_width=2'b00) and BL8 mode of operation (MSTR.burst_rdwr=4'b0100). In modes where single HIF command results in multiple DFI commands (eg: Half Bus Width, BL4 etc.), the same rank commands would be serviced for as long as they are available, which is equivalent to this feature being disabled. FOR PERFORMANCE ONLY. Value After Reset: 0xf Exists: MEMC_NUM_RANKS>1.",
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
                    name: "diff_rank_rd_gap",
                    description: Some(
                        "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive reads to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during reads. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1.",
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
                    name: "diff_rank_wr_gap",
                    description: Some(
                        "Description: Only present for multi-rank configurations. Indicates the number of clocks of gap in data responses when performing consecutive writes to different ranks. This is used to switch the delays in the PHY to match the rank requirements. The value programmed in this register takes care of the ODT switch off timing requirement when switching ranks during writes. For configurations with MEMC_FREQ_RATIO=2, program this to (N/2) and round it up to the next integer value. N is value required by PHY, in terms of PHY clocks. Value After Reset: 0x6 Exists: MEMC_NUM_RANKS>1.",
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
            name: "Rfshctl0",
            extends: None,
            description: Some(
                "Description: Refresh Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refresh_burst",
                    description: Some(
                        "Description: The programmed value + 1 is the number of refresh timeouts that is allowed to accumulate before traffic is blocked and the refreshes are forced to execute. Closing pages to perform a refresh is a one-time penalty that must be paid for each group of refreshes. Therefore, performing refreshes in a burst reduces the per-refresh penalty of these page closings. Higher numbers for RFSHCTL.refresh_burst slightly increases utilization; lower numbers decreases the worst-case latency associated with refreshes. 0 - single refresh 1 - burst-of-2 refresh 7 - burst-of-8 refresh For information on burst refresh feature refer to section 3.9 of DDR2 JEDEC specification - JESD79-2F.pdf. For DDR2/3, the refresh is always per-rank and not per- bank. The rank refresh can be accumulated over 8*tREFI cycles using the burst refresh feature. In DDR4 mode, according to Fine Granuarity feature, 8 refreshes can be postponed in 1X mode, 16 refreshes in 2X mode and 32 refreshes in 4X mode. If using PHY-initiated updates, care must be taken in the setting of RFSHCTL0.refresh_burst, to ensure that tRFCmax is not violated due to a PHY-initiated update occurring shortly before a refresh burst was due. In this situation, the refresh burst will be delayed until the PHY- initiated update is complete. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refresh_to_x32",
                    description: Some(
                        "Description: If the refresh timer (tRFCnom, also known as tREFI) has expired at least once, but it has not expired (RFSHCTL0.refresh_burst+1) times yet, then a speculative refresh may be performed. A speculative refresh is a refresh performed at a time when refresh would be useful, but before it is absolutely required. When the SDRAM bus is idle for a period of time determined by this RFSHCTL0.refresh_to_x32 and the refresh timer has expired at least once since the last refresh, then a speculative refresh is performed. Speculative refreshes continues successively until there are no refreshes pending or until new reads or writes are issued to the uMCTL2. FOR PERFORMANCE ONLY. Value After Reset: 0x10 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refresh_margin",
                    description: Some(
                        "Description: Threshold value in number of clock cycles before the critical refresh or page timer expires. A critical refresh is to be issued before this threshold is reached. It is recommended that this not be changed from the default value, currently shown as 0x2. It must always be less than internally used t_rfc_nom_x32. Note that, in LPDDR2/LPDDR3, internally used t_rfc_nom_x32 may be equal to RFSHTMG.t_rfc_nom_x32>>2 if derating is enabled (DERATEEN.derate_enable=1). Otherwise, internally used t_rfc_nom_x32 will be equal to RFSHTMG.t_rfc_nom_x32. Unit: Multiples of 32 clocks. Value After Reset: 0x2 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Rfshctl1",
            extends: None,
            description: Some(
                "Description: Refresh Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refresh_timer0_start_value_x32",
                    description: Some(
                        "Description: Refresh timer start for rank 0 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
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
                    name: "refresh_timer1_start_value_x32",
                    description: Some(
                        "Description: Refresh timer start for rank 1 (only present in multi-rank configurations). This is useful in staggering the refreshes to multiple ranks to help traffic to proceed. This is explained in Refresh Controls section of architecture chapter. Unit: Multiples of 32 clocks. FOR PERFORMANCE ONLY. Value After Reset: 0x0 Exists: MEMC_NUM_RANKS>1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rfshctl3",
            extends: None,
            description: Some(
                "Description: Refresh Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dis_auto_refresh",
                    description: Some(
                        "Description: When '1', disable auto-refresh generated by the uMCTL2. When auto-refresh is disabled, the SoC core must generate refreshes using the registers reg_ddrc_rank0_refresh, reg_ddrc_rank1_refresh, reg_ddrc_rank2_refresh and reg_ddrc_rank3_refresh. When dis_auto_refresh transitions from 0 to 1, any pending refreshes are immediately scheduled by the uMCTL2. If DDR4 CRC/parity retry is enabled (CRCPARCTL1.crc_parity_retry_enable = 1), disable auto- refresh is not supported, and this bit must be set to '0'. This register field is changeable on the fly. Value After Reset: 0x0 Exists: Always.",
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
                    name: "refresh_update_level",
                    description: Some(
                        "Description: Toggle this signal (either from 0 to 1 or from 1 to 0) to indicate that the refresh register(s) have been updated. The value is automatically updated when exiting soft reset, so it does not need to be toggled initially. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Rfshtmg",
            extends: None,
            description: Some(
                "Description: Refresh Timing Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_rfc_min",
                    description: Some(
                        "Description: tRFC (min): Minimum time from refresh to refresh or activate. For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tRFCab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tRFCpb For configurations with MEMC_FREQ_RATIO=2, program this to tRFC(min)/2 and round up to next integer value. In DDR4 mode, tRFC(min) value is different depending on the refresh mode (fixed 1X,2X,4X) and the device density. The user should program the appropriate value from the spec based on the 'refresh_mode' and the device density that is used. Unit: Clocks. Value After Reset: 0x8c Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "t_rfc_nom_x32",
                    description: Some(
                        "Description: tREFI: Average time interval between refreshes per rank (specification: 7.8us for DDR2, DDR3 and DDR4. See JEDEC specification for mDDR, LPDDR2 and LPDDR3). For LPDDR2/LPDDR3: if using all-bank refreshes (RFSHCTL0.per_bank_refresh = 0), this register should be set to tREFIab if using per-bank refreshes (RFSHCTL0.per_bank_refresh = 1), this register should be set to tREFIpb For configurations with MEMC_FREQ_RATIO=2, program this to (tREFI/2), no rounding up. In DDR4 mode, tREFI value is different depending on the refresh mode. The user should program the appropriate value from the spec based on the value programmed in the refresh mode register. Note that RFSHTMG.t_rfc_nom_x32 * 32 must be greater than RFSHTMG.t_rfc_min. Unit: Multiples of 32 clocks. Value After Reset: 0x62 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sbrctl",
            extends: None,
            description: Some(
                "Description: Scrubber Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scrub_en",
                    description: Some(
                        "Description: Enable ECC scrubber. If set to 1, enables the scrubber to generate background read commands after the memories are initialized. If set to 0, disables the scrubber, resets the address generator to 0 and clears the scrubber status. This bitfield must be accessed separately from the other bitfields in this register. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
                    name: "scrub_during_lowpower",
                    description: Some(
                        "Description: Continue scrubbing during low power. If set to 1, burst of scrubs will be issued in HW controlled low power modes. There are two such modes: automatically initiated by idleness or initiated by HW low-power (LP) interface. If set to 0, the scrubber will not attempt to send commands while the DDRC is in HW controlled low power modes. In this case, the scrubber will remember the last address issued and will automatically continue from there when the DDRC exits the LP mode. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
                    name: "scrub_mode",
                    description: Some(
                        "Description: scrub_mode:0 ECC scrubber will perform reads scrub_mode:1 ECC scrubber will perform writes Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
                    name: "scrub_burst",
                    description: Some(
                        "Description: Scrub burst count. Determines the number of back-to-back scrub read commands that can be issued together when the controller is in one of the HW controlled low power modes. During low power, the period of the scrub burst becomes \\\"scrub_burst*scrub_interval\\\" cycles. During normal operation mode of the controller (ie. not in power-down or self refresh), scrub_burst is ignored and only one scrub command is generated. Valid values are: 1: 1 read, 2: 4 reads, 3: 16 reads, 4: 64 reads, 5: 256 reads, 6: 1024 reads. Value After Reset: 0x1 Exists: UMCTL2_SBR_EN_1==1.",
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
                    name: "scrub_interval",
                    description: Some(
                        "Description: Scrub interval. (512 x scrub_interval) number of clock cycles between two scrub read commands. If set to 0, scrub commands are issued back-to-back. This mode of operation (scrub_interval=0) can typically be used for scrubbing the full range of memory at once before or after SW controlled low power operations. After completing the full range of scrub while scrub_interval=0, scrub_done register is set and sbr_done_intr interrupt signal is asserted. Value After Reset: 0xff Exists: UMCTL2_SBR_EN_1==1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sbrstat",
            extends: None,
            description: Some(
                "Description: Scrubber Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scrub_busy",
                    description: Some(
                        "Description: Scrubber busy. Controller sets this bit to 1 when the scrubber logic has outstanding read commands being executed. Cleared when there are no active outstanding scrub reads in the system. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
                    name: "scrub_done",
                    description: Some(
                        "Description: Scrubber done. Controller sets this bit to 1, after full range of addresses are scrubbed once while scrub_interval is set to 0. Cleared if scrub_en is set to 0 (scrubber disabled) or scrub_interval is set to a non-zero value for normal scrub operation. The interrupt signal, sbr_done_intr, is equivalent to this status bitfield. Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
            ],
        },
        FieldSet {
            name: "Sbrwdata0",
            extends: None,
            description: Some(
                "Description: Scrubber Write Data Pattern0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scrub_pattern0",
                    description: Some(
                        "Description: ECC Scrubber write data pattern for data bus[31:0] Value After Reset: 0x0 Exists: UMCTL2_SBR_EN_1==1.",
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
            name: "Sched",
            extends: None,
            description: Some(
                "Description: Scheduler Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_low_pri_n",
                    description: Some(
                        "Description: Active low signal. When asserted ('0'), all incoming transactions are forced to low priority. This implies that all High Priority Read (HPR) and Variable Priority Read commands (VPR) will be treated as Low Priority Read (LPR) commands. On the write side, all Variable Priority Write (VPW) commands will be treated as Normal Priority Write (NPW) commands. Forcing the incoming transactions to low priority implicitly turns off Bypass path for read commands. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always.",
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
                    name: "prefer_write",
                    description: Some(
                        "Description: If set then the bank selector prefers writes over reads. FOR DEBUG ONLY. Value After Reset: 0x0 Exists: Always.",
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
                    name: "pageclose",
                    description: Some(
                        "Description: If true, bank is kept open only until there are page hit transactions available in the CAM to that bank. The last read or write command in the CAM with a bank and page hit will be executed with auto-precharge if SCHED1.pageclose_timer=0. Even if this register set to 1 and SCHED1.pageclose_timer is set to 0, explicit precharge (and not auto-precharge) may be issued in some cases where there is a mode switch between Write and Read or between LPR and HPR. The Read and Write commands that are executed as part of the ECC scrub requests are also executed without auto-precharge. If false, the bank remains open until there is a need to close it (to open a different page, or for page timeout or refresh timeout) - also known as open page policy. The open page policy can be overridden by setting the per-command-autopre bit on the HIF interface (co_ih_rxcmd_autopre). The pageclose feature provids a midway between Open and Close page policies. FOR PERFORMANCE ONLY. Value After Reset: 0x1 Exists: Always.",
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
                    name: "lpr_num_entries",
                    description: Some(
                        "Description: Number of entries in the low priority transaction store is this value + 1. (MEMC_NO_OF_ENTRY - (SCHED.lpr_num_entries + 1)) is the number of entries available for the high priority transaction store. Setting this to maximum value allocates all entries to low priority transaction store. Setting this to 0 allocates 1 entry to low priority transaction store and the rest to high priority transaction store. Note: In ECC configurations, the numbers of write and low priority read credits issued is one less than in the non-ECC case. One entry each is reserved in the write and low- priority read CAMs for storing the RMW requests arising out of single bit error correction RMW operation. Value After Reset: \"MEMC_NO_OF_ENTRY/2\" Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "go2critical_hysteresis",
                    description: Some(
                        "Description: UNUSED Value After Reset: 0x0 Exists: Always.",
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
                    name: "rdwr_idle_gap",
                    description: Some(
                        "Description: When the preferred transaction store is empty for these many clock cycles, switch to the alternate transaction store if it is non-empty. The read transaction store (both high and low priority) is the default preferred transaction store and the write transaction store is the alternative store. When prefer write over read is set this is reversed. 0x0 is a legal value for this register. When set to 0x0, the transaction store switching will happen immediately when the switching conditions become true. FOR PERFORMANCE ONLY Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sched1",
            extends: None,
            description: Some(
                "Description: Scheduler Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pageclose_timer",
                    description: Some(
                        "Description: This field works in conjunction with SCHED.pageclose. It only has meaning if SCHED.pageclose==1. If SCHED.pageclose==1 and pageclose_timer==0, then an auto-precharge may be scheduled for last read or write command in the CAM with a bank and page hit. Note, sometimes an explicit precharge is scheduled instead of the auto-precharge. See SCHED.pageclose for details of when this may happen. If SCHED.pageclose==1 and pageclose_timer>0, then an auto-precharge is not scheduled for last read or write command in the CAM with a bank and page hit. Instead, a timer is started, with pageclose_timer as the initial value. There is a timer on a per bank basis. The timer decrements unless the next read or write in the CAM to a bank is a page hit. It gets reset to pageclose_timer value if the next read or write in the CAM to a bank is a page hit. Once the timer has reached zero, an explcit precharge will be attempted to be scheduled. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Size",
            extends: None,
            description: Some(
                "Description: SAR Size Register n.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nblocks",
                    description: Some(
                        "Description: Number of blocks for address region n. This register determines the total size of the region in multiples of minimum block size as specified by the hardware parameter UMCTL2_SARMINSIZE. The register value is encoded as number of blocks = nblocks + 1. Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "Description: Operating Mode Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "operating_mode",
                    description: Some(
                        "Description: Operating mode. This is 3-bits wide in configurations with mDDR/LPDDR2/LPDDR3/DDR4 support and 2-bits in all other configurations. non-mDDR/LPDDR2/LPDDR3 and non-DDR4 designs: 00 - Init 01 - Normal 10 - Power-down 11 - Self refresh mDDR/LPDDR2/LPDDR3 or DDR4 designs: 000 - Init 001 - Normal 010 - Power-down 011 - Self refresh 1XX - Deep power-down / Maximum Power Saving Mode Value After Reset: 0x0 Exists: Always.",
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
                    name: "selfref_type",
                    description: Some(
                        "Description: Flags if Self Refresh is entered and if it was under Automatic Self Refresh control only or not. 00 - SDRAM is not in Self Refresh 11 - SDRAM is in Self Refresh and Self Refresh was caused by Automatic Self Refresh only 10 - SDRAM is in Self Refresh and Self Refresh was not caused solely under Automatic Self Refresh control. It could have been caused by Hardware Low Power Interface and/or Software (reg_ddrc_selfref_sw). Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Valuech",
            extends: None,
            description: Some(
                "Description: Port n Channel m Configuration ID Value Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id_value",
                    description: Some(
                        "Description: Determines the value used in the ID mapping function for virtual channel m. Value After Reset: 0x0 Exists: Always.",
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
            name: "W",
            extends: None,
            description: Some(
                "Description: Port n Configuration Write Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_port_priority",
                    description: Some(
                        "Description: Determines the initial load value of write aging counters. These counters will be parallel loaded after reset, or after each grant to the corresponding port. The aging counters down-count every clock cycle where the port is requesting but not granted. The higher significant 5-bits of the write aging counter sets the initial priority of the write channel of a given port. Port's priority will increase as the higher significant 5-bits of the counter starts to decrease. When the aging counter becomes 0, the corresponding port channel will have the highest priority level. For multi-port configurations, the aging counters cannot be used to set port priorities when external dynamic priority inputs (awqos) are enabled (timeout is still applicable). For single port configurations, the aging counters are only used when they timeout (become 0) to force read-write direction switching. Note: The two LSBs of this register field are tied internally to 2'b00. Value After Reset: 0x0 Exists: Always.",
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
                    name: "wr_port_aging_en",
                    description: Some(
                        "Description: If set to 1, enables aging function for the write channel of the port. Value After Reset: 0x0 Exists: Always.",
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
                    name: "wr_port_urgent_en",
                    description: Some(
                        "Description: If set to 1, enables the AXI urgent sideband signal (awurgent). When enabled and awurgent is asserted by the master, that port becomes the highest priority and co_gs_go2critical_wr signal to DDRC is asserted if enabled in PCCFG.go2critical_en register. Note that awurgent signal can be asserted anytime and as long as required which is independent of address handshaking (it is not associated with any particular command). Value After Reset: 0x0 Exists: Always.",
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
                    name: "wr_port_pagematch_en",
                    description: Some(
                        "Description: If set to 1, enables the Page Match feature. If enabled, once a requesting port is granted, the port is continued to be granted if the following immediate commands are to the same memory page (i.e. same bank and same row). See also related PCCFG.pagematch_limit register. Value After Reset: 0x1 Exists: Always.",
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
            ],
        },
        FieldSet {
            name: "Wqos0",
            extends: None,
            description: Some(
                "Description: Port n Write QoS Configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wqos_map_level",
                    description: Some(
                        "Description: Separation level indicating the end of region0 mapping; start of region0 is 0. Possible values for level1 are 0 to 14 which corresponds to awqos. Note that for PA, awqos values are used directly as port priorities, where the higher the value corresponds to higher port priority. Value After Reset: 0x0 Exists: Always.",
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
                    name: "wqos_map_region0",
                    description: Some(
                        "Description: This bitfield indicates the traffic class of region 0. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region0 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always.",
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
                    name: "wqos_map_region1",
                    description: Some(
                        "Description: This bitfield indicates the traffic class of region 1. Valid values are: 0: NPW 1: VPW When VPW support is disabled (UMCTL2_VPW_EN = 0) and traffic class of region 1 is set to 1 (VPW) then VPW traffic is aliased to NPW traffic. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wqos1",
            extends: None,
            description: Some(
                "Description: Port n Write QoS Configuration Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wqos_map_timeout",
                    description: Some(
                        "Description: Specifies the timeout value for write transactions. Value After Reset: 0x0 Exists: Always.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Zqctl0",
            extends: None,
            description: Some(
                "Description: ZQ Control Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_zq_short_nop",
                    description: Some(
                        "Description: tZQCS: Number of cycles of NOP required after a ZQCS (ZQ calibration short) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2, program this to tZQCS/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x40 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
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
                    name: "t_zq_long_nop",
                    description: Some(
                        "Description: tZQoper for DDR3/DDR4, tZQCL for LPDDR2/LPDDR3: Number of cycles of NOP required after a ZQCL (ZQ calibration long) command is issued to SDRAM. For configurations with MEMC_FREQ_RATIO=2: DDR3/DDR4: program this to tZQoper/2 and round it up to the next integer value. LPDDR2/LPDDR3: program this to tZQCL/2 and round it up to the next integer value. Unit: Clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x200 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "zq_resistor_shared",
                    description: Some(
                        "Description: 1 - Denotes that ZQ resistor is shared between ranks. Means ZQinit/ZQCL/ZQCS commands are sent to one rank at a time with tZQinit/tZQCL/tZQCS timing met between commands so that commands to different ranks do not overlap. 0 - ZQ resistor is not shared. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
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
                    name: "dis_srx_zqcl",
                    description: Some(
                        "Description: 1 - Disable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. 0 - Enable issuing of ZQCL command at Self-Refresh exit. Only applicable when run in DDR3 or DDR4 or LPDDR2 or LPDDR3 mode. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
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
                    name: "dis_auto_zq",
                    description: Some(
                        "Description: 1 - Disable uMCTL2 generation of ZQCS command. Register reg_ddrc_zq_calib_short can be used instead to control ZQ calibration commands. 0 - Internally generate ZQCS commands based on ZQCTL1.t_zq_short_interval_x1024. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x0 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
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
            name: "Zqctl1",
            extends: None,
            description: Some(
                "Description: ZQ Control Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "t_zq_short_interval_x1024",
                    description: Some(
                        "Description: Average interval to wait between automatically issuing ZQCS (ZQ calibration short) commands to DDR3/DDR4/LPDDR2/LPDDR3 devices. Meaningless, if ZQCTL0.dis_auto_zq=1. Unit: 1024 clock cycles. This is only present for designs supporting DDR3/DDR4 or LPDDR2/LPDDR3 devices. Value After Reset: 0x100 Exists: MEMC_DDR3==1 || MEMC_DDR4==1 || MEMC_LPDDR2==1.",
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
            ],
        },
        FieldSet {
            name: "Zqstat",
            extends: None,
            description: Some(
                "Description: ZQ Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zq_reset_busy",
                    description: Some(
                        "Description: SoC core may initiate a ZQ Reset operation only if this signal is low. This signal goes high in the clock after the uMCTL2 accepts the ZQ Reset request. It goes low when the ZQ Reset command is issued to the SDRAM and the associated NOP period is over. It is recommended not to perform ZQ Reset commands when this signal is high. 0 - Indicates that the SoC core can initiate a ZQ Reset operation 1 - Indicates that ZQ Reset operation is in progress Value After Reset: 0x0 Exists: Always.",
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
            ],
        },
    ],
    enums: &[],
};
