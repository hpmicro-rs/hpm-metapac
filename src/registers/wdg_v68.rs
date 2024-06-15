use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Wdg",
            extends: None,
            description: Some(
                "EWDG0.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl0",
                    description: Some(
                        "wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl1",
                    description: Some(
                        "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ot_rst_val",
                    description: Some(
                        "wdog timeout reset counter value.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OtRstVal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdt_refresh_reg",
                    description: Some(
                        "wdog refresh register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdtRefreshReg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdt_status",
                    description: Some(
                        "wdog status register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdtStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg_prot",
                    description: Some(
                        "ctrl register protection register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CfgProt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ref_prot",
                    description: Some(
                        "refresh protection register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RefProt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdt_en",
                    description: Some(
                        "Wdog enable.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdtEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ref_time",
                    description: Some(
                        "Refresh period value.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RefTime",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CfgProt",
            extends: None,
            description: Some(
                "ctrl register protection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "upd_psd",
                    description: Some(
                        "The password of unlocking register update.",
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
                    name: "upd_ot_time",
                    description: Some(
                        "The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle.",
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
            ],
        },
        FieldSet {
            name: "Ctrl0",
            extends: None,
            description: Some(
                "wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_lp",
                    description: Some(
                        "WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode.",
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
                    name: "en_dbg",
                    description: Some(
                        "WTD enable or not in debug mode.",
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
                    name: "ref_unlock_mec",
                    description: Some(
                        "Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1.",
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
                    name: "ref_lock",
                    description: Some(
                        "WDT refresh has to be unlocked firstly once refresh lock is enable.",
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
                    name: "win_upper",
                    description: Some(
                        "The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation.",
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
                    name: "ref_ot_req",
                    description: Some(
                        "If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter.",
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
                    name: "ot_self_clear",
                    description: Some(
                        "overtime reset can be self released after 32 function cycles.",
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
                    name: "cfg_lock",
                    description: Some(
                        "The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register.",
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
                    name: "win_lower",
                    description: Some(
                        "Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value.",
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
                    name: "win_en",
                    description: Some(
                        "window mode enable.",
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
                    name: "div_value",
                    description: Some(
                        "clock divider, the clock divider works as 2 ^ div_value for wdt counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clk_sel",
                    description: Some(
                        "clock select 0：bus clock 1：ext clock.",
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
            name: "Ctrl1",
            extends: None,
            description: Some(
                "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "parity_fail_int_en",
                    description: Some(
                        "Parity error will trigger a interrupt.",
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
                    name: "parity_fail_rst_en",
                    description: Some(
                        "Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits.",
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
                    name: "unl_ctl_fail_int_en",
                    description: Some(
                        "Unlock register update failure will trigger a interrupt.",
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
                    name: "unl_ctl_fail_rst_en",
                    description: Some(
                        "Unlock register update failure will trigger a reset.",
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
                    name: "ctl_vio_int_en",
                    description: Some(
                        "Ctrl update violation will trigger a interrupt.",
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
                    name: "ctl_vio_rst_en",
                    description: Some(
                        "Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them.",
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
                    name: "ot_rst_en",
                    description: Some(
                        "WDT overtime will generate a reset.",
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
                    name: "unl_ref_fail_int_en",
                    description: Some(
                        "Refresh unlock fail will trigger a interrupt.",
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
                    name: "unl_ref_fail_rst_en",
                    description: Some(
                        "Refresh unlock fail will trigger a reset.",
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
                    name: "ref_fail_int_en",
                    description: Some(
                        "Refresh violation will trigger an interrupt.",
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
                    name: "ref_fail_rst_en",
                    description: Some(
                        "Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt.",
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
            name: "OtRstVal",
            extends: None,
            description: Some(
                "wdog timeout reset counter value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ot_rst_val",
                    description: Some(
                        "WDT timeout reset value.",
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
            name: "RefProt",
            extends: None,
            description: Some(
                "refresh protection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ref_unl_psd",
                    description: Some(
                        "The password to unlock refreshing.",
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
            name: "RefTime",
            extends: None,
            description: Some(
                "Refresh period value.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "refresh_period",
                    description: Some(
                        "The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter.",
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
            name: "WdtEn",
            extends: None,
            description: Some(
                "Wdog enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdog_en",
                    description: Some(
                        "Wdog is enabled, the re-written of this register is impacted by enable lock function.",
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
            name: "WdtRefreshReg",
            extends: None,
            description: Some(
                "wdog refresh register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdt_refresh_reg",
                    description: Some(
                        "Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose.",
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
            name: "WdtStatus",
            extends: None,
            description: Some(
                "wdog status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ref_vio",
                    description: Some(
                        "Refresh fail Write one to clear the bit.",
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
                    name: "ref_unl_fail",
                    description: Some(
                        "Refresh unlock fail Write one to clear the bit.",
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
                    name: "ctl_vio",
                    description: Some(
                        "Violate register update protection mechanism Write one to clear the bit.",
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
                    name: "ctl_unl_fail",
                    description: Some(
                        "Unlock ctrl reg update protection fail Write one to clear the bit.",
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
                    name: "ot_rst",
                    description: Some(
                        "Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset.",
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
                    name: "parity_error",
                    description: Some(
                        "parity error Write one to clear the bit.",
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
            ],
        },
    ],
    enums: &[],
};
