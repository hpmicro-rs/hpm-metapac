use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "MipiCsi",
            extends: None,
            description: Some(
                "MIPI_CSI0.",
            ),
            items: &[
                BlockItem {
                    name: "version",
                    description: Some(
                        "version code.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Version",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "n_lanes",
                    description: Some(
                        "the number of active lanes.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "NLanes",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csi2_resetn",
                    description: Some(
                        "the internal logic of the controller goes into the reset state when active.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csi2Resetn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_main",
                    description: Some(
                        "contains the stateus of individual interrupt sources.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStMain",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data_ids_1",
                    description: Some(
                        "programs data type fields for data ID monitors.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DataIds1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data_ids_2",
                    description: Some(
                        "programs data type fields for data ID monitors.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DataIds2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_ap_main",
                    description: Some(
                        "contains the status of individual interrupt sources.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStApMain",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_shutdownz",
                    description: Some(
                        "controls the phy shutdown mode.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyShutdownz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dphy_rstz",
                    description: Some(
                        "controls the phy reset mode.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DphyRstz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_rx",
                    description: Some(
                        "contains the status of rx-related signals from phy.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyRx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_stopstate",
                    description: Some(
                        "contains the stopstate signal status from phy.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyStopstate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_mode",
                    description: Some(
                        "selects how the ipi interface generates the video frame.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_vcid",
                    description: Some(
                        "selects the vritual channel processed by ipi.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiVcid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_data_type",
                    description: Some(
                        "selects the data type processed by ipi.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiDataType",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_mem_flash",
                    description: Some(
                        "control the flush of ipi memory.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiMemFlash",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_hsa_time",
                    description: Some(
                        "configures the video horizontal synchronism active time.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiHsaTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_hbp_time",
                    description: Some(
                        "configures the video horizontal synchronism back porch time.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiHbpTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_hsd_time",
                    description: Some(
                        "configures the vedeo Horizontal Sync Delay time.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiHsdTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_hline_time",
                    description: Some(
                        "configures the overall tiem for each video line.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiHlineTime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_softrstn",
                    description: Some(
                        "congtrols the ipi logic reset state.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiSoftrstn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_adv_features",
                    description: Some(
                        "configures advanced features for ipi mode.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiAdvFeatures",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_vsa_lines",
                    description: Some(
                        "configures the vertical synchronism active period.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiVsaLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_vbp_lines",
                    description: Some(
                        "configures the verticall back porch period.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiVbpLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_vfp_lines",
                    description: Some(
                        "configures the vertical front porch period.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiVfpLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipi_vactive_lines",
                    description: Some(
                        "configures the vertical resolution of video.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpiVactiveLines",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vc_extension",
                    description: Some(
                        "active extra bits for virtual channel.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "VcExtension",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cal",
                    description: Some(
                        "contains the calibration signal status from synopsys d-phy.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_phy_fatal",
                    description: Some(
                        "groups the phy interruptions caused by phy packets discarded.",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStPhyFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_phy_fatal",
                    description: Some(
                        "interrupt mask for int_st_phy_fatal.",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskPhyFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_phy_fatal",
                    description: Some(
                        "interrupt force register for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForcePhyFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_pkt_fatal",
                    description: Some(
                        "groups the fatal interruption related with packet construction.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStPktFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_pkt_fatal",
                    description: Some(
                        "interrupt mask for int_st_pkt_fatal.",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskPktFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_pkt_fatal",
                    description: Some(
                        "interrupt force register is used for test purpos.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForcePktFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_phy",
                    description: Some(
                        "interruption caused by phy.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStPhy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_phy",
                    description: Some(
                        "interrupt mask for int_st_phy.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskPhy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_phy",
                    description: Some(
                        "interrupt force register.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForcePhy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_ipi_fatal",
                    description: Some(
                        "fatal interruption caused by ipi interface.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_ipi_fatal",
                    description: Some(
                        "interrupt mask for int_st_ipi_fatal.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_ipi_fatal",
                    description: Some(
                        "interrupt force register.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_ap_generic",
                    description: Some(
                        "groups and notifies which interruption bits caused the interruption.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStApGeneric",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_ap_generic",
                    description: Some(
                        "interrupt mask for int_st_ap_generic.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskApGeneric",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_ap_generic",
                    description: Some(
                        "interrupt force register used for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceApGeneric",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_ap_ipi_fatal",
                    description: Some(
                        "groups and notifies which interruption bits.",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStApIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_ap_ipi_fatal",
                    description: Some(
                        "interrupt mask for int_st_ap_ipi_fatal controls.",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskApIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_ap_ipi_fatal",
                    description: Some(
                        "interrupt force register.",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceApIpiFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_bndry_frame_fatal",
                    description: Some(
                        "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStBndryFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_bndry_frame_fatal",
                    description: Some(
                        "interrupt mask for int_st_bndry_frame_fatal.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskBndryFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_bndry_frame_fatal",
                    description: Some(
                        "interrupt force register is used for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceBndryFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_seq_frame_fatal",
                    description: Some(
                        "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStSeqFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_seq_frame_fatal",
                    description: Some(
                        "interrupt mask for int_st_seq_frame_fatal.",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskSeqFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_seq_frame_fatal",
                    description: Some(
                        "interrupt force register is used for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceSeqFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_crc_frame_fatal",
                    description: Some(
                        "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStCrcFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_crc_frame_fatal",
                    description: Some(
                        "interrupt mask for int_st_crc_frame_fatal.",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskCrcFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_crc_frame_fatal",
                    description: Some(
                        "interrupt force register is used for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForceCrcFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st_pld_crc_frame_fatal",
                    description: Some(
                        "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
                    ),
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStPldCrcFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_msk_pld_crc_frame_fatal",
                    description: Some(
                        "interrupt mask for int_st_crc_frame_fatal.",
                    ),
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntMskPldCrcFrameFatal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_force_pld_crc_frame_fatal",
                    description: Some(
                        "interrupt force register is used for test purposes.",
                    ),
                    array: None,
                    byte_offset: 0x2b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntForcePldCrcFrameFatal",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Csi2Resetn",
            extends: None,
            description: Some(
                "the internal logic of the controller goes into the reset state when active.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csi2_resetn",
                    description: Some(
                        "DWC_mipi_csi2_host reset output, active low.",
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
            name: "DataIds1",
            extends: None,
            description: Some(
                "programs data type fields for data ID monitors.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "di0_dt",
                    description: Some(
                        "data type for programmed data ID 0.",
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
                    name: "di1_dt",
                    description: Some(
                        "data type for programmed data ID 1.",
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
                    name: "di2_dt",
                    description: Some(
                        "data type for programmed data ID 2.",
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
                    name: "di3_dt",
                    description: Some(
                        "data type for programmed data ID 3.",
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
            name: "DataIds2",
            extends: None,
            description: Some(
                "programs data type fields for data ID monitors.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "di4_dt",
                    description: Some(
                        "data type for programmed data ID 4.",
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
                    name: "di5_dt",
                    description: Some(
                        "data type for programmed data ID 5.",
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
                    name: "di6_dt",
                    description: Some(
                        "data type for programmed data ID 6.",
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
                    name: "di7_dt",
                    description: Some(
                        "data type for programmed data ID 7.",
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
            name: "DphyRstz",
            extends: None,
            description: Some(
                "controls the phy reset mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dphy_rstz",
                    description: Some(
                        "phy reset output, active low.",
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
            name: "IntForceApGeneric",
            extends: None,
            description: Some(
                "interrupt force register used for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_apb_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_reg_bank_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_de_skew_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_pipeline_delay_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_descrambler_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_phy_adapter_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_packet_analyzer_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_prep_outs_ap_err",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "force_err_msgr_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_err_handle_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_synchronizer_fpclk_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_synchronizer_rxbyteclkhs_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_synchronizer_pixclk_ap_err",
                    description: Some(
                        "No description available.",
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
            name: "IntForceApIpiFatal",
            extends: None,
            description: Some(
                "interrupt force register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_parity_tx_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_parity_rx_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_ecc_single_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_ecc_multiple_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_crc_err",
                    description: Some(
                        "No description available.",
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
                    name: "force_redundancy_err",
                    description: Some(
                        "No description available.",
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
            name: "IntForceBndryFrameFatal",
            extends: None,
            description: Some(
                "interrupt force register is used for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_err_f_bndry_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "force_err_f_bndry_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "force_err_f_bndry_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "force_err_f_bndry_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "force_err_f_bndry_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "force_err_f_bndry_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "force_err_f_bndry_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "force_err_f_bndry_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "force_err_f_bndry_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "force_err_f_bndry_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "force_err_f_bndry_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "force_err_f_bndry_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "force_err_f_bndry_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "force_err_f_bndry_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "force_err_f_bndry_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "force_err_f_bndry_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntForceCrcFrameFatal",
            extends: None,
            description: Some(
                "interrupt force register is used for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_err_f_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "force_err_f_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "force_err_f_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "force_err_f_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "force_err_f_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "force_err_f_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "force_err_f_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "force_err_f_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "force_err_f_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "force_err_f_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "force_err_f_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "force_err_f_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "force_err_f_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "force_err_f_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "force_err_f_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "force_err_f_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntForceIpiFatal",
            extends: None,
            description: Some(
                "interrupt force register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_pixel_if_fifo_underflow",
                    description: Some(
                        "force for pixel_if_fifo_underflow.",
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
                    name: "force_pixel_if_fifo_overflow",
                    description: Some(
                        "force for pixel_if_fifo_overflow.",
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
                    name: "force_frame_sync_err",
                    description: Some(
                        "force for frame_sync_err.",
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
                    name: "force_pixel_if_fifo_nempty_fs",
                    description: Some(
                        "force pixel_if_fifo_nempty_fs.",
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
                    name: "force_pixel_if_hline_err",
                    description: Some(
                        "force pixel_if_hline_err.",
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
                    name: "force_int_event_fifo_overflow",
                    description: Some(
                        "force int_event_fifo_overflow.",
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
            name: "IntForcePhy",
            extends: None,
            description: Some(
                "interrupt force register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_phy_errsoths_0",
                    description: Some(
                        "force phy_errsoths_0.",
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
                    name: "force_phy_errsoths_1",
                    description: Some(
                        "force phy_errsoths_1.",
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
                    name: "force_phy_erresc_0",
                    description: Some(
                        "force phy_erresc_0.",
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
                    name: "force_phy_erresc_1",
                    description: Some(
                        "force phy_erresc_1.",
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
            name: "IntForcePhyFatal",
            extends: None,
            description: Some(
                "interrupt force register for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_phy_errsotsynchs_0",
                    description: Some(
                        "force phy_errsotsynchs_0.",
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
                    name: "force_phy_errsotsynchs_1",
                    description: Some(
                        "force phy_errsotsynchs_1.",
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
                    name: "err_deskew",
                    description: Some(
                        "force err_deskew.",
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
            ],
        },
        FieldSet {
            name: "IntForcePktFatal",
            extends: None,
            description: Some(
                "interrupt force register is used for test purpos.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_err_ecc_double",
                    description: Some(
                        "force err_ecc_double.",
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
            name: "IntForcePldCrcFrameFatal",
            extends: None,
            description: Some(
                "interrupt force register is used for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_err_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "force_err_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "force_err_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "force_err_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "force_err_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "force_err_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "force_err_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "force_err_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "force_err_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "force_err_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "force_err_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "force_err_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "force_err_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "force_err_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "force_err_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "force_err_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntForceSeqFrameFatal",
            extends: None,
            description: Some(
                "interrupt force register is used for test purposes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "force_err_f_seq_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "force_err_f_seq_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "force_err_f_seq_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "force_err_f_seq_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "force_err_f_seq_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "force_err_f_seq_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "force_err_f_seq_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "force_err_f_seq_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "force_err_f_seq_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "force_err_f_seq_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "force_err_f_seq_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "force_err_f_seq_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "force_err_f_seq_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "force_err_f_seq_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "force_err_f_seq_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "force_err_f_seq_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntMskApGeneric",
            extends: None,
            description: Some(
                "interrupt mask for int_st_ap_generic.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_apb_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_reg_bank_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_de_skew_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_pipeline_delay_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_descrambler_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_phy_adapter_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_packet_analyzer_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_prep_outs_ap_err",
                    description: Some(
                        "No description available.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msk_err_msgr_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_err_handle_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_synchronizer_fpclk_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_synchronizer_rxbyteclkhs_ap_err",
                    description: Some(
                        "No description available.",
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
                    name: "msk_synchronizer_pixclk_ap_err",
                    description: Some(
                        "No description available.",
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
            name: "IntMskApIpiFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_ap_ipi_fatal controls.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_parity_tx_err",
                    description: Some(
                        "No description available.",
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
                    name: "mask_parity_rx_err",
                    description: Some(
                        "No description available.",
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
                    name: "mask_ecc_single_err",
                    description: Some(
                        "No description available.",
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
                    name: "mask_ecc_multiple_err",
                    description: Some(
                        "No description available.",
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
                    name: "mask_crc_err",
                    description: Some(
                        "No description available.",
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
                    name: "mask_redundancy_err",
                    description: Some(
                        "No description available.",
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
            name: "IntMskBndryFrameFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_bndry_frame_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_err_f_bndry_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "msk_err_f_bndry_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "msk_err_f_bndry_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "msk_err_f_bndry_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "msk_err_f_bndry_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "msk_err_f_bndry_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "msk_err_f_bndry_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "msk_err_f_bndry_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "msk_err_f_bndry_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "msk_err_f_bndry_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "msk_err_f_bndry_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "msk_err_f_bndry_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "msk_err_f_bndry_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "msk_err_f_bndry_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "msk_err_f_bndry_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "msk_err_f_bndry_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntMskCrcFrameFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_crc_frame_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_err_f_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "msk_err_f_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "msk_err_f_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "msk_err_f_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "msk_err_f_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "msk_err_f_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "msk_err_f_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "msk_err_f_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "msk_err_f_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "msk_err_f_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "msk_err_f_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "msk_err_f_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "msk_err_f_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "msk_err_f_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "msk_err_f_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "msk_err_f_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntMskIpiFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_ipi_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_pixel_if_fifo_underflow",
                    description: Some(
                        "mask for pixel_if_fifo_unterflow.",
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
                    name: "msk_pixel_if_fifo_overflow",
                    description: Some(
                        "mask for pixel_if_fifo_overflow.",
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
                    name: "msk_frame_sync_err",
                    description: Some(
                        "mask for pixel_if_frame_sync_err.",
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
                    name: "msk_pixel_if_fifo_nempty_fs",
                    description: Some(
                        "mask pixel_if_fifo_nempty_fs.",
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
                    name: "msk_pixel_if_hline_err",
                    description: Some(
                        "mask pixel_if_hline_err.",
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
                    name: "msk_int_event_fifo_overflow",
                    description: Some(
                        "mask int_event_fifo_overflow.",
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
            name: "IntMskPhy",
            extends: None,
            description: Some(
                "interrupt mask for int_st_phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_phy_errsoths_0",
                    description: Some(
                        "mask for phy_errsoths_0.",
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
                    name: "mask_phy_errsoths_1",
                    description: Some(
                        "mask for phy_errsoths_1.",
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
                    name: "mask_phy_erresc_0",
                    description: Some(
                        "mask for phy_erresc_0.",
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
                    name: "mask_phy_erresc_1",
                    description: Some(
                        "mask for phy_erresc_1.",
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
            name: "IntMskPhyFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_phy_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_phy_errsotsynchs_0",
                    description: Some(
                        "mask for phy_errsotsynchs_0.",
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
                    name: "mask_phy_errsotsynchs_1",
                    description: Some(
                        "mask for phy_errsotsynchs_1.",
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
                    name: "err_deskew",
                    description: Some(
                        "mask for err_deskew.",
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
            ],
        },
        FieldSet {
            name: "IntMskPktFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_pkt_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mask_err_ecc_double",
                    description: Some(
                        "mask for err_ecc_double.",
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
            name: "IntMskPldCrcFrameFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_crc_frame_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_err_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "msk_err_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "msk_err_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "msk_err_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "msk_err_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "msk_err_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "msk_err_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "msk_err_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "msk_err_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "msk_err_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "msk_err_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "msk_err_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "msk_err_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "msk_err_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "msk_err_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "msk_err_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntMskSeqFrameFatal",
            extends: None,
            description: Some(
                "interrupt mask for int_st_seq_frame_fatal.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msk_err_f_seq_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "msk_err_f_seq_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "msk_err_f_seq_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "msk_err_f_seq_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "msk_err_f_seq_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "msk_err_f_seq_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "msk_err_f_seq_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "msk_err_f_seq_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "msk_err_f_seq_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "msk_err_f_seq_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "msk_err_f_seq_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "msk_err_f_seq_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "msk_err_f_seq_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "msk_err_f_seq_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "msk_err_f_seq_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "msk_err_f_seq_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntStApGeneric",
            extends: None,
            description: Some(
                "groups and notifies which interruption bits caused the interruption.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "apb_ap_err",
                    description: Some(
                        "ap error in apb block.",
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
                    name: "reg_bank_ap_err",
                    description: Some(
                        "ap error in register bank block.",
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
                    name: "de_skew_ap_err",
                    description: Some(
                        "ap error in de-skew block.",
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
                    name: "pipeline_delay_ap_err",
                    description: Some(
                        "ap error in pipeline delay block.",
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
                    name: "descrambler_ap_err",
                    description: Some(
                        "ap error in descrambler block.",
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
                    name: "phy_adapter_ap_err",
                    description: Some(
                        "ap error in phy adapter block.",
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
                    name: "packet_analyzer_ap_err",
                    description: Some(
                        "ap error in packet analyzer block.",
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
                    name: "prep_outs_ap_err",
                    description: Some(
                        "ap error in prepare outs block.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "err_msgr_ap_err",
                    description: Some(
                        "ap error in err msgr block.",
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
                    name: "err_handle_ap_err",
                    description: Some(
                        "ap error in error handler block.",
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
                    name: "synchronizer_fpclk_ap_err",
                    description: Some(
                        "ap error in synchronizer block for fpclk domain.",
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
                    name: "synchronizer_rxbyteclkhs_ap_err",
                    description: Some(
                        "ap error in synchronizer block for rxbyteclkhs domain.",
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
                    name: "synchronizer_pixclk_ap_err",
                    description: Some(
                        "ap error in synchronizer block for pixclk domain.",
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
            name: "IntStApIpiFatal",
            extends: None,
            description: Some(
                "groups and notifies which interruption bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "parity_tx_err",
                    description: Some(
                        "ap parity tx error in ipi1.",
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
                    name: "parity_rx_err",
                    description: Some(
                        "ap parity rx error in ipi1.",
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
                    name: "ecc_single_err",
                    description: Some(
                        "ap ecc sigle error in ipi1.",
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
                    name: "ecc_multiple_err",
                    description: Some(
                        "ap ecc multiple error in ipi1.",
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
                    name: "crc_err",
                    description: Some(
                        "ap crc error in ipi1.",
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
                    name: "redundancy_err",
                    description: Some(
                        "ap redundancy error in ipi1.",
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
            name: "IntStApMain",
            extends: None,
            description: Some(
                "contains the status of individual interrupt sources.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status_int_st_ap_generic",
                    description: Some(
                        "status of int_st_ap_generic.",
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
                    name: "status_int_phy_fatal",
                    description: Some(
                        "status of int_st_phy_fatal.",
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
                    name: "status_int_pkt_fatal",
                    description: Some(
                        "status of int_st_pkt_fatal.",
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
                    name: "status_int_bndry_frame_fatal",
                    description: Some(
                        "status of int_st_bndry_frame_fatal.",
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
                    name: "status_int_seq_frame_fatal",
                    description: Some(
                        "status of status_int_seq_frame_fatal.",
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
                    name: "status_int_crc_frame_fatal",
                    description: Some(
                        "status of status_int_crc_frame_fatal.",
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
                    name: "status_int_phy",
                    description: Some(
                        "status of int_st_phy.",
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
                    name: "status_int_pld_crc_fatal",
                    description: Some(
                        "status of status_int_pld_crc_fatal.",
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
                    name: "status_int_data_id",
                    description: Some(
                        "status of status_int_data_id.",
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
                    name: "status_int_ecc_corrected",
                    description: Some(
                        "status of status_int_ecc_corrected.",
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
                    name: "status_int_line",
                    description: Some(
                        "status of int_st_line.",
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
                    name: "status_int_st_ap_ipi_fatal",
                    description: Some(
                        "status of int_st_ap_ipi_fatal.",
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
                    name: "status_int_ipi_fatal",
                    description: Some(
                        "status of int_st_ipi_fatal.",
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
            name: "IntStBndryFrameFatal",
            extends: None,
            description: Some(
                "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_f_bndry_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "err_f_bndry_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "err_f_bndry_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "err_f_bndry_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "err_f_bndry_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "err_f_bndry_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "err_f_bndry_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "err_f_bndry_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "err_f_bndry_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "err_f_bndry_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "err_f_bndry_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "err_f_bndry_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "err_f_bndry_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "err_f_bndry_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "err_f_bndry_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "err_f_bndry_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntStCrcFrameFatal",
            extends: None,
            description: Some(
                "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_f_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "err_f_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "err_f_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "err_f_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "err_f_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "err_f_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "err_f_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "err_f_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "err_f_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "err_f_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "err_f_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "err_f_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "err_f_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "err_f_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "err_f_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "err_f_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntStIpiFatal",
            extends: None,
            description: Some(
                "fatal interruption caused by ipi interface.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pixel_if_fifo_underflow",
                    description: Some(
                        "the fifo has become empty before the expected bumber of pixels could be extracted to the pixel intefcese.",
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
                    name: "pixel_if_fifo_overflow",
                    description: Some(
                        "the fifo of pixel interface has lost information because some data arrived and fifo is already full.",
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
                    name: "pixel_if_frame_sync_err",
                    description: Some(
                        "whenever in controller mode, notifies if a new frame is received but previous has not been completed.",
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
                    name: "pixel_if_fifo_nempty_fs",
                    description: Some(
                        "the fifo of pixel interface is not empty at the starat of a new frame.",
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
                    name: "pixel_if_hline_err",
                    description: Some(
                        "horizontal line time error.",
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
                    name: "int_event_fifo_overflow",
                    description: Some(
                        "reporting internal fifo overflow.",
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
            name: "IntStMain",
            extends: None,
            description: Some(
                "contains the stateus of individual interrupt sources.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "status_int_phy_fatal",
                    description: Some(
                        "status of int_st_phy_fatal.",
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
                    name: "status_int_pkt_fatal",
                    description: Some(
                        "status of int_st_pkt_fatal.",
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
                    name: "status_int_bndry_frame_fatal",
                    description: Some(
                        "status of int_st_bndry_frame_fatal.",
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
                    name: "status_int_seq_frame_fatal",
                    description: Some(
                        "status of status_int_seq_frame_fatal.",
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
                    name: "status_int_crc_frame_fatal",
                    description: Some(
                        "status of status_int_crc_frame_fatal.",
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
                    name: "status_int_pld_crc_fatal",
                    description: Some(
                        "status of status_int_pld_crc_fatal.",
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
                    name: "status_int_data_id",
                    description: Some(
                        "status of status_int_data_id.",
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
                    name: "status_int_ecc_corrected",
                    description: Some(
                        "status of status_int_ecc_corrected.",
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
                    name: "status_int_phy",
                    description: Some(
                        "status of int_st_phy.",
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
                    name: "status_int_line",
                    description: Some(
                        "status of int_st_line.",
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
                    name: "status_int_ipi4_fatal",
                    description: Some(
                        "status of int_st_ipi_fatal.",
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
            ],
        },
        FieldSet {
            name: "IntStPhy",
            extends: None,
            description: Some(
                "interruption caused by phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_errsoths_0",
                    description: Some(
                        "start of transmission error on data lane 0.",
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
                    name: "phy_errsoths_1",
                    description: Some(
                        "start of transmission error on data lane 1.",
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
                    name: "phy_erresc_0",
                    description: Some(
                        "start of transmission error on data lane 0.",
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
                    name: "phy_erresc_1",
                    description: Some(
                        "start of transmission error on data lane 1.",
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
            name: "IntStPhyFatal",
            extends: None,
            description: Some(
                "groups the phy interruptions caused by phy packets discarded.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_errsotsynchs_0",
                    description: Some(
                        "start of transmission error on data lane0.",
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
                    name: "phy_errsotsynchs_1",
                    description: Some(
                        "start of transmission error on data lane1.",
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
                    name: "err_deskew",
                    description: Some(
                        "reports whenever data is lost due to an existent skew between lanes greater than 2 rxwordclkhs.",
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
            ],
        },
        FieldSet {
            name: "IntStPktFatal",
            extends: None,
            description: Some(
                "groups the fatal interruption related with packet construction.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_ecc_double",
                    description: Some(
                        "header ecc contains at least 2 errors.",
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
            name: "IntStPldCrcFrameFatal",
            extends: None,
            description: Some(
                "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_crc_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "err_crc_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "err_crc_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "err_crc_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "err_crc_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "err_crc_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "err_crc_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "err_crc_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "err_crc_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "err_crc_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "err_crc_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "err_crc_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "err_crc_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "err_crc_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "err_crc_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "err_crc_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IntStSeqFrameFatal",
            extends: None,
            description: Some(
                "fatal interruption related with matching frame start with frame end for a specific virtual channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err_f_seq_match_vc0",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 0.",
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
                    name: "err_f_seq_match_vc1",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 1.",
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
                    name: "err_f_seq_match_vc2",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 2.",
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
                    name: "err_f_seq_match_vc3",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 3.",
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
                    name: "err_f_seq_match_vc4",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 4.",
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
                    name: "err_f_seq_match_vc5",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 5.",
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
                    name: "err_f_seq_match_vc6",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 6.",
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
                    name: "err_f_seq_match_vc7",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 7.",
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
                    name: "err_f_seq_match_vc8",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 8.",
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
                    name: "err_f_seq_match_vc9",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 9.",
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
                    name: "err_f_seq_match_vc10",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 10.",
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
                    name: "err_f_seq_match_vc11",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 11.",
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
                    name: "err_f_seq_match_vc12",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 12.",
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
                    name: "err_f_seq_match_vc13",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 13.",
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
                    name: "err_f_seq_match_vc14",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 14.",
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
                    name: "err_f_seq_match_vc15",
                    description: Some(
                        "error matching frame start with frame end for virtual channel 15.",
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
            ],
        },
        FieldSet {
            name: "IpiAdvFeatures",
            extends: None,
            description: Some(
                "configures advanced features for ipi mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_dt_overwrite",
                    description: Some(
                        "ignore datatype of the header using the programmed datatype for decoding.",
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
                    name: "ipi_dt",
                    description: Some(
                        "datatype to overwrite.",
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
                    name: "line_event_selection",
                    description: Some(
                        "for camero mode, allows manual selection of the packet fo line delimiter as follows: 0x0-controller seletc it automaticlly 0x1-select packets from list programmed in 17:21.",
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
                    name: "en_video",
                    description: Some(
                        "allows the use of video packets for ipi synchronization events.",
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
                    name: "en_line_start",
                    description: Some(
                        "allows the use of line start packets for ipi synchronization events.",
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
                    name: "en_null",
                    description: Some(
                        "allows the use of null packets for IPI synchronization events.",
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
                    name: "en_blanking",
                    description: Some(
                        "allows the use of blankong packets for IPI synchronization events.",
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
                    name: "en_embedded",
                    description: Some(
                        "allows the use of embendded packets for ipi synchronization events.",
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
                    name: "ipi_sync_event_mode",
                    description: Some(
                        "for camera mode: 0x0- frame start do not trigger any sync event.",
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
            ],
        },
        FieldSet {
            name: "IpiDataType",
            extends: None,
            description: Some(
                "selects the data type processed by ipi.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_data_type",
                    description: Some(
                        "data type of data to be processed by pixel interface.",
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
                    name: "embended_data",
                    description: Some(
                        "enable embedded data processing on ipi interface.",
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
            ],
        },
        FieldSet {
            name: "IpiHbpTime",
            extends: None,
            description: Some(
                "configures the video horizontal synchronism back porch time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_hbp_time",
                    description: Some(
                        "configures the Horizontal Synchronism back porch period in pixclk cycles.",
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
            ],
        },
        FieldSet {
            name: "IpiHlineTime",
            extends: None,
            description: Some(
                "configures the overall tiem for each video line.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_hlin_time",
                    description: Some(
                        "configures the size of the line time counted in pixclk cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IpiHsaTime",
            extends: None,
            description: Some(
                "configures the video horizontal synchronism active time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_hsa_time",
                    description: Some(
                        "configures the Horizontal Synchronism Active period in pixclk cycles.",
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
            ],
        },
        FieldSet {
            name: "IpiHsdTime",
            extends: None,
            description: Some(
                "configures the vedeo Horizontal Sync Delay time.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_hsd_time",
                    description: Some(
                        "configures the Horizontal Sync Porch delay period in pixclk cycles.",
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
            ],
        },
        FieldSet {
            name: "IpiMemFlash",
            extends: None,
            description: Some(
                "control the flush of ipi memory.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_flush",
                    description: Some(
                        "flush ipi memory, this bit is auto clear.",
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
                    name: "ipi_auto_flush",
                    description: Some(
                        "memory is automatically flashed at each vsync.",
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
            ],
        },
        FieldSet {
            name: "IpiMode",
            extends: None,
            description: Some(
                "selects how the ipi interface generates the video frame.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_mode",
                    description: Some(
                        "indicates the video mode transmission type 0x0: camera timing 0x1:controller timing.",
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
                    name: "ipi_color_com",
                    description: Some(
                        "if color mode components are deliverd as follows: 0x0 48bit intercase 0x1: 16bit interface.",
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
                    name: "ipi_cut_through",
                    description: Some(
                        "cut-through mode state active when high.",
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
                    name: "ipi_enable",
                    description: Some(
                        "enables the interface.",
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
            ],
        },
        FieldSet {
            name: "IpiSoftrstn",
            extends: None,
            description: Some(
                "congtrols the ipi logic reset state.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_softrstn",
                    description: Some(
                        "resets ipi one, active low.",
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
            name: "IpiVactiveLines",
            extends: None,
            description: Some(
                "configures the vertical resolution of video.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_vactive_lines",
                    description: Some(
                        "configures the vertical active period measured in bumber of horizontal lines.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IpiVbpLines",
            extends: None,
            description: Some(
                "configures the verticall back porch period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_vbp_lines",
                    description: Some(
                        "configuress the vertical back porch period measured in number of horizontal lines.",
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
            ],
        },
        FieldSet {
            name: "IpiVcid",
            extends: None,
            description: Some(
                "selects the vritual channel processed by ipi.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ip_vcid",
                    description: Some(
                        "virtual channel of data to be processed by pixel interface.",
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
                    name: "ipi_vcx_0_1",
                    description: Some(
                        "virtual channel extension of data to be processed by pixel interface.",
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
            ],
        },
        FieldSet {
            name: "IpiVfpLines",
            extends: None,
            description: Some(
                "configures the vertical front porch period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_vfp_lines",
                    description: Some(
                        "configures the vertical front porch period measured in number of horizontall lines.",
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
            ],
        },
        FieldSet {
            name: "IpiVsaLines",
            extends: None,
            description: Some(
                "configures the vertical synchronism active period.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ipi_vsa_lines",
                    description: Some(
                        "configures the vertical synchronism active period measured in number of horizontal lines.",
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
            ],
        },
        FieldSet {
            name: "NLanes",
            extends: None,
            description: Some(
                "the number of active lanes.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "n_lanes",
                    description: Some(
                        "number of active data lanes.",
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
            ],
        },
        FieldSet {
            name: "PhyCal",
            extends: None,
            description: Some(
                "contains the calibration signal status from synopsys d-phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxskewcalhs",
                    description: Some(
                        "a low-to-high transition on rxskewcalhs signal means the the phy has initiated the de-skew calibration.",
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
            name: "PhyRx",
            extends: None,
            description: Some(
                "contains the status of rx-related signals from phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_rxulpsesc_0",
                    description: Some(
                        "lane module 0 has entered the ultra low power mode.",
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
                    name: "phy_rxullpsesc_1",
                    description: Some(
                        "lane module 1 has entered the ultra low power mode.",
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
                    name: "phy_rxulpsclknot",
                    description: Some(
                        "active low. Indicates the d-phy clock lane module has entered the Ultra low power state.",
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
                    name: "phy_rxclkactivehs",
                    description: Some(
                        "indicates the d-phy clock lane is actively receiving a ddr clock.",
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
            name: "PhyShutdownz",
            extends: None,
            description: Some(
                "controls the phy shutdown mode.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_shutdownz",
                    description: Some(
                        "shutdown input,active low.",
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
            name: "PhyStopstate",
            extends: None,
            description: Some(
                "contains the stopstate signal status from phy.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "phy_stopstatedata_0",
                    description: Some(
                        "data lane 0 in stop state.",
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
                    name: "phy_stopstatedata_1",
                    description: Some(
                        "data lane 1 in stop state.",
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
                    name: "phy_stopstateclk",
                    description: Some(
                        "d-phy clock lane in stop state.",
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
            name: "VcExtension",
            extends: None,
            description: Some(
                "active extra bits for virtual channel.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcx",
                    description: Some(
                        "indicates status of virtual channel extension: 0-virtual channel extension is enable 1-legacy mode.",
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
            name: "Version",
            extends: None,
            description: Some(
                "version code.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "version code.",
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
