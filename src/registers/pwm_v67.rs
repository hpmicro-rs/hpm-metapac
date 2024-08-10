use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwm",
            extends: None,
            description: Some(
                "PWM0.",
            ),
            items: &[
                BlockItem {
                    name: "unlk",
                    description: Some(
                        "Shadow registers unlock register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Unlk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sta",
                    description: Some(
                        "Counter start register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rld",
                    description: Some(
                        "Counter reload register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rld",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frcmd",
                    description: Some(
                        "Force output mode register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Frcmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shlk",
                    description: Some(
                        "Shadow registers lock register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shlk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "Global control register.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
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
                    name: "shcr",
                    description: Some(
                        "Shadow register control register.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cappos",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cappos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt",
                    description: Some(
                        "Counter.",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "capneg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Capneg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cntcopy",
                    description: Some(
                        "Counter copy.",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cntcopy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwmcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwmcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register.",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irqen",
                    description: Some(
                        "Interrupt request enable register.",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irqen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaen",
                    description: Some(
                        "DMA request enable register.",
                    ),
                    array: None,
                    byte_offset: 0x22c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmpcfg",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 24,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmpcfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Capneg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "capneg",
                    description: Some(
                        "counter value captured at input signal falling edge.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cappos",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cappos",
                    description: Some(
                        "counter value captured at input posedge.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chcfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "outpol",
                    description: Some(
                        "output polarity, set to 1 will invert the output.",
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
                    name: "cmpselbeg",
                    description: Some(
                        "assign the first comparator for this output channel.",
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
                    name: "cmpselend",
                    description: Some(
                        "assign the last comparator for this output channel.",
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
            name: "Cmp",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpjit",
                    description: Some(
                        "jitter counter compare value.",
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
                    name: "cmphlf",
                    description: Some(
                        "half clock counter compare value.",
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
                    name: "cmp",
                    description: Some(
                        "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xcmp",
                    description: Some(
                        "extended counter compare value.",
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
            name: "Cmpcfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmode",
                    description: Some(
                        "comparator mode 0- output compare mode 1- input capture mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "CmpMode",
                    ),
                },
                Field {
                    name: "cmpshdwupt",
                    description: Some(
                        "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ShadowUpdateTrigger",
                    ),
                },
                Field {
                    name: "xcntcmpen",
                    description: Some(
                        "This bitfield enable the comparator to compare xcmp with xcnt.",
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
            ],
        },
        FieldSet {
            name: "Cnt",
            extends: None,
            description: Some(
                "Counter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "current clock counter value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xcnt",
                    description: Some(
                        "current extended counter value.",
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
            name: "Cntcopy",
            extends: None,
            description: Some(
                "Counter copy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "current clock counter value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xcnt",
                    description: Some(
                        "current extended counter value.",
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
            name: "Dmaen",
            extends: None,
            description: Some(
                "DMA request enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpenx",
                    description: Some(
                        "comparator output compare or input capture flag DMA request enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rlden",
                    description: Some(
                        "reload flag DMA request enable.",
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
                    name: "halfrlden",
                    description: Some(
                        "half reload flag DMA request enable.",
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
                    name: "xrlden",
                    description: Some(
                        "extended reload flag DMA request enable.",
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
                    name: "faulten",
                    description: Some(
                        "fault condition DMA request enable.",
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
            ],
        },
        FieldSet {
            name: "Frcmd",
            extends: None,
            description: Some(
                "Force output mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frcmd",
                    description: Some(
                        "2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force.",
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
            name: "Gcr",
            extends: None,
            description: Some(
                "Global control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swfrc",
                    description: Some(
                        "1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect.",
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
                    name: "frctime",
                    description: Some(
                        "This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xrldsyncen",
                    description: Some(
                        "1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled.",
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
                    name: "faultclr",
                    description: Some(
                        "1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again.",
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
                    name: "cen",
                    description: Some(
                        "1- enable the pwm timer counter 0- stop the pwm timer counter.",
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
                    name: "rldsyncen",
                    description: Some(
                        "1- pwm timer counter reset to reload value (rld) by synci is enabled.",
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
                    name: "faultexpol",
                    description: Some(
                        "external fault polarity 1-active low 0-active high.",
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
                Field {
                    name: "faulte0en",
                    description: Some(
                        "1- enable the external fault input 0.",
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
                    name: "faulte1en",
                    description: Some(
                        "1- enable the external fault input 1.",
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
                    name: "faultrechwsel",
                    description: Some(
                        "Selec one of the 24 comparators as fault output recover trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "faultrecedg",
                    description: Some(
                        "When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge.",
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
                    name: "cmpshdwsel",
                    description: Some(
                        "This bitfield select one of the comparators as hardware event time to load comparator shadow registers.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hwshdwedg",
                    description: Some(
                        "When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge.",
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
                    name: "frcpol",
                    description: Some(
                        "polarity of input pwm_force, 1- active low 0- active high.",
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
                    name: "debugfault",
                    description: Some(
                        "1- enable debug mode output protection.",
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
                    name: "faulti0en",
                    description: Some(
                        "1- enable the internal fault input 0.",
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
                    name: "faulti1en",
                    description: Some(
                        "1- enable the internal fault input 1.",
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
                    name: "faulti2en",
                    description: Some(
                        "1- enable the internal fault input 2.",
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
                    name: "faulti3en",
                    description: Some(
                        "1- enable the internal fault input 3.",
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
            name: "Irqen",
            extends: None,
            description: Some(
                "Interrupt request enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpirqex",
                    description: Some(
                        "comparator output compare or input capture flag interrupt enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rldirqe",
                    description: Some(
                        "reload flag interrupt enable.",
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
                    name: "halfrldirqe",
                    description: Some(
                        "half reload flag interrupt enable.",
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
                    name: "xrldirqe",
                    description: Some(
                        "extended reload flag interrupt enable.",
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
                    name: "faultirqe",
                    description: Some(
                        "fault condition interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "Pwmcfg",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "deadarea",
                    description: Some(
                        "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled.",
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
                    name: "pair",
                    description: Some(
                        "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode.",
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
                    name: "frcsrcsel",
                    description: Some(
                        "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1.",
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
                    name: "faultrectime",
                    description: Some(
                        "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "faultmode",
                    description: Some(
                        "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz.",
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
                Field {
                    name: "frcshdwupt",
                    description: Some(
                        "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ShadowUpdateTrigger",
                    ),
                },
                Field {
                    name: "oen",
                    description: Some(
                        "PWM output enable 1- output is enabled 0- output is disabled.",
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
            name: "Rld",
            extends: None,
            description: Some(
                "Counter reload register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rld",
                    description: Some(
                        "pwm timer counter reload value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xrld",
                    description: Some(
                        "timeout counter extended reload point, counter will reload to xsta after reach this point.",
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
            name: "Shcr",
            extends: None,
            description: Some(
                "Shadow register control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shlken",
                    description: Some(
                        "1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0.",
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
                    name: "cntshdwupt",
                    description: Some(
                        "This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "ShadowUpdateTrigger",
                    ),
                },
                Field {
                    name: "cntshdwsel",
                    description: Some(
                        "This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "frcshdwsel",
                    description: Some(
                        "This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers.",
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
            name: "Shlk",
            extends: None,
            description: Some(
                "Shadow registers lock register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shlk",
                    description: Some(
                        "write 1 to lock all shawdow register, write access is not permitted.",
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
            name: "Sr",
            extends: None,
            description: Some(
                "Status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpfx",
                    description: Some(
                        "comparator output compare or input capture flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rldf",
                    description: Some(
                        "reload flag, this flag set when cnt count to rld value or when SYNCI assert.",
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
                    name: "halfrldf",
                    description: Some(
                        "half reload flag, this flag set when cnt count to rld/2.",
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
                    name: "xrldf",
                    description: Some(
                        "extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert.",
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
                    name: "faultf",
                    description: Some(
                        "fault condition flag.",
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
            ],
        },
        FieldSet {
            name: "Sta",
            extends: None,
            description: Some(
                "Counter start register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sta",
                    description: Some(
                        "pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xsta",
                    description: Some(
                        "pwm timer counter extended start point, should back to this value after reach xrld.",
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
            name: "Unlk",
            extends: None,
            description: Some(
                "Shadow registers unlock register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shunlk",
                    description: Some(
                        "write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written.",
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
    ],
    enums: &[
        Enum {
            name: "CmpMode",
            description: Some(
                "comparator mode",
            ),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "OUTPUT_COMPARE",
                    description: Some(
                        "output compare mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "INPUT_CAPTURE",
                    description: Some(
                        "input capture mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "ShadowUpdateTrigger",
            description: Some(
                "no description available.",
            ),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ON_SHLK",
                    description: Some(
                        "after software set shlk bit of shlk register",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ON_MODIFY",
                    description: Some(
                        "immediately after the register being modified",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "ON_HW_EVENT",
                    description: Some(
                        "after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ON_SHSYNCI",
                    description: Some(
                        "after SHSYNCI assert.",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
