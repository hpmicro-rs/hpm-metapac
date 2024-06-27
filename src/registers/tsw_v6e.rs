use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Bin",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "txdata",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 60,
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
                                "Txdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_txbuf_bin0_tque_and_tx_len",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynTxbufBin0TqueAndTxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_txbuf_bin0_tx_timestamp_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynTxbufBin0TxTimestampL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_txbuf_bin0_tx_timestamp_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynTxbufBin0TxTimestampH",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Mac",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "mac_ver",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacVer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macaddr_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacaddrL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_macaddr_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacaddrH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_mac_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMacCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_tx_frames",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacTxFrames",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_rx_frames",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacRxFrames",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_tx_octets",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacTxOctets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_rx_octets",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacRxOctets",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_mdio_cfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMdioCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_mdio_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMdioCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_mdio_rd_data",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMdioRdData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_mdio_wr_data",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacMdioWrData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_irq_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacIrqCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Rxfifo",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_fdmem_cnt_byte",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEFdmemCntByte",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_fdmem_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEFdmemSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_error_flag",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEErrorFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_ie_error_flag",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEIeErrorFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_in_config",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEInConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_out_config",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEOutConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_reset",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_param",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_strfwd",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEStrfwd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_portmask",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEPortmask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_mirror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEMirror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_igress_rx_fdfifo_e_mirror_tx",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlIgressRxFdfifoEMirrorTx",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Shacl",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "tsn_shaper_aclist_entry0_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperAclistEntry0L",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_aclist_entry0_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperAclistEntry0H",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tsnport",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "mac",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 512,
                            },
                        ),
                    ),
                    byte_offset: 0x0,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Mac",
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_cr",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_sr",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ct_curtime_ns",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x810,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcCtCurtimeNs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ct_curtime_sec",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x814,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcCtCurtimeSec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ct_timer_incr",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x81c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcCtTimerIncr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ofs_ns",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x820,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcOfsNs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ofs_sl",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x824,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcOfsSl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ofs_sh",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x828,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcOfsSh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_ofs_ch",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x82c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcOfsCh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_alarm_ns",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x830,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcAlarmNs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_alarm_sl",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x834,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcAlarmSl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_alarm_sh",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x838,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcAlarmSh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtc_timer_a_period",
                    description: Some(
                        "ONLY IN PORT1.",
                    ),
                    array: None,
                    byte_offset: 0x840,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RtcTimerAPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_cr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_sr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_ptp_tx_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynPtpTxSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_ptp_tx_done",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynPtpTxDone",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_ptp_tx_trig",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynPtpTxTrig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_ptp_rx_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x101c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynPtpRxSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyntmr",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsyntmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_hclkdiv",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x103c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynHclkdiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_rxbuf_rx_frame_length_bytes",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynRxbufRxFrameLengthBytes",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_rxbuf_rx_time_stamp_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1608,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynRxbufRxTimeStampL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsyn_rxbuf_rx_time_stamp_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x160c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsynRxbufRxTimeStampH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxdata",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 60,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1610,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rxdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bin",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x1800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Bin",
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_hwcfg1",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperHwcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tqav",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x200c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTqav",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tqem",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTqem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_fpst",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperFpst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_mmct",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperMmct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_holdadv",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x201c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperHoldadv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mxsdu",
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
                    byte_offset: 0x2100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mxsdu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txsel",
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
                    byte_offset: 0x2120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idsel",
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
                    byte_offset: 0x2140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port1_qch0_cfg",
                    description: Some(
                        "qch channel0 control.",
                    ),
                    array: None,
                    byte_offset: 0x2800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Port1Qch0Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port1_qch1_cfg",
                    description: Some(
                        "qch channel1 control.",
                    ),
                    array: None,
                    byte_offset: 0x2804,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Port1Qch1Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port1_qch2_cfg",
                    description: Some(
                        "qch channel2 control.",
                    ),
                    array: None,
                    byte_offset: 0x2808,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Port1Qch2Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port1_qch3_cfg",
                    description: Some(
                        "qch channel3 control.",
                    ),
                    array: None,
                    byte_offset: 0x280c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Port1Qch3Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port1_qch_err_cfg",
                    description: Some(
                        "qch clear.",
                    ),
                    array: None,
                    byte_offset: 0x2810,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Port1QchErrCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_crsr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasCrsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_acycletm",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasAcycletm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_abasetm_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasAbasetmL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_abasetm_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x300c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasAbasetmH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_listlen",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasListlen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_ocycletm",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasOcycletm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_obasetm_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x3018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasObasetmL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_shaper_tas_obasetm_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x301c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnShaperTasObasetmH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mxtk",
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
                    byte_offset: 0x3020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mxtk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "txov",
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
                    byte_offset: 0x3040,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Txov",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shacl",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 256,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x3800,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Shacl",
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ver",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpVer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_txuf",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpTxuf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ipcfg",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpIpcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_tsf_d0",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpTsfD0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_tsf_d1",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpTsfD1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_tsf_d2",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpTsfD2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_tsf_sr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf02c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpTsfSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_mms_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf030,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpMmsCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_mms_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf034,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpMmsSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_mms_vtime",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf038,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpMmsVtime",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_mms_stat",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf03c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpMmsStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ptp_uptm_ns",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf040,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpPtpUptmNs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ptp_uptm_s",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf044,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpPtpUptmS",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsn_ep_ptp_sr",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0xf048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TsnEpPtpSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_port_main_tagging",
                    description: Some(
                        "PVID Tagging Register.",
                    ),
                    array: None,
                    byte_offset: 0x10000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlPortMainTagging",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_port_main_ennable",
                    description: Some(
                        "Port Module Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x10004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlPortMainEnnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_egress_ecsr_qdrop",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlEgressEcsrQdrop",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rxfifo",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 256,
                            },
                        ),
                    ),
                    byte_offset: 0x14000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Rxfifo",
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_monitor_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlMonitorCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_monitor_reset",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlMonitorReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sw_ctrl_monitor_param",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1800c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SwCtrlMonitorParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_tx_counter_tx_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorTxCounterTxFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_tx_counter_tx_ferror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorTxCounterTxFerror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_tx_counter_tx_drop_ovfl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorTxCounterTxDropOvfl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18040,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_ferror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxFerror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_known",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18050,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxKnown",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_unknown",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18058,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxUnknown",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_uc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18060,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxUc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_intern",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18068,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxIntern",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_bc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18070,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxBc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_multi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18078,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxMulti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_vlan",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxVlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_drop_ovfl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxDropOvfl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_drop_lu",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18090,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxDropLu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_drop_err",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18098,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxDropErr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_drop_vlan",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x180a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxDropVlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "monitor_rx_counter_rx_fpe_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x180a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MonitorRxCounterRxFpeFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ctrl0",
                    description: Some(
                        "control register0.",
                    ),
                    array: None,
                    byte_offset: 0x1c000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpr_ctrl2",
                    description: Some(
                        "control register2.",
                    ),
                    array: None,
                    byte_offset: 0x1c008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "GprCtrl2",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Tsw",
            extends: None,
            description: Some(
                "TSW.",
            ),
            items: &[
                BlockItem {
                    name: "lu_main_ctrl",
                    description: Some(
                        "LU_MAIN control.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_hitmem",
                    description: Some(
                        "LU_MAIN hit.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainHitmem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_param",
                    description: Some(
                        "LU_MAIN parameter.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_bypass",
                    description: Some(
                        "LU_MAIN bypass.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainBypass",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_pcp_remap",
                    description: Some(
                        "LU_MAIN PCP remap.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainPcpRemap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_version",
                    description: Some(
                        "LU_MAIN version.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainVersion",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_intf_action",
                    description: Some(
                        "LU_MAIN low word of action data for internal frames.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainIntfAction",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_bc_action",
                    description: Some(
                        "LU_MAIN low word of action data for broadcast frames.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainBcAction",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lu_main_nn_action",
                    description: Some(
                        "LU_MAIN low word of action data for unknown frames.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "LuMainNnAction",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_cam_sts",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisCamSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_cam_req_cnt",
                    description: Some(
                        "request count.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisCamReqCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_cam_fillsts",
                    description: Some(
                        "fill status.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisCamFillsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_cam_reset",
                    description: Some(
                        "reset.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisCamReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_cam_param",
                    description: Some(
                        "parameter.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisCamParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axi_cam_reqdata_0",
                    description: Some(
                        "data0.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axiCamReqdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axi_cam_reqdata_1",
                    description: Some(
                        "data1.",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axiCamReqdata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axi_cam_reqdata_2",
                    description: Some(
                        "data2.",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axiCamReqdata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_sts",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_req_cnt",
                    description: Some(
                        "request count.",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemReqCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_fillsts",
                    description: Some(
                        "fill status.",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemFillsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_reset",
                    description: Some(
                        "reset.",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_param",
                    description: Some(
                        "parameter.",
                    ),
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_reqdata_0",
                    description: Some(
                        "data0.",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemReqdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_almem_reqdata_1",
                    description: Some(
                        "data1.",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisAlmemReqdata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_sts",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_resp_cnt",
                    description: Some(
                        "response count.",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemRespCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_fillsts",
                    description: Some(
                        "fill status.",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemFillsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_reset",
                    description: Some(
                        "reset.",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_param",
                    description: Some(
                        "parameter.",
                    ),
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_respdata_0",
                    description: Some(
                        "data0.",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemRespdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_almem_respdata_1",
                    description: Some(
                        "data1.",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbAlmemRespdata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hitmem",
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
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hitmem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_sts",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x1000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_req_cnt",
                    description: Some(
                        "response count.",
                    ),
                    array: None,
                    byte_offset: 0x1010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupReqCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_fillsts",
                    description: Some(
                        "fill status.",
                    ),
                    array: None,
                    byte_offset: 0x1014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupFillsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_reset",
                    description: Some(
                        "reset.",
                    ),
                    array: None,
                    byte_offset: 0x1018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_param",
                    description: Some(
                        "parameter.",
                    ),
                    array: None,
                    byte_offset: 0x101c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_reqdata_0",
                    description: Some(
                        "LOOKUP REQUEST Register REQ_DATA_0.",
                    ),
                    array: None,
                    byte_offset: 0x1020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupReqdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_reqdata_1",
                    description: Some(
                        "LOOKUP REQUEST Register REQ_DATA_1.",
                    ),
                    array: None,
                    byte_offset: 0x1024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupReqdata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2axis_lookup_reqdata_3",
                    description: Some(
                        "LOOKUP REQUEST Register REQ_DATA_2.",
                    ),
                    array: None,
                    byte_offset: 0x102c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2axisLookupReqdata3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_sts",
                    description: Some(
                        "status register.",
                    ),
                    array: None,
                    byte_offset: 0x1080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_resp_cnt",
                    description: Some(
                        "response count.",
                    ),
                    array: None,
                    byte_offset: 0x1090,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupRespCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_fillsts",
                    description: Some(
                        "fill status.",
                    ),
                    array: None,
                    byte_offset: 0x1094,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupFillsts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_reset",
                    description: Some(
                        "reset.",
                    ),
                    array: None,
                    byte_offset: 0x1098,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_param",
                    description: Some(
                        "parameter.",
                    ),
                    array: None,
                    byte_offset: 0x109c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_respdata_0",
                    description: Some(
                        "LOOKUP RESPONSE Data Register.",
                    ),
                    array: None,
                    byte_offset: 0x10a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupRespdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axis2apb_lookup_respdata_1",
                    description: Some(
                        "LOOKUP RESPONSE Data Register.",
                    ),
                    array: None,
                    byte_offset: 0x10a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Axis2apbLookupRespdata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_csr_version",
                    description: Some(
                        "version register.",
                    ),
                    array: None,
                    byte_offset: 0x2000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralCsrVersion",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_csr_param",
                    description: Some(
                        "Parameter Register.",
                    ),
                    array: None,
                    byte_offset: 0x2004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralCsrParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_csr_config",
                    description: Some(
                        "Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x2008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralCsrConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_csr_cb_param",
                    description: Some(
                        "CB Parameter Register.",
                    ),
                    array: None,
                    byte_offset: 0x200c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralCsrCbParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_csr_qci_ctrl_param",
                    description: Some(
                        "QCI Control Parameter Register.",
                    ),
                    array: None,
                    byte_offset: 0x2010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralCsrQciCtrlParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_hwcfg",
                    description: Some(
                        "PSPF General CTRAL.",
                    ),
                    array: None,
                    byte_offset: 0x2104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciHwcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_filtersel",
                    description: Some(
                        "Filter select index.",
                    ),
                    array: None,
                    byte_offset: 0x2110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciFiltersel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_metersel",
                    description: Some(
                        "Flowmeter select index.",
                    ),
                    array: None,
                    byte_offset: 0x2114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciMetersel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_gatesel",
                    description: Some(
                        "Gate select index.",
                    ),
                    array: None,
                    byte_offset: 0x2118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciGatesel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_fctrl",
                    description: Some(
                        "FILTER SETTING.",
                    ),
                    array: None,
                    byte_offset: 0x2120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciFctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_fsize",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciFsize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qci_cnt",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x2140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "QciCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_mctrl",
                    description: Some(
                        "Flow meter settings.",
                    ),
                    array: None,
                    byte_offset: 0x2160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciMctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_cir",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciCir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_cbs",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciCbs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_eir",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciEir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_ebs",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x217c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciEbs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_gctrl",
                    description: Some(
                        "Gate settings.",
                    ),
                    array: None,
                    byte_offset: 0x2180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciGctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_gstatus",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciGstatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_glistindex",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciGlistindex",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_listlen",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x218c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciListlen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_acycletm",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAcycletm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_abasetm_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAbasetmL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_abasetm_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x2198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAbasetmH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_aentry_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x21a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAentryCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_aentry_aentry_ival",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x21a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAentryAentryIval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_aentry_ocycletm",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x21a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAentryOcycletm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_aentry_obasetm_l",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x21ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAentryObasetmL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "central_qci_aentry_obasetm_h",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x21b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CentralQciAentryObasetmH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_dma_cr",
                    description: Some(
                        "mm2s control register.",
                    ),
                    array: None,
                    byte_offset: 0x4000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sDmaCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_dma_sr",
                    description: Some(
                        "mm2s status.",
                    ),
                    array: None,
                    byte_offset: 0x4004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sDmaSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_dma_fill",
                    description: Some(
                        "mm2s dma fill status.",
                    ),
                    array: None,
                    byte_offset: 0x4008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sDmaFill",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_dma_cfg",
                    description: Some(
                        "mm2s dma configure.",
                    ),
                    array: None,
                    byte_offset: 0x401c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sDmaCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_addrlo",
                    description: Some(
                        "mm2s axi address.",
                    ),
                    array: None,
                    byte_offset: 0x4020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sAddrlo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_length",
                    description: Some(
                        "mm2s axi length.",
                    ),
                    array: None,
                    byte_offset: 0x4028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sLength",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_ctrl",
                    description: Some(
                        "mm2s command control.",
                    ),
                    array: None,
                    byte_offset: 0x402c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mm2s_resp",
                    description: Some(
                        "mm2s response buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4030,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mm2sResp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_dma_cr",
                    description: Some(
                        "s2mm dma control.",
                    ),
                    array: None,
                    byte_offset: 0x4080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmDmaCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_dma_sr",
                    description: Some(
                        "s2mm state.",
                    ),
                    array: None,
                    byte_offset: 0x4084,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmDmaSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_dma_fill",
                    description: Some(
                        "s2mm buffer fill status.",
                    ),
                    array: None,
                    byte_offset: 0x4088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmDmaFill",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_dma_cfg",
                    description: Some(
                        "s2mm dma config status.",
                    ),
                    array: None,
                    byte_offset: 0x409c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmDmaCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_addrlo",
                    description: Some(
                        "s2mm axi address.",
                    ),
                    array: None,
                    byte_offset: 0x40a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmAddrlo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_length",
                    description: Some(
                        "s2mm axi length.",
                    ),
                    array: None,
                    byte_offset: 0x40a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmLength",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_ctrl",
                    description: Some(
                        "s2mm command control.",
                    ),
                    array: None,
                    byte_offset: 0x40ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s2mm_resp",
                    description: Some(
                        "s2mm response buffer.",
                    ),
                    array: None,
                    byte_offset: 0x40b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "S2mmResp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_ts_ctl",
                    description: Some(
                        "timestamp control.",
                    ),
                    array: None,
                    byte_offset: 0x6000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtTsCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps_tod_sec",
                    description: Some(
                        "pps tod seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPpsTodSec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps_tod_ns",
                    description: Some(
                        "pps tod sun seconds.",
                    ),
                    array: None,
                    byte_offset: 0x600c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPpsTodNs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_sec0",
                    description: Some(
                        "target time seconds.",
                    ),
                    array: None,
                    byte_offset: 0x601c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpSec0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_ns0",
                    description: Some(
                        "target time sub seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpNs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_tmr_sts",
                    description: Some(
                        "timer status.",
                    ),
                    array: None,
                    byte_offset: 0x6028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtTmrSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps_cmd",
                    description: Some(
                        "pps command control.",
                    ),
                    array: None,
                    byte_offset: 0x602c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPpsCmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_atslo",
                    description: Some(
                        "auxiliray read data sub seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6030,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtAtslo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_atshi",
                    description: Some(
                        "auxiliray read data seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6034,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtAtshi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps0_interval",
                    description: Some(
                        "pps0 interval configure.",
                    ),
                    array: None,
                    byte_offset: 0x6060,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps0Interval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps0_width",
                    description: Some(
                        "pps0 width configure.",
                    ),
                    array: None,
                    byte_offset: 0x6064,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps0Width",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_sec1",
                    description: Some(
                        "target time seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpSec1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_ns1",
                    description: Some(
                        "target time sub seconds.",
                    ),
                    array: None,
                    byte_offset: 0x6084,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpNs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps1_interval",
                    description: Some(
                        "pps1 interval configure.",
                    ),
                    array: None,
                    byte_offset: 0x6088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps1Interval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps1_width",
                    description: Some(
                        "pps1 width configure.",
                    ),
                    array: None,
                    byte_offset: 0x608c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps1Width",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_sec2",
                    description: Some(
                        "target time seconds.",
                    ),
                    array: None,
                    byte_offset: 0x60a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpSec2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_ns2",
                    description: Some(
                        "target time sub seconds.",
                    ),
                    array: None,
                    byte_offset: 0x60a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpNs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps2_interval",
                    description: Some(
                        "pps2 interval configure.",
                    ),
                    array: None,
                    byte_offset: 0x60a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps2Interval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps2_width",
                    description: Some(
                        "pps2 width configure.",
                    ),
                    array: None,
                    byte_offset: 0x60ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps2Width",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_sec3",
                    description: Some(
                        "target time seconds.",
                    ),
                    array: None,
                    byte_offset: 0x60c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpSec3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_scp_ns3",
                    description: Some(
                        "target time sub seconds.",
                    ),
                    array: None,
                    byte_offset: 0x60c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtScpNs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps3_interval",
                    description: Some(
                        "pps3 interval configure.",
                    ),
                    array: None,
                    byte_offset: 0x60c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps3Interval",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps3_width",
                    description: Some(
                        "pps3 width configure.",
                    ),
                    array: None,
                    byte_offset: 0x60cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPps3Width",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps_ctrl0",
                    description: Some(
                        "pps control 0 register.",
                    ),
                    array: None,
                    byte_offset: 0x60e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPpsCtrl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_evt_pps_sel",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x60e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEvtPpsSel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "soft_rst_ctrl",
                    description: Some(
                        "softer reset control.",
                    ),
                    array: None,
                    byte_offset: 0x60f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SoftRstCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_port_main_tagging",
                    description: Some(
                        "PVID Tagging Register.",
                    ),
                    array: None,
                    byte_offset: 0x10000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortPortMainTagging",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_port_main_ennable",
                    description: Some(
                        "Port Module Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x10004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortPortMainEnnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_eselect",
                    description: Some(
                        "Stream Identification.",
                    ),
                    array: None,
                    byte_offset: 0x12800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidEselect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_control",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12840,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_seqno",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12844,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidSeqno",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_matchcnt",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12848,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidMatchcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_maclo",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12850,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidMaclo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_machi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12854,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidMachi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_stmid_amachi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1285c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressStmidAmachi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_control",
                    description: Some(
                        "Frame Replication and Elimination.",
                    ),
                    array: None,
                    byte_offset: 0x12a00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_sidsel",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerSidsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_irfunc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerIrfunc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_srfunc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerSrfunc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_fselect",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerFselect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_fctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerFctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_resetmsec",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerResetmsec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_lat_rs_period",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerLatRsPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_lat_test_period",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerLatTestPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_lat_err_diff_alw",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerLatErrDiffAlw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_egress_frer_lat_err_cnt",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x12a54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortEgressFrerLatErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "egfrcnt",
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
                    byte_offset: 0x12a60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Egfrcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_fdmem_cnt_byte",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14000,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoFdmemCntByte",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_fdmem_sts",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoFdmemSts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_error_flag",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoErrorFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_ie_error_flag",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1400c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoIeErrorFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_in_config",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoInConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_out_config",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14014,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoOutConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_reset",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_param",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1401c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_strfwd",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoStrfwd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_portmask",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoPortmask",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_mirror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoMirror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_rx_fdfifo_mirror_tx",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1402c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressRxFdfifoMirrorTx",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_eselect",
                    description: Some(
                        "Stream Identification.",
                    ),
                    array: None,
                    byte_offset: 0x14800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidEselect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_control",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14840,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_seqno",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14844,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidSeqno",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_matchcnt",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14848,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidMatchcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_maclo",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14850,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidMaclo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_machi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14854,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidMachi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_stmid_amachi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1485c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressStmidAmachi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_control",
                    description: Some(
                        "Frame Replication and Elimination.",
                    ),
                    array: None,
                    byte_offset: 0x14a00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_sidsel",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerSidsel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_irfunc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerIrfunc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_srfunc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerSrfunc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_fselect",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerFselect",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_fctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerFctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_resetmsec",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerResetmsec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_lat_rs_period",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerLatRsPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_lat_test_period",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerLatTestPeriod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_lat_err_diff_alw",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerLatErrDiffAlw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_igress_frer_lat_err_cnt",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x14a54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortIgressFrerLatErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "igfrcnt",
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
                    byte_offset: 0x14a60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Igfrcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_ctrl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18004,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_reset",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorReset",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_param",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x1800c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorParam",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_tx_counter_tx_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18010,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorTxCounterTxFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_tx_counter_tx_ferror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18018,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorTxCounterTxFerror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_tx_counter_tx_drop_ovfl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18020,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorTxCounterTxDropOvfl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18040,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_ferror",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18048,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxFerror",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_known",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18050,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxKnown",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_unknown",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18058,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxUnknown",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_uc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18060,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxUc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_intern",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18068,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxIntern",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_bc",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18070,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxBc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_multi",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18078,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxMulti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_vlan",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18080,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxVlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_drop_ovfl",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18088,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxDropOvfl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_drop_lu",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18090,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxDropLu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_drop_err",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x18098,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxDropErr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_drop_vlan",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x180a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxDropVlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_port_monitor_rx_counter_rx_fpe_fgood",
                    description: Some(
                        "No description available.",
                    ),
                    array: None,
                    byte_offset: 0x180a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuPortMonitorRxCounterRxFpeFgood",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsnport",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 131072,
                            },
                        ),
                    ),
                    byte_offset: 0x20000,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Tsnport",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Apb2axiCamReqdata0",
            extends: None,
            description: Some(
                "data0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "CAM APB2AXIS channel selection.",
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
                    name: "type_",
                    description: Some(
                        "select between set, clear or clear all.",
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
                    name: "entry_num",
                    description: Some(
                        "entry number.",
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
            name: "Apb2axiCamReqdata1",
            extends: None,
            description: Some(
                "data1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "destmac_lo_port_vec",
                    description: Some(
                        "dest-mac[31:0] when CH=0；PORT_VEC when CH=1.",
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
            name: "Apb2axiCamReqdata2",
            extends: None,
            description: Some(
                "data2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "destmac_hi",
                    description: Some(
                        "dest-mac[47:32] when CH=0.",
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
                    name: "vid",
                    description: Some(
                        "VLAN-ID value (12 bit) for the VLAN_ID table. Use the fefault VLAN-ID(VID=1), if setup an entry for non-VLAN traffic.",
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
            name: "Apb2axisAlmemFillsts",
            extends: None,
            description: Some(
                "fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO failure, internal controller lost synchronization.",
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
                    name: "full",
                    description: Some(
                        "frame was dropped because the internal descriptor FIFO is full.",
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
            name: "Apb2axisAlmemParam",
            extends: None,
            description: Some(
                "parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wordlen_byte",
                    description: Some(
                        "number of configured 32bit words for this controller.",
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
                    name: "depth",
                    description: Some(
                        "number of configured buffer depth.",
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
            ],
        },
        FieldSet {
            name: "Apb2axisAlmemReqCnt",
            extends: None,
            description: Some(
                "request count.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrcnt",
                    description: Some(
                        "number of streams in queue.",
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
            name: "Apb2axisAlmemReqdata0",
            extends: None,
            description: Some(
                "data0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "destination ports.",
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
                    name: "queue",
                    description: Some(
                        "select the priority queue if qsel=11.",
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
                    name: "drop",
                    description: Some(
                        "frame should dropped.",
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
                    name: "qsel",
                    description: Some(
                        "define the traffic queue selection.",
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
                Field {
                    name: "utag",
                    description: Some(
                        "user sideband information.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2axisAlmemReqdata1",
            extends: None,
            description: Some(
                "data1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "entry_num",
                    description: Some(
                        "define the entry number for reading and writing.",
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
                    name: "resp",
                    description: Some(
                        "write response enable.",
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
                    name: "wr_nrd",
                    description: Some(
                        "1 for write and 0 for read.",
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
            name: "Apb2axisAlmemReset",
            extends: None,
            description: Some(
                "reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "resets controller and clears all pending stream data.",
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
            name: "Apb2axisAlmemSts",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdy",
                    description: Some(
                        "the new data is written to data register.",
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
                    name: "busy",
                    description: Some(
                        "the controller is writing data and/or data is pending.",
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
            name: "Apb2axisCamFillsts",
            extends: None,
            description: Some(
                "fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO failure, internal controller lost synchronization.",
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
                    name: "full",
                    description: Some(
                        "frame was dropped because the internal descriptor FIFO is full.",
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
            name: "Apb2axisCamParam",
            extends: None,
            description: Some(
                "parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wordlen_byte",
                    description: Some(
                        "number of configured 32bit words for this controller.",
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
                    name: "depth",
                    description: Some(
                        "number of configured buffer depth.",
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
            ],
        },
        FieldSet {
            name: "Apb2axisCamReqCnt",
            extends: None,
            description: Some(
                "request count.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrcnt",
                    description: Some(
                        "number of streams in queue.",
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
            name: "Apb2axisCamReset",
            extends: None,
            description: Some(
                "reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "resets controller and clears all pending stream data.",
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
            name: "Apb2axisCamSts",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdy",
                    description: Some(
                        "the new data is written to data register.",
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
                    name: "busy",
                    description: Some(
                        "the controller is writing data and/or data is pending.",
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
            name: "Apb2axisLookupFillsts",
            extends: None,
            description: Some(
                "fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO failure.",
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
                    name: "full",
                    description: Some(
                        "FD FIFO full.",
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
            name: "Apb2axisLookupParam",
            extends: None,
            description: Some(
                "parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wordlen_byte",
                    description: Some(
                        "number of configured 32bit for this controller.",
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
                    name: "depth",
                    description: Some(
                        "number of configured buffer depth.",
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
            ],
        },
        FieldSet {
            name: "Apb2axisLookupReqCnt",
            extends: None,
            description: Some(
                "response count.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrcnt",
                    description: Some(
                        "number of streams in queue.",
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
            name: "Apb2axisLookupReqdata0",
            extends: None,
            description: Some(
                "LOOKUP REQUEST Register REQ_DATA_0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "destmac",
                    description: Some(
                        "Holding the first four bytes of requested MAC address.",
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
            name: "Apb2axisLookupReqdata1",
            extends: None,
            description: Some(
                "LOOKUP REQUEST Register REQ_DATA_1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "destmac",
                    description: Some(
                        "Holding the last two bytes of requested MAC address.",
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
            name: "Apb2axisLookupReqdata3",
            extends: None,
            description: Some(
                "LOOKUP REQUEST Register REQ_DATA_2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlan_tci",
                    description: Some(
                        "Set the requested traffic VLAN_TCI, if IS_VLAN=1.",
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
                    name: "is_vlan",
                    description: Some(
                        "Tell the LOOKUP module the requested traffic is VLAN tagged.",
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
            name: "Apb2axisLookupReset",
            extends: None,
            description: Some(
                "reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "Resets controller and clears all pending stream data.",
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
            name: "Apb2axisLookupSts",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdy",
                    description: Some(
                        "the new data is written to data register.",
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
                    name: "busy",
                    description: Some(
                        "the controller is writing data and/or data is pending.",
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
            name: "Axis2apbAlmemFillsts",
            extends: None,
            description: Some(
                "fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO failure.",
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
                    name: "full",
                    description: Some(
                        "FD FIFO full.",
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
            name: "Axis2apbAlmemParam",
            extends: None,
            description: Some(
                "parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wordlen_byte",
                    description: Some(
                        "number of configured 32bit for this controller.",
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
                    name: "depth",
                    description: Some(
                        "number of configured buffer depth.",
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
            ],
        },
        FieldSet {
            name: "Axis2apbAlmemReset",
            extends: None,
            description: Some(
                "reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "Resets controller and clears all pending stream data.",
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
            name: "Axis2apbAlmemRespCnt",
            extends: None,
            description: Some(
                "response count.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdcnt",
                    description: Some(
                        "number of streams in queue.",
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
            name: "Axis2apbAlmemRespdata0",
            extends: None,
            description: Some(
                "data0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "destination ports.",
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
                    name: "queue",
                    description: Some(
                        "select the priority queue if qsel=11.",
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
                    name: "drop",
                    description: Some(
                        "frame should dropped.",
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
                    name: "qsel",
                    description: Some(
                        "define the traffic queue selection.",
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
                Field {
                    name: "utag",
                    description: Some(
                        "user sideband information.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Axis2apbAlmemRespdata1",
            extends: None,
            description: Some(
                "data1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "entry_num",
                    description: Some(
                        "define the entry number for reading and writing.",
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
                    name: "resp",
                    description: Some(
                        "write response enable.",
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
                    name: "wr_nrd",
                    description: Some(
                        "1 for write and 0 for read.",
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
            name: "Axis2apbAlmemSts",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdy",
                    description: Some(
                        "the new data is written to data register.",
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
                    name: "busy",
                    description: Some(
                        "the controller is writing data and/or data is pending.",
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
            name: "Axis2apbLookupFillsts",
            extends: None,
            description: Some(
                "fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO failure.",
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
                    name: "full",
                    description: Some(
                        "FD FIFO full.",
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
            name: "Axis2apbLookupParam",
            extends: None,
            description: Some(
                "parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wordlen_byte",
                    description: Some(
                        "number of configured 32bit for this controller.",
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
                    name: "depth",
                    description: Some(
                        "number of configured buffer depth.",
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
            ],
        },
        FieldSet {
            name: "Axis2apbLookupReset",
            extends: None,
            description: Some(
                "reset.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "reset",
                    description: Some(
                        "Resets controller and clears all pending stream data.",
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
            name: "Axis2apbLookupRespCnt",
            extends: None,
            description: Some(
                "response count.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdcnt",
                    description: Some(
                        "number of streams in queue.",
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
            name: "Axis2apbLookupRespdata0",
            extends: None,
            description: Some(
                "LOOKUP RESPONSE Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "Forwarding ports from 0 to 15, Bit 0 is CPU port.",
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
                    name: "queue",
                    description: Some(
                        "TX traffic queue selection.",
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
                    name: "drop",
                    description: Some(
                        "Indicate that the frame should be dropped.",
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
                    name: "hit_vlan",
                    description: Some(
                        "Is 1, if VID hit entry in VLAN_PORT table.",
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
                    name: "utag",
                    description: Some(
                        "TSN user sideband information from ALMEM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hit",
                    description: Some(
                        "Is 1, if DESTMAC and VID hit an entry.",
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
                    name: "drop_vlan",
                    description: Some(
                        "Used for statistics. Shows that drop occurs by VLAN-ID.",
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
            ],
        },
        FieldSet {
            name: "Axis2apbLookupRespdata1",
            extends: None,
            description: Some(
                "LOOKUP RESPONSE Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "entry_num",
                    description: Some(
                        "Entry number of ALMEM.",
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
            name: "Axis2apbLookupSts",
            extends: None,
            description: Some(
                "status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdy",
                    description: Some(
                        "the new data is written to data register.",
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
                    name: "busy",
                    description: Some(
                        "the controller is writing data and/or data is pending.",
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
            name: "CentralCsrCbParam",
            extends: None,
            description: Some(
                "CB Parameter Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frer_d",
                    description: Some(
                        "Number of 802.1CB Recovery Function entries. 2^FRER_D entries.",
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
                    name: "sid_d",
                    description: Some(
                        "Number of 802.1CB Stream Identification entries. 2^SID_D entries.",
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
            ],
        },
        FieldSet {
            name: "CentralCsrConfig",
            extends: None,
            description: Some(
                "Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msec_cycles",
                    description: Some(
                        "Number of SYS_CLK cycles during 1 ms. It is required to calculate a correct time.",
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
            ],
        },
        FieldSet {
            name: "CentralCsrParam",
            extends: None,
            description: Some(
                "Parameter Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nports",
                    description: Some(
                        "Number of TSN ports without counting internal CPU port. For TSN-SE, it returns always 2.",
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
                    name: "type_",
                    description: Some(
                        "Specify type of switch core.",
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
                    name: "testmode",
                    description: Some(
                        "Shows if IP is configured in TESTMODE.",
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
                    name: "incl_cb0",
                    description: Some(
                        "Shows if IP is configured with “lightweight” 802.1CB at CPU-Port.",
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
                    name: "incl_qci",
                    description: Some(
                        "Shows if QCI module is present.",
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
            name: "CentralCsrQciCtrlParam",
            extends: None,
            description: Some(
                "QCI Control Parameter Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qci_ftd",
                    description: Some(
                        "(Log) filter table depth. 2**FTD entries.",
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
                    name: "qci_fmd",
                    description: Some(
                        "(Log) flow meter depth. 2**FMD entries.",
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
                    name: "qci_gtd",
                    description: Some(
                        "(Log) gate table depth. 2**GTD entries.",
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
            name: "CentralCsrVersion",
            extends: None,
            description: Some(
                "version register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver_rev",
                    description: Some(
                        "Reversion number of TSN-SW core.",
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
                    name: "ver_lo",
                    description: Some(
                        "Minor Version number of TSN-SW core.",
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
                    name: "ver_hi",
                    description: Some(
                        "Major Version number of TSN-SW core.",
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
            name: "CentralQciAbasetmH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "abth",
                    description: Some(
                        "No description available.",
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
            name: "CentralQciAbasetmL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "abtl",
                    description: Some(
                        "Administrative base time. Nanoseconds and seconds part. Cycle starts after becoming operational when time is reached by inputs <rtc_sec> and <rtc_ns>.",
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
            ],
        },
        FieldSet {
            name: "CentralQciAcycletm",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "act",
                    description: Some(
                        "Administrative cycle time length, nanoseconds.",
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
            ],
        },
        FieldSet {
            name: "CentralQciAentryAentryIval",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ival",
                    description: Some(
                        "AdminList – time interval in clock ticks.",
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
            name: "CentralQciAentryCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oct",
                    description: Some(
                        "AdminList – maximum octets (0 – disabled).",
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
                    name: "ipv",
                    description: Some(
                        "AdminList – IPV.",
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
                Field {
                    name: "state",
                    description: Some(
                        "AdminList – gate state (1: open).",
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
            name: "CentralQciAentryObasetmH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "obth",
                    description: Some(
                        "No description available.",
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
            name: "CentralQciAentryObasetmL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "obtl",
                    description: Some(
                        "OperBaseTime – nanoseconds and seconds. Constantly updated – OperBaseTime + N * OperCycleTimt. Might be non-normalized.",
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
            name: "CentralQciAentryOcycletm",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oct",
                    description: Some(
                        "OperCycleTime in nanoseconds.",
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
            name: "CentralQciCbs",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbs",
                    description: Some(
                        "Committed burst size, in bits (not octets!) (802.1Qci – 8.6.5.1.3 (c)).",
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
            name: "CentralQciCir",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cir",
                    description: Some(
                        "Committed information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (b)).",
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
            ],
        },
        FieldSet {
            name: "CentralQciEbs",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ebs",
                    description: Some(
                        "Excess burst size, in bits (not octets) (802.1Qci – 8.6.5.1.3 (e)).",
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
            name: "CentralQciEir",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eir",
                    description: Some(
                        "Excess information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (d)).",
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
            ],
        },
        FieldSet {
            name: "CentralQciFctrl",
            extends: None,
            description: Some(
                "FILTER SETTING.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sid",
                    description: Some(
                        "Filter Stream ID – if enabled by ENSID.",
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
                    name: "gid",
                    description: Some(
                        "Associated Gate.",
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
                    name: "fmd",
                    description: Some(
                        "Associated Flow Meter – if enabled by ENFID.",
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
                    name: "pcp",
                    description: Some(
                        "Filter priority code point, if enabled by ENPCP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enpcp",
                    description: Some(
                        "0: Filter match any PCP value 1: Filter match PCP value (802.1Qci – 8.6.5.1.1 (c)).",
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
                    name: "ensid",
                    description: Some(
                        "0: Filter match any SID value 1: Filter match SID value (802.1Qci – 8.6.5.1.1 (b)).",
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
                    name: "enfid",
                    description: Some(
                        "0: No Flow Meter 1: Enable Flow Metering (802.1Qci – 8.6.5.1.1 (e.2)).",
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
                    name: "enfsz",
                    description: Some(
                        "0: No frame size check 1: Frame size checking, size defined by FSIZE.MXSZ (802.1Qci – 8.6.5.1.1 (e.1)).",
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
                    name: "enblk",
                    description: Some(
                        "Enable blocking of oversized frames (802.1Qci – 8.6.5.1.1 (g)).",
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
            name: "CentralQciFiltersel",
            extends: None,
            description: Some(
                "Filter select index.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "Filter select index Any written value larger than the maximum index (2**FTD-1) will result in a read-back value of <0>.",
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
            name: "CentralQciFsize",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mxsz",
                    description: Some(
                        "Maximum-SDU size in octets.",
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
                    name: "blk",
                    description: Some(
                        "Stream blocked due to oversize frame. Write <1> to clear. (802.1Qci – 8.6.5.1.1 (h)).",
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
            name: "CentralQciGatesel",
            extends: None,
            description: Some(
                "Gate select index.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "Gate select index Any written value larger than the maximum index (2**GTD-1) will result in a read-back value of <0>.",
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
            name: "CentralQciGctrl",
            extends: None,
            description: Some(
                "Gate settings.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Gate control – enable.",
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
                    name: "cfgch",
                    description: Some(
                        "Gate – change config (self-resetting to <0>).",
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
                    name: "cdire",
                    description: Some(
                        "Gate – ClosedDueToInvalidRxEnable (802.1Qci – 8.6.5.1.2 (d)).",
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
                    name: "cdoee",
                    description: Some(
                        "Gate – ClosedDueToOctetsExceededEnable (802.1Qci – 8.6.5.1.2 (f)).",
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
                    name: "state",
                    description: Some(
                        "Administrative stream gate state (802.1Qci – 8.6.5.1.2 (b)).",
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
                    name: "ipv",
                    description: Some(
                        "Administrative internal priority value specification (802.1Qci – 8.6.5.1.2 (c)).",
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
            ],
        },
        FieldSet {
            name: "CentralQciGlistindex",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idx",
                    description: Some(
                        "Admin list pointer, select entry 0 – 15.",
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
            name: "CentralQciGstatus",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfgerr",
                    description: Some(
                        "Configuration change error. Write <1> to clear.",
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
                    name: "cfgp",
                    description: Some(
                        "Configuration change pending.",
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
                    name: "cdir",
                    description: Some(
                        "Gate – ClosedDueToInvalidRx. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (e)).",
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
                    name: "cdoe",
                    description: Some(
                        "Gate – ClosedDueToOctetsExceeded. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (g)).",
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
                    name: "state",
                    description: Some(
                        "Operational stream gate state (802.1Qci – 8.6.5.1.2 (b)).",
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
                    name: "ipv",
                    description: Some(
                        "Operational internal priority value specification (802.1Qci – 8.6.5.1.2 (c)).",
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
            ],
        },
        FieldSet {
            name: "CentralQciHwcfg",
            extends: None,
            description: Some(
                "PSPF General CTRAL.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftd",
                    description: Some(
                        "FTD – parameter.",
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
                    name: "gtd",
                    description: Some(
                        "GTD – parameter.",
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
                    name: "fmd",
                    description: Some(
                        "FMD – parameter.",
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
            name: "CentralQciListlen",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alen",
                    description: Some(
                        "Administrative list length.",
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
                    name: "olen",
                    description: Some(
                        "Operational list length.",
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
            name: "CentralQciMctrl",
            extends: None,
            description: Some(
                "Flow meter settings.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cf",
                    description: Some(
                        "Coupling flag (802.1Qci – 8.6.5.1.3 (f)).",
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
                    name: "cm",
                    description: Some(
                        "Color mode – functionally unused (802.1Qci – 8.6.5.1.3 (g)).",
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
                    name: "doy",
                    description: Some(
                        "DropOnYellow (802.1Qci – 8.6.5.1.3 (h)).",
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
                    name: "mafren",
                    description: Some(
                        "MarkAllFramesRedEnable (802.1Qci – 8.6.5.1.3 (i)).",
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
                    name: "mafr",
                    description: Some(
                        "MarkAllFramesRed – cleared by RESET (802.1Qci – 8.6.5.1.3 (j)).",
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
                    name: "reset",
                    description: Some(
                        "Flow Meter reset – self-resetting to <0>.",
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
            name: "CentralQciMetersel",
            extends: None,
            description: Some(
                "Flowmeter select index.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "index",
                    description: Some(
                        "Flowmeter select index Any written value larger than the maximum index (2**FMD-1) will result in a read-back value of <0>.",
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
            name: "CpuPortEgressFrerControl",
            extends: None,
            description: Some(
                "Frame Replication and Elimination.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtenc",
                    description: Some(
                        "R-TAG encoding enable.",
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
                    name: "later",
                    description: Some(
                        "Latent error flag – write 1 to clear.",
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
            name: "CpuPortEgressFrerFctrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tns",
                    description: Some(
                        "TakeNoSequence (802.1CB 10.4.1.9).",
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
                    name: "ind",
                    description: Some(
                        "Individual function (802.1CB 10.4.1.10).",
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
                    name: "laten",
                    description: Some(
                        "Latent error detection enable.",
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
                    name: "algo",
                    description: Some(
                        "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm.",
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
                    name: "hlen",
                    description: Some(
                        "History length (used by Vector recovery algorithm).",
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
                    name: "paths",
                    description: Some(
                        "Number of paths (used by latent error detection).",
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
                    name: "frset",
                    description: Some(
                        "Reset recovery function – self-resetting to 0.",
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
            name: "CpuPortEgressFrerFselect",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "Recovery function selection for host access at offset 0x140+.",
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
            name: "CpuPortEgressFrerIrfunc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "No description available.",
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
                    name: "fen",
                    description: Some(
                        "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0.",
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
            name: "CpuPortEgressFrerLatErrCnt",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laterr",
                    description: Some(
                        "Counter – latent error detect. Write any value to clear.",
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
            name: "CpuPortEgressFrerLatErrDiffAlw",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fdiff",
                    description: Some(
                        "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1).",
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
            name: "CpuPortEgressFrerLatRsPeriod",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flatr",
                    description: Some(
                        "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4).",
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
            ],
        },
        FieldSet {
            name: "CpuPortEgressFrerLatTestPeriod",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flatt",
                    description: Some(
                        "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2).",
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
            ],
        },
        FieldSet {
            name: "CpuPortEgressFrerResetmsec",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsrms",
                    description: Some(
                        "frerSeqRcvyResetMSec (802.1CB 10.4.1.7).",
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
            ],
        },
        FieldSet {
            name: "CpuPortEgressFrerSidsel",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sid",
                    description: Some(
                        "Stream ID selection for host access to IRFUNC and SRFUNC.",
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
            name: "CpuPortEgressFrerSrfunc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "No description available.",
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
                    name: "fen",
                    description: Some(
                        "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0.",
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
            name: "CpuPortEgressStmidAmachi",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amach",
                    description: Some(
                        "Active Destination MAC, MAC-Address [47:32].",
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
                    name: "avid",
                    description: Some(
                        "Active Destination MAC, VLAN ID.",
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
                    name: "apcp",
                    description: Some(
                        "Active Destination MAC, PCP.",
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
            name: "CpuPortEgressStmidControl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable entry.",
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
                    name: "mode",
                    description: Some(
                        "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged.",
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
                    name: "smac",
                    description: Some(
                        "0: Lookup by Destination MAC 1: Lookup by Source MAC.",
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
                    name: "actctl",
                    description: Some(
                        "Active Destination MAC – control. See Table 6-6.",
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
                    name: "seqgen",
                    description: Some(
                        "Sequence number generation enable.",
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
                    name: "sid",
                    description: Some(
                        "Stream ID – inserted to header on match.",
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
            ],
        },
        FieldSet {
            name: "CpuPortEgressStmidEselect",
            extends: None,
            description: Some(
                "Stream Identification.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "esel",
                    description: Some(
                        "Select entry. Selected entry mapped to 0x40 – 0x5C.",
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
            name: "CpuPortEgressStmidMachi",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "MAC-Address [47:31] used by lookup.",
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
                    name: "vid",
                    description: Some(
                        "VLAN ID used by lookup.",
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
            name: "CpuPortEgressStmidMaclo",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "macl",
                    description: Some(
                        "MAC-Address [31:0] used by lookup.",
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
            name: "CpuPortEgressStmidMatchcnt",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "Entry match counter – any write access to clear.",
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
            name: "CpuPortEgressStmidSeqno",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seqno",
                    description: Some(
                        "Sequence number – next number when generating,any write access to clear.",
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
            name: "CpuPortIgressFrerControl",
            extends: None,
            description: Some(
                "Frame Replication and Elimination.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtenc",
                    description: Some(
                        "R-TAG encoding enable.",
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
                    name: "later",
                    description: Some(
                        "Latent error flag – write 1 to clear.",
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
            name: "CpuPortIgressFrerFctrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tns",
                    description: Some(
                        "TakeNoSequence (802.1CB 10.4.1.9).",
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
                    name: "ind",
                    description: Some(
                        "Individual function (802.1CB 10.4.1.10).",
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
                    name: "laten",
                    description: Some(
                        "Latent error detection enable.",
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
                    name: "algo",
                    description: Some(
                        "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm.",
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
                    name: "hlen",
                    description: Some(
                        "History length (used by Vector recovery algorithm).",
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
                    name: "paths",
                    description: Some(
                        "Number of paths (used by latent error detection).",
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
                    name: "frset",
                    description: Some(
                        "Reset recovery function – self-resetting to 0.",
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
            name: "CpuPortIgressFrerFselect",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "Recovery function selection for host access at offset 0x140+.",
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
            name: "CpuPortIgressFrerIrfunc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "No description available.",
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
                    name: "fen",
                    description: Some(
                        "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0.",
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
            name: "CpuPortIgressFrerLatErrCnt",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laterr",
                    description: Some(
                        "Counter – latent error detect. Write any value to clear.",
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
            name: "CpuPortIgressFrerLatErrDiffAlw",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fdiff",
                    description: Some(
                        "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1).",
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
            name: "CpuPortIgressFrerLatRsPeriod",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flatr",
                    description: Some(
                        "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4).",
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
            ],
        },
        FieldSet {
            name: "CpuPortIgressFrerLatTestPeriod",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flatt",
                    description: Some(
                        "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2).",
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
            ],
        },
        FieldSet {
            name: "CpuPortIgressFrerResetmsec",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsrms",
                    description: Some(
                        "frerSeqRcvyResetMSec (802.1CB 10.4.1.7).",
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
            ],
        },
        FieldSet {
            name: "CpuPortIgressFrerSidsel",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sid",
                    description: Some(
                        "Stream ID selection for host access to IRFUNC and SRFUNC.",
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
            name: "CpuPortIgressFrerSrfunc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fidx",
                    description: Some(
                        "No description available.",
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
                    name: "fen",
                    description: Some(
                        "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0.",
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
            name: "CpuPortIgressRxFdfifoErrorFlag",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "desc_seq_err",
                    description: Some(
                        "FD FIFO failure. Internal controller lost synchronization.",
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
                    name: "desc_nrdy_err",
                    description: Some(
                        "FD FIFO failure. Descriptor not received correctly.",
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
                    name: "drop_full_mem",
                    description: Some(
                        "Frame was dropped because the FIFO is full. Full by too much data.",
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
                    name: "drop_full_desc",
                    description: Some(
                        "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames.",
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
                    name: "drop_nrdy",
                    description: Some(
                        "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO.",
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
                    name: "wrfail_full",
                    description: Some(
                        "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set.",
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
                    name: "lu_desc_err",
                    description: Some(
                        "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset.",
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
        FieldSet {
            name: "CpuPortIgressRxFdfifoFdmemCntByte",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fdmem_cnt_byte",
                    description: Some(
                        "Number of bytes stored in frame drop FIFO.",
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
            name: "CpuPortIgressRxFdfifoFdmemSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO empty.",
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
                    name: "amst_empty",
                    description: Some(
                        "FD FIFO almost empty. Few bytes in FIFO.",
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
                    name: "amst_full",
                    description: Some(
                        "FD FIFO almost full. Less than 1600 Byte left.",
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
                    name: "full",
                    description: Some(
                        "FD FIFO full.",
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
                    name: "ready",
                    description: Some(
                        "FD FIFO ready to work or working.",
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
                    name: "busy",
                    description: Some(
                        "FD FIFO processes data.",
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
                    name: "wait_for_frame",
                    description: Some(
                        "FD FIFO waits for more frame data.",
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
                    name: "wait_for_lu",
                    description: Some(
                        "FD FIFO waits for LookUp information.",
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
            name: "CpuPortIgressRxFdfifoIeErrorFlag",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ie",
                    description: Some(
                        "Interrupt enable of ERROR_FLAG.",
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
            name: "CpuPortIgressRxFdfifoInConfig",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nocut_error",
                    description: Some(
                        "FD_FIFO does not shorten frames which contain an error.",
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
            name: "CpuPortIgressRxFdfifoMirror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CpuPortIgressRxFdfifoMirrorTx",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CpuPortIgressRxFdfifoOutConfig",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode_store_fw",
                    description: Some(
                        "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward.",
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
                    name: "nodrop_error",
                    description: Some(
                        "Do not drop frame errors.",
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
                    name: "mirror_to_cpu",
                    description: Some(
                        "Duplicate frames to CPU.",
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
                    name: "error_to_cpu",
                    description: Some(
                        "Send error frames to CPU.",
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
                    name: "drop_all",
                    description: Some(
                        "Route all frames to DROP_DEST.",
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
                    name: "disable",
                    description: Some(
                        "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL.",
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
                    name: "ct_fpe_ovrd",
                    description: Some(
                        "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F.",
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
                    name: "mirror_rx_en",
                    description: Some(
                        "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX.",
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
                    name: "mirror_tx_en",
                    description: Some(
                        "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX.",
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
                    name: "drop_dest",
                    description: Some(
                        "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port[15:0].",
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
            name: "CpuPortIgressRxFdfifoParam",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fd_fifo_desc",
                    description: Some(
                        "Number of words (4byte) the Frame Drop FIFO can store.",
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
                    name: "fd_desc_fifo_desc",
                    description: Some(
                        "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame.",
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
                    name: "lu_fifo_depth",
                    description: Some(
                        "Number of MAC lookup descriptors the FIFO can store.",
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
            name: "CpuPortIgressRxFdfifoPortmask",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CpuPortIgressRxFdfifoReset",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "softrs",
                    description: Some(
                        "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched.",
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
            name: "CpuPortIgressRxFdfifoStrfwd",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CpuPortIgressStmidAmachi",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "amach",
                    description: Some(
                        "Active Destination MAC, MAC-Address [47:32].",
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
                    name: "avid",
                    description: Some(
                        "Active Destination MAC, VLAN ID.",
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
                    name: "apcp",
                    description: Some(
                        "Active Destination MAC, PCP.",
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
            name: "CpuPortIgressStmidControl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable entry.",
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
                    name: "mode",
                    description: Some(
                        "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged.",
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
                    name: "smac",
                    description: Some(
                        "0: Lookup by Destination MAC 1: Lookup by Source MAC.",
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
                    name: "actctl",
                    description: Some(
                        "Active Destination MAC – control. See Table 6-6.",
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
                    name: "seqgen",
                    description: Some(
                        "Sequence number generation enable.",
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
                    name: "sid",
                    description: Some(
                        "Stream ID – inserted to header on match.",
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
            ],
        },
        FieldSet {
            name: "CpuPortIgressStmidEselect",
            extends: None,
            description: Some(
                "Stream Identification.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "esel",
                    description: Some(
                        "Select entry. Selected entry mapped to 0x40 – 0x5C.",
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
            name: "CpuPortIgressStmidMachi",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "MAC-Address [47:31] used by lookup.",
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
                    name: "vid",
                    description: Some(
                        "VLAN ID used by lookup.",
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
            name: "CpuPortIgressStmidMaclo",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "macl",
                    description: Some(
                        "MAC-Address [31:0] used by lookup.",
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
            name: "CpuPortIgressStmidMatchcnt",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "match_",
                    description: Some(
                        "Entry match counter – any write access to clear.",
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
            name: "CpuPortIgressStmidSeqno",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seqno",
                    description: Some(
                        "Sequence number – next number when generating,any write access to clear.",
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
            name: "CpuPortMonitorCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enables counter. If deasserted the counter process stops and the counters hold their value.",
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
            name: "CpuPortMonitorParam",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntw",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
                    name: "tx_cnt_en_vec",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
                    name: "rx_cnt_en_vec",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
            name: "CpuPortMonitorReset",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsall",
                    description: Some(
                        "Write '1' to reset all TX&RX counters.",
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
                    name: "rstx",
                    description: Some(
                        "Write '1' to reset all TX counters.",
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
                    name: "rsrx",
                    description: Some(
                        "Write '1' to reset all RX counters.",
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
            name: "CpuPortMonitorRxCounterRxBc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_bc",
                    description: Some(
                        "Number of Broadcast frames.",
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
            name: "CpuPortMonitorRxCounterRxDropErr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_err",
                    description: Some(
                        "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress.",
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
            name: "CpuPortMonitorRxCounterRxDropLu",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_lu",
                    description: Some(
                        "Dropped frames by LookUp decision.",
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
            name: "CpuPortMonitorRxCounterRxDropOvfl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_ovfl",
                    description: Some(
                        "Dropped frames by ingress overflow.",
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
            name: "CpuPortMonitorRxCounterRxDropVlan",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_vlan",
                    description: Some(
                        "Dropped frames by incompatible VLAN.",
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
            name: "CpuPortMonitorRxCounterRxFerror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_ferror",
                    description: Some(
                        "Bad received frame by ingress buffer.",
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
            name: "CpuPortMonitorRxCounterRxFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_fgood",
                    description: Some(
                        "Good received frame by ingress buffer.",
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
            name: "CpuPortMonitorRxCounterRxFpeFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_fpe_fgood",
                    description: Some(
                        "Number of preemptable frames. Subset of RX_FGOOD.",
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
            name: "CpuPortMonitorRxCounterRxIntern",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_intern",
                    description: Some(
                        "Number of non-relay frames.",
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
            name: "CpuPortMonitorRxCounterRxKnown",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_known",
                    description: Some(
                        "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames.",
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
            name: "CpuPortMonitorRxCounterRxMulti",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_multi",
                    description: Some(
                        "Number of Multicast frames.",
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
            name: "CpuPortMonitorRxCounterRxUc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_uc",
                    description: Some(
                        "Number of unicast frames.",
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
            name: "CpuPortMonitorRxCounterRxUnknown",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_unknown",
                    description: Some(
                        "Number of frames passed ingress without hit by MAC table.",
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
            name: "CpuPortMonitorRxCounterRxVlan",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_vlan",
                    description: Some(
                        "Number of VLAN tagged frames.",
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
            name: "CpuPortMonitorTxCounterTxDropOvfl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_drop_ovfl",
                    description: Some(
                        "Dropped frames by full queue of TSN-EP.",
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
            name: "CpuPortMonitorTxCounterTxFerror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_ferror",
                    description: Some(
                        "Transmitted Frames with Error to TX TSN-EP.",
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
            name: "CpuPortMonitorTxCounterTxFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_fgood",
                    description: Some(
                        "Good transmitted Frames to TX TSN-EP.",
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
            name: "CpuPortPortMainEnnable",
            extends: None,
            description: Some(
                "Port Module Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_qci",
                    description: Some(
                        "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption.",
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
                    name: "en_sf",
                    description: Some(
                        "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption.",
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
            name: "CpuPortPortMainTagging",
            extends: None,
            description: Some(
                "PVID Tagging Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvid",
                    description: Some(
                        "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1.",
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
                    name: "dei",
                    description: Some(
                        "VLAN-TCI: Drop Eligible Indicator, used when tagged.",
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
                    name: "pcp",
                    description: Some(
                        "VLAN-TCI: Priority Code Point, used when tagged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "access",
                    description: Some(
                        "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged.",
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
                    name: "force",
                    description: Some(
                        "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports.",
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
            name: "Egfrcnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Frame counters.",
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
            name: "GprCtrl0",
            extends: None,
            description: Some(
                "control register0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txclk_dly_sel",
                    description: Some(
                        "delay value of txclk_delay_chain.",
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
                    name: "rxclk_dly_sel",
                    description: Some(
                        "delay value of rxclk_delay_chain.",
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
            name: "GprCtrl2",
            extends: None,
            description: Some(
                "control register2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmii_txclk_sel",
                    description: Some(
                        "txclk select control for RMII.",
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
                    name: "phy_intf_sel",
                    description: Some(
                        "phy interface select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pad_oe_eth_refclk",
                    description: Some(
                        "refclock output enable when rmii.",
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
                    name: "mac_speed",
                    description: Some(
                        "mac speed.",
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
            name: "Hitmem",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hitmem_reg",
                    description: Some(
                        "Every bit represents a lookup entry starting with bit 0 as entry 0. The memory can be written and cleared by the host system via common memory-mapped bus access.",
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
            name: "Idsel",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fract",
                    description: Some(
                        "No description available.",
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
                    name: "int",
                    description: Some(
                        "CBS idle slope for traffic queue n (n = 0 – 7). Returns 0 when n > TQC. The register must only be written when TXSELi.CBE_EN=0. The idle slope value is defined as (INT + FRACT / 65536). The idle slope is set in bits per tick related to <tx_clk>.",
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
            name: "Igfrcnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Frame counters.",
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
            name: "LuMainBcAction",
            extends: None,
            description: Some(
                "LU_MAIN low word of action data for broadcast frames.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3.",
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
                    name: "queue",
                    description: Some(
                        "Select the Priority Queue for TSN TX, only used if QSEL=11.",
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
                    name: "drop",
                    description: Some(
                        "1 if frame should be dropped.",
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
                    name: "qsel",
                    description: Some(
                        "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List.",
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
                Field {
                    name: "utag",
                    description: Some(
                        "TSN user sideband information from ALMEM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LuMainBypass",
            extends: None,
            description: Some(
                "LU_MAIN bypass.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "target destination ports of frame.",
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
                    name: "queue",
                    description: Some(
                        "number of configured buffer depth.",
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
                    name: "drop",
                    description: Some(
                        "mark frame to be dropped.",
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
                    name: "hit_vlan",
                    description: Some(
                        "mark frame to be vlan-tagged.",
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
                    name: "utag",
                    description: Some(
                        "set internal user tag field.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hit",
                    description: Some(
                        "set hit bit to frame, only for debugging.",
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
            name: "LuMainCtrl",
            extends: None,
            description: Some(
                "LU_MAIN control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "byp_en",
                    description: Some(
                        "MAC lookup bypass.",
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
            name: "LuMainHitmem",
            extends: None,
            description: Some(
                "LU_MAIN hit.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hitmemclr",
                    description: Some(
                        "clears the hit memory.",
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
                    name: "cammemclr",
                    description: Some(
                        "clear the cam memory.",
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
            name: "LuMainIntfAction",
            extends: None,
            description: Some(
                "LU_MAIN low word of action data for internal frames.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3.",
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
                    name: "queue",
                    description: Some(
                        "Select the Priority Queue for TSN TX, only used if QSEL=11.",
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
                    name: "drop",
                    description: Some(
                        "1 if frame should be dropped.",
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
                    name: "qsel",
                    description: Some(
                        "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List.",
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
                Field {
                    name: "utag",
                    description: Some(
                        "TSN user sideband information from ALMEM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LuMainNnAction",
            extends: None,
            description: Some(
                "LU_MAIN low word of action data for unknown frames.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dest",
                    description: Some(
                        "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3.",
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
                    name: "queue",
                    description: Some(
                        "Select the Priority Queue for TSN TX, only used if QSEL=11.",
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
                    name: "drop",
                    description: Some(
                        "1 if frame should be dropped.",
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
                    name: "qsel",
                    description: Some(
                        "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List.",
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
                Field {
                    name: "utag",
                    description: Some(
                        "TSN user sideband information from ALMEM.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LuMainParam",
            extends: None,
            description: Some(
                "LU_MAIN parameter.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrw_entry",
                    description: Some(
                        "bit width of entry address vector.",
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
                    name: "nstr",
                    description: Some(
                        "number of supported streams.",
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
            ],
        },
        FieldSet {
            name: "LuMainPcpRemap",
            extends: None,
            description: Some(
                "LU_MAIN PCP remap.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pcp0",
                    description: Some(
                        "queue value for PCP=0.",
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
                    name: "pcp1",
                    description: Some(
                        "queue value for PCP=1.",
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
                    name: "pcp2",
                    description: Some(
                        "queue value for PCP=2.",
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
                    name: "pcp3",
                    description: Some(
                        "queue value for PCP=3.",
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
                    name: "pcp4",
                    description: Some(
                        "queue value for PCP=4.",
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
                    name: "pcp5",
                    description: Some(
                        "queue value for PCP=5.",
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
                    name: "pcp6",
                    description: Some(
                        "queue value for PCP=6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcp7",
                    description: Some(
                        "queue value for PCP=7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "LuMainVersion",
            extends: None,
            description: Some(
                "LU_MAIN version.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver_rev",
                    description: Some(
                        "revision number.",
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
                    name: "ver_lo",
                    description: Some(
                        "minor version.",
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
                    name: "ver_hi",
                    description: Some(
                        "major version.",
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
            name: "MacIrqCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdie",
                    description: Some(
                        "MDIO Interrupt Enable 0 – Disabled 1 – Enabled.",
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
                    name: "swie",
                    description: Some(
                        "Safety warning interrupt enable 0 – SWIF disabled 1 – SWIF enabled.",
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
                    name: "caie",
                    description: Some(
                        "Clock activity interrupt enable 0 – CAIF disabled 1 – CAIF enabled.",
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
                    name: "mdif",
                    description: Some(
                        "MDIO Interrupt Flag 1 – A transfer has been finished 0 – No transfer done.",
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
                    name: "seif",
                    description: Some(
                        "Safety Error Interrupt Flag 0 – no interrupt 1 – interrupt pending If SEN=1 and if there is a mismatch between both instances of the logic core of LLEMAC-1G then this results in SEIF=1, TX_EN=0 and RX_EN=0.",
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
                    name: "swif",
                    description: Some(
                        "Safety warning interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.2 for details.",
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
                    name: "caif",
                    description: Some(
                        "Clock activity interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.3 for details.",
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
            name: "MacMacCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "resstat",
                    description: Some(
                        "Software reset of the statistic counters (see Table 3-8) 0 – no reset 1 – reset active RESSTAT will be automatically set to 0 after the counters have been reset.",
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
                    name: "rx_en",
                    description: Some(
                        "RX path enable 0 – reception disabled – no frames fed to Avalon-ST RX path 1 – reception enabled RX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in reception, then this frame will be completed first. Afterwards bit RX_EN can be read as 0.",
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
                    name: "tx_en",
                    description: Some(
                        "TX path enable 0 – transmission disabled - Avalon-ST READY for the TX path will be set to 0. 1 – transmission enabled TX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in transmission, then this frame will be completed fist. Afterwards bit TX_EN can be read as 0. After the transmission is disabled there may be pending frames left, waiting at the TX stream interface.",
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
                    name: "jumbo",
                    description: Some(
                        "Jumbo frame support 0 – jumbo frames not supported 1 – jumbo frame supported (not recommended) Jumbo frames are non-standard Ethernet frames with a size bigger than envelope frames (which contain 1982 payload bytes). If jumbo frames are not supported, then LLEMAC-1G generates the appropriate error signals (<tx_gmii_er> for the TX path and <rx_avst_err> for the RX path). Although jumbo frames typically contain up to 9000 bytes, the LLEMAC-1G can handle an infinite frame size. The problem of jumbo frames is the necessary storage space in transmission and reception buffers. LLEMAC-1G does not include storage buffers. JUMBO can be activated or deactivated at any time. The new setting becomes valid immediately after clock domain crossing.",
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
                    name: "gmiimode",
                    description: Some(
                        "GMII mode / Ethernet speed selection (See Chapter 4.5.) 0 – MII: 10Mbit/s or 100Mbit/s 1 – GMII: 1GBit/s GMIIMODE can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. GMIIMODE can only be changed, if these register bits can be read as 0. It is possible to change GMIIMODE together with the activation of RX_EN and TX_EN. GMIIMODE drives the outputs <tx_gmiimode> and <rx_gmiimode>.",
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
                    name: "physel",
                    description: Some(
                        "Selection of the PHY (See Chapter 4.6.) 00 – MII 01 – GMII 10 – RGMII 11 – reserved PHYSEL can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. PHYSEL can only be changed, if these register bits can be read as 0. It is possible to change PHYSEL together with the activation of RX_EN and TX_EN. PHYSEL drives the output <rx_physel>.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clksel",
                    description: Some(
                        "TX path clock selector 000 – <mii_clk> 001 – <ref_clk> (recommended setting for this selection) 010 – <ref_clk> divided by 5 011 – <ref_clk> divided by 10 100 – <ref_clk> divided by 50 111 – <ref_clk> and enables modification of RCE and MCE others – <ref_clk> See Chapter 7 for further details. CLKSEL is write-locked if CSA=1.",
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
                    name: "mce",
                    description: Some(
                        "<mii_clk> enable 0 – disabled 1 – enabled MCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details.",
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
                    name: "rce",
                    description: Some(
                        "<ref_clk> enable 0 – disabled 1 – enabled RCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details.",
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
                    name: "csa",
                    description: Some(
                        "Clock switching active (<tx_clk>) 0 – not active 1 – active Switching of <tx_clk> is commanded if CLKSEL or FSTIM (see Table 11-1) are written. Clock switching takes a few clock cycles and this is signaled with CSA=1. When CSA=1 then CLKSEL and FSTIM are write-locked and cannot be changed.",
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
                    name: "sen",
                    description: Some(
                        "Safety Enable 0 – disabled 1 – enabled If enabled, then two instances of the logic core of LLEMAC-1G are compared at runtime to each other. SEN can only be changed if RX_EN and TX_EN can be read as 0. Deactivation delays of RX_EN and TX_EN have to be considered. It is possible to change SEN together with the activation of RX_EN and TX_EN.",
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
                    name: "mca",
                    description: Some(
                        "<mii_clk> active 0 – not active 1 – active See chapter 11.2.3 for details.",
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
                    name: "rca",
                    description: Some(
                        "<ref_clk> active 0 – not active 1 – active See chapter 11.2.3 for details.",
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
                    name: "fstim",
                    description: Some(
                        "Fault Stimulation See Chapter 11.3, Table 11-1 for details. FSTIM is write-locked if CSA=1.",
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
            name: "MacMacaddrH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "macaddr",
                    description: Some(
                        "MAC address (see Chapter 4.1) Upper bits of MAC address (47:32). MACADDR can only be modified if TX_EN=0 and RX_EN=0.",
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
                    name: "promisc",
                    description: Some(
                        "0 – disabled 1 – enabled If promiscuous mode is enabled, then reception of all frames independent from the Ethernet destination address is enabled. PROMISC can be changed at any time.",
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
            name: "MacMacaddrL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "macaddr",
                    description: Some(
                        "MAC address Lower bits of MAC address (31:0). MACADDR only be modified if TX_EN=0 and RX_EN=0.",
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
            name: "MacMdioCfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdc_clkdiv",
                    description: Some(
                        "Clock Divider to configure MDC clock frequency. Refer to 10.1 Clock Divider for more details.",
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
                    name: "enable",
                    description: Some(
                        "Enable the MDIO controller. If the controller is enabled then MDC will be toggled. ENABLE can only be read as 1 if a valid MDC_CLKDIV value is set.",
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
                    name: "npre",
                    description: Some(
                        "No Preamble With NPRE=1 the preamble generation is suppressed and frames are initiated with Start of Frame pattern directly. Suitable in case that all connected PHYs accept management frames without a preamble pattern. Recommended to be used if only one PHY is connected.",
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
            name: "MacMdioCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ready",
                    description: Some(
                        "READY=1 indicates a finished transfer and also shows that the controller is ready for a new transfer. READY=1 is only possible if ENABLE=1. If READY=1 is signaled after a read transfer, then RD_DATA is valid until a new transfer is started.",
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
                    name: "init",
                    description: Some(
                        "INIT=1 results in a MDIO write/read transfer if READY=1. If READY=0 while a transfer is already pending or if ENABLE=0 then settings INIT=1 has no effect and the current transaction is withdrawn.",
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
                    name: "regad",
                    description: Some(
                        "Management Frame Register Address.",
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
                    name: "phyad",
                    description: Some(
                        "Management Frame PHY Address.",
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
                Field {
                    name: "op",
                    description: Some(
                        "Opcode to determine transfer type 01 – Write Access 10 – Read Access.",
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
            name: "MacMdioRdData",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_data",
                    description: Some(
                        "Read Data is available if READY=1 after a transfer has been started. RD_DATA represents the content of the management data field of the read transfer.",
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
            name: "MacMdioWrData",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_data",
                    description: Some(
                        "Data is used for the management data field after a write transfer has been started.",
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
            name: "MacRxFrames",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_frames",
                    description: Some(
                        "Number of successfully received frames.",
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
            name: "MacRxOctets",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_octets",
                    description: Some(
                        "Number of successfully received payload and padding octets.",
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
            name: "MacTxFrames",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_frames",
                    description: Some(
                        "Number of successfully transmitted frames.",
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
            name: "MacTxOctets",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_octets",
                    description: Some(
                        "Number of successfully transmitted payload and padding octets.",
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
            name: "MacVer",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver_l",
                    description: Some(
                        "Minor version number (lower part of the version).",
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
                    name: "ver_h",
                    description: Some(
                        "Major version number (higher part of the version).",
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
            name: "Mm2sAddrlo",
            extends: None,
            description: Some(
                "mm2s axi address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrlo",
                    description: Some(
                        "axi address.",
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
            name: "Mm2sCtrl",
            extends: None,
            description: Some(
                "mm2s command control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id",
                    description: Some(
                        "command id.",
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
                    name: "ngenlast",
                    description: Some(
                        "no generation of TLAST.",
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
                    name: "go",
                    description: Some(
                        "commit buffered descriptor to command queue.",
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
            name: "Mm2sDmaCfg",
            extends: None,
            description: Some(
                "mm2s dma configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver",
                    description: Some(
                        "ip version.",
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
                    name: "asize",
                    description: Some(
                        "axi data bus width.",
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
                    name: "ena64",
                    description: Some(
                        "enable support for 64 bit addressing.",
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
                    name: "cbufd",
                    description: Some(
                        "command buffer depth.",
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
                    name: "dbufd",
                    description: Some(
                        "data buffer depth.",
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
            name: "Mm2sDmaCr",
            extends: None,
            description: Some(
                "mm2s control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "run",
                    description: Some(
                        "run command from queue to data mover.",
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
                    name: "soe",
                    description: Some(
                        "stop on error flag.",
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
                    name: "reset",
                    description: Some(
                        "do reset when active.",
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
                    name: "irqen",
                    description: Some(
                        "interrupt request enable.",
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
                    name: "mxlen",
                    description: Some(
                        "max axi burst size.",
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
            name: "Mm2sDmaFill",
            extends: None,
            description: Some(
                "mm2s dma fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfill",
                    description: Some(
                        "command buffer fill level.",
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
                    name: "rfill",
                    description: Some(
                        "response buffer fill level.",
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
            name: "Mm2sDmaSr",
            extends: None,
            description: Some(
                "mm2s status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stop",
                    description: Some(
                        "mm2s is stopped.",
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
                    name: "busy",
                    description: Some(
                        "busy.",
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
                    name: "rset",
                    description: Some(
                        "resetting status.",
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
                    name: "irq",
                    description: Some(
                        "interrupt request pending.",
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
                    name: "cbufe",
                    description: Some(
                        "command buffer empty.",
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
                    name: "cbuff",
                    description: Some(
                        "command buffer full.",
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
                    name: "rbufe",
                    description: Some(
                        "response buffer empty.",
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
                    name: "rbuff",
                    description: Some(
                        "response buffer full.",
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
            name: "Mm2sLength",
            extends: None,
            description: Some(
                "mm2s axi length.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "length",
                    description: Some(
                        "transfer request length in bytes.",
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
            name: "Mm2sResp",
            extends: None,
            description: Some(
                "mm2s response buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "length",
                    description: Some(
                        "requested length of tansfer in bytes from command.",
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
                    name: "id",
                    description: Some(
                        "command ID feedback.",
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
                    name: "slverr",
                    description: Some(
                        "slave error.",
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
                    name: "decerr",
                    description: Some(
                        "decode error.",
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
                    name: "last",
                    description: Some(
                        "axi-stream with TLAST.",
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
            ],
        },
        FieldSet {
            name: "MonitorRxCounterRxBc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_bc",
                    description: Some(
                        "Number of Broadcast frames.",
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
            name: "MonitorRxCounterRxDropErr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_err",
                    description: Some(
                        "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress.",
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
            name: "MonitorRxCounterRxDropLu",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_lu",
                    description: Some(
                        "Dropped frames by LookUp decision.",
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
            name: "MonitorRxCounterRxDropOvfl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_ovfl",
                    description: Some(
                        "Dropped frames by ingress overflow.",
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
            name: "MonitorRxCounterRxDropVlan",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_drop_vlan",
                    description: Some(
                        "Dropped frames by incompatible VLAN.",
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
            name: "MonitorRxCounterRxFerror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_ferror",
                    description: Some(
                        "Bad received frame by ingress buffer.",
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
            name: "MonitorRxCounterRxFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_fgood",
                    description: Some(
                        "Good received frame by ingress buffer.",
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
            name: "MonitorRxCounterRxFpeFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_fpe_fgood",
                    description: Some(
                        "Number of preemptable frames. Subset of RX_FGOOD.",
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
            name: "MonitorRxCounterRxIntern",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_intern",
                    description: Some(
                        "Number of non-relay frames.",
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
            name: "MonitorRxCounterRxKnown",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_known",
                    description: Some(
                        "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames.",
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
            name: "MonitorRxCounterRxMulti",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_multi",
                    description: Some(
                        "Number of Multicast frames.",
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
            name: "MonitorRxCounterRxUc",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_uc",
                    description: Some(
                        "Number of unicast frames.",
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
            name: "MonitorRxCounterRxUnknown",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_unknown",
                    description: Some(
                        "Number of frames passed ingress without hit by MAC table.",
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
            name: "MonitorRxCounterRxVlan",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_vlan",
                    description: Some(
                        "Number of VLAN tagged frames.",
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
            name: "MonitorTxCounterTxDropOvfl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_drop_ovfl",
                    description: Some(
                        "Dropped frames by full queue of TSN-EP.",
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
            name: "MonitorTxCounterTxFerror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_ferror",
                    description: Some(
                        "Transmitted Frames with Error to TX TSN-EP.",
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
            name: "MonitorTxCounterTxFgood",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_fgood",
                    description: Some(
                        "Good transmitted Frames to TX TSN-EP.",
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
            name: "Mxsdu",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sdu",
                    description: Some(
                        "Maximum SDU size for traffic queue n (n = 0 – 7)Returns 0 when n > TQC. Value is size in words (32 bit word size).",
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
            name: "Mxtk",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tick",
                    description: Some(
                        "Maximum SDU size in clock ticks. MXTKi is only supported when TQC > i, otherwise read-only with value 0.",
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
            ],
        },
        FieldSet {
            name: "Port1Qch0Cfg",
            extends: None,
            description: Some(
                "qch channel0 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cqf_en",
                    description: Some(
                        "qch enable.",
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
                    name: "axis_qch_en",
                    description: Some(
                        "qch queue in select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tas_gpio_sel",
                    description: Some(
                        "tas_gpio select.",
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
                    name: "cqf_num",
                    description: Some(
                        "qch queue destination buffer select.",
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
                    name: "cqf_in_err",
                    description: Some(
                        "qch queue in error.",
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
            name: "Port1Qch1Cfg",
            extends: None,
            description: Some(
                "qch channel1 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cqf_en",
                    description: Some(
                        "qch enable.",
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
                    name: "axis_qch_en",
                    description: Some(
                        "qch queue in select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tas_gpio_sel",
                    description: Some(
                        "tas_gpio select.",
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
                    name: "cqf_num",
                    description: Some(
                        "qch queue destination buffer select.",
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
                    name: "cqf_in_err",
                    description: Some(
                        "qch queue in error.",
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
            name: "Port1Qch2Cfg",
            extends: None,
            description: Some(
                "qch channel2 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cqf_en",
                    description: Some(
                        "qch enable.",
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
                    name: "axis_qch_en",
                    description: Some(
                        "qch queue in select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tas_gpio_sel",
                    description: Some(
                        "tas_gpio select.",
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
                    name: "cqf_num",
                    description: Some(
                        "qch queue destination buffer select.",
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
                    name: "cqf_in_err",
                    description: Some(
                        "qch queue in error.",
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
            name: "Port1Qch3Cfg",
            extends: None,
            description: Some(
                "qch channel3 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cqf_en",
                    description: Some(
                        "qch enable.",
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
                    name: "axis_qch_en",
                    description: Some(
                        "qch queue in select.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tas_gpio_sel",
                    description: Some(
                        "tas_gpio select.",
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
                    name: "cqf_num",
                    description: Some(
                        "qch queue destination buffer select.",
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
                    name: "cqf_in_err",
                    description: Some(
                        "qch queue in error.",
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
            name: "Port1QchErrCfg",
            extends: None,
            description: Some(
                "qch clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cqf_clr_ctrl",
                    description: Some(
                        "enable cqf buffer auto clear when error.",
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
                    name: "axis_qch_cfg_err",
                    description: Some(
                        "axis_qch_en config error.",
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
                    name: "cqf_num_cfg_err",
                    description: Some(
                        "cqf_num config error.",
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
                    name: "cqf_que_err",
                    description: Some(
                        "que gate error for each cqf.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtAtshi",
            extends: None,
            description: Some(
                "auxiliray read data seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stshi",
                    description: Some(
                        "auxiliary fifo read seconds info.",
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
            name: "PtpEvtAtslo",
            extends: None,
            description: Some(
                "auxiliray read data sub seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stslo",
                    description: Some(
                        "auxiliary fifo read sub seconds info.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpEvtPps0Interval",
            extends: None,
            description: Some(
                "pps0 interval configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsint",
                    description: Some(
                        "PPS0 output signal interval.",
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
            name: "PtpEvtPps0Width",
            extends: None,
            description: Some(
                "pps0 width configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_width",
                    description: Some(
                        "pps0 output signal width.",
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
            name: "PtpEvtPps1Interval",
            extends: None,
            description: Some(
                "pps1 interval configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsint",
                    description: Some(
                        "PPS1 output signal interval.",
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
            name: "PtpEvtPps1Width",
            extends: None,
            description: Some(
                "pps1 width configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_width",
                    description: Some(
                        "pps1 output signal width.",
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
            name: "PtpEvtPps2Interval",
            extends: None,
            description: Some(
                "pps2 interval configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsint",
                    description: Some(
                        "PPS2 output signal interval.",
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
            name: "PtpEvtPps2Width",
            extends: None,
            description: Some(
                "pps2 width configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_width",
                    description: Some(
                        "pps2 output signal width.",
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
            name: "PtpEvtPps3Interval",
            extends: None,
            description: Some(
                "pps3 interval configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsint",
                    description: Some(
                        "PPS3 output signal interval.",
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
            name: "PtpEvtPps3Width",
            extends: None,
            description: Some(
                "pps3 width configure.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_width",
                    description: Some(
                        "pps3 output signal width.",
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
            name: "PtpEvtPpsCmd",
            extends: None,
            description: Some(
                "pps command control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_cmd0",
                    description: Some(
                        "pps0 command.",
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
                    name: "pps_en0",
                    description: Some(
                        "flexible PPS0 output mode enable.",
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
                    name: "pps_mode0",
                    description: Some(
                        "Target Time Register Mode for PPS0 Output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pps_cmd1",
                    description: Some(
                        "pps1 command.",
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
                    name: "pps_mode1",
                    description: Some(
                        "Target Time Register Mode for PPS1 Output.",
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
                    name: "pps_cmd2",
                    description: Some(
                        "pps2 command.",
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
                    name: "pps_mode2",
                    description: Some(
                        "Target Time Register Mode for PPS2 Output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pps_cmd3",
                    description: Some(
                        "pps3 command.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pps_mode3",
                    description: Some(
                        "Target Time Register Mode for PPS3 Output.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtPpsCtrl0",
            extends: None,
            description: Some(
                "pps control 0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time_sel",
                    description: Some(
                        "timer selection.",
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
                    name: "fifo_wr_intr_msk",
                    description: Some(
                        "auxiliary snapshot fifo write interrupt enable.",
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
                    name: "target_rac_intr_msk",
                    description: Some(
                        "target timmer interrupt mask.",
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
                    name: "pps_tod_intr_msk",
                    description: Some(
                        "pps tod interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtPpsSel",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps0_sel",
                    description: Some(
                        "pps selection for pps0.",
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
                    name: "pps1_sel",
                    description: Some(
                        "pps selection for pps1.",
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
                    name: "pps2_sel",
                    description: Some(
                        "pps selection for pps2.",
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
                    name: "pps3_sel",
                    description: Some(
                        "pps selection for pps3.",
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
            name: "PtpEvtPpsTodNs",
            extends: None,
            description: Some(
                "pps tod sun seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_tod_ns",
                    description: Some(
                        "pps tod sub seconds.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtPpsTodSec",
            extends: None,
            description: Some(
                "pps tod seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pps_tod_sec",
                    description: Some(
                        "pps tod seconds.",
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
            name: "PtpEvtScpNs0",
            extends: None,
            description: Some(
                "target time sub seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_ns",
                    description: Some(
                        "target time sub seconds.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtScpNs1",
            extends: None,
            description: Some(
                "target time sub seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_ns",
                    description: Some(
                        "target time sub seconds.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtScpNs2",
            extends: None,
            description: Some(
                "target time sub seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_ns",
                    description: Some(
                        "target time sub seconds.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtScpNs3",
            extends: None,
            description: Some(
                "target time sub seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_ns",
                    description: Some(
                        "target time sub seconds.",
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
            ],
        },
        FieldSet {
            name: "PtpEvtScpSec0",
            extends: None,
            description: Some(
                "target time seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_sec",
                    description: Some(
                        "target time seconds.",
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
            name: "PtpEvtScpSec1",
            extends: None,
            description: Some(
                "target time seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_sec",
                    description: Some(
                        "target time seconds.",
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
            name: "PtpEvtScpSec2",
            extends: None,
            description: Some(
                "target time seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_sec",
                    description: Some(
                        "target time seconds.",
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
            name: "PtpEvtScpSec3",
            extends: None,
            description: Some(
                "target time seconds.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scp_sec",
                    description: Some(
                        "target time seconds.",
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
            name: "PtpEvtTmrSts",
            extends: None,
            description: Some(
                "timer status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "target_time0_reach_intr",
                    description: Some(
                        "target time0 reached.",
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
                    name: "ptp_fifo_wr_intr",
                    description: Some(
                        "auxiliary timestamp trigger snapshot.",
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
                    name: "target_time0_cfg_err",
                    description: Some(
                        "target time0 configure error.",
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
                    name: "target_time1_reach_intr",
                    description: Some(
                        "target time1 reached.",
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
                    name: "target_time1_cfg_err",
                    description: Some(
                        "target time1 configure error.",
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
                    name: "target_time2_reach_intr",
                    description: Some(
                        "target time2 reached.",
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
                    name: "target_time2_cfg_err",
                    description: Some(
                        "target time2 configure error.",
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
                    name: "target_time3_reach_intr",
                    description: Some(
                        "target time3 reached.",
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
                    name: "target_time3_cfg_err",
                    description: Some(
                        "target time3 configure error.",
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
                    name: "pps_tod_intr",
                    description: Some(
                        "pps tod intrrupt.",
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
                    name: "atport",
                    description: Some(
                        "auxiliary port.",
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
                    name: "atsstm",
                    description: Some(
                        "auxiliary fifo full error.",
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
                    name: "rd_cnt",
                    description: Some(
                        "fifo valid count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpEvtTsCtl",
            extends: None,
            description: Some(
                "timestamp control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tstig",
                    description: Some(
                        "timestamp interrupt trigger enable.",
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
                    name: "atsfc",
                    description: Some(
                        "auxiliary snapshot fifo clear.",
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
                    name: "atsen",
                    description: Some(
                        "auxiliay snapshot enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "QciCnt",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Filter counter (see 802.1Qci 8.6.5.1.1 f) CNT0: Frames that matched filter CNT1: Frames that passed gate CNT2: Frames that did not pass gate CNT3: Frames that passed Maximum-SDU size check CNT4: Frames that did not pass size check CNT5: Frames discarded by Flow Meter operation Counters starting at value <0> after reset.",
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
            name: "RtcAlarmNs",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "al_ns",
                    description: Some(
                        "Alarm Time (nanoseconds part). Valid value range from 0 – 999999999.",
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
            ],
        },
        FieldSet {
            name: "RtcAlarmSh",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "al_sh",
                    description: Some(
                        "Alarm Time (seconds hi part).",
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
            name: "RtcAlarmSl",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "al_sl",
                    description: Some(
                        "Alarm Time (seconds lo part).",
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
            name: "RtcCr",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alie",
                    description: Some(
                        "Alarm interrupt enable: alarm interrupt enabled when 1.",
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
                    name: "taen",
                    description: Some(
                        "Timer A enable: timer enabled when 1.",
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
                    name: "taie",
                    description: Some(
                        "Timer A interrupt enable: interrupt enabled when 1.",
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
            ],
        },
        FieldSet {
            name: "RtcCtCurtimeNs",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ct_ns",
                    description: Some(
                        "Local Time (nanosecond part): Update can be triggered by write access to this register. Value range from 0 – 999999999.",
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
            ],
        },
        FieldSet {
            name: "RtcCtCurtimeSec",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ct_sec",
                    description: Some(
                        "Current Time (second part): Update can be triggered by write access to register CURTIME_NS.",
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
            name: "RtcCtTimerIncr",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fns",
                    description: Some(
                        "Local time increment – fractional ns, unsigned, in (1 / 2^24) n.",
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
                    name: "ns",
                    description: Some(
                        "Local time increment – nanoseconds (integer).",
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
            name: "RtcOfsCh",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sfns",
                    description: Some(
                        "Real Time Offset Change in fractional nanoseconds, signed value; value range from -2^23 / 2^24 to (2^23-1) / 2^24 nanoseconds.",
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
                    name: "sext",
                    description: Some(
                        "Real Time Offset Change – sign extension of SFNS (Bit 23).",
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
            name: "RtcOfsNs",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ofs_ns",
                    description: Some(
                        "Real Time Offset (nanoseconds part). Valid value range from 0 – 999999999.",
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
            ],
        },
        FieldSet {
            name: "RtcOfsSh",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ofs_sh",
                    description: Some(
                        "48 Bit Real Time Offset (seconds hi part).",
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
            name: "RtcOfsSl",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ofs_sl",
                    description: Some(
                        "48 Bit Real Time Offset (seconds lo part).",
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
            name: "RtcSr",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alis",
                    description: Some(
                        "ALIS ro Alarm Interrupt Status: Always set while RTC-Time >= Alarm-Time.",
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
                    name: "tais",
                    description: Some(
                        "Timer A Interrupt Status: set at rising edge of “timer_clk_a”, write 1 to clear.",
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
            ],
        },
        FieldSet {
            name: "RtcTimerAPeriod",
            extends: None,
            description: Some(
                "ONLY IN PORT1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "period_ns",
                    description: Some(
                        "Timer A Period in ns. This is the period of the timer until the next event, but the half-period of the signal “timer_a_clk”.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rxdata",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxbuf_data_word",
                    description: Some(
                        "RXBUF_DATA_WORD.",
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
            name: "S2mmAddrlo",
            extends: None,
            description: Some(
                "s2mm axi address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addrlo",
                    description: Some(
                        "axi address.",
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
            name: "S2mmCtrl",
            extends: None,
            description: Some(
                "s2mm command control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "id",
                    description: Some(
                        "command id.",
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
                    name: "go",
                    description: Some(
                        "commit buffered descriptor to command queue.",
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
            name: "S2mmDmaCfg",
            extends: None,
            description: Some(
                "s2mm dma config status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver",
                    description: Some(
                        "IP version.",
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
                    name: "asize",
                    description: Some(
                        "axi data bus width.",
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
                    name: "ena64",
                    description: Some(
                        "enabled support for 64 bit.",
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
                    name: "cbufd",
                    description: Some(
                        "command buffer depth.",
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
                    name: "dbufd",
                    description: Some(
                        "data buffer depth.",
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
            name: "S2mmDmaCr",
            extends: None,
            description: Some(
                "s2mm dma control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "run",
                    description: Some(
                        "run commands from queue to data mover.",
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
                    name: "soe",
                    description: Some(
                        "stop on error flag.",
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
                    name: "reset",
                    description: Some(
                        "do reset when writing 1.",
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
                    name: "irqen",
                    description: Some(
                        "interrupt request enable.",
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
                    name: "mxlen",
                    description: Some(
                        "max axi burst size.",
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
            name: "S2mmDmaFill",
            extends: None,
            description: Some(
                "s2mm buffer fill status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfill",
                    description: Some(
                        "command buffer fill level.",
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
                    name: "rfill",
                    description: Some(
                        "response buffer fill level.",
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
            name: "S2mmDmaSr",
            extends: None,
            description: Some(
                "s2mm state.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stop",
                    description: Some(
                        "s2mm is stopped.",
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
                    name: "busy",
                    description: Some(
                        "busy, issued command and outstanding response.",
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
                    name: "rset",
                    description: Some(
                        "resetting status.",
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
                    name: "irq",
                    description: Some(
                        "interrupt request pending.",
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
                    name: "cbufe",
                    description: Some(
                        "command buffer empty.",
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
                    name: "cbuff",
                    description: Some(
                        "command buffer full.",
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
                    name: "rbufe",
                    description: Some(
                        "response buffer empty.",
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
                    name: "rbuff",
                    description: Some(
                        "response buffer full.",
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
            name: "S2mmLength",
            extends: None,
            description: Some(
                "s2mm axi length.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "length",
                    description: Some(
                        "transfer request length in bytes.",
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
            name: "S2mmResp",
            extends: None,
            description: Some(
                "s2mm response buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "length",
                    description: Some(
                        "received packet size when terminated by TLAST.",
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
                    name: "id",
                    description: Some(
                        "command ID feedback.",
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
                    name: "slverr",
                    description: Some(
                        "slave error.",
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
                    name: "decerr",
                    description: Some(
                        "decode error.",
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
                    name: "last",
                    description: Some(
                        "axi-stream with last.",
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
            ],
        },
        FieldSet {
            name: "SoftRstCtrl",
            extends: None,
            description: Some(
                "softer reset control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port1_tx_rst",
                    description: Some(
                        "port1 tx reset control.",
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
                    name: "port1_rx_rst",
                    description: Some(
                        "port1 rx reset control.",
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
                    name: "port2_tx_rst",
                    description: Some(
                        "port2 tx reset control.",
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
                    name: "port2_rx_rst",
                    description: Some(
                        "port2 rx reset control.",
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
                    name: "port3_tx_rst",
                    description: Some(
                        "port3 tx reset control.",
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
                    name: "port3_rx_rst",
                    description: Some(
                        "port3 rx reset control.",
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
                    name: "dma0_rst",
                    description: Some(
                        "dma0 reset control.",
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
                    name: "ptp_evt_rst",
                    description: Some(
                        "ptp event module reset control.",
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
                    name: "tsn_core_rst",
                    description: Some(
                        "tsn core reset control.",
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
            name: "SwCtrlEgressEcsrQdrop",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_vec",
                    description: Some(
                        "Enable/Disable drop in egress when TSN queue not free. 1 - drop enabled 0 - drop disabled TSN-SW: bit[i] - from Port[i].",
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
                    name: "dis_vec",
                    description: Some(
                        "disable drop for each queue when queue not free.",
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
            name: "SwCtrlIgressRxFdfifoEErrorFlag",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "desc_seq_err",
                    description: Some(
                        "FD FIFO failure. Internal controller lost synchronization.",
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
                    name: "desc_nrdy_err",
                    description: Some(
                        "FD FIFO failure. Descriptor not received correctly.",
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
                    name: "drop_full_mem",
                    description: Some(
                        "Frame was dropped because the FIFO is full. Full by too much data.",
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
                    name: "drop_full_desc",
                    description: Some(
                        "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames.",
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
                    name: "drop_nrdy",
                    description: Some(
                        "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO.",
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
                    name: "wrfail_full",
                    description: Some(
                        "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set.",
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
                    name: "lu_desc_err",
                    description: Some(
                        "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset.",
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
        FieldSet {
            name: "SwCtrlIgressRxFdfifoEFdmemCntByte",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fdmem_cnt_byte",
                    description: Some(
                        "Number of bytes stored in frame drop FIFO.",
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
            name: "SwCtrlIgressRxFdfifoEFdmemSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "FD FIFO empty.",
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
                    name: "amst_empty",
                    description: Some(
                        "FD FIFO almost empty. Few bytes in FIFO.",
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
                    name: "amst_full",
                    description: Some(
                        "FD FIFO almost full. Less than 1600 Byte left.",
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
                    name: "full",
                    description: Some(
                        "FD FIFO full.",
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
                    name: "ready",
                    description: Some(
                        "FD FIFO ready to work or working.",
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
                    name: "busy",
                    description: Some(
                        "FD FIFO processes data.",
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
                    name: "wait_for_frame",
                    description: Some(
                        "FD FIFO waits for more frame data.",
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
                    name: "wait_for_lu",
                    description: Some(
                        "FD FIFO waits for LookUp information.",
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
            name: "SwCtrlIgressRxFdfifoEIeErrorFlag",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ie",
                    description: Some(
                        "Interrupt enable of ERROR_FLAG.",
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
            name: "SwCtrlIgressRxFdfifoEInConfig",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nocut_error",
                    description: Some(
                        "FD_FIFO does not shorten frames which contain an error.",
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
            name: "SwCtrlIgressRxFdfifoEMirror",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SwCtrlIgressRxFdfifoEMirrorTx",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SwCtrlIgressRxFdfifoEOutConfig",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode_store_fw",
                    description: Some(
                        "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward.",
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
                    name: "nodrop_error",
                    description: Some(
                        "Do not drop frame errors.",
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
                    name: "mirror_to_cpu",
                    description: Some(
                        "Duplicate frames to CPU.",
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
                    name: "error_to_cpu",
                    description: Some(
                        "Send error frames to CPU.",
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
                    name: "drop_all",
                    description: Some(
                        "Route all frames to DROP_DEST.",
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
                    name: "disable",
                    description: Some(
                        "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL.",
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
                    name: "ct_fpe_ovrd",
                    description: Some(
                        "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F.",
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
                    name: "mirror_rx_en",
                    description: Some(
                        "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX.",
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
                    name: "mirror_tx_en",
                    description: Some(
                        "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX.",
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
                    name: "drop_dest",
                    description: Some(
                        "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port[15:0].",
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
            name: "SwCtrlIgressRxFdfifoEParam",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fd_fifo_desc",
                    description: Some(
                        "Number of words (4byte) the Frame Drop FIFO can store.",
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
                    name: "fd_desc_fifo_desc",
                    description: Some(
                        "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame.",
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
                    name: "lu_fifo_depth",
                    description: Some(
                        "Number of MAC lookup descriptors the FIFO can store.",
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
            name: "SwCtrlIgressRxFdfifoEPortmask",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SwCtrlIgressRxFdfifoEReset",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "softrs",
                    description: Some(
                        "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched.",
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
            name: "SwCtrlIgressRxFdfifoEStrfwd",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "port",
                    description: Some(
                        "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, ….",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SwCtrlMonitorCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enables counter. If deasserted the counter process stops and the counters hold their value.",
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
            name: "SwCtrlMonitorParam",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntw",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
                    name: "tx_cnt_en_vec",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
                    name: "rx_cnt_en_vec",
                    description: Some(
                        "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available.",
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
            name: "SwCtrlMonitorReset",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsall",
                    description: Some(
                        "Write '1' to reset all TX&RX counters.",
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
                    name: "rstx",
                    description: Some(
                        "Write '1' to reset all TX counters.",
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
                    name: "rsrx",
                    description: Some(
                        "Write '1' to reset all RX counters.",
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
            name: "SwCtrlPortMainEnnable",
            extends: None,
            description: Some(
                "Port Module Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en_qci",
                    description: Some(
                        "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption.",
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
                    name: "en_sf",
                    description: Some(
                        "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption.",
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
            name: "SwCtrlPortMainTagging",
            extends: None,
            description: Some(
                "PVID Tagging Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvid",
                    description: Some(
                        "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1.",
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
                    name: "dei",
                    description: Some(
                        "VLAN-TCI: Drop Eligible Indicator, used when tagged.",
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
                    name: "pcp",
                    description: Some(
                        "VLAN-TCI: Priority Code Point, used when tagged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "access",
                    description: Some(
                        "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged.",
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
                    name: "force",
                    description: Some(
                        "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports.",
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
            name: "TsnEpCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ie_tsf",
                    description: Some(
                        "TxTimestampFifo interrupt enable; interrupt will be set when IE_TSF=<1> and TSF_SR.USED>0.",
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
                    name: "ptp_1s_en",
                    description: Some(
                        "Enable PTPv2 1-step synchronization suppor.",
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
                    name: "filtdis",
                    description: Some(
                        "Disable filtering of PTP frames (Ethertype = 0x88F7).",
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
            name: "TsnEpIpcfg",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "incl_1step",
                    description: Some(
                        "IP core parameter “INCL_1STEP”.",
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
                    name: "incl_tsync",
                    description: Some(
                        "IP core parameter “INCL_TSYNC”.",
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
                    name: "incl_tsf",
                    description: Some(
                        "IP core parameter “INCL_TSF”.",
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
                    name: "incl_fpe",
                    description: Some(
                        "IP core parameter “INCL_FPE”.",
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
                    name: "incl_shap",
                    description: Some(
                        "IP core parameter “INCL_SHAPER”.",
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
                    name: "incl_rtc",
                    description: Some(
                        "IP core parameter “INCL_RTC”.",
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
            name: "TsnEpMmsCtrl",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable preemption.",
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
                    name: "link",
                    description: Some(
                        "Link error.",
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
                    name: "disv",
                    description: Some(
                        "Disable verification.",
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
                    name: "fragsz",
                    description: Some(
                        "Minimum non-final fragment size: 64 x (1 + FRAGSZ) – 4 octets.",
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
                    name: "statsel",
                    description: Some(
                        "MMS statistic counter selection, value can be read in register MMS_STAT <000>: Frame reassembly error counter (802.3br, 30.14.1.8) <001>: Frames rejected due to wrong SMD (802.3br, 30.14.1.9) <010>: Frame assembly ok counter (802.3br, 30.14.1.10) <011>: Fragment rx counter (802.3br, 30.14.1.11) <100>: Fragment tx counter (802.3br, 30.14.1.12) <101>: Hold request counter (802.3br, 30.14.1.13) otherwise: <0>.",
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
            ],
        },
        FieldSet {
            name: "TsnEpMmsStat",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "Statistic counter of MMS, selected by MMS_CTRL.STATSEL,any write access will clear selected counter.",
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
            name: "TsnEpMmsSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hld",
                    description: Some(
                        "HOLD-Signal.",
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
                    name: "vok",
                    description: Some(
                        "802.3br verification state ok; verification is done when any bit VFAIL or VOK is <1>.",
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
                    name: "vfail",
                    description: Some(
                        "802.3br verification state failure; verification is done when any bit VFAIL or VOK is <1>.",
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
            name: "TsnEpMmsVtime",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtime",
                    description: Some(
                        "802.3br verification timeout counter in <sys_clk> cycles. Must be set by software in range of 1ms to 128ms.",
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
            name: "TsnEpPtpSr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "meas_ns",
                    description: Some(
                        "Measured value of the deviation of the early timestamping for PTP frames. This value is informational only. The deviation is already included to the corrected “correctionField”.",
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
            name: "TsnEpPtpUptmNs",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uptm_ns",
                    description: Some(
                        "PTP SYNC frame “upstreamTxTime” in format “seconds.nanoseconds” as potentially received by another TSN-EP port. The correction field of a transmitted PTP SYNC frame is modified by (egressTimestamp –upstreamTxTime), relative to the LocalClock. The “rateRatio” to the Grandmaster Clock is not taken into account.",
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
            name: "TsnEpPtpUptmS",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uptm_ns",
                    description: Some(
                        "No description available.",
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
            name: "TsnEpTsfD0",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsf_ns",
                    description: Some(
                        "Tx-Timestamp-Fifo, lower 32 bit part of local time (curtime) at the start of transmission of the packet. Usually nanoseconds part when used with included RTC.",
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
            name: "TsnEpTsfD1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsf_sec",
                    description: Some(
                        "Tx-Timestamp-Fifo, upper 32 bit part of the local time (curtime) at the start of the transmission of the packet. Usually seconds part when used with included RTC.",
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
            name: "TsnEpTsfD2",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsf_usr",
                    description: Some(
                        "Tx-Timestamp-Fifo, user sideband <tx_tuser> of sent packet; Note: any read to register will remove actual value from FIFO.",
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
                    name: "tsf_tq",
                    description: Some(
                        "Tx-Timestamp-Fifo, traffic queue <tx_tqueue> of sent packet.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TsnEpTsfSr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsf_used",
                    description: Some(
                        "Tx-Timestamp-Fifo currently used entries counter; reading of TSF_Dx is only valid if field value > 0. Any read from TSF_D2 will decrement counter (unless already 0).",
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
                    name: "tsf_ov",
                    description: Some(
                        "Overflow of Tx-Timestamp-Fifo. At least one transmitted packet has been sent and timestamp was not stored; write bit to clear flag.",
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
            name: "TsnEpTxuf",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "counter",
                    description: Some(
                        "TX buffer underflow counter; incremented when any MAC runs out of data during transmission. The counter is cleared at any write access. The counter is shared by pMAC and eMAC. If underflow event occurs at the same time for pMAC and eMAC, it will be counted as one event.",
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
            name: "TsnEpVer",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ver_rev",
                    description: Some(
                        "revision number.",
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
                    name: "ver_lo",
                    description: Some(
                        "minor version number.",
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
                    name: "ver_hi",
                    description: Some(
                        "major version number.",
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
            name: "TsnShaperAclistEntry0H",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "time",
                    description: Some(
                        "Time interval, entry execution in in host clock ticks (<sys_clk>).",
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
            name: "TsnShaperAclistEntry0L",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "state",
                    description: Some(
                        "gate state vector; 1 – Gate is open.",
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
                    name: "op",
                    description: Some(
                        "gate operation: 0 – SetGateStates 1 – Set-And-Hold-MAC 2 – Set-And-Release-MAC 3 – undefined.",
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
                    name: "tas_gpio",
                    description: Some(
                        "gate states for qch and ptp event source.",
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
            ],
        },
        FieldSet {
            name: "TsnShaperFpst",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "table",
                    description: Some(
                        "Frame Preemption Status Table, Bit[i] = 1: Preemptable traffic in TQ[i], otherwise Express traffic (default).",
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
            name: "TsnShaperHoldadv",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "holdAdvance time for TAS operation Set-And-Hold-MAC in <sys_clk> cycles.",
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
            name: "TsnShaperHwcfg1",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dw",
                    description: Some(
                        "Traffic queue data width (Bytes); fixed to value 4 within IP core.",
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
                    name: "tqd",
                    description: Some(
                        "Traffic queue depth (IP core parameter TQD).",
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
                    name: "tqc",
                    description: Some(
                        "Traffic queue count (IP core parameter TQC).",
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
                    name: "lwidth",
                    description: Some(
                        "Scheduler list address width (IP core parameter LWIDTH).",
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
            name: "TsnShaperMmct",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rqhld",
                    description: Some(
                        "Request HOLD-Signal hold operation. Will be automatically set to <0>.",
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
                    name: "rqrel",
                    description: Some(
                        "Request HOLD-Signal release operation. Will be automatically set to <0>.",
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
            name: "TsnShaperTasAbasetmH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "basetm_h",
                    description: Some(
                        "No description available.",
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
            name: "TsnShaperTasAbasetmL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "basetm_l",
                    description: Some(
                        "Admin basetime – nanoseconds and seconds part.",
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
            ],
        },
        FieldSet {
            name: "TsnShaperTasAcycletm",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctime",
                    description: Some(
                        "Admin cycletime in nanoseconds.",
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
            ],
        },
        FieldSet {
            name: "TsnShaperTasCrsr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable time aware scheduling.",
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
                    name: "cfgchg",
                    description: Some(
                        "Switch configuration; Bit is automatically reset to 0; Setting Bit=1 triggers configuration change event.",
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
                    name: "cfgerr",
                    description: Some(
                        "Configuration error.",
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
                    name: "cfgpend",
                    description: Some(
                        "Configuration change is pending – Admin basetime not yet reached.",
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
                    name: "tas_gpio_sta",
                    description: Some(
                        "operational tas gpio gate status of TQ[i].",
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
                    name: "opergs",
                    description: Some(
                        "Operational gate states of TQ[i] (i = 0 – TQC-1) Bit[i]=0 – Gate is closed; no start of frame TX possible Bit[i]=1 – Gate is open.",
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
                    name: "admings",
                    description: Some(
                        "Admin gate states, fixed 0xFF. Gate states when TAS is disabled.",
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
            name: "TsnShaperTasListlen",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "alistlen",
                    description: Some(
                        "Admin list length.",
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
                    name: "olistlen",
                    description: Some(
                        "Oper list length.",
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
            name: "TsnShaperTasObasetmH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "basetm_h",
                    description: Some(
                        "No description available.",
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
            name: "TsnShaperTasObasetmL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "basetm_l",
                    description: Some(
                        "Operational basetime – nanoseconds and seconds part. The operational basetime might occasionally have a non-normalized value (ns >= 10^9) for one clock cycle.",
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
            name: "TsnShaperTasOcycletm",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctime",
                    description: Some(
                        "Operational cycletime in nanoseconds.",
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
            ],
        },
        FieldSet {
            name: "TsnShaperTqav",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avail",
                    description: Some(
                        "Traffic queue buffer space available for complete packet of size MaxSDU (register MXSDUi) Bit[i] = 1: space available Bit[i] = 0: no space available or TQ not implemented (I >= TQC).",
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
                    name: "avie",
                    description: Some(
                        "Traffic queue interrupt enable on buffer space available, one bit per traffic queue Bit[i] = 0: no interrupt Bit[i] = 1: interrupt, when AVAIL[i]=1.",
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
            ],
        },
        FieldSet {
            name: "TsnShaperTqem",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "empty",
                    description: Some(
                        "Traffic queue empty Bit[i] = 1: traffic queue i is empty.",
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
            name: "TsynCr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txie",
                    description: Some(
                        "Tx Interrupt Enable.",
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
                    name: "rxie",
                    description: Some(
                        "Rx Interrupt Enable.",
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
                    name: "tmrie",
                    description: Some(
                        "Timer Interrupt Enable.",
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
                    name: "tmr_en",
                    description: Some(
                        "Timer Enable: every bit corresponds to Timer 0 – 4.",
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
                    name: "tmr_ald",
                    description: Some(
                        "Timer Auto Load: automatic reloading of timer when reaching 0. Done flag stays set after countdown. Used for periodic events, when following event shall not be delayed by host interaction.",
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
            name: "TsynHclkdiv",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "period",
                    description: Some(
                        "Period in host clocks <sys_clk>. Host clock shall be scaled to ticks of 1/1024th second. Ticks are used by timer TMR0 – TMR4.",
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
            name: "TsynPtpRxSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_sel",
                    description: Some(
                        "Current selected RX buffer for reading (0-7). Can be used to determine when RX buffer has been switched after setting PTP_RX_STS.NXT.",
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
                    name: "av_nxt",
                    description: Some(
                        "Read access: buffer data available – reading data from RX_BUF is valid. Write access: switch to next RX buffer – shall only be done when buffer not empty (AV=1). Use field RX_SEL as indication when rx buffer switch has been done.",
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
                    name: "ov",
                    description: Some(
                        "FIFO overflow flag. PTP frame has been received and there was no free buffer available. Data has been lost.",
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
            name: "TsynPtpTxDone",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "done",
                    description: Some(
                        "Transmission done status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission done. Writing a ‘1’ clears corresponding bit..",
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
            name: "TsynPtpTxSts",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sts",
                    description: Some(
                        "Transmission status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission pending.",
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
            name: "TsynPtpTxTrig",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trig",
                    description: Some(
                        "Trigger PTP TX bin n (bit 0 – 7 correspond to tx bin 0 –7). Writing ‘1’ will trigger transmission. Corresponding bit PTP_TX_STS.STS(n) will be set immediately.",
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
            name: "TsynRxbufRxFrameLengthBytes",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_frame_length_bytes",
                    description: Some(
                        "RX frame length bytes [11:0].",
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
            name: "TsynRxbufRxTimeStampH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_timestamp_high",
                    description: Some(
                        "RX Timestamp [63:32].",
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
            name: "TsynRxbufRxTimeStampL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_timestamp_low",
                    description: Some(
                        "RX Timestamp [31:0].",
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
            name: "TsynSr",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txis",
                    description: Some(
                        "Tx Done Interrupt Status: OR’ed PTP_TX_DONE.",
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
                    name: "rxis",
                    description: Some(
                        "Rx Interrupt Status, RX buffer data available equal to PTP_RX_STS.AV).",
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
                    name: "tmris",
                    description: Some(
                        "Timer Interrupt Status: OR’ed (TMR_DN AND TMR_EN) flags. 1 when timer is enabled and countdown is done.",
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
                    name: "tmr_dn",
                    description: Some(
                        "Timer Done: 1 when timer reached 0.",
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
            name: "TsynTxbufBin0TqueAndTxLen",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbuf_bin0_tx_len",
                    description: Some(
                        "TXBUF_BIN0_TX_LEN.",
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
                    name: "txbuf_bin0_tque",
                    description: Some(
                        "TXBUF_BIN0_TQUE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TsynTxbufBin0TxTimestampH",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbuf_bin0_tx_timestamp_h",
                    description: Some(
                        "TXBUF_BIN0TX_TIMESTAMP_H.",
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
            name: "TsynTxbufBin0TxTimestampL",
            extends: None,
            description: Some(
                "No description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbuf_bin0_tx_timestamp_l",
                    description: Some(
                        "TXBUF_BIN0_TX_TIMESTAMP_L.",
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
            name: "Tsyntmr",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "period",
                    description: Some(
                        "Period in ticks, ticks based on register HCLKDIV and host clock <sys_clk>.",
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
            name: "Txdata",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txbuf_bin0_data_word0",
                    description: Some(
                        "TXBUF_BIN0_DATA_WORD0.",
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
            name: "Txov",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "value",
                    description: Some(
                        "Transmission overrun counter; increments on transmission when gate is closed; any write access will clear register to 0. TXOVi is only supported when TQC > i.",
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
            name: "Txsel",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cbs_en",
                    description: Some(
                        "CBS enable traffic queue n (n = 0 – 7). Returns 0 when n > TQC. Must be 0 when changing register IDSLPi.",
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
    ],
    enums: &[],
};
