use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Otp",
            extends: None,
            description: Some(
                "OTP.",
            ),
            items: &[
                BlockItem {
                    name: "shadow",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 128,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shadow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shadow_lock",
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
                                "ShadowLock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fuse",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 128,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fuse",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fuse_lock",
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
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FuseLock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "unlock",
                    description: Some(
                        "UNLOCK.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Unlock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "DATA.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr",
                    description: Some(
                        "ADDR.",
                    ),
                    array: None,
                    byte_offset: 0x808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd",
                    description: Some(
                        "CMD.",
                    ),
                    array: None,
                    byte_offset: 0x80c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "load_req",
                    description: Some(
                        "LOAD Request.",
                    ),
                    array: None,
                    byte_offset: 0xa00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LoadReq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "load_comp",
                    description: Some(
                        "LOAD complete.",
                    ),
                    array: None,
                    byte_offset: 0xa04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LoadComp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "region",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 4,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Region",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0xc00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0xc04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr",
            extends: None,
            description: Some(
                "ADDR.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "word address to be read or write.",
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
            ],
        },
        FieldSet {
            name: "Cmd",
            extends: None,
            description: Some(
                "CMD.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd",
                    description: Some(
                        "command to access fure array \"BLOW\" will update fuse word at ADDR to value hold in DATA \"READ\" will fetch fuse value in at ADDR to DATA register.",
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
            name: "Data",
            extends: None,
            description: Some(
                "DATA.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "data register for non-blocking access this register hold dat read from fuse array or data to by programmed to fuse array.",
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
            name: "Fuse",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fuse",
                    description: Some(
                        "fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready).",
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
            name: "FuseLock",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "lock for fuse array, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "interrupt enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "load",
                    description: Some(
                        "fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable.",
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
                    name: "read",
                    description: Some(
                        "fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable.",
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
                    name: "write",
                    description: Some(
                        "fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "interrupt flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "load",
                    description: Some(
                        "fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded.",
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
                    name: "read",
                    description: Some(
                        "fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register.",
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
                    name: "write",
                    description: Some(
                        "fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse.",
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
            name: "LoadComp",
            extends: None,
            description: Some(
                "LOAD complete.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "complete",
                    description: Some(
                        "reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3.",
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
            ],
        },
        FieldSet {
            name: "LoadReq",
            extends: None,
            description: Some(
                "LOAD Request.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "request",
                    description: Some(
                        "reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3.",
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
            ],
        },
        FieldSet {
            name: "Region",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable.",
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
                    name: "stop",
                    description: Some(
                        "stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable.",
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
            name: "Shadow",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "shadow",
                    description: Some(
                        "shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128.",
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
            name: "ShadowLock",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock",
                    description: Some(
                        "lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked.",
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
            name: "Unlock",
            extends: None,
            description: Some(
                "UNLOCK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "unlock",
                    description: Some(
                        "unlock word for fuse array operation write \"OPEN\" to unlock fuse array, write any other value will lock write to fuse. Please make sure 24M crystal is running and 2.5V LDO working properly.",
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
    enums: &[],
};
