use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpu",
            extends: None,
            description: Some(
                "GPU.",
            ),
            items: &[
                BlockItem {
                    name: "aqhi_clock_control",
                    description: Some(
                        "clock control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqhiClockControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqhildle",
                    description: Some(
                        "idle status register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aqhildle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqintr_acknowledge",
                    description: Some(
                        "interrupt acknoledge register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqintrAcknowledge",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqintr_enbl",
                    description: Some(
                        "interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqintrEnbl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcchip_rev",
                    description: Some(
                        "chip revison register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcchipRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcchip_date",
                    description: Some(
                        "chip date register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcchipDate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcreg_hichip_patch_rev",
                    description: Some(
                        "chip patch revision register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcregHichipPatchRev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc_product_id",
                    description: Some(
                        "product identification register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcProductId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc_module_power_controls",
                    description: Some(
                        "module power control register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcModulePowerControls",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc_module_power_module_control",
                    description: Some(
                        "module power module control register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcModulePowerModuleControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gc_module_power_module_status",
                    description: Some(
                        "module power module status register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcModulePowerModuleStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqmemory_fe_page_table",
                    description: Some(
                        "fetch engine page table base address register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqmemoryFePageTable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqmemory_debug",
                    description: Some(
                        "memory debug register.",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqmemoryDebug",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aqregister_timing_control",
                    description: Some(
                        "timing control register.",
                    ),
                    array: None,
                    byte_offset: 0x42c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AqregisterTimingControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcreg_fetch_address",
                    description: Some(
                        "fetch command buffer base address register.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcregFetchAddress",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcreg_fetch_control",
                    description: Some(
                        "fetch control register.",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcregFetchControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcreg_current_fetch_address",
                    description: Some(
                        "current fetch command address register.",
                    ),
                    array: None,
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GcregCurrentFetchAddress",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AqhiClockControl",
            extends: None,
            description: Some(
                "clock control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clk2d_dis",
                    description: Some(
                        "disable 2D/VG clock.",
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
                    name: "fscale_val",
                    description: Some(
                        "core clock frequency scale value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fscale_cmd_load",
                    description: Some(
                        "core clock frequency scale value enable.",
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
                    name: "disable_ram_clock_gating",
                    description: Some(
                        "disables clock gating for rams.",
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
                    name: "disable_debug_registers",
                    description: Some(
                        "disable debug registers.",
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
                    name: "soft_reset",
                    description: Some(
                        "soft reset the IP.",
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
                    name: "disable_ram_power_optimization",
                    description: Some(
                        "disables ram power optimization.",
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
                    name: "idle3_d",
                    description: Some(
                        "3D pipe is idle or not present.",
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
                    name: "idle2_d",
                    description: Some(
                        "2D pipe is idle or not present.",
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
                    name: "idle_vg",
                    description: Some(
                        "vg pipe is idle.",
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
                    name: "isolate_gpu",
                    description: Some(
                        "isolate GPU bit, used for power on/off.",
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
            ],
        },
        FieldSet {
            name: "Aqhildle",
            extends: None,
            description: Some(
                "idle status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idle_fe",
                    description: Some(
                        "0: fetch engine is busy 1:fetch engine is idle.",
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
                    name: "idle_de",
                    description: Some(
                        "DE is dile or not present.",
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
                    name: "idle_pe",
                    description: Some(
                        "Pixel engine is idle.",
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
                    name: "idle_sh",
                    description: Some(
                        "SH is idle or not present.",
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
                    name: "idle_pa",
                    description: Some(
                        "PA is idle or not present.",
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
                    name: "idle_se",
                    description: Some(
                        "SE is idle or not present.",
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
                    name: "idle_ra",
                    description: Some(
                        "RA is idle or not present.",
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
                    name: "idle_tx",
                    description: Some(
                        "TX is idle or not present.",
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
                    name: "idle_vg",
                    description: Some(
                        "Vector Graphics Engine is idle.",
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
                    name: "idle_im",
                    description: Some(
                        "Image Engine is idle.",
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
                    name: "idle_fp",
                    description: Some(
                        "FP is idle or not present.",
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
                    name: "idle_ts",
                    description: Some(
                        "Tessellation Engine is idle.",
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
                    name: "idle_blt",
                    description: Some(
                        "BLT is idle or not present.",
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
                    name: "axi_lp",
                    description: Some(
                        "axi is in low power mode.",
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
            name: "AqintrAcknowledge",
            extends: None,
            description: Some(
                "interrupt acknoledge register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intr_vec",
                    description: Some(
                        "for each interrupt event, 0=clear,1=interrupt active.",
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
            name: "AqintrEnbl",
            extends: None,
            description: Some(
                "interrupt enable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intr_enbl_vec",
                    description: Some(
                        "0=disable interrupt; 1=enable interrupt.",
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
            name: "AqmemoryDebug",
            extends: None,
            description: Some(
                "memory debug register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_outstanding_reads",
                    description: Some(
                        "limits the total number of outstanding read requests.",
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
                    name: "zcomp_limit",
                    description: Some(
                        "not relevant for vector graphics IP.",
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
            name: "AqmemoryFePageTable",
            extends: None,
            description: Some(
                "fetch engine page table base address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "base_address",
                    description: Some(
                        "base address for the FE virtual address lookup table.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AqregisterTimingControl",
            extends: None,
            description: Some(
                "timing control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "for_rf1p",
                    description: Some(
                        "for 1 port ram.",
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
                    name: "for_rf2p",
                    description: Some(
                        "for 2 port ram.",
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
                    name: "fast_rtc",
                    description: Some(
                        "RTC for fast rams.",
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
                    name: "fast_wtc",
                    description: Some(
                        "WTC for fast rams.",
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
                    name: "power_down",
                    description: Some(
                        "powerdown memory.",
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
            ],
        },
        FieldSet {
            name: "GcModulePowerControls",
            extends: None,
            description: Some(
                "module power control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "enable_module_clock_gating",
                    description: Some(
                        "enable module level clock gating.",
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
                    name: "disable_stall_module_clock_gating",
                    description: Some(
                        "disable module level clock gating for stall condition.",
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
                    name: "disable_starve_module_clock_gating",
                    description: Some(
                        "disable module level clock gating for starve/idle condition.",
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
                    name: "turn_on_counter",
                    description: Some(
                        "number of clock cycle gating the module if the modules is idle for this amout of clockk cycles.",
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
                    name: "turn_off_counter",
                    description: Some(
                        "counter value for clock gating the module if the module is idle for this amout of clock cycles.",
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
            name: "GcModulePowerModuleControl",
            extends: None,
            description: Some(
                "module power module control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "disable_module_clock_gating_fe",
                    description: Some(
                        "disables module level clock gating for FE.",
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
                    name: "disable_module_clock_gating_pe",
                    description: Some(
                        "disables module level clock gating for PE.",
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
                    name: "disable_module_clock_gating_vg",
                    description: Some(
                        "disables module lelvel clock gating for VG.",
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
                    name: "disable_module_clock_gating_im",
                    description: Some(
                        "disables module level clock gating for IM.",
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
                    name: "disable_module_clock_gating_ts",
                    description: Some(
                        "disables module level clock gating for TS.",
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
                    name: "disable_module_clockgating_flexa",
                    description: Some(
                        "disables module level clock gating for flexa, not supported for all variants.",
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
            name: "GcModulePowerModuleStatus",
            extends: None,
            description: Some(
                "module power module status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "module_clock_gated_fe",
                    description: Some(
                        "module level clock gating is on for FE.",
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
                    name: "module_clock_gated_pe",
                    description: Some(
                        "module level clock gating is on for PE.",
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
                    name: "module_clock_gated_vg",
                    description: Some(
                        "module level clock gating is on for VG.",
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
                    name: "module_clock_gated_im",
                    description: Some(
                        "module level clock gating is on for IM.",
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
                    name: "module_clock_gated_ts",
                    description: Some(
                        "module level ckock gating is on for ts.",
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
                    name: "module_clock_gated_flexa",
                    description: Some(
                        "module level ckock gating is on for flexa.",
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
            name: "GcProductId",
            extends: None,
            description: Some(
                "product identification register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "grade_level",
                    description: Some(
                        "0:None_no extra letter on the product name for this core 1:nano 5:nano ultra.",
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
                    name: "num",
                    description: Some(
                        "product number is 265.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "type_",
                    description: Some(
                        "product type is 3:VG.",
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
            name: "GcchipDate",
            extends: None,
            description: Some(
                "chip date register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "date",
                    description: Some(
                        "date.",
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
            name: "GcchipRev",
            extends: None,
            description: Some(
                "chip revison register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rev",
                    description: Some(
                        "revision.",
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
            name: "GcregCurrentFetchAddress",
            extends: None,
            description: Some(
                "current fetch command address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "address.",
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
            name: "GcregFetchAddress",
            extends: None,
            description: Some(
                "fetch command buffer base address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "type_",
                    description: Some(
                        "0=system 2=vritual 1=local.",
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
                    name: "address",
                    description: Some(
                        "address of command buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GcregFetchControl",
            extends: None,
            description: Some(
                "fetch control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "count",
                    description: Some(
                        "number of 64bit words to fetch.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GcregHichipPatchRev",
            extends: None,
            description: Some(
                "chip patch revision register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "patch_rev",
                    description: Some(
                        "patch revision.",
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
    ],
    enums: &[],
};
