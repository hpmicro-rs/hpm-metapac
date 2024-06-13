use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Can",
            extends: None,
            description: Some(
                "CAN0.",
            ),
            items: &[
                BlockItem {
                    name: "rbuf",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 20,
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
                                "Rbuf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tbuf",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 18,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tbuf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tts",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmd_sta_cmd_ctrl",
                    description: Some(
                        "config, status, command and control bits.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CmdStaCmdCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtie",
                    description: Some(
                        "Receive and Transmit Interrupt Enable Register RTIE.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Rtie",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtif",
                    description: Some(
                        "Receive and Transmit Interrupt Flag Register RTIF (0xa5).",
                    ),
                    array: None,
                    byte_offset: 0xa5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Rtif",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "errint",
                    description: Some(
                        "ERRor INTerrupt Enable and Flag Register ERRINT.",
                    ),
                    array: None,
                    byte_offset: 0xa6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Errint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "limit",
                    description: Some(
                        "Warning Limits Register LIMIT.",
                    ),
                    array: None,
                    byte_offset: 0xa7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Limit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "s_presc",
                    description: Some(
                        "Bit Timing Register(Slow Speed).",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SPresc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "f_presc",
                    description: Some(
                        "Bit Timing Register(Fast Speed).",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FPresc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ealcap",
                    description: Some(
                        "Error and Arbitration Lost Capture Register EALCAP.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ealcap",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tdc",
                    description: Some(
                        "Transmitter Delay Compensation Register TDC.",
                    ),
                    array: None,
                    byte_offset: 0xb1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Tdc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "recnt",
                    description: Some(
                        "Error Counter Registers RECNT.",
                    ),
                    array: None,
                    byte_offset: 0xb2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Recnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tecnt",
                    description: Some(
                        "Error Counter Registers TECNT.",
                    ),
                    array: None,
                    byte_offset: 0xb3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Tecnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acfctrl",
                    description: Some(
                        "Acceptance Filter Control Register ACFCTRL.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Acfctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timecfg",
                    description: Some(
                        "CiA 603 Time-Stamping TIMECFG.",
                    ),
                    array: None,
                    byte_offset: 0xb5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Timecfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acf_en",
                    description: Some(
                        "Acceptance Filter Enable ACF_EN.",
                    ),
                    array: None,
                    byte_offset: 0xb6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "AcfEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "acf",
                    description: Some(
                        "Acceptance CODE ACODE or ACMASK.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Acf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ver",
                    description: Some(
                        "Version Information VER.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ver",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tbslot",
                    description: Some(
                        "TTCAN: TB Slot Pointer TBSLOT.",
                    ),
                    array: None,
                    byte_offset: 0xbe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Tbslot",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ttcfg",
                    description: Some(
                        "TTCAN: Time Trigger Configuration TTCFG.",
                    ),
                    array: None,
                    byte_offset: 0xbf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ttcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ref_msg",
                    description: Some(
                        "TTCAN: Reference Message REF_MSG.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RefMsg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trig_cfg",
                    description: Some(
                        "TTCAN: Trigger Configuration TRIG_CFG.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "TrigCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tt_trig",
                    description: Some(
                        "TTCAN: Trigger Time TT_TRIG.",
                    ),
                    array: None,
                    byte_offset: 0xc6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "TtTrig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tt_wtrig",
                    description: Some(
                        "TTCAN: Watch Trigger Time TT_WTRIG.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "TtWtrig",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Acf",
            extends: None,
            description: Some(
                "Acceptance CODE ACODE or ACMASK.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "code_mask",
                    description: Some(
                        "Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset.",
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
                Field {
                    name: "aide",
                    description: Some(
                        "Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized.",
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
                    name: "aidee",
                    description: Some(
                        "Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized.",
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
            name: "AcfEn",
            extends: None,
            description: Some(
                "Acceptance Filter Enable ACF_EN.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "acf_en",
                    description: Some(
                        "Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches.",
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
            name: "Acfctrl",
            extends: None,
            description: Some(
                "Acceptance Filter Control Register ACFCTRL.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "acfadr",
                    description: Some(
                        "acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16.",
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
                    name: "selmask",
                    description: Some(
                        "SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter.",
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
            name: "CmdStaCmdCtrl",
            extends: None,
            description: Some(
                "config, status, command and control bits.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busoff",
                    description: Some(
                        "Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details.",
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
                    name: "tactive",
                    description: Some(
                        "Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity.",
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
                    name: "ractive",
                    description: Some(
                        "Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity.",
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
                    name: "tsss",
                    description: Some(
                        "Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled.",
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
                    name: "tpss",
                    description: Some(
                        "Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled.",
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
                    name: "lbmi",
                    description: Some(
                        "Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active.",
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
                    name: "lbme",
                    description: Some(
                        "Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active.",
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
                    name: "reset",
                    description: Some(
                        "RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock.",
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
                    name: "tsa",
                    description: Some(
                        "Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL.",
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
                    name: "tsall",
                    description: Some(
                        "Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty.",
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
                    name: "tsone",
                    description: Some(
                        "Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1).",
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
                    name: "tpa",
                    description: Some(
                        "Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE.",
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
                    name: "tpe",
                    description: Some(
                        "Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1).",
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
                    name: "stby",
                    description: Some(
                        "Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission.",
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
                    name: "lom",
                    description: Some(
                        "Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames.",
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
                    name: "tbsel",
                    description: Some(
                        "Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1).",
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
                    name: "tsstat",
                    description: Some(
                        "Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full.",
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
                    name: "tttbm",
                    description: Some(
                        "TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty.",
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
                    name: "tsmode",
                    description: Some(
                        "Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty.",
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
                    name: "tsnext",
                    description: Some(
                        "Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0.",
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
                    name: "fd_iso",
                    description: Some(
                        "CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1.",
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
                    name: "rstat",
                    description: Some(
                        "Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV).",
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
                    name: "rball",
                    description: Some(
                        "Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error.",
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
                    name: "rrel",
                    description: Some(
                        "Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release.",
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
                    name: "rov",
                    description: Some(
                        "Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1.",
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
                    name: "rom",
                    description: Some(
                        "Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten.",
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
                    name: "sack",
                    description: Some(
                        "Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1.",
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
            name: "Ealcap",
            extends: None,
            description: Some(
                "Error and Arbitration Lost Capture Register EALCAP.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "alc",
                    description: Some(
                        "Arbitration Lost Capture (bit position in the frame where the arbitration has been lost).",
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
                    name: "koer",
                    description: Some(
                        "Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received.",
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
            name: "Errint",
            extends: None,
            description: Some(
                "ERRor INTerrupt Enable and Flag Register ERRINT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "beif",
                    description: Some(
                        "Bus Error Interrupt Flag.",
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
                    name: "beie",
                    description: Some(
                        "Bus Error Interrupt Enable.",
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
                    name: "alif",
                    description: Some(
                        "Arbitration Lost Interrupt Flag.",
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
                    name: "alie",
                    description: Some(
                        "Arbitration Lost Interrupt Enable.",
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
                    name: "epif",
                    description: Some(
                        "Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled.",
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
                    name: "epie",
                    description: Some(
                        "Error Passive Interrupt Enable.",
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
                    name: "epass",
                    description: Some(
                        "Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive).",
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
                    name: "ewarn",
                    description: Some(
                        "Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL.",
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
            name: "FPresc",
            extends: None,
            description: Some(
                "Bit Timing Register(Fast Speed).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "f_seg_1",
                    description: Some(
                        "Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time.",
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
                    name: "f_seg_2",
                    description: Some(
                        "Bit Timing Segment 2 (fast speed) Time after the sample point.",
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
                    name: "f_sjw",
                    description: Some(
                        "Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta.",
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
                    name: "f_presc",
                    description: Some(
                        "Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=[0x00, 0xff] results in divider values 1 to 256.",
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
            name: "Limit",
            extends: None,
            description: Some(
                "Warning Limits Register LIMIT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ewl",
                    description: Some(
                        "Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF.",
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
                    name: "afwl",
                    description: Some(
                        "receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists.",
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
            name: "Rbuf",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rbuf",
                    description: Some(
                        "receive buffer.",
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
            name: "Recnt",
            extends: None,
            description: Some(
                "Error Counter Registers RECNT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "recnt",
                    description: Some(
                        "Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen.",
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
            name: "RefMsg",
            extends: None,
            description: Some(
                "TTCAN: Reference Message REF_MSG.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ref_msg",
                    description: Some(
                        "REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message.",
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
                Field {
                    name: "ref_ide",
                    description: Some(
                        "REFerence message IDE bit.",
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
            name: "Rtie",
            extends: None,
            description: Some(
                "Receive and Transmit Interrupt Enable Register RTIE.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tsff",
                    description: Some(
                        "If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty.",
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
                    name: "eie",
                    description: Some(
                        "Error Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "tsie",
                    description: Some(
                        "Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "tpie",
                    description: Some(
                        "Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "rafie",
                    description: Some(
                        "RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "rfie",
                    description: Some(
                        "RB Full Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "roie",
                    description: Some(
                        "RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
                    name: "rie",
                    description: Some(
                        "Receive Interrupt Enable 0 – Disabled, 1 – Enabled.",
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
            name: "Rtif",
            extends: None,
            description: Some(
                "Receive and Transmit Interrupt Flag Register RTIF (0xa5).",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "aif",
                    description: Some(
                        "Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register.",
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
                    name: "eif",
                    description: Some(
                        "Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change.",
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
                    name: "tsif",
                    description: Some(
                        "Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message.",
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
                    name: "tpif",
                    description: Some(
                        "Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid.",
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
                    name: "rafif",
                    description: Some(
                        "RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i.",
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
                    name: "rfif",
                    description: Some(
                        "RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full.",
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
                    name: "roif",
                    description: Some(
                        "RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set.",
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
                    name: "rif",
                    description: Some(
                        "Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received.",
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
            name: "SPresc",
            extends: None,
            description: Some(
                "Bit Timing Register(Slow Speed).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "s_seg_1",
                    description: Some(
                        "Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time.",
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
                    name: "s_seg_2",
                    description: Some(
                        "Bit Timing Segment 2 (slow speed) Time after the sample point.",
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
                    name: "s_sjw",
                    description: Some(
                        "Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta.",
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
                    name: "s_presc",
                    description: Some(
                        "Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=[0x00, 0xff] results in divider values 1 to 256.",
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
            name: "Tbslot",
            extends: None,
            description: Some(
                "TTCAN: TB Slot Pointer TBSLOT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tbptr",
                    description: Some(
                        "Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place.",
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
                    name: "tbf",
                    description: Some(
                        "set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins.",
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
                    name: "tbe",
                    description: Some(
                        "set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins.",
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
            name: "Tbuf",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbuf",
                    description: Some(
                        "transmit buffer.",
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
            name: "Tdc",
            extends: None,
            description: Some(
                "Transmitter Delay Compensation Register TDC.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sspoff",
                    description: Some(
                        "Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ.",
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
                    name: "tdcen",
                    description: Some(
                        "Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1.",
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
            name: "Tecnt",
            extends: None,
            description: Some(
                "Error Counter Registers TECNT.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tecnt",
                    description: Some(
                        "Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen.",
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
            name: "Timecfg",
            extends: None,
            description: Some(
                "CiA 603 Time-Stamping TIMECFG.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "timeen",
                    description: Some(
                        "TIME-stamping ENable 0 – disabled 1 – enabled.",
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
                    name: "timepos",
                    description: Some(
                        "TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1.",
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
            name: "TrigCfg",
            extends: None,
            description: Some(
                "TTCAN: Trigger Configuration TRIG_CFG.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ttptr",
                    description: Some(
                        "Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached.",
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
                    name: "ttype",
                    description: Some(
                        "Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details.",
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
                    name: "tew",
                    description: Some(
                        "Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick.",
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
            name: "TtTrig",
            extends: None,
            description: Some(
                "TTCAN: Trigger Time TT_TRIG.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tt_trig",
                    description: Some(
                        "Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1.",
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
            name: "TtWtrig",
            extends: None,
            description: Some(
                "TTCAN: Watch Trigger Time TT_WTRIG.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "tt_wtrig",
                    description: Some(
                        "Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff.",
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
            name: "Ttcfg",
            extends: None,
            description: Some(
                "TTCAN: Time Trigger Configuration TTCFG.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tten",
                    description: Some(
                        "Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled.",
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
                    name: "t_presc",
                    description: Some(
                        "TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access.",
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
                    name: "ttif",
                    description: Some(
                        "Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing a one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle.",
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
                    name: "ttie",
                    description: Some(
                        "Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG.",
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
                    name: "teif",
                    description: Some(
                        "Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF.",
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
                    name: "wtif",
                    description: Some(
                        "Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set.",
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
                    name: "wtie",
                    description: Some(
                        "Watch Trigger Interrupt Enable.",
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
            name: "Tts",
            extends: None,
            description: Some(
                "no description available.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tts_wrd0",
                    description: Some(
                        "transmission time stamp, word 0, LSB 32bit.",
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
            name: "Ver",
            extends: None,
            description: Some(
                "Version Information VER.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "Version of CAN-CTRL, given as decimal value. VER_1 holds the major version and VER_0 the minor version.Example: version 5x16N00S00 is represented by VER_1=5 and VER_0=16.",
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
